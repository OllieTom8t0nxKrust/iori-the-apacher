use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ForensicTelemetry {
    pub tracking_id: String,
    pub source_ip: String,
    pub user_agent: String,
    pub hardware_fingerprint: String,
    pub geo_location: String,
    pub timestamp: String,
}

impl ForensicTelemetry {
    pub fn new(source_ip: String, user_agent: String, hardware_fingerprint: String, geo_location: String) -> Self {
        Self {
            tracking_id: uuid::Uuid::new_v4().to_string(),
            source_ip,
            user_agent,
            hardware_fingerprint,
            geo_location,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}
