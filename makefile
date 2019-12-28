build: build-storage build-master build-directory

build-library: build-proto
	cargo build

build-storage: build-library
	cargo build --package storage
	
build-master: build-library
	cargo build --package master

build-directory: build-library
	cargo build --package directory

release: build-storage build-master build-directory


release-library: release-proto
	cargo build --release

release-storage: release-library
	cargo build --release --package storage
	
release-master: release-library
	cargo build --release --package master

release-directory: release-library
	cargo build --release --package directory

test: build-library
	RUST_BACKTRACE=full cargo test -- --nocapture
	
build-proto: format
	cargo build --package proto

release-proto: format
	cargo build --release --package proto

format:
	cargo fmt

proto: build-proto release-proto
