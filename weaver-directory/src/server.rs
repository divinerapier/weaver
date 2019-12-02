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
    async fn lookup_entry(
        &self,
        _request: tonic::Request<weaver_proto::directory::LookupEntryRequest>,
    ) -> Result<tonic::Response<weaver_proto::directory::LookupEntryResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
    async fn list_entries(
        &self,
        _request: tonic::Request<weaver_proto::directory::ListEntriesRequest>,
    ) -> Result<tonic::Response<weaver_proto::directory::ListEntriesResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
    async fn create_entry(
        &self,
        _request: tonic::Request<weaver_proto::directory::CreateEntryRequest>,
    ) -> Result<tonic::Response<weaver_proto::directory::CreateEntryResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
    async fn update_entry(
        &self,
        _request: tonic::Request<weaver_proto::directory::UpdateEntryRequest>,
    ) -> Result<tonic::Response<weaver_proto::directory::UpdateEntryResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
    async fn delete_entry(
        &self,
        _request: tonic::Request<weaver_proto::directory::DeleteEntryRequest>,
    ) -> Result<tonic::Response<weaver_proto::directory::DeleteEntryResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
    async fn assign_volume(
        &self,
        _request: tonic::Request<weaver_proto::directory::AssignVolumeRequest>,
    ) -> Result<tonic::Response<weaver_proto::directory::AssignVolumeResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
    async fn lookup_volume(
        &self,
        _request: tonic::Request<weaver_proto::directory::LookupVolumeRequest>,
    ) -> Result<tonic::Response<weaver_proto::directory::LookupVolumeResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
    async fn delete_collection(
        &self,
        _request: tonic::Request<weaver_proto::directory::DeleteCollectionRequest>,
    ) -> Result<tonic::Response<weaver_proto::directory::DeleteCollectionResponse>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
    async fn statistics(
        &self,
        _request: tonic::Request<weaver_proto::directory::StatisticsRequest>,
    ) -> Result<tonic::Response<weaver_proto::directory::StatisticsResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
}
