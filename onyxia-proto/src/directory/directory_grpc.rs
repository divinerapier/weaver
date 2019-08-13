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

const METHOD_DIRECTORY_WRITE_FILE: ::grpcio::Method<super::directory::DirectoryWriteFileRequest, super::directory::DirectoryWriteFileResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/Directory/WriteFile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct DirectoryClient {
    client: ::grpcio::Client,
}

impl DirectoryClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        DirectoryClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn write_file_opt(&self, req: &super::directory::DirectoryWriteFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::directory::DirectoryWriteFileResponse> {
        self.client.unary_call(&METHOD_DIRECTORY_WRITE_FILE, req, opt)
    }

    pub fn write_file(&self, req: &super::directory::DirectoryWriteFileRequest) -> ::grpcio::Result<super::directory::DirectoryWriteFileResponse> {
        self.write_file_opt(req, ::grpcio::CallOption::default())
    }

    pub fn write_file_async_opt(&self, req: &super::directory::DirectoryWriteFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::directory::DirectoryWriteFileResponse>> {
        self.client.unary_call_async(&METHOD_DIRECTORY_WRITE_FILE, req, opt)
    }

    pub fn write_file_async(&self, req: &super::directory::DirectoryWriteFileRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::directory::DirectoryWriteFileResponse>> {
        self.write_file_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Directory {
    fn write_file(&mut self, ctx: ::grpcio::RpcContext, req: super::directory::DirectoryWriteFileRequest, sink: ::grpcio::UnarySink<super::directory::DirectoryWriteFileResponse>);
}

pub fn create_directory<S: Directory + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_DIRECTORY_WRITE_FILE, move |ctx, req, resp| {
        instance.write_file(ctx, req, resp)
    });
    builder.build()
}
