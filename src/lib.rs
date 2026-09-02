pub mod domain;
pub mod ports;
pub mod adapters;
pub mod application;
pub mod cli;

pub fn hex_encode(data: &[u8]) -> String {
    data.iter().map(|b| format!("{:02x}", b)).collect()
}

pub fn parse_protocol(s: &str) -> crate::domain::routing::NetworkProtocol {
    match s.to_lowercase().as_str() {
        "https" => crate::domain::routing::NetworkProtocol::Https,
        "quic" | "http3" => crate::domain::routing::NetworkProtocol::Quic,
        "tcp" => crate::domain::routing::NetworkProtocol::Tcp,
        "tor" | "onion" => crate::domain::routing::NetworkProtocol::TorOnionV3,
        "i2p" => crate::domain::routing::NetworkProtocol::I2PStream,
        "freenet" => crate::domain::routing::NetworkProtocol::FreenetSst,
        _ => crate::domain::routing::NetworkProtocol::Http,
    }
}

pub fn parse_crypto_req(s: &str) -> crate::domain::routing::CryptoRequirement {
    match s.to_lowercase().as_str() {
        "aes" | "aes256" => crate::domain::routing::CryptoRequirement::DomesticAes256,
        "chacha" | "chacha20" => crate::domain::routing::CryptoRequirement::DomesticChaCha20,
        "kyber" | "ml-kem" => crate::domain::routing::CryptoRequirement::QuantumKyber1024,
        "dilithium" | "ml-dsa" => crate::domain::routing::CryptoRequirement::QuantumDilithium,
        "pfe969" | "lattice" => crate::domain::routing::CryptoRequirement::QuantumPfe969Lattice,
        _ => crate::domain::routing::CryptoRequirement::None,
    }
}

pub fn prompt_background_thread() -> bool {
    use std::io::{self, Write};
    loop {
        print!("Do you wish to open a thread to run this service as a background process? [Y/N]: ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            let ans = input.trim().to_uppercase();
            if ans == "Y" || ans == "YES" {
                return true;
            } else if ans == "N" || ans == "NO" || ans.is_empty() {
                return false;
            } else {
                println!("Please enter Y or N (accepts Y, y, N, n, YES, NO in all caps or lower case).");
            }
        } else {
            return false;
        }
    }
}
