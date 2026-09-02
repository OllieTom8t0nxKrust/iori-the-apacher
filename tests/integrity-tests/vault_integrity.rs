#[cfg(test)]
mod tests {
    use iori_the_apacher::adapters::db_adapter::SqliteStorageAdapter;
    use iori_the_apacher::ports::storage_port::StoragePort;
    use iori_the_apacher::domain::crypto_vault::CryptoVaultRecord;

    #[tokio::test]
    async fn test_crypto_vault_persistence_integrity() {
        let adapter = SqliteStorageAdapter::new(":memory:").unwrap();

        let record = CryptoVaultRecord::new(
            "pfe969".to_string(),
            "aabbccdd".to_string(),
            "11223344".to_string(),
            "Integrity check metadata".to_string(),
        );

        adapter.save_crypto_vault(&record).await.unwrap();

        let records = adapter.get_crypto_vaults().await.unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].id, record.id);
        assert_eq!(records[0].algorithm, "pfe969");
        assert_eq!(records[0].ciphertext_hex, "aabbccdd");
        assert_eq!(records[0].key_hex, "11223344");
        assert_eq!(records[0].metadata, "Integrity check metadata");
    }
}
