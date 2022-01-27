use std::{
    collections::HashMap,
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
    sync::{Arc, RwLock},
};

use serde::{Deserialize, Serialize};
use tokio::sync::{
    mpsc::{Receiver as MpscReceiver, Sender as MpscSender},
    oneshot::Sender as OneshotSender,
};

use crate::{directory::DirectoryStorage, error::VolumeError};

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
struct NeedleIndexes(Arc<RwLock<HashMap<u64, NeedleIndex>>>);

impl NeedleIndexes {
    pub fn new(file: std::fs::File) -> NeedleIndexes {
        let mut map = HashMap::new();
        let reader = std::io::BufReader::new(file);
        let mut iter = serde_json::Deserializer::from_reader(reader).into_iter::<NeedleIndex>();
        while let Some(Ok(index)) = iter.next() {
            map.insert(index.key, index);
        }
        NeedleIndexes(Arc::new(RwLock::new(map)))
    }
}

type ReadSender = MpscSender<(
    u64,
    OneshotSender<std::result::Result<Vec<u8>, VolumeError>>,
)>;

type ReadReceiver = MpscReceiver<(
    u64,
    OneshotSender<std::result::Result<Vec<u8>, VolumeError>>,
)>;

type WriteSender = MpscSender<(
    u64,
    Vec<u8>,
    OneshotSender<std::result::Result<(), VolumeError>>,
)>;

type WriteReceiver = MpscReceiver<(
    u64,
    Vec<u8>,
    OneshotSender<std::result::Result<(), VolumeError>>,
)>;

#[derive(Clone)]
pub struct VolumeImpl {
    read_sender: ReadSender,
    write_sender: WriteSender,
}

impl VolumeFiles {
    fn new<P: AsRef<Path>>(dir: P, index: usize) -> std::result::Result<VolumeFiles, VolumeError> {
        let volume_dir = dir.as_ref().join(index.to_string());
        Self::ensure_dir(dir.as_ref())?;
        Self::ensure_dir(&volume_dir)?;
        Self::load(volume_dir, index as u64)
    }

    fn load<P: AsRef<Path>>(dir: P, id: u64) -> std::result::Result<VolumeFiles, VolumeError> {
        let data_file = Self::openfile(dir.as_ref().join("data"))?;
        let index_file = Self::openfile(dir.as_ref().join("index"))?;
        let current_size = data_file.metadata()?.len();
        let indexes = NeedleIndexes::new(index_file.try_clone().unwrap());
        Ok(VolumeFiles {
            id,
            data_file,
            index_file,
            current_size,
            indexes,
        })
    }

    fn ensure_dir<P: AsRef<Path>>(dir: P) -> std::result::Result<(), VolumeError> {
        if !dir.as_ref().exists() {
            log::info!("volume: create dir: {:?}", dir.as_ref());
            std::fs::create_dir_all(dir.as_ref())?;
            return Ok(());
        }
        if dir.as_ref().is_dir() {
            return Ok(());
        }
        log::error!("ensure volume: {:?} is not a dir", dir.as_ref());
        Err(VolumeError::NotDir(dir.as_ref().to_path_buf()))
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

    fn openfile<P: AsRef<Path>>(path: P) -> std::result::Result<std::fs::File, VolumeError> {
        log::debug!("path: {:?}", std::fs::canonicalize(path.as_ref()));
        Ok(std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .write(true)
            .read(true)
            .open(path)?)
    }

    fn start_read(mut self, mut receiver: ReadReceiver) -> std::result::Result<(), VolumeError> {
        let runtime = tokio::runtime::Builder::new_current_thread().build()?;
        std::thread::spawn(move || {
            runtime.block_on(async move {
                while let Some((key, sender)) = receiver.recv().await {
                    if sender.send(self.read_process(key).await).is_err() {
                        log::error!("key: {}. the receiver dropped", key,)
                    }
                }
            });
            log::warn!("quit read thread")
        });
        Ok(())
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

    fn start_write(mut self, mut receiver: WriteReceiver) -> std::result::Result<(), VolumeError> {
        let runtime = tokio::runtime::Builder::new_current_thread().build()?;
        std::thread::spawn(move || {
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
        Ok(())
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
    pub fn new<P: AsRef<Path>>(
        dir: P,
        index: usize,
    ) -> std::result::Result<VolumeImpl, VolumeError> {
        let (read_sender, read_receiver) = tokio::sync::mpsc::channel(128);
        let (write_sender, write_receiver) = tokio::sync::mpsc::channel(128);
        let (reader, writer) = VolumeFiles::new(dir.as_ref(), index)?.split();

        reader.start_read(read_receiver)?;
        writer.start_write(write_receiver)?;

        Ok(VolumeImpl {
            read_sender,
            write_sender,
        })
    }

    pub async fn write(&self, key: u64, buf: Vec<u8>) -> std::result::Result<(), VolumeError> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.write_sender
            .send((key, buf, tx))
            .await
            .map_err(|_| VolumeError::ChannelClosed)?;
        match rx.await {
            Err(e) => {
                log::error!("recv write. key: {}, error: {}", key, e);
                Ok(())
            }
            Ok(Err(e)) => Err(e),
            Ok(Ok(buf)) => Ok(buf),
        }
    }

    pub async fn read(&self, key: u64) -> std::result::Result<Vec<u8>, VolumeError> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        if let Err(e) = self.read_sender.send((key, tx)).await {
            log::error!("send read. key: {}, error: {}", key, e);
            return Ok(vec![]);
        }
        match rx.await {
            Err(e) => {
                log::error!("recv read. key: {}, error: {}", key, e);
                Err(VolumeError::ChannelClosed)
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

        test1().await.unwrap();
        test2().await.unwrap();
    }
    async fn test1() -> std::result::Result<(), Box<dyn std::error::Error>> {
        std::fs::remove_dir_all("./testdata")?;
        let volume = VolumeImpl::new("./testdata", 1)?;
        let v = volume.clone();
        let task1 = tokio::spawn(async move { v.write(10, "hello world".bytes().collect()).await });
        let v = volume.clone();

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
            Err(VolumeError::NeedleNotFound(1, 12)),
            volume.read(12).await
        );
        assert_eq!(
            "hello world2".bytes().collect::<Vec<u8>>(),
            volume.read(10).await.unwrap()
        );
        Ok(())
    }

    async fn test2() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let volume = VolumeImpl::new("./testdata", 1)?;
        assert_eq!(
            "hello world2".bytes().collect::<Vec<u8>>(),
            volume.read(10).await.unwrap()
        );
        Ok(())
    }
}
