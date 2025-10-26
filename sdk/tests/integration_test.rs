//! Integration tests for deployed contract on Base Sepolia
//! Run with: cargo test --test integration_test

#[cfg(all(feature = "std", feature = "ethers"))]
mod tests {
    use polyendpoint_sdk::client::{create_client, PolyEndpointClient};
    use ethers::providers::{Http, Middleware, Provider};
    use ethers::types::Address;

    const CONTRACT_ADDRESS: &str = "0xdcc474b1f6aecbbe140803255155762dd7783e59";
    const BASE_SEPOLIA_RPC: &str = "https://sepolia.base.org";

    fn get_contract_address() -> Address {
        CONTRACT_ADDRESS.parse().expect("Invalid contract address")
    }

    fn get_rpc_url() -> &'static str {
        BASE_SEPOLIA_RPC
    }

    #[tokio::test]
    #[ignore] // Ignore by default, run with: cargo test --test integration_test -- --ignored
    async fn test_get_endpoint_count() {
        let client = create_client(get_rpc_url(), get_contract_address()).await;
        
        match client.get_endpoint_count().await {
            Ok(count) => {
                println!("✓ Endpoint count: {}", count);
                assert!(count >= 0);
            }
            Err(e) => {
                eprintln!("Failed to get endpoint count: {:?}", e);
                // Don't fail the test if RPC is not available
            }
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_all_endpoints() {
        let client = create_client(get_rpc_url(), get_contract_address()).await;
        
        match client.get_all_endpoints().await {
            Ok(endpoints) => {
                println!("✓ Found {} endpoints:", endpoints.len());
                for (i, endpoint) in endpoints.iter().enumerate() {
                    println!("  {}: {}", i, endpoint);
                }
            }
            Err(e) => {
                eprintln!("Failed to get all endpoints: {:?}", e);
            }
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_endpoint_by_index() {
        let client = create_client(get_rpc_url(), get_contract_address()).await;
        
        // Try to get first endpoint
        match client.get_endpoint(0).await {
            Ok(endpoint) => {
                println!("✓ First endpoint: {}", endpoint);
            }
            Err(e) => {
                eprintln!("Failed to get endpoint at index 0: {:?}", e);
            }
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_owner() {
        let client = create_client(get_rpc_url(), get_contract_address()).await;
        
        match client.owner().await {
            Ok(owner) => {
                println!("✓ Contract owner: {:#x}", owner);
            }
            Err(e) => {
                eprintln!("Failed to get owner: {:?}", e);
            }
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_encode_functions() {
        use polyendpoint_sdk::contract::*;

        println!("Testing ABI encoding:");
        
        let tx = encode_add_endpoint("https://api.example.com");
        println!("✓ addEndpoint: {}", tx.to_hex());
        assert!(tx.to_hex().starts_with("0x"));

        let tx = encode_get_endpoint_count();
        println!("✓ getEndpointCount: {}", tx.to_hex());
        assert_eq!(tx.to_hex().len(), 10); // 0x + 8 bytes

        let tx = encode_get_all_endpoints();
        println!("✓ getAllEndpoints: {}", tx.to_hex());

        let tx = encode_get_endpoint(5);
        println!("✓ getEndpoint(5): {}", tx.to_hex());

        let tx = encode_has_endpoint("https://api.example.com");
        println!("✓ hasEndpoint: {}", tx.to_hex());
        assert!(tx.to_hex().len() > 10);

        let admin = "0x1234567890abcdef1234567890abcdef12345678";
        if let Ok(tx) = encode_add_admin(admin) {
            println!("✓ addAdmin: {}", tx.to_hex());
        }

        if let Ok(tx) = encode_owner() {
            println!("✓ owner: {}", tx.to_hex());
        }
    }
}

#[cfg(not(all(feature = "std", feature = "ethers")))]
mod tests {
    #[test]
    fn test_requires_features() {
        println!("Integration tests require 'std' and 'ethers' features");
        println!("Run with: cargo test --test integration_test --features ethers");
    }
}

