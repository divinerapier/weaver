use futures::{Stream, StreamExt};
use onyxia_proto::*;
use tokio::sync::mpsc;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[derive(Clone, Copy)] // clone trait required by `fn create_directory`
pub struct DirectoryService {}

#[tonic::async_trait]
impl onyxia_proto::directory::server::Directory for DirectoryService {
    async fn assign(
        &self,
        request: tonic::Request<onyxia_proto::directory::AssignRequest>,
    ) -> Result<tonic::Response<onyxia_proto::directory::AssignResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    type KeepaliveStream = mpsc::Receiver<Result<onyxia_proto::directory::KeepaliveResponse, Status>>;

    async fn keepalive(
        &self,
        request: tonic::Request<tonic::Streaming<onyxia_proto::directory::KeepaliveRequest>>,
    ) -> Result<tonic::Response<Self::KeepaliveStream>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
    async fn register_storage_service(
        &self,
        request: tonic::Request<tonic::Streaming<onyxia_proto::directory::RegisterStorageServiceRequest>>,
    ) -> Result<tonic::Response<onyxia_proto::directory::RegisterStorageServiceResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
}
