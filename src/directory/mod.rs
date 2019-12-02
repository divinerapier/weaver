use crate::Result;
use std::collections::HashMap;
use std::marker::{Send, Sync};

pub mod server;

#[derive(Clone)]
pub struct Chunk {
    pub id: u64,
    pub volume_id: u64,
    pub needle_id: u64,
}

/// Directory is a manager that keep the mapper from
/// filepath to its chunks
pub struct Directory<S>
where
    S: DirectoryStorage,
{
    storage: S,
}

#[tonic::async_trait]
pub trait DirectoryStorage: Send + Sync {
    async fn create(&mut self, key: &str, chunks: Vec<Chunk>) -> Result<()>;
    async fn update(&mut self, key: &str, chunks: Vec<Chunk>) -> Result<()>;
    async fn retrieve<'a>(&'a self, key: &str) -> Result<&'a Vec<Chunk>>;
    async fn delete(&mut self, key: &str) -> Result<()>;
}

impl<S> Directory<S>
where
    S: DirectoryStorage,
{
    pub fn new(storage: S) -> Directory<S> {
        Directory { storage }
    }
}

#[derive(Copy, Clone)]
pub enum FileType {
    File = 1,
    Dir = 2,
}

pub struct MemoryDirectoryStorage {
    /// Store all objects, including directories and regular files.
    /// The key of hashmap indicates the full path of the chunk.
    /// If the chunk is a directory, the value of the hash map will
    /// contain its children name.
    pub entries: HashMap<String, Vec<Chunk>>,
}

impl MemoryDirectoryStorage {
    pub fn new() -> MemoryDirectoryStorage {
        MemoryDirectoryStorage {
            entries: HashMap::new(),
        }
    }
}

#[tonic::async_trait]
impl DirectoryStorage for MemoryDirectoryStorage {
    async fn create(&mut self, key: &str, chunks: Vec<Chunk>) -> Result<()> {
        if self.entries.contains_key(key) {
            return Ok(());
        }
        self.entries.insert(key.to_owned(), chunks);
        Ok(())
    }

    async fn update(&mut self, key: &str, chunks: Vec<Chunk>) -> Result<()> {
        match self.entries.get_mut(key) {
            Some(ent) => {
                *ent = chunks;
                Ok(())
            }
            None => Err(directory_error!("entry not found. {}", key)),
        }
    }

    async fn retrieve<'a>(&'a self, key: &str) -> Result<&'a Vec<Chunk>> {
        match self.entries.get(key) {
            Some(entry) => Ok(entry),
            None => Err(directory_error!("entry not found. {}", key)),
        }
    }

    async fn delete(&mut self, key: &str) -> Result<()> {
        self.entries.remove(key);
        Ok(())
    }
}
