default:
    @just --list

check:
    cargo check

clippy:
    cargo clippy -- -D warnings

fmt:
    cargo fmt

fmt-check:
    cargo fmt --check

test:
    cargo test

lint: check clippy fmt-check

build:
    cargo build --release

install:
    cargo install --path .

ci: lint test
