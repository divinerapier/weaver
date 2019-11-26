use crate::error::{self, Error, Result};
use crate::index::{Index, RawIndex};
use crate::needle::{Needle, NeedleBody, NeedleHeader};
use crate::utils;

use bytes::ByteOrder;

use std::collections::HashMap;
use std::error::Error as StdError;
use std::ffi::OsStr;
use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc, Mutex, RwLock,
};

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

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct ReplicaReplacement {
    pub data_center_count: u8,
    pub rack_count: u8,
    pub node_count: u8,
}

impl ReplicaReplacement {
    pub fn replica_count(&self) -> usize {
        self.data_center_count as usize * self.rack_count as usize * self.node_count as usize
    }

    pub fn is_valid(&self) -> bool {
        self.data_center_count > 0 && self.rack_count > 0 && self.node_count > 0
    }
}

impl From<weaver_proto::weaver::ReplicaReplacement> for ReplicaReplacement {
    fn from(rr: weaver_proto::weaver::ReplicaReplacement) -> Self {
        ReplicaReplacement {
            data_center_count: (rr.data_center_count & 0xf) as u8,
            rack_count: (rr.rack_count & 0xf) as u8,
            node_count: (rr.node_count & 0xf) as u8,
        }
    }
}

impl Default for ReplicaReplacement {
    fn default() -> Self {
        ReplicaReplacement {
            data_center_count: 1,
            rack_count: 1,
            node_count: 1,
        }
    }
}

pub struct Volume {
    pub super_block: Arc<RwLock<SuperBlock>>,
    pub attibute: VolumeAttibute,
    pub writable_volume: Arc<Mutex<File>>,
    pub readonly_volume: Arc<File>,
    pub current_length: AtomicU64,
    pub index_file: Arc<Mutex<File>>,
    pub indexes: Arc<RwLock<HashMap<u64, RawIndex>>>,
}

unsafe impl Send for Volume {}
unsafe impl Sync for Volume {}

pub struct VolumeAttibute {
    pub id: AtomicU64,
    pub path: Arc<String>,
    pub max_length: AtomicU64,
}

impl Volume {
    pub fn id(&self) -> u64 {
        self.attibute.id.load(Ordering::Relaxed)
    }
    pub fn path(&self) -> &str {
        &self.attibute.path
    }
    pub fn max_length(&self) -> u64 {
        self.attibute.max_length.load(Ordering::Relaxed)
    }
    pub fn current_length(&self) -> u64 {
        self.current_length.load(Ordering::Relaxed)
    }
}

#[derive(Copy, Clone)]
pub struct SuperBlock {
    pub replica_replacement: ReplicaReplacement,
    pub max_volume_size: u32,
    pub max_needle_count: u32,
}

impl Default for SuperBlock {
    fn default() -> Self {
        SuperBlock {
            replica_replacement: ReplicaReplacement::default(),
            max_volume_size: 0,
            max_needle_count: 0,
        }
    }
}

impl SuperBlock {
    pub fn new(
        replica_replacement: &Option<ReplicaReplacement>,
        max_volume_size: u32,
        max_needle_count: u32,
    ) -> SuperBlock {
        SuperBlock {
            replica_replacement: replica_replacement.unwrap_or_default(),
            max_volume_size,
            max_needle_count,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let buffer_length = 4 /*replica replacement*/ + 4 /*max_volume_size*/ + 4 /*max_needle_count*/;
        let mut buffer = vec![0u8; buffer_length];
        let value = (self.replica_replacement.data_center_count as u32) << 16
            | (self.replica_replacement.rack_count as u32) << 8
            | self.replica_replacement.node_count as u32;
        bytes::BigEndian::write_u32(&mut buffer[0..4], value);
        bytes::BigEndian::write_u32(&mut buffer[4..8], self.max_volume_size);
        bytes::BigEndian::write_u32(&mut buffer[8..], self.max_needle_count);
        buffer
    }

    pub fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.as_bytes())?;
        Ok(writer.flush()?)
    }

    pub fn read_from<R: std::io::Read>(reader: &mut R) -> Result<SuperBlock> {
        let buffer_length = 4 /*replica replacement*/ + 4 /*max_volume_size*/ + 4 /*max_needle_count*/;
        let mut buffer = vec![0u8; buffer_length];
        reader.read_exact(&mut buffer)?;
        let buffer: &[u8] = &buffer;
        let super_block: SuperBlock = SuperBlock::from(buffer);
        Ok(super_block)
    }
}

impl From<&[u8]> for SuperBlock {
    fn from(data: &[u8]) -> Self {
        assert!(data.len() >= 12);
        let value = bytes::BigEndian::read_u32(&data[0..4]);
        let max_volume_size = bytes::BigEndian::read_u32(&data[4..8]);
        let max_needle_count = bytes::BigEndian::read_u32(&data[8..12]);
        let data_center_count = ((value & 0x0f00) >> 16) as u8;
        let rack_count = ((value & 0x00f0) >> 8) as u8;
        let node_count = (value & 0x000f) as u8;
        SuperBlock {
            replica_replacement: ReplicaReplacement {
                data_center_count,
                rack_count,
                node_count,
            },
            max_volume_size,
            max_needle_count,
        }
    }
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
            return Err(storage_error!("exists volume data: {}", id));
        }
        if index_path.exists() {
            log::error!(
                "couldn't create the volume index which exists already. id: {}, path: {}",
                id,
                index_path.display()
            );
            return Err(storage_error!("exists volume index: {}", id));
        }
        let (index_file, index_map, _) = Self::open_indexes(index_path, true)?;
        let (readonly_file, writable_file) = Self::open_volumes(&volume_path, true)?;
        let current_length = writable_file.metadata()?.len();
        Ok(Volume {
            super_block: Arc::new(RwLock::new(SuperBlock::default())),
            attibute: VolumeAttibute {
                id: AtomicU64::new(id as u64),
                path: Arc::new(
                    volume_path
                        .to_str()
                        .ok_or(storage_error!("parse volume path: {:?}", volume_path))?
                        .to_owned(),
                ),
                max_length: AtomicU64::new(size),
            },
            writable_volume: Arc::new(Mutex::new(writable_file)),
            readonly_volume: Arc::new(readonly_file),
            current_length: AtomicU64::new(current_length),
            index_file: Arc::new(Mutex::new(index_file)),
            indexes: Arc::new(RwLock::new(index_map)),
        })
    }

    /// open a physical volume file from disk
    /// volume_path is the
    pub fn open(volume_path: &Path, size: u64) -> Result<Volume> {
        let extension = volume_path
            .extension()
            .ok_or(storage_error!("get file_stem from {:?}", volume_path))?;
        if extension != "index" {
            return Err(storage_error!("open path: {:?}", volume_path));
        }
        // filename should be a usize number
        let volume_path_str = volume_path
            .to_str()
            .ok_or(storage_error!("std::path::Path -> &str. {:?}", volume_path))?;
        let id = Self::parse_volume_file_stem_name(volume_path)?;
        let extension_str = extension
            .to_str()
            .ok_or(storage_error!("std::path::Path -> &str. {:?}", extension))?;
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
            return Err(storage_error!(
                "volume: {}, current length: {}, last_index.offset: {}, last_index.length: {}",
                id,
                current_length,
                last_index.offset,
                last_index.length
            ));
        }

        Ok(Volume {
            super_block: Arc::new(RwLock::new(SuperBlock::default())),
            attibute: VolumeAttibute {
                id: AtomicU64::new(id as u64),
                path: Arc::new(
                    volume_path
                        .to_str()
                        .ok_or(storage_error!("parse volume path: {:?}", volume_path))?
                        .to_owned(),
                ),
                max_length: AtomicU64::new(size),
            },
            writable_volume: Arc::new(Mutex::new(writable_file)),
            readonly_volume: Arc::new(readonly_file),
            current_length: AtomicU64::new(current_length),
            index_file: Arc::new(Mutex::new(index_file)),
            indexes: Arc::new(RwLock::new(index_map)),
        })
    }

    fn path_exists(id: u32, path: &PathBuf) -> Result<()> {
        if path.exists() {
            log::error!("file exists. id: {}, path: {}", id, path.display());
            return Err(storage_error!(
                "exists volume file. id: {}, path: {:?}",
                id,
                path
            ));
        } else {
            Ok(())
        }
    }

    // Create a new volume on the specified directory with the id as its name.
    // And set the size and replica replacement of the volume.
    pub fn new2<P: AsRef<Path>>(
        dir: P,
        id: u32,
        size: u64,
        super_block: &SuperBlock,
    ) -> Result<Volume> {
        let volume_path: PathBuf = dir.as_ref().join(format!("{}.data", id));
        let index_path: PathBuf = dir.as_ref().join(format!("{}.index", id));
        Self::path_exists(id, &volume_path)?;
        Self::path_exists(id, &index_path)?;
        let (index_file, index_map, _) = Self::open_indexes(index_path, true)?;
        let (readonly_file, mut writable_file) = Self::open_volumes(&volume_path, true)?;

        super_block.write_to(&mut writable_file)?;

        let current_length = writable_file.metadata()?.len();
        Ok(Volume {
            super_block: Arc::new(RwLock::new(super_block.clone())),
            attibute: VolumeAttibute {
                id: AtomicU64::new(id as u64),
                path: Arc::new(
                    volume_path
                        .to_str()
                        .ok_or(storage_error!("parse volume path: {:?}", volume_path))?
                        .to_owned(),
                ),
                max_length: AtomicU64::new(size),
            },
            writable_volume: Arc::new(Mutex::new(writable_file)),
            readonly_volume: Arc::new(readonly_file),
            current_length: AtomicU64::new(current_length),
            index_file: Arc::new(Mutex::new(index_file)),
            indexes: Arc::new(RwLock::new(index_map)),
        })
    }

    // Open the exist file.
    pub fn open2(dir: &str, id: u64, size: u64) -> Result<Volume> {
        // filename should be a usize number
        let data_file_path = format!("{}/{}.data", dir, id);
        let index_file_path = format!("{}/{}.index", dir, id);

        let (index_file, index_map, last_index) = Self::open_indexes(index_file_path, false)?;
        let (readonly_file, mut writable_file) = Self::open_volumes(&data_file_path, false)?;
        let current_length = writable_file.metadata()?.len();
        if current_length != (last_index.offset + last_index.length) as u64 {
            log::error!(
                "volume data corruption. {}:{}, {}, current_length: {}, last_index.offset: {}, last_index.length: {}",
                dir,
                id,
                data_file_path,
                current_length,
                last_index.offset,
                last_index.length
            );
            return Err(storage_error!(
                "volume: {}, current length: {}, last_index.offset: {}, last_index.length: {}",
                id,
                current_length,
                last_index.offset,
                last_index.length
            ));
        }

        let super_block = SuperBlock::read_from(&mut writable_file)?;

        Ok(Volume {
            // FIXME: read super block from volume file
            super_block: Arc::new(RwLock::new(super_block)),
            attibute: VolumeAttibute {
                id: AtomicU64::new(id as u64),
                path: Arc::new(dir.to_owned()),
                max_length: AtomicU64::new(size),
            },
            writable_volume: Arc::new(Mutex::new(writable_file)),
            readonly_volume: Arc::new(readonly_file),
            current_length: AtomicU64::new(current_length),
            index_file: Arc::new(Mutex::new(index_file)),
            indexes: Arc::new(RwLock::new(index_map)),
        })
    }

    fn parse_volume_file_stem_name(volume_path: &Path) -> Result<u32> {
        let file_stem = volume_path
            .file_stem()
            .ok_or(error!("get file_stem from {:?}", volume_path))?;
        let file_stem_str = file_stem
            .to_str()
            .ok_or(error!("&std::ffi::OsStr -> &str. {:?}", file_stem))?;
        let id = file_stem_str.parse::<u32>()?;
        Ok(id)
    }

    pub fn writable(&self) -> bool {
        let max_length = self.max_length();
        let current_length = self.current_length.load(Ordering::SeqCst);
        current_length < max_length
    }

    pub fn readonly(&self) -> bool {
        !self.writable()
    }

    pub fn available_length(&self) -> u64 {
        let max_length = self.max_length();
        let current_length = self.current_length.load(Ordering::SeqCst);
        if max_length > current_length {
            max_length - current_length
        } else {
            0
        }
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
    }

    pub fn write_needle(&mut self, needle_id: u64, needle: Needle) -> Result<()> {
        let actual_data_length = needle.actual_length() as usize;
        let total_length = needle.total_length() as usize;
        if !self.can_write(total_length as u64) {
            log::error!(
                "couldn't write to the volume. id: {}, path: {}, writable: {}, max_length: {}, current_length: {}, actual_data_length: {}, total_length: {}",
                self.id(),
                self.path(),
                self.writable(),
                self.max_length(),
                self.current_length(),
               actual_data_length,
               total_length
            );
            return Err(storage_error!(
                "overflow. id: {}, max length: {}, current: {}, to be written: {}",
                self.id(),
                self.max_length(),
                self.current_length(),
                total_length
            ));
        }
        let mut received_length = 0usize;

        {
            let writable_volume = self.writable_volume.clone();
            let mut writable_volume = writable_volume.lock().unwrap();
            writable_volume.seek(SeekFrom::Start(self.current_length()))?;

            let writable_volume: &mut File = &mut writable_volume;
            let mut writer = BufWriter::new(writable_volume);

            let needle_iter = needle.into_iter();
            for data in needle_iter {
                let data = data?;
                log::debug!("data: {:?}", data);
                received_length += data.len();
                writer.write_all(data.as_ref())?;
            }

            writer.flush()?;
        }
        if received_length != actual_data_length {
            log::error!(
                "mismatched needle length. received: {}, actual: {}, total: {}",
                received_length,
                actual_data_length,
                total_length,
            );
            return Err(storage_error!(
                "write length not matched. volume: {}, needle: {}, recv: {}, write: {}",
                self.id(),
                needle_id,
                received_length,
                actual_data_length
            ));
        }

        let index = Index::new(
            needle_id,
            self.id() as u32,
            self.current_length() as usize,
            total_length,
        );

        {
            let mut index_file = self.index_file.lock().unwrap();
            index_file.write_all(serde_json::to_string(&index)?.as_bytes())?;
            self.current_length
                .fetch_add(total_length as u64, Ordering::SeqCst);
        }
        let mut indexes = self.indexes.write().unwrap();
        indexes.insert(
            needle_id,
            RawIndex::new(index.volume_id, index.offset, index.length),
        );
        Ok(())
    }

    pub fn write_needle2(&mut self, needle: &weaver_proto::weaver::Needle) -> Result<()> {
        if needle.header.is_none() {
            return Err(error!("failed to write empty needle"));
        }
        self._write_needle(&needle)?;
        self._write_index(&needle)?;
        Ok(())
    }

    fn _write_needle(&mut self, needle: &weaver_proto::weaver::Needle) -> Result<()> {
        let header = needle.header.as_ref().unwrap();
        let body: &[u8] = &needle.body;

        assert_eq!(header.size as usize, body.len());

        let mut buffer = vec![0u8; 40];
        bytes::BigEndian::write_u64(&mut buffer[0..8], header.id);
        bytes::BigEndian::write_u64(&mut buffer[8..16], header.cookie);
        bytes::BigEndian::write_u64(&mut buffer[16..24], header.offset);
        bytes::BigEndian::write_u32(&mut buffer[24..28], header.size);
        bytes::BigEndian::write_u64(&mut buffer[28..36], header.total_size);
        bytes::BigEndian::write_u32(&mut buffer[36..40], header.crc);

        {
            let writable_volume = self.writable_volume.clone();
            let writable_volume = writable_volume.lock().unwrap();
            let writer: &File = &writable_volume;
            let mut writer = BufWriter::new(writer);
            writer.write_all(&buffer)?;
            writer.write_all(body)?;
            Ok(writer.flush()?)
        }
    }

    fn _write_index(&mut self, needle: &weaver_proto::weaver::Needle) -> Result<()> {
        let needle = needle.header.as_ref().unwrap();
        let index = Index::new(
            needle.id,
            self.id() as u32,
            self.current_length() as usize,
            needle.total_size as usize,
        );

        {
            let mut index_file = self.index_file.lock().unwrap();
            index_file.write_all(serde_json::to_string(&index)?.as_bytes())?;
        }
        self.current_length
            .fetch_add(needle.total_size, Ordering::SeqCst);
        {
            let mut indexes = self.indexes.write().unwrap();
            indexes.insert(
                needle.id,
                RawIndex::new(index.volume_id, index.offset, index.length),
            );
        }
        Ok(())
    }

    pub fn get(&self, needle_id: u64) -> Result<Needle> {
        let indexes = self.indexes.read().unwrap();
        let index: RawIndex = indexes
            .get(&needle_id)
            .ok_or(storage_error!(
                "needle not in indexes: {}, indexes: {:?}",
                needle_id,
                self.indexes
            ))?
            .clone();
        log::debug!("index: {:?}", index);
        if ((index.offset + index.length) as u64) > self.current_length() {
            log::error!(
                "volume data corruption. needle: {}, volume_length: {}, index.offset: {}, index.length: {}",
                needle_id,
                self.current_length(),
                index.offset,
                index.length
            );
            return Err(storage_error!("data corruption. needle: {}", needle_id));
            // return Err(Error::data_corruption(
            //     needle_id.to_string(),
            //     "index out of current length",
            // ));
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
        let readonly_volume = self.readonly_volume.clone();
        let mut readonly_volume = readonly_volume.try_clone()?;
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
                        match tx.send(Err(external_error!(e))) {
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
                        match tx.send(Err(external_error!(e))) {
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
