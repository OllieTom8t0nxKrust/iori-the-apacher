use async_trait::async_trait;
use rusqlite::{Connection, params, OptionalExtension};
use std::sync::{Arc, Mutex};
use crate::ports::storage_port::StoragePort;
use crate::domain::tunnel::TunnelSession;
use crate::domain::crypto_vault::CryptoVaultRecord;
use crate::domain::routing::{ServerLaunchConfig, NetworkProtocol, CryptoRequirement};

#[derive(Clone)]
pub struct SqliteStorageAdapter {
    connection: Arc<Mutex<Connection>>,
}

impl SqliteStorageAdapter {
    pub fn new(db_path: &str) -> Result<Self, String> {
        let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tunnels (
                id TEXT PRIMARY KEY,
                subdomain TEXT NOT NULL,
                target_port INTEGER NOT NULL,
                protocol TEXT NOT NULL,
                active BOOLEAN NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        ).map_err(|e| e.to_string())?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS crypto_vault (
                id TEXT PRIMARY KEY,
                algorithm TEXT NOT NULL,
                ciphertext_hex TEXT NOT NULL,
                key_hex TEXT NOT NULL,
                metadata TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        ).map_err(|e| e.to_string())?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS server_launches (
                id TEXT PRIMARY KEY,
                subdomain TEXT NOT NULL,
                target_port INTEGER NOT NULL,
                protocol TEXT NOT NULL,
                crypto_requirement TEXT NOT NULL,
                multi_hop_nodes TEXT NOT NULL,
                proxychains_enabled BOOLEAN NOT NULL,
                public_internet_launch BOOLEAN NOT NULL,
                status TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        ).map_err(|e| e.to_string())?;

        Ok(Self {
            connection: Arc::new(Mutex::new(conn)),
        })
    }
}

#[async_trait]
impl StoragePort for SqliteStorageAdapter {
    async fn save_tunnel(&self, session: &TunnelSession) -> Result<(), String> {
        let conn = self.connection.lock().map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT OR REPLACE INTO tunnels (id, subdomain, target_port, protocol, active, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![session.id, session.subdomain, session.target_port, session.protocol, session.active, session.created_at],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn get_tunnels(&self) -> Result<Vec<TunnelSession>, String> {
        let conn = self.connection.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn.prepare("SELECT id, subdomain, target_port, protocol, active, created_at FROM tunnels").map_err(|e| e.to_string())?;
        let tunnel_iter = stmt.query_map([], |row| {
            Ok(TunnelSession {
                id: row.get(0)?,
                subdomain: row.get(1)?,
                target_port: row.get(2)?,
                protocol: row.get(3)?,
                active: row.get(4)?,
                created_at: row.get(5)?,
            })
        }).map_err(|e| e.to_string())?;

        let mut tunnels = Vec::new();
        for tunnel in tunnel_iter {
            tunnels.push(tunnel.map_err(|e| e.to_string())?);
        }
        Ok(tunnels)
    }

    async fn get_tunnel(&self, id: &str) -> Result<Option<TunnelSession>, String> {
        let conn = self.connection.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn.prepare("SELECT id, subdomain, target_port, protocol, active, created_at FROM tunnels WHERE id = ?1").map_err(|e| e.to_string())?;
        let tunnel = stmt.query_row(params![id], |row| {
            Ok(TunnelSession {
                id: row.get(0)?,
                subdomain: row.get(1)?,
                target_port: row.get(2)?,
                protocol: row.get(3)?,
                active: row.get(4)?,
                created_at: row.get(5)?,
            })
        }).optional().map_err(|e| e.to_string())?;
        Ok(tunnel)
    }

    async fn update_tunnel(&self, session: &TunnelSession) -> Result<(), String> {
        let conn = self.connection.lock().map_err(|e| e.to_string())?;
        let rows = conn.execute(
            "UPDATE tunnels SET subdomain = ?1, target_port = ?2, protocol = ?3, active = ?4 WHERE id = ?5",
            params![session.subdomain, session.target_port, session.protocol, session.active, session.id],
        ).map_err(|e| e.to_string())?;
        if rows == 0 {
            return Err(format!("Tunnel with ID {} not found", session.id));
        }
        Ok(())
    }

    async fn delete_tunnel(&self, id: &str) -> Result<(), String> {
        let conn = self.connection.lock().map_err(|e| e.to_string())?;
        let rows = conn.execute("DELETE FROM tunnels WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
        if rows == 0 {
            return Err(format!("Tunnel with ID {} not found", id));
        }
        Ok(())
    }

    async fn save_crypto_vault(&self, record: &CryptoVaultRecord) -> Result<(), String> {
        let conn = self.connection.lock().map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT OR REPLACE INTO crypto_vault (id, algorithm, ciphertext_hex, key_hex, metadata, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![record.id, record.algorithm, record.ciphertext_hex, record.key_hex, record.metadata, record.created_at],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn get_crypto_vaults(&self) -> Result<Vec<CryptoVaultRecord>, String> {
        let conn = self.connection.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn.prepare("SELECT id, algorithm, ciphertext_hex, key_hex, metadata, created_at FROM crypto_vault").map_err(|e| e.to_string())?;
        let iter = stmt.query_map([], |row| {
            Ok(CryptoVaultRecord {
                id: row.get(0)?,
                algorithm: row.get(1)?,
                ciphertext_hex: row.get(2)?,
                key_hex: row.get(3)?,
                metadata: row.get(4)?,
                created_at: row.get(5)?,
            })
        }).map_err(|e| e.to_string())?;

        let mut list = Vec::new();
        for item in iter {
            list.push(item.map_err(|e| e.to_string())?);
        }
        Ok(list)
    }

    async fn get_crypto_vault(&self, id: &str) -> Result<Option<CryptoVaultRecord>, String> {
        let conn = self.connection.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn.prepare("SELECT id, algorithm, ciphertext_hex, key_hex, metadata, created_at FROM crypto_vault WHERE id = ?1").map_err(|e| e.to_string())?;
        let record = stmt.query_row(params![id], |row| {
            Ok(CryptoVaultRecord {
                id: row.get(0)?,
                algorithm: row.get(1)?,
                ciphertext_hex: row.get(2)?,
                key_hex: row.get(3)?,
                metadata: row.get(4)?,
                created_at: row.get(5)?,
            })
        }).optional().map_err(|e| e.to_string())?;
        Ok(record)
    }

    async fn delete_crypto_vault(&self, id: &str) -> Result<(), String> {
        let conn = self.connection.lock().map_err(|e| e.to_string())?;
        let rows = conn.execute("DELETE FROM crypto_vault WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
        if rows == 0 {
            return Err(format!("Crypto vault record with ID {} not found", id));
        }
        Ok(())
    }

    async fn save_server_launch(&self, config: &ServerLaunchConfig) -> Result<(), String> {
        let conn = self.connection.lock().map_err(|e| e.to_string())?;
        let proto_str = format!("{:?}", config.protocol);
        let crypto_str = format!("{:?}", config.crypto_requirement);
        let hops_json = serde_json::to_string(&config.multi_hop_nodes).unwrap_or_default();
        conn.execute(
            "INSERT OR REPLACE INTO server_launches (id, subdomain, target_port, protocol, crypto_requirement, multi_hop_nodes, proxychains_enabled, public_internet_launch, status, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                config.id,
                config.subdomain,
                config.target_port,
                proto_str,
                crypto_str,
                hops_json,
                config.proxychains_enabled,
                config.public_internet_launch,
                config.status,
                config.created_at
            ],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn get_server_launches(&self) -> Result<Vec<ServerLaunchConfig>, String> {
        let conn = self.connection.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn.prepare("SELECT id, subdomain, target_port, protocol, crypto_requirement, multi_hop_nodes, proxychains_enabled, public_internet_launch, status, created_at FROM server_launches").map_err(|e| e.to_string())?;
        let iter = stmt.query_map([], |row| {
            let proto_str: String = row.get(3)?;
            let proto = match proto_str.as_str() {
                "Https" => NetworkProtocol::Https,
                "Quic" => NetworkProtocol::Quic,
                "Tcp" => NetworkProtocol::Tcp,
                "TorOnionV3" => NetworkProtocol::TorOnionV3,
                "I2PStream" => NetworkProtocol::I2PStream,
                "FreenetSst" => NetworkProtocol::FreenetSst,
                _ => NetworkProtocol::Http,
            };
            let crypto_str: String = row.get(4)?;
            let crypto = match crypto_str.as_str() {
                "DomesticAes256" => CryptoRequirement::DomesticAes256,
                "DomesticChaCha20" => CryptoRequirement::DomesticChaCha20,
                "QuantumKyber1024" => CryptoRequirement::QuantumKyber1024,
                "QuantumDilithium" => CryptoRequirement::QuantumDilithium,
                "QuantumPfe969Lattice" => CryptoRequirement::QuantumPfe969Lattice,
                _ => CryptoRequirement::None,
            };
            let hops_str: String = row.get(5)?;
            let multi_hop_nodes: Vec<String> = serde_json::from_str(&hops_str).unwrap_or_default();

            Ok(ServerLaunchConfig {
                id: row.get(0)?,
                subdomain: row.get(1)?,
                target_port: row.get(2)?,
                protocol: proto,
                crypto_requirement: crypto,
                multi_hop_nodes,
                proxychains_enabled: row.get(6)?,
                public_internet_launch: row.get(7)?,
                status: row.get(8)?,
                created_at: row.get(9)?,
            })
        }).map_err(|e| e.to_string())?;

        let mut list = Vec::new();
        for item in iter {
            list.push(item.map_err(|e| e.to_string())?);
        }
        Ok(list)
    }

    async fn get_server_launch(&self, id: &str) -> Result<Option<ServerLaunchConfig>, String> {
        let conn = self.connection.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn.prepare("SELECT id, subdomain, target_port, protocol, crypto_requirement, multi_hop_nodes, proxychains_enabled, public_internet_launch, status, created_at FROM server_launches WHERE id = ?1").map_err(|e| e.to_string())?;
        let record = stmt.query_row(params![id], |row| {
            let proto_str: String = row.get(3)?;
            let proto = match proto_str.as_str() {
                "Https" => NetworkProtocol::Https,
                "Quic" => NetworkProtocol::Quic,
                "Tcp" => NetworkProtocol::Tcp,
                "TorOnionV3" => NetworkProtocol::TorOnionV3,
                "I2PStream" => NetworkProtocol::I2PStream,
                "FreenetSst" => NetworkProtocol::FreenetSst,
                _ => NetworkProtocol::Http,
            };
            let crypto_str: String = row.get(4)?;
            let crypto = match crypto_str.as_str() {
                "DomesticAes256" => CryptoRequirement::DomesticAes256,
                "DomesticChaCha20" => CryptoRequirement::DomesticChaCha20,
                "QuantumKyber1024" => CryptoRequirement::QuantumKyber1024,
                "QuantumDilithium" => CryptoRequirement::QuantumDilithium,
                "QuantumPfe969Lattice" => CryptoRequirement::QuantumPfe969Lattice,
                _ => CryptoRequirement::None,
            };
            let hops_str: String = row.get(5)?;
            let multi_hop_nodes: Vec<String> = serde_json::from_str(&hops_str).unwrap_or_default();

            Ok(ServerLaunchConfig {
                id: row.get(0)?,
                subdomain: row.get(1)?,
                target_port: row.get(2)?,
                protocol: proto,
                crypto_requirement: crypto,
                multi_hop_nodes,
                proxychains_enabled: row.get(6)?,
                public_internet_launch: row.get(7)?,
                status: row.get(8)?,
                created_at: row.get(9)?,
            })
        }).optional().map_err(|e| e.to_string())?;
        Ok(record)
    }

    async fn update_server_launch(&self, config: &ServerLaunchConfig) -> Result<(), String> {
        let conn = self.connection.lock().map_err(|e| e.to_string())?;
        let proto_str = format!("{:?}", config.protocol);
        let crypto_str = format!("{:?}", config.crypto_requirement);
        let hops_json = serde_json::to_string(&config.multi_hop_nodes).unwrap_or_default();
        let rows = conn.execute(
            "UPDATE server_launches SET subdomain = ?1, target_port = ?2, protocol = ?3, crypto_requirement = ?4, multi_hop_nodes = ?5, proxychains_enabled = ?6, public_internet_launch = ?7, status = ?8 WHERE id = ?9",
            params![
                config.subdomain,
                config.target_port,
                proto_str,
                crypto_str,
                hops_json,
                config.proxychains_enabled,
                config.public_internet_launch,
                config.status,
                config.id
            ],
        ).map_err(|e| e.to_string())?;
        if rows == 0 {
            return Err(format!("Server launch with ID {} not found", config.id));
        }
        Ok(())
    }

    async fn delete_server_launch(&self, id: &str) -> Result<(), String> {
        let conn = self.connection.lock().map_err(|e| e.to_string())?;
        let rows = conn.execute("DELETE FROM server_launches WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
        if rows == 0 {
            return Err(format!("Server launch record with ID {} not found", id));
        }
        Ok(())
    }
}
