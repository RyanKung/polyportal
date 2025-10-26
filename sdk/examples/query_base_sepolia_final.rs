//! Query Base Sepolia contract using direct HTTP
//! Run with: cargo run --example query_base_sepolia_final
//! 
//! This example demonstrates:
//! 1. Querying endpoint count
//! 2. Verifying contract owner
//! 3. Contract: 0xdcc474b1f6aecbbe140803255155762dd7783e59
//! 4. Owner: 0x8A32CF9C9c8D57784be80C3fA77E508d09213FEB

use sha3::{Digest, Keccak256};

const CONTRACT: &str = "0xdcc474b1f6aecbbe140803255155762dd7783e59";
const OWNER: &str = "0x8a32cf9c9c8d57784be80c3fa77e508d09213feb";
const RPC_URL: &str = "https://sepolia.base.org";

fn method_hash(sig: &str) -> String {
    let hash = Keccak256::digest(sig.as_bytes());
    format!("{:02x}{:02x}{:02x}{:02x}", hash[0], hash[1], hash[2], hash[3])
}

fn make_rpc_call(method: &str, params: Vec<serde_json::Value>) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": method,
        "params": params
    });
    
    let response: serde_json::Value = client
        .post(RPC_URL)
        .json(&payload)
        .send()?
        .json()?;
    
    if let Some(result) = response.get("result").and_then(|r| r.as_str()) {
        Ok(result.to_string())
    } else if let Some(error) = response.get("error") {
        Err(format!("RPC error: {}", error).into())
    } else {
        Err("Invalid RPC response".into())
    }
}

fn hex_to_u64(hex: &str) -> u64 {
    let cleaned = hex.strip_prefix("0x").unwrap_or(hex);
    let bytes = hex::decode(cleaned).unwrap_or_default();
    
    if bytes.len() >= 8 {
        let mut num_bytes = [0u8; 8];
        let start = bytes.len().saturating_sub(8);
        num_bytes.copy_from_slice(&bytes[start..]);
        u64::from_be_bytes(num_bytes)
    } else {
        0
    }
}

fn hex_to_address(hex: &str) -> String {
    let cleaned = hex.strip_prefix("0x").unwrap_or(hex);
    // Remove leading zeros and format as address
    if cleaned.len() >= 40 {
        format!("0x{}", &cleaned[cleaned.len()-40..])
    } else {
        format!("0x{}", cleaned.trim_start_matches('0'))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("==========================================");
    println!("Base Sepolia Contract Query");
    println!("==========================================");
    println!("Contract: {}", CONTRACT);
    println!("Owner: {}", OWNER);
    println!("RPC: {}\n", RPC_URL);

    // 1. Query endpoint count
    println!("1. Getting endpoint count...");
    let method_id = method_hash("getEndpointCount()");
    let call_data = format!("0x{}", method_id);
    
    let params = vec![
        serde_json::json!({
            "to": CONTRACT,
            "data": call_data
        }),
        serde_json::json!("latest")
    ];
    
    match make_rpc_call("eth_call", params) {
        Ok(result) => {
            let count = hex_to_u64(&result);
            println!("✓ Endpoint count: {}\n", count);
            
            if count > 0 {
                println!("Contract has {} endpoints registered.\n", count);
            }
        }
        Err(e) => println!("✗ Failed: {}\n", e),
    }

    // 2. Query owner
    println!("2. Verifying owner...");
    let method_id = method_hash("owner()");
    let call_data = format!("0x{}", method_id);
    
    let params = vec![
        serde_json::json!({
            "to": CONTRACT,
            "data": call_data
        }),
        serde_json::json!("latest")
    ];
    
    match make_rpc_call("eth_call", params) {
        Ok(result) => {
            let owner_address = hex_to_address(&result);
            println!("✓ Contract owner: {}", owner_address);
            
            if owner_address.to_lowercase() == OWNER.to_lowercase() {
                println!("✓ Owner verified!\n");
            } else {
                println!("⚠ Owner mismatch\n");
            }
        }
        Err(e) => println!("✗ Failed: {}\n", e),
    }

    println!("✓ Query completed!");
    Ok(())
}

