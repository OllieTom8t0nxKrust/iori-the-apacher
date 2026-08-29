use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum NetworkProtocol {
    Http,
    Https,
    Quic,
    Tcp,
    TorOnionV3,
    I2PStream,
    FreenetSst,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum CryptoRequirement {
    None,
    DomesticAes256,
    DomesticChaCha20,
    QuantumKyber1024,
    QuantumDilithium,
    QuantumPfe969Lattice,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ServerLaunchConfig {
    pub id: String,
    pub subdomain: String,
    pub target_port: u16,
    pub protocol: NetworkProtocol,
    pub crypto_requirement: CryptoRequirement,
    pub multi_hop_nodes: Vec<String>,
    pub proxychains_enabled: bool,
    pub public_internet_launch: bool,
    pub status: String,
    pub created_at: String,
}

impl ServerLaunchConfig {
    pub fn new(
        subdomain: String,
        target_port: u16,
        protocol: NetworkProtocol,
        crypto_requirement: CryptoRequirement,
        multi_hop_nodes: Vec<String>,
        proxychains_enabled: bool,
        public_internet_launch: bool,
    ) -> Result<Self, String> {
        // Enforce crypto verification policy: if launching to public internet without explicit crypto verification, check rules
        if public_internet_launch && crypto_requirement == CryptoRequirement::None {
            return Err("Security Violation: Public internet server launch requires active cryptographic protection. Please select a domestic or quantum cipher (e.g. PFE-969, AES, Kyber) or explicitly configure non-cryptographed local/test mode.".to_string());
        }

        Ok(Self {
            id: uuid::Uuid::new_v4().to_string(),
            subdomain,
            target_port,
            protocol,
            crypto_requirement,
            multi_hop_nodes,
            proxychains_enabled,
            public_internet_launch,
            status: "Launched & Secured".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        })
    }
}
