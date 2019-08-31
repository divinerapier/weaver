use std::sync::mpsc::Receiver;

use byteorder::ByteOrder;
use bytes::Bytes;

use crate::error::Result;

pub struct Needle {
    pub header: NeedleHeader,
    pub body: NeedleBody,
    pub length: usize,
}

pub struct NeedleIterator {
    start_reading: bool,
    needle: Needle,
}

pub struct NeedleHeader {
    pub header_length: u16,
    pub content_length: u32,
}

impl Default for NeedleHeader {
    fn default() -> NeedleHeader {
        NeedleHeader {
            header_length: 0,
            content_length: 0,
        }
    }
}

pub enum NeedleBody {
    SinglePart(Bytes),
    MultiParts(Receiver<Result<Bytes>>),
}

impl NeedleHeader {
    pub fn as_bytes(&self) -> Bytes {
        let mut buffer = Vec::with_capacity(6);
        buffer.resize(6, 0);
        bytes::LittleEndian::write_u16(&mut buffer, self.header_length);
        bytes::LittleEndian::write_u32(&mut buffer[2..6], self.content_length);
        Bytes::from(buffer)
    }
}

impl From<Vec<u8>> for NeedleHeader {
    fn from(v: Vec<u8>) -> NeedleHeader {
        assert!(v.len() >= 6);
        NeedleHeader {
            header_length: bytes::LittleEndian::read_u16(&v[0..2]),
            content_length: bytes::LittleEndian::read_u32(&v[2..6]),
        }
    }
}

impl IntoIterator for Needle {
    type Item = Result<Bytes>;

    type IntoIter = NeedleIterator;

    fn into_iter(self) -> Self::IntoIter {
        NeedleIterator {
            start_reading: false,
            needle: self,
        }
    }
}

impl Iterator for NeedleIterator {
    type Item = Result<Bytes>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.start_reading {
            self.start_reading = true;
            return Some(Ok(self.needle.header.as_bytes()));
        }
        match &self.needle.body {
            NeedleBody::SinglePart(data) => Some(Ok(data.clone())),
            NeedleBody::MultiParts(receiver) => receiver.recv().ok(),
        }
    }
}
