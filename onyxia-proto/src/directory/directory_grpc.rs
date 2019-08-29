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

const METHOD_EXAMPLE_DIRECTORY_WRITE_FILE: ::grpcio::Method<super::directory::ExampleDirectoryWriteFileRequest, super::directory::ExampleDirectoryWriteFileResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/ExampleDirectory/WriteFile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ExampleDirectoryClient {
    client: ::grpcio::Client,
}

impl ExampleDirectoryClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ExampleDirectoryClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn write_file_opt(&self, req: &super::directory::ExampleDirectoryWriteFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::directory::ExampleDirectoryWriteFileResponse> {
        self.client.unary_call(&METHOD_EXAMPLE_DIRECTORY_WRITE_FILE, req, opt)
    }

    pub fn write_file(&self, req: &super::directory::ExampleDirectoryWriteFileRequest) -> ::grpcio::Result<super::directory::ExampleDirectoryWriteFileResponse> {
        self.write_file_opt(req, ::grpcio::CallOption::default())
    }

    pub fn write_file_async_opt(&self, req: &super::directory::ExampleDirectoryWriteFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::directory::ExampleDirectoryWriteFileResponse>> {
        self.client.unary_call_async(&METHOD_EXAMPLE_DIRECTORY_WRITE_FILE, req, opt)
    }

    pub fn write_file_async(&self, req: &super::directory::ExampleDirectoryWriteFileRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::directory::ExampleDirectoryWriteFileResponse>> {
        self.write_file_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ExampleDirectory {
    fn write_file(&mut self, ctx: ::grpcio::RpcContext, req: super::directory::ExampleDirectoryWriteFileRequest, sink: ::grpcio::UnarySink<super::directory::ExampleDirectoryWriteFileResponse>);
}

pub fn create_example_directory<S: ExampleDirectory + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_EXAMPLE_DIRECTORY_WRITE_FILE, move |ctx, req, resp| {
        instance.write_file(ctx, req, resp)
    });
    builder.build()
}

const METHOD_DIRECTORY_WRITE_FILE: ::grpcio::Method<super::directory::WriteFileRequest, super::directory::WriteFileResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/Directory/WriteFile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DIRECTORY_READ_FILE: ::grpcio::Method<super::directory::ReadFileRequest, super::directory::ReadFileResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/Directory/ReadFile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DIRECTORY_KEEPALIVE: ::grpcio::Method<super::directory::KeepaliveRequest, super::directory::KeepaliveResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/Directory/Keepalive",
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

    pub fn write_file_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::directory::WriteFileRequest>, ::grpcio::ClientCStreamReceiver<super::directory::WriteFileResponse>)> {
        self.client.client_streaming(&METHOD_DIRECTORY_WRITE_FILE, opt)
    }

    pub fn write_file(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::directory::WriteFileRequest>, ::grpcio::ClientCStreamReceiver<super::directory::WriteFileResponse>)> {
        self.write_file_opt(::grpcio::CallOption::default())
    }

    pub fn read_file_opt(&self, req: &super::directory::ReadFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::directory::ReadFileResponse>> {
        self.client.server_streaming(&METHOD_DIRECTORY_READ_FILE, req, opt)
    }

    pub fn read_file(&self, req: &super::directory::ReadFileRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::directory::ReadFileResponse>> {
        self.read_file_opt(req, ::grpcio::CallOption::default())
    }

    pub fn keepalive_opt(&self, req: &super::directory::KeepaliveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::directory::KeepaliveResponse> {
        self.client.unary_call(&METHOD_DIRECTORY_KEEPALIVE, req, opt)
    }

    pub fn keepalive(&self, req: &super::directory::KeepaliveRequest) -> ::grpcio::Result<super::directory::KeepaliveResponse> {
        self.keepalive_opt(req, ::grpcio::CallOption::default())
    }

    pub fn keepalive_async_opt(&self, req: &super::directory::KeepaliveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::directory::KeepaliveResponse>> {
        self.client.unary_call_async(&METHOD_DIRECTORY_KEEPALIVE, req, opt)
    }

    pub fn keepalive_async(&self, req: &super::directory::KeepaliveRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::directory::KeepaliveResponse>> {
        self.keepalive_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Directory {
    fn write_file(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::directory::WriteFileRequest>, sink: ::grpcio::ClientStreamingSink<super::directory::WriteFileResponse>);
    fn read_file(&mut self, ctx: ::grpcio::RpcContext, req: super::directory::ReadFileRequest, sink: ::grpcio::ServerStreamingSink<super::directory::ReadFileResponse>);
    fn keepalive(&mut self, ctx: ::grpcio::RpcContext, req: super::directory::KeepaliveRequest, sink: ::grpcio::UnarySink<super::directory::KeepaliveResponse>);
}

pub fn create_directory<S: Directory + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_DIRECTORY_WRITE_FILE, move |ctx, req, resp| {
        instance.write_file(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_DIRECTORY_READ_FILE, move |ctx, req, resp| {
        instance.read_file(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_DIRECTORY_KEEPALIVE, move |ctx, req, resp| {
        instance.keepalive(ctx, req, resp)
    });
    builder.build()
}
