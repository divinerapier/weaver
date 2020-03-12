use futures::{Stream, StreamExt};
use serde_json::ser::State;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use proto::storage::*;
use super::index::Codec;

use super::storage::Storage;

// clone trait required by `fn create_directory`, so inner fields must be arc
pub struct StorageService<C> where C: Codec {
    storage: Storage<C>,
}

impl<C> StorageService<C> where C: Codec + Send + Sync + 'static {
    pub async fn new(dir: &str, ip: &str, port: u16, codec: C) -> StorageService<C> {
        StorageService {
            storage: Storage::open(dir, ip, port, codec).await.unwrap(),
        }
    }
}

#[tonic::async_trait]
impl<C> storage_server::Storage for StorageService<C> where C: Codec + Send + Sync + 'static {
    /// Create a new volume with the specified replica replacement.
    async fn allocate_volume(
        &self,
        request: Request<AllocateVolumeRequest>,
    ) -> Result<Response<AllocateVolumeResponse>, Status> {
        let request: AllocateVolumeRequest = request.into_inner();

        let replica_replacement = request
            .replica_replacement
            .map(|rr| super::volume::ReplicaReplacement::from(rr));

        self.storage
            .create_volume(
                request.volume_id as u64,
                &replica_replacement,
                request.max_volume_size,
                request.max_needle_count,
            )
            .await?;

        Ok(Response::new(AllocateVolumeResponse { status: None }))
    }

    /// Write the needle to a volume.
    async fn write_needle(
        &self,
        request: Request<tonic::Streaming<WriteNeedleRequest>>,
    ) -> Result<Response<WriteNeedleResponse>, Status> {
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
        let mut needle_header: Option<proto::weaver::NeedleHeader> = None;
        let mut buffer = Vec::with_capacity(32 * 1024 * 1024);
        while let Some(request) = stream.next().await {
            let request: WriteNeedleRequest = request?;
            check_and_set::<u64>(
                &mut volume_id,
                &if request.volume_id == 0 {
                    None
                } else {
                    Some(request.volume_id)
                },
            )?;
            check_and_set::<proto::weaver::NeedleHeader>(
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
                &proto::weaver::Needle {
                    header: Some(needle_header),
                    body: buffer,
                },
            )
            .await?;
        Ok(Response::new(WriteNeedleResponse {}))
    }

    #[doc = "Server streaming response type for the ReadNeedle method."]
    type ReadNeedleStream = Receiver<Result<ReadNeedleResponse, Status>>;
    async fn read_needle(
        &self,
        request: Request<ReadNeedleRequest>,
    ) -> Result<Response<Self::ReadNeedleStream>, Status> {
        let request: ReadNeedleRequest = request.into_inner();
        let volume_id = request.volume_id;
        let needle_id = request.needle_id;
        // let storage = self.storage.clone();
        let needle = self.storage.read_needle(volume_id, needle_id).await?;
        let (mut tx, rx) = tokio::sync::mpsc::channel(1);

        tokio::spawn(async move {
            let mut iter = needle.body.into_iter();
            let mut offset = 0;
            while let Some(body) = iter.next() {
                let res = match body {
                    Ok(body) => {
                        let body: bytes::Bytes = body;
                        let body: Vec<u8> = body.to_vec();
                        let body_length = body.len() as u64;
                        let r = tx.send(Ok(ReadNeedleResponse {
                            volume_id,
                            needle_id,
                            offset,
                            size: body_length,
                            content: body,
                        }));
                        offset += body_length;
                        r
                    }
                    Err(err) => tx.send(Err(Status::from(err))),
                }
                    .await;
                if let Err(e) = res {
                    log::error!("failed to send needle body. {}", e);
                }
            }
        });

        Ok(tonic::Response::new(rx))
    }
}
