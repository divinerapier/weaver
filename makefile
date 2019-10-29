build-all:
	cargo build --package weaver-proto
	cargo build
	cargo build --package weaver-directory
	cargo build --package weaver-store

release:
	cargo build --release --package weaver-proto
	cargo build --release
	cargo build --release --package weaver-directory
	cargo build --release --package weaver-store

test:
	RUST_BACKTRACE=full cargo test -- --nocapture
	
generate-proto:
	cargo build --package weaver-proto

