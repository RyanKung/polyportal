//! Integration tests for the SDK
//! Run with: cargo test --test integration_test -- --nocapture

#![cfg(not(target_arch = "wasm32"))]

use polyendpoint_sdk::PolyEndpointClient;

/// Test Base Sepolia contract
#[tokio::test]
async fn test_get_endpoints_base_sepolia() {
    let contract = "0xf16e03526d1be6d120cfbf5a24e1ac78a8192663";
    let client = PolyEndpointClient::new(contract);
    
    println!("Testing contract: {}", contract);
    
    let endpoints = client.get_endpoints("base-sepolia").await.expect("Failed to get endpoints");
    
    println!("Retrieved {} endpoints", endpoints.len());
    
    // Verify we got exactly 2 endpoints (no hardcoding of their content)
    assert_eq!(endpoints.len(), 2, "Expected 2 endpoints");
    
    // Verify each endpoint has both url and description
    for (i, endpoint) in endpoints.iter().enumerate() {
        println!("Endpoint {}: URL={}, Description={}", 
                 i+1, endpoint.url, endpoint.description);
        assert!(!endpoint.url.is_empty(), "Endpoint {} should have a URL", i+1);
        assert!(!endpoint.description.is_empty(), "Endpoint {} should have a description", i+1);
    }
}

