#[cfg(test)]
mod tests {
    use iori_the_apacher::domain::routing::{ServerLaunchConfig, NetworkProtocol, CryptoRequirement};

    #[test]
    fn test_server_launch_config_valid() {
        let config = ServerLaunchConfig::new(
            "secure-sub".to_string(),
            443,
            NetworkProtocol::TorOnionV3,
            CryptoRequirement::QuantumPfe969Lattice,
            vec!["node1".to_string(), "node2".to_string()],
            true,
            true,
        );

        assert!(config.is_ok());
        let cfg = config.unwrap();
        assert_eq!(cfg.subdomain, "secure-sub");
        assert_eq!(cfg.target_port, 443);
        assert_eq!(cfg.protocol, NetworkProtocol::TorOnionV3);
        assert_eq!(cfg.crypto_requirement, CryptoRequirement::QuantumPfe969Lattice);
        assert!(cfg.public_internet_launch);
        assert!(cfg.proxychains_enabled);
    }

    #[test]
    fn test_server_launch_security_violation() {
        let config = ServerLaunchConfig::new(
            "insecure-sub".to_string(),
            80,
            NetworkProtocol::Http,
            CryptoRequirement::None,
            vec![],
            false,
            true, // Public internet launch with no crypto -> violation
        );

        assert!(config.is_err());
        assert!(config.unwrap_err().contains("Security Violation"));
    }
}
