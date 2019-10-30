/// LookupEntryRequest is the type of message used to look up an entry
/// with a given directory and name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupEntryRequest {
    /// lookup in
    #[prost(string, tag = "1")]
    pub directory: std::string::String,
    /// name of entry
    #[prost(string, tag = "2")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupEntryResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::std::option::Option<super::Status>,
    #[prost(message, optional, tag = "2")]
    pub entry: ::std::option::Option<super::Entry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntriesRequest {
    #[prost(string, tag = "1")]
    pub directory: std::string::String,
    /// begin at
    #[prost(string, tag = "2")]
    pub offset: std::string::String,
    #[prost(uint32, tag = "3")]
    pub limit: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntriesResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::std::option::Option<super::Status>,
    #[prost(message, repeated, tag = "2")]
    pub entries: ::std::vec::Vec<super::Entry>,
    #[prost(string, tag = "3")]
    pub next: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEntryRequest {
    #[prost(string, tag = "1")]
    pub directory: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub entry: ::std::option::Option<super::Entry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEntryResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEntryRequest {
    #[prost(string, tag = "1")]
    pub directory: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub entry: ::std::option::Option<super::Entry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEntryResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEntryRequest {
    #[prost(string, tag = "1")]
    pub directory: std::string::String,
    #[prost(string, tag = "2")]
    pub name: std::string::String,
    /// bool is_directory = 3;
    #[prost(bool, tag = "4")]
    pub is_delete_data: bool,
    #[prost(bool, tag = "5")]
    pub is_recursive: bool,
    #[prost(bool, tag = "6")]
    pub ignore_recursive_error: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEntryResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AtomicRenameEntryRequest {
    #[prost(string, tag = "1")]
    pub old_directory: std::string::String,
    #[prost(string, tag = "2")]
    pub old_name: std::string::String,
    #[prost(string, tag = "3")]
    pub new_directory: std::string::String,
    #[prost(string, tag = "4")]
    pub new_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AtomicRenameEntryResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssignVolumeRequest {
    #[prost(int32, tag = "1")]
    pub count: i32,
    #[prost(string, tag = "2")]
    pub collection: std::string::String,
    #[prost(message, optional, tag = "3")]
    pub replica_replacement: ::std::option::Option<super::ReplicaReplacement>,
    #[prost(int32, tag = "4")]
    pub ttl_sec: i32,
    #[prost(string, tag = "5")]
    pub data_center: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssignVolumeResponse {
    #[prost(string, tag = "1")]
    pub file_id: std::string::String,
    #[prost(string, tag = "2")]
    pub url: std::string::String,
    #[prost(string, tag = "3")]
    pub presigned_url: std::string::String,
    #[prost(int32, tag = "4")]
    pub count: i32,
    #[prost(string, tag = "5")]
    pub auth: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupVolumeRequest {
    #[prost(string, repeated, tag = "1")]
    pub volume_ids: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupVolumeResponse {
    #[prost(map = "string, message", tag = "1")]
    pub locations_map: ::std::collections::HashMap<std::string::String, super::Locations>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCollectionRequest {
    #[prost(string, tag = "1")]
    pub collection: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCollectionResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatisticsRequest {
    #[prost(message, optional, tag = "1")]
    pub replica_replacement: ::std::option::Option<super::ReplicaReplacement>,
    #[prost(string, tag = "2")]
    pub collection: std::string::String,
    #[prost(string, tag = "3")]
    pub ttl: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatisticsResponse {
    #[prost(message, optional, tag = "1")]
    pub replica_replacement: ::std::option::Option<super::ReplicaReplacement>,
    #[prost(string, tag = "2")]
    pub collection: std::string::String,
    #[prost(string, tag = "3")]
    pub ttl: std::string::String,
    #[prost(uint64, tag = "4")]
    pub total_size: u64,
    #[prost(uint64, tag = "5")]
    pub used_size: u64,
    #[prost(uint64, tag = "6")]
    pub file_count: u64,
}
#[doc = r" Generated client implementations."]
pub mod client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct DirectoryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DirectoryClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            tonic::transport::Endpoint::new(dst).map(|c| Self::new(c.channel()))
        }
    }
    impl<T> DirectoryClient<T>
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
        #[doc = ""]
        pub async fn lookup_entry(
            &mut self,
            request: tonic::Request<super::LookupEntryRequest>,
        ) -> Result<tonic::Response<super::LookupEntryResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path =
                http::uri::PathAndQuery::from_static("/weaver.directory.Directory/LookupEntry");
            self.inner.unary(request, path, codec).await
        }
        pub async fn list_entries(
            &mut self,
            request: tonic::Request<super::ListEntriesRequest>,
        ) -> Result<tonic::Response<super::ListEntriesResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path =
                http::uri::PathAndQuery::from_static("/weaver.directory.Directory/ListEntries");
            self.inner.unary(request, path, codec).await
        }
        pub async fn create_entry(
            &mut self,
            request: tonic::Request<super::CreateEntryRequest>,
        ) -> Result<tonic::Response<super::CreateEntryResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path =
                http::uri::PathAndQuery::from_static("/weaver.directory.Directory/CreateEntry");
            self.inner.unary(request, path, codec).await
        }
        pub async fn update_entry(
            &mut self,
            request: tonic::Request<super::UpdateEntryRequest>,
        ) -> Result<tonic::Response<super::UpdateEntryResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path =
                http::uri::PathAndQuery::from_static("/weaver.directory.Directory/UpdateEntry");
            self.inner.unary(request, path, codec).await
        }
        pub async fn delete_entry(
            &mut self,
            request: tonic::Request<super::DeleteEntryRequest>,
        ) -> Result<tonic::Response<super::DeleteEntryResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path =
                http::uri::PathAndQuery::from_static("/weaver.directory.Directory/DeleteEntry");
            self.inner.unary(request, path, codec).await
        }
        pub async fn assign_volume(
            &mut self,
            request: tonic::Request<super::AssignVolumeRequest>,
        ) -> Result<tonic::Response<super::AssignVolumeResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path =
                http::uri::PathAndQuery::from_static("/weaver.directory.Directory/AssignVolume");
            self.inner.unary(request, path, codec).await
        }
        pub async fn lookup_volume(
            &mut self,
            request: tonic::Request<super::LookupVolumeRequest>,
        ) -> Result<tonic::Response<super::LookupVolumeResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path =
                http::uri::PathAndQuery::from_static("/weaver.directory.Directory/LookupVolume");
            self.inner.unary(request, path, codec).await
        }
        pub async fn delete_collection(
            &mut self,
            request: tonic::Request<super::DeleteCollectionRequest>,
        ) -> Result<tonic::Response<super::DeleteCollectionResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static(
                "/weaver.directory.Directory/DeleteCollection",
            );
            self.inner.unary(request, path, codec).await
        }
        pub async fn statistics(
            &mut self,
            request: tonic::Request<super::StatisticsRequest>,
        ) -> Result<tonic::Response<super::StatisticsResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path =
                http::uri::PathAndQuery::from_static("/weaver.directory.Directory/Statistics");
            self.inner.unary(request, path, codec).await
        }
    }
    impl<T: Clone> Clone for DirectoryClient<T> {
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
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with DirectoryServer."]
    #[async_trait]
    pub trait Directory: Send + Sync + 'static {
        #[doc = ""]
        async fn lookup_entry(
            &self,
            request: tonic::Request<super::LookupEntryRequest>,
        ) -> Result<tonic::Response<super::LookupEntryResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn list_entries(
            &self,
            request: tonic::Request<super::ListEntriesRequest>,
        ) -> Result<tonic::Response<super::ListEntriesResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn create_entry(
            &self,
            request: tonic::Request<super::CreateEntryRequest>,
        ) -> Result<tonic::Response<super::CreateEntryResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn update_entry(
            &self,
            request: tonic::Request<super::UpdateEntryRequest>,
        ) -> Result<tonic::Response<super::UpdateEntryResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn delete_entry(
            &self,
            request: tonic::Request<super::DeleteEntryRequest>,
        ) -> Result<tonic::Response<super::DeleteEntryResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn assign_volume(
            &self,
            request: tonic::Request<super::AssignVolumeRequest>,
        ) -> Result<tonic::Response<super::AssignVolumeResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn lookup_volume(
            &self,
            request: tonic::Request<super::LookupVolumeRequest>,
        ) -> Result<tonic::Response<super::LookupVolumeResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn delete_collection(
            &self,
            request: tonic::Request<super::DeleteCollectionRequest>,
        ) -> Result<tonic::Response<super::DeleteCollectionResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn statistics(
            &self,
            request: tonic::Request<super::StatisticsRequest>,
        ) -> Result<tonic::Response<super::StatisticsResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
    }
    #[derive(Clone, Debug)]
    pub struct DirectoryServer<T: Directory> {
        inner: Arc<T>,
    }
    #[derive(Clone, Debug)]
    #[doc(hidden)]
    pub struct DirectoryServerSvc<T: Directory> {
        inner: Arc<T>,
    }
    impl<T: Directory> DirectoryServer<T> {
        #[doc = "Create a new DirectoryServer from a type that implements Directory."]
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            Self::from_shared(inner)
        }
        pub fn from_shared(inner: Arc<T>) -> Self {
            Self { inner }
        }
    }
    impl<T: Directory> DirectoryServerSvc<T> {
        pub fn new(inner: Arc<T>) -> Self {
            Self { inner }
        }
    }
    impl<T: Directory, R> Service<R> for DirectoryServer<T> {
        type Response = DirectoryServerSvc<T>;
        type Error = Never;
        type Future = Ready<Result<Self::Response, Self::Error>>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, _: R) -> Self::Future {
            ok(DirectoryServerSvc::new(self.inner.clone()))
        }
    }
    impl<T: Directory> Service<http::Request<HyperBody>> for DirectoryServerSvc<T> {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<HyperBody>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/weaver.directory.Directory/LookupEntry" => {
                    struct LookupEntrySvc<T: Directory>(pub Arc<T>);
                    impl<T: Directory> tonic::server::UnaryService<super::LookupEntryRequest> for LookupEntrySvc<T> {
                        type Response = super::LookupEntryResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LookupEntryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.lookup_entry(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = LookupEntrySvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.directory.Directory/ListEntries" => {
                    struct ListEntriesSvc<T: Directory>(pub Arc<T>);
                    impl<T: Directory> tonic::server::UnaryService<super::ListEntriesRequest> for ListEntriesSvc<T> {
                        type Response = super::ListEntriesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListEntriesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_entries(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListEntriesSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.directory.Directory/CreateEntry" => {
                    struct CreateEntrySvc<T: Directory>(pub Arc<T>);
                    impl<T: Directory> tonic::server::UnaryService<super::CreateEntryRequest> for CreateEntrySvc<T> {
                        type Response = super::CreateEntryResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateEntryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_entry(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateEntrySvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.directory.Directory/UpdateEntry" => {
                    struct UpdateEntrySvc<T: Directory>(pub Arc<T>);
                    impl<T: Directory> tonic::server::UnaryService<super::UpdateEntryRequest> for UpdateEntrySvc<T> {
                        type Response = super::UpdateEntryResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateEntryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_entry(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateEntrySvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.directory.Directory/DeleteEntry" => {
                    struct DeleteEntrySvc<T: Directory>(pub Arc<T>);
                    impl<T: Directory> tonic::server::UnaryService<super::DeleteEntryRequest> for DeleteEntrySvc<T> {
                        type Response = super::DeleteEntryResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteEntryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_entry(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteEntrySvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.directory.Directory/AssignVolume" => {
                    struct AssignVolumeSvc<T: Directory>(pub Arc<T>);
                    impl<T: Directory> tonic::server::UnaryService<super::AssignVolumeRequest> for AssignVolumeSvc<T> {
                        type Response = super::AssignVolumeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AssignVolumeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.assign_volume(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = AssignVolumeSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.directory.Directory/LookupVolume" => {
                    struct LookupVolumeSvc<T: Directory>(pub Arc<T>);
                    impl<T: Directory> tonic::server::UnaryService<super::LookupVolumeRequest> for LookupVolumeSvc<T> {
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
                "/weaver.directory.Directory/DeleteCollection" => {
                    struct DeleteCollectionSvc<T: Directory>(pub Arc<T>);
                    impl<T: Directory> tonic::server::UnaryService<super::DeleteCollectionRequest>
                        for DeleteCollectionSvc<T>
                    {
                        type Response = super::DeleteCollectionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteCollectionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_collection(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteCollectionSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.directory.Directory/Statistics" => {
                    struct StatisticsSvc<T: Directory>(pub Arc<T>);
                    impl<T: Directory> tonic::server::UnaryService<super::StatisticsRequest> for StatisticsSvc<T> {
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
