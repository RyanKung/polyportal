//! PolyEndpoint Client
//! 
//! Provides a unified interface for native and WASM environments

use serde::{Deserialize, Serialize};

// Import the HTTP implementation based on target
#[cfg(target_arch = "wasm32")]
use crate::http_impl::wasm::make_rpc_call;
#[cfg(not(target_arch = "wasm32"))]
use crate::http_impl::native::make_rpc_call;

#[derive(Clone)]
pub struct PolyEndpointClient {
    contract_address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EndpointInfo {
    pub url: String,
    pub description: String,
}

impl PolyEndpointClient {
    /// Create a new SDK instance with a contract address
    pub fn new(contract_address: impl Into<String>) -> Self {
        Self {
            contract_address: contract_address.into(),
        }
    }

    /// Get the contract address
    pub fn contract_address(&self) -> &str {
        &self.contract_address
    }

    /// Fetch all endpoints from the contract
    /// 
    /// # Arguments
    /// 
    /// * `network` - Network name ("mainnet", "sepolia", "polygon", "arbitrum") or custom RPC URL
    /// 
    /// # Returns
    /// 
    /// A vector of endpoint information containing URLs and descriptions
    pub async fn get_endpoints(&self, network: impl AsRef<str>) -> Result<Vec<EndpointInfo>, ClientError> {
        let network = network.as_ref();
        let rpc_url = get_rpc_url_impl(network);
        
        // Encode function selector: getAllEndpoints()
        let method_id = "0x36346628"; // keccak256("getAllEndpoints()")[0:4]
        
        // Make RPC call
        let payload = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_call",
            "params": [{
                "to": self.contract_address,
                "data": method_id
            }, "latest"]
        });

        let response = make_rpc_call(rpc_url, &payload).await?;
        let endpoints = decode_endpoints_response(response)?;
        
        Ok(endpoints)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Decode error: {0}")]
    Decode(String),
    
    #[error("Invalid contract address")]
    InvalidAddress,
}


fn get_rpc_url_impl(network: &str) -> &str {
    match network.to_lowercase().as_str() {
        "mainnet" => "https://eth.llamarpc.com",
        "sepolia" => "https://rpc.sepolia.org",
        "base" | "base-mainnet" => "https://mainnet.base.org",
        "base-sepolia" | "base-testnet" => "https://sepolia.base.org",
        "polygon" => "https://polygon-rpc.com",
        "arbitrum" => "https://arb1.arbitrum.io/rpc",
        _ => network,
    }
}

fn decode_endpoints_response(response: String) -> Result<Vec<EndpointInfo>, ClientError> {
    let json: serde_json::Value = serde_json::from_str(&response)
        .map_err(|e| ClientError::Parse(format!("Parse error: {}", e)))?;
    
    // Check for error in response
    if let Some(error) = json.get("error") {
        // If error is "execution reverted", it means the contract call failed
        // This could mean no endpoints are registered or the contract doesn't implement the method
        let error_code = error.get("code").and_then(|c| c.as_u64()).unwrap_or(0);
        let error_msg = error.get("message").and_then(|m| m.as_str()).unwrap_or("Unknown error");
        
        if error_code == 3 && error_msg.contains("execution reverted") {
            // Contract reverted - likely no endpoints or invalid method
            return Ok(Vec::new());
        }
        
        let error_msg = format!("RPC error {}: {}", error_code, error_msg);
        return Err(ClientError::Network(error_msg));
    }
    
    let result = json.get("result")
        .and_then(|r| r.as_str())
        .ok_or_else(|| ClientError::Parse("No result in response".to_string()))?;
    
    let bytes = hex::decode(&result[2..])
        .map_err(|e| ClientError::Decode(format!("Hex decode error: {}", e)))?;
    
    let endpoints = decode_abi_string_array(&bytes)
        .map_err(|e| ClientError::Decode(format!("Decode error: {}", e)))?;
    
    Ok(endpoints)
}

fn decode_abi_string_array(bytes: &[u8]) -> Result<Vec<EndpointInfo>, Box<dyn std::error::Error>> {
    if bytes.len() < 64 {
        return Ok(vec![]);
    }
    
    let urls_offset = parse_u256(&bytes[0..32])? as usize;
    let descs_offset = parse_u256(&bytes[32..64])? as usize;
    
    let urls = decode_abi_string_array_inner(bytes, urls_offset)?;
    let descriptions = decode_abi_string_array_inner(bytes, descs_offset)?;
    
    let endpoints = urls
        .into_iter()
        .zip(descriptions.into_iter())
        .map(|(url, description)| EndpointInfo { url, description })
        .collect();
    
    Ok(endpoints)
}

fn decode_abi_string_array_inner(bytes: &[u8], offset: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    if bytes.len() < offset + 32 {
        return Ok(vec![]);
    }
    
    let len = parse_u256(&bytes[offset..offset+32])? as usize;
    let mut strings = Vec::new();
    let mut pos = offset + 32;
    
    for _ in 0..len {
        if bytes.len() < pos + 32 {
            break;
        }
        let string_offset = parse_u256(&bytes[pos..pos+32])? as usize;
        
        if bytes.len() >= string_offset + 32 {
            let string_len = parse_u256(&bytes[string_offset..string_offset+32])? as usize;
            if bytes.len() >= string_offset + 32 + string_len {
                let string_bytes = &bytes[string_offset + 32..string_offset + 32 + string_len];
                if let Ok(s) = String::from_utf8(string_bytes.to_vec()) {
                    strings.push(s);
                }
            }
        }
        
        pos += 32;
    }
    
    Ok(strings)
}

fn parse_u256(bytes: &[u8]) -> Result<u64, Box<dyn std::error::Error>> {
    if bytes.len() < 32 {
        return Err("Not enough bytes".into());
    }
    let mut arr = [0u8; 8];
    arr.copy_from_slice(&bytes[24..32]);
    Ok(u64::from_be_bytes(arr))
}

