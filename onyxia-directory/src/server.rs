use onyxia_proto::directory::directory;
use onyxia_proto::directory::directory_grpc;

#[derive(Clone, Copy)] // clone trait required by `fn create_directory`
pub struct DirectoryService;

impl directory_grpc::Directory for DirectoryService {
    fn register_storage_service(
        &mut self,
        ctx: ::grpcio::RpcContext,
        stream: ::grpcio::RequestStream<directory::RegisterStorageServiceRequest>,
        sink: ::grpcio::ClientStreamingSink<directory::RegisterStorageServiceResponse>,
    ) {
    }

    fn assign(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: directory::AssignRequest,
        sink: ::grpcio::UnarySink<directory::AssignResponse>,
    ) {
    }

    fn keepalive(
        &mut self,
        ctx: ::grpcio::RpcContext,
        stream: ::grpcio::RequestStream<directory::KeepaliveRequest>,
        sink: ::grpcio::DuplexSink<directory::KeepaliveResponse>,
    ) {
    }
}
