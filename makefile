build-all:
	cargo build --package weaver-proto
	cargo build
	cargo build --package weaver-directory
	cargo build --package weaver-storage
	cargo build --package weaver-master

release:
	cargo build --release --package weaver-proto
	cargo build --release
	cargo build --release --package weaver-directory
	cargo build --release --package weaver-storage
	cargo build --release --package weaver-master

test: cargo-fmt generate-proto
	RUST_BACKTRACE=full cargo test -- --nocapture
	
generate-proto: cargo-fmt
	cargo build --package weaver-proto

cargo-fmt:
	cargo fmt
