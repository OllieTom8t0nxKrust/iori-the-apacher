#[cfg(test)]
mod tests {
    use iori_the_apacher::domain::crypto_vault::CryptoVaultRecord;

    #[test]
    fn test_crypto_vault_record_creation() {
        let algo = "pfe969".to_string();
        let ct = "deadbeef".to_string();
        let key = "feedface".to_string();
        let meta = "Test vault entry".to_string();

        let record = CryptoVaultRecord::new(algo.clone(), ct.clone(), key.clone(), meta.clone());

        assert_eq!(record.algorithm, algo);
        assert_eq!(record.ciphertext_hex, ct);
        assert_eq!(record.key_hex, key);
        assert_eq!(record.metadata, meta);
        assert!(!record.id.is_empty());
        assert!(!record.created_at.is_empty());
    }
}
