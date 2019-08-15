use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::ops::Try;
use std::path::{Path, PathBuf};

use crate::error::{self, Error, Result};
use crate::needle::Needle;
use crate::utils::size::Size;
use crate::volume::Volume;

#[allow(dead_code)]
pub struct Directory {
    pub volumes_dir: PathBuf,
    pub volumes: Vec<Volume>,

    // TODO: use a min-heap to store volumes? tuple(id, remain_length)
    pub writable_volumes: HashSet<usize>,
    pub readonly_volumes: HashSet<usize>,

    /// map from file path to volume index in self.volumes
    pub needle_map: HashMap<String, usize>,

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
                let index = result.volumes.len();
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
                    .get(index)
                    .ok_or(Error::not_found(format!("volume: {}", index)));
                let volume_ref = volume_ref?;
                for (k, v) in &volume_ref.indexes {
                    result.needle_map.insert(k.to_owned(), index);
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
    pub fn write<K>(&mut self, path: K, body: Needle) -> Result<()>
    where
        K: Into<String> + Clone + Display,
    {
        let mut retry_times = 3;
        while let Err(volume_error) = self.try_write(path.clone(), &body) {
            log::error!(
                "directory try write. path: {}, error: {}",
                path,
                volume_error
            );
            if volume_error.is_retriable() && retry_times > 0 {
                retry_times -= 1;
                continue;
            }
            return Err(volume_error);
        }
        Ok(())
    }

    fn try_write<K>(&mut self, path: K, body: &Needle) -> Result<()>
    where
        K: Into<String>,
    {
        let mut volume_id = self.get_writable_volume(body.length)?;
        let volume: &mut Volume = self
            .volumes
            .get_mut(volume_id)
            .ok_or(Error::volume(error::VolumeError::not_found(volume_id)))?;
        let path = path.into();
        volume.write_needle(&path, &body)?;
        self.needle_map.insert(path.clone(), volume.id);
        if !volume.writable() {
            self.writable_volumes.remove(&volume_id);
            self.readonly_volumes.insert(volume_id);
        }
        Ok(())
    }

    fn get_writable_volume(&mut self, length: usize) -> Result<usize> {
        let mut retry_times = std::cmp::max(3, self.volumes.len());
        loop {
            match self.try_get_writable_volume(length) {
                Ok(volume_id) => return Ok(volume_id),
                Err(_) => {
                    if retry_times > 0 {
                        retry_times -= 1;
                        continue;
                    } else {
                        let volume =
                            Volume::new(&self.volumes_dir, self.volumes.len(), self.volume_size)?;
                        let volume_id = volume.id;
                        self.volumes.push(volume);
                        self.writable_volumes.insert(volume_id);
                        return Ok(volume_id);
                    }
                }
            }
        }
    }

    fn try_get_writable_volume(&mut self, length: usize) -> Result<usize> {
        let volume_id =
            self.random_writable_volume()
                .into_result()
                .or_else(|_| -> Result<usize> {
                    let volume =
                        Volume::new(&self.volumes_dir, self.volumes.len(), self.volume_size)?;
                    let volume_id = volume.id;
                    self.volumes.push(volume);
                    self.writable_volumes.insert(volume_id);
                    Ok(volume_id)
                })?;
        let volume: &mut Volume = self
            .volumes
            .get_mut(volume_id)
            .ok_or(Error::volume(error::VolumeError::not_found(volume_id)))?;
        if volume.max_length - volume.current_length < length as u64 {
            log::warn!(
                "volume almost full. max: {}, current: {}, todo: {}",
                volume.max_length,
                volume.current_length,
                length
            );
            return Err(Error::retry(Error::directory(
                error::DirectoryError::GetWritableVolume,
            )));
        }
        Ok(volume.id)
    }

    pub fn read<K>(&self, key: K) -> Result<Needle>
    where
        K: Into<String>,
    {
        let key = key.into();
        let volume_id = self
            .needle_map
            .get(&key)
            .ok_or(Error::not_found(format!("path: {}", key)))?;
        let volume: &Volume = self
            .volumes
            .get(*volume_id)
            .ok_or(Error::not_found(format!(
                "path: {}, got volume id: {}",
                key, *volume_id
            )))?;
        Ok(volume.get(key)?)
    }

    // Notice: this is not randomly, different from golang
    // I have no idea about how to test it
    fn random_writable_volume(&self) -> Option<usize> {
        use rand::Rng;
        let length = self.writable_volumes.len();
        if length == 0 {
            return None;
        }
        let index = rand::thread_rng().gen::<i64>() as usize;
        let index = index % self.writable_volumes.len();
        let volume_id = self.writable_volumes.iter().skip(index).next()?;
        assert_eq!(length, self.writable_volumes.len());
        Some(*volume_id)
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
    use std::env;

    #[test]
    fn foo1() {
        env_logger::init();
        log::log_enabled!(log::Level::Trace);
        let testdata_dir = env::current_dir().unwrap().join("testdata");
        // std::fs::remove_dir_all(testdata_dir.as_path());
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
                    length: data1.len(),
                    body: NeedleBody::SinglePart(data1),
                };
                directory.write("/path/to/file/1", needle).unwrap();
            }
            {
                log::debug!("test2",);
                let needle = Needle {
                    length: data2.len(),
                    body: NeedleBody::SinglePart(data2),
                };
                directory.write("/path/to/file/2", needle).unwrap();
            }
            {
                log::debug!("test3",);
                let needle = Needle {
                    length: data3.len(),
                    body: NeedleBody::SinglePart(data3),
                };
                directory.write("/path/to/file/3", needle).unwrap();
            }
            {
                log::debug!("test4",);
                let needle = Needle {
                    length: data4.len(),
                    body: NeedleBody::SinglePart(data4),
                };
                directory.write("/path/to/file/4", needle).unwrap();
            }
            {
                log::debug!("test5",);
                let needle = Needle {
                    length: data5.len(),
                    body: NeedleBody::SinglePart(data5),
                };
                directory.write("/path/to/file/5", needle).unwrap();
            }
            {
                log::debug!("test6",);
                let needle = Needle {
                    length: data6.len(),
                    body: NeedleBody::SinglePart(data6),
                };
                directory.write("/path/to/file/6", needle).unwrap();
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
                    length,
                    body: NeedleBody::MultiParts(rx),
                };
                println!("write test7",);
                directory.write("/path/to/file/7", needle).unwrap();
            }
        }
        // read
        {
            let needle1 = directory.read("/path/to/file/1").unwrap();
            check_needle_body(needle1, "data1: hello world data1\n");
            let needle2 = directory.read("/path/to/file/2").unwrap();
            check_needle_body(needle2, "data2: hello world data2\n");
            let needle3 = directory.read("/path/to/file/3").unwrap();
            check_needle_body(needle3, "data3: hello world data3\n");
            let needle4 = directory.read("/path/to/file/4").unwrap();
            check_needle_body(needle4, "data4: hello world data4\n");
            let needle5 = directory.read("/path/to/file/5").unwrap();
            check_needle_body(needle5, "data5: hello world data5\n");
            let needle6 = directory.read("/path/to/file/6").unwrap();
            check_needle_body(needle6, "data6: hello world data6\n");
            let needle7 = directory.read("/path/to/file/7").unwrap();
            check_needle_body(needle7, "data7_1: hello world data7_1\ndata7_2: hello world data7_2\ndata7_3: hello world data7_3\n");
        }
    }

    fn check_needle_body(needle: Needle, data: &str) {
        assert_eq!(needle.length, data.len());
        match needle.body {
            NeedleBody::SinglePart(body) => {
                assert_eq!(body.len(), data.len());
                assert_eq!(body.as_ref(), data.as_bytes());
            }
            // TODO: test read multiparts
            NeedleBody::MultiParts(body_chain) => {}
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