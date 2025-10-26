//! Test against Base Sepolia contract
//! Contract: 0xf16e03526d1be6d120cfbf5a24e1ac78a8192663
#![cfg(not(target_arch = "wasm32"))]

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
    
    println!("🎉 Test completed successfully!");
    
    Ok(())
}

