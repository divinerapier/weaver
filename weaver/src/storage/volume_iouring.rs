use std::{
    io::{Read, Seek, SeekFrom},
    path::Path,
};

struct FilePair {
    file: std::fs::File,
    index: std::fs::File,
}

#[derive(Clone)]
pub struct VolumeImpl {
    read_sender: tokio::sync::mpsc::Sender<(Vec<u8>, tokio::sync::oneshot::Sender<Vec<u8>>)>,
    write_sender: tokio::sync::mpsc::Sender<(Vec<u8>, tokio::sync::oneshot::Sender<Vec<u8>>)>,
}

impl FilePair {
    fn new<P: AsRef<Path>>(path: P) -> FilePair {
        let file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .write(true)
            .read(true)
            .open(path.as_ref())
            .unwrap();

        let index = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .write(true)
            .read(true)
            .open(path)
            .unwrap();

        FilePair { file, index }
    }

    fn start_read(
        mut self,
        mut receiver: tokio::sync::mpsc::Receiver<(Vec<u8>, tokio::sync::oneshot::Sender<Vec<u8>>)>,
    ) {
        std::thread::spawn(move || {
            tokio::runtime::Handle::current().spawn(async move {
                while let Some((mut buf, sender)) = receiver.recv().await {
                    match self.file.seek(SeekFrom::Start(0)) {
                        Ok(pos) => {
                            if let Err(e) = self.file.read_exact(&mut buf) {}
                            if let Err(e) = sender.send(buf) {}
                        }
                        Err(e) => {}
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
                while let Some(input) = receiver.recv().await {
                    match self.file.seek(SeekFrom::Start(0)) {
                        Ok(pos) => {}
                        Err(e) => {}
                    }
                }
            })
        });
    }
}

impl VolumeImpl {
    pub fn new<P: AsRef<Path>>(path: P) -> VolumeImpl {
        let (read_sender, read_receiver) = tokio::sync::mpsc::channel(128);
        let (write_sender, write_receiver) = tokio::sync::mpsc::channel(128);
        let reader = FilePair::new(path.as_ref());
        let writer = FilePair::new(path);

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

    pub async fn read(&mut self, buf: Vec<u8>) {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.read_sender.send((buf, tx)).await.unwrap();
        let res = rx.await;
    }
}
