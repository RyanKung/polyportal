.PHONY: help build-native build-wasm build-wasm-pack clean test check examples

help:
	@echo "PolyEndpoint SDK - Build Commands"
	@echo ""
	@echo "Building:"
	@echo "  make build-native   - Build native library (std)"
	@echo "  make build-wasm     - Build WASM library"
	@echo "  make build-wasm-pack - Build with wasm-pack (recommended)"
	@echo ""
	@echo "Checking:"
	@echo "  make check          - Check code for both targets"
	@echo "  make test           - Run tests"
	@echo "  make examples       - Run examples"
	@echo ""
	@echo "Cleaning:"
	@echo "  make clean          - Clean build artifacts"

build-native:
	@echo "Building native library..."
	cargo build --release

build-wasm:
	@echo "Building WASM library..."
	cargo build --target wasm32-unknown-unknown --release

build-wasm-pack:
	@echo "Building with wasm-pack..."
	wasm-pack build --target web

check:
	@echo "Checking native target..."
	cargo check
	@echo "Checking WASM target..."
	cargo check --target wasm32-unknown-unknown
	@echo "✓ All checks passed!"

test:
	@echo "Running tests..."
	cargo test
	@echo "✓ Tests passed!"

examples:
	@echo "Running basic example..."
	cargo run --example basic

clean:
	@echo "Cleaning build artifacts..."
	cargo clean

