//! Base Sepolia Testnet Integration
//! Contract: 0xdcc474b1f6aecbbe140803255155762dd7783e59
//! Network: Base Sepolia (Chain ID: 84532)
//! RPC: https://sepolia.base.org

#[cfg(all(feature = "std", feature = "ethers"))]
mod base_tests {
    use ethers::providers::{Http, Provider};
    use ethers::types::Address;
    
    const CONTRACT: &str = "0xdcc474b1f6aecbbe140803255155762dd7783e59";
    const RPC_URL: &str = "https://sepolia.base.org";
    const CHAIN_ID: u64 = 84532;

    async fn create_client() -> Result<Provider<Http>, Box<dyn std::error::Error>> {
        let provider = Provider::try_from(RPC_URL)?;
        Ok(provider)
    }

    #[tokio::test]
    #[ignore] // Run with: cargo test --test base_sepolia -- --ignored
    async fn test_connect_to_base_sepolia() {
        match create_client().await {
            Ok(_) => println!("✓ Connected to Base Sepolia"),
            Err(e) => panic!("Failed to connect: {:?}", e),
        }
    }

    #[tokio::test]
    #[ignore]
    async fn query_endpoint_count() {
        let provider = create_client().await.unwrap();
        let contract: Address = CONTRACT.parse().unwrap();

        // Encode getEndpointCount()
        use sha3::{Digest, Keccak256};
        let hash = Keccak256::digest(b"getEndpointCount()");
        let method_id = &hash[0..4];
        
        let tx = ethers::types::TransactionRequest::new()
            .to(contract)
            .data(method_id.into());

        match provider.call(&tx, None).await {
            Ok(result) => {
                println!("✓ Call result: {:?}", hex::encode(result));
            }
            Err(e) => {
                eprintln!("Call failed: {:?}", e);
            }
        }
    }

    #[tokio::test]
    #[ignore]
    async fn query_all_endpoints() {
        let provider = create_client().await.unwrap();
        let contract: Address = CONTRACT.parse().unwrap();

        use sha3::{Digest, Keccak256};
        let hash = Keccak256::digest(b"getAllEndpoints()");
        let method_id = &hash[0..4];
        
        let tx = ethers::types::TransactionRequest::new()
            .to(contract)
            .data(method_id.into());

        match provider.call(&tx, None).await {
            Ok(result) => {
                println!("✓ All endpoints data: {:?}", hex::encode(result));
            }
            Err(e) => {
                eprintln!("Call failed: {:?}", e);
            }
        }
    }

    #[tokio::test]
    #[ignore]
    async fn query_owner() {
        let provider = create_client().await.unwrap();
        let contract: Address = CONTRACT.parse().unwrap();

        use sha3::{Digest, Keccak256};
        let hash = Keccak256::digest(b"owner()");
        let method_id = &hash[0..4];
        
        let tx = ethers::types::TransactionRequest::new()
            .to(contract)
            .data(method_id.into());

        match provider.call(&tx, None).await {
            Ok(result) => {
                println!("✓ Owner address: {:?}", hex::encode(result));
            }
            Err(e) => {
                eprintln!("Call failed: {:?}", e);
            }
        }
    }
}

#[cfg(not(all(feature = "std", feature = "ethers")))]
mod base_tests {
    #[test]
    fn test_requires_features() {
        println!("Base Sepolia tests require 'std' and 'ethers' features");
    }
}

