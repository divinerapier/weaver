use std::io::Read;
use std::path::Path;

use futures::stream::Stream;
use futures::{Async, Poll};

pub struct FileStream {
    start: std::time::SystemTime,
    batch_size: usize,
    index: usize,
    pub file: std::fs::File,
    pub buffer: Vec<u8>,
    pub length: usize,
}

impl FileStream {
    pub fn new<P: AsRef<Path>>(path: P) -> std::io::Result<FileStream> {
        let batch_size: usize = 4096000;
        let file = std::fs::File::open(path)?;
        let mut buffer = Vec::with_capacity(batch_size);
        buffer.resize(batch_size, 0);
        Ok(FileStream {
            start: std::time::SystemTime::now(),
            batch_size,
            index: 0,
            file,
            buffer,
            length: 0,
        })
    }

    pub fn from_std_file(file: std::fs::File) -> FileStream {
        let batch_size: usize = 4096000;
        let mut buffer = Vec::with_capacity(batch_size);
        buffer.resize(batch_size, 0);
        FileStream {
            start: std::time::SystemTime::now(),
            batch_size,
            index: 0,
            file,
            buffer,
            length: 0,
        }
    }
}

impl Stream for FileStream {
    type Item = bytes::Bytes;
    type Error = std::io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let start = std::time::SystemTime::now();
        let size = self.file.read(&mut self.buffer)?;
        println!(
            "index: {}, read size: {}, start at {:?}, elapsed {:?}",
            self.index,
            size,
            start,
            start.elapsed()
        );
        if self.index == 0 {
            self.start = std::time::SystemTime::now();
        }
        self.index += 1;
        if size == 0 {
            println!("read elapsed {:?}", self.start.elapsed());
            return Ok(Async::Ready(None));
        }
        println!("poll elapsed {:?}", start.elapsed());
        Ok(Async::Ready(Some(bytes::Bytes::from(
            self.buffer[0..size].to_vec(),
        ))))
    }
}
