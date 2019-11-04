use crate::{Error, Result};
use futures::stream::StreamExt;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::{Arc, RwLock};

/// Directory is a manager that keep the mapper from
/// filepath to pair of voluem id and file id
pub struct Directory<S>
where
    S: DirectoryStorage,
{
    storage: S,
}

#[tonic::async_trait]
pub trait DirectoryStorage {
    type ListDirectoryStream;

    async fn insert_entry(&mut self, entry: Entry) -> Result<()>;
    async fn update_entry(&mut self, entry: Entry) -> Result<()>;
    async fn find_entry(&self, path: String) -> Result<Entry>;
    async fn delete_entry(&mut self, entry: Entry) -> Result<()>;
    async fn list_entries(&self, path: String) -> Result<Self::ListDirectoryStream>;
}

#[derive(Copy, Clone)]
pub enum FileType {
    File = 1,
    Dir = 2,
}

#[derive(Clone)]
pub struct Attr {
    pub mtime: chrono::DateTime<chrono::Utc>, // time of last modification
    pub crtime: chrono::DateTime<chrono::Utc>, // time of creation (OS X only)
    pub mode: FileType,                       // file mode
    pub uid: u32,                             // owner uid
    pub gid: u32,                             // group gid
    pub mime: String,                         // mime type
    pub replication: String,                  // replication
    pub collection: String,                   // collection name
    pub ttl_sec: i32,                         // ttl in seconds
    pub user_name: String,
    pub group_names: Vec<String>,
    pub symlink_target: String,
}

pub struct MemoryDirectoryStorage {
    /// Store all entries, including directories and regular files.
    /// The key of hashmap indicates the full path of the entry.
    /// If the entry is a directory, the value of the hash map will
    /// contain its children name.
    pub entries: Arc<RwLock<HashMap<String, (Entry, Option<Vec<String>>)>>>,
}

#[derive(Clone)]
pub struct Entry {
    pub fullpath: String,
    pub attr: Attr,
}

impl Entry {
    pub fn is_dir(&self) -> bool {
        match self.attr.mode {
            FileType::Dir => true,
            _ => false,
        }
    }

    pub fn split_filename(&self) -> (&str, &str) {
        if self.fullpath == "" {
            return ("", "");
        }
        if self.fullpath == "/" {
            return ("/", "");
        }
        let mut fullpath: &str = &self.fullpath;
        if fullpath.chars().last().unwrap() == '/' {
            fullpath = &fullpath[0..fullpath.len() - 1];
        }

        match fullpath.rfind('/') {
            Some(pos) => (&fullpath[0..pos], &fullpath[pos + 1..]),
            None => ("/", fullpath),
        }
    }
}

#[tonic::async_trait]
impl DirectoryStorage for MemoryDirectoryStorage {
    async fn insert_entry(&mut self, entry: Entry) -> Result<()> {
        {
            let map = self.entries.read().unwrap();
            if map.contains_key(&entry.fullpath) {
                return Ok(());
            }
        }
        let mut entries = self.entries.write().unwrap();
        let fullpath = &entry.fullpath;
        let (dir, filename) = entry.split_filename();
        match entries
            .get_mut(dir)
            .expect(&format!("parent entry is not found. {}", dir))
        {
            (_, v @ None) => {
                *v = Some(vec![filename.to_owned()]);
            }
            (_, Some(parent)) => {
                parent.push(filename.to_owned());
            }
        }
        let children = if entry.is_dir() { Some(vec![]) } else { None };
        entries.insert(fullpath.to_owned(), (entry, children));
        Ok(())
    }
    async fn update_entry(&mut self, entry: Entry) -> Result<()> {
        let mut map = self.entries.write().unwrap();
        match map.get_mut(&entry.fullpath) {
            Some((ent, _)) => {
                *ent = entry;
                Ok(())
            }
            None => Err(directory_error!("entry is not found. {}", entry.fullpath)),
        }
    }

    async fn find_entry(&self, path: String) -> Result<Entry> {
        let entries = self.entries.read().unwrap();
        match entries.get(&path) {
            Some((entry, _)) => Ok(entry.clone()),
            None => Err(directory_error!("entry is not found. {}", path)),
        }
    }

    async fn delete_entry(&mut self, entry: Entry) -> Result<()> {
        unimplemented!()
    }

    type ListDirectoryStream = tokio::sync::mpsc::Receiver<Result<Entry>>;

    async fn list_entries(&self, path: String) -> Result<Self::ListDirectoryStream> {
        let entries = self.entries.clone();
        let (mut tx, rx) = tokio::sync::mpsc::channel::<Result<Entry>>(1);
        let path = Arc::new(path);
        tokio::spawn(async move {
            {
                let entries = entries.read().unwrap();
                let p: &String = &path;
                match entries.get(p) {
                    Some((_, None)) => {
                        tx.send(Err(directory_error!(
                            "entry is not a directory. {:?}",
                            path
                        )));
                        return;
                    }
                    Some((_, Some(children))) => {
                        let children: &[String] = children;
                        children.iter().for_each(|child| {
                            let path = p.to_owned() + "/" + child;
                            tx.send(Ok(entries.get(&path).unwrap().0.clone()));
                        })
                    }
                    None => {
                        tx.send(Err(directory_error!("entry is not found. {:?}", p)));
                        return;
                    }
                }
            }
        });
        Ok(rx)
    }
}
