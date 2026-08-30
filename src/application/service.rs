use std::sync::Arc;
use crate::ports::storage_port::StoragePort;
use crate::domain::tunnel::TunnelSession;
use crate::domain::crypto_vault::CryptoVaultRecord;
use crate::domain::routing::{ServerLaunchConfig, NetworkProtocol, CryptoRequirement};
use crate::domain::crypto_config::{CryptoEngine, DomesticAlgorithm, QuantumAlgorithm};
use crate::domain::pfe969::Pfe969Cipher;
use crate::adapters::launcher::NetworkLauncher;

#[derive(Clone)]
pub struct ApplicationService {
    storage: Arc<dyn StoragePort>,
}

impl ApplicationService {
    pub fn new(storage: Arc<dyn StoragePort>) -> Self {
        Self { storage }
    }

    pub async fn create_tunnel(&self, subdomain: String, target_port: u16, protocol: String) -> Result<TunnelSession, String> {
        let session = TunnelSession::new(subdomain, target_port, protocol);
        self.storage.save_tunnel(&session).await?;
        Ok(session)
    }

    pub async fn list_tunnels(&self) -> Result<Vec<TunnelSession>, String> {
        self.storage.get_tunnels().await
    }

    pub async fn get_tunnel(&self, id: &str) -> Result<Option<TunnelSession>, String> {
        self.storage.get_tunnel(id).await
    }

    pub async fn update_tunnel(&self, id: String, subdomain: String, target_port: u16, protocol: String, active: bool) -> Result<(), String> {
        let session = TunnelSession {
            id,
            subdomain,
            target_port,
            protocol,
            active,
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        self.storage.update_tunnel(&session).await
    }

    pub async fn delete_tunnel(&self, id: String) -> Result<(), String> {
        self.storage.delete_tunnel(&id).await
    }

    pub async fn save_crypto_vault(&self, algorithm: String, ciphertext_hex: String, key_hex: String, metadata: String) -> Result<CryptoVaultRecord, String> {
        let record = CryptoVaultRecord::new(algorithm, ciphertext_hex, key_hex, metadata);
        self.storage.save_crypto_vault(&record).await?;
        Ok(record)
    }

    pub async fn list_crypto_vaults(&self) -> Result<Vec<CryptoVaultRecord>, String> {
        self.storage.get_crypto_vaults().await
    }

    pub async fn get_crypto_vault(&self, id: &str) -> Result<Option<CryptoVaultRecord>, String> {
        self.storage.get_crypto_vault(id).await
    }

    pub async fn delete_crypto_vault(&self, id: String) -> Result<(), String> {
        self.storage.delete_crypto_vault(&id).await
    }

    pub async fn launch_server(
        &self,
        subdomain: String,
        target_port: u16,
        protocol: NetworkProtocol,
        crypto_requirement: CryptoRequirement,
        multi_hop_nodes: Vec<String>,
        proxychains_enabled: bool,
        public_internet_launch: bool,
    ) -> Result<ServerLaunchConfig, String> {
        let config = ServerLaunchConfig::new(
            subdomain,
            target_port,
            protocol,
            crypto_requirement,
            multi_hop_nodes,
            proxychains_enabled,
            public_internet_launch,
        )?;
        
        // Actually launch the process
        let _child = NetworkLauncher::launch(&config)?;

        self.storage.save_server_launch(&config).await?;
        Ok(config)
    }

    pub async fn list_server_launches(&self) -> Result<Vec<ServerLaunchConfig>, String> {
        self.storage.get_server_launches().await
    }

    pub async fn get_server_launch(&self, id: &str) -> Result<Option<ServerLaunchConfig>, String> {
        self.storage.get_server_launch(id).await
    }

    pub async fn update_server_launch(
        &self,
        id: String,
        subdomain: String,
        target_port: u16,
        protocol: NetworkProtocol,
        crypto_requirement: CryptoRequirement,
        multi_hop_nodes: Vec<String>,
        proxychains_enabled: bool,
        public_internet_launch: bool,
    ) -> Result<(), String> {
        let config = ServerLaunchConfig {
            id,
            subdomain,
            target_port,
            protocol,
            crypto_requirement,
            multi_hop_nodes,
            proxychains_enabled,
            public_internet_launch,
            status: "Updated".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        self.storage.update_server_launch(&config).await
    }

    pub async fn delete_server_launch(&self, id: String) -> Result<(), String> {
        self.storage.delete_server_launch(&id).await
    }

    pub fn encrypt_domestic(&self, algo: DomesticAlgorithm, key: &[u8], plaintext: &[u8]) -> Result<(Vec<u8>, Vec<u8>), String> {
        CryptoEngine::execute_domestic_encryption(algo, key, plaintext)
    }

    pub fn encrypt_quantum(&self, algo: QuantumAlgorithm, plaintext: &[u8]) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), String> {
        CryptoEngine::execute_quantum_encryption(algo, plaintext)
    }

    pub fn decrypt_quantum(&self, algo: QuantumAlgorithm, ciphertext: &[u8], sk: &[u8]) -> Result<Vec<u8>, String> {
        match algo {
            QuantumAlgorithm::Pfe969HyperLattice => {
                let pfe = Pfe969Cipher::new(256, 2048, 32);
                pfe.decrypt(ciphertext, sk)
            }
            QuantumAlgorithm::MlKemKyber1024 => {
                let pfe = Pfe969Cipher::new(384, 1536, 24);
                if ciphertext.starts_with(b"ML-KEM-1024-CAPSULE:") {
                    pfe.decrypt(&ciphertext[20..], sk)
                } else {
                    Err("Invalid ML-KEM capsule format".to_string())
                }
            }
            QuantumAlgorithm::MlDsaDilithium => {
                let pfe = Pfe969Cipher::new(256, 1024, 16);
                if ciphertext.starts_with(b"ML-DSA-SIGNATURE-WRAP:") {
                    pfe.decrypt(&ciphertext[23..], sk)
                } else {
                    Err("Invalid ML-DSA signature wrap format".to_string())
                }
            }
        }
    }
}
