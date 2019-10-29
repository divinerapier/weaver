fn main() {
    tonic_build::compile_protos("proto/directory/directory.proto").unwrap();
    tonic_build::compile_protos("proto/store/store.proto").unwrap();
}
