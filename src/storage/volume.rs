use crate::error::{self, Error, Result};
use crate::index::{Index, RawIndex};
use crate::needle::{Needle, NeedleBody, NeedleHeader};
use crate::utils;

use std::collections::HashMap;
use std::error::Error as StdError;
use std::ffi::OsStr;
use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use serde::Serialize;
use serde_json::Deserializer;

pub enum VolumeExtension {
    Index = 1,
    Data = 2,
    Unknown = 99,
}

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

#[derive(Serialize, Debug)]
pub struct Volume {
    pub id: u32,
    pub volume_path: String,
    #[serde(skip_serializing)]
    pub writable_volume: File,
    #[serde(skip_serializing)]
    pub readonly_volume: File,
    pub current_length: u64,
    pub max_length: u64,
    #[serde(skip_serializing)]
    pub index_file: File,
    pub indexes: HashMap<u64, RawIndex>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ReplicaReplacement {
    pub diff_data_centers: u8,
    pub diff_rack: u8,
    pub diff_node: u8,
}

impl ReplicaReplacement {
    pub fn count(&self) -> usize {
        self.diff_data_centers as usize * self.diff_node as usize * self.diff_rack as usize
    }
}

pub struct SuperBlock {
    pub replica_replacement: ReplicaReplacement,
}

impl Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Volume {
    pub fn new(dir: &Path, id: u32, size: u64) -> Result<Volume> {
        let volume_path: PathBuf = dir.join(format!("{}.data", id));
        let index_path: PathBuf = dir.join(format!("{}.index", id));
        if volume_path.exists() {
            log::error!(
                "couldn't create the volume data which exists already. id: {}, path: {}",
                id,
                volume_path.display()
            );
            return Err(boxed_volume_create!(id, "exists"));
        }
        if index_path.exists() {
            log::error!(
                "couldn't create the volume index which exists already. id: {}, path: {}",
                id,
                index_path.display()
            );
            return Err(boxed_index_create!(id, "exists"));
        }
        let (index_file, index_map, _) = Self::open_indexes(index_path, true)?;
        let (readonly_file, writable_file) = Self::open_volumes(&volume_path, true)?;
        let current_length = writable_file.metadata()?.len();
        Ok(Volume {
            id,
            volume_path: volume_path
                .to_str()
                .ok_or(naive!("{:?} to string", volume_path))?
                .to_owned(),
            writable_volume: writable_file,
            readonly_volume: readonly_file,
            current_length,
            max_length: size,
            index_file,
            indexes: index_map,
        })
    }

    /// open a physical volume file from disk
    /// volume_path is the
    pub fn open(volume_path: &Path, size: u64) -> Result<Volume> {
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
        let id = Self::parse_volume_file_stem_name(volume_path)?;
        let extension_str = extension.to_str().ok_or(Error::parse(
            "std::path::Path",
            "&str",
            format!("{:?}", extension),
        ))?;
        let naive_volume_path_str = utils::strings::trim_suffix(volume_path_str, extension_str)?;
        let index_file_str = naive_volume_path_str.to_owned() + "index";
        let volume_file_str = naive_volume_path_str.to_owned() + "data";

        let (index_file, index_map, last_index) = Self::open_indexes(index_file_str, false)?;
        let (readonly_file, writable_file) = Self::open_volumes(volume_file_str, false)?;
        let current_length = writable_file.metadata()?.len();
        if current_length != (last_index.offset + last_index.length) as u64 {
            log::error!(
                "volume data corruption. path: {}, current_length: {}, last_index.offset: {}, last_index.length: {}",
                volume_path.display(),
                current_length,
                last_index.offset,
                last_index.length
            );
            return Err(Error::volume(error::VolumeError::data_corruption(
                id,
                format!(
                    "volume current length: {}, last_index.offset: {}, last_index.length: {}",
                    current_length, last_index.offset, last_index.length
                ),
            )));
        }
        Ok(Volume {
            id,
            volume_path: volume_path
                .to_str()
                .ok_or(naive!("{:?} to string", volume_path))?
                .to_owned(),
            writable_volume: writable_file,
            readonly_volume: readonly_file,
            current_length,
            max_length: size,
            index_file,
            indexes: index_map,
        })
    }

    fn parse_volume_file_stem_name(volume_path: &Path) -> Result<u32> {
        let file_stem = volume_path
            .file_stem()
            .ok_or(Error::path(format!("get file_stem from {:?}", volume_path)))?;
        let file_stem_str = file_stem.to_str().ok_or(Error::parse(
            "&std::ffi::OsStr",
            "&str",
            format!("{:?}", file_stem),
        ))?;
        let id = file_stem_str.parse::<u32>()?;
        Ok(id)
    }

    pub fn writable(&self) -> bool {
        self.current_length < self.max_length
    }

    pub fn readonly(&self) -> bool {
        !self.writable()
    }

    pub fn available_length(&self) -> u64 {
        if self.max_length > self.current_length {
            self.max_length - self.current_length
        } else {
            0
        }
    }

    pub fn info(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn pretty_info(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    fn open_volumes<P: AsRef<Path>>(volume_filepath: P, new: bool) -> Result<(File, File)> {
        let writable_file: File = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(new)
            .create_new(new)
            .truncate(false)
            .open(volume_filepath.as_ref())
            .expect(&format!("volume filepath: {:?}", volume_filepath.as_ref()));
        let readonly_file = writable_file.try_clone().expect(&format!(
            "try clone volume filepath: {:?}",
            volume_filepath.as_ref()
        ));

        Ok((readonly_file, writable_file))
    }

    fn open_indexes<P: AsRef<Path>>(
        filepath: P,
        new: bool,
    ) -> Result<(File, HashMap<u64, RawIndex>, RawIndex)> {
        let index_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(new)
            .create_new(new)
            .truncate(false)
            .append(true)
            .open(filepath.as_ref())?;

        let mut index_map = HashMap::new();

        if new {
            return Ok((index_file, index_map, RawIndex::default()));
        }
        let volume_id: u32 = Self::parse_volume_file_stem_name(filepath.as_ref())?;
        let mut readonly_index_file = index_file.try_clone()?;
        readonly_index_file.seek(SeekFrom::Start(0))?;
        let reader = std::io::BufReader::new(readonly_index_file);
        let indexes_reader = Deserializer::from_reader(reader).into_iter::<Index>();
        let mut last_index = RawIndex::default();
        for index_result in indexes_reader {
            let index: Index = index_result?;
            let raw_index = RawIndex::new(volume_id, index.offset, index.length);
            last_index = raw_index;
            index_map.insert(index.needle_id, raw_index);
        }

        Ok((index_file, index_map, last_index))
    }

    fn can_write(&self, _length: u64) -> bool {
        return self.writable();
        // if self.readonly() {
        //     return false;
        // }
        // let length_after_write = self.current_length + length;
        // return length_after_write <= self.max_length;
    }

    pub fn write_needle(&mut self, needle_id: u64, needle: Needle) -> Result<()> {
        let actual_data_length = needle.actual_length() as usize;
        let total_length = needle.total_length() as usize;
        if !self.can_write(total_length as u64) {
            log::error!(
                "couldn't write to the volume. id: {}, path: {}, writable: {}, max_length: {}, current_length: {}, actual_data_length: {}, total_length: {}",
                self.id,
                self.volume_path,
                self.writable(),
                self.max_length,
                self.current_length,
               actual_data_length,
               total_length
            );
            return Err(Error::volume(error::VolumeError::overflow(
                self.id,
                self.max_length,
                self.current_length,
                total_length as u64,
            )));
        }
        let mut received_length = 0usize;
        let mut writable_volume = self.writable_volume.try_clone()?;
        writable_volume.seek(SeekFrom::Start(self.current_length))?;

        let mut writer = BufWriter::new(writable_volume);

        let needle_iter = needle.into_iter();
        for data in needle_iter {
            let data = data?;
            log::debug!("data: {:?}", data);
            received_length += data.len();
            writer.write_all(data.as_ref())?;
        }

        writer.flush()?;

        if received_length != actual_data_length {
            log::error!(
                "mismatched needle length. received: {}, actual: {}, total: {}",
                received_length,
                actual_data_length,
                total_length,
            );
            return Err(Error::volume(error::VolumeError::write_length_mismatch(
                self.id,
                needle_id.to_string(),
                actual_data_length,
                received_length,
            )));
        }

        // write index
        // TODO: supports write-ahead log

        let index = Index::new(
            needle_id,
            self.id,
            self.current_length as usize,
            total_length,
        );
        self.index_file
            .write_all(serde_json::to_string(&index)?.as_bytes())?;
        self.current_length += total_length as u64;
        self.writable_volume.set_len(self.current_length)?;
        self.indexes.insert(
            needle_id,
            RawIndex::new(index.volume_id, index.offset, index.length),
        );
        Ok(())
    }

    pub fn get(&self, needle_id: u64) -> Result<Needle> {
        let index: RawIndex = self
            .indexes
            .get(&needle_id)
            .ok_or(Error::not_found(format!(
                "needle not in indexes: {}, indexes: {:?}",
                needle_id, self.indexes
            )))?
            .clone();
        log::debug!("index: {:?}", index);
        if ((index.offset + index.length) as u64) > self.current_length {
            log::error!(
                "volume data corruption. needle: {}, volume_length: {}, index.offset: {}, index.length: {}",
                needle_id,
                self.current_length,
                index.offset,
                index.length
            );
            return Err(Error::data_corruption(
                needle_id.to_string(),
                "index out of current length",
            ));
        }
        Ok(self.read_needle(&index)?)
    }

    pub fn read_needle_header(file: &mut File, offset: usize) -> Result<NeedleHeader> {
        let mut buffer = Vec::with_capacity(26);
        buffer.resize(26, 0);
        file.seek(std::io::SeekFrom::Start(offset as u64))?;
        file.read_exact(&mut buffer)?;
        // automatically provides the implementation of [`Into`]
        Ok(buffer.into())
    }

    pub fn read_needle_body() {}

    pub fn read_needle(&self, index: &RawIndex) -> Result<Needle> {
        let mut readonly_volume = self.readonly_volume.try_clone()?;
        let needle_header = Self::read_needle_header(&mut readonly_volume, index.offset)?;
        log::debug!("header: {:?}", needle_header);
        let body_size = needle_header.body_length();
        let batch_size = if body_size > 1024 * 1024 {
            1024 * 1024 // 1M
        } else {
            body_size as usize
        };
        readonly_volume.seek(std::io::SeekFrom::Start(
            index.offset as u64 + needle_header.length() as u64,
        ))?;
        let mut buffer = Vec::with_capacity(batch_size);
        buffer.resize(batch_size, 0 as u8);
        let mut readonly_volume = self.readonly_volume.try_clone()?;
        if index.length <= 8 {
            readonly_volume.read_exact(&mut buffer)?;
            return Ok(Needle::new(
                needle_header,
                NeedleBody::SinglePart(bytes::Bytes::from(buffer)),
                8,
            ));
        }
        // TODO: using thread pool
        let (tx, rx) = std::sync::mpsc::sync_channel(1);
        let mut remains = body_size;
        std::thread::spawn(move || {
            while remains > 0 {
                let current = match readonly_volume.read(&mut buffer) {
                    Ok(current) => current,
                    Err(e) => {
                        // NOTE: This function will never panic, but it may return [`Err`]
                        // if the [`Receiver`] has disconnected and is no longer able to
                        // receive information.
                        log::error!("failed to read from volume. error: {:?}", e);
                        match tx.send(Err(Error::io(e))) {
                            Ok(_) => {}
                            Err(send_error) => {
                                log::error!("failed to send error into channel. {:?}", send_error);
                            }
                        }
                        return;
                    }
                };
                remains -= current as u32;
                match tx.send(Ok(bytes::Bytes::from(&buffer[0..current]))) {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("failed to read from volume. error: {:?}", e);
                        match tx.send(Err(Error::channel(e.description()))) {
                            Ok(_) => {}
                            Err(send_error) => {
                                log::error!("failed to send error into channel. {:?}", send_error);
                            }
                        }
                        return;
                    }
                }
            }
        });
        // TODO: read needle footer
        Ok(Needle::new(needle_header, NeedleBody::MultiParts(rx), 8))
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
            Index::new(0, 1, 0, 10),
            Index::new(1, 1, 10, 20),
            Index::new(2, 1, 20, 30),
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
