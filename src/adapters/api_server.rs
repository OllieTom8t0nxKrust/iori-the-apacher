use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::application::service::ApplicationService;
use crate::domain::routing::{NetworkProtocol, CryptoRequirement};

pub struct ApiServer {
    service: Arc<ApplicationService>,
    port: u16,
}

impl ApiServer {
    pub fn new(service: Arc<ApplicationService>, port: u16) -> Self {
        Self { service, port }
    }

    pub async fn run(&self) -> Result<(), String> {
        let addr = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(&addr).await.map_err(|e| e.to_string())?;
        println!("IORI THE APACHER API Server running at http://{}", addr);

        loop {
            let (mut socket, _) = listener.accept().await.map_err(|e| e.to_string())?;
            let service = self.service.clone();
            tokio::spawn(async move {
                if let Err(e) = handle_connection(&mut socket, service).await {
                    eprintln!("API connection error: {}", e);
                }
            });
        }
    }
}

fn parse_protocol(s: &str) -> NetworkProtocol {
    match s.to_lowercase().as_str() {
        "https" => NetworkProtocol::Https,
        "quic" | "http3" => NetworkProtocol::Quic,
        "tcp" => NetworkProtocol::Tcp,
        "tor" | "onion" => NetworkProtocol::TorOnionV3,
        "i2p" => NetworkProtocol::I2PStream,
        "freenet" => NetworkProtocol::FreenetSst,
        _ => NetworkProtocol::Http,
    }
}

fn parse_crypto_req(s: &str) -> CryptoRequirement {
    match s.to_lowercase().as_str() {
        "aes" | "aes256" => CryptoRequirement::DomesticAes256,
        "chacha" | "chacha20" => CryptoRequirement::DomesticChaCha20,
        "kyber" | "ml-kem" => CryptoRequirement::QuantumKyber1024,
        "dilithium" | "ml-dsa" => CryptoRequirement::QuantumDilithium,
        "pfe969" | "lattice" => CryptoRequirement::QuantumPfe969Lattice,
        _ => CryptoRequirement::None,
    }
}

async fn handle_connection(socket: &mut TcpStream, service: Arc<ApplicationService>) -> Result<(), String> {
    let mut buf = [0; 4096];
    let n = socket.read(&mut buf).await.map_err(|e| e.to_string())?;
    if n == 0 {
        return Ok(());
    }

    let request_str = String::from_utf8_lossy(&buf[..n]);
    let mut lines = request_str.lines();
    let request_line = lines.next().unwrap_or("");
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return Ok(());
    }

    let method = parts[0];
    let path = parts[1];

    // Extract body if present
    let mut body = "";
    if let Some(idx) = request_str.find("\r\n\r\n") {
        body = &request_str[idx + 4..];
    } else if let Some(idx) = request_str.find("\n\n") {
        body = &request_str[idx + 2..];
    }

    let (status, content_type, response_body) = match (method, path) {
        ("OPTIONS", _) => ("200 OK", "text/plain", "".to_string()),
        
        // Tunnels API
        ("GET", "/api/tunnels") => {
            match service.list_tunnels().await {
                Ok(tunnels) => ("200 OK", "application/json", serde_json::to_string(&tunnels).unwrap_or_default()),
                Err(e) => ("500 Internal Server Error", "application/json", format!(r#"{{"error":"{}"}}"#, e)),
            }
        }
        ("POST", "/api/tunnels") => {
            #[derive(serde::Deserialize)]
            struct CreateTunnelReq {
                subdomain: String,
                port: u16,
                protocol: String,
            }
            match serde_json::from_str::<CreateTunnelReq>(body.trim()) {
                Ok(req) => {
                    match service.create_tunnel(req.subdomain, req.port, req.protocol).await {
                        Ok(t) => ("200 OK", "application/json", serde_json::to_string(&t).unwrap_or_default()),
                        Err(e) => ("400 Bad Request", "application/json", format!(r#"{{"error":"{}"}}"#, e)),
                    }
                }
                Err(e) => ("400 Bad Request", "application/json", format!(r#"{{"error":"Invalid JSON: {}"}}"#, e)),
            }
        }
        _ if method == "DELETE" && path.starts_with("/api/tunnels/") => {
            let id = path.trim_start_matches("/api/tunnels/");
            match service.delete_tunnel(id.to_string()).await {
                Ok(_) => ("200 OK", "application/json", r#"{"status":"deleted"}"#.to_string()),
                Err(e) => ("404 Not Found", "application/json", format!(r#"{{"error":"{}"}}"#, e)),
            }
        }

        // Servers API
        ("GET", "/api/servers") => {
            match service.list_server_launches().await {
                Ok(servers) => ("200 OK", "application/json", serde_json::to_string(&servers).unwrap_or_default()),
                Err(e) => ("500 Internal Server Error", "application/json", format!(r#"{{"error":"{}"}}"#, e)),
            }
        }
        ("POST", "/api/servers") => {
            #[derive(serde::Deserialize)]
            struct LaunchServerReq {
                subdomain: String,
                port: u16,
                protocol: Option<String>,
                crypto: Option<String>,
                public: Option<bool>,
            }
            match serde_json::from_str::<LaunchServerReq>(body.trim()) {
                Ok(req) => {
                    let proto = parse_protocol(req.protocol.as_deref().unwrap_or("http"));
                    let crypto = parse_crypto_req(req.crypto.as_deref().unwrap_or("pfe969"));
                    match service.launch_server(req.subdomain, req.port, proto, crypto, vec![], false, req.public.unwrap_or(false)).await {
                        Ok(s) => ("200 OK", "application/json", serde_json::to_string(&s).unwrap_or_default()),
                        Err(e) => ("400 Bad Request", "application/json", format!(r#"{{"error":"{}"}}"#, e)),
                    }
                }
                Err(e) => ("400 Bad Request", "application/json", format!(r#"{{"error":"Invalid JSON: {}"}}"#, e)),
            }
        }
        _ if method == "DELETE" && path.starts_with("/api/servers/") => {
            let id = path.trim_start_matches("/api/servers/");
            match service.delete_server_launch(id.to_string()).await {
                Ok(_) => ("200 OK", "application/json", r#"{"status":"deleted"}"#.to_string()),
                Err(e) => ("404 Not Found", "application/json", format!(r#"{{"error":"{}"}}"#, e)),
            }
        }

        // Crypto Vault API
        ("GET", "/api/crypto") => {
            match service.list_crypto_vaults().await {
                Ok(vaults) => ("200 OK", "application/json", serde_json::to_string(&vaults).unwrap_or_default()),
                Err(e) => ("500 Internal Server Error", "application/json", format!(r#"{{"error":"{}"}}"#, e)),
            }
        }
        ("POST", "/api/crypto") => {
            #[derive(serde::Deserialize)]
            struct SaveVaultReq {
                algorithm: String,
                ciphertext: String,
                key: String,
                metadata: Option<String>,
            }
            match serde_json::from_str::<SaveVaultReq>(body.trim()) {
                Ok(req) => {
                    let meta = req.metadata.unwrap_or_else(|| "Web UI Entry".to_string());
                    match service.save_crypto_vault(req.algorithm, req.ciphertext, req.key, meta).await {
                        Ok(r) => ("200 OK", "application/json", serde_json::to_string(&r).unwrap_or_default()),
                        Err(e) => ("400 Bad Request", "application/json", format!(r#"{{"error":"{}"}}"#, e)),
                    }
                }
                Err(e) => ("400 Bad Request", "application/json", format!(r#"{{"error":"Invalid JSON: {}"}}"#, e)),
            }
        }
        _ if method == "DELETE" && path.starts_with("/api/crypto/") => {
            let id = path.trim_start_matches("/api/crypto/");
            match service.delete_crypto_vault(id.to_string()).await {
                Ok(_) => ("200 OK", "application/json", r#"{"status":"deleted"}"#.to_string()),
                Err(e) => ("404 Not Found", "application/json", format!(r#"{{"error":"{}"}}"#, e)),
            }
        }

        // PFE-969 Telemetry API
        ("GET", "/api/pfe969") => {
            let telemetry = serde_json::json!({
                "algorithm": "PFE-969 Hyper-Dimensional Lattice",
                "status": "Active",
                "latticeDimension": 2048,
                "errorCorrectionDensity": "0.9998",
                "quantumEntropyBits": 256,
                "entanglementIndex": 42.7,
                "lastSync": chrono::Utc::now().to_rfc3339()
            });
            ("200 OK", "application/json", telemetry.to_string())
        }

        _ => ("404 Not Found", "application/json", r#"{"error":"Not Found"}"#.to_string()),
    };

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, DELETE, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        status,
        content_type,
        response_body.len(),
        response_body
    );

    socket.write_all(response.as_bytes()).await.map_err(|e| e.to_string())?;
    socket.flush().await.map_err(|e| e.to_string())?;
    Ok(())
}
