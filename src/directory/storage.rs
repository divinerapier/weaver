use crate::Result;

use std::collections::HashMap;
use std::marker::{Send, Sync};
use std::ops::{Index, IndexMut};
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::{Arc, RwLock};

use futures::{Stream, StreamExt};

use rose_tree::petgraph::graph::{DefaultIx, NodeIndex};
use rose_tree::RoseTree;

pub type Chunk = weaver_proto::weaver::Chunk;
pub type Entry = weaver_proto::weaver::Entry;

#[tonic::async_trait]
pub trait DirectoryStorage: Send + Sync {
    async fn create(&mut self, key: &str, chunks: Vec<Chunk>) -> Result<()>;
    async fn update(&mut self, key: &str, chunks: Vec<Chunk>) -> Result<()>;

    /// Retrieve chunks of the target key.
    ///
    /// Ok(Some()): if target key is an regular file object.
    /// Ok(None): if target key is a directory.
    /// Err(_): if any error occurs such as key not found.
    ///
    async fn retrieve(&self, key: &str) -> Result<Option<Vec<Chunk>>>;

    async fn delete(&mut self, key: &str) -> Result<()>;

    async fn list(&self, key: &str, offset: u64, limit: u64) -> Result<Vec<String>>;
}

#[derive(Copy, Clone)]
pub enum FileType {
    File = 1,
    Dir = 2,
}

pub struct MemoryDirectoryStorage<Ix = DefaultIx> {
    pub entries: HashMap<String, NodeIndex<Ix>>,
    pub tree: RoseTree<Option<Vec<Chunk>>>,

    index2entry: Arc<RwLock<HashMap<NodeIndex<Ix>, String>>>,
}

impl MemoryDirectoryStorage {
    pub fn new() -> MemoryDirectoryStorage {
        let (tree, root) = RoseTree::new(None);
        let mut entries = HashMap::new();
        let mut index2entry = HashMap::new();
        entries.insert("/".to_owned(), root);
        index2entry.insert(root, "/".to_owned());
        let index2entry = Arc::new(RwLock::new(index2entry));
        MemoryDirectoryStorage {
            entries,
            tree,
            index2entry,
        }
    }

    fn insert_parents(&mut self, parent: &Path) -> Result<NodeIndex> {
        let parent_path = parent
            .to_str()
            .expect(&format!("can't parse path to &str. {:?}", parent));
        let parent_index = self.entries.get(parent_path);
        if parent_index.is_some() {
            let parent_index = parent_index.unwrap();
            return Ok(*parent_index);
        }
        self.insert_parents(parent.parent().unwrap())?;
        match self.entries.get(parent_path) {
            Some(parent_index) => {
                let child_index = self.tree.add_child(*parent_index, None);
                self.entries.insert(parent_path.to_owned(), child_index);
                Ok(child_index)
            }
            None => Err(directory_error!("unknown key. {:?}", parent)),
        }
    }
}

#[tonic::async_trait]
impl DirectoryStorage for MemoryDirectoryStorage {
    async fn list(&self, key: &str, offset: u64, limit: u64) -> Result<Vec<String>> {
        let index: Option<_> = self.entries.get(key);
        if index.is_none() {
            return Err(directory_error!("unknown key. {}", key));
        }
        let index = *index.unwrap();
        let children = self.tree.children(index);
        let index2entry = self.index2entry.clone();

        let children: Vec<String> = children
            .skip(offset as usize)
            .enumerate()
            .filter(move |(i, _)| *i < limit as usize)
            .map(move |(_, index)| {
                let index2entry = index2entry.read().unwrap();
                let entry: &str = index2entry.index(&index);
                entry.to_owned()
            })
            .collect();

        Ok(children)
    }

    async fn create(&mut self, key: &str, chunks: Vec<Chunk>) -> Result<()> {
        if self.entries.contains_key(key) {
            return Ok(());
        }
        let path = PathBuf::from(&key);
        if !path.is_absolute() {
            return Err(directory_error!("only absolute path is supported. {}", key));
        }
        let parent_index = self.insert_parents(
            path.parent()
                .expect(&format!("can't get parent entry. {:?}", path)),
        )?;

        let child_index = self.tree.add_child(parent_index, Some(chunks));
        self.entries.insert(key.to_owned(), child_index);
        {
            let mut index2entry = self.index2entry.write().unwrap();
            index2entry.insert(child_index, key.to_owned());
        }
        Ok(())
    }

    async fn update(&mut self, key: &str, chunks: Vec<Chunk>) -> Result<()> {
        match self.entries.get_mut(key) {
            Some(index) => {
                let child_node = self.tree.index_mut(*index);
                *child_node = Some(chunks);
                Ok(())
            }
            None => Err(directory_error!("entry not found. {}", key)),
        }
    }

    async fn retrieve(&self, key: &str) -> Result<Option<Vec<Chunk>>> {
        match self.entries.get(key) {
            Some(index) => {
                let chunks: &Option<_> = self.tree.index(*index);
                Ok(chunks.clone())
            }
            None => Err(directory_error!("entry not found. {}", key)),
        }
    }

    async fn delete(&mut self, key: &str) -> Result<()> {
        match self.entries.get(key) {
            Some(&index) => {
                self.tree.remove_node(index);
                self.entries.remove(key);
                {
                    let mut index2entry = self.index2entry.write().unwrap();
                    index2entry.remove(&index);
                }
                Ok(())
            }
            None => Ok(()),
        }
    }
}
