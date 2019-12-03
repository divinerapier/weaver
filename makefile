build: build-storage build-master build-directory

build-library: build-proto
	cargo build

build-storage: build-library
	cargo build --package weaver-storage
	
build-master: build-library
	cargo build --package weaver-master

build-directory: build-library
	cargo build --package weaver-directory

release: build-storage build-master build-directory


release-library: release-proto
	cargo build --release

release-storage: release-library
	cargo build --release --package weaver-storage
	
release-master: release-library
	cargo build --release --package weaver-master

release-directory: release-library
	cargo build --release --package weaver-directory

test: build-library
	RUST_BACKTRACE=full cargo test -- --nocapture
	
build-proto: format
	cargo build --package weaver-proto

release-proto: format
	cargo build --release --package weaver-proto

format:
	cargo fmt

proto: build-proto release-proto
