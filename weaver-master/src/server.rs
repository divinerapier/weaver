use std::collections::HashMap;
use std::path::Path;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc, RwLock,
};
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};
pub struct MasterService {}

impl MasterService {
    pub fn new() -> MasterService {
        MasterService {}
    }
}

#[tonic::async_trait]
impl weaver_proto::master::server::Master for MasterService {
    #[doc = "Server streaming response type for the SendHeartbeat method."]
    type SendHeartbeatStream =
        tokio::sync::mpsc::Receiver<Result<weaver_proto::master::HeartbeatResponse, Status>>;
    #[doc = "Server streaming response type for the KeepConnected method."]
    type KeepConnectedStream =
        tokio::sync::mpsc::Receiver<Result<weaver_proto::master::VolumeLocation, Status>>;

    async fn send_heartbeat(
        &self,
        request: tonic::Request<tonic::Streaming<weaver_proto::master::Heartbeat>>,
    ) -> Result<tonic::Response<Self::SendHeartbeatStream>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn keep_connected(
        &self,
        request: tonic::Request<tonic::Streaming<weaver_proto::master::KeepConnectedRequest>>,
    ) -> Result<tonic::Response<Self::KeepConnectedStream>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn lookup_volume(
        &self,
        request: tonic::Request<weaver_proto::master::LookupVolumeRequest>,
    ) -> Result<tonic::Response<weaver_proto::master::LookupVolumeResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn assign(
        &self,
        request: tonic::Request<weaver_proto::master::AssignRequest>,
    ) -> Result<tonic::Response<weaver_proto::master::AssignResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn statistics(
        &self,
        request: tonic::Request<weaver_proto::master::StatisticsRequest>,
    ) -> Result<tonic::Response<weaver_proto::master::StatisticsResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn collection_list(
        &self,
        request: tonic::Request<weaver_proto::master::CollectionListRequest>,
    ) -> Result<tonic::Response<weaver_proto::master::CollectionListResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn collection_delete(
        &self,
        request: tonic::Request<weaver_proto::master::CollectionDeleteRequest>,
    ) -> Result<tonic::Response<weaver_proto::master::CollectionDeleteResponse>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn volume_list(
        &self,
        request: tonic::Request<weaver_proto::master::VolumeListRequest>,
    ) -> Result<tonic::Response<weaver_proto::master::VolumeListResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn lookup_ec_volume(
        &self,
        request: tonic::Request<weaver_proto::master::LookupEcVolumeRequest>,
    ) -> Result<tonic::Response<weaver_proto::master::LookupEcVolumeResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn get_master_configuration(
        &self,
        request: tonic::Request<weaver_proto::master::GetMasterConfigurationRequest>,
    ) -> Result<tonic::Response<weaver_proto::master::GetMasterConfigurationResponse>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
}
