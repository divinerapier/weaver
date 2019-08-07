use crate::needle::Needle;
use crate::volume::Volume;

use crate::error::{self, Error, Result};

use std::collections::{HashMap, HashSet};
use std::ops::Try;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
pub struct Directory {
    pub volumes_dir: PathBuf,
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
    pub fn new<P>(path: P) -> Result<Directory>
    where
        P: AsRef<Path>,
    {
        let mut result = Directory::default();
        result.volumes_dir = PathBuf::from(path.as_ref());
        let dir: std::fs::ReadDir = std::fs::read_dir(path)?;
        for entry in dir {
            let entry = entry?;
            let inner_file_path: std::path::PathBuf = entry.path();
            Volume::open(inner_file_path.as_path()).map(|volume| -> Result<()> {
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
                let volume_ref: Result<&Volume> = result
                    .volumes
                    .get(index)
                    .ok_or(Error::not_found(format!("volume: {}", index)));
                let volume_ref = volume_ref?;
                for (k, v) in &volume_ref.indexes {
                    result.needle_map.insert(k.to_owned(), index);
                }
                Ok(())
            });
        }
        Ok(result)
    }

    /// write appends the body to any available volume and
    /// then records the offset and body size to index file
    pub fn write<K>(&mut self, path: K, body: Needle) -> Result<()>
    where
        K: Into<String>,
    {
        let volume_id =
            self.random_writable_volume()
                .into_result()
                .or_else(|_| -> Result<usize> {
                    let volume = Volume::new(&self.volumes_dir, self.volumes.len())?;
                    Ok(volume.id)
                })?;
        let volume: &mut Volume = self
            .volumes
            .get_mut(volume_id)
            .ok_or(Error::volume(error::VolumeError::not_found(volume_id)))?;
        let path = path.into();
        self.needle_map.insert(path.clone(), volume_id);
        volume.write_needle(&path, body)
    }

    pub fn read<K>(&self, key: K) -> Result<Needle>
    where
        K: Into<String>,
    {
        let key = key.into();
        let volume_id = self
            .needle_map
            .get(&key)
            .ok_or(Error::not_found(format!("path: {}", key)))?;
        let volume: &Volume = self
            .volumes
            .get(*volume_id)
            .ok_or(Error::not_found(format!(
                "path: {}, got volume id: {}",
                key, *volume_id
            )))?;
        volume.get(key)
    }

    fn random_writable_volume(&self) -> Option<usize> {
        if self.writable_volumes.len() == 0 {
            return None;
        }
        for i in self.writable_volumes.iter() {
            return Some(*i);
        }
        None
    }
}

impl Default for Directory {
    fn default() -> Directory {
        Directory {
            volumes_dir: PathBuf::default(),
            volumes: vec![],
            writable_volumes: HashSet::new(),
            readonly_volumes: HashSet::new(),
            needle_map: HashMap::new(),
        }
    }
}
