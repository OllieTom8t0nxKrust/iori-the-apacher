use async_trait::async_trait;
use rusqlite::{Connection, params};
use std::sync::{Arc, Mutex};
use crate::ports::storage_port::StoragePort;
use crate::domain::tunnel::TunnelSession;
use crate::domain::forensic::ForensicTelemetry;

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
            "CREATE TABLE IF NOT EXISTS forensics (
                tracking_id TEXT PRIMARY KEY,
                source_ip TEXT NOT NULL,
                user_agent TEXT NOT NULL,
                hardware_fingerprint TEXT NOT NULL,
                geo_location TEXT NOT NULL,
                timestamp TEXT NOT NULL
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

    async fn delete_tunnel(&self, id: &str) -> Result<(), String> {
        let conn = self.connection.lock().map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM tunnels WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn save_forensic(&self, telemetry: &ForensicTelemetry) -> Result<(), String> {
        let conn = self.connection.lock().map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT OR REPLACE INTO forensics (tracking_id, source_ip, user_agent, hardware_fingerprint, geo_location, timestamp) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![telemetry.tracking_id, telemetry.source_ip, telemetry.user_agent, telemetry.hardware_fingerprint, telemetry.geo_location, telemetry.timestamp],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn get_forensics(&self) -> Result<Vec<ForensicTelemetry>, String> {
        let conn = self.connection.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn.prepare("SELECT tracking_id, source_ip, user_agent, hardware_fingerprint, geo_location, timestamp FROM forensics").map_err(|e| e.to_string())?;
        let forensic_iter = stmt.query_map([], |row| {
            Ok(ForensicTelemetry {
                tracking_id: row.get(0)?,
                source_ip: row.get(1)?,
                user_agent: row.get(2)?,
                hardware_fingerprint: row.get(3)?,
                geo_location: row.get(4)?,
                timestamp: row.get(5)?,
            })
        }).map_err(|e| e.to_string())?;

        let mut forensics = Vec::new();
        for telemetry in forensic_iter {
            forensics.push(telemetry.map_err(|e| e.to_string())?);
        }
        Ok(forensics)
    }
}
