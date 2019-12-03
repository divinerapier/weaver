#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::from_env(env_logger::Env::default().default_filter_or("trace")).init();
    log::set_max_level(log::LevelFilter::max());
    let storage = weaver::directory::MemoryDirectoryStorage::new();
    let svc = weaver_proto::directory::server::DirectoryServer::new(
        weaver::directory::service::DirectoryService::new(storage),
    );
    tonic::transport::Server::builder()
        .serve("127.0.0.1:50050".parse().unwrap(), svc)
        .await?;
    Ok(())
}
