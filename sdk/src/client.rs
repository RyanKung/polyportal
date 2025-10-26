//! Ethers client for native RPC interactions
//! Only compiled when ethers feature is enabled

#![cfg(feature = "ethers")]

use crate::contract;
use ethers::{
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, Bytes, TxResponse},
};

#[derive(Clone)]
pub struct PolyEndpointClient<M: Middleware> {
    contract_address: Address,
    provider: M,
}

impl<M: Middleware> PolyEndpointClient<M> {
    pub fn new(contract_address: Address, provider: M) -> Self {
        Self {
            contract_address,
            provider,
        }
    }

    pub fn contract_address(&self) -> Address {
        self.contract_address
    }

    pub fn provider(&self) -> &M {
        &self.provider
    }
}

// Transaction methods
impl<M> PolyEndpointClient<M>
where
    M: Middleware,
    M::Error: 'static,
{
    /// Add an endpoint (requires admin or owner)
    pub async fn add_endpoint(&self, endpoint: &str) -> Result<TxResponse, M::Error> {
        let data = contract::encode_add_endpoint(endpoint).build();
        self.send_transaction(data).await
    }

    /// Remove an endpoint (requires admin or owner)
    pub async fn remove_endpoint(&self, endpoint: &str) -> Result<TxResponse, M::Error> {
        let data = contract::encode_remove_endpoint(endpoint).build();
        self.send_transaction(data).await
    }

    /// Add an admin (owner only)
    pub async fn add_admin(&self, admin: &str) -> Result<TxResponse, M::Error> {
        let admin_address: Address = admin.parse().unwrap();
        let data = self.encode_address_method("addAdmin(address)", admin_address);
        self.send_transaction(data).await
    }

    /// Remove an admin (owner only)
    pub async fn remove_admin(&self, admin: &str) -> Result<TxResponse, M::Error> {
        let admin_address: Address = admin.parse().unwrap();
        let data = self.encode_address_method("removeAdmin(address)", admin_address);
        self.send_transaction(data).await
    }

    /// Transfer ownership (owner only)
    pub async fn transfer_ownership(&self, new_owner: &str) -> Result<TxResponse, M::Error> {
        let new_owner_address: Address = new_owner.parse().unwrap();
        let data = self.encode_address_method("transferOwnership(address)", new_owner_address);
        self.send_transaction(data).await
    }

    async fn send_transaction(&self, data: Vec<u8>) -> Result<TxResponse, M::Error> {
        use ethers::types::TransactionRequest;
        let tx = TransactionRequest::new()
            .to(self.contract_address)
            .data(Bytes::from(data));
        self.provider.send_transaction(tx, None).await
    }
}

// View methods (read-only)
impl<M> PolyEndpointClient<M>
where
    M: Middleware,
    M::Error: 'static,
{
    /// Get total endpoint count
    pub async fn get_endpoint_count(&self) -> Result<u64, M::Error> {
        let data = contract::encode_get_endpoint_count().build();
        self.call_read(data).await
    }

    /// Get all endpoints
    pub async fn get_all_endpoints(&self) -> Result<Vec<String>, M::Error> {
        let data = contract::encode_get_all_endpoints().build();
        self.call_read(data).await
    }

    /// Get endpoint by index
    pub async fn get_endpoint(&self, index: u64) -> Result<String, M::Error> {
        let data = contract::encode_get_endpoint(index).build();
        self.call_read(data).await
    }

    /// Check if endpoint exists
    pub async fn has_endpoint(&self, endpoint: &str) -> Result<bool, M::Error> {
        let data = contract::encode_has_endpoint(endpoint).build();
        self.call_read(data).await
    }

    /// Get contract owner
    pub async fn owner(&self) -> Result<Address, M::Error> {
        let data = contract::encode_owner().build();
        self.call_read(data).await
    }

    /// Check if address is admin
    pub async fn admins(&self, admin: &str) -> Result<bool, M::Error> {
        let admin_address: Address = admin.parse().unwrap();
        let data = self.encode_address_method("admins(address)", admin_address);
        self.call_read(data).await
    }

    async fn call_read(&self, data: Vec<u8>) -> Result<Bytes, M::Error> {
        use ethers::types::TransactionRequest;
        self.provider.call(
            TransactionRequest::new()
                .to(self.contract_address)
                .data(Bytes::from(data)),
            None,
        ).await
    }
}

// Encoding helpers
impl<M: Middleware> PolyEndpointClient<M> {
    fn encode_address_method(&self, signature: &str, address: Address) -> Vec<u8> {
        use sha3::{Digest, Keccak256};
        let method_id = {
            let hash = Keccak256::digest(signature.as_bytes());
            [hash[0], hash[1], hash[2], hash[3]]
        };
        
        let mut data = method_id.to_vec();
        let mut address_bytes = vec![0u8; 32];
        address_bytes[12..].copy_from_slice(&address.to_fixed_bytes());
        data.extend_from_slice(&address_bytes);
        data
    }
}

/// Create a client with HTTP provider (native only)
#[cfg(feature = "std")]
pub async fn create_client<A: Into<Address>>(
    rpc_url: &str,
    contract_address: A,
) -> PolyEndpointClient<Provider<Http>> {
    let provider = Provider::try_from(rpc_url).expect("Failed to create provider");
    PolyEndpointClient::new(contract_address.into(), provider)
}
