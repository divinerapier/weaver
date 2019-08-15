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

const METHOD_STORE_WRITE_FILE: ::grpcio::Method<super::store::WriteFileRequest, super::store::WriteFileResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/Store/WriteFile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_STORE_READ_FILE: ::grpcio::Method<super::store::ReadFileRequest, super::store::ReadFileResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/Store/ReadFile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct StoreClient {
    client: ::grpcio::Client,
}

impl StoreClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        StoreClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn write_file_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::store::WriteFileRequest>, ::grpcio::ClientCStreamReceiver<super::store::WriteFileResponse>)> {
        self.client.client_streaming(&METHOD_STORE_WRITE_FILE, opt)
    }

    pub fn write_file(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::store::WriteFileRequest>, ::grpcio::ClientCStreamReceiver<super::store::WriteFileResponse>)> {
        self.write_file_opt(::grpcio::CallOption::default())
    }

    pub fn read_file_opt(&self, req: &super::store::ReadFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::store::ReadFileResponse>> {
        self.client.server_streaming(&METHOD_STORE_READ_FILE, req, opt)
    }

    pub fn read_file(&self, req: &super::store::ReadFileRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::store::ReadFileResponse>> {
        self.read_file_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Store {
    fn write_file(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::store::WriteFileRequest>, sink: ::grpcio::ClientStreamingSink<super::store::WriteFileResponse>);
    fn read_file(&mut self, ctx: ::grpcio::RpcContext, req: super::store::ReadFileRequest, sink: ::grpcio::ServerStreamingSink<super::store::ReadFileResponse>);
}

pub fn create_store<S: Store + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_STORE_WRITE_FILE, move |ctx, req, resp| {
        instance.write_file(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_server_streaming_handler(&METHOD_STORE_READ_FILE, move |ctx, req, resp| {
        instance.read_file(ctx, req, resp)
    });
    builder.build()
}
