use std::sync::{Arc, RwLock, RwLockReadGuard};

use libonyxia::error::Error;
use libonyxia::needle::{Needle, NeedleStream};
use libonyxia::store::Store;
// use onyxia_proto::store::*;

// use futures::future::Future;
// use futures::sink::Sink;
// use futures::stream::Stream;

use futures::{Stream, StreamExt};
use onyxia_proto::*;
use tokio::sync::mpsc;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

// clone trait required by `fn create_directory`, so inner fields must be arc
#[derive(Clone)]
pub struct StoreService {
    storage: Arc<RwLock<Store>>,
}

impl StoreService {
    pub fn new(dir: &str) -> StoreService {
        StoreService {
            storage: Arc::new(RwLock::new(Store::new(dir).unwrap())),
        }
    }
}

#[tonic::async_trait]
impl onyxia_proto::store::server::Store for StoreService {

    async fn write_needle(
        &self,
        request: tonic::Request<tonic::Streaming<onyxia_proto::store::WriteNeedleRequest>>,
    ) -> Result<tonic::Response<onyxia_proto::store::WriteNeedleResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    type ReadNeedleStream = mpsc::Receiver<Result<onyxia_proto::store::ReadNeedleResponse, Status>>;

    async fn read_needle(
        &self,
        request: tonic::Request<onyxia_proto::store::ReadNeedleRequest>,
    ) -> Result<tonic::Response<Self::ReadNeedleStream>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    // fn write_needle(
    //     &mut self,
    //     ctx: ::grpcio::RpcContext,
    //     stream: ::grpcio::RequestStream<store::WriteNeedleRequest>,
    //     sink: ::grpcio::ClientStreamingSink<store::WriteNeedleResponse>,
    // ) {
    // }
    // fn read_needle(
    //     &mut self,
    //     ctx: ::grpcio::RpcContext,
    //     req: store::ReadNeedleRequest,
    //     sink: ::grpcio::ServerStreamingSink<store::ReadNeedleResponse>,
    // ) {
    //     let volume_id = req.volume_id;
    //     let needle_id = req.needle_id;
    //     let storage: RwLockReadGuard<'_, Store> = self.storage.read().unwrap();
    //     match storage.read_needle(volume_id as u32, needle_id as u64) {
    //         Ok(needle) => {
    //             let needle_stream: NeedleStream = needle.into();
    //             let mut offset = 0;
    //             let s = needle_stream.map(move |chunk| {
    //                 let chunk: bytes::Bytes = chunk;
    //                 let mut resp = store::ReadNeedleResponse::new();
    //                 resp.set_length(chunk.len() as i64);
    //                 resp.set_offset(offset);
    //                 let mut status = store::CommonStatus::default();
    //                 status.set_status_code(200);
    //                 resp.set_status(status);
    //                 resp.set_volume_id(volume_id);
    //                 resp.set_needle_id(needle_id);
    //                 resp.set_data(chunk.as_ref().to_owned());
    //                 offset += chunk.len() as i64;
    //                 (resp, grpcio::WriteFlags::default())
    //             });
    //             // Spawn executes futures of which Item and Error both should
    //             // be empty tuple. So we must call map and map_err with empty
    //             // tuple returned.
    //             let f = sink
    //                 .send_all(s)
    //                 .map(|_| {})
    //                 .map_err(|e| log::error!("failed to send response. error: {}", e));
    //             ctx.spawn(f);
    //         }
    //         Err(message) => {
    //             let mut resp = store::ReadNeedleResponse::new();
    //             let mut status = store::CommonStatus::default();
    //             status.set_message(message.to_string());
    //             resp.set_status(status);

    //             // let s = futures::stream::iter_ok::<_, Error>(vec![(
    //             //     resp,
    //             //     grpcio::WriteFlags::default(),
    //             // )]);

    //             let s =
    //                 futures::stream::once::<_, Error>(Ok((resp, grpcio::WriteFlags::default())));
    //             let f = sink
    //                 .send_all(s)
    //                 .map(|_| {})
    //                 .map_err(|e| log::error!("failed to send response. error: {}", e));
    //             ctx.spawn(f);
    //         }
    //     }
    // }
}
