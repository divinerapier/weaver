use crate::{Error, Result};
use rand::seq::IteratorRandom;
use rand::Rng;
use std::collections::{HashMap, HashSet};

pub struct Node {
    pub rack_id: u64,
    pub data_center_id: u64,

    pub node_id: u64,
    /// the external network address of the server, includes ip and port
    /// because a single machine may run multiple server(not recommended)
    pub address: String,
}

pub struct Rack {
    pub rack_id: u64,
    pub data_center_id: u64,
    pub nodes: HashSet<u64>,
}

pub struct DataCenter {
    pub data_center_id: u64,
    pub racks: HashSet<u64>,
}

pub struct Router {
    pub data_centers: HashMap<u64, DataCenter>,
    pub racks: HashMap<u64, Rack>,
    pub nodes: HashMap<u64, Node>,

    /// lookup the index of nodes where the volume is located
    /// volume_id: [node_id]
    pub volumes: HashMap<u64, Vec<u64>>,

    /// replica_replacement: [volume_id]
    pub volume_replica_replacement: HashMap<crate::storage::volume::ReplicaReplacement, Vec<u64>>,
}

impl Router {
    pub fn get_volume(&self, volume_id: u64) -> Result<Vec<&Node>> {
        let node_indexes = self
            .volumes
            .get(&volume_id)
            .ok_or(boxed_naive!("volume: {} not found", volume_id))?;
        let mut result = vec![];
        for node_index in node_indexes {
            match self.nodes.get(node_index) {
                Some(node) => result.push(node),
                None => continue,
            }
        }
        Ok(result)
    }

    pub fn assign(
        &self,
        count: usize,
        replica_replacement: crate::storage::volume::ReplicaReplacement,
    ) -> Result<Option<Vec<&Node>>> {
        let diff_cent = replica_replacement.diff_data_centers;
        let diff_rack = replica_replacement.diff_rack;
        let diff_node = replica_replacement.diff_node;

        if diff_node == 0 || diff_rack == 0 || diff_cent == 0 {
            return Err(boxed_naive!(
                "invalid replica_replacement: {:?}",
                replica_replacement
            ));
        }

        let volumes_ids = self.volume_replica_replacement.get(&replica_replacement);
        if volumes_ids.is_none() {
            return Ok(None);
        }
        let volumes_ids = volumes_ids.unwrap();

        let mut result = Vec::with_capacity(replica_replacement.count());

        for volume_id in volumes_ids {
            let nodes_ids = self.volumes.get(volume_id);
            if nodes_ids.is_none() {
                continue;
            }
            let nodes_ids = nodes_ids.unwrap();
            for node_id in nodes_ids {
                match self.nodes.get(node_id) {
                    Some(node) => result.push(node),
                    None => {
                        unsafe { result.set_len(0) }
                        break;
                    }
                }
            }
            return Ok(Some(result));
        }

        Ok(None)
    }
}
