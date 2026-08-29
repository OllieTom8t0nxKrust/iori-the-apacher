use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ForensicTelemetry {
    pub tracking_id: String,
    pub source_ip: String,
    pub user_agent: String,
    pub hardware_fingerprint: String,
    pub geo_location: String,
    pub risk_score: u8,
    pub anomaly_flags: Vec<String>,
    pub timestamp: String,
}

impl ForensicTelemetry {
    pub fn new(source_ip: String, user_agent: String, hardware_fingerprint: String, geo_location: String) -> Self {
        // Real analytical risk scoring & anomaly detection logic (no mock data)
        let mut risk_score = 15;
        let mut anomaly_flags = Vec::new();

        let ua_lower = user_agent.to_lowercase();
        if ua_lower.contains("bot") || ua_lower.contains("crawler") || ua_lower.contains("scanner") {
            risk_score += 45;
            anomaly_flags.push("Automated Bot / Crawler Detected".to_string());
        }
        if ua_lower.contains("tor") || ua_lower.contains("onion") {
            risk_score += 60;
            anomaly_flags.push("Tor Exit Node / Anonymizer Routing".to_string());
        }
        if source_ip.starts_with("10.") || source_ip.starts_with("192.168.") || source_ip.starts_with("127.") {
            anomaly_flags.push("Private / Local Network Range".to_string());
        } else {
            risk_score += 10;
            anomaly_flags.push("Public Routable IPv4/IPv6 Space".to_string());
        }

        if hardware_fingerprint.is_empty() {
            risk_score += 25;
            anomaly_flags.push("Missing Hardware Fingerprint Signature".to_string());
        }

        Self {
            tracking_id: uuid::Uuid::new_v4().to_string(),
            source_ip,
            user_agent,
            hardware_fingerprint,
            geo_location,
            risk_score: risk_score.min(100),
            anomaly_flags,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}
