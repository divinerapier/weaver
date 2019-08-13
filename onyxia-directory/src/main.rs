use std::io::Read;

use futures::future::Future;

use onyxia_proto::directory::directory_grpc;

mod server;

fn main() {
    env_logger::from_env(env_logger::Env::default().default_filter_or("trace")).init();
    log::set_max_level(log::LevelFilter::max());
    let env = std::sync::Arc::new(grpcio::Environment::new(1));
    let service = directory_grpc::create_directory(server::DirectoryService);
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
