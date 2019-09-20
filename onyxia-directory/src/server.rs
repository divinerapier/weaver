use std::sync::{Arc, RwLock};

use onyxia_proto::directory::directory;
use onyxia_proto::directory::directory_grpc;

#[derive(Clone)] // clone trait required by `fn create_directory`
pub struct DirectoryService {
    dir: Arc<RwLock<libonyxia::directory::Directory>>,
}

impl DirectoryService {
    pub fn new() -> DirectoryService {
        DirectoryService {
            dir: Arc::new(RwLock::new(libonyxia::directory::Directory::new())),
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
        let dir: std::sync::RwLockReadGuard<'_, libonyxia::directory::Directory> =
            dir.read().unwrap();
        let mut resp = directory::AssignResponse::new();

        if replication_count <= 0 {
            resp.mut_status()
                .set_message(format!("invalid replication count: {}", replication_count));
            resp.mut_status().set_status_code(400);
            sink.send();
            return;
        }

        if dir.volumes.len() < replication_count as usize {
            return;
        }

        for i in 0..replication_count as usize {
            match dir.random_writable_volume() {
                Some(volume_id) => {
                    if i == 0 {
                        let mut master = directory::Replica::new();
                        master.set_volume_id(volume_id);
                        resp.set_master(master);
                    } else {
                        let mut replica = directory::Replica::new();
                        replica.set_volume_id(volume_id);
                        resp.mut_slaves().push(replica);
                    }
                }
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
