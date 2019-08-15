use std::io::Write;

use futures::future::Future;
use futures::stream::Stream;

use onyxia_proto::store::store;
use onyxia_proto::store::store_grpc;

pub struct StoreClient {
    client: store_grpc::StoreClient,
}

impl StoreClient {
    pub fn new(addr: &str) -> StoreClient {
        let env = std::sync::Arc::new(grpcio::EnvBuilder::new().build());
        let ch = grpcio::ChannelBuilder::new(env).connect("localhost:50051");
        let client = store_grpc::StoreClient::new(ch);
        StoreClient { client }
    }

    pub fn download_file(&self, req_path: String, output_path: String) {
        let mut req = store::ReadFileRequest::default();
        req.set_path(req_path);
        let mut output_file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .create_new(false)
            .truncate(true)
            .open(output_path)
            .unwrap();
        let mut reply = self.client.read_file(&req).unwrap();
        loop {
            let fu = reply.into_future();
            match fu.wait() {
                Ok((Some(resp), s)) => {
                    reply = s;
                    let resp: store::ReadFileResponse = resp;
                    let data = resp.get_data();
                    output_file.write_all(data).unwrap();
                }
                Ok((None, _)) => break,
                Err((e, _)) => panic!("failed to get future. error: {}", e),
            }
        }
        log::info!("read file successful");
    }
}
