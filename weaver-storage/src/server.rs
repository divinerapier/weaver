use std::sync::{Arc, RwLock, RwLockReadGuard};

use weaver::error::Error;
use weaver::needle::Needle;
use weaver::storage::Storage;

use futures::{Stream, StreamExt};
use tokio::sync::mpsc;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

// clone trait required by `fn create_directory`, so inner fields must be arc
pub struct StorageService {
    storage: Storage,
}

impl StorageService {
    pub fn new(dir: &str, ip: &str, port: u16) -> StorageService {
        StorageService {
            storage: Storage::new(dir, ip, port).unwrap(),
        }
    }
}

#[tonic::async_trait]
impl weaver_proto::storage::server::Storage for StorageService {
    /// Create a new volume.
    async fn allocate_volume(
        &self,
        request: tonic::Request<weaver_proto::storage::AllocateVolumeRequest>,
    ) -> Result<tonic::Response<weaver_proto::storage::AllocateVolumeResponse>, tonic::Status> {
        let request: weaver_proto::storage::AllocateVolumeRequest = request.into_inner();

        let replica_replacement = request
            .replica_replacement
            .map(|rr| weaver::storage::volume::ReplicaReplacement::from(rr));

        self.storage
            .create_volume(request.volume_id as u64, &replica_replacement, 128, 128)?;

        Ok(tonic::Response::new(
            weaver_proto::storage::AllocateVolumeResponse { status: None },
        ))
    }

    /// Write the needle to a volume.
    async fn write_needle(
        &self,
        request: tonic::Request<tonic::Streaming<weaver_proto::storage::WriteNeedleRequest>>,
    ) -> Result<tonic::Response<weaver_proto::storage::WriteNeedleResponse>, tonic::Status> {
        let stream = request.into_inner();
        futures::pin_mut!(stream);

        fn check_and_set<T>(old: &mut Option<T>, new: &Option<T>) -> weaver::error::Result<()>
        where
            T: Eq + PartialEq + Clone + std::fmt::Debug,
        {
            if new.is_none() {
                return Err(weaver::error!("invalid volume id or needle id"));
            }

            match old {
                Some(old) => {
                    let new = new.as_ref().unwrap();
                    let old: &T = old;
                    if !old.eq(new) {
                        return Err(weaver::error!(
                            "mismatched volume or needle. old: {:?}, new: {:?}",
                            old,
                            new
                        ));
                    }
                }
                old @ None => {
                    //*old = Some(new.clone()),
                    *old = new.clone()
                }
            }
            Ok(())
        };

        let mut volume_id: Option<u64> = None;
        let mut needle_header: Option<weaver_proto::weaver::NeedleHeader> = None;
        let mut buffer = Vec::with_capacity(32 * 1024 * 1024);
        while let Some(request) = stream.next().await {
            let request: weaver_proto::storage::WriteNeedleRequest = request?;
            check_and_set::<u64>(
                &mut volume_id,
                &if request.volume_id == 0 {
                    None
                } else {
                    Some(request.volume_id)
                },
            )?;
            check_and_set::<weaver_proto::weaver::NeedleHeader>(
                &mut needle_header,
                &request.needle_header,
            )?;
            buffer.extend_from_slice(&request.content);
        }

        if volume_id.is_none() {
            return Err(tonic::Status::new(
                tonic::Code::InvalidArgument,
                "missing volume id",
            ));
        }

        if needle_header.is_none() {
            return Err(tonic::Status::new(
                tonic::Code::InvalidArgument,
                "missing needle id",
            ));
        }

        let volume_id = volume_id.unwrap();
        let needle_header = needle_header.unwrap();

        self.storage
            .write_needle(
                0,
                &weaver_proto::weaver::Needle {
                    header: Some(needle_header),
                    body: buffer,
                },
            )
            .await?;
        Ok(tonic::Response::new(
            weaver_proto::storage::WriteNeedleResponse {},
        ))
    }

    #[doc = "Server streaming response type for the VolumeIncrementalCopy method."]
    type VolumeIncrementalCopyStream = tokio::sync::mpsc::Receiver<
        Result<weaver_proto::storage::VolumeIncrementalCopyResponse, Status>,
    >;

    #[doc = "Server streaming response type for the CopyFile method."]
    type CopyFileStream =
        tokio::sync::mpsc::Receiver<Result<weaver_proto::storage::CopyFileResponse, Status>>;

    #[doc = "Server streaming response type for the VolumeTailSender method."]
    type VolumeTailSenderStream = tokio::sync::mpsc::Receiver<
        Result<weaver_proto::storage::VolumeTailSenderResponse, Status>,
    >;

    #[doc = "Server streaming response type for the VolumeEcShardRead method."]
    type VolumeEcShardReadStream = tokio::sync::mpsc::Receiver<
        Result<weaver_proto::storage::VolumeEcShardReadResponse, Status>,
    >;

    #[doc = "Server streaming response type for the Query method."]
    type QueryStream =
        tokio::sync::mpsc::Receiver<Result<weaver_proto::storage::QueriedStripe, Status>>;

    #[doc = "Experts only: takes multiple fid parameters. This function does not propagate deletes to replicas."]

    async fn batch_delete(
        &self,
        request: tonic::Request<weaver_proto::storage::BatchDeleteRequest>,
    ) -> Result<tonic::Response<weaver_proto::storage::BatchDeleteResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn vacuum_volume_check(
        &self,
        request: tonic::Request<weaver_proto::storage::VacuumVolumeCheckRequest>,
    ) -> Result<tonic::Response<weaver_proto::storage::VacuumVolumeCheckResponse>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn vacuum_volume_compact(
        &self,
        request: tonic::Request<weaver_proto::storage::VacuumVolumeCompactRequest>,
    ) -> Result<tonic::Response<weaver_proto::storage::VacuumVolumeCompactResponse>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn vacuum_volume_commit(
        &self,
        request: tonic::Request<weaver_proto::storage::VacuumVolumeCommitRequest>,
    ) -> Result<tonic::Response<weaver_proto::storage::VacuumVolumeCommitResponse>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn vacuum_volume_cleanup(
        &self,
        request: tonic::Request<weaver_proto::storage::VacuumVolumeCleanupRequest>,
    ) -> Result<tonic::Response<weaver_proto::storage::VacuumVolumeCleanupResponse>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn delete_collection(
        &self,
        request: tonic::Request<weaver_proto::storage::DeleteCollectionRequest>,
    ) -> Result<tonic::Response<weaver_proto::storage::DeleteCollectionResponse>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn volume_sync_status(
        &self,
        request: tonic::Request<weaver_proto::storage::VolumeSyncStatusRequest>,
    ) -> Result<tonic::Response<weaver_proto::storage::VolumeSyncStatusResponse>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn volume_incremental_copy(
        &self,
        request: tonic::Request<weaver_proto::storage::VolumeIncrementalCopyRequest>,
    ) -> Result<tonic::Response<Self::VolumeIncrementalCopyStream>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn volume_mount(
        &self,
        request: tonic::Request<weaver_proto::storage::VolumeMountRequest>,
    ) -> Result<tonic::Response<weaver_proto::storage::VolumeMountResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn volume_unmount(
        &self,
        request: tonic::Request<weaver_proto::storage::VolumeUnmountRequest>,
    ) -> Result<tonic::Response<weaver_proto::storage::VolumeUnmountResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn volume_delete(
        &self,
        request: tonic::Request<weaver_proto::storage::VolumeDeleteRequest>,
    ) -> Result<tonic::Response<weaver_proto::storage::VolumeDeleteResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn volume_mark_readonly(
        &self,
        request: tonic::Request<weaver_proto::storage::VolumeMarkReadonlyRequest>,
    ) -> Result<tonic::Response<weaver_proto::storage::VolumeMarkReadonlyResponse>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
    #[doc = " copy the .idx .dat files, and mount this volume"]

    async fn volume_copy(
        &self,
        request: tonic::Request<weaver_proto::storage::VolumeCopyRequest>,
    ) -> Result<tonic::Response<weaver_proto::storage::VolumeCopyResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn read_volume_file_status(
        &self,
        request: tonic::Request<weaver_proto::storage::ReadVolumeFileStatusRequest>,
    ) -> Result<tonic::Response<weaver_proto::storage::ReadVolumeFileStatusResponse>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn copy_file(
        &self,
        request: tonic::Request<weaver_proto::storage::CopyFileRequest>,
    ) -> Result<tonic::Response<Self::CopyFileStream>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn volume_tail_sender(
        &self,
        request: tonic::Request<weaver_proto::storage::VolumeTailSenderRequest>,
    ) -> Result<tonic::Response<Self::VolumeTailSenderStream>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn volume_tail_receiver(
        &self,
        request: tonic::Request<weaver_proto::storage::VolumeTailReceiverRequest>,
    ) -> Result<tonic::Response<weaver_proto::storage::VolumeTailReceiverResponse>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
    #[doc = " erasure coding"]

    async fn volume_ec_shards_generate(
        &self,
        request: tonic::Request<weaver_proto::storage::VolumeEcShardsGenerateRequest>,
    ) -> Result<tonic::Response<weaver_proto::storage::VolumeEcShardsGenerateResponse>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn volume_ec_shards_rebuild(
        &self,
        request: tonic::Request<weaver_proto::storage::VolumeEcShardsRebuildRequest>,
    ) -> Result<tonic::Response<weaver_proto::storage::VolumeEcShardsRebuildResponse>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn volume_ec_shards_copy(
        &self,
        request: tonic::Request<weaver_proto::storage::VolumeEcShardsCopyRequest>,
    ) -> Result<tonic::Response<weaver_proto::storage::VolumeEcShardsCopyResponse>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn volume_ec_shards_delete(
        &self,
        request: tonic::Request<weaver_proto::storage::VolumeEcShardsDeleteRequest>,
    ) -> Result<tonic::Response<weaver_proto::storage::VolumeEcShardsDeleteResponse>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn volume_ec_shards_mount(
        &self,
        request: tonic::Request<weaver_proto::storage::VolumeEcShardsMountRequest>,
    ) -> Result<tonic::Response<weaver_proto::storage::VolumeEcShardsMountResponse>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn volume_ec_shards_unmount(
        &self,
        request: tonic::Request<weaver_proto::storage::VolumeEcShardsUnmountRequest>,
    ) -> Result<tonic::Response<weaver_proto::storage::VolumeEcShardsUnmountResponse>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn volume_ec_shard_read(
        &self,
        request: tonic::Request<weaver_proto::storage::VolumeEcShardReadRequest>,
    ) -> Result<tonic::Response<Self::VolumeEcShardReadStream>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn volume_ec_blob_delete(
        &self,
        request: tonic::Request<weaver_proto::storage::VolumeEcBlobDeleteRequest>,
    ) -> Result<tonic::Response<weaver_proto::storage::VolumeEcBlobDeleteResponse>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    #[doc = " query"]

    async fn query(
        &self,
        request: tonic::Request<weaver_proto::storage::QueryRequest>,
    ) -> Result<tonic::Response<Self::QueryStream>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
}
