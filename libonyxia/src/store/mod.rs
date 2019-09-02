use std::collections::{HashMap, HashSet};
use std::fs::Metadata;
use std::path::PathBuf;

use crate::error::{Error, Result};
use crate::store::volume::Volume;

pub mod volume;

/// Store consists of many volumes.
pub struct Store {
    pub volumes_dir: PathBuf,
    pub volumes: Vec<Volume>,

    // TODO: use a min-heap to store volumes? tuple(id, remain_length)
    pub writable_volumes: HashSet<usize>,
    pub readonly_volumes: HashSet<usize>,

    /// map from file path to volume index in self.volumes
    pub needle_map: HashMap<String, usize>,
}

impl Store {
    pub fn new(dir: &str) -> Result<Store> {
        let dir_result = std::fs::read_dir(dir)?;
        for entry in dir_result {
            let entry: std::fs::DirEntry = entry?;
            let metadata: Metadata = entry.metadata()?;
            if !metadata.is_file() {
                continue;
            }
            if metadata.permissions().readonly() {
                log::warn!(
                    "file in volume dir is readonly. {}/{}",
                    dir,
                    entry.file_name().to_str().unwrap(),
                );
            }
            // filter data files and open them
        }

        Ok(Store {
            volumes_dir: PathBuf::from(dir),
            volumes: Vec::new(),
            writable_volumes: HashSet::new(),
            readonly_volumes: HashSet::new(),
            needle_map: HashMap::new(),
        })
    }
}
