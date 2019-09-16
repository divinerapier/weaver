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
        self.header.body_length
    }
    pub fn total_length(&self) -> u32 {
        4 + self.body_length()
    }
}

pub struct NeedleIterator {
    reading_started: bool,
    reading_finished: bool,
    needle: Needle,
}

pub struct NeedleHeader {
    pub body_length: u32,
}

impl Default for NeedleHeader {
    fn default() -> NeedleHeader {
        NeedleHeader { body_length: 0 }
    }
}

pub enum NeedleBody {
    SinglePart(Bytes),
    MultiParts(Receiver<Result<Bytes>>),
}

impl NeedleHeader {
    pub fn as_bytes(&self) -> Bytes {
        let mut buffer = Vec::with_capacity(3);
        buffer.resize(4, 0);
        bytes::LittleEndian::write_u32(&mut buffer[0..4], self.body_length);
        Bytes::from(buffer)
    }
}

impl From<Vec<u8>> for NeedleHeader {
    fn from(v: Vec<u8>) -> NeedleHeader {
        assert!(v.len() >= 4);
        NeedleHeader {
            body_length: bytes::LittleEndian::read_u32(&v[0..4]),
            ..NeedleHeader::default()
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
        // if self.reading_finished {
        //     return Ok(Async::Ready(None));
        // }
        // if !self.reading_started {
        //     self.reading_started = true;
        //     return Ok(Async::Ready(Some(self.needle.header.as_bytes())));
        // }
        // match &self.needle.body {
        //     NeedleBody::SinglePart(data) => {
        //         self.reading_finished = true;
        //         Ok(Async::Ready(Some(data.clone())))
        //     }
        //     NeedleBody::MultiParts(receiver) => match receiver.recv().ok() {
        //         Some(data) => match data {
        //             Ok(data) => Ok(Async::Ready(Some(data))),
        //             Err(err) => Err(*err),
        //         },

        //         None => Ok(Async::Ready(None)),
        //     },
        // }
        match self.iter.next() {
            Some(data) => match data {
                Ok(data) => Ok(Async::Ready(Some(data))),
                Err(e) => Err(*e),
            },
            None => Ok(Async::Ready(None)),
        }
    }
}
