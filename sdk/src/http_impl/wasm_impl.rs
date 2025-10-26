//! WASM HTTP implementation using web-sys

use crate::client::ClientError;
use serde_json::Value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

pub async fn make_rpc_call(url: &str, payload: &Value) -> Result<String, ClientError> {
    let window = web_sys::window()
        .ok_or_else(|| ClientError::Network("No window object".to_string()))?;
    
    let init = init_request(payload);
    let fetch_promise = window.fetch_with_str_and_init(url, &init)
        .map_err(|e| ClientError::Network(format!("Fetch setup failed: {:?}", e)))?;
    
    let resp_value = JsFuture::from(fetch_promise)
        .await
        .map_err(|e| ClientError::Network(format!("Fetch failed: {:?}", e)))?;
    
    let resp: web_sys::Response = resp_value
        .dyn_into()
        .map_err(|e| ClientError::Network(format!("Not a Response: {:?}", e)))?;
    
    let text_promise = resp.text()
        .map_err(|e| ClientError::Network(format!("Could not get text: {:?}", e)))?;
    
    let text = JsFuture::from(text_promise)
        .await
        .map_err(|e| ClientError::Network(format!("Text future failed: {:?}", e)))?;
    
    Ok(text.as_string().ok_or_else(|| ClientError::Network("No text returned".to_string()))?)
}

fn init_request(payload: &Value) -> web_sys::RequestInit {
    let mut opts = web_sys::RequestInit::new();
    opts.method("POST");
    
    let headers = web_sys::Headers::new().unwrap();
    headers.set("Content-Type", "application/json").unwrap();
    opts.headers(&headers.into());
    
    let body_str = serde_json::to_string(payload).unwrap();
    opts.body(Some(&wasm_bindgen::JsValue::from_str(&body_str)));
    
    opts
}

