use async_trait::async_trait;
use crate::domain::tunnel::TunnelSession;
use crate::domain::forensic::ForensicTelemetry;
use crate::domain::crypto_vault::CryptoVaultRecord;
use crate::domain::routing::ServerLaunchConfig;

#[async_trait]
pub trait StoragePort: Send + Sync {
    async fn save_tunnel(&self, session: &TunnelSession) -> Result<(), String>;
    async fn get_tunnels(&self) -> Result<Vec<TunnelSession>, String>;
    async fn get_tunnel(&self, id: &str) -> Result<Option<TunnelSession>, String>;
    async fn update_tunnel(&self, session: &TunnelSession) -> Result<(), String>;
    async fn delete_tunnel(&self, id: &str) -> Result<(), String>;

    async fn save_forensic(&self, telemetry: &ForensicTelemetry) -> Result<(), String>;
    async fn get_forensics(&self) -> Result<Vec<ForensicTelemetry>, String>;
    async fn get_forensic(&self, tracking_id: &str) -> Result<Option<ForensicTelemetry>, String>;
    async fn update_forensic(&self, telemetry: &ForensicTelemetry) -> Result<(), String>;
    async fn delete_forensic(&self, tracking_id: &str) -> Result<(), String>;

    async fn save_crypto_vault(&self, record: &CryptoVaultRecord) -> Result<(), String>;
    async fn get_crypto_vaults(&self) -> Result<Vec<CryptoVaultRecord>, String>;
    async fn get_crypto_vault(&self, id: &str) -> Result<Option<CryptoVaultRecord>, String>;
    async fn delete_crypto_vault(&self, id: &str) -> Result<(), String>;

    async fn save_server_launch(&self, config: &ServerLaunchConfig) -> Result<(), String>;
    async fn get_server_launches(&self) -> Result<Vec<ServerLaunchConfig>, String>;
    async fn get_server_launch(&self, id: &str) -> Result<Option<ServerLaunchConfig>, String>;
    async fn update_server_launch(&self, config: &ServerLaunchConfig) -> Result<(), String>;
    async fn delete_server_launch(&self, id: &str) -> Result<(), String>;
}
