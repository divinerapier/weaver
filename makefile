test:
	RUST_BACKTRACE=full cargo test --package libonyxia -- --nocapture
	
generate-proto:
	cargo build --package onyxia-proto

build-all:
	cargo build --package onyxia-proto
	cargo build --package libonyxia
	cargo build --package onyxia-directory
	cargo build --package onyxia-store
