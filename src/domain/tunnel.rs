use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TunnelSession {
    pub id: String,
    pub subdomain: String,
    pub target_port: u16,
    pub protocol: String,
    pub active: bool,
    pub created_at: String,
}

impl TunnelSession {
    pub fn new(subdomain: String, target_port: u16, protocol: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            subdomain,
            target_port,
            protocol,
            active: true,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}
