use std::sync::Arc;
use crate::ports::storage_port::StoragePort;
use crate::domain::tunnel::TunnelSession;
use crate::domain::forensic::ForensicTelemetry;
use crate::domain::crypto_config::{CryptoEngine, DomesticAlgorithm, QuantumAlgorithm};
use crate::domain::pfe969::Pfe969Cipher;

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

    pub async fn delete_tunnel(&self, id: String) -> Result<(), String> {
        self.storage.delete_tunnel(&id).await
    }

    pub async fn record_forensic(&self, ip: String, ua: String, hw: String, geo: String) -> Result<ForensicTelemetry, String> {
        let telemetry = ForensicTelemetry::new(ip, ua, hw, geo);
        self.storage.save_forensic(&telemetry).await?;
        Ok(telemetry)
    }

    pub async fn list_forensics(&self) -> Result<Vec<ForensicTelemetry>, String> {
        self.storage.get_forensics().await
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
