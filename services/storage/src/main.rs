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
                .help("Sets a custom config file.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("ip")
                .long("ip")
                .value_name("IP")
                .help("Sets the public IP used by client to contact with.")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .value_name("PORT")
                .help("Sets the public PORT used by client to contact with.")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("path")
                .long("path")
                .value_name("PATH")
                .help("Sets the path for storing files.")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let ip = matches.value_of("ip").unwrap();
    let port = matches.value_of("port").unwrap().parse::<u16>().unwrap();
    let path = matches.value_of("path").unwrap();

    env_logger::from_env(env_logger::Env::default().default_filter_or("trace")).init();
    log::set_max_level(log::LevelFilter::max());
    let svc = proto::storage::storage_server::StorageServer::new(
        weaver::storage::service::StorageServiceBuilder::new()
            .set_dir(path)
            .set_address(ip, port)
            .set_codec(weaver::storage::index::JSONCodec)
            .build()
            .await?,
    );
    tonic::transport::Server::builder()
        .add_service(svc)
        .serve(format!("{}:{}", ip, port).parse().unwrap())
        .await?;
    Ok(())
}
