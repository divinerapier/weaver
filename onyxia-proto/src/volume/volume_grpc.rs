// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_VOLUME_WRITE_FILE: ::grpcio::Method<super::volume::WriteFileRequest, super::volume::WriteFileResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/Volume/WriteFile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_VOLUME_READ_FILE: ::grpcio::Method<super::volume::ReadFileRequest, super::volume::ReadFileResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/Volume/ReadFile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct VolumeClient {
    client: ::grpcio::Client,
}

impl VolumeClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        VolumeClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn write_file_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::volume::WriteFileRequest>, ::grpcio::ClientCStreamReceiver<super::volume::WriteFileResponse>)> {
        self.client.client_streaming(&METHOD_VOLUME_WRITE_FILE, opt)
    }

    pub fn write_file(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::volume::WriteFileRequest>, ::grpcio::ClientCStreamReceiver<super::volume::WriteFileResponse>)> {
        self.write_file_opt(::grpcio::CallOption::default())
    }

    pub fn read_file_opt(&self, req: &super::volume::ReadFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::volume::ReadFileResponse>> {
        self.client.server_streaming(&METHOD_VOLUME_READ_FILE, req, opt)
    }

    pub fn read_file(&self, req: &super::volume::ReadFileRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::volume::ReadFileResponse>> {
        self.read_file_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Volume {
    fn write_file(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::volume::WriteFileRequest>, sink: ::grpcio::ClientStreamingSink<super::volume::WriteFileResponse>);
    fn read_file(&mut self, ctx: ::grpcio::RpcContext, req: super::volume::ReadFileRequest, sink: ::grpcio::ServerStreamingSink<super::volume::ReadFileResponse>);
}

pub fn create_volume<S: Volume + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_VOLUME_WRITE_FILE, move |ctx, req, resp| {
        instance.write_file(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_server_streaming_handler(&METHOD_VOLUME_READ_FILE, move |ctx, req, resp| {
        instance.read_file(ctx, req, resp)
    });
    builder.build()
}
