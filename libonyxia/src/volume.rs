use crate::error::{self, Error, Result};
use crate::index::{Index, RawIndex};
use crate::needle::Needle;
use crate::utils;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek};
use std::path::{Path, PathBuf};

use serde_json::Deserializer;

pub enum VolumeExtension {
    Index = 1,
    Data = 2,
    Unknown = 99,
}

const MAX_VOLUME_SIZE: u64 = 1 * (2 << 30);

impl From<&str> for VolumeExtension {
    fn from(t: &str) -> VolumeExtension {
        if t == "index" {
            VolumeExtension::Index
        } else if t == "data" {
            VolumeExtension::Data
        } else {
            VolumeExtension::Unknown
        }
    }
}

impl From<&OsStr> for VolumeExtension {
    fn from(t: &OsStr) -> VolumeExtension {
        match t.to_str() {
            None => VolumeExtension::Unknown,
            Some(s) => VolumeExtension::from(s),
        }
    }
}

#[allow(dead_code)]
pub struct Volume {
    pub volume_path: String,
    pub writable_volume: File,
    pub readonly_volume: File,
    pub current_length: u64,
    pub max_length: u64,
    pub index_file: File,
    pub indexes: HashMap<String, RawIndex>,
}

impl Volume {
    pub fn new(dir: &Path, index: usize) -> Result<Volume> {
        let volume_path: PathBuf = dir.join(format!("{}.data", index));
        let index_path: PathBuf = dir.join(format!("{}.index", index));
        if volume_path.exists() {
            return Err(Error::volume(error::VolumeError::create(
                index,
                "already exists",
            )));
        }
        if index_path.exists() {
            return Err(Error::index(error::IndexError::create(
                index,
                "already exists",
            )));
        }
        let (index_file, index_map) = Self::open_indexes(index_path, true)?;
        let (readonly_file, writable_file) = Self::open_volumes(&volume_path, false)?;
        let current_length = writable_file.metadata()?.len();
        Ok(Volume {
            volume_path: volume_path
                .to_str()
                .ok_or(format!("{:?} to string", volume_path))?
                .to_owned(),
            writable_volume: writable_file,
            readonly_volume: readonly_file,
            current_length,
            max_length: MAX_VOLUME_SIZE,
            index_file,
            indexes: index_map,
        })
    }

    /// open a physical volume file from disk
    /// volume_path is the
    pub fn open(volume_path: &Path) -> Result<Volume> {
        let extension = volume_path.extension().ok_or(Error::path(
            format! {"get file_stem from {:?}", volume_path},
        ))?;
        if extension != "index" {
            return Err(Box::new(Error::OpenVolume));
        }
        // filename should be a usize number
        let volume_path_str = volume_path.to_str().ok_or(Error::parse(
            "std::path::Path",
            "&str",
            format!("{:?}", volume_path),
        ))?;
        let _filename = Self::parse_volume_file_stem_name(volume_path)?;
        let extension_str = extension.to_str().ok_or(Error::parse(
            "std::path::Path",
            "&str",
            format!("{:?}", extension),
        ))?;
        let naive_volume_path_str = utils::strings::trim_suffix(volume_path_str, extension_str)?;
        let index_file_str = naive_volume_path_str.to_owned() + "index";
        let volume_file_str = naive_volume_path_str.to_owned() + "data";

        let (index_file, index_map) = Self::open_indexes(index_file_str, false)?;
        let (readonly_file, writable_file) = Self::open_volumes(volume_file_str, false)?;
        let current_length = writable_file.metadata()?.len();
        Ok(Volume {
            volume_path: volume_path
                .to_str()
                .ok_or(Error::naive(format!("{:?} to string", volume_path)))?
                .to_owned(),
            writable_volume: writable_file,
            readonly_volume: readonly_file,
            current_length,
            max_length: MAX_VOLUME_SIZE,
            index_file,
            indexes: index_map,
        })
    }

    fn parse_volume_file_stem_name(volume_path: &Path) -> Result<usize> {
        let stem = volume_path
            .file_stem()
            .ok_or(Error::path(format!("get file_stem from {:?}", volume_path)))?;
        let stem = stem.to_str().ok_or(Error::parse(
            "&std::ffi::OsStr",
            "&str",
            format!("{:?}", stem),
        ))?;
        let stem = stem.parse::<usize>()?;
        Ok(stem)
    }

    pub fn writable(&self) -> bool {
        self.current_length < self.max_length
    }

    pub fn readonly(&self) -> bool {
        !self.writable()
    }

    fn open_volumes<P: AsRef<Path>>(volume_filepath: P, new: bool) -> Result<(File, File)> {
        let writable_file: File = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(new)
            .create_new(new)
            .truncate(false)
            .open(volume_filepath.as_ref())?;
        let readonly_file: File = OpenOptions::new()
            .read(true)
            .write(false)
            .append(false)
            .create(new)
            .create_new(new)
            .truncate(false)
            .open(volume_filepath)?;

        Ok((readonly_file, writable_file))
    }

    fn open_indexes<P: AsRef<Path>>(
        filepath: P,
        new: bool,
    ) -> Result<(File, HashMap<String, RawIndex>)> {
        let index_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(new)
            .create_new(new)
            .truncate(false)
            .append(true)
            .open(filepath)?;

        let mut index_map = HashMap::new();

        if new {
            return Ok((index_file, index_map));
        }

        let reader = std::io::BufReader::new(index_file.try_clone()?);
        let indexes_reader = Deserializer::from_reader(reader).into_iter::<Index>();
        for index_result in indexes_reader {
            let index: Index = index_result?;
            let raw_index = RawIndex::new(index.offset, index.length);
            index_map.insert(index.path, raw_index);
        }

        Ok((index_file, index_map))
    }

    pub fn get<K>(&mut self, key: K) -> Result<Needle>
    where
        K: Into<String>,
    {
        let key = key.into();
        let index: &RawIndex = self
            .indexes
            .get(&key)
            .ok_or(Error::not_found(format!("not in indexes: {}", key)))?;
        if ((index.offset + index.length) as u64) < self.current_length {
            return Err(Error::data_corruption(key, "index out of current length"));
        }
        self.readonly_volume
            .seek(std::io::SeekFrom::Start(index.offset as u64))?;
        let mut buffer = Vec::with_capacity(index.length);
        buffer.resize(index.length, 0 as u8);
        self.readonly_volume.read_exact(&mut buffer)?;
        Ok(Needle {
            body: bytes::Bytes::from(buffer),
            length: index.length,
        })
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn read_json_from_file() {
        use super::Index;
        use serde_json::Deserializer;
        use std::fs::File;
        use std::io::Seek;
        use std::io::Write;

        let mut file: File = tempfile::tempfile().unwrap();

        let indexes: Vec<Index> = vec![
            Index::new("/tmp/file1".to_owned(), 0, 10),
            Index::new("/tmp/file2".to_owned(), 10, 20),
            Index::new("/tmp/file3".to_owned(), 20, 30),
        ];

        for index in &indexes {
            let value = serde_json::to_string(index).unwrap();
            file.write(value.as_bytes()).unwrap();
            file.sync_all().unwrap();
        }
        file.seek(std::io::SeekFrom::Start(0)).unwrap();
        let reader = std::io::BufReader::new(file.try_clone().unwrap());
        let mut result = vec![];
        let indexes_reader = Deserializer::from_reader(reader).into_iter::<Index>();
        for index_result in indexes_reader {
            let index: Index = index_result.unwrap();
            result.push(index);
        }
        assert_eq!(indexes.len(), result.len());
        assert_eq!(indexes[0], result[0]);
        for (i, _) in indexes.iter().enumerate() {
            assert_eq!(result[i], indexes[i]);
        }
    }

}
