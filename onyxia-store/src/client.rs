use std::io::Write;

use futures::future::Future;
use futures::stream::Stream;

use onyxia_proto::volume::volume;
use onyxia_proto::volume::volume_grpc;

pub struct VolumeClient {
    client: volume_grpc::VolumeClient,
}

impl VolumeClient {
    pub fn new(addr: &str) -> VolumeClient {
        let env = std::sync::Arc::new(grpcio::EnvBuilder::new().build());
        let ch = grpcio::ChannelBuilder::new(env).connect("localhost:50051");
        let client = volume_grpc::VolumeClient::new(ch);
        VolumeClient { client }
    }

    pub fn download_file(&self, req_path: String, output_path: String) {
        let mut req = volume::ReadFileRequest::default();
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
                    let resp: volume::ReadFileResponse = resp;
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
