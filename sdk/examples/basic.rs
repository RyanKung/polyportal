//! Basic usage example for native target
#![cfg(not(target_arch = "wasm32"))]

use polyendpoint_sdk::PolyEndpointClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Replace with actual contract address
    let contract_address = "0x1234567890123456789012345678901234567890";
    
    let client = PolyEndpointClient::new(contract_address);
    
    println!("Fetching endpoints from contract: {}", client.contract_address());
    
    let endpoints = client.get_endpoints("mainnet").await?;
    
    println!("Found {} endpoints:", endpoints.len());
    
    for (i, endpoint) in endpoints.iter().enumerate() {
        println!("\nEndpoint {}:", i + 1);
        println!("  URL: {}", endpoint.url);
        println!("  Description: {}", endpoint.description);
    }
    
    Ok(())
}

