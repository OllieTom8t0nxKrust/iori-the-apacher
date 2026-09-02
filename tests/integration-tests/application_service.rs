#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::application::service::ApplicationService;
    use crate::adapters::db_adapter::SqliteStorageAdapter; // Assuming standard location

    // Mock storage setup would be ideal, but for now we use an in-memory test DB
    fn setup_service() -> ApplicationService {
        // This is simplified; in a real project, we'd mock the trait
        // For testing purposes, we use a temporary SQLite in-memory DB
        let adapter = SqliteStorageAdapter::new(":memory:").expect("Failed to init test DB");
        ApplicationService::new(Arc::new(adapter))
    }

    #[tokio::test]
    async fn test_application_tunnel_lifecycle() {
        let service = setup_service();
        
        // 1. Create
        let session = service.create_tunnel("test-sub".to_string(), 8080, "https".to_string()).await.unwrap();
        
        // 2. List
        let list = service.list_tunnels().await.unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, session.id);
        
        // 3. Delete
        service.delete_tunnel(session.id).await.unwrap();
        let list_after = service.list_tunnels().await.unwrap();
        assert_eq!(list_after.len(), 0);
    }
}
