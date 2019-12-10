default:
	cargo build

test:
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test
	cargo fmt --all -- --check
