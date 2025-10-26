# PolyEndpoint SDK

A simple Rust SDK for fetching endpoint lists from PolyEndpoint smart contract. Works seamlessly in both native and WASM environments.

## Features

- ✅ Simple API - just provide a contract address
- ✅ Works in both native and WASM environments  
- ✅ Automatic network detection (mainnet, sepolia, polygon, arbitrum)
- ✅ Async/await support
- ✅ No external dependencies in WASM target

## Usage

### Basic Example

```rust
use polyendpoint_sdk::PolyEndpointClient;

#[tokio::main]
async fn main() {
    let client = PolyEndpointClient::new("0x1234...");
    
    let endpoints = client.get_endpoints("mainnet").await.unwrap();
    
    for endpoint in endpoints {
        println!("URL: {}", endpoint.url);
        println!("Description: {}", endpoint.description);
    }
}
```

### In Yew (WASM)

```rust
use polyendpoint_sdk::PolyEndpointClient;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let endpoints = use_state(Vec::new);
    
    {
        let endpoints = endpoints.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let client = PolyEndpointClient::new("0x1234...");
            let result = client.get_endpoints("mainnet").await.unwrap();
            endpoints.set(result);
        });
    }
    
    html! {
        <div>
            {for endpoints.iter().map(|ep| html! {
                <div>
                    <p>{&ep.url}</p>
                    <p>{&ep.description}</p>
                </div>
            })}
        </div>
    }
}
```

## Networks

The SDK supports these networks by name:

- `"mainnet"` - Ethereum mainnet
- `"sepolia"` - Sepolia testnet  
- `"base-mainnet"` or `"base"` - Base mainnet
- `"base-sepolia"` or `"base-testnet"` - Base Sepolia testnet
- `"polygon"` - Polygon mainnet
- `"arbitrum"` - Arbitrum One

Or provide a custom RPC URL directly.

## Building

### For Native Target

```bash
cargo build --release
```

### For WASM Target

```bash
cargo build --release --target wasm32-unknown-unknown
```

## API

### `PolyEndpointClient`

- `new(address)` - Create a new client instance
- `get_endpoints(network)` - Fetch all endpoints from the contract

### `EndpointInfo`

- `url` - The endpoint URL
- `description` - The endpoint description

## License

MIT

