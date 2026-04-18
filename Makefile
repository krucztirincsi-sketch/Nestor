.PHONY: build run test clean help

build:
	cargo build --release

run:
	cargo run -- demo.ai

test:
	cargo test

clean:
	cargo clean

help:
	@echo "Aion Compiler Makefile"
	@echo "  make build  - Build the compiler in release mode"
	@echo "  make run    - Run the compiler on demo.ai"
	@echo "  make test   - Run unit tests"
	@echo "  make clean  - Remove build artifacts"
