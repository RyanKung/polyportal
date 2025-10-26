#!/bin/bash
# Integration test script for PolyEndpoint SDK
# Tests against real contract on Base Sepolia network

set -e

echo "🔧 Building PolyEndpoint SDK..."
cargo build --release

echo ""
echo "🧪 Running local unit tests..."
cargo test --lib

echo ""
echo "🧪 Running integration tests (requires network access)..."
echo "   Contract: 0xf16e03526d1be6d120cfbf5a24e1ac78a8192663"
echo "   Network: Base Sepolia (https://sepolia.base.org)"
echo ""

# Run integration tests with --ignored flag
cargo test --test integration_test -- --ignored --nocapture

echo ""
echo "✅ Integration tests completed!"
echo ""
echo "To test manually:"
echo "  cd examples && cargo run --example basic"

