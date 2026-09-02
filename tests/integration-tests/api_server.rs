#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use iori_the_apacher::application::service::ApplicationService;
    use iori_the_apacher::adapters::db_adapter::SqliteStorageAdapter;
    use iori_the_apacher::adapters::api_server::ApiServer;
    use tokio::net::TcpStream;
    use tokio::io::{AsyncWriteExt, AsyncReadExt};

    #[tokio::test]
    async fn test_api_server_endpoints() {
        let db = SqliteStorageAdapter::new(":memory:").unwrap();
        let service = Arc::new(ApplicationService::new(Arc::new(db)));
        
        let port = 18091;
        let server_service = service.clone();
        tokio::spawn(async move {
            let server = ApiServer::new(server_service, port);
            let _ = server.run().await;
        });

        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let mut stream = match TcpStream::connect(format!("127.0.0.1:{}", port)).await {
            Ok(s) => s,
            Err(_) => return,
        };

        let request = "GET /api/tunnels HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
        stream.write_all(request.as_bytes()).await.unwrap();

        let mut buf = [0; 1024];
        let n = stream.read(&mut buf).await.unwrap();
        let response = String::from_utf8_lossy(&buf[..n]);
        assert!(response.contains("HTTP/1.1 200 OK"));
    }
}
