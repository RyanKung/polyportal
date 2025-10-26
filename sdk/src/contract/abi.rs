//! ABI encoding utilities

use sha3::{Digest, Keccak256};

pub struct AbiEncoder;

impl AbiEncoder {
    /// Encode a string parameter according to ABI encoding
    pub fn encode_string(s: &str) -> Vec<u8> {
        let offset = 0x20u64; // First 32 bytes for offset
        let length = s.len() as u64;
        let padded_length = (s.len() + 31) / 32 * 32;
        let data_length = padded_length + 64; // offset + length + data

        let mut result = Vec::new();

        // Offset to data (32 bytes)
        result.extend_from_slice(&encode_uint256(offset));

        // String length (32 bytes)
        result.extend_from_slice(&encode_uint256(length));

        // String data with padding
        let mut string_bytes = s.as_bytes().to_vec();
        string_bytes.resize(padded_length, 0);
        result.extend_from_slice(&string_bytes);

        result
    }

    /// Encode an address parameter
    pub fn encode_address(addr: &str) -> Result<Vec<u8>, String> {
        let addr = addr.strip_prefix("0x").unwrap_or(addr);
        let addr_bytes = hex::decode(addr)
            .map_err(|e| format!("Invalid address: {}", e))?;

        if addr_bytes.len() != 20 {
            return Err(format!("Address must be 20 bytes, got {}", addr_bytes.len()));
        }

        // Pad to 32 bytes
        let mut result = vec![0u8; 32];
        result[12..].copy_from_slice(&addr_bytes);

        Ok(result)
    }

    /// Encode a uint256 parameter
    pub fn encode_uint256(value: u64) -> [u8; 32] {
        let mut result = [0u8; 32];
        let bytes = value.to_be_bytes();
        result[24..].copy_from_slice(&bytes);
        result
    }

    /// Encode bytes32 parameter
    pub fn encode_bytes32(bytes: &[u8]) -> Vec<u8> {
        let mut result = vec![0u8; 32];
        let len = bytes.len().min(32);
        result[..len].copy_from_slice(&bytes[..len]);
        result
    }

    /// Compute keccak256 hash
    pub fn keccak256(data: &[u8]) -> Vec<u8> {
        Keccak256::digest(data).to_vec()
    }

    /// Compute function selector (first 4 bytes of keccak256)
    pub fn function_selector(signature: &str) -> [u8; 4] {
        let hash = Keccak256::digest(signature.as_bytes());
        [hash[0], hash[1], hash[2], hash[3]]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_address() {
        let addr = "0x1234567890abcdef1234567890abcdef12345678";
        let encoded = AbiEncoder::encode_address(addr).unwrap();
        assert_eq!(encoded.len(), 32);
    }

    #[test]
    fn test_encode_uint256() {
        let encoded = AbiEncoder::encode_uint256(123);
        assert_eq!(encoded.len(), 32);
    }

    #[test]
    fn test_function_selector() {
        let selector = AbiEncoder::function_selector("addEndpoint(string)");
        assert_eq!(selector.len(), 4);
    }
}

