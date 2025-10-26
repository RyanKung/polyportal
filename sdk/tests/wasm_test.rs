//! WASM tests for PolyEndpoint SDK
//! These tests require compilation to WASM target
//! Run with: wasm-pack test --firefox --headless

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use polyendpoint_sdk::PolyEndpointClient;

// Base testnet contract address
const BASE_SEPOLIA_CONTRACT: &str = "0xf16e03526d1be6d120cfbf5a24e1ac78a8192663";

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_wasm_base_sepolia() {
    let client = PolyEndpointClient::new(BASE_SEPOLIA_CONTRACT);
    
    // Test with network name
    let result = client.get_endpoints("base-sepolia").await;
    
    assert!(result.is_ok(), "Failed to fetch from base-sepolia: {:?}", result);
    
    if let Ok(endpoints) = result {
        web_sys::console::log_1(&format!("Found {} endpoints on Base Sepolia", endpoints.len()).into());
        
        // Verify we got exactly 2 endpoints (no hardcoding of their content)
        assert_eq!(endpoints.len(), 2, "Expected 2 endpoints");
        
        // Verify each endpoint has both url and description
        for (i, endpoint) in endpoints.iter().enumerate() {
            web_sys::console::log_1(&format!("Endpoint {}: URL={}, Description={}", 
                i+1, endpoint.url, endpoint.description).into());
            assert!(!endpoint.url.is_empty(), "Endpoint {} should have a URL", i+1);
            assert!(!endpoint.description.is_empty(), "Endpoint {} should have a description", i+1);
        }
    }
}

#[wasm_bindgen_test]
async fn test_wasm_direct_rpc() {
    let client = PolyEndpointClient::new(BASE_SEPOLIA_CONTRACT);
    
    // Test with direct RPC URL
    let rpc_url = "https://sepolia.base.org";
    let result = client.get_endpoints(rpc_url).await;
    
    assert!(result.is_ok(), "Failed to fetch from direct RPC: {:?}", result);
    
    if let Ok(endpoints) = result {
        web_sys::console::log_1(&format!("Found {} endpoints via direct RPC", endpoints.len()).into());
        
        // Verify we got exactly 2 endpoints (no hardcoding of their content)
        assert_eq!(endpoints.len(), 2, "Expected 2 endpoints");
    }
}

#[wasm_bindgen_test]
fn test_client_creation() {
    let client = PolyEndpointClient::new(BASE_SEPOLIA_CONTRACT);
    
    assert_eq!(client.contract_address(), BASE_SEPOLIA_CONTRACT);
    
    let client2 = PolyEndpointClient::new(format!("0x{}", "1234567890"));
    assert_eq!(client2.contract_address(), "0x1234567890");
}

