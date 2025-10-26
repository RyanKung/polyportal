//! Native HTTP implementation using reqwest

use crate::client::ClientError;
use serde_json::Value;

pub async fn make_rpc_call(url: &str, payload: &Value) -> Result<String, ClientError> {
    let client = reqwest::Client::new();
    
    let response = client
        .post(url)
        .json(payload)
        .send()
        .await
        .map_err(|e| ClientError::Network(format!("Request failed: {}", e)))?;
    
    let text = response
        .text()
        .await
        .map_err(|e| ClientError::Network(format!("Failed to read response: {}", e)))?;
    
    Ok(text)
}

