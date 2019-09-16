use std::collections::{HashMap, HashSet};
use std::ops::Try;
use std::path::{Path, PathBuf};

use crate::error::Result;
use crate::needle::Needle;
use crate::store::volume::Volume;
use crate::utils::size::Size;

#[allow(dead_code)]
pub struct Directory {
    pub volumes_dir: PathBuf,
    pub volumes: Vec<Volume>,

    // TODO: use a min-heap to store volumes? tuple(id, remain_length)
    pub writable_volumes: HashSet<u32>,
    pub readonly_volumes: HashSet<u32>,

    /// map from file path to volume index in self.volumes
    pub needle_map: HashMap<u64, u32>,

    pub volume_size: Size,
}

#[allow(dead_code, unused)]
impl Directory {
    /// new opens the storage by specified path
    /// and also loads the indexes
    pub fn new<P>(path: P, volume_size: Size) -> Result<Directory>
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
        let volume_id =
            self.random_writable_volume()
                .into_result()
                .or_else(|_| -> Result<u32> {
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

    // pub fn read<K>(&self, key: K) -> Result<Needle>
    // where
    //     K: Into<String>,
    // {
    //     let key = key.into();
    //     let volume_id = *self
    //         .needle_map
    //         .get(&key)
    //         .ok_or(Error::not_found(format!("path: {}", key)))?;
    //     let volume: &Volume = self
    //         .volumes
    //         .get(volume_id)
    //         .ok_or(boxed_volume_not_found!(volume_id))?;
    //     Ok(volume.get(key)?)
    // }

    // Notice: this is not randomly, different from golang
    // I have no idea about how to test it
    fn random_writable_volume(&self) -> Option<u32> {
        use rand::Rng;
        let length = self.writable_volumes.len();
        if length == 0 {
            return None;
        }
        let index = rand::thread_rng().gen::<i64>() as usize;
        let index = index % self.writable_volumes.len();
        let volume_id = *self.writable_volumes.iter().skip(index).next()?;
        assert_eq!(length, self.writable_volumes.len());
        Some(volume_id as u32)
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
            volume_size: Size::default(),
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
        let mut directory = Directory::new(testdata_dir.as_path(), Size::byte(100)).unwrap();
        let data1 = bytes::Bytes::from("data1: hello world data1\n");
        let data2 = bytes::Bytes::from("data2: hello world data2\n");
        let data3 = bytes::Bytes::from("data3: hello world data3\n");
        let data4 = bytes::Bytes::from("data4: hello world data4\n");
        let data5 = bytes::Bytes::from("data5: hello world data5\n");
        let data6 = bytes::Bytes::from("data6: hello world data6\n");
        let data7_1 = bytes::Bytes::from("data7_1: hello world data7_1\n");
        let data7_2 = bytes::Bytes::from("data7_2: hello world data7_2\n");
        let data7_3 = bytes::Bytes::from("data7_3: hello world data7_3\n");
        // write
        {
            {
                log::debug!("test1",);
                let needle = Needle {
                    header: NeedleHeader {
                        body_length: data1.len() as u32,
                    },
                    body: NeedleBody::SinglePart(data1),
                };
                directory.write(0, needle).unwrap();
            }
            {
                log::debug!("test2",);
                let needle = Needle {
                    header: NeedleHeader {
                        body_length: data2.len() as u32,
                    },
                    body: NeedleBody::SinglePart(data2),
                };
                directory.write(1, needle).unwrap();
            }
            {
                log::debug!("test3",);
                let needle = Needle {
                    header: NeedleHeader {
                        body_length: data3.len() as u32,
                    },
                    body: NeedleBody::SinglePart(data3),
                };
                directory.write(2, needle).unwrap();
            }
            {
                log::debug!("test4",);
                let needle = Needle {
                    header: NeedleHeader {
                        body_length: data4.len() as u32,
                    },
                    body: NeedleBody::SinglePart(data4),
                };
                directory.write(3, needle).unwrap();
            }
            {
                log::debug!("test5",);
                let needle = Needle {
                    header: NeedleHeader {
                        body_length: data5.len() as u32,
                    },
                    body: NeedleBody::SinglePart(data5),
                };
                directory.write(4, needle).unwrap();
            }
            {
                log::debug!("test6",);
                let needle = Needle {
                    header: NeedleHeader {
                        body_length: data6.len() as u32,
                    },
                    body: NeedleBody::SinglePart(data6),
                };
                directory.write(5, needle).unwrap();
            }
            {
                log::debug!("test7",);
                let (tx, rx) = std::sync::mpsc::channel();
                let length = data7_1.len() + data7_2.len() + data7_3.len();
                std::thread::spawn(move || {
                    tx.send(Ok(data7_1)).unwrap();
                    tx.send(Ok(data7_2)).unwrap();
                    tx.send(Ok(data7_3)).unwrap();
                });
                let needle = Needle {
                    header: NeedleHeader {
                        body_length: length as u32,
                    },
                    body: NeedleBody::MultiParts(rx),
                };
                directory.write(6, needle).unwrap();
            }
        }
        // read
        // {
        //     let needle1 = directory.read("/path/to/file/1").unwrap();
        //     check_needle_body(needle1, "data1: hello world data1\n");
        //     let needle2 = directory.read("/path/to/file/2").unwrap();
        //     check_needle_body(needle2, "data2: hello world data2\n");
        //     let needle3 = directory.read("/path/to/file/3").unwrap();
        //     check_needle_body(needle3, "data3: hello world data3\n");
        //     let needle4 = directory.read("/path/to/file/4").unwrap();
        //     check_needle_body(needle4, "data4: hello world data4\n");
        //     let needle5 = directory.read("/path/to/file/5").unwrap();
        //     check_needle_body(needle5, "data5: hello world data5\n");
        //     let needle6 = directory.read("/path/to/file/6").unwrap();
        //     check_needle_body(needle6, "data6: hello world data6\n");
        //     let needle7 = directory.read("/path/to/file/7").unwrap();
        //     check_needle_body(needle7, "data7_1: hello world data7_1\ndata7_2: hello world data7_2\ndata7_3: hello world data7_3\n");
        // }
    }

    fn check_needle_body(needle: Needle, data: &str) {
        assert_eq!(needle.body_length() as usize, data.len());
        match needle.body {
            NeedleBody::SinglePart(body) => {
                assert_eq!(body.len(), data.len());
                assert_eq!(body.as_ref(), data.as_bytes());
            }
            // TODO: test read multiparts
            NeedleBody::MultiParts(_body_chain) => {}
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
