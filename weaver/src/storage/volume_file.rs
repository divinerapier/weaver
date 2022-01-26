use std::{
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
    sync::{Arc, RwLock},
};

use serde::{Deserialize, Serialize};

use crate::error::VolumeError;

struct VolumeFiles {
    id: u64,
    current_size: u64,
    data_file: std::fs::File,
    index_file: std::fs::File,
    indexes: NeedleIndexes,
}

#[derive(Serialize, Deserialize)]
struct NeedleIndex {
    key: u64,           // 8
    alternate_key: u32, // 4
    flags: u32,         // 4
    offset: u64,        // 8
    size: u32,          // 4
}

#[derive(Clone)]
struct NeedleIndexes(Arc<RwLock<std::collections::HashMap<u64, NeedleIndex>>>);

impl NeedleIndexes {
    pub fn new(file: std::fs::File) -> NeedleIndexes {
        let mut map = std::collections::HashMap::new();
        let reader = std::io::BufReader::new(file);
        let mut iter = serde_json::Deserializer::from_reader(reader).into_iter::<NeedleIndex>();
        while let Some(Ok(index)) = iter.next() {
            map.insert(index.key, index);
        }
        NeedleIndexes(Arc::new(RwLock::new(map)))
    }
}

#[derive(Clone)]
pub struct VolumeImpl {
    read_sender: tokio::sync::mpsc::Sender<(
        u64,
        tokio::sync::oneshot::Sender<std::result::Result<Vec<u8>, VolumeError>>,
    )>,
    write_sender: tokio::sync::mpsc::Sender<(
        u64,
        Vec<u8>,
        tokio::sync::oneshot::Sender<std::result::Result<(), VolumeError>>,
    )>,
}

impl VolumeFiles {
    fn new<P: AsRef<Path>>(dir: P, index: usize) -> VolumeFiles {
        let data_file = Self::openfile(dir.as_ref().join(format!("{}.dat", index)));
        let index_file = Self::openfile(dir.as_ref().join(format!("{}.idx", index)));
        let current_size = data_file.metadata().unwrap().len();
        let indexes = NeedleIndexes::new(index_file.try_clone().unwrap());
        VolumeFiles {
            id: index as u64,
            data_file,
            index_file,
            current_size,
            indexes,
        }
    }

    fn split(self) -> (VolumeFiles, VolumeFiles) {
        let reader = VolumeFiles {
            data_file: self.data_file.try_clone().unwrap(),
            index_file: self.index_file.try_clone().unwrap(),
            indexes: self.indexes.clone(),
            ..self
        };
        (reader, self)
    }

    fn openfile<P: AsRef<Path>>(path: P) -> std::fs::File {
        let dir = path.as_ref().parent().unwrap();
        if let Err(e) = std::fs::create_dir_all(dir) {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                log::error!("create dir all. path: {:?}, error: {}", path.as_ref(), e);
                panic!("create dir")
            }
        }
        log::debug!("path: {:?}", std::fs::canonicalize(dir));
        std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .write(true)
            .read(true)
            .open(path.as_ref())
            .unwrap()
    }

    fn start_read(
        mut self,
        mut receiver: tokio::sync::mpsc::Receiver<(
            u64,
            tokio::sync::oneshot::Sender<std::result::Result<Vec<u8>, VolumeError>>,
        )>,
    ) {
        std::thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .build()
                .unwrap();
            runtime.block_on(async move {
                while let Some((key, sender)) = receiver.recv().await {
                    if sender.send(self.read_process(key).await).is_err() {
                        log::error!("key: {}. the receiver dropped", key,)
                    }
                }
            });
            log::warn!("quit read thread")
        });
    }

    async fn read_process(&mut self, key: u64) -> std::result::Result<Vec<u8>, VolumeError> {
        let indexes = match self.indexes.0.read() {
            Ok(idx) => idx,
            Err(a) => a.into_inner(),
        };
        let index = indexes
            .get(&key)
            .ok_or(VolumeError::NeedleNotFound(self.id, key))?;
        log::info!("key: {} offset: {}", key, index.offset);
        self.data_file.seek(SeekFrom::Start(index.offset))?;
        let mut buf = vec![0u8; index.size as usize];
        self.data_file.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn start_write(
        mut self,
        mut receiver: tokio::sync::mpsc::Receiver<(
            u64,
            Vec<u8>,
            tokio::sync::oneshot::Sender<std::result::Result<(), VolumeError>>,
        )>,
    ) {
        std::thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .build()
                .unwrap();
            runtime.block_on(async move {
                while let Some((key, buf, sender)) = receiver.recv().await {
                    log::info!("write. key: {}, buf: {:?}", key, buf);
                    if sender.send(self.write_process(key, buf).await).is_err() {
                        log::error!(
                            "failed to send err value. key: {}. the receiver dropped",
                            key
                        )
                    }
                }
                log::warn!("quit write thread")
            })
        });
    }

    async fn write_process(
        &mut self,
        key: u64,
        buf: Vec<u8>,
    ) -> std::result::Result<(), VolumeError> {
        self.data_file.write_all(&buf)?;
        let index = NeedleIndex {
            key,
            alternate_key: 0, // FIXME:
            flags: 0,
            offset: self.current_size,
            size: buf.len() as u32,
        };
        let index_buf = serde_json::to_string(&index).unwrap();
        self.index_file.write_all(index_buf.as_bytes()).unwrap();
        let mut indexes = match self.indexes.0.write() {
            Ok(idx) => idx,
            Err(a) => a.into_inner(),
        };
        self.current_size += index.size as u64;
        indexes.insert(key, index);
        Ok(())
    }
}

impl VolumeImpl {
    pub fn new<P: AsRef<Path>>(dir: P, index: usize) -> VolumeImpl {
        let (read_sender, read_receiver) = tokio::sync::mpsc::channel(128);
        let (write_sender, write_receiver) = tokio::sync::mpsc::channel(128);
        let volume = VolumeFiles::new(dir.as_ref(), index);
        let (reader, writer) = volume.split();

        reader.start_read(read_receiver);
        writer.start_write(write_receiver);

        VolumeImpl {
            read_sender,
            write_sender,
        }
    }

    pub async fn write(&mut self, key: u64, buf: Vec<u8>) -> std::result::Result<(), VolumeError> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.write_sender.send((key, buf, tx)).await.unwrap();
        match rx.await {
            Err(e) => {
                log::error!("recv write. key: {}, error: {}", key, e);
                Ok(())
            }
            Ok(Err(e)) => Err(e),
            Ok(Ok(buf)) => Ok(buf),
        }
    }

    pub async fn read(&mut self, key: u64) -> std::result::Result<Vec<u8>, VolumeError> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        if let Err(e) = self.read_sender.send((key, tx)).await {
            log::error!("send read. key: {}, error: {}", key, e);
            return Ok(vec![]);
        }
        match rx.await {
            Err(e) => {
                log::error!("recv read. key: {}, error: {}", key, e);
                Err(VolumeError::Channel)
            }
            Ok(Err(e)) => Err(e),
            Ok(Ok(buf)) => Ok(buf),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test() {
        env_logger::builder()
            .format_target(true)
            .format_target(true)
            .filter_level(log::LevelFilter::Debug)
            .init();

        test1().await;
        test2().await;
    }
    async fn test1() {
        std::fs::remove_dir_all("./testdata").unwrap();
        let mut volume = VolumeImpl::new("./testdata", 1);
        let mut v = volume.clone();
        let task1 = tokio::spawn(async move { v.write(10, "hello world".bytes().collect()).await });
        let mut v = volume.clone();

        if let Err(e) = task1.await {
            log::error!("{}", e);
        }
        let task2 =
            tokio::spawn(async move { v.write(10, "hello world2".bytes().collect()).await });
        if let Err(e) = task2.await {
            log::error!("{}", e);
        }
        assert_eq!(
            Err(VolumeError::NeedleNotFound(1, 12)),
            volume.read(12).await
        );
        assert_eq!(
            "hello world2".bytes().collect::<Vec<u8>>(),
            volume.read(10).await.unwrap()
        );
    }
    async fn test2() {
        let mut volume = VolumeImpl::new("./testdata", 1);
        assert_eq!(
            "hello world2".bytes().collect::<Vec<u8>>(),
            volume.read(10).await.unwrap()
        );
    }
}
