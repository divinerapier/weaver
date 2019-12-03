use std::collections::HashMap;
use std::path::Path;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc, RwLock,
};
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};
pub struct DirectoryService<S>
where
    S: crate::directory::DirectoryStorage,
{
    /// map volume id to locations
    pub storage: crate::directory::Directory<S>,
}

impl<S> DirectoryService<S>
where
    S: crate::directory::DirectoryStorage,
{
    pub fn new(storage: S) -> DirectoryService<S>
    where
        S: crate::directory::DirectoryStorage,
    {
        DirectoryService {
            storage: crate::directory::Directory::new(storage),
        }
    }
}

#[tonic::async_trait]
impl<S> weaver_proto::directory::server::Directory for DirectoryService<S>
where
    S: crate::directory::DirectoryStorage + 'static,
{
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