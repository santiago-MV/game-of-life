build:
	cargo build
run:
	cargo run
clippy:
	cargo clippy
fmt:
	cargo fmt --check
test:
	cargo test
wasm:
	cargo build --target wasm32-unknown-unknown
	basic-http-server .
