use std::io::Read;

#[macro_use]
extern crate log;

use futures::future::Future;

use onyxia_proto::directory::directory;
use onyxia_proto::directory::directory_grpc;

#[derive(Clone, Copy)] // clone trait required by `fn create_directory`
struct DirectoryService;

impl directory_grpc::Directory for DirectoryService {
    fn write_file(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: directory::DirectoryWriteFileRequest,
        sink: ::grpcio::UnarySink<directory::DirectoryWriteFileResponse>,
    ) {
        let mut resp = directory::DirectoryWriteFileResponse::new();
        resp.set_body("hello world".as_bytes().to_owned());
        resp.set_path("onyxiafs/onyxia-examples".to_owned());
        let f = sink
            .success(resp)
            .map_err(move |e| log::error!("failed to reply {:?}. error: {:?}", req, e));
        ctx.spawn(f);
    }
}

fn main() {
    env_logger::init();
    log::set_max_level(log::LevelFilter::max());
    let env = std::sync::Arc::new(grpcio::Environment::new(1));
    let service = directory_grpc::create_directory(DirectoryService);
    let mut server = grpcio::ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50_051)
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        log::info!("listening on {}:{}", host, port);
    }
    let (tx, rx) = futures::sync::oneshot::channel();
    std::thread::spawn(move || {
        log::info!("Press ENTER to exit...");
        let _ = std::io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
