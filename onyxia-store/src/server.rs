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
        // let f = std::fs::OpenOptions::new()
        //     .read(true)
        //     .open("/data/2.txt")
        //     .unwrap();
        // let file_stream = libonyxia::file::FileStream::from_std_file(f);
        // let s = file_stream.map(|chunk| {
        //     (volume::ReadFileResponse::default(), grpcio::WriteFlags::default())
        // }).map_err(|e| {
        //     grpcio::Error::Codec(Box::new(e))
        // });
        // sink.send_all(s);
    }
}
