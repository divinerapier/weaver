use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc, Mutex, RwLock,
};

use bytes::ByteOrder;
use serde_json::Deserializer;

use crate::error::Result;
use crate::needle::{Needle, NeedleBody, NeedleHeader};
use crate::storage::index::{Codec as IndexCodec, Entry as IndexEntry, Index};
use crate::utils;

pub enum VolumeExtension {
    Index = 1,
    Data = 2,
    Unknown = 99,
}

impl From<&str> for VolumeExtension {
    fn from(t: &str) -> VolumeExtension {
        match t {
            "index" | "idx" => VolumeExtension::Index,
            "data" | "dat" => VolumeExtension::Data,
            _ => VolumeExtension::Unknown
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

impl From<proto::weaver::ReplicaReplacement> for ReplicaReplacement {
    fn from(rr: proto::weaver::ReplicaReplacement) -> Self {
        ReplicaReplacement {
            data_center_count: (rr.data_center_count & 0xff) as u8,
            rack_count: (rr.rack_count & 0xff) as u8,
            node_count: (rr.node_count & 0xff) as u8,
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

pub struct Volume<C: super::index::Codec> {
    pub super_block: Arc<RwLock<SuperBlock>>,
    pub attibute: VolumeAttibute,
    pub writable_volume: Arc<Mutex<File>>,
    pub readonly_volume: Arc<File>,
    pub current_length: AtomicU64,
    pub index: Index<C>,
}

// unsafe impl<C: super::index::Codec> Send for Volume<C> {}
//
// unsafe impl<C: super::index::Codec> Sync for Volume<C> {}

pub struct VolumeAttibute {
    pub id: AtomicU64,
    pub path: Arc<PathBuf>,
    pub max_length: AtomicU64,
}

impl<C: IndexCodec> Volume<C> {
    pub fn id(&self) -> u64 {
        self.attibute.id.load(Ordering::Relaxed)
    }
    pub fn path(&self) -> &PathBuf {
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

// 18009840492
pub struct VolumeBuilder<P, C> where P: AsRef<Path>, C: IndexCodec {
    dir: Option<P>,
    size: Option<u64>,
    super_block: Option<SuperBlock>,
    codec: Option<C>,
}


impl<P, C> VolumeBuilder<P, C> where P: AsRef<Path>, C: IndexCodec {
    pub fn new() -> VolumeBuilder<P, C> {
        VolumeBuilder {
            dir: None,
            size: None,
            super_block: None,
            codec: None,
        }
    }

    pub fn set_dir(mut self, dir: P) -> VolumeBuilder<P, C> {
        self.dir = Some(dir);
        self
    }
    pub fn set_size(mut self, size: u64) -> VolumeBuilder<P, C> {
        self.size = Some(size);
        self
    }
    pub fn set_super_block(mut self, super_block: SuperBlock) -> VolumeBuilder<P, C> {
        self.super_block = Some(super_block);
        self
    }
    pub fn set_codec(mut self, codec: C) -> VolumeBuilder<P, C> {
        self.codec = Some(codec);
        self
    }
}

impl<C> Volume<C>
    where
        C: IndexCodec,
{
    /// Create a new volume on the specified directory with the id as its name.
    /// And set the size and replica replacement of the volume.
    pub async fn new<P: AsRef<Path>>(
        dir: P,
        id: u64,
        size: u64,
        super_block: &SuperBlock,
        codec: C,
    ) -> Result<Volume<C>>
        where
            C: super::index::Codec,
    {
        let volume_path: PathBuf = dir.as_ref().join(format!("{}.data", id));
        let index_path: PathBuf = dir.as_ref().join(format!("{}.index", id));
        if volume_path.exists() {
            log::error!(
                "couldn't create a volume on an existing path. path: {}",
                volume_path.display()
            );
            return Err(storage_error!("exists volume data: {}", id));
        }
        if index_path.exists() {
            log::error!(
                "couldn't create a volume on an existing path. path: {}",
                index_path.display()
            );
            return Err(storage_error!("exists volume index: {}", id));
        }
        let (readonly_file, mut writable_file) = Self::open_volume(&volume_path, true)?;

        // write super block to volume file
        super_block.write_to(&mut writable_file)?;

        let current_length = writable_file.metadata()?.len();
        Ok(Volume {
            super_block: Arc::new(RwLock::new(super_block.clone())),
            attibute: VolumeAttibute {
                id: AtomicU64::new(id as u64),
                path: Arc::new(volume_path),
                max_length: AtomicU64::new(size),
            },
            writable_volume: Arc::new(Mutex::new(writable_file)),
            readonly_volume: Arc::new(readonly_file),
            current_length: AtomicU64::new(current_length),
            index: super::index::Index::new(&index_path, codec)?,
        })
    }

    // Open the exist file.
    pub async fn open<P: AsRef<Path>>(dir: P, id: u64, size: u64, codec: C) -> Result<Volume<C>> {
        // filename should be usize
        let volume_path = dir.as_ref().join(format!("{}.data", id));
        let index_path = dir.as_ref().join(format!("{}.index", id));

        if !volume_path.exists() {
            log::error!(
                "couldn't open the missing volume. path: {}",
                volume_path.display()
            );
            return Err(storage_error!("exists volume data: {}", id));
        }
        if !index_path.exists() {
            log::error!(
                "couldn't open the missing volume. path: {}",
                index_path.display()
            );
            return Err(storage_error!("exists volume index: {}", id));
        }
        let index = Index::new(&index_path, codec)?;

        let (readonly_file, mut writable_file) = Self::open_volume(&volume_path, false)?;
        let current_length = writable_file.metadata()?.len();
        let super_block = SuperBlock::read_from(&mut writable_file)?;

        let v = Volume {
            super_block: Arc::new(RwLock::new(super_block)),
            attibute: VolumeAttibute {
                id: AtomicU64::new(id as u64),
                path: Arc::new(volume_path),
                max_length: AtomicU64::new(size),
            },
            writable_volume: Arc::new(Mutex::new(writable_file)),
            readonly_volume: Arc::new(readonly_file),
            current_length: AtomicU64::new(current_length),
            index,
        };
        v.validate()?;
        Ok(v)
    }

    fn validate(&self) -> Result<()> {
        let current_length = self.current_length();
        match self.index.last_index()? {
            None => {
                // is a new volume, only superblock was written
                if current_length > 24 {
                    return Err(storage_error!(
                        "last index is None. current length is {}",
                        current_length
                    ));
                }
                return Ok(());
            }
            Some(last_index) => {
                if last_index.offset + last_index.length != current_length as usize {
                    return Err(storage_error!(
                        "last_index.offset is {}, last_index.length is {}, current length is {}",
                        last_index.offset,
                        last_index.length,
                        current_length
                    ));
                }
                return Ok(());
            }
        }
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

    fn open_volume<P: AsRef<Path>>(volume_filepath: P, new: bool) -> Result<(File, File)> {
        let writable_file: File = OpenOptions::new()
            .read(true)
            .write(true)
            .create(new)
            .truncate(new)
            .append(!new)
            .open(volume_filepath.as_ref())
            .expect(&format!(
                "volume filepath: {}",
                volume_filepath.as_ref().display()
            ));
        let readonly_file = writable_file.try_clone()?;
        Ok((readonly_file, writable_file))
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
                self.path().display(),
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

        let index = IndexEntry::new(needle_id, self.current_length() as usize, total_length);

        {
            self.index.write(&index)?;
            self.current_length
                .fetch_add(total_length as u64, Ordering::SeqCst);
        }
        Ok(())
    }

    pub fn write_needle2(&mut self, needle: &proto::weaver::Needle) -> Result<()> {
        if needle.header.is_none() {
            return Err(error!("failed to write empty needle"));
        }
        self._write_needle(&needle)?;
        self._write_index(&needle)?;
        Ok(())
    }

    fn _write_needle(&mut self, needle: &proto::weaver::Needle) -> Result<()> {
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

    fn _write_index(&mut self, needle: &proto::weaver::Needle) -> Result<()> {
        let needle = needle.header.as_ref().unwrap();
        let index = IndexEntry::new(
            needle.id,
            self.current_length() as usize,
            needle.total_size as usize,
        );

        {
            self.index.write(&index)?;
        }
        self.current_length
            .fetch_add(needle.total_size, Ordering::SeqCst);
        Ok(())
    }

    pub fn get(&self, needle_id: u64) -> Result<Needle> {
        let index: IndexEntry = self.index.read(needle_id)?;
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
        }
        Ok(self.read_needle(&index)?)
    }

    pub fn read_needle_header(file: &mut File, offset: usize) -> Result<NeedleHeader> {
        let mut buffer = Vec::with_capacity(26);
        buffer.resize(26, 0);
        file.seek(std::io::SeekFrom::Start(offset as u64))?;
        file.read_exact(&mut buffer)?;
        Ok(buffer.into())
    }

    pub fn read_needle_body() {}

    pub fn read_needle(&self, index: &IndexEntry) -> Result<Needle> {
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
        use super::IndexEntry;
        use serde_json::Deserializer;
        use std::fs::File;
        use std::io::Seek;
        use std::io::Write;

        let mut file: File = tempfile::tempfile().unwrap();

        let indexes: Vec<IndexEntry> = vec![
            IndexEntry::new(0, 0, 10),
            IndexEntry::new(1, 10, 20),
            IndexEntry::new(2, 20, 30),
        ];

        for index in &indexes {
            let value = serde_json::to_string(index).unwrap();
            file.write(value.as_bytes()).unwrap();
            file.sync_all().unwrap();
        }
        file.seek(std::io::SeekFrom::Start(0)).unwrap();
        let reader = std::io::BufReader::new(file.try_clone().unwrap());
        let mut result = vec![];
        let indexes_reader = Deserializer::from_reader(reader).into_iter::<IndexEntry>();
        for index_result in indexes_reader {
            let index: IndexEntry = index_result.unwrap();
            result.push(index);
        }
        assert_eq!(indexes.len(), result.len());
        assert_eq!(indexes[0], result[0]);
        for (i, _) in indexes.iter().enumerate() {
            assert_eq!(result[i], indexes[i]);
        }
    }
}
