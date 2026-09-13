check:
    cargo check
    cargo clippy
    cargo fmt --check

release-build: check
    cargo build --release

release-test:
    cargo test --release

release: release-build release-test

run $RUST_LOG="info": 
    cargo run
