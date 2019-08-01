use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

pub struct Storage {
    pub volumes: Vec<Volume>,
}

pub struct Volume {
    filepath: String,
    volume: File,
    indexes: HashMap<String, Index>,
}

pub struct Needle<B> {
    body: B,
    length: usize,
}

pub struct Index {
    offset: usize,
    length: usize,
}

impl Storage {
    /// new opens the storage by specified path
    /// and also loads the indexes
    pub fn new<P>(path: P) -> Storage
    where
        P: AsRef<Path>,
    {
        unimplemented!()
    }

    /// upload appends the body to any avaiable volume and
    /// then records the offset and body size to index file
    pub fn upload<K, B>(&mut self, key: K, body: Needle<B>)
    where
        K: Into<String>,
        B: std::io::Read,
    {
        unimplemented!()
    }

    pub fn download<K, B>(&self, key: K) -> Needle<B>
    where
        K: Into<String>,
        B: std::io::Write,
    {
        unimplemented!()
    }
}
