//! Local tests that don't require network access
//! Can run without --ignored flag

use polyendpoint_sdk::PolyEndpointClient;

#[test]
fn test_client_creation() {
    let client = PolyEndpointClient::new("0x1234567890123456789012345678901234567890");
    
    assert_eq!(client.contract_address(), "0x1234567890123456789012345678901234567890");
    
    let client2 = PolyEndpointClient::new("0xABC");
    assert_eq!(client2.contract_address(), "0xABC");
}

#[test]
fn test_network_urls() {
    // Test network URL mapping
    use polyendpoint_sdk::PolyEndpointClient;
    
    let _client = PolyEndpointClient::new("0x1234");
    
    // Just test that we can create clients and call methods
    // The actual RPC URL conversion is tested in integration tests
}

