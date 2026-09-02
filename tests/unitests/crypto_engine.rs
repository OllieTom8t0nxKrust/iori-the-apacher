#[cfg(test)]
mod tests {
    use crate::domain::crypto_config::{CryptoEngine, DomesticAlgorithm};

    #[test]
    fn test_aes_gcm_encryption_success() {
        let key = vec![0u8; 32];
        let msg = b"secret message".to_vec();
        
        let result = CryptoEngine::execute_domestic_encryption(
            DomesticAlgorithm::Aes256Gcm,
            &key,
            &msg
        );
        
        assert!(result.is_ok());
        let (ct, nonce) = result.unwrap();
        assert_ne!(ct, msg); // Should be encrypted
        assert_eq!(nonce.len(), 12);
    }

    #[test]
    fn test_aes_gcm_invalid_key_length() {
        let key = vec![0u8; 16]; // Too short
        let msg = b"secret message".to_vec();
        
        let result = CryptoEngine::execute_domestic_encryption(
            DomesticAlgorithm::Aes256Gcm,
            &key,
            &msg
        );
        
        assert!(result.is_err());
    }
}
