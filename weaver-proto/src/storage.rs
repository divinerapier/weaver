#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocateVolumeRequest {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
    #[prost(string, tag = "2")]
    pub bucket: std::string::String,
    #[prost(int64, tag = "3")]
    pub preallocate: i64,
    #[prost(message, optional, tag = "4")]
    pub replica_replacement: ::std::option::Option<super::ReplicaReplacement>,
    #[prost(string, tag = "5")]
    pub ttl: std::string::String,
    #[prost(uint32, tag = "6")]
    pub memory_map_max_size_mb: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocateVolumeResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::std::option::Option<super::Status>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteRequest {
    #[prost(string, repeated, tag = "1")]
    pub file_ids: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteResponse {
    #[prost(message, repeated, tag = "1")]
    pub results: ::std::vec::Vec<DeleteResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteResult {
    #[prost(string, tag = "1")]
    pub file_id: std::string::String,
    #[prost(int32, tag = "2")]
    pub status: i32,
    #[prost(string, tag = "3")]
    pub error: std::string::String,
    #[prost(uint32, tag = "4")]
    pub size: u32,
    #[prost(uint32, tag = "5")]
    pub version: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VacuumVolumeCheckRequest {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VacuumVolumeCheckResponse {
    #[prost(double, tag = "1")]
    pub garbage_ratio: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VacuumVolumeCompactRequest {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
    #[prost(int64, tag = "2")]
    pub preallocate: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VacuumVolumeCompactResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VacuumVolumeCommitRequest {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VacuumVolumeCommitResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VacuumVolumeCleanupRequest {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VacuumVolumeCleanupResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCollectionRequest {
    #[prost(string, tag = "1")]
    pub collection: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCollectionResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeSyncStatusRequest {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeSyncStatusResponse {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
    #[prost(string, tag = "2")]
    pub bucket: std::string::String,
    #[prost(message, optional, tag = "3")]
    pub replica_replacement: ::std::option::Option<super::ReplicaReplacement>,
    #[prost(string, tag = "5")]
    pub ttl: std::string::String,
    #[prost(uint64, tag = "6")]
    pub tail_offset: u64,
    #[prost(uint32, tag = "7")]
    pub compact_revision: u32,
    #[prost(uint64, tag = "8")]
    pub idx_file_size: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeIncrementalCopyRequest {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
    #[prost(uint64, tag = "2")]
    pub since_ns: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeIncrementalCopyResponse {
    #[prost(bytes, tag = "1")]
    pub file_content: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeMountRequest {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeMountResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeUnmountRequest {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeUnmountResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeDeleteRequest {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeDeleteResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeMarkReadonlyRequest {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeMarkReadonlyResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeCopyRequest {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
    #[prost(string, tag = "2")]
    pub bucket: std::string::String,
    #[prost(message, optional, tag = "3")]
    pub replica_replacement: ::std::option::Option<super::ReplicaReplacement>,
    #[prost(string, tag = "4")]
    pub ttl: std::string::String,
    #[prost(string, tag = "5")]
    pub source_data_node: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeCopyResponse {
    #[prost(uint64, tag = "1")]
    pub last_append_at_ns: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CopyFileRequest {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
    #[prost(string, tag = "2")]
    pub ext: std::string::String,
    #[prost(uint32, tag = "3")]
    pub compaction_revision: u32,
    #[prost(uint64, tag = "4")]
    pub stop_offset: u64,
    #[prost(string, tag = "5")]
    pub bucket: std::string::String,
    #[prost(bool, tag = "6")]
    pub is_ec_volume: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CopyFileResponse {
    #[prost(bytes, tag = "1")]
    pub file_content: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeTailSenderRequest {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
    #[prost(uint64, tag = "2")]
    pub since_ns: u64,
    #[prost(uint32, tag = "3")]
    pub idle_timeout_seconds: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeTailSenderResponse {
    #[prost(bytes, tag = "1")]
    pub needle_header: std::vec::Vec<u8>,
    #[prost(bytes, tag = "2")]
    pub needle_body: std::vec::Vec<u8>,
    #[prost(bool, tag = "3")]
    pub is_last_chunk: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeTailReceiverRequest {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
    #[prost(uint64, tag = "2")]
    pub since_ns: u64,
    #[prost(uint32, tag = "3")]
    pub idle_timeout_seconds: u32,
    #[prost(string, tag = "4")]
    pub source_volume_server: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeTailReceiverResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeEcShardsGenerateRequest {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
    #[prost(string, tag = "2")]
    pub bucket: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeEcShardsGenerateResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeEcShardsRebuildRequest {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
    #[prost(string, tag = "2")]
    pub bucket: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeEcShardsRebuildResponse {
    #[prost(uint32, repeated, tag = "1")]
    pub rebuilt_shard_ids: ::std::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeEcShardsCopyRequest {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
    #[prost(string, tag = "2")]
    pub bucket: std::string::String,
    #[prost(uint32, repeated, tag = "3")]
    pub shard_ids: ::std::vec::Vec<u32>,
    #[prost(bool, tag = "4")]
    pub copy_ecx_file: bool,
    #[prost(string, tag = "5")]
    pub source_data_node: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeEcShardsCopyResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeEcShardsDeleteRequest {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
    #[prost(string, tag = "2")]
    pub bucket: std::string::String,
    #[prost(uint32, repeated, tag = "3")]
    pub shard_ids: ::std::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeEcShardsDeleteResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeEcShardsMountRequest {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
    #[prost(string, tag = "2")]
    pub bucket: std::string::String,
    #[prost(uint32, repeated, tag = "3")]
    pub shard_ids: ::std::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeEcShardsMountResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeEcShardsUnmountRequest {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
    #[prost(uint32, repeated, tag = "3")]
    pub shard_ids: ::std::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeEcShardsUnmountResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeEcShardReadRequest {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
    #[prost(uint32, tag = "2")]
    pub shard_id: u32,
    #[prost(int64, tag = "3")]
    pub offset: i64,
    #[prost(int64, tag = "4")]
    pub size: i64,
    #[prost(uint64, tag = "5")]
    pub file_key: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeEcShardReadResponse {
    #[prost(bytes, tag = "1")]
    pub data: std::vec::Vec<u8>,
    #[prost(bool, tag = "2")]
    pub is_deleted: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeEcBlobDeleteRequest {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
    #[prost(string, tag = "2")]
    pub bucket: std::string::String,
    #[prost(uint64, tag = "3")]
    pub file_key: u64,
    #[prost(uint32, tag = "4")]
    pub version: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeEcBlobDeleteResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadVolumeFileStatusRequest {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadVolumeFileStatusResponse {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
    #[prost(uint64, tag = "2")]
    pub idx_file_timestamp_seconds: u64,
    #[prost(uint64, tag = "3")]
    pub idx_file_size: u64,
    #[prost(uint64, tag = "4")]
    pub dat_file_timestamp_seconds: u64,
    #[prost(uint64, tag = "5")]
    pub dat_file_size: u64,
    #[prost(uint64, tag = "6")]
    pub file_count: u64,
    #[prost(uint32, tag = "7")]
    pub compaction_revision: u32,
    #[prost(string, tag = "8")]
    pub bucket: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRequest {
    #[prost(string, repeated, tag = "1")]
    pub selections: ::std::vec::Vec<std::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub from_file_ids: ::std::vec::Vec<std::string::String>,
    #[prost(message, optional, tag = "3")]
    pub filter: ::std::option::Option<query_request::Filter>,
    #[prost(message, optional, tag = "4")]
    pub input_serialization: ::std::option::Option<query_request::InputSerialization>,
    #[prost(message, optional, tag = "5")]
    pub output_serialization: ::std::option::Option<query_request::OutputSerialization>,
}
pub mod query_request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Filter {
        #[prost(string, tag = "1")]
        pub field: std::string::String,
        #[prost(string, tag = "2")]
        pub operand: std::string::String,
        #[prost(string, tag = "3")]
        pub value: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InputSerialization {
        /// NONE | GZIP | BZIP2
        #[prost(string, tag = "1")]
        pub compression_type: std::string::String,
        #[prost(message, optional, tag = "2")]
        pub csv_input: ::std::option::Option<input_serialization::CsvInput>,
        #[prost(message, optional, tag = "3")]
        pub json_input: ::std::option::Option<input_serialization::JsonInput>,
        #[prost(message, optional, tag = "4")]
        pub parquet_input: ::std::option::Option<input_serialization::ParquetInput>,
    }
    pub mod input_serialization {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CsvInput {
            /// Valid values: NONE | USE | IGNORE
            #[prost(string, tag = "1")]
            pub file_header_info: std::string::String,
            /// Default: \n
            #[prost(string, tag = "2")]
            pub record_delimiter: std::string::String,
            /// Default: ,
            #[prost(string, tag = "3")]
            pub field_delimiter: std::string::String,
            /// Default: "
            #[prost(string, tag = "4")]
            pub quote_charactoer: std::string::String,
            /// Default: "
            #[prost(string, tag = "5")]
            pub quote_escape_character: std::string::String,
            /// Default: #
            #[prost(string, tag = "6")]
            pub comments: std::string::String,
            /// If true, records might contain record delimiters within quote
            /// characters
            ///
            /// default False.
            #[prost(bool, tag = "7")]
            pub allow_quoted_record_delimiter: bool,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct JsonInput {
            /// Valid values: DOCUMENT | LINES
            #[prost(string, tag = "1")]
            pub r#type: std::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ParquetInput {}
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OutputSerialization {
        #[prost(message, optional, tag = "2")]
        pub csv_output: ::std::option::Option<output_serialization::CsvOutput>,
        #[prost(message, optional, tag = "3")]
        pub json_output: ::std::option::Option<output_serialization::JsonOutput>,
    }
    pub mod output_serialization {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CsvOutput {
            /// Valid values: ALWAYS | ASNEEDED
            #[prost(string, tag = "1")]
            pub quote_fields: std::string::String,
            /// Default: \n
            #[prost(string, tag = "2")]
            pub record_delimiter: std::string::String,
            /// Default: ,
            #[prost(string, tag = "3")]
            pub field_delimiter: std::string::String,
            /// Default: "
            #[prost(string, tag = "4")]
            pub quote_charactoer: std::string::String,
            /// Default: "
            #[prost(string, tag = "5")]
            pub quote_escape_character: std::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct JsonOutput {
            #[prost(string, tag = "1")]
            pub record_delimiter: std::string::String,
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueriedStripe {
    #[prost(bytes, tag = "1")]
    pub records: std::vec::Vec<u8>,
}
#[doc = r" Generated client implementations."]
pub mod client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct StorageClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StorageClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            tonic::transport::Endpoint::new(dst).map(|c| Self::new(c.channel()))
        }
    }
    impl<T> StorageClient<T>
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
        pub async fn allocate_volume(
            &mut self,
            request: tonic::Request<super::AllocateVolumeRequest>,
        ) -> Result<tonic::Response<super::AllocateVolumeResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path =
                http::uri::PathAndQuery::from_static("/weaver.storage.Storage/AllocateVolume");
            self.inner.unary(request, path, codec).await
        }
        #[doc = " Experts only: takes multiple fid parameters. This function does not"]
        #[doc = " propagate deletes to replicas."]
        pub async fn batch_delete(
            &mut self,
            request: tonic::Request<super::BatchDeleteRequest>,
        ) -> Result<tonic::Response<super::BatchDeleteResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static("/weaver.storage.Storage/BatchDelete");
            self.inner.unary(request, path, codec).await
        }
        pub async fn vacuum_volume_check(
            &mut self,
            request: tonic::Request<super::VacuumVolumeCheckRequest>,
        ) -> Result<tonic::Response<super::VacuumVolumeCheckResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path =
                http::uri::PathAndQuery::from_static("/weaver.storage.Storage/VacuumVolumeCheck");
            self.inner.unary(request, path, codec).await
        }
        pub async fn vacuum_volume_compact(
            &mut self,
            request: tonic::Request<super::VacuumVolumeCompactRequest>,
        ) -> Result<tonic::Response<super::VacuumVolumeCompactResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path =
                http::uri::PathAndQuery::from_static("/weaver.storage.Storage/VacuumVolumeCompact");
            self.inner.unary(request, path, codec).await
        }
        pub async fn vacuum_volume_commit(
            &mut self,
            request: tonic::Request<super::VacuumVolumeCommitRequest>,
        ) -> Result<tonic::Response<super::VacuumVolumeCommitResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path =
                http::uri::PathAndQuery::from_static("/weaver.storage.Storage/VacuumVolumeCommit");
            self.inner.unary(request, path, codec).await
        }
        pub async fn vacuum_volume_cleanup(
            &mut self,
            request: tonic::Request<super::VacuumVolumeCleanupRequest>,
        ) -> Result<tonic::Response<super::VacuumVolumeCleanupResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path =
                http::uri::PathAndQuery::from_static("/weaver.storage.Storage/VacuumVolumeCleanup");
            self.inner.unary(request, path, codec).await
        }
        pub async fn delete_collection(
            &mut self,
            request: tonic::Request<super::DeleteCollectionRequest>,
        ) -> Result<tonic::Response<super::DeleteCollectionResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path =
                http::uri::PathAndQuery::from_static("/weaver.storage.Storage/DeleteCollection");
            self.inner.unary(request, path, codec).await
        }
        pub async fn volume_sync_status(
            &mut self,
            request: tonic::Request<super::VolumeSyncStatusRequest>,
        ) -> Result<tonic::Response<super::VolumeSyncStatusResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path =
                http::uri::PathAndQuery::from_static("/weaver.storage.Storage/VolumeSyncStatus");
            self.inner.unary(request, path, codec).await
        }
        pub async fn volume_incremental_copy(
            &mut self,
            request: tonic::Request<super::VolumeIncrementalCopyRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::VolumeIncrementalCopyResponse>>,
            tonic::Status,
        > {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static(
                "/weaver.storage.Storage/VolumeIncrementalCopy",
            );
            self.inner.server_streaming(request, path, codec).await
        }
        pub async fn volume_mount(
            &mut self,
            request: tonic::Request<super::VolumeMountRequest>,
        ) -> Result<tonic::Response<super::VolumeMountResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static("/weaver.storage.Storage/VolumeMount");
            self.inner.unary(request, path, codec).await
        }
        pub async fn volume_unmount(
            &mut self,
            request: tonic::Request<super::VolumeUnmountRequest>,
        ) -> Result<tonic::Response<super::VolumeUnmountResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path =
                http::uri::PathAndQuery::from_static("/weaver.storage.Storage/VolumeUnmount");
            self.inner.unary(request, path, codec).await
        }
        pub async fn volume_delete(
            &mut self,
            request: tonic::Request<super::VolumeDeleteRequest>,
        ) -> Result<tonic::Response<super::VolumeDeleteResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static("/weaver.storage.Storage/VolumeDelete");
            self.inner.unary(request, path, codec).await
        }
        pub async fn volume_mark_readonly(
            &mut self,
            request: tonic::Request<super::VolumeMarkReadonlyRequest>,
        ) -> Result<tonic::Response<super::VolumeMarkReadonlyResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path =
                http::uri::PathAndQuery::from_static("/weaver.storage.Storage/VolumeMarkReadonly");
            self.inner.unary(request, path, codec).await
        }
        #[doc = " copy the .idx .dat files, and mount this volume"]
        pub async fn volume_copy(
            &mut self,
            request: tonic::Request<super::VolumeCopyRequest>,
        ) -> Result<tonic::Response<super::VolumeCopyResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static("/weaver.storage.Storage/VolumeCopy");
            self.inner.unary(request, path, codec).await
        }
        pub async fn read_volume_file_status(
            &mut self,
            request: tonic::Request<super::ReadVolumeFileStatusRequest>,
        ) -> Result<tonic::Response<super::ReadVolumeFileStatusResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static(
                "/weaver.storage.Storage/ReadVolumeFileStatus",
            );
            self.inner.unary(request, path, codec).await
        }
        pub async fn copy_file(
            &mut self,
            request: tonic::Request<super::CopyFileRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::CopyFileResponse>>, tonic::Status>
        {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static("/weaver.storage.Storage/CopyFile");
            self.inner.server_streaming(request, path, codec).await
        }
        pub async fn volume_tail_sender(
            &mut self,
            request: tonic::Request<super::VolumeTailSenderRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::VolumeTailSenderResponse>>,
            tonic::Status,
        > {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path =
                http::uri::PathAndQuery::from_static("/weaver.storage.Storage/VolumeTailSender");
            self.inner.server_streaming(request, path, codec).await
        }
        pub async fn volume_tail_receiver(
            &mut self,
            request: tonic::Request<super::VolumeTailReceiverRequest>,
        ) -> Result<tonic::Response<super::VolumeTailReceiverResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path =
                http::uri::PathAndQuery::from_static("/weaver.storage.Storage/VolumeTailReceiver");
            self.inner.unary(request, path, codec).await
        }
        #[doc = " erasure coding"]
        pub async fn volume_ec_shards_generate(
            &mut self,
            request: tonic::Request<super::VolumeEcShardsGenerateRequest>,
        ) -> Result<tonic::Response<super::VolumeEcShardsGenerateResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static(
                "/weaver.storage.Storage/VolumeEcShardsGenerate",
            );
            self.inner.unary(request, path, codec).await
        }
        pub async fn volume_ec_shards_rebuild(
            &mut self,
            request: tonic::Request<super::VolumeEcShardsRebuildRequest>,
        ) -> Result<tonic::Response<super::VolumeEcShardsRebuildResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static(
                "/weaver.storage.Storage/VolumeEcShardsRebuild",
            );
            self.inner.unary(request, path, codec).await
        }
        pub async fn volume_ec_shards_copy(
            &mut self,
            request: tonic::Request<super::VolumeEcShardsCopyRequest>,
        ) -> Result<tonic::Response<super::VolumeEcShardsCopyResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path =
                http::uri::PathAndQuery::from_static("/weaver.storage.Storage/VolumeEcShardsCopy");
            self.inner.unary(request, path, codec).await
        }
        pub async fn volume_ec_shards_delete(
            &mut self,
            request: tonic::Request<super::VolumeEcShardsDeleteRequest>,
        ) -> Result<tonic::Response<super::VolumeEcShardsDeleteResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static(
                "/weaver.storage.Storage/VolumeEcShardsDelete",
            );
            self.inner.unary(request, path, codec).await
        }
        pub async fn volume_ec_shards_mount(
            &mut self,
            request: tonic::Request<super::VolumeEcShardsMountRequest>,
        ) -> Result<tonic::Response<super::VolumeEcShardsMountResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path =
                http::uri::PathAndQuery::from_static("/weaver.storage.Storage/VolumeEcShardsMount");
            self.inner.unary(request, path, codec).await
        }
        pub async fn volume_ec_shards_unmount(
            &mut self,
            request: tonic::Request<super::VolumeEcShardsUnmountRequest>,
        ) -> Result<tonic::Response<super::VolumeEcShardsUnmountResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static(
                "/weaver.storage.Storage/VolumeEcShardsUnmount",
            );
            self.inner.unary(request, path, codec).await
        }
        pub async fn volume_ec_shard_read(
            &mut self,
            request: tonic::Request<super::VolumeEcShardReadRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::VolumeEcShardReadResponse>>,
            tonic::Status,
        > {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path =
                http::uri::PathAndQuery::from_static("/weaver.storage.Storage/VolumeEcShardRead");
            self.inner.server_streaming(request, path, codec).await
        }
        pub async fn volume_ec_blob_delete(
            &mut self,
            request: tonic::Request<super::VolumeEcBlobDeleteRequest>,
        ) -> Result<tonic::Response<super::VolumeEcBlobDeleteResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path =
                http::uri::PathAndQuery::from_static("/weaver.storage.Storage/VolumeEcBlobDelete");
            self.inner.unary(request, path, codec).await
        }
        #[doc = " query"]
        pub async fn query(
            &mut self,
            request: tonic::Request<super::QueryRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::QueriedStripe>>, tonic::Status>
        {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static("/weaver.storage.Storage/Query");
            self.inner.server_streaming(request, path, codec).await
        }
    }
    impl<T: Clone> Clone for StorageClient<T> {
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
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with StorageServer."]
    #[async_trait]
    pub trait Storage: Send + Sync + 'static {
        async fn allocate_volume(
            &self,
            request: tonic::Request<super::AllocateVolumeRequest>,
        ) -> Result<tonic::Response<super::AllocateVolumeResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        #[doc = " Experts only: takes multiple fid parameters. This function does not"]
        #[doc = " propagate deletes to replicas."]
        async fn batch_delete(
            &self,
            request: tonic::Request<super::BatchDeleteRequest>,
        ) -> Result<tonic::Response<super::BatchDeleteResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn vacuum_volume_check(
            &self,
            request: tonic::Request<super::VacuumVolumeCheckRequest>,
        ) -> Result<tonic::Response<super::VacuumVolumeCheckResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn vacuum_volume_compact(
            &self,
            request: tonic::Request<super::VacuumVolumeCompactRequest>,
        ) -> Result<tonic::Response<super::VacuumVolumeCompactResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn vacuum_volume_commit(
            &self,
            request: tonic::Request<super::VacuumVolumeCommitRequest>,
        ) -> Result<tonic::Response<super::VacuumVolumeCommitResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn vacuum_volume_cleanup(
            &self,
            request: tonic::Request<super::VacuumVolumeCleanupRequest>,
        ) -> Result<tonic::Response<super::VacuumVolumeCleanupResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn delete_collection(
            &self,
            request: tonic::Request<super::DeleteCollectionRequest>,
        ) -> Result<tonic::Response<super::DeleteCollectionResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn volume_sync_status(
            &self,
            request: tonic::Request<super::VolumeSyncStatusRequest>,
        ) -> Result<tonic::Response<super::VolumeSyncStatusResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        #[doc = "Server streaming response type for the VolumeIncrementalCopy method."]
        type VolumeIncrementalCopyStream: Stream<Item = Result<super::VolumeIncrementalCopyResponse, tonic::Status>>
            + Send
            + 'static;
        async fn volume_incremental_copy(
            &self,
            request: tonic::Request<super::VolumeIncrementalCopyRequest>,
        ) -> Result<tonic::Response<Self::VolumeIncrementalCopyStream>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn volume_mount(
            &self,
            request: tonic::Request<super::VolumeMountRequest>,
        ) -> Result<tonic::Response<super::VolumeMountResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn volume_unmount(
            &self,
            request: tonic::Request<super::VolumeUnmountRequest>,
        ) -> Result<tonic::Response<super::VolumeUnmountResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn volume_delete(
            &self,
            request: tonic::Request<super::VolumeDeleteRequest>,
        ) -> Result<tonic::Response<super::VolumeDeleteResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn volume_mark_readonly(
            &self,
            request: tonic::Request<super::VolumeMarkReadonlyRequest>,
        ) -> Result<tonic::Response<super::VolumeMarkReadonlyResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        #[doc = " copy the .idx .dat files, and mount this volume"]
        async fn volume_copy(
            &self,
            request: tonic::Request<super::VolumeCopyRequest>,
        ) -> Result<tonic::Response<super::VolumeCopyResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn read_volume_file_status(
            &self,
            request: tonic::Request<super::ReadVolumeFileStatusRequest>,
        ) -> Result<tonic::Response<super::ReadVolumeFileStatusResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        #[doc = "Server streaming response type for the CopyFile method."]
        type CopyFileStream: Stream<Item = Result<super::CopyFileResponse, tonic::Status>>
            + Send
            + 'static;
        async fn copy_file(
            &self,
            request: tonic::Request<super::CopyFileRequest>,
        ) -> Result<tonic::Response<Self::CopyFileStream>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        #[doc = "Server streaming response type for the VolumeTailSender method."]
        type VolumeTailSenderStream: Stream<Item = Result<super::VolumeTailSenderResponse, tonic::Status>>
            + Send
            + 'static;
        async fn volume_tail_sender(
            &self,
            request: tonic::Request<super::VolumeTailSenderRequest>,
        ) -> Result<tonic::Response<Self::VolumeTailSenderStream>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn volume_tail_receiver(
            &self,
            request: tonic::Request<super::VolumeTailReceiverRequest>,
        ) -> Result<tonic::Response<super::VolumeTailReceiverResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        #[doc = " erasure coding"]
        async fn volume_ec_shards_generate(
            &self,
            request: tonic::Request<super::VolumeEcShardsGenerateRequest>,
        ) -> Result<tonic::Response<super::VolumeEcShardsGenerateResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn volume_ec_shards_rebuild(
            &self,
            request: tonic::Request<super::VolumeEcShardsRebuildRequest>,
        ) -> Result<tonic::Response<super::VolumeEcShardsRebuildResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn volume_ec_shards_copy(
            &self,
            request: tonic::Request<super::VolumeEcShardsCopyRequest>,
        ) -> Result<tonic::Response<super::VolumeEcShardsCopyResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn volume_ec_shards_delete(
            &self,
            request: tonic::Request<super::VolumeEcShardsDeleteRequest>,
        ) -> Result<tonic::Response<super::VolumeEcShardsDeleteResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn volume_ec_shards_mount(
            &self,
            request: tonic::Request<super::VolumeEcShardsMountRequest>,
        ) -> Result<tonic::Response<super::VolumeEcShardsMountResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn volume_ec_shards_unmount(
            &self,
            request: tonic::Request<super::VolumeEcShardsUnmountRequest>,
        ) -> Result<tonic::Response<super::VolumeEcShardsUnmountResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        #[doc = "Server streaming response type for the VolumeEcShardRead method."]
        type VolumeEcShardReadStream: Stream<Item = Result<super::VolumeEcShardReadResponse, tonic::Status>>
            + Send
            + 'static;
        async fn volume_ec_shard_read(
            &self,
            request: tonic::Request<super::VolumeEcShardReadRequest>,
        ) -> Result<tonic::Response<Self::VolumeEcShardReadStream>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        async fn volume_ec_blob_delete(
            &self,
            request: tonic::Request<super::VolumeEcBlobDeleteRequest>,
        ) -> Result<tonic::Response<super::VolumeEcBlobDeleteResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        #[doc = "Server streaming response type for the Query method."]
        type QueryStream: Stream<Item = Result<super::QueriedStripe, tonic::Status>>
            + Send
            + 'static;
        #[doc = " query"]
        async fn query(
            &self,
            request: tonic::Request<super::QueryRequest>,
        ) -> Result<tonic::Response<Self::QueryStream>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
    }
    #[derive(Clone, Debug)]
    pub struct StorageServer<T: Storage> {
        inner: Arc<T>,
    }
    #[derive(Clone, Debug)]
    #[doc(hidden)]
    pub struct StorageServerSvc<T: Storage> {
        inner: Arc<T>,
    }
    impl<T: Storage> StorageServer<T> {
        #[doc = "Create a new StorageServer from a type that implements Storage."]
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            Self::from_shared(inner)
        }
        pub fn from_shared(inner: Arc<T>) -> Self {
            Self { inner }
        }
    }
    impl<T: Storage> StorageServerSvc<T> {
        pub fn new(inner: Arc<T>) -> Self {
            Self { inner }
        }
    }
    impl<T: Storage, R> Service<R> for StorageServer<T> {
        type Response = StorageServerSvc<T>;
        type Error = Never;
        type Future = Ready<Result<Self::Response, Self::Error>>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, _: R) -> Self::Future {
            ok(StorageServerSvc::new(self.inner.clone()))
        }
    }
    impl<T: Storage> Service<http::Request<HyperBody>> for StorageServerSvc<T> {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<HyperBody>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/weaver.storage.Storage/AllocateVolume" => {
                    struct AllocateVolumeSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage> tonic::server::UnaryService<super::AllocateVolumeRequest>
                        for AllocateVolumeSvc<T>
                    {
                        type Response = super::AllocateVolumeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AllocateVolumeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.allocate_volume(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = AllocateVolumeSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.storage.Storage/BatchDelete" => {
                    struct BatchDeleteSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage> tonic::server::UnaryService<super::BatchDeleteRequest> for BatchDeleteSvc<T> {
                        type Response = super::BatchDeleteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchDeleteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.batch_delete(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = BatchDeleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.storage.Storage/VacuumVolumeCheck" => {
                    struct VacuumVolumeCheckSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage> tonic::server::UnaryService<super::VacuumVolumeCheckRequest>
                        for VacuumVolumeCheckSvc<T>
                    {
                        type Response = super::VacuumVolumeCheckResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VacuumVolumeCheckRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.vacuum_volume_check(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = VacuumVolumeCheckSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.storage.Storage/VacuumVolumeCompact" => {
                    struct VacuumVolumeCompactSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage> tonic::server::UnaryService<super::VacuumVolumeCompactRequest>
                        for VacuumVolumeCompactSvc<T>
                    {
                        type Response = super::VacuumVolumeCompactResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VacuumVolumeCompactRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.vacuum_volume_compact(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = VacuumVolumeCompactSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.storage.Storage/VacuumVolumeCommit" => {
                    struct VacuumVolumeCommitSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage> tonic::server::UnaryService<super::VacuumVolumeCommitRequest>
                        for VacuumVolumeCommitSvc<T>
                    {
                        type Response = super::VacuumVolumeCommitResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VacuumVolumeCommitRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.vacuum_volume_commit(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = VacuumVolumeCommitSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.storage.Storage/VacuumVolumeCleanup" => {
                    struct VacuumVolumeCleanupSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage> tonic::server::UnaryService<super::VacuumVolumeCleanupRequest>
                        for VacuumVolumeCleanupSvc<T>
                    {
                        type Response = super::VacuumVolumeCleanupResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VacuumVolumeCleanupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.vacuum_volume_cleanup(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = VacuumVolumeCleanupSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.storage.Storage/DeleteCollection" => {
                    struct DeleteCollectionSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage> tonic::server::UnaryService<super::DeleteCollectionRequest>
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
                "/weaver.storage.Storage/VolumeSyncStatus" => {
                    struct VolumeSyncStatusSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage> tonic::server::UnaryService<super::VolumeSyncStatusRequest>
                        for VolumeSyncStatusSvc<T>
                    {
                        type Response = super::VolumeSyncStatusResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VolumeSyncStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.volume_sync_status(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = VolumeSyncStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.storage.Storage/VolumeIncrementalCopy" => {
                    struct VolumeIncrementalCopySvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage>
                        tonic::server::ServerStreamingService<super::VolumeIncrementalCopyRequest>
                        for VolumeIncrementalCopySvc<T>
                    {
                        type Response = super::VolumeIncrementalCopyResponse;
                        type ResponseStream = T::VolumeIncrementalCopyStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VolumeIncrementalCopyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.volume_incremental_copy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = VolumeIncrementalCopySvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.storage.Storage/VolumeMount" => {
                    struct VolumeMountSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage> tonic::server::UnaryService<super::VolumeMountRequest> for VolumeMountSvc<T> {
                        type Response = super::VolumeMountResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VolumeMountRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.volume_mount(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = VolumeMountSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.storage.Storage/VolumeUnmount" => {
                    struct VolumeUnmountSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage> tonic::server::UnaryService<super::VolumeUnmountRequest> for VolumeUnmountSvc<T> {
                        type Response = super::VolumeUnmountResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VolumeUnmountRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.volume_unmount(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = VolumeUnmountSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.storage.Storage/VolumeDelete" => {
                    struct VolumeDeleteSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage> tonic::server::UnaryService<super::VolumeDeleteRequest> for VolumeDeleteSvc<T> {
                        type Response = super::VolumeDeleteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VolumeDeleteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.volume_delete(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = VolumeDeleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.storage.Storage/VolumeMarkReadonly" => {
                    struct VolumeMarkReadonlySvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage> tonic::server::UnaryService<super::VolumeMarkReadonlyRequest>
                        for VolumeMarkReadonlySvc<T>
                    {
                        type Response = super::VolumeMarkReadonlyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VolumeMarkReadonlyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.volume_mark_readonly(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = VolumeMarkReadonlySvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.storage.Storage/VolumeCopy" => {
                    struct VolumeCopySvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage> tonic::server::UnaryService<super::VolumeCopyRequest> for VolumeCopySvc<T> {
                        type Response = super::VolumeCopyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VolumeCopyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.volume_copy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = VolumeCopySvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.storage.Storage/ReadVolumeFileStatus" => {
                    struct ReadVolumeFileStatusSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage> tonic::server::UnaryService<super::ReadVolumeFileStatusRequest>
                        for ReadVolumeFileStatusSvc<T>
                    {
                        type Response = super::ReadVolumeFileStatusResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadVolumeFileStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.read_volume_file_status(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ReadVolumeFileStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.storage.Storage/CopyFile" => {
                    struct CopyFileSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage> tonic::server::ServerStreamingService<super::CopyFileRequest> for CopyFileSvc<T> {
                        type Response = super::CopyFileResponse;
                        type ResponseStream = T::CopyFileStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CopyFileRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.copy_file(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CopyFileSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.storage.Storage/VolumeTailSender" => {
                    struct VolumeTailSenderSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage>
                        tonic::server::ServerStreamingService<super::VolumeTailSenderRequest>
                        for VolumeTailSenderSvc<T>
                    {
                        type Response = super::VolumeTailSenderResponse;
                        type ResponseStream = T::VolumeTailSenderStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VolumeTailSenderRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.volume_tail_sender(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = VolumeTailSenderSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.storage.Storage/VolumeTailReceiver" => {
                    struct VolumeTailReceiverSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage> tonic::server::UnaryService<super::VolumeTailReceiverRequest>
                        for VolumeTailReceiverSvc<T>
                    {
                        type Response = super::VolumeTailReceiverResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VolumeTailReceiverRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.volume_tail_receiver(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = VolumeTailReceiverSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.storage.Storage/VolumeEcShardsGenerate" => {
                    struct VolumeEcShardsGenerateSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage>
                        tonic::server::UnaryService<super::VolumeEcShardsGenerateRequest>
                        for VolumeEcShardsGenerateSvc<T>
                    {
                        type Response = super::VolumeEcShardsGenerateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VolumeEcShardsGenerateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.volume_ec_shards_generate(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = VolumeEcShardsGenerateSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.storage.Storage/VolumeEcShardsRebuild" => {
                    struct VolumeEcShardsRebuildSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage>
                        tonic::server::UnaryService<super::VolumeEcShardsRebuildRequest>
                        for VolumeEcShardsRebuildSvc<T>
                    {
                        type Response = super::VolumeEcShardsRebuildResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VolumeEcShardsRebuildRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.volume_ec_shards_rebuild(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = VolumeEcShardsRebuildSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.storage.Storage/VolumeEcShardsCopy" => {
                    struct VolumeEcShardsCopySvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage> tonic::server::UnaryService<super::VolumeEcShardsCopyRequest>
                        for VolumeEcShardsCopySvc<T>
                    {
                        type Response = super::VolumeEcShardsCopyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VolumeEcShardsCopyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.volume_ec_shards_copy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = VolumeEcShardsCopySvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.storage.Storage/VolumeEcShardsDelete" => {
                    struct VolumeEcShardsDeleteSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage> tonic::server::UnaryService<super::VolumeEcShardsDeleteRequest>
                        for VolumeEcShardsDeleteSvc<T>
                    {
                        type Response = super::VolumeEcShardsDeleteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VolumeEcShardsDeleteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.volume_ec_shards_delete(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = VolumeEcShardsDeleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.storage.Storage/VolumeEcShardsMount" => {
                    struct VolumeEcShardsMountSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage> tonic::server::UnaryService<super::VolumeEcShardsMountRequest>
                        for VolumeEcShardsMountSvc<T>
                    {
                        type Response = super::VolumeEcShardsMountResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VolumeEcShardsMountRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.volume_ec_shards_mount(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = VolumeEcShardsMountSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.storage.Storage/VolumeEcShardsUnmount" => {
                    struct VolumeEcShardsUnmountSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage>
                        tonic::server::UnaryService<super::VolumeEcShardsUnmountRequest>
                        for VolumeEcShardsUnmountSvc<T>
                    {
                        type Response = super::VolumeEcShardsUnmountResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VolumeEcShardsUnmountRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.volume_ec_shards_unmount(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = VolumeEcShardsUnmountSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.storage.Storage/VolumeEcShardRead" => {
                    struct VolumeEcShardReadSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage>
                        tonic::server::ServerStreamingService<super::VolumeEcShardReadRequest>
                        for VolumeEcShardReadSvc<T>
                    {
                        type Response = super::VolumeEcShardReadResponse;
                        type ResponseStream = T::VolumeEcShardReadStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VolumeEcShardReadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.volume_ec_shard_read(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = VolumeEcShardReadSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.storage.Storage/VolumeEcBlobDelete" => {
                    struct VolumeEcBlobDeleteSvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage> tonic::server::UnaryService<super::VolumeEcBlobDeleteRequest>
                        for VolumeEcBlobDeleteSvc<T>
                    {
                        type Response = super::VolumeEcBlobDeleteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VolumeEcBlobDeleteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.volume_ec_blob_delete(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = VolumeEcBlobDeleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/weaver.storage.Storage/Query" => {
                    struct QuerySvc<T: Storage>(pub Arc<T>);
                    impl<T: Storage> tonic::server::ServerStreamingService<super::QueryRequest> for QuerySvc<T> {
                        type Response = super::QueriedStripe;
                        type ResponseStream = T::QueryStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.query(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = QuerySvc(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.server_streaming(method, req).await;
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
