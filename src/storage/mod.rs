use std::collections::{HashMap, HashSet};
use std::fs::Metadata;
use std::path::{Path, PathBuf};

use crate::error::Result;
use crate::needle::Needle;
use volume::Volume;

pub mod volume;

/// Storage consists of many volumes.
pub struct Storage {
    pub directory: PathBuf,

    /// all volumes on storage
    /// volume id -> volume
    pub volumes: HashMap<u64, Volume>,

    /// public address
    pub ip: String,
    pub port: u16,

    /// the max volume id of storage
    pub latest_volume: u64,

    pub writable_volumes: HashSet<u64>,
    pub readonly_volumes: HashSet<u64>,
}

impl Storage {
    pub fn new(dir: &str, ip: &str, port: u16) -> Result<Storage> {
        let dir_result = std::fs::read_dir(dir)?;
        let mut latest_volume = 0;

        let index_files = dir_result
            .filter_map(|entry| {
                let entry: std::fs::DirEntry = entry.ok()?;
                let metadata = entry.metadata().ok()?;
                if !metadata.is_file() || metadata.permissions().readonly() {
                    log::warn!(
                        "file in volume dir is readonly. {}/{}",
                        dir,
                        entry.file_name().to_str().unwrap()
                    );
                    None
                } else {
                    Some((entry, metadata))
                }
            })
            .filter_map(|(entry, _)| {
                if let Some(extension) = PathBuf::from(entry.file_name()).extension() {
                    if extension == "index" {
                        // file_name, eg: 1.index
                        // index_files.push(entry.file_name());
                        Some(entry.file_name())
                    } else {
                        log::warn!("open store. skip entry: {:?}", entry);
                        None
                    }
                } else {
                    log::error!("open store. skip entry: {:?}", entry);
                    None
                }
            })
            .collect::<Vec<std::ffi::OsString>>();

        let volumes = index_files
            .iter()
            .filter_map(|index_file_name| {
                let index_file_name = index_file_name.to_str()?;
                let char_at = index_file_name.find('.')?;
                let index_file_name: &str = index_file_name;
                let index: std::result::Result<usize, std::num::ParseIntError> =
                    index_file_name[0..char_at].parse::<usize>();
                let index = index.ok()?;
                Some((index as u64, index_file_name))
            })
            .map(|(index, index_file_name)| {
                let volume_result = Volume::open(&Path::new(dir).join(index_file_name), 128);
                if index > latest_volume {
                    latest_volume = index;
                }
                (index, volume_result)
            })
            .filter(|(_, volume_result)| volume_result.is_ok())
            .map(|(index, volume_result)| (index, volume_result.unwrap()))
            .collect::<HashMap<u64, Volume>>();

        Ok(Storage {
            directory: PathBuf::from(dir),
            volumes,
            ip: ip.to_owned(),
            port,
            latest_volume,
            writable_volumes: HashSet::new(),
            readonly_volumes: HashSet::new(),
        })
    }

    pub fn read_needle(&self, volume_id: u32, needle_id: u64) -> Result<Needle> {
        if volume_id as usize >= self.volumes.len() {
            // FIXME: index out of range
            return Err(boxed_naive!("volume not found"));
        }
        let volume: &Volume = &self.volumes[&(volume_id as u64)];
        volume.get(needle_id)
    }
}
