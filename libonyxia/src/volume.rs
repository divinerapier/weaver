use crate::error::{self, Error, Result};
use crate::index::{Index, RawIndex};
use crate::needle::{Needle, NeedleBody};
use crate::utils::{self, size::Size};

use std::collections::HashMap;
use std::error::Error as StdError;
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

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

#[allow(dead_code)]
pub struct Volume {
    pub id: usize,
    pub volume_path: String,
    pub writable_volume: File,
    pub readonly_volume: File,
    pub current_length: u64,
    pub max_length: u64,
    pub index_file: File,
    pub indexes: HashMap<String, RawIndex>,
}

impl Volume {
    pub fn new(dir: &Path, id: usize, size: Size) -> Result<Volume> {
        let volume_path: PathBuf = dir.join(format!("{}.data", id));
        let index_path: PathBuf = dir.join(format!("{}.index", id));
        if volume_path.exists() {
            return Err(Error::volume(error::VolumeError::create(
                id,
                "already exists",
            )));
        }
        if index_path.exists() {
            return Err(Error::index(error::IndexError::create(
                id,
                "already exists",
            )));
        }
        let (index_file, index_map, _) = Self::open_indexes(index_path, true)?;
        let (readonly_file, writable_file) = Self::open_volumes(&volume_path, false)?;
        let current_length = writable_file.metadata()?.len();
        Ok(Volume {
            id,
            volume_path: volume_path
                .to_str()
                .ok_or(format!("{:?} to string", volume_path))?
                .to_owned(),
            writable_volume: writable_file,
            readonly_volume: readonly_file,
            current_length,
            max_length: size.into(),
            index_file,
            indexes: index_map,
        })
    }

    /// open a physical volume file from disk
    /// volume_path is the
    pub fn open(volume_path: &Path, size: Size) -> Result<Volume> {
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
                .ok_or(Error::naive(format!("{:?} to string", volume_path)))?
                .to_owned(),
            writable_volume: writable_file,
            readonly_volume: readonly_file,
            current_length,
            max_length: size.into(),
            index_file,
            indexes: index_map,
        })
    }

    fn parse_volume_file_stem_name(volume_path: &Path) -> Result<usize> {
        let file_stem = volume_path
            .file_stem()
            .ok_or(Error::path(format!("get file_stem from {:?}", volume_path)))?;
        let file_stem_str = file_stem.to_str().ok_or(Error::parse(
            "&std::ffi::OsStr",
            "&str",
            format!("{:?}", file_stem),
        ))?;
        let id = file_stem_str.parse::<usize>()?;
        Ok(id)
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
    ) -> Result<(File, HashMap<String, RawIndex>, RawIndex)> {
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
            return Ok((index_file, index_map, RawIndex::default()));
        }
        let mut readonly_index_file = index_file.try_clone()?;
        readonly_index_file.seek(SeekFrom::Start(0))?;
        let reader = std::io::BufReader::new(readonly_index_file);
        let indexes_reader = Deserializer::from_reader(reader).into_iter::<Index>();
        let mut last_index = RawIndex::default();
        for index_result in indexes_reader {
            let index: Index = index_result?;
            let raw_index = RawIndex::new(index.offset, index.length);
            last_index = raw_index;
            index_map.insert(index.path, raw_index);
        }

        Ok((index_file, index_map, last_index))
    }

    fn can_write(&self, length: u64) -> bool {
        if self.readonly() {
            return false;
        }
        let length_after_write = self.current_length + length;
        return length_after_write < self.max_length;
    }

    pub fn write_needle(&mut self, path: &str, needle: Needle) -> Result<()> {
        if self.readonly() {
            return Err(Error::volume(error::VolumeError::readonly(self.id)));
        }
        let length = needle.length;
        if !self.can_write(length as u64) {
            return Err(Error::volume(error::VolumeError::overflow(
                self.id,
                self.max_length,
                self.current_length,
                length as u64,
            )));
        }
        let mut received_length = 0usize;
        self.writable_volume
            .seek(SeekFrom::Start(self.current_length))?;

        let mut writer = BufWriter::new(self.writable_volume.try_clone()?);

        match needle.body {
            NeedleBody::SinglePart(data) => {
                received_length += data.len();
                writer.write_all(data.as_ref())?;
            }
            NeedleBody::MultiParts(receiver) => {
                for data in receiver.iter() {
                    match data {
                        Ok(data) => {
                            received_length += data.len();
                            writer.write_all(data.as_ref())?;
                        }
                        Err(e) => {
                            return Err(Error::volume(error::VolumeError::write_needle(
                                path,
                                format!("{:?}", e),
                            )));
                        }
                    }
                }
            }
        }

        if received_length != length {
            return Err(Error::volume(error::VolumeError::write_length_mismatch(
                self.id,
                path,
                length,
                received_length,
            )));
        }

        // write index
        // TODO: supports write-ahead log

        let index = Index::new(path.to_owned(), self.current_length as usize, length);
        self.index_file
            .write_all(serde_json::to_string(&index)?.as_bytes())?;
        self.current_length += length as u64;
        self.indexes
            .insert(path.to_owned(), RawIndex::new(index.offset, index.length));
        Ok(())
    }

    pub fn get<K>(&self, key: K) -> Result<Needle>
    where
        K: Into<String>,
    {
        let key = key.into();
        let index: RawIndex = self
            .indexes
            .get(&key)
            .ok_or(Error::not_found(format!("not in indexes: {}", key)))?
            .clone();
        if ((index.offset + index.length) as u64) > self.current_length {
            return Err(Error::data_corruption(key, "index out of current length"));
        }
        let body = self.read_needle(&index)?;
        Ok(Needle {
            body: body,
            length: index.length,
        })
    }

    pub fn read_needle(&self, index: &RawIndex) -> Result<NeedleBody> {
        let mut readonly_volume = self.readonly_volume.try_clone()?;
        readonly_volume.seek(std::io::SeekFrom::Start(index.offset as u64))?;
        let batch_size = if index.length > 1024 * 1024 {
            1024 * 1024 // 1M
        } else {
            index.length
        };
        let mut buffer = Vec::with_capacity(batch_size);
        buffer.resize(batch_size, 0 as u8);
        let mut readonly_volume = self.readonly_volume.try_clone()?;
        if index.length <= 1024 * 1024 {
            readonly_volume.read_exact(&mut buffer)?;
            Ok(NeedleBody::SinglePart(bytes::Bytes::from(buffer)))
        } else {
            // TODO: using thread pool
            let (tx, rx) = std::sync::mpsc::sync_channel(1);
            let mut remains = index.length;
            std::thread::spawn(move || {
                while remains > 0 {
                    let current = match readonly_volume.read(&mut buffer) {
                        Ok(current) => current,
                        Err(e) => {
                            // NOTE: This function will never panic, but it may return [`Err`]
                            // if the [`Receiver`] has disconnected and is no longer able to
                            // receive information.
                            match tx.send(Err(Error::io(e))) {
                                Ok(_) => {}
                                Err(send_error) => {
                                    println!("failed to send error into channel. {:?}", send_error);
                                }
                            }
                            return;
                        }
                    };
                    remains -= current;
                    match tx.send(Ok(bytes::Bytes::from(&buffer[0..current]))) {
                        Ok(_) => {}
                        Err(e) => {
                            match tx.send(Err(Error::channel(e.description()))) {
                                Ok(_) => {}
                                Err(send_error) => {
                                    println!("failed to send error into channel. {:?}", send_error);
                                }
                            }
                            return;
                        }
                    }
                }
            });
            Ok(NeedleBody::MultiParts(rx))
        }
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
