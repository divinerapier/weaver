use std::{
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
    sync::{Arc, RwLock},
};

use serde::{Deserialize, Serialize};

struct VolumeFiles {
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
    read_sender: tokio::sync::mpsc::Sender<(u64, tokio::sync::oneshot::Sender<Vec<u8>>)>,
    write_sender: tokio::sync::mpsc::Sender<(Vec<u8>, tokio::sync::oneshot::Sender<Vec<u8>>)>,
}

impl VolumeFiles {
    fn new<P: AsRef<Path>>(dir: P, index: usize) -> VolumeFiles {
        let data_file = Self::openfile(dir.as_ref().join(format!("{}.dat", index)));
        let index_file = Self::openfile(dir.as_ref().join(format!("{}.idx", index)));
        let current_size = data_file.metadata().unwrap().len();
        let indexes = NeedleIndexes::new(index_file.try_clone().unwrap());
        VolumeFiles {
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
            current_size: self.current_size,
            indexes: self.indexes.clone(),
        };
        let writer = VolumeFiles {
            data_file: self.data_file.try_clone().unwrap(),
            index_file: self.index_file.try_clone().unwrap(),
            current_size: self.current_size,
            indexes: self.indexes,
        };
        (reader, writer)
    }

    fn openfile<P: AsRef<Path>>(path: P) -> std::fs::File {
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
        mut receiver: tokio::sync::mpsc::Receiver<(u64, tokio::sync::oneshot::Sender<Vec<u8>>)>,
    ) {
        std::thread::spawn(move || {
            tokio::runtime::Handle::current().spawn(async move {
                while let Some((key, sender)) = receiver.recv().await {
                    let indexes = match self.indexes.0.read() {
                        Ok(idx) => idx,
                        Err(a) => a.into_inner(),
                    };
                    match indexes.get(&key) {
                        Some(index) => match self.data_file.seek(SeekFrom::Start(index.offset)) {
                            Ok(pos) => {
                                let mut buf = vec![0u8; index.size as usize];
                                if let Err(e) = self.data_file.read_exact(&mut buf) {
                                    log::error!("")
                                }
                                if let Err(e) = sender.send(buf) {
                                    log::error!("")
                                }
                            }
                            Err(e) => {
                                log::error!("")
                            }
                        },
                        None => {
                            log::error!("")
                        }
                    }
                }
            })
        });
    }

    fn start_write(
        mut self,
        mut receiver: tokio::sync::mpsc::Receiver<(Vec<u8>, tokio::sync::oneshot::Sender<Vec<u8>>)>,
    ) {
        std::thread::spawn(move || {
            tokio::runtime::Handle::current().spawn(async move {
                while let Some((buf, sender)) = receiver.recv().await {
                    match self.data_file.write_all(&buf) {
                        Ok(_) => {
                            let index = NeedleIndex {
                                key: 0,
                                alternate_key: 0,
                                flags: 0,
                                offset: self.current_size,
                                size: buf.len() as u32,
                            };
                            let buf = serde_json::to_string(&index).unwrap();
                            self.index_file.write_all(buf.as_bytes()).unwrap();
                        }
                        Err(e) => {}
                    }
                }
            })
        });
    }

    fn write_buffer(&mut self, buf: &[u8]) -> std::result::Result<(), Box<dyn std::error::Error>> {
        self.data_file.write_all(buf)?;

        let index = NeedleIndex {
            key: 0,
            alternate_key: 0,
            flags: 0,
            offset: self.current_size,
            size: buf.len() as u32,
        };
        serde_json::to_string(&index)?;
        Ok(())
    }
}

impl VolumeImpl {
    pub fn new<P: AsRef<Path>>(dir: P, index: usize) -> VolumeImpl {
        let (read_sender, read_receiver) = tokio::sync::mpsc::channel(128);
        let (write_sender, write_receiver) = tokio::sync::mpsc::channel(128);
        let files = VolumeFiles::new(dir.as_ref(), index);
        let (reader, writer) = files.split();

        reader.start_read(read_receiver);
        writer.start_write(write_receiver);

        VolumeImpl {
            read_sender,
            write_sender,
        }
    }

    pub async fn write(&mut self, buf: Vec<u8>) {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.write_sender.send((buf, tx)).await.unwrap();
        let res = rx.await;
    }

    pub async fn read(&mut self, key: u64) {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.read_sender.send((key, tx)).await.unwrap();
        let res = rx.await;
    }
}
