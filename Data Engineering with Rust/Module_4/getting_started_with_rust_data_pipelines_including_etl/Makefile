# Makefile for Rust project

# Define the name of your project
# PROJECT_NAME = rust_sequences_and_maps

# Define the default target
.PHONY: all
all: format lint test build

# Format code using rustfmt
.PHONY: format
format:
	cargo fmt

# Lint code using Clippy
.PHONY: lint
lint:
	cargo clippy --all-targets --all-features -- -D warnings

# Run tests
.PHONY: test
test:
	cargo test

# Build the project
.PHONY: build
build:
	cargo build --release

# Clean build artifacts
.PHONY: clean
clean:
	cargo clean
