use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CryptoVaultRecord {
    pub id: String,
    pub algorithm: String,
    pub ciphertext_hex: String,
    pub key_hex: String,
    pub metadata: String,
    pub created_at: String,
}

impl CryptoVaultRecord {
    pub fn new(algorithm: String, ciphertext_hex: String, key_hex: String, metadata: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            algorithm,
            ciphertext_hex,
            key_hex,
            metadata,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}
