// extern crate bytes;

pub mod directory;
pub mod master;
pub mod storage;
pub mod weaver;

use weaver::*;

impl weaver::ReplicaReplacement {
    pub fn replica_count(&self) -> usize {
        self.data_center_count as usize * self.rack_count as usize * self.node_count as usize
    }
}
