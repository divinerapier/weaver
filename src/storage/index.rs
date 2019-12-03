#[allow(dead_code)]
use crate::error::Result;

use std::collections::HashMap;
use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};

use byteorder::ByteOrder;

pub trait Codec: Send + Sync {
    fn encode(&self, entry: &Entry) -> Result<Vec<u8>>;
    fn decode(&self, data: &[u8]) -> Result<Entry>;
}

pub struct JSONCodec;

impl Codec for JSONCodec {
    fn encode(&self, entry: &Entry) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(entry)?)
    }
    fn decode(&self, data: &[u8]) -> Result<Entry> {
        Ok(serde_json::from_slice(data)?)
    }
}

pub struct BinaryCodec;

impl Codec for BinaryCodec {
    fn encode(&self, entry: &Entry) -> Result<Vec<u8>> {
        let mut data = vec![0; 24];
        bytes::BigEndian::write_u64(&mut data[0..8], entry.needle_id);
        bytes::BigEndian::write_u64(&mut data[8..16], entry.offset as u64);
        bytes::BigEndian::write_u64(&mut data[16..24], entry.length as u64);
        Ok(data)
    }
    fn decode(&self, data: &[u8]) -> Result<Entry> {
        assert_eq!(data.len(), 24);
        Ok(Entry {
            needle_id: bytes::BigEndian::read_u64(&data[0..8]),
            offset: bytes::BigEndian::read_u64(&data[8..16]) as usize,
            length: bytes::BigEndian::read_u64(&data[16..24]) as usize,
        })
    }
}

pub struct Index<C: Codec> {
    writer: Arc<Mutex<std::fs::File>>,
    indexes: Arc<RwLock<HashMap<u64, Entry>>>,
    last_needle: AtomicU64,
    codec: C,
}

impl<C: Codec> Index<C> {
    pub fn new<P: AsRef<std::path::Path>>(path: P, codec: C) -> Result<Index<C>> {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(path.as_ref())?;
        let mut indexes = HashMap::new();
        let mut last_needle = 0;
        let reader = file.try_clone()?;

        let indexes_reader = serde_json::Deserializer::from_reader(reader).into_iter::<Entry>();
        for index_result in indexes_reader {
            let entry = index_result?;
            indexes.insert(entry.needle_id, entry);
            last_needle = entry.needle_id;
        }

        Ok(Index {
            writer: Arc::new(Mutex::new(file)),
            indexes: Arc::new(RwLock::new(indexes)),
            last_needle: AtomicU64::new(last_needle),
            codec,
        })
    }

    pub fn write(&self, entry: &Entry) -> Result<()> {
        {
            let data = self.codec.encode(entry).unwrap();
            let mut w = self.writer.lock().unwrap();
            w.write_all(&data)?;
        }
        {
            let mut indexes = self.indexes.write().unwrap();
            indexes.insert(entry.needle_id, entry.clone());
            self.last_needle.store(entry.needle_id, Ordering::SeqCst);
        }
        Ok(())
    }

    pub fn read(&self, needle_id: u64) -> Result<Entry> {
        let indexes = self.indexes.read().unwrap();
        match indexes.get(&needle_id) {
            Some(entry) => Ok(entry.clone()),
            None => Err(storage_error!("not found needle: {}", needle_id)),
        }
    }

    pub fn last_index(&self) -> Result<Option<Entry>> {
        let last_needle = self.last_needle.load(Ordering::SeqCst);
        if last_needle == 0 {
            return Ok(None);
        }
        self.read(last_needle).map(|entry| Some(entry))
    }
}

#[derive(Copy, Clone, serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct Entry {
    pub needle_id: u64,
    pub offset: usize,
    pub length: usize,
}

impl Entry {
    pub fn new(needle_id: u64, offset: usize, length: usize) -> Entry {
        Entry {
            needle_id,
            offset,
            length,
        }
    }
}

impl Default for Entry {
    fn default() -> Entry {
        Entry {
            needle_id: 0,
            offset: 0,
            length: 0,
        }
    }
}
