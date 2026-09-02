#[cfg(test)]
mod tests {
    use iori_the_apacher::domain::crypto_config::{CryptoEngine, DomesticAlgorithm};

    #[test]
    fn test_mutation_invalid_key_length() {
        let short_key = vec![0u8; 10];
        let msg = b"test payload";

        let result = CryptoEngine::execute_domestic_encryption(
            DomesticAlgorithm::Aes256Gcm,
            &short_key,
            msg
        );

        assert!(result.is_err());
    }
}
