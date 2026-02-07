check:
	cargo fmt --all --check --quiet
	cargo clippy --all-targets --quiet
	cargo check --all-targets --quiet
	cargo build --all-targets --quiet
	cargo test --quiet -- --format=terse
