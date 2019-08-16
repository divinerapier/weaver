use std::collections::{HashMap, HashSet};
use std::fs::Metadata;
use std::path::{Path, PathBuf};

use crate::error::Result;
use crate::store::volume::Volume;
use crate::utils::size::Size;

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

        let mut data_files = vec![];

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
                continue;
            }
            // filter data files and open them
            // [`None`], if there is no file name;
            // [`None`], if there is no embedded `.`;
            // [`None`], if the file name begins with `.` and has no other `.`s within;
            // Otherwise, the portion of the file name after the final `.`
            if let Some(extension) = PathBuf::from(entry.file_name()).extension() {
                if extension == "index" {
                    data_files.push(entry.file_name());
                }
            } else {
                log::warn!("open store. skip entry: {:?}", entry);
            }
        }

        let volumes: Result<Vec<Volume>> = data_files
            .into_iter()
            .map(|file_name| Volume::open(&Path::new(dir).join(&file_name), Size::default()))
            .collect();

        let volumes: Vec<Volume> = volumes?;

        Ok(Store {
            volumes_dir: PathBuf::from(dir),
            volumes,
            writable_volumes: HashSet::new(),
            readonly_volumes: HashSet::new(),
            needle_map: HashMap::new(),
        })
    }
}
