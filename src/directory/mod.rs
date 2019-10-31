use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::error::Result;
use crate::needle::Needle;
use crate::storage::volume::Volume;

#[allow(dead_code)]
pub struct Directory {
    pub volumes_dir: PathBuf,
    pub volumes: Vec<Volume>,

    // TODO: use a min-heap to store volumes? tuple(id, remain_length)
    pub writable_volumes: HashSet<u32>,
    pub readonly_volumes: HashSet<u32>,

    /// map from file path to volume index in self.volumes
    pub needle_map: HashMap<u64, u32>,

    pub volume_size: u64,
}

#[allow(dead_code, unused)]
impl Directory {
    /// new opens the storage by specified path
    /// and also loads the indexes
    pub fn new<P>(path: P, volume_size: u64) -> Result<Directory>
    where
        P: AsRef<Path>,
    {
        let mut result = Directory {
            volumes_dir: PathBuf::from(path.as_ref()),
            volume_size,
            ..Directory::default()
        };
        let dir: std::fs::ReadDir = std::fs::read_dir(path)?;
        for entry in dir {
            let entry = entry?;
            let inner_file_path: std::path::PathBuf = entry.path();
            Volume::open(inner_file_path.as_path(), volume_size).map(|volume| -> Result<()> {
                let volume: Volume = volume;
                let index = result.volumes.len() as u32;
                let writable = volume.writable();
                result.volumes.push(volume);
                if writable {
                    result.writable_volumes.insert(index);
                } else {
                    result.readonly_volumes.insert(index);
                }
                // TODO: optimize copying index
                let volume_ref: Result<&Volume> = result
                    .volumes
                    .get(index as usize)
                    .ok_or(boxed_volume_not_found!(index));

                let volume_ref = volume_ref?;
                for (k, v) in &volume_ref.indexes {
                    result.needle_map.insert(*k, index);
                }
                Ok(())
            });
        }
        // should sort volumes by volume.id
        result.volumes.sort_by_key(|e| e.id);
        Ok(result)
    }

    pub fn read(&self, volume_id: u32, needle_id: u64) -> Result<Needle> {
        let volume: &Volume = self
            .volumes
            .get(volume_id as usize)
            .ok_or(boxed_volume_not_found!(volume_id))?;
        Ok(volume.get(needle_id)?)
    }

    /// write appends the body to any available volume and
    /// then records the offset and body size to index file
    pub fn write(&mut self, needle_id: u64, body: Needle) -> Result<()> {
        self.try_write(needle_id, body)
    }

    fn try_write(&mut self, needle_id: u64, body: Needle) -> Result<()> {
        let mut volume_id = self.get_writable_volume(body.total_length() as usize)?;
        let volume: &mut Volume = self
            .volumes
            .get_mut(volume_id as usize)
            .ok_or(boxed_volume_not_found!(volume_id))?;
        volume.write_needle(needle_id, body)?;
        self.needle_map.insert(needle_id, volume.id);
        if !volume.writable() {
            self.writable_volumes.remove(&volume_id);
            self.readonly_volumes.insert(volume_id);
        }
        Ok(())
    }

    fn get_writable_volume(&mut self, length: usize) -> Result<u32> {
        let mut retry_times = std::cmp::max(3, self.volumes.len());
        loop {
            match self.try_get_writable_volume(length) {
                Ok(volume_id) => return Ok(volume_id),
                Err(_) => {
                    if retry_times > 0 {
                        retry_times -= 1;
                        continue;
                    }
                    let volume = Volume::new(
                        &self.volumes_dir,
                        self.volumes.len() as u32,
                        self.volume_size,
                    )?;
                    let volume_id = volume.id;
                    self.volumes.push(volume);
                    self.writable_volumes.insert(volume_id);
                    return Ok(volume_id);
                }
            }
        }
    }

    fn try_get_writable_volume(&mut self, length: usize) -> Result<u32> {
        let volume_id = self.random_writable_volume().or_else(|_| -> Result<u32> {
            let volume = Volume::new(
                &self.volumes_dir,
                self.volumes.len() as u32,
                self.volume_size,
            )?;
            let volume_id = volume.id;
            self.volumes.push(volume);
            self.writable_volumes.insert(volume_id);
            Ok(volume_id)
        })?;
        let volume: &mut Volume = self
            .volumes
            .get_mut(volume_id as usize)
            .ok_or(boxed_volume_not_found!(volume_id))?;
        if volume.max_length - volume.current_length < length as u64 {
            log::warn!(
                "volume almost full. max: {}, current: {}, todo: {}",
                volume.max_length,
                volume.current_length,
                length
            );
            return Err(boxed_no_writable_volumes!());
        }
        Ok(volume.id)
    }

    // Notice: this is not randomly, different from golang
    // I have no idea about how to test it
    fn random_writable_volume(&self) -> Result<u32> {
        let length = self.writable_volumes.len();
        if length == 0 {
            return Err(boxed_no_writable_volumes!());
        }
        let index = srand::ThreadLocal::uint64() as usize;
        let index = index % self.writable_volumes.len();
        assert_eq!(length, self.writable_volumes.len());
        match self.writable_volumes.iter().skip(index).next() {
            Some(&volume_id) => Ok(volume_id),
            None => Err(boxed_no_writable_volumes!()),
        }
    }
}

impl Default for Directory {
    fn default() -> Directory {
        Directory {
            volumes_dir: PathBuf::default(),
            volumes: vec![],
            writable_volumes: HashSet::new(),
            readonly_volumes: HashSet::new(),
            needle_map: HashMap::new(),
            volume_size: 128,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::needle::NeedleBody;
    use crate::needle::NeedleHeader;
    use std::env;

    #[test]
    fn foo1() {
        env_logger::from_env(env_logger::Env::default().default_filter_or("trace")).init();
        log::set_max_level(log::LevelFilter::max());
        let testdata_dir = env::current_dir().unwrap().join("testdata");
        std::fs::create_dir_all(testdata_dir.as_path()).unwrap();
        std::fs::remove_dir_all(testdata_dir.as_path()).unwrap();
        std::fs::create_dir_all(testdata_dir.as_path()).unwrap();
        let mut directory = Directory::new(testdata_dir.as_path(), 100).unwrap();
        let data: Vec<bytes::Bytes> = vec![
            bytes::Bytes::from("data1: hello world data1\n"),
            bytes::Bytes::from("data2: hello world data2\n"),
            bytes::Bytes::from("data3: hello world data3\n"),
            bytes::Bytes::from("data4: hello world data4\n"),
            bytes::Bytes::from("data5: hello world data5\n"),
            bytes::Bytes::from("data6: hello world data6\n"),
            bytes::Bytes::from("data7_1: hello world data7_1\n"),
            bytes::Bytes::from("data7_2: hello world data7_2\n"),
            bytes::Bytes::from("data7_3: hello world data7_3\n"),
            bytes::Bytes::from("sd8_1\n"),
            bytes::Bytes::from("sd8_2\n"),
            bytes::Bytes::from("sd8_3\n"),
            bytes::Bytes::from("sd8_4\n"),
            bytes::Bytes::from("sd8_5\n"),
        ];
        // write
        {
            for i in 0..6 {
                log::debug!("test{}", i);
                let needle = Needle::new(
                    NeedleHeader::new(i as u64, data[i as usize].len() as u32),
                    NeedleBody::SinglePart(data[i].clone()),
                    8,
                );
                directory.write(i as u64, needle).unwrap();
            }

            // FIXME: space of writable volume is not enough
            {
                log::debug!("test6",);
                let (tx, rx) = std::sync::mpsc::channel();
                let length = data[6].len() + data[7].len() + data[8].len();
                let cloned_data = data.clone();
                std::thread::spawn(move || {
                    tx.send(Ok(cloned_data[6].clone())).unwrap();
                    tx.send(Ok(cloned_data[7].clone())).unwrap();
                    tx.send(Ok(cloned_data[8].clone())).unwrap();
                });
                let needle = Needle::new(
                    NeedleHeader::new(6, length as u32),
                    NeedleBody::MultiParts(rx),
                    8,
                );
                directory.write(6, needle).unwrap();
            }
            {
                for i in 9..14 {
                    log::debug!("test{}", i);
                    let needle = Needle::new(
                        NeedleHeader::new(i, data[i as usize].len() as u32),
                        NeedleBody::SinglePart(data[i as usize].clone()),
                        8,
                    );
                    directory.write(i as u64, needle).unwrap();
                }
            }
        }
        // read
        {
            for i in 0..6 {
                let needle1 = directory.read(i, i as u64).unwrap();
                check_needle_body(needle1, unsafe {
                    &String::from_utf8_unchecked((data[i as usize].as_ref().to_vec()))
                });
            }

            let needle1 = directory.read(6, 6).unwrap();
            check_needle_body(needle1, unsafe {
                let mut expect = data[6].clone().as_ref().to_vec();
                expect.append(&mut { data[7].clone().as_ref().to_vec() });
                expect.append(&mut { data[8].clone().as_ref().to_vec() });
                &String::from_utf8_unchecked(expect)
            });

            let needle1 = directory.read(7, 9).unwrap();
            check_needle_body(needle1, unsafe {
                &String::from_utf8_unchecked((data[9].as_ref().to_vec()))
            });
            // unpredictable behavior
            // let needle1 = directory.read(7, 10).unwrap();
            // check_needle_body(needle1, unsafe {
            //     &String::from_utf8_unchecked((data[10].as_ref().to_vec()))
            // });
        }
    }

    fn check_needle_body(needle: Needle, data: &str) {
        assert_eq!(needle.body_length() as usize, data.len());
        let mut v = vec![];
        match needle.body {
            NeedleBody::SinglePart(body) => {
                assert_eq!(body.len(), data.len());
                log::debug!("body: {:?}", body.as_ref());
                log::debug!("data: {:?}", data.as_bytes());
                assert_eq!(body.as_ref(), data.as_bytes());
            }
            // TODO: test read multiparts
            NeedleBody::MultiParts(_body_chain) => {
                for data in _body_chain {
                    let data: bytes::Bytes = data.unwrap();
                    let data: &[u8] = data.as_ref();
                    v.append(&mut data.to_vec());
                }
                assert_eq!(v.len(), data.len());
                log::debug!("body: {:?}", v);
                log::debug!("data: {:?}", data.as_bytes());
                assert_eq!(v, data.as_bytes());
            }
        }
    }

    fn hashset_pop_range<'a>(set: &'a HashSet<i32>) -> Option<&'a i32> {
        for i in set.iter() {
            return Some(i);
        }
        None
    }

    #[test]
    fn test_hashset_pop_in_range() {
        // Notice: this behavior is not same as golang
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set.insert(5);
        set.insert(6);
        let a1 = hashset_pop_range(&set);
        let a2 = hashset_pop_range(&set);
        let a3 = hashset_pop_range(&set);
        let a4 = hashset_pop_range(&set);
        let a5 = hashset_pop_range(&set);
        let a6 = hashset_pop_range(&set);
        assert_eq!(a1, a2);
        assert_eq!(a1, a3);
        assert_eq!(a1, a4);
        assert_eq!(a1, a5);
        assert_eq!(a1, a6);
    }
}
