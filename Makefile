.PHONY: clean all

all: format lint build test

build:
	cargo build

release:
	cargo build --release

format:
	cargo fmt

lint:
	cargo clippy

test:
	cargo test -- --test-threads=1

clean:
	cargo clean