#[cfg(test)]
mod tests {
    use iori_the_apacher::domain::proxychains::{ProxychainsConfig, ChainType, ProxyServer};

    #[test]
    fn test_proxychains_config_generation() {
        let server = ProxyServer {
            proxy_type: "socks5".to_string(),
            ip: "127.0.0.1".to_string(),
            port: 9050,
            credentials: None,
        };

        let config = ProxychainsConfig {
            chain_type: ChainType::DynamicChain,
            tcp_read_time_out: 15000,
            tcp_connect_time_out: 8000,
            proxies: vec![server],
        };

        assert_eq!(config.chain_type, ChainType::DynamicChain);
        assert_eq!(config.proxies.len(), 1);
        assert_eq!(config.proxies[0].port, 9050);
    }
}
