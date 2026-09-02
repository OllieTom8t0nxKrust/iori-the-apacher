#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use iori_the_apacher::application::service::ApplicationService;
    use iori_the_apacher::adapters::db_adapter::SqliteStorageAdapter;

    #[tokio::test]
    async fn test_chaos_rapid_session_churn() {
        let db = SqliteStorageAdapter::new(":memory:").unwrap();
        let service = ApplicationService::new(Arc::new(db));

        for i in 0..50 {
            let session = service.create_tunnel(format!("churn-{}", i), 9000, "https".to_string()).await.unwrap();
            service.delete_tunnel(session.id).await.unwrap();
        }

        let remaining = service.list_tunnels().await.unwrap();
        assert_eq!(remaining.len(), 0);
    }
}
