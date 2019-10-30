#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Heartbeat {
    #[prost(string, tag = "1")]
    pub ip: std::string::String,
    #[prost(uint32, tag = "2")]
    pub port: u32,
    #[prost(string, tag = "3")]
    pub presigned_url: std::string::String,
    #[prost(uint32, tag = "4")]
    pub max_volume_count: u32,
    #[prost(uint64, tag = "5")]
    pub max_file_key: u64,
    #[prost(string, tag = "6")]
    pub data_center: std::string::String,
    #[prost(string, tag = "7")]
    pub rack: std::string::String,
    #[prost(uint32, tag = "8")]
    pub admin_port: u32,
    #[prost(message, repeated, tag = "9")]
    pub volumes: ::std::vec::Vec<super::VolumeInformationMessage>,
    /// delta volumes
    #[prost(message, repeated, tag = "10")]
    pub new_volumes: ::std::vec::Vec<super::VolumeShortInformationMessage>,
    #[prost(message, repeated, tag = "11")]
    pub deleted_volumes: ::std::vec::Vec<super::VolumeShortInformationMessage>,
    #[prost(bool, tag = "12")]
    pub has_no_volumes: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartbeatResponse {
    #[prost(uint64, tag = "1")]
    pub volume_size_limit: u64,
    #[prost(string, tag = "2")]
    pub leader: std::string::String,
    #[prost(string, tag = "3")]
    pub metrics_address: std::string::String,
    #[prost(uint32, tag = "4")]
    pub metrics_interval_seconds: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeepConnectedRequest {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeLocation {
    #[prost(string, tag = "1")]
    pub url: std::string::String,
    #[prost(string, tag = "2")]
    pub presigned_url: std::string::String,
    #[prost(uint32, repeated, tag = "3")]
    pub new_vids: ::std::vec::Vec<u32>,
    #[prost(uint32, repeated, tag = "4")]
    pub deleted_vids: ::std::vec::Vec<u32>,
    /// optional when leader is not itself
    #[prost(string, tag = "5")]
    pub leader: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupVolumeRequest {
    #[prost(string, repeated, tag = "1")]
    pub volume_ids: ::std::vec::Vec<std::string::String>,
    /// optional, a bit faster if provided.
    #[prost(string, tag = "2")]
    pub bucket: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupVolumeResponse {
    #[prost(message, repeated, tag = "1")]
    pub volume_id_locations: ::std::vec::Vec<lookup_volume_response::VolumeIdLocation>,
}
pub mod lookup_volume_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VolumeIdLocation {
        #[prost(string, tag = "1")]
        pub volume_id: std::string::String,
        #[prost(message, repeated, tag = "2")]
        pub locations: ::std::vec::Vec<super::super::Location>,
        #[prost(string, tag = "3")]
        pub error: std::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssignRequest {
    #[prost(uint64, tag = "1")]
    pub count: u64,
    #[prost(message, optional, tag = "2")]
    pub replica_replacement: ::std::option::Option<super::ReplicaReplacement>,
    #[prost(string, tag = "3")]
    pub bucket: std::string::String,
    #[prost(string, tag = "4")]
    pub ttl: std::string::String,
    #[prost(string, tag = "5")]
    pub data_center: std::string::String,
    #[prost(string, tag = "6")]
    pub rack: std::string::String,
    #[prost(string, tag = "7")]
    pub data_node: std::string::String,
    #[prost(uint32, tag = "8")]
    pub memory_map_max_size_mb: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssignResponse {
    #[prost(string, tag = "1")]
    pub fid: std::string::String,
    #[prost(string, tag = "2")]
    pub url: std::string::String,
    #[prost(string, tag = "3")]
    pub presigned_url: std::string::String,
    #[prost(uint64, tag = "4")]
    pub count: u64,
    #[prost(string, tag = "5")]
    pub error: std::string::String,
    #[prost(string, tag = "6")]
    pub auth: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatisticsRequest {
    #[prost(message, optional, tag = "1")]
    pub replica_replacement: ::std::option::Option<super::ReplicaReplacement>,
    #[prost(string, tag = "2")]
    pub bucket: std::string::String,
    #[prost(string, tag = "3")]
    pub ttl: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatisticsResponse {
    #[prost(message, optional, tag = "1")]
    pub replica_replacement: ::std::option::Option<super::ReplicaReplacement>,
    #[prost(string, tag = "2")]
    pub bucket: std::string::String,
    #[prost(string, tag = "3")]
    pub ttl: std::string::String,
    #[prost(uint64, tag = "4")]
    pub total_size: u64,
    #[prost(uint64, tag = "5")]
    pub used_size: u64,
    #[prost(uint64, tag = "6")]
    pub file_count: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BucketListRequest {
    #[prost(bool, tag = "1")]
    pub include_normal_volumes: bool,
    #[prost(bool, tag = "2")]
    pub include_ec_volumes: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BucketListResponse {
    #[prost(message, repeated, tag = "1")]
    pub buckets: ::std::vec::Vec<super::Bucket>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BucketDeleteRequest {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BucketDeleteResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeListRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeListResponse {
    #[prost(uint64, tag = "2")]
    pub volume_size_limit_mb: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupEcVolumeRequest {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupEcVolumeResponse {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
    #[prost(message, repeated, tag = "2")]
    pub shard_id_locations: ::std::vec::Vec<lookup_ec_volume_response::EcShardIdLocation>,
}
pub mod lookup_ec_volume_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EcShardIdLocation {
        #[prost(uint32, tag = "1")]
        pub shard_id: u32,
        #[prost(message, repeated, tag = "2")]
        pub locations: ::std::vec::Vec<super::super::Location>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMasterConfigurationRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMasterConfigurationResponse {
    #[prost(string, tag = "1")]
    pub metrics_address: std::string::String,
    #[prost(uint32, tag = "2")]
    pub metrics_interval_seconds: u32,
}
#[doc = r" Generated client implementations."]
pub mod client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct MasterClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MasterClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            tonic::transport::Endpoint::new(dst).map(|c| Self::new(c.channel()))
        }
    }
    impl<T> MasterClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
        <T::ResponseBody as HttpBody>::Data: Into<bytes::Bytes> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        #[doc = r" Check if the service is ready."]
        pub async fn ready(&mut self) -> Result<(), tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })
        }
        pub async fn send_heartbeat<S>(
            &mut self,
            request: tonic::Request<S>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::HeartbeatResponse>>, tonic::Status>
        where
            S: Stream<Item = super::Heartbeat> + Send + 'static,
        {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static("/weaver.master.Master/SendHeartbeat");
            self.inner.streaming(request, path, codec).await
        }
        pub async fn keep_connected<S>(
            &mut self,
            request: tonic::Request<S>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::VolumeLocation>>, tonic::Status>
        where
            S: Stream<Item = super::KeepConnectedRequest> + Send + 'static,
        {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static("/weaver.master.Master/KeepConnected");
            self.inner.streaming(request, path, codec).await
        }
        pub async fn lookup_volume(
            &mut self,
            request: tonic::Request<super::LookupVolumeRequest>,
        ) -> Result<tonic::Response<super::LookupVolumeResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static("/weaver.master.Master/LookupVolume");
            self.inner.unary(request, path, codec).await
        }
        pub async fn assign(
            &mut self,
            request: tonic::Request<super::AssignRequest>,
        ) -> Result<tonic::Response<super::AssignResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static("/weaver.master.Master/Assign");
            self.inner.unary(request, path, codec).await
        }
        pub async fn statistics(
            &mut self,
            request: tonic::Request<super::StatisticsRequest>,
        ) -> Result<tonic::Response<super::StatisticsResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static("/weaver.master.Master/Statistics");
            self.inner.unary(request, path, codec).await
        }
        pub async fn bucket_list(
            &mut self,
            request: tonic::Request<super::BucketListRequest>,
        ) -> Result<tonic::Response<super::BucketListResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static("/weaver.master.Master/BucketList");
            self.inner.unary(request, path, codec).await
        }
        pub async fn bucket_delete(
            &mut self,
            request: tonic::Request<super::BucketDeleteRequest>,
        ) -> Result<tonic::Response<super::BucketDeleteResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static("/weaver.master.Master/BucketDelete");
            self.inner.unary(request, path, codec).await
        }
        pub async fn volume_list(
            &mut self,
            request: tonic::Request<super::VolumeListRequest>,
        ) -> Result<tonic::Response<super::VolumeListResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static("/weaver.master.Master/VolumeList");
            self.inner.unary(request, path, codec).await
        }
        pub async fn lookup_ec_volume(
            &mut self,
            request: tonic::Request<super::LookupEcVolumeRequest>,
        ) -> Result<tonic::Response<super::LookupEcVolumeResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static("/weaver.master.Master/LookupEcVolume");
            self.inner.unary(request, path, codec).await
        }
        pub async fn get_master_configuration(
            &mut self,
            request: tonic::Request<super::GetMasterConfigurationRequest>,
        ) -> Result<tonic::Response<super::GetMasterConfigurationResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static(
                "/weaver.master.Master/GetMasterConfiguration",
            );
            self.inner.unary(request, path, codec).await
        }
    }
    impl<T: Clone> Clone for MasterClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with MasterServer."]
    #[async_trait]
    pub trait Master: Send + Sync + 'static {
        #[doc = "Server streaming response type for the SendHeartbeat method."]
        type SendHeartbeatStream: Stream<Item = Result<super::HeartbeatResponse, tonic::Status>>
            + Send
            + 'static;
        async fn send_heartbeat(
            &self,
            request: tonic::Request<tonic::Streaming<super::Heartbeat>>,
        ) -> Result<tonic::Response<Self::SendHeartbeatStream>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        #[doc = "Server streaming response type for the KeepConnected method."]
        type KeepConnectedStream: Stream<Item = Result<super::VolumeLocation, tonic::Status>>
            + Send
            + 'static;
        async fn keep_connected(
            &self,
            request: tonic::Request<tonic::Streaming<super::KeepConnectedRequest>>,
        ) -> Result<tonic::Response<Self::KeepConnectedStream>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn lookup_volume(
            &self,
            request: tonic::Request<super::LookupVolumeRequest>,
        ) -> Result<tonic::Response<super::LookupVolumeResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn assign(
            &self,
            request: tonic::Request<super::AssignRequest>,
        ) -> Result<tonic::Response<super::AssignResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn statistics(
            &self,
            request: tonic::Request<super::StatisticsRequest>,
        ) -> Result<tonic::Response<super::StatisticsResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn bucket_list(
            &self,
            request: tonic::Request<super::BucketListRequest>,
        ) -> Result<tonic::Response<super::BucketListResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn bucket_delete(
            &self,
            request: tonic::Request<super::BucketDeleteRequest>,
        ) -> Result<tonic::Response<super::BucketDeleteResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn volume_list(
            &self,
            request: tonic::Request<super::VolumeListRequest>,
        ) -> Result<tonic::Response<super::VolumeListResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn lookup_ec_volume(
            &self,
            request: tonic::Request<super::LookupEcVolumeRequest>,
        ) -> Result<tonic::Response<super::LookupEcVolumeResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn get_master_configuration(
            &self,
            request: tonic::Request<super::GetMasterConfigurationRequest>,
        ) -> Result<tonic::Response<super::GetMasterConfigurationResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
    }
    #[derive(Clone, Debug)]
    pub struct MasterServer<T: Master> {
        inner: Arc<T>,
    }
    #[derive(Clone, Debug)]
    #[doc(hidden)]
    pub struct MasterServerSvc<T: Master> {
        inner: Arc<T>,
    }
    impl<T: Master> MasterServer<T> {
        #[doc = "Create a new MasterServer from a type that implements Master."]
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            Self::from_shared(inner)
        }
        pub fn from_shared(inner: Arc<T>) -> Self {
            Self { inner }
        }
    }
    impl<T: Master> MasterServerSvc<T> {
        pub fn new(inner: Arc<T>) -> Self {
            Self { inner }
        }
    }
    impl<T: Master, R> Service<R> for MasterServer<T> {
        type Response = MasterServerSvc<T>;
        type Error = Never;
        type Future = Ready<Result<Self::Response, Self::Error>>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, _: R) -> Self::Future {
            ok(MasterServerSvc::new(self.inner.clone()))
        }
    }
    impl<T: Master> Service<http::Request<HyperBody>> for MasterServerSvc<T> {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<HyperBody>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/weaver.master.Master/SendHeartbeat" => {
                    struct SendHeartbeatSvc<T: Master>(pub Arc<T>);
                    impl<T: Master> tonic::server::StreamingService<super::Heartbeat> for SendHeartbeatSvc<T> {
                        type Response = super::HeartbeatResponse;
                        type ResponseStream = T::SendHeartbeatStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::Heartbeat>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.send_heartbeat(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = SendHeartbeatSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.master.Master/KeepConnected" => {
                    struct KeepConnectedSvc<T: Master>(pub Arc<T>);
                    impl<T: Master> tonic::server::StreamingService<super::KeepConnectedRequest>
                        for KeepConnectedSvc<T>
                    {
                        type Response = super::VolumeLocation;
                        type ResponseStream = T::KeepConnectedStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::KeepConnectedRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.keep_connected(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = KeepConnectedSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.master.Master/LookupVolume" => {
                    struct LookupVolumeSvc<T: Master>(pub Arc<T>);
                    impl<T: Master> tonic::server::UnaryService<super::LookupVolumeRequest> for LookupVolumeSvc<T> {
                        type Response = super::LookupVolumeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LookupVolumeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.lookup_volume(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = LookupVolumeSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.master.Master/Assign" => {
                    struct AssignSvc<T: Master>(pub Arc<T>);
                    impl<T: Master> tonic::server::UnaryService<super::AssignRequest> for AssignSvc<T> {
                        type Response = super::AssignResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AssignRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.assign(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = AssignSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.master.Master/Statistics" => {
                    struct StatisticsSvc<T: Master>(pub Arc<T>);
                    impl<T: Master> tonic::server::UnaryService<super::StatisticsRequest> for StatisticsSvc<T> {
                        type Response = super::StatisticsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StatisticsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.statistics(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = StatisticsSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.master.Master/BucketList" => {
                    struct BucketListSvc<T: Master>(pub Arc<T>);
                    impl<T: Master> tonic::server::UnaryService<super::BucketListRequest> for BucketListSvc<T> {
                        type Response = super::BucketListResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BucketListRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.bucket_list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = BucketListSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.master.Master/BucketDelete" => {
                    struct BucketDeleteSvc<T: Master>(pub Arc<T>);
                    impl<T: Master> tonic::server::UnaryService<super::BucketDeleteRequest> for BucketDeleteSvc<T> {
                        type Response = super::BucketDeleteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BucketDeleteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.bucket_delete(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = BucketDeleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.master.Master/VolumeList" => {
                    struct VolumeListSvc<T: Master>(pub Arc<T>);
                    impl<T: Master> tonic::server::UnaryService<super::VolumeListRequest> for VolumeListSvc<T> {
                        type Response = super::VolumeListResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VolumeListRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.volume_list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = VolumeListSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.master.Master/LookupEcVolume" => {
                    struct LookupEcVolumeSvc<T: Master>(pub Arc<T>);
                    impl<T: Master> tonic::server::UnaryService<super::LookupEcVolumeRequest> for LookupEcVolumeSvc<T> {
                        type Response = super::LookupEcVolumeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LookupEcVolumeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.lookup_ec_volume(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = LookupEcVolumeSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.master.Master/GetMasterConfiguration" => {
                    struct GetMasterConfigurationSvc<T: Master>(pub Arc<T>);
                    impl<T: Master>
                        tonic::server::UnaryService<super::GetMasterConfigurationRequest>
                        for GetMasterConfigurationSvc<T>
                    {
                        type Response = super::GetMasterConfigurationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetMasterConfigurationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_master_configuration(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetMasterConfigurationSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
}
