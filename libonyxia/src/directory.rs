use crate::needle::Needle;
use crate::volume::{Volume, VolumeExtension};
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
        let dir: std::fs::ReadDir = std::fs::read_dir(path)?;
        for entry in dir {
            let entry = entry?;
            let inner_file_path: std::path::PathBuf = entry.path();
            if let Some(extension) = inner_file_path.extension() {
                if let VolumeExtension::Index = VolumeExtension::from(extension) {
                    // TODO: load indexes and open related physical volume
                }
            }
        }
        unimplemented!()
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

mod test {
    #[test]
    fn foo() {}
}
