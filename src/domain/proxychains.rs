use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ChainType {
    StrictChain,
    DynamicChain,
    RandomChain,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProxyServer {
    pub proxy_type: String, // socks4, socks5, http
    pub ip: String,
    pub port: u16,
    pub credentials: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProxychainsConfig {
    pub chain_type: ChainType,
    pub tcp_read_time_out: u32,
    pub tcp_connect_time_out: u32,
    pub proxies: Vec<ProxyServer>,
}

impl Default for ProxychainsConfig {
    fn default() -> Self {
        Self {
            chain_type: ChainType::DynamicChain,
            tcp_read_time_out: 15000,
            tcp_connect_time_out: 8000,
            proxies: vec![
                ProxyServer {
                    proxy_type: "socks5".to_string(),
                    ip: "127.0.0.1".to_string(),
                    port: 9050,
                    credentials: None,
                },
                ProxyServer {
                    proxy_type: "socks4".to_string(),
                    ip: "127.0.0.1".to_string(),
                    port: 4444,
                    credentials: None,
                },
            ],
        }
    }
}
