use async_trait::async_trait;
use crate::domain::tunnel::TunnelSession;
use crate::domain::forensic::ForensicTelemetry;

#[async_trait]
pub trait StoragePort: Send + Sync {
    async fn save_tunnel(&self, session: &TunnelSession) -> Result<(), String>;
    async fn get_tunnels(&self) -> Result<Vec<TunnelSession>, String>;
    async fn delete_tunnel(&self, id: &str) -> Result<(), String>;
    async fn save_forensic(&self, telemetry: &ForensicTelemetry) -> Result<(), String>;
    async fn get_forensics(&self) -> Result<Vec<ForensicTelemetry>, String>;
}
