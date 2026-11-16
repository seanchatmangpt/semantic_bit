test:
    cargo test

test-full:
    cargo test --all-features

polish:
    cargo fmt --all && cargo clippy -- -D warnings

build:
    cargo build

build-release:
    cargo build --release

doc:
    cargo doc --no-deps

clean:
    cargo clean

ci: polish test-full
