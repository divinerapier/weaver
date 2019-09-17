use std::sync::mpsc::Receiver;

use byteorder::ByteOrder;
use bytes::Bytes;
use futures::stream::{self, Stream};
use futures::{Async, Poll};

use crate::error::{Error, Result};

pub struct Needle {
    pub header: NeedleHeader,
    pub body: NeedleBody,
}

impl Needle {
    pub fn body_length(&self) -> u32 {
        self.header.size
    }
    pub fn total_length(&self) -> u32 {
        26 + self.body_length()
    }
}

pub struct NeedleIterator {
    reading_started: bool,
    reading_finished: bool,
    needle: Needle,
}

pub struct NeedleHeader {
    pub magic_number: u32,
    pub cookie: u32,
    pub needle_id: u64,
    pub alternate_key: u32,
    pub flags: u16,
    pub size: u32,
}

impl Default for NeedleHeader {
    fn default() -> NeedleHeader {
        NeedleHeader {
            magic_number: 0,
            cookie: 0,
            needle_id: 0,
            alternate_key: 0,
            flags: 0,
            size: 0,
        }
    }
}

pub enum NeedleBody {
    SinglePart(Bytes),
    MultiParts(Receiver<Result<Bytes>>),
}

impl NeedleHeader {
    #[inline]
    pub fn length() -> usize {
        26
    }

    pub fn as_bytes(&self) -> Bytes {
        let mut buffer = Vec::with_capacity(Self::length());
        buffer.resize(Self::length(), 0);
        bytes::LittleEndian::write_u32(&mut buffer[0..4], self.magic_number);
        bytes::LittleEndian::write_u32(&mut buffer[4..8], self.cookie);
        bytes::LittleEndian::write_u64(&mut buffer[8..16], self.needle_id);
        bytes::LittleEndian::write_u32(&mut buffer[16..20], self.alternate_key);
        bytes::LittleEndian::write_u16(&mut buffer[20..22], self.flags);
        bytes::LittleEndian::write_u32(&mut buffer[22..26], self.size);
        Bytes::from(buffer)
    }
}

impl From<Vec<u8>> for NeedleHeader {
    fn from(v: Vec<u8>) -> NeedleHeader {
        assert!(v.len() >= 4);
        NeedleHeader {
            magic_number: bytes::LittleEndian::read_u32(&v[0..4]),
            cookie: bytes::LittleEndian::read_u32(&v[4..8]),
            needle_id: bytes::LittleEndian::read_u64(&v[8..16]),
            alternate_key: bytes::LittleEndian::read_u32(&v[16..20]),
            flags: bytes::LittleEndian::read_u16(&v[20..22]),
            size: bytes::LittleEndian::read_u32(&v[22..26]),
        }
    }
}

impl IntoIterator for Needle {
    type Item = Result<Bytes>;

    type IntoIter = NeedleIterator;

    fn into_iter(self) -> Self::IntoIter {
        NeedleIterator {
            reading_started: false,
            reading_finished: false,
            needle: self,
        }
    }
}

impl Iterator for NeedleIterator {
    type Item = Result<Bytes>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reading_finished {
            return None;
        }
        if !self.reading_started {
            self.reading_started = true;
            return Some(Ok(self.needle.header.as_bytes()));
        }
        match &self.needle.body {
            NeedleBody::SinglePart(data) => {
                self.reading_finished = true;
                Some(Ok(data.clone()))
            }
            NeedleBody::MultiParts(receiver) => receiver.recv().ok(),
        }
    }
}

pub struct NeedleStream {
    iter: NeedleIterator,
}

impl From<Needle> for NeedleStream {
    fn from(n: Needle) -> NeedleStream {
        NeedleStream {
            iter: n.into_iter(),
        }
    }
}

impl Stream for NeedleStream {
    type Item = bytes::Bytes;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.iter.next() {
            Some(data) => match data {
                Ok(data) => Ok(Async::Ready(Some(data))),
                Err(e) => Err(*e),
            },
            None => Ok(Async::Ready(None)),
        }
    }
}
