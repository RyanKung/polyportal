# PolyEndpoint SDK

WASM-compatible SDK for interacting with the PolyEndpoint smart contract.

## Features

- ✨ Native and WASM support  
- 🔧 ABI encoding for all contract methods
- 📦 Lightweight and dependency-minimal
- 🚀 Ready for browser and Node.js environments
- ⚡ Optional Ethers integration with feature flags
- 🎯 Comprehensive test coverage

## Usage

### Native Rust

```rust
use polyendpoint_sdk::contract::*;

// Encode addEndpoint transaction
let tx_data = encode_add_endpoint("https://example.com");
let hex = tx_data.to_hex();
println!("Transaction data: {}", hex);

// Encode addAdmin transaction
let tx_data = encode_add_admin("0x1234567890abcdef1234567890abcdef12345678")?;
let hex = tx_data.to_hex();
println!("Transaction data: {}", hex);

// Read-only calls
let call_data = encode_get_endpoint_count().to_hex();
let endpoint = encode_get_endpoint(0).to_hex();
let has_endpoint = encode_has_endpoint("https://example.com").to_hex();
```

### WASM/JavaScript

```javascript
import init, { PolyEndpointSdk } from './pkg/polyendpoint_sdk.js';

await init();
const sdk = new PolyEndpointSdk();

// Encode addEndpoint transaction
const txData = sdk.add_endpoint("https://example.com");
console.log("Transaction data:", txData);

// Encode addAdmin transaction
const adminTx = sdk.add_admin("0x1234567890abcdef1234567890abcdef12345678");
console.log("Transaction data:", adminTx);

// Read-only calls
const countCall = sdk.get_endpoint_count();
const endpointCall = sdk.get_endpoint(0);
const hasEndpoint = sdk.has_endpoint("https://example.com");
```

## Available Methods

### Write Operations (Transactions)

- `add_endpoint(endpoint: &str)` - Add an endpoint
- `remove_endpoint(endpoint: &str)` - Remove an endpoint
- `add_admin(admin: &str)` - Add an admin (owner only)
- `remove_admin(admin: &str)` - Remove an admin (owner only)
- `transfer_ownership(new_owner: &str)` - Transfer ownership (owner only)

### Read Operations (View functions)

- `get_endpoint_count()` - Get total endpoint count
- `get_all_endpoints()` - Get all endpoints
- `get_endpoint(index: u64)` - Get endpoint by index
- `has_endpoint(endpoint: &str)` - Check if endpoint exists
- `owner()` - Get contract owner
- `admins(admin: &str)` - Check admin status

## Building

### Feature Flags

The SDK supports multiple feature combinations:

- `std` (default): Standard library support
- `ethers`: Enable Ethers integration for RPC calls  
- `wasm32`: Automatically enabled when targeting WASM

### Native

```bash
# Basic build
cargo build

# With Ethers support
cargo build --features ethers

# Without std (minimal)
cargo build --no-default-features
```

### WASM

```bash
# Build for WASM
cargo build --target wasm32-unknown-unknown

# Using wasm-pack (recommended)
wasm-pack build --target web
```

### Both Targets

```bash
# Check both targets
make check

# Or manually
cargo check
cargo check --target wasm32-unknown-unknown
```

### Examples

```bash
# Basic encoding example
cargo run --example basic

# Client example (requires ethers feature)
cargo run --example client --features ethers
```

## Architecture

The SDK is modular with feature-gated components:

1. **Core** (`src/abi.rs`, `src/contract.rs`) - Always available
   - ABI encoding utilities
   - Contract method encoders
   - Transaction data builders

2. **Client** (`src/client.rs`) - Available with `ethers` feature
   - RPC client for native environments
   - Read/write operations via Ethers

3. **WASM** (`src/wasm.rs`) - Automatically included for WASM targets
   - JavaScript bindings via `wasm-bindgen`
   - Browser-compatible interface

## Testing

```bash
# Run tests
cargo test

# Run WASM tests
wasm-pack test --headless --chrome
```

## License

MIT

