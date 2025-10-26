#!/bin/bash
# Test script for deployed contract on Base Sepolia
# Contract: 0xdcc474b1f6aecbbe140803255155762dd7783e59

set -e

CONTRACT="0xdcc474b1f6aecbbe140803255155762dd7783e59"
RPC_URL="https://sepolia.base.org"

echo "=========================================="
echo "Base Sepolia Contract Test"
echo "Contract: $CONTRACT"
echo "=========================================="
echo ""

echo "1. Testing contract connectivity..."
cast call $CONTRACT "getEndpointCount()" --rpc-url $RPC_URL
echo ""

echo "2. Getting owner..."
cast call $CONTRACT "owner()" --rpc-url $RPC_URL
echo ""

echo "3. Getting all endpoints (first attempt)..."
cast call $CONTRACT "getAllEndpoints()" --rpc-url $RPC_URL
echo ""

echo "✓ Tests completed!"

