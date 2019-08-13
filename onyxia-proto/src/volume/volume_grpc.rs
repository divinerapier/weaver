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

const METHOD_VOLUME_WRITE_FILE: ::grpcio::Method<super::volume::VolumeWriteFileRequest, super::volume::VolumeWriteFileResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/Volume/WriteFile",
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

    pub fn write_file_opt(&self, req: &super::volume::VolumeWriteFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::volume::VolumeWriteFileResponse> {
        self.client.unary_call(&METHOD_VOLUME_WRITE_FILE, req, opt)
    }

    pub fn write_file(&self, req: &super::volume::VolumeWriteFileRequest) -> ::grpcio::Result<super::volume::VolumeWriteFileResponse> {
        self.write_file_opt(req, ::grpcio::CallOption::default())
    }

    pub fn write_file_async_opt(&self, req: &super::volume::VolumeWriteFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::volume::VolumeWriteFileResponse>> {
        self.client.unary_call_async(&METHOD_VOLUME_WRITE_FILE, req, opt)
    }

    pub fn write_file_async(&self, req: &super::volume::VolumeWriteFileRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::volume::VolumeWriteFileResponse>> {
        self.write_file_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Volume {
    fn write_file(&mut self, ctx: ::grpcio::RpcContext, req: super::volume::VolumeWriteFileRequest, sink: ::grpcio::UnarySink<super::volume::VolumeWriteFileResponse>);
}

pub fn create_volume<S: Volume + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_VOLUME_WRITE_FILE, move |ctx, req, resp| {
        instance.write_file(ctx, req, resp)
    });
    builder.build()
}
