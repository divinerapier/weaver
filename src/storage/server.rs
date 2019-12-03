use crate::storage::Storage;

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
            storage: Storage::open(dir, ip, port).unwrap(),
        }
    }
}

#[tonic::async_trait]
impl weaver_proto::storage::server::Storage for StorageService {
    /// Create a new volume with the specified replica replacement.
    async fn allocate_volume(
        &self,
        request: tonic::Request<weaver_proto::storage::AllocateVolumeRequest>,
    ) -> Result<tonic::Response<weaver_proto::storage::AllocateVolumeResponse>, tonic::Status> {
        let request: weaver_proto::storage::AllocateVolumeRequest = request.into_inner();

        let replica_replacement = request
            .replica_replacement
            .map(|rr| super::volume::ReplicaReplacement::from(rr));

        self.storage.create_volume(
            request.volume_id as u64,
            &replica_replacement,
            request.max_volume_size,
            request.max_needle_count,
        )?;

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

        fn check_and_set<T>(old: &mut Option<T>, new: &Option<T>) -> crate::error::Result<()>
        where
            T: Eq + PartialEq + Clone + std::fmt::Debug,
        {
            if new.is_none() {
                return Err(crate::error!("invalid volume id or needle id"));
            }

            match old {
                Some(old) => {
                    let new = new.as_ref().unwrap();
                    let old: &T = old;
                    if !old.eq(new) {
                        return Err(crate::error!(
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
                volume_id,
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

    #[doc = "Server streaming response type for the ReadNeedle method."]
    type ReadNeedleStream =
        tokio::sync::mpsc::Receiver<Result<weaver_proto::storage::ReadNeedleResponse, Status>>;
    async fn read_needle(
        &self,
        _request: tonic::Request<weaver_proto::storage::ReadNeedleRequest>,
    ) -> Result<tonic::Response<Self::ReadNeedleStream>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
}
