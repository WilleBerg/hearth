alias b:= build

clean:
    cargo clean

build:
    cargo build

test:
    cargo test

format:
    cargo fmt 

release-build:
    cargo check
    cargo fmt
    cargo build --release

release-test:
    cargo test --release

release: release-build release-test

run $RUST_LOG="info": 
    cargo run
