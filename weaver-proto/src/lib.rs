// pub mod master {
//     tonic::include_proto!("weaver.master");
// }

// pub mod directory {
//     tonic::include_proto!("weaver.directory");
// }
// pub mod storage {
//     tonic::include_proto!("weaver.storage");
// }

// pub mod status {
//     tonic::include_proto!("weaver.status");
// }

pub mod directory;
pub mod master;
pub mod storage;
pub mod weaver;

use weaver::*;
