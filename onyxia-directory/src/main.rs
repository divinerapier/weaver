use onyxia_proto::directory::directory_grpc;

mod server;

fn main() {
    run();
}

fn run() {
    env_logger::from_env(env_logger::Env::default().default_filter_or("trace")).init();
    log::set_max_level(log::LevelFilter::max());
    let service = directory_grpc::create_directory(server::DirectoryService {});
    libonyxia::server::Server::new(service).serve("127.0.0.1", 50_050);
}
