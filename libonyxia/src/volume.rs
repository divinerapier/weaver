use crate::index::Index;
use crate::needle::Needle;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Seek};

pub enum VolumeExtension {
    Index = 1,
    Data = 2,
    Unknown = 99,
}

impl From<&str> for VolumeExtension {
    fn from(t: &str) -> VolumeExtension {
        if t == "index" {
            VolumeExtension::Index
        } else if t == "data" {
            VolumeExtension::Data
        } else {
            VolumeExtension::Unknown
        }
    }
}

impl From<&OsStr> for VolumeExtension {
    fn from(t: &OsStr) -> VolumeExtension {
        match t.to_str() {
            None => VolumeExtension::Unknown,
            Some(s) => VolumeExtension::from(s),
        }
    }
}

#[allow(dead_code)]
pub struct Volume {
    volume_path: String,
    physical_volume: File,
    current_length: usize,
    max_length: usize,
    indexes: HashMap<String, Index>,
}

impl Volume {
    pub fn get<K>(&mut self, key: K) -> Option<Needle>
    where
        K: Into<String>,
    {
        let key = key.into();
        let index: &Index = self.indexes.get(&key)?;
        if index.offset + index.length < self.current_length {
            // FIXME: should return an error
            return None;
        }
        // TODO: error should be handled instead of calling unwrap
        self.physical_volume
            .seek(std::io::SeekFrom::Start(index.offset as u64))
            .unwrap();
        let mut buffer = Vec::with_capacity(index.length);
        buffer.resize(index.length, 0 as u8);
        self.physical_volume.read_exact(&mut buffer).unwrap();
        Some(Needle {
            body: bytes::Bytes::from(buffer),
            length: index.length,
        })
    }
}
