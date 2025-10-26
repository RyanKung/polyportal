use anyhow::{Context, Result};
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit},
    Aes256Gcm, Nonce,
};
use sha2::{Sha256, Digest};
use rand::Rng;

const SALT_SIZE: usize = 16;
const NONCE_SIZE: usize = 12;

pub fn encrypt_private_key(key: &str, password: &str) -> Result<String> {
    // Generate random salt
    let mut salt = [0u8; SALT_SIZE];
    rand::thread_rng().fill(&mut salt[..]);
    
    // Derive encryption key from password + salt
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(salt);
    let encryption_key = hasher.finalize();
    
    // Encrypt the private key
    let cipher = Aes256Gcm::new_from_slice(&encryption_key)?;
    let nonce = Aes256Gcm::generate_nonce(&mut rand::thread_rng());
    
    let private_key_bytes = hex::decode(key.strip_prefix("0x").unwrap_or(key))
        .context("Failed to decode private key")?;
    
    let ciphertext = cipher.encrypt(&nonce, private_key_bytes.as_ref())
        .map_err(|e| anyhow::anyhow!("Encryption failed: {:?}", e))?;
    
    // Combine: salt (16) + nonce (12) + ciphertext
    let mut combined = salt.to_vec();
    #[allow(deprecated)]
    combined.extend_from_slice(nonce.as_slice());
    combined.extend_from_slice(&ciphertext);
    
    Ok(hex::encode(&combined))
}

pub fn decrypt_private_key(encrypted: &str, password: &str) -> Result<String> {
    // Decode the encrypted data
    let data = hex::decode(encrypted)?;
    
    if data.len() < SALT_SIZE + NONCE_SIZE {
        anyhow::bail!("Invalid encrypted data format");
    }
    
    // Extract components
    let salt = &data[0..SALT_SIZE];
    let nonce_bytes = &data[SALT_SIZE..SALT_SIZE + NONCE_SIZE];
    let ciphertext = &data[SALT_SIZE + NONCE_SIZE..];
    
    // Derive the same encryption key from password + salt
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(salt);
    let encryption_key = hasher.finalize();
    
    // Decrypt
    let cipher = Aes256Gcm::new_from_slice(&encryption_key)?;
    #[allow(deprecated)]
    let nonce = Nonce::from_slice(nonce_bytes);
    
    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|_| anyhow::anyhow!("Decryption failed - wrong password?"))?;
    
    Ok(format!("0x{}", hex::encode(plaintext)))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encrypt_decrypt() {
        let key = "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let password = "test_password123";
        
        let encrypted = encrypt_private_key(key, password).unwrap();
        assert_ne!(encrypted, key);
        
        let decrypted = decrypt_private_key(&encrypted, password).unwrap();
        assert_eq!(key, decrypted);
        
        // Test wrong password
        assert!(decrypt_private_key(&encrypted, "wrong_password").is_err());
    }
}
