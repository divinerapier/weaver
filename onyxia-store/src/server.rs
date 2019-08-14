use futures::future::Future;
use futures::sink::Sink;
use futures::stream::Stream;

use onyxia_proto::volume::*;

#[derive(Clone, Copy)] // clone trait required by `fn create_directory`
pub struct VolumeService;

impl volume_grpc::Volume for VolumeService {
    fn write_file(
        &mut self,
        ctx: ::grpcio::RpcContext,
        stream: ::grpcio::RequestStream<volume::WriteFileRequest>,
        sink: ::grpcio::ClientStreamingSink<volume::WriteFileResponse>,
    ) {
    }
    fn read_file(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: volume::ReadFileRequest,
        sink: ::grpcio::ServerStreamingSink<volume::ReadFileResponse>,
    ) {
        let path = String::from(req.get_path());
        let f = std::fs::OpenOptions::new()
            .read(true)
            .open(path.to_string())
            .unwrap();
        let file_stream = libonyxia::file::FileStream::from_std_file(f);
        let mut offset = 0;
        let s = file_stream.map(move |chunk| {
            let chunk: bytes::Bytes = chunk;
            let mut resp = volume::ReadFileResponse::new();
            resp.set_length(chunk.len() as i64);
            resp.set_offset(offset);
            let mut status = volume::CommonStatus::default();
            status.set_status_code(200);
            resp.set_status(status);
            resp.set_path(path.to_string());
            resp.set_data(chunk.as_ref().to_owned());
            offset += chunk.len() as i64;
            (resp, grpcio::WriteFlags::default())
        });
        // Spawn executes futures of which Item and Error both should
        // be empty tuple. So we must call map and map_err with empty
        // tuple returned.
        let f = sink
            .send_all(s)
            .map(|_| {})
            .map_err(|e| log::error!("failed to send response. error: {}", e));
        ctx.spawn(f);
    }
}
