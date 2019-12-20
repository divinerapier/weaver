use weaver_proto::directory::*;

use std::collections::HashMap;
use std::path::Path;
use std::pin::Pin;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc, RwLock,
};

use futures::future::FutureExt;
use futures::Stream;
use tonic::{Request, Response, Status};

pub struct DirectoryService<S>
where
    S: crate::directory::DirectoryStorage,
{
    pub storage: S,
}

impl<'a, S> DirectoryService<S>
where
    S: crate::directory::DirectoryStorage,
{
    pub fn new(storage: S) -> DirectoryService<S>
    where
        S: crate::directory::storage::DirectoryStorage,
    {
        DirectoryService { storage }
    }
}

#[tonic::async_trait]
impl<S> server::Directory for DirectoryService<S>
where
    S: crate::directory::DirectoryStorage + 'static,
{
    async fn lookup_entry(
        &self,
        request: Request<LookupEntryRequest>,
    ) -> Result<Response<LookupEntryResponse>, Status> {
        use super::Chunk;
        use weaver_proto::weaver::Entry;
        let request: LookupEntryRequest = request.into_inner();
        let key: String = request.key;
        Ok(Response::new(LookupEntryResponse {
            entry: self.storage.retrieve(&key).await?.map(|chunks| {
                let chunks: Vec<Chunk> = chunks;
                {
                    Entry {
                        key,
                        attribute: None,
                        chunks,
                    }
                }
            }),
        }))
    }

    type ListEntriesStream =
        Pin<Box<dyn Stream<Item = Result<ListEntriesResponse, Status>> + Send + Sync + 'static>>;

    async fn list_entries(
        &self,
        request: Request<ListEntriesRequest>,
    ) -> Result<Response<Self::ListEntriesStream>, tonic::Status> {
        let request = request.into_inner();
        let children = self
            .storage
            .list(&request.directory, request.offset, request.limit)
            .await?;

        let iter = children.map(|child| Ok(ListEntriesResponse { entry: child }));

        let entries: Vec<Result<ListEntriesResponse, Status>> = iter.collect();

        Ok(Response::new(
            Box::pin(futures::stream::iter(entries)) as Self::ListEntriesStream
        ))
    }

    async fn create_entry(
        &self,
        _request: Request<CreateEntryRequest>,
    ) -> Result<Response<CreateEntryResponse>, Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
    async fn update_entry(
        &self,
        _request: Request<UpdateEntryRequest>,
    ) -> Result<Response<UpdateEntryResponse>, Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
    async fn delete_entry(
        &self,
        _request: Request<DeleteEntryRequest>,
    ) -> Result<Response<DeleteEntryResponse>, Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
    async fn assign_volume(
        &self,
        _request: Request<AssignVolumeRequest>,
    ) -> Result<Response<AssignVolumeResponse>, Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
    async fn lookup_volume(
        &self,
        _request: Request<LookupVolumeRequest>,
    ) -> Result<Response<LookupVolumeResponse>, Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
    async fn delete_collection(
        &self,
        _request: Request<DeleteCollectionRequest>,
    ) -> Result<Response<DeleteCollectionResponse>, Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
    async fn statistics(
        &self,
        _request: tonic::Request<weaver_proto::directory::StatisticsRequest>,
    ) -> Result<tonic::Response<weaver_proto::directory::StatisticsResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
}
