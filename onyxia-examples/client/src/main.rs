use onyxia_proto::directory::directory;
use onyxia_proto::directory::directory_grpc;

fn main() {
    env_logger::init();
    log::set_max_level(log::LevelFilter::max());
    let env = std::sync::Arc::new(grpcio::EnvBuilder::new().build());
    let ch = grpcio::ChannelBuilder::new(env).connect("localhost:50051");
    let client = directory_grpc::DirectoryClient::new(ch);

    let mut req = directory::DirectoryWriteFileRequest::default();
    req.set_path("/public/static/void/main".to_owned());
    let reply = client.write_file(&req).expect("failed");
    log::info!("Directory Service received: {}", reply.get_path());
}
