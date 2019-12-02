#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::from_env(env_logger::Env::default().default_filter_or("trace")).init();
    log::set_max_level(log::LevelFilter::max());
    let svc = weaver_proto::master::server::MasterServer::new(
        weaver::master::server::MasterService::new(),
    );
    tonic::transport::Server::builder()
        .serve("127.0.0.1:50050".parse().unwrap(), svc)
        .await?;
    Ok(())
}
