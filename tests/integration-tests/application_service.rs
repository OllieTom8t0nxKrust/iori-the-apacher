#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use iori_the_apacher::application::service::ApplicationService;
    use iori_the_apacher::adapters::db_adapter::SqliteStorageAdapter;

    fn setup_service() -> ApplicationService {
        let adapter = SqliteStorageAdapter::new(":memory:").expect("Failed to init test DB");
        ApplicationService::new(Arc::new(adapter))
    }

    #[tokio::test]
    async fn test_application_tunnel_lifecycle() {
        let service = setup_service();
        
        let session = service.create_tunnel("test-sub".to_string(), 8080, "https".to_string()).await.unwrap();
        
        let list = service.list_tunnels().await.unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, session.id);
        
        service.delete_tunnel(session.id).await.unwrap();
        let list_after = service.list_tunnels().await.unwrap();
        assert_eq!(list_after.len(), 0);
    }
}
