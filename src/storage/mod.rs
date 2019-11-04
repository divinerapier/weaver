use std::collections::{HashMap, HashSet};
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
    pub latest_volume_id: u64,

    pub writable_volumes: HashSet<u64>,
    pub readonly_volumes: HashSet<u64>,
}

impl Storage {
    pub fn new(dir: &str, ip: &str, port: u16) -> Result<Storage> {
        let dir_result = std::fs::read_dir(dir)?;
        let mut latest_volume_id = 0;

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
                if index > latest_volume_id {
                    latest_volume_id = index;
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
            latest_volume_id,
            writable_volumes: HashSet::new(),
            readonly_volumes: HashSet::new(),
        })
    }

    // Create a storage instance on the specified directory and network address.
    // Check and open volumes if there are data.
    pub fn new2(dir: &str, ip: &str, port: u16) -> Result<Storage> {
        let dir_result = std::fs::read_dir(dir)?;
        let mut latest_volume_id = 0;

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
            .map(|(index, _index_file_name)| {
                let volume_result = Volume::open2(dir, index, 128);
                if index > latest_volume_id {
                    latest_volume_id = index;
                }
                (index, volume_result)
            })
            .filter(|(_, volume_result)| volume_result.is_ok())
            .map(|(index, volume_result)| (index, volume_result.unwrap()))
            .collect::<HashMap<u64, Volume>>();

        let writable_volumes = volumes
            .iter()
            .filter(|(_, volume)| {
                let volume: &Volume = volume;
                volume.writable()
            })
            .map(|(volume_id, _)| *volume_id)
            .collect::<HashSet<u64>>();
        let readonly_volumes = volumes
            .iter()
            .filter(|(_, volume)| {
                let volume: &Volume = volume;
                !volume.writable()
            })
            .map(|(volume_id, _)| *volume_id)
            .collect::<HashSet<u64>>();

        Ok(Storage {
            directory: PathBuf::from(dir),
            volumes,
            ip: ip.to_owned(),
            port,
            latest_volume_id,
            writable_volumes,
            readonly_volumes,
        })
    }

    pub fn create_volume(
        &mut self,
        volume_id: u64,
        replica_replacement: Option<volume::ReplicaReplacement>,
        max_volume_size: u32,
        max_needle_count: u32,
    ) -> Result<()> {
        if self.volumes.contains_key(&volume_id) {
            return Err(boxed_naive!(
                "failed to create an exists volume {}",
                volume_id
            ));
        }
        let super_block =
            volume::SuperBlock::new(replica_replacement, max_volume_size, max_needle_count);
        let volume = Volume::new2(&self.directory, volume_id as u32, 128, &super_block)?;

        if volume.writable() {
            self.writable_volumes.insert(volume_id);
        } else {
            self.readonly_volumes.insert(volume_id);
        }
        self.volumes.insert(volume_id, volume);

        Ok(())
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
