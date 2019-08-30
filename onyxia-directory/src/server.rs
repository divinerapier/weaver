use onyxia_proto::directory::directory;
use onyxia_proto::directory::directory_grpc;

#[derive(Clone, Copy)] // clone trait required by `fn create_directory`
pub struct DirectoryService;

impl directory_grpc::Directory for DirectoryService {
    fn write_file(
        &mut self,
        ctx: ::grpcio::RpcContext,
        stream: ::grpcio::RequestStream<directory::WriteFileRequest>,
        sink: ::grpcio::ClientStreamingSink<directory::WriteFileResponse>,
    ) {
    }
    fn read_file(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: directory::ReadFileRequest,
        sink: ::grpcio::ServerStreamingSink<directory::ReadFileResponse>,
    ) {
    }

    fn keepalive(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: directory::KeepaliveRequest,
        sink: ::grpcio::UnarySink<directory::KeepaliveResponse>,
    ) {
    }
}
