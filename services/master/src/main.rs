#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();
    log::set_max_level(log::LevelFilter::max());
    let svc = proto::master::master_server::MasterServer::new(
        weaver::master::service::MasterService::new(),
    );
    tonic::transport::Server::builder()
        .add_service(svc)
        .serve("127.0.0.1:50050".parse().unwrap())
        .await?;
    Ok(())
}
