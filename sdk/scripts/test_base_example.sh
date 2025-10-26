#!/bin/bash
# Quick test script for Base Sepolia contract

set -e

cat > /tmp/test_base.rs << 'EOF'
use polyendpoint_sdk::PolyEndpointClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contract_address = "0xf16e03526d1be6d120cfbf5a24e1ac78a8192663";
    
    println!("🔍 Fetching endpoints from Base Sepolia...");
    println!("Contract: {}", contract_address);
    println!("Network: base-sepolia");
    println!("");
    
    let client = PolyEndpointClient::new(contract_address);
    
    match client.get_endpoints("base-sepolia").await {
        Ok(endpoints) => {
            println!("✅ Found {} endpoints:", endpoints.len());
            println!("");
            
            for (i, endpoint) in endpoints.iter().enumerate() {
                println!("Endpoint {}:", i + 1);
                println!("  URL: {}", endpoint.url);
                println!("  Description: {}", endpoint.description);
                println!("");
            }
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            return Err(e.into());
        }
    }
    
    Ok(())
}
EOF

cd /Users/ryan/Dev/farcaster/polyendpoint/rust
rustc --edition 2021 -L target/release/deps --extern polyendpoint_sdk=target/release/libpolyendpoint_sdk.rlib --extern tokio --extern serde_json --extern hex --crate-type bin /tmp/test_base.rs -o /tmp/test_base 2>&1 || cat /tmp/test_base.rs | cargo run --bin -

run_terminal_cmd
