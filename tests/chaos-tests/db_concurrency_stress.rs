#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use iori_the_apacher::adapters::db_adapter::SqliteStorageAdapter;
    use iori_the_apacher::ports::storage_port::StoragePort;
    use iori_the_apacher::domain::tunnel::TunnelSession;

    #[tokio::test]
    async fn test_chaos_concurrent_db_writes() {
        let adapter = Arc::new(SqliteStorageAdapter::new(":memory:").unwrap());
        
        let mut handles = vec![];
        for i in 0..20 {
            let db = adapter.clone();
            let handle = tokio::spawn(async move {
                let session = TunnelSession::new(format!("sub-{}", i), 8000 + i, "http".to_string());
                db.save_tunnel(&session).await
            });
            handles.push(handle);
        }

        for handle in handles {
            let res = handle.await.unwrap();
            assert!(res.is_ok());
        }

        let tunnels = adapter.get_tunnels().await.unwrap();
        assert_eq!(tunnels.len(), 20);
    }
}
