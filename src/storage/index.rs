#[allow(dead_code)]
use crate::error::Result;

use std::collections::HashMap;
use std::io::Write;
use std::sync::atomic::{AtomicPtr, Ordering};
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
    reader: std::fs::File,
    indexes: Arc<RwLock<HashMap<u64, Entry>>>,
    last: AtomicPtr<Option<Entry>>,
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
        let reader = file.try_clone()?;
        let cloned_file = file.try_clone()?;

        let mut last_index = None;

        let indexes_reader = serde_json::Deserializer::from_reader(reader).into_iter::<Entry>();
        for index_result in indexes_reader {
            let index = index_result?;
            indexes.insert(index.needle_id, index);
            last_index = Some(index);
        }

        Ok(Index {
            reader: cloned_file,
            writer: Arc::new(Mutex::new(file)),
            indexes: Arc::new(RwLock::new(indexes)),
            last: AtomicPtr::new(&mut last_index as *mut Option<Entry>),
            codec,
        })
    }

    pub fn write(&self, entry: &Entry) -> Result<()> {
        {
            let data = self.codec.encode(entry).unwrap();
            let mut w: std::sync::MutexGuard<'_, std::fs::File> = self.writer.lock().unwrap();
            w.write_all(&data)?;
        }
        {
            let mut indexes: std::sync::RwLockWriteGuard<
                '_,
                std::collections::HashMap<u64, Entry>,
            > = self.indexes.write().unwrap();
            indexes.insert(entry.needle_id, entry.clone());
            let mut entry = Some(entry.clone());
            self.last.store(&mut entry as *mut Option<Entry>, Ordering::SeqCst);
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

    pub fn last_index(&self) -> Box<Option<Entry>> {
        let entry: *mut Option<Entry> = self.last.load(Ordering::SeqCst);
        unsafe {
            Box::from_raw(entry)
        }
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
