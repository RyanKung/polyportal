//! Contract interface for PolyEndpoint

use crate::abi::AbiEncoder;

/// Contract method IDs
pub mod method_id {
    use sha3::{Digest, Keccak256};

    pub fn add_endpoint() -> [u8; 4] {
        let hash = Keccak256::digest(b"addEndpoint(string)");
        [hash[0], hash[1], hash[2], hash[3]]
    }

    pub fn remove_endpoint() -> [u8; 4] {
        let hash = Keccak256::digest(b"removeEndpoint(string)");
        [hash[0], hash[1], hash[2], hash[3]]
    }

    pub fn add_admin() -> [u8; 4] {
        let hash = Keccak256::digest(b"addAdmin(address)");
        [hash[0], hash[1], hash[2], hash[3]]
    }

    pub fn remove_admin() -> [u8; 4] {
        let hash = Keccak256::digest(b"removeAdmin(address)");
        [hash[0], hash[1], hash[2], hash[3]]
    }

    pub fn get_endpoint_count() -> [u8; 4] {
        let hash = Keccak256::digest(b"getEndpointCount()");
        [hash[0], hash[1], hash[2], hash[3]]
    }

    pub fn get_all_endpoints() -> [u8; 4] {
        let hash = Keccak256::digest(b"getAllEndpoints()");
        [hash[0], hash[1], hash[2], hash[3]]
    }

    pub fn get_endpoint() -> [u8; 4] {
        let hash = Keccak256::digest(b"getEndpoint(uint256)");
        [hash[0], hash[1], hash[2], hash[3]]
    }

    pub fn has_endpoint() -> [u8; 4] {
        let hash = Keccak256::digest(b"hasEndpoint(string)");
        [hash[0], hash[1], hash[2], hash[3]]
    }

    pub fn transfer_ownership() -> [u8; 4] {
        let hash = Keccak256::digest(b"transferOwnership(address)");
        [hash[0], hash[1], hash[2], hash[3]]
    }

    pub fn owner() -> [u8; 4] {
        let hash = Keccak256::digest(b"owner()");
        [hash[0], hash[1], hash[2], hash[3]]
    }

    pub fn admins() -> [u8; 4] {
        let hash = Keccak256::digest(b"admins(address)");
        [hash[0], hash[1], hash[2], hash[3]]
    }
}

/// Transaction data builder
#[derive(Clone, Debug)]
pub struct TransactionData {
    pub method_id: Vec<u8>,
    pub data: Vec<u8>,
}

impl TransactionData {
    pub fn new() -> Self {
        Self {
            method_id: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn with_method(method_id: [u8; 4]) -> Self {
        Self {
            method_id: method_id.to_vec(),
            data: Vec::new(),
        }
    }

    pub fn encode_with_data(&mut self, data: Vec<u8>) -> &Self {
        self.data = data;
        self
    }

    pub fn build(&self) -> Vec<u8> {
        let mut result = self.method_id.clone();
        result.extend_from_slice(&self.data);
        result
    }

    pub fn to_hex(&self) -> String {
        let bytes = self.build();
        format!("0x{}", hex::encode(bytes))
    }
}

impl Default for TransactionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for addEndpoint(string)
pub fn encode_add_endpoint(endpoint: &str) -> TransactionData {
    let method_id = method_id::add_endpoint();
    let encoded = AbiEncoder::encode_string(endpoint);
    TransactionData {
        method_id: method_id.to_vec(),
        data: encoded,
    }
}

/// Builder for removeEndpoint(string)
pub fn encode_remove_endpoint(endpoint: &str) -> TransactionData {
    let method_id = method_id::remove_endpoint();
    let encoded = AbiEncoder::encode_string(endpoint);
    TransactionData {
        method_id: method_id.to_vec(),
        data: encoded,
    }
}

/// Builder for addAdmin(address)
pub fn encode_add_admin(admin: &str) -> Result<TransactionData, String> {
    let method_id = method_id::add_admin();
    let encoded = AbiEncoder::encode_address(admin)?;
    Ok(TransactionData {
        method_id: method_id.to_vec(),
        data: encoded,
    })
}

/// Builder for removeAdmin(address)
pub fn encode_remove_admin(admin: &str) -> Result<TransactionData, String> {
    let method_id = method_id::remove_admin();
    let encoded = AbiEncoder::encode_address(admin)?;
    Ok(TransactionData {
        method_id: method_id.to_vec(),
        data: encoded,
    })
}

/// Builder for getEndpointCount()
pub fn encode_get_endpoint_count() -> TransactionData {
    let method_id = method_id::get_endpoint_count();
    TransactionData {
        method_id: method_id.to_vec(),
        data: Vec::new(),
    }
}

/// Builder for getAllEndpoints()
pub fn encode_get_all_endpoints() -> TransactionData {
    let method_id = method_id::get_all_endpoints();
    TransactionData {
        method_id: method_id.to_vec(),
        data: Vec::new(),
    }
}

/// Builder for getEndpoint(uint256)
pub fn encode_get_endpoint(index: u64) -> TransactionData {
    let method_id = method_id::get_endpoint();
    let encoded = AbiEncoder::encode_uint256(index);
    TransactionData {
        method_id: method_id.to_vec(),
        data: encoded.to_vec(),
    }
}

/// Builder for hasEndpoint(string)
pub fn encode_has_endpoint(endpoint: &str) -> TransactionData {
    let method_id = method_id::has_endpoint();
    let encoded = AbiEncoder::encode_string(endpoint);
    TransactionData {
        method_id: method_id.to_vec(),
        data: encoded,
    }
}

/// Builder for transferOwnership(address)
pub fn encode_transfer_ownership(new_owner: &str) -> Result<TransactionData, String> {
    let method_id = method_id::transfer_ownership();
    let encoded = AbiEncoder::encode_address(new_owner)?;
    Ok(TransactionData {
        method_id: method_id.to_vec(),
        data: encoded,
    })
}

/// Builder for owner()
pub fn encode_owner() -> TransactionData {
    let method_id = method_id::owner();
    TransactionData {
        method_id: method_id.to_vec(),
        data: Vec::new(),
    }
}

/// Builder for admins(address)
pub fn encode_admins(admin: &str) -> Result<TransactionData, String> {
    let method_id = method_id::admins();
    let encoded = AbiEncoder::encode_address(admin)?;
    Ok(TransactionData {
        method_id: method_id.to_vec(),
        data: encoded,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_add_endpoint() {
        let tx = encode_add_endpoint("https://example.com");
        let hex = tx.to_hex();
        assert!(hex.starts_with("0x"));
        // The hex string should be at least as long as method id (4 bytes = 8 hex chars)
        assert!(hex.len() > 10);
    }

    #[test]
    fn test_encode_get_endpoint_count() {
        let tx = encode_get_endpoint_count();
        let hex = tx.to_hex();
        assert!(hex.starts_with("0x"));
        assert_eq!(hex.len(), 10); // 2 (0x) + 8 (method id)
    }
}

