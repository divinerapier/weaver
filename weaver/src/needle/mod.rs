use std::sync::mpsc::Receiver;

use byteorder::ByteOrder;
use bytes::Bytes;
use futures::stream::Stream;

use crate::error::{Error, Result};

#[derive(Debug, Copy, Clone)]
pub struct NeedleHeader {
    magic_number: u32,
    cookie: u32,
    needle_id: u64,
    alternate_key: u32,
    flags: u16,
    size: u32,
}

pub struct NeedleFooter {
    magic_number: u32,
    check_sum: Option<u32>,

    // extra data needed alignes to 8 bytes
    padding: u32,
}

pub struct Needle {
    pub header: NeedleHeader,
    pub body: NeedleBody,
    pub footer: NeedleFooter,
}

pub struct NeedleIterator {
    started: bool,
    finished: bool,
    header: NeedleHeader,
    body: NeedleBodyIterator,
    footer: NeedleFooter,
}

pub enum NeedleBody {
    SinglePart(Bytes),
    MultiParts(Receiver<Result<Bytes>>),
}

pub struct NeedleBodyIterator {
    finished: bool,
    needle_body: NeedleBody,
}

impl NeedleHeader {
    pub fn new(needle_id: u64, size: u32) -> NeedleHeader {
        NeedleHeader {
            magic_number: srand::ThreadLocal::uint32(),
            cookie: srand::ThreadLocal::uint32(),
            needle_id,
            alternate_key: srand::ThreadLocal::uint32(),
            flags: 0,
            size,
        }
    }

    #[inline]
    pub fn length(&self) -> u32 {
        26
    }

    #[inline]
    pub fn body_length(&self) -> u32 {
        self.size
    }

    pub fn as_bytes(&self) -> Bytes {
        let mut buffer = Vec::with_capacity(self.length() as usize);
        buffer.resize(self.length() as usize, 0);
        bytes::LittleEndian::write_u32(&mut buffer[0..4], self.magic_number);
        bytes::LittleEndian::write_u32(&mut buffer[4..8], self.cookie);
        bytes::LittleEndian::write_u64(&mut buffer[8..16], self.needle_id);
        bytes::LittleEndian::write_u32(&mut buffer[16..20], self.alternate_key);
        bytes::LittleEndian::write_u16(&mut buffer[20..22], self.flags);
        bytes::LittleEndian::write_u32(&mut buffer[22..26], self.size);
        Bytes::from(buffer)
    }
}

impl NeedleFooter {
    pub fn new(header_length: u32, body_length: u32, align_to: u32) -> NeedleFooter {
        let current_length = header_length + body_length + 12;
        let remain_length = current_length % align_to;
        NeedleFooter {
            magic_number: srand::ThreadLocal::uint32(),
            check_sum: None,
            padding: align_to - remain_length,
        }
    }

    #[inline]
    pub fn length(&self) -> u32 {
        self.real_data_length() + self.padding
    }

    #[inline]
    fn real_data_length(&self) -> u32 {
        4 + 4 + 4
    }

    pub fn as_bytes(&self) -> Bytes {
        let mut buffer = Vec::with_capacity(self.real_data_length() as usize);
        buffer.resize(self.real_data_length() as usize, 0);
        // TODO: set magic number and compute check sum
        bytes::LittleEndian::write_u32(&mut buffer[0..4], 0);
        bytes::LittleEndian::write_u32(&mut buffer[4..8], 0);
        bytes::LittleEndian::write_u32(&mut buffer[8..12], self.padding);
        Bytes::from(buffer)
    }
}

impl From<Vec<u8>> for NeedleHeader {
    fn from(v: Vec<u8>) -> NeedleHeader {
        assert!(v.len() >= 26);
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

impl Needle {
    #[inline]
    pub fn header_length(&self) -> u32 {
        self.header.length()
    }

    #[inline]
    pub fn body_length(&self) -> u32 {
        self.header.size
    }

    pub fn actual_length(&self) -> u32 {
        log::debug!(
            "header: {}, body: {}, real footer: {}",
            self.header_length(),
            self.body_length(),
            self.footer.real_data_length()
        );
        self.header_length() + self.body_length() + self.footer.real_data_length()
    }

    #[inline]
    pub fn total_length(&self) -> u32 {
        self.header_length() + self.body_length() + self.footer.length()
    }

    pub fn new(header: NeedleHeader, body: NeedleBody, align_to: u32) -> Needle {
        let header_length = header.length();
        let body_length = header.size;
        Needle {
            header,
            body,
            footer: NeedleFooter::new(header_length, body_length, align_to),
        }
    }
}

impl IntoIterator for Needle {
    type Item = Result<Bytes>;

    type IntoIter = NeedleIterator;

    fn into_iter(self) -> Self::IntoIter {
        NeedleIterator {
            started: false,
            finished: false,
            header: self.header,
            body: self.body.into_iter(),
            footer: self.footer,
        }
    }
}

impl Iterator for NeedleIterator {
    type Item = Result<Bytes>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        if !self.started {
            self.started = true;
            return Some(Ok(self.header.as_bytes()));
        }
        match self.body.next() {
            Some(data) => Some(data),
            None => {
                self.finished = true;
                Some(Ok(self.footer.as_bytes()))
            }
        }
    }
}

impl Iterator for NeedleBodyIterator {
    type Item = Result<Bytes>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        match &self.needle_body {
            NeedleBody::SinglePart(data) => {
                self.finished = true;
                Some(Ok(data.clone()))
            }
            NeedleBody::MultiParts(receiver) => receiver.recv().ok(),
        }
    }
}

impl IntoIterator for NeedleBody {
    type Item = Result<Bytes>;
    type IntoIter = NeedleBodyIterator;

    fn into_iter(self) -> Self::IntoIter {
        NeedleBodyIterator {
            finished: false,
            needle_body: self,
        }
    }
}
