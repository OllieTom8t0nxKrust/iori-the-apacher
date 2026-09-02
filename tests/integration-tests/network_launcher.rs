#[cfg(test)]
mod tests {
    use iori_the_apacher::adapters::launcher::NetworkLauncher;
    use iori_the_apacher::domain::routing::{NetworkProtocol, CryptoRequirement};

    #[tokio::test]
    async fn test_network_launcher_execution() {
        let launcher = NetworkLauncher::new();
        let result = launcher.launch_network_tunnel(
            "test-launch",
            8080,
            &NetworkProtocol::TorOnionV3,
            &CryptoRequirement::QuantumPfe969Lattice,
            &["node1".to_string(), "node2".to_string()],
            true
        ).await;

        assert!(result.is_ok());
    }
}
