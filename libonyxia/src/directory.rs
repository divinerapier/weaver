use crate::needle::Needle;
use crate::volume::Volume;

use std::collections::{HashMap, HashSet};
use std::path::Path;

#[allow(dead_code)]
pub struct Directory {
    pub volumes: Vec<Volume>,
    pub writable_volumes: HashSet<usize>,
    pub readonly_volumes: HashSet<usize>,

    /// map from file path to volume index in self.volumes
    pub needle_map: HashMap<String, usize>,
}

#[allow(dead_code, unused)]
impl Directory {
    /// new opens the storage by specified path
    /// and also loads the indexes
    pub fn new<P>(path: P) -> std::io::Result<Directory>
    where
        P: AsRef<Path>,
    {
        let mut result = Directory::default();
        let dir: std::fs::ReadDir = std::fs::read_dir(path)?;
        for entry in dir {
            let entry = entry?;
            let inner_file_path: std::path::PathBuf = entry.path();
            Volume::open(inner_file_path.as_path()).map(|volume| {
                let volume: Volume = volume;
                let index = result.volumes.len();
                let writable = volume.writable();
                result.volumes.push(volume);
                if writable {
                    result.writable_volumes.insert(index);
                } else {
                    result.readonly_volumes.insert(index);
                }
                // TODO: optimize copying index
                let volume_ref: &Volume = result.volumes.get(index).unwrap();
                for (k, v) in &volume_ref.indexes {
                    result.needle_map.insert(k.to_owned(), index);
                }
            });
        }
        Ok(result)
    }

    /// upload appends the body to any avaiable volume and
    /// then records the offset and body size to index file
    pub fn upload<K>(&mut self, key: K, body: Needle)
    where
        K: Into<String>,
    {
        unimplemented!()
    }

    pub fn download<K>(&self, key: K) -> Option<Needle>
    where
        K: Into<String>,
    {
        let key = key.into();
        let index = self.needle_map.get(&key)?;
        let volume: &Volume = self.volumes.get(*index)?;
        unimplemented!()
    }
}

impl Default for Directory {
    fn default() -> Directory {
        Directory {
            volumes: vec![],
            writable_volumes: HashSet::new(),
            readonly_volumes: HashSet::new(),
            needle_map: HashMap::new(),
        }
    }
}
