#[cfg(test)]
mod tests {
    use iori_the_apacher::adapters::db_adapter::SqliteStorageAdapter;
    use iori_the_apacher::ports::storage_port::StoragePort;

    #[tokio::test]
    async fn test_sqlite_schema_auto_migration_integrity() {
        let db_path = ":memory:";
        let adapter = SqliteStorageAdapter::new(db_path).unwrap();

        let tunnels = adapter.get_tunnels().await;
        assert!(tunnels.is_ok());
        assert_eq!(tunnels.unwrap().len(), 0);

        let vault_records = adapter.get_crypto_vaults().await;
        assert!(vault_records.is_ok());
        assert_eq!(vault_records.unwrap().len(), 0);

        let server_launches = adapter.get_server_launches().await;
        assert!(server_launches.is_ok());
        assert_eq!(server_launches.unwrap().len(), 0);
    }
}
