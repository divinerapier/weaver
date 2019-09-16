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

const METHOD_STORE_WRITE_NEEDLE: ::grpcio::Method<super::store::WriteNeedleRequest, super::store::WriteNeedleResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/Store/WriteNeedle",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_STORE_READ_NEEDLE: ::grpcio::Method<super::store::ReadNeedleRequest, super::store::ReadNeedleResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/Store/ReadNeedle",
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

    pub fn write_needle_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::store::WriteNeedleRequest>, ::grpcio::ClientCStreamReceiver<super::store::WriteNeedleResponse>)> {
        self.client.client_streaming(&METHOD_STORE_WRITE_NEEDLE, opt)
    }

    pub fn write_needle(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::store::WriteNeedleRequest>, ::grpcio::ClientCStreamReceiver<super::store::WriteNeedleResponse>)> {
        self.write_needle_opt(::grpcio::CallOption::default())
    }

    pub fn read_needle_opt(&self, req: &super::store::ReadNeedleRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::store::ReadNeedleResponse>> {
        self.client.server_streaming(&METHOD_STORE_READ_NEEDLE, req, opt)
    }

    pub fn read_needle(&self, req: &super::store::ReadNeedleRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::store::ReadNeedleResponse>> {
        self.read_needle_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Store {
    fn write_needle(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::store::WriteNeedleRequest>, sink: ::grpcio::ClientStreamingSink<super::store::WriteNeedleResponse>);
    fn read_needle(&mut self, ctx: ::grpcio::RpcContext, req: super::store::ReadNeedleRequest, sink: ::grpcio::ServerStreamingSink<super::store::ReadNeedleResponse>);
}

pub fn create_store<S: Store + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_STORE_WRITE_NEEDLE, move |ctx, req, resp| {
        instance.write_needle(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_server_streaming_handler(&METHOD_STORE_READ_NEEDLE, move |ctx, req, resp| {
        instance.read_needle(ctx, req, resp)
    });
    builder.build()
}
