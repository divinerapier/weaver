use futures::{Stream, StreamExt};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc, RwLock,
};
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};

pub struct MasterService {
    not_ready_volumes: Arc<RwLock<HashMap<u64, ()>>>,
    actived_volumes: Arc<RwLock<HashMap<u64, ()>>>,
}

impl MasterService {
    pub fn new() -> MasterService {
        MasterService {
            not_ready_volumes: Arc::new(RwLock::new(HashMap::new())),
            actived_volumes: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[tonic::async_trait]
impl proto::master::master_server::Master for MasterService {
    #[doc = "Server streaming response type for the SendHeartbeat method."]
    type HeartbeatStream =
        tokio::sync::mpsc::Receiver<Result<proto::master::HeartbeatResponse, Status>>;

    async fn heartbeat(
        &self,
        request: tonic::Request<tonic::Streaming<proto::master::HeartbeatRequest>>,
    ) -> Result<tonic::Response<Self::HeartbeatStream>, tonic::Status> {
        let stream = request.into_inner();

        let (mut tx, rx) = tokio::sync::mpsc::channel(1);
        tokio::spawn(async move {
            futures::pin_mut!(stream);
        });
        Ok(Response::new(rx))
    }

    async fn lookup_volume(
        &self,
        _request: tonic::Request<proto::master::LookupVolumeRequest>,
    ) -> Result<tonic::Response<proto::master::LookupVolumeResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn assign(
        &self,
        _request: tonic::Request<proto::master::AssignRequest>,
    ) -> Result<tonic::Response<proto::master::AssignResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
}
