use onyxia_proto::store::store_grpc;

mod client;
mod server;

fn main() {
    run();
}

fn run() {
    env_logger::from_env(env_logger::Env::default().default_filter_or("trace")).init();
    log::set_max_level(log::LevelFilter::max());
    let service = store_grpc::create_store(server::StoreService::new("/data/onyxia/"));
    libonyxia::server::Server::new(service).serve("127.0.0.1", 50_051);
}

// #[cfg(test)]
// mod test {
//     #[test]
//     fn test_read_file() {
//         use std::io::Write;

//         std::thread::spawn(move || {
//             super::run();
//         });

//         std::thread::sleep(std::time::Duration::from_secs(5));
//         std::fs::remove_dir_all("/data/onyxia/");
//         std::fs::create_dir_all("/data/onyxia/").expect("create test dir");
//         let mut file = std::fs::OpenOptions::new()
//             .append(false)
//             .create(true)
//             .truncate(true)
//             .write(true)
//             .read(true)
//             .create_new(false)
//             .open("/data/onyxia/1.txt")
//             .expect("unable to open file");
//         file.write_all("Hello, Onyxia!".as_bytes())
//             .expect("unable to write");
//         let client = super::client::StoreClient::new(":50051");
//         client.download_file(
//             "/data/onyxia/1.txt".to_string(),
//             "/data/onyxia/2.txt".to_string(),
//         );
//         assert_eq!(
//             md5_file("/data/onyxia/1.txt").expect("/data/onyxia/1.txt"),
//             md5_file("/data/onyxia/2.txt").expect("/data/onyxia/2.txt")
//         );
//     }

//     fn md5_file(path: &str) -> Option<Vec<u8>> {
//         if let Ok(mut file) = std::fs::File::open(&path) {
//             return process::<md5::Md5, _>(&mut file, &path);
//         }
//         None
//     }

//     fn process<D: md5::Digest + Default, R: std::io::Read>(
//         reader: &mut R,
//         name: &str,
//     ) -> Option<Vec<u8>> {
//         const BUFFER_SIZE: usize = 1024;

//         let mut sh = D::default();
//         let mut buffer = [0u8; BUFFER_SIZE];
//         loop {
//             let n = match reader.read(&mut buffer) {
//                 Ok(n) => n,
//                 Err(_) => return None,
//             };
//             sh.input(&buffer[..n]);
//             if n == 0 || n < BUFFER_SIZE {
//                 break;
//             }
//         }
//         let result: &[u8] = &sh.result();
//         Some(Vec::from(result))
//     }

//     fn print_result(sum: &[u8], name: &str) {
//         for byte in sum {
//             print!("{:02x}", byte);
//         }
//         println!("\t{}", name);
//     }
// }
