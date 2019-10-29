use std::collections::HashMap;
use std::path::Path;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc, RwLock,
};
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};
pub struct DirectoryService {
    /// map volume id to locations
    pub volume_locations: Arc<RwLock<HashMap<u64, String>>>,
    pub dir: weaver::directory::Directory,
    /// the next file_id of each volume
    pub volume_next_fileid: Arc<AtomicU64>,
}

impl DirectoryService {
    pub fn new<P: AsRef<Path>>(path: P, volume_size: u64) -> DirectoryService {
        DirectoryService {
            volume_locations: Arc::new(RwLock::new(HashMap::new())),
            volume_next_fileid: Arc::new(AtomicU64::new(0)),
            dir: weaver::directory::Directory::new(path.as_ref(), volume_size).unwrap(),
        }
    }
}

#[tonic::async_trait]
impl weaver_proto::directory::server::Directory for DirectoryService {
    /// assign a pair of volume_id and file_id to the given needle
    async fn assign(
        &self,
        request: tonic::Request<weaver_proto::directory::AssignRequest>,
    ) -> Result<tonic::Response<weaver_proto::directory::AssignResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    type KeepaliveStream =
        mpsc::Receiver<Result<weaver_proto::directory::KeepaliveResponse, Status>>;

    async fn keepalive(
        &self,
        request: tonic::Request<tonic::Streaming<weaver_proto::directory::KeepaliveRequest>>,
    ) -> Result<tonic::Response<Self::KeepaliveStream>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
    async fn register_storage_service(
        &self,
        request: tonic::Request<
            tonic::Streaming<weaver_proto::directory::RegisterStorageServiceRequest>,
        >,
    ) -> Result<
        tonic::Response<weaver_proto::directory::RegisterStorageServiceResponse>,
        tonic::Status,
    > {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
}
