fn main() {
    tonic_build::compile_protos("proto/weaver.proto").unwrap();
    tonic_build::compile_protos("proto/directory.proto").unwrap();
    tonic_build::compile_protos("proto/storage.proto").unwrap();
    tonic_build::compile_protos("proto/master.proto").unwrap();
}
