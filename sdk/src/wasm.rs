//! WASM bindings for PolyEndpoint SDK
//! This module provides JavaScript bindings for the SDK

#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use crate::contract::{
    encode_add_endpoint, encode_remove_endpoint, encode_add_admin, encode_remove_admin,
    encode_get_endpoint_count, encode_get_all_endpoints, encode_get_endpoint,
    encode_has_endpoint, encode_transfer_ownership, encode_owner, encode_admins,
};

/// SDK for interacting with PolyEndpoint contract
#[wasm_bindgen]
pub struct PolyEndpointSdk;

#[wasm_bindgen]
impl PolyEndpointSdk {
    /// Create a new SDK instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> PolyEndpointSdk {
        PolyEndpointSdk
    }

    /// Encode addEndpoint(string) transaction data
    /// 
    /// # Arguments
    /// * `endpoint` - The endpoint URL to add
    /// 
    /// # Returns
    /// Hex-encoded transaction data
    #[wasm_bindgen]
    pub fn add_endpoint(&self, endpoint: &str) -> String {
        encode_add_endpoint(endpoint).to_hex()
    }

    /// Encode removeEndpoint(string) transaction data
    /// 
    /// # Arguments
    /// * `endpoint` - The endpoint URL to remove
    /// 
    /// # Returns
    /// Hex-encoded transaction data
    #[wasm_bindgen]
    pub fn remove_endpoint(&self, endpoint: &str) -> String {
        encode_remove_endpoint(endpoint).to_hex()
    }

    /// Encode addAdmin(address) transaction data
    /// 
    /// # Arguments
    /// * `admin` - The admin address to add (0x-prefixed)
    /// 
    /// # Returns
    /// Hex-encoded transaction data, or error message
    #[wasm_bindgen]
    pub fn add_admin(&self, admin: &str) -> Result<String, String> {
        encode_add_admin(admin).map(|tx| tx.to_hex())
    }

    /// Encode removeAdmin(address) transaction data
    /// 
    /// # Arguments
    /// * `admin` - The admin address to remove (0x-prefixed)
    /// 
    /// # Returns
    /// Hex-encoded transaction data, or error message
    #[wasm_bindgen]
    pub fn remove_admin(&self, admin: &str) -> Result<String, String> {
        encode_remove_admin(admin).map(|tx| tx.to_hex())
    }

    /// Encode getEndpointCount() call data
    /// 
    /// # Returns
    /// Hex-encoded call data
    #[wasm_bindgen]
    pub fn get_endpoint_count(&self) -> String {
        encode_get_endpoint_count().to_hex()
    }

    /// Encode getAllEndpoints() call data
    /// 
    /// # Returns
    /// Hex-encoded call data
    #[wasm_bindgen]
    pub fn get_all_endpoints(&self) -> String {
        encode_get_all_endpoints().to_hex()
    }

    /// Encode getEndpoint(uint256) call data
    /// 
    /// # Arguments
    /// * `index` - The index of the endpoint
    /// 
    /// # Returns
    /// Hex-encoded call data
    #[wasm_bindgen]
    pub fn get_endpoint(&self, index: u32) -> String {
        encode_get_endpoint(index as u64).to_hex()
    }

    /// Encode hasEndpoint(string) call data
    /// 
    /// # Arguments
    /// * `endpoint` - The endpoint URL to check
    /// 
    /// # Returns
    /// Hex-encoded call data
    #[wasm_bindgen]
    pub fn has_endpoint(&self, endpoint: &str) -> String {
        encode_has_endpoint(endpoint).to_hex()
    }

    /// Encode transferOwnership(address) transaction data
    /// 
    /// # Arguments
    /// * `new_owner` - The new owner address (0x-prefixed)
    /// 
    /// # Returns
    /// Hex-encoded transaction data, or error message
    #[wasm_bindgen]
    pub fn transfer_ownership(&self, new_owner: &str) -> Result<String, String> {
        encode_transfer_ownership(new_owner).map(|tx| tx.to_hex())
    }

    /// Encode owner() call data
    /// 
    /// # Returns
    /// Hex-encoded call data
    #[wasm_bindgen]
    pub fn owner(&self) -> String {
        encode_owner().to_hex()
    }

    /// Encode admins(address) call data
    /// 
    /// # Arguments
    /// * `admin` - The admin address to check (0x-prefixed)
    /// 
    /// # Returns
    /// Hex-encoded call data, or error message
    #[wasm_bindgen]
    pub fn admins(&self, admin: &str) -> Result<String, String> {
        encode_admins(admin).map(|tx| tx.to_hex())
    }

    /// Compute method ID for a given function signature
    /// 
    /// # Arguments
    /// * `signature` - Function signature (e.g., "addEndpoint(string)")
    /// 
    /// # Returns
    /// First 4 bytes of keccak256 hash
    #[wasm_bindgen]
    pub fn method_id(&self, signature: &str) -> Vec<u8> {
        use sha3::{Digest, Keccak256};
        let hash = Keccak256::digest(signature.as_bytes());
        hash[..4].to_vec()
    }
}

// Export a default constructor
#[wasm_bindgen]
pub fn create_sdk() -> PolyEndpointSdk {
    PolyEndpointSdk::new()
}

