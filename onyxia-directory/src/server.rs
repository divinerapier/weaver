use std::sync::{Arc, Mutex};

use onyxia_proto::directory::directory;
use onyxia_proto::directory::directory_grpc;

#[derive(Clone)] // clone trait required by `fn create_directory`
pub struct DirectoryService {
    dir: Arc<Mutex<libonyxia::directory::Directory>>,
}

impl DirectoryService {
    pub fn new() -> DirectoryService {
        DirectoryService {
            dir: Arc::new(Mutex::new(libonyxia::directory::Directory::new())),
        }
    }
}

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
        let replication_count = req.get_replication_count();
        let dir = self.dir.clone();
        let dir: std::sync::MutexGuard<'_, libonyxia::directory::Directory> = dir.lock().unwrap();
        for i in 0..replication_count as usize {
            match dir.random_writable_volume() {
                Some(i) => {}
                None => {}
            }
        }
    }

    fn keepalive(
        &mut self,
        ctx: ::grpcio::RpcContext,
        stream: ::grpcio::RequestStream<directory::KeepaliveRequest>,
        sink: ::grpcio::DuplexSink<directory::KeepaliveResponse>,
    ) {
    }
}
