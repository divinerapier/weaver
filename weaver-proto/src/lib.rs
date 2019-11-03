use weaver::*;

pub mod weaver {
    tonic::include_proto!("weaver");
}

pub mod directory {
    tonic::include_proto!("weaver.directory");
}

pub mod storage {
    tonic::include_proto!("weaver.storage");
}

pub mod master {
    tonic::include_proto!("weaver.master");
}

impl weaver::ReplicaReplacement {
    pub fn replica_count(&self) -> usize {
        self.data_center_count as usize * self.rack_count as usize * self.node_count as usize
    }
}
