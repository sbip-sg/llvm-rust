.PHONY: all debug release clean format linting test
.DEFAULT_GOAL := all

all: debug

# Build all targets in debug mode.
debug:
	cargo +stable build
	@echo "Output files were compiled to the folder: target/debug"

# Build all targets in release mode
release:
	cargo +stable build --release
	@echo "Output files were compiled to the folder: target/release"

# Format code
format:
	cargo +stable fmt

# Check source code linting rules
linting: format
	cargo +stable clippy --tests --benches --features linting

# Run unit tests
test:
	cargo +stable test --workspace --features linting

# Clean code
clean:
	cargo +stable clean
