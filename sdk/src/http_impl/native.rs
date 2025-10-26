//! Native HTTP implementation using reqwest

use crate::simple_client::ClientError;
use serde_json::Value;

pub async fn make_rpc_call(url: &str, payload: &Value) -> Result<String, ClientError> {
    let client = reqwest::Client::new();
    
    let body_str = serde_json::to_string(payload)
        .map_err(|e| ClientError::Network(format!("Serialize error: {}", e)))?;
    
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(body_str)
        .send()
        .await
        .map_err(|e| ClientError::Network(format!("Request failed: {}", e)))?;
    
    let text = response
        .text()
        .await
        .map_err(|e| ClientError::Network(format!("Failed to read response: {}", e)))?;
    
    Ok(text)
}

