use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

pub use volume::Volume;

use crate::error::Result;
use crate::needle::Needle;

pub mod index;
pub mod service;
pub mod volume;

/// Storage consists of many volumes.
struct InnerStorage {
    /// location to store volumes
    pub directory: PathBuf,

    /// all volumes on storage
    /// volume id -> volume
    pub volumes: HashMap<u64, Volume<index::JSONCodec>>,

    /// public address
    pub ip: String,
    pub port: u16,

    pub writable_volumes: HashSet<u64>,
    pub readonly_volumes: HashSet<u64>,
}

impl InnerStorage {
    // Create a storage instance on the specified directory and network address.
    // Open if there are some volumes located at.
    pub async fn open(dir: &str, ip: &str, port: u16) -> Result<InnerStorage> {
        // make sure the directory exists
        if !Path::new(dir).exists() {
            std::fs::create_dir_all(dir)?;
        }

        let dir_result = std::fs::read_dir(dir)?;

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
            .map(|(idx, _index_file_name)| {
                let volume_result = Volume::open(dir, idx, 128, index::JSONCodec);
                (idx, volume_result)
            })
            .filter(|(_, volume_result)| volume_result.is_ok())
            .map(|(index, volume_result)| (index, volume_result.unwrap()))
            .collect::<HashMap<u64, Volume<index::JSONCodec>>>();

        let writable_volumes = volumes
            .iter()
            .filter(|(_, volume)| {
                let volume = volume;
                volume.writable()
            })
            .map(|(volume_id, _)| *volume_id)
            .collect::<HashSet<u64>>();
        let readonly_volumes = volumes
            .iter()
            .filter(|(_, volume)| {
                let volume = volume;
                !volume.writable()
            })
            .map(|(volume_id, _)| *volume_id)
            .collect::<HashSet<u64>>();

        Ok(InnerStorage {
            directory: PathBuf::from(dir),
            volumes,
            ip: ip.to_owned(),
            port,
            writable_volumes,
            readonly_volumes,
        })
    }
}

#[derive(Clone)]
pub struct Storage {
    inner: Arc<RwLock<InnerStorage>>,
}

impl Storage {
    // Create a storage instance on the specified directory and network address.
    // Open if there are some volumes located at.
    pub async fn open(dir: &str, ip: &str, port: u16) -> Result<Storage> {
        InnerStorage::open(dir, ip, port).await.map(|s| Storage {
            inner: Arc::new(RwLock::new(s)),
        })
    }

    pub async fn create_volume(
        &self,
        volume_id: u64,
        replica_replacement: &Option<volume::ReplicaReplacement>,
        max_volume_size: u32,
        max_needle_count: u32,
    ) -> Result<()> {
        let mut storage = self.inner.write().unwrap();
        if storage.volumes.contains_key(&volume_id) {
            return Err(error!("failed to create an exists volume {}", volume_id));
        }
        let super_block =
            volume::SuperBlock::new(&replica_replacement, max_volume_size, max_needle_count);
        let volume = Volume::new(
            &storage.directory,
            volume_id,
            max_volume_size as u64,
            &super_block,
            index::JSONCodec,
        )?;

        if volume.writable() {
            storage.writable_volumes.insert(volume_id);
        } else {
            storage.readonly_volumes.insert(volume_id);
        }
        storage.volumes.insert(volume_id, volume);

        Ok(())
    }

    pub async fn write_needle(&self, volume_id: u64, needle: &proto::weaver::Needle) -> Result<()> {
        let mut storage = self.inner.write().unwrap();
        match storage.volumes.get_mut(&volume_id) {
            Some(volume) => Ok(volume.write_needle2(needle)?),
            None => Err(storage_error!("volume not found: {}", volume_id)),
        }
    }

    pub fn read_needle(&self, volume_id: u64, needle_id: u64) -> Result<Needle> {
        let storage = self.inner.read().unwrap();
        if volume_id as usize >= storage.volumes.len() {
            // FIXME: index out of range
            return Err(error!("volume not found"));
        }
        let volume = &storage.volumes[&volume_id];
        volume.get(needle_id)
    }
}
