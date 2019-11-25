use clap::App;
use clap::Arg;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Weaver Storage")
        .version("0.0.1")
        .author("divinerapier")
        .about("Storage service")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("ip")
                .long("ip")
                .value_name("IP")
                .help("Sets the public IP used by client to contact with")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .value_name("PORT")
                .help("Sets the public PORT used by client to contact with")
                .takes_value(true),
        )
        .get_matches();

    let ip = matches.value_of("ip").unwrap();
    let port = matches.value_of("port").unwrap().parse::<u16>().unwrap();

    env_logger::from_env(env_logger::Env::default().default_filter_or("trace")).init();
    log::set_max_level(log::LevelFilter::max());
    let svc = weaver_proto::storage::server::StorageServer::new(
        weaver::storage::server::StorageService::new("/data/weaver/", ip, port),
    );
    tonic::transport::Server::builder()
        .serve(format!("{}:{}", ip, port).parse().unwrap(), svc)
        .await?;
    Ok(())
}
