//! Simple HTTP-based client for PolyEndpoint
//! Works without ethers dependency

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone)]
pub struct PolyEndpointClient {
    contract_address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EndpointInfo {
    pub url: String,
    pub description: String,
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Network error: {0}")]
    Network(String),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Decode error: {0}")]
    Decode(String),
}

impl PolyEndpointClient {
    pub fn new(contract_address: impl Into<String>) -> Self {
        Self {
            contract_address: contract_address.into(),
        }
    }

    pub fn contract_address(&self) -> &str {
        &self.contract_address
    }

    pub async fn get_endpoints(&self, network: impl AsRef<str>) -> Result<Vec<EndpointInfo>, ClientError> {
        let network = network.as_ref();
        let rpc_url = get_rpc_url(network);
        
        // Compute method ID for getAllEndpoints() - use sha3 like CLI does
        let method_id = ethers::utils::keccak256("getAllEndpoints()")[0..4].to_vec();
        
        // Make RPC call
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_call",
            "params": [{
                "to": format!("{:#x}", parse_address(&self.contract_address)?),
                "data": format!("0x{}", hex::encode(&method_id))
            }, "latest"],
            "id": 1
        });

        // Call RPC
        #[cfg(not(target_arch = "wasm32"))]
        let response = {
            let client = reqwest::Client::new();
            let res: serde_json::Value = client
                .post(rpc_url)
                .json(&request)
                .send()
                .await
                .map_err(|e| ClientError::Network(format!("Request failed: {}", e)))?
                .json()
                .await
                .map_err(|e| ClientError::Network(format!("Parse failed: {}", e)))?;
            res.to_string()
        };

        #[cfg(target_arch = "wasm32")]
        let response = {
            use wasm_bindgen::JsCast;
            use wasm_bindgen_futures::JsFuture;
            let window = web_sys::window()
                .ok_or_else(|| ClientError::Network("No window".to_string()))?;
            
            let mut opts = web_sys::RequestInit::new();
            opts.set_method("POST");
            let headers = web_sys::Headers::new().unwrap();
            headers.set("Content-Type", "application/json").unwrap();
            opts.set_headers(&headers.into());
            
            let body = wasm_bindgen::JsValue::from_str(&request.to_string());
            opts.set_body(&body);
            
            let fetch_promise = window.fetch_with_str_and_init(rpc_url, &opts);
            
            let resp_value = JsFuture::from(fetch_promise).await
                .map_err(|e| ClientError::Network(format!("Fetch: {:?}", e)))?;
            
            let resp: web_sys::Response = resp_value.dyn_into()
                .map_err(|e| ClientError::Network(format!("Response: {:?}", e)))?;
            
            let text_promise = resp.text()
                .map_err(|e| ClientError::Network(format!("Text: {:?}", e)))?;
            
            let text = JsFuture::from(text_promise).await
                .map_err(|e| ClientError::Network(format!("Text future: {:?}", e)))?;
            
            text.as_string().ok_or_else(|| ClientError::Network("No text".to_string()))?
        };
        
        let endpoints = decode_endpoints_response(response)?;
        Ok(endpoints)
    }
}

fn parse_address(addr: &str) -> Result<ethers::types::Address, ClientError> {
    addr.parse()
        .map_err(|e| ClientError::Parse(format!("Invalid address: {}", e)))
}

fn get_rpc_url(network: &str) -> &str {
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
    
    if let Some(error) = json.get("error") {
        let error_msg = format!("RPC error: {}", error);
        return Err(ClientError::Network(error_msg));
    }
    
    let result = json.get("result")
        .and_then(|r| r.as_str())
        .ok_or_else(|| ClientError::Parse("No result in response".to_string()))?;
    
    let result_bytes = hex::decode(result.trim_start_matches("0x"))
        .map_err(|e| ClientError::Decode(format!("Hex decode: {}", e)))?;
    
    // Decode using ethers ABI decoder
    let tokens = ethers::abi::decode(&[
        ethers::abi::ParamType::Array(Box::new(ethers::abi::ParamType::String)),
        ethers::abi::ParamType::Array(Box::new(ethers::abi::ParamType::String))
    ], result_bytes.as_slice())
    .map_err(|e| ClientError::Decode(format!("ABI decode: {}", e)))?;
    
    if tokens.len() < 2 {
        return Err(ClientError::Decode("Invalid response format".to_string()));
    }
    
    let urls = if let Some(ethers::abi::Token::Array(arr)) = tokens.first() {
        arr.iter().filter_map(|token| {
            if let ethers::abi::Token::String(s) = token {
                Some(s.clone())
            } else {
                None
            }
        }).collect::<Vec<_>>()
    } else {
        vec![]
    };
    
    let descriptions = if let Some(ethers::abi::Token::Array(arr)) = tokens.get(1) {
        arr.iter().filter_map(|token| {
            if let ethers::abi::Token::String(s) = token {
                Some(s.clone())
            } else {
                None
            }
        }).collect::<Vec<_>>()
    } else {
        vec![]
    };
    
    let endpoints = urls
        .into_iter()
        .zip(descriptions.into_iter())
        .map(|(url, description)| EndpointInfo { url, description })
        .collect();
    
    Ok(endpoints)
}
