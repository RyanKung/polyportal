.PHONY: help build test clean install release clippy fmt run deploy import-key

help:
	@echo "PolyPortal - Makefile Commands"
	@echo ""
	@echo "Building:"
	@echo "  make build           - Build in debug mode"
	@echo "  make release         - Build in release mode"
	@echo "  make install         - Install CLI tool"
	@echo ""
	@echo "Testing:"
	@echo "  make test            - Run Rust tests"
	@echo "  make test-sol        - Run Solidity tests (Hardhat)"
	@echo ""
	@echo "Code Quality:"
	@echo "  make clippy          - Run clippy linter"
	@echo "  make fmt             - Format code"
	@echo ""
	@echo "CLI Operations:"
	@echo "  make import-key      - Import private key"
	@echo "  make deploy          - Deploy contract"
	@echo ""
	@echo "Cleaning:"
	@echo "  make clean           - Clean build artifacts"
	@echo ""

build:
	@echo "Building PolyPortal CLI (debug)..."
	cd cli && cargo build

release:
	@echo "Building PolyPortal CLI (release)..."
	cd cli && cargo build --release

install:
	@echo "Installing PolyPortal CLI..."
	cd cli && cargo install --path . --force

test:
	@echo "Running Rust tests..."
	cd cli && cargo test

test-sol:
	@echo "Running Solidity tests..."
	npm run test

test-forge:
	@echo "Running Foundry tests..."
	forge test

clippy:
	@echo "Running clippy..."
	cd cli && cargo clippy -- -D warnings

fmt:
	@echo "Formatting code..."
	cd cli && cargo fmt

import-key:
	@echo "Importing private key..."
	cd cli && cargo run -- import-key

deploy:
	@echo "Deploying contract..."
	cd cli && cargo run -- deploy

clean:
	@echo "Cleaning build artifacts..."
	cd cli && cargo clean
	rm -rf artifacts cache
