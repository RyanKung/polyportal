//! Client example with Ethers RPC calls
//! Run with: cargo run --example client --features ethers,std

#[cfg(all(feature = "ethers", feature = "std"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use polyendpoint_sdk::client::{create_client, PolyEndpointClient};
    use ethers::providers::Provider;
    use ethers::providers::Http;
    use ethers::types::Address;

    // Replace with your actual contract address
    let contract_address: Address = "0x0000000000000000000000000000000000000000".parse()?;
    let rpc_url = "http://localhost:8545";

    println!("=== PolyEndpoint SDK Client Example ===\n");
    println!("Connecting to: {}", rpc_url);
    println!("Contract: {}\n", contract_address);

    // Create client (commented out to avoid actual RPC call)
    // let client = create_client(rpc_url, contract_address).await;
    
    println!("Example usage:");
    println!("  let client = create_client(rpc_url, contract_address).await;");
    println!("  let count = client.get_endpoint_count().await?;");
    println!("  let endpoints = client.get_all_endpoints().await?;");
    println!("  client.add_endpoint(\"https://example.com\").await?;");
    
    Ok(())
}

#[cfg(not(all(feature = "ethers", feature = "std")))]
fn main() {
    println!("This example requires the 'ethers' and 'std' features. Run with:");
    println!("cargo run --example client --features ethers,std");
}

