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
impl weaver_proto::master::server::Master for MasterService {
    #[doc = "Server streaming response type for the SendHeartbeat method."]
    type SendHeartbeatStream =
        tokio::sync::mpsc::Receiver<Result<weaver_proto::master::HeartbeatResponse, Status>>;
    // type SendHeartbeatStream = std::pin::Pin<
    //     Box<
    //         dyn Stream<Item = Result<weaver_proto::master::HeartbeatResponse, Status>>
    //             + Send
    //             + Sync
    //             + 'static,
    //     >,
    // >;

    #[doc = "Server streaming response type for the KeepConnected method."]
    type KeepConnectedStream =
        tokio::sync::mpsc::Receiver<Result<weaver_proto::master::VolumeLocation, Status>>;

    async fn send_heartbeat(
        &self,
        request: tonic::Request<tonic::Streaming<weaver_proto::master::Heartbeat>>,
    ) -> Result<tonic::Response<Self::SendHeartbeatStream>, tonic::Status> {
        // {
        //     let stream = request.into_inner();
        //     // let mut stream = stream;
        //     let output = async_stream::try_stream! {
        //             while let Some(heartbeat) = stream.next().await {
        //             futures::pin_mut!(stream);
        //             let heartbeat: weaver_proto::master::Heartbeat = heartbeat?;
        //             for volume in heartbeat.volumes {
        //                 // update exists volumes
        //             }
        //             for new_volume in heartbeat.new_volumes {
        //                 // add new volumes
        //             }
        //             for deleted_volume in heartbeat.deleted_volumes {
        //                 // delete exists volume
        //             }
        //             yield weaver_proto::master::HeartbeatResponse {
        //                 volume_size_limit: 0,
        //                 leader: "127.0.0.1:12345".to_owned(),
        //                 metrics_address: "127.0.0.1:23456".to_owned(),
        //                 metrics_interval_seconds: 1,
        //             };
        //         }
        //     };

        //     Ok(tonic::Response::new(Box::new(output)
        //         as std::pin::Pin<
        //             Box<
        //                 dyn Stream<Item = Result<weaver_proto::master::HeartbeatResponse, Status>>
        //                     + Send
        //                     + Sync
        //                     + 'static,
        //             >,
        //         >))
        // }
        // Err(tonic::Status::unimplemented("Not yet implemented"))

        let stream = request.into_inner();

        let (mut tx, rx) = tokio::sync::mpsc::channel(1);
        tokio::spawn(async move {
            futures::pin_mut!(stream);
        });
        Ok(Response::new(rx))
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
    async fn bucket_list(
        &self,
        request: tonic::Request<weaver_proto::master::BucketListRequest>,
    ) -> Result<tonic::Response<weaver_proto::master::BucketListResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
    async fn bucket_delete(
        &self,
        request: tonic::Request<weaver_proto::master::BucketDeleteRequest>,
    ) -> Result<tonic::Response<weaver_proto::master::BucketDeleteResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
}
