use std::collections::{HashMap, HashSet};
use std::fs::Metadata;
use std::path::{Path, PathBuf};

use crate::error::Result;
use crate::needle::Needle;
use volume::Volume;

pub mod volume;

/// Store consists of many volumes.
pub struct Store {
    pub volumes_dir: PathBuf,
    pub volumes: HashMap<u64, Volume>,

    // TODO: use a min-heap to store volumes? tuple(id, remain_length)
    pub writable_volumes: HashSet<usize>,
    pub readonly_volumes: HashSet<usize>,
}

impl Store {
    pub fn new(dir: &str) -> Result<Store> {
        let dir_result = std::fs::read_dir(dir)?;

        let mut index_files = vec![];

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
                    // file_name, eg: 1.index
                    index_files.push(entry.file_name());
                } else {
                    log::warn!("open store. skip entry: {:?}", entry);
                }
            } else {
                log::error!("open store. skip entry: {:?}", entry);
            }
        }

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
                (index, volume_result)
            })
            .filter(|(index, volume_result)| volume_result.is_ok())
            .map(|(index, volume_result)| (index, volume_result.unwrap()))
            .collect::<HashMap<u64, Volume>>();

        Ok(Store {
            volumes_dir: PathBuf::from(dir),
            volumes,
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
