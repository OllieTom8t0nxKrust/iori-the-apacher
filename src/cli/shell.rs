use std::process::Command;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use tokio::runtime::Runtime;
use crate::application::service::ApplicationService;
use crate::domain::routing::{NetworkProtocol, CryptoRequirement};
use crate::domain::crypto_config::{DomesticAlgorithm, QuantumAlgorithm};

const ASCII_BANNERS: &[&str] = &[
r#"
 ██╗ ██████╗ ██████╗ ██╗    ████████╗███████╗██╗  ██╗███████╗     █████╗ ██████╗  █████╗  ██████╗██╗  ██╗███████╗██████╗ 
 ██║██╔═══██╗██╔══██╗██║    ╚══██╔══╝██╔════╝██║  ██║██╔════╝    ██╔══██╗██╔══██╗██╔══██╗██╔════╝██║  ██║██╔════╝██╔══██╗
 ██║██║   ██║██████╔╝██║       ██║   ███████╗███████║███████╗    ███████║██████╔╝███████║██║     ███████║█████╗  ██████╔╝
 ██║██║   ██║██╔══██╗██║       ██║   ██╔═══╝ ██╔══██║██╔════╝    ██╔══██║██╔═══╝ ██╔══██║██║     ██╔══██║██╔══╝  ██╔══██╗
 ██║╚██████╔╝██║  ██║██║       ██║   ███████╗██║  ██║███████╗    ██║  ██║██║     ██║  ██║╚██████╗██║  ██║███████╗██║  ██║
 ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝       ╚═╝   ╚══════╝╚═╝  ╚═╝╚══════╝    ╚═╝  ╚═╝╚═╝     ╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝
"#
];

fn tokenize_line(line: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut quote_char = '\0';
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if in_quotes {
            if c == quote_char {
                in_quotes = false;
            } else {
                current.push(c);
            }
        } else {
            if c == '"' || c == '\'' {
                in_quotes = true;
                quote_char = c;
            } else if c.is_whitespace() {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            } else {
                current.push(c);
            }
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

pub fn run_interactive_shell(service: ApplicationService, rt: &Runtime) -> Result<(), String> {
    let mut rl = DefaultEditor::new().map_err(|e| e.to_string())?;
    let _ = rl.load_history(".iori_history");

    let banner_idx = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos() as usize % ASCII_BANNERS.len();

    println!("{}", ASCII_BANNERS[banner_idx]);
    println!("==========================================================================================");
    println!("  Iori the APACHER - Interactive REPL Shell & Advanced Routing Suite (Tor/I2P/Freenet/PQC)");
    println!("==========================================================================================");
    println!("Type 'help' for command list and interactive documentation.");
    
    loop {
        let readline = rl.readline("iori-the-apacher> ");
        match readline {
            Ok(line) => {
                let line_str = line.trim();
                if line_str.is_empty() { continue; }
                let _ = rl.add_history_entry(line_str);
                let parts = tokenize_line(line_str);
                if parts.is_empty() { continue; }
                
                let cmd = parts[0].to_lowercase();
                
                match cmd.as_str() {
                    "exit" | "quit" => {
                        println!("Exiting interactive shell. Active background tunnels and server listener states preserved.");
                        break;
                    }
                    "help" => {
                        println!("==========================================================================================");
                        println!("Iori the APACHER - Interactive Shell Command Directory");
                        println!("==========================================================================================");
                        println!("1. OS Commands: 'ls', 'pwd', 'cat', etc. execute natively.");
                        println!("2. App Commands:");
                        println!("   - tunnel list                                   : List all stored network tunnels");
                        println!("   - tunnel create <subdomain> <port> [protocol]   : Create a network tunnel session");
                        println!("   - crypto domestic <aes|chacha> <key> <msg>      : Run domestic encryption");
                        println!("   - crypto quantum <pfe969|kyber|dilithium> <msg> : Run PQC quantum encryption");
                        println!("   - crypto vault-list                             : List secure crypto vault records");
                        println!("   - server list                                   : List launched secure servers");
                        println!("   - server launch <subdomain> <port> [proto] [crypto] : Launch server with Tor/I2P/PQC");
                        println!("   - create <subdomain> <port> [protocol]          : Shortcut to create tunnel session");
                        println!("   - help                                          : Print this help menu");
                        println!("   - exit / quit                                   : Exit the interactive shell");
                        println!("==========================================================================================");
                    }
                    "tunnel" => {
                        if parts.len() > 1 && parts[1] == "list" {
                            let s = service.clone();
                            if let Err(e) = rt.block_on(async {
                                let tunnels = s.list_tunnels().await?;
                                println!("Active Tunnels ({})", tunnels.len());
                                for t in tunnels {
                                    println!("- [{}] {} -> localhost:{} ({}) [Active: {}]", t.id, t.subdomain, t.target_port, t.protocol, t.active);
                                }
                                Ok::<(), String>(())
                            }) {
                                println!("Error: {}", e);
                            }
                        } else if parts.len() > 3 && parts[1] == "create" {
                            let subdomain = parts[2].to_string();
                            let port = parts[3].parse::<u16>().unwrap_or(8080);
                            let protocol = if parts.len() > 4 { parts[4].clone() } else { "http".to_string() };
                            let s = service.clone();
                            if let Err(e) = rt.block_on(async {
                                let session = s.create_tunnel(subdomain, port, protocol).await?;
                                println!("Tunnel Created Successfully: ID={}, Subdomain={}, Port={}", session.id, session.subdomain, session.target_port);
                                Ok::<(), String>(())
                            }) {
                                println!("Error: {}", e);
                            }
                        } else {
                            println!("Usage: tunnel list | tunnel create <subdomain> <port> [protocol]");
                        }
                    }

                    "crypto" => {
                        if parts.len() > 1 && parts[1] == "vault-list" {
                            let s = service.clone();
                            if let Err(e) = rt.block_on(async {
                                let list = s.list_crypto_vaults().await?;
                                println!("Stored Crypto Vault Records ({})", list.len());
                                for r in list {
                                    println!("- [{}] Algo: {} | Metadata: {} | Created: {}", r.id, r.algorithm, r.metadata, r.created_at);
                                }
                                Ok::<(), String>(())
                            }) {
                                println!("Error: {}", e);
                            }
                        } else if parts.len() > 3 && parts[1] == "domestic" {
                            let algo = match parts[2].to_lowercase().as_str() {
                                "chacha" => DomesticAlgorithm::ChaCha20Poly1305,
                                _ => DomesticAlgorithm::Aes256Gcm,
                            };
                            let key = parts[3].as_bytes();
                            let msg = if parts.len() > 4 { parts[4..].join(" ") } else { "test".to_string() };
                            match service.encrypt_domestic(algo, key, msg.as_bytes()) {
                                Ok((ct, nonce)) => {
                                    println!("Domestic Encryption Successful");
                                    println!("Ciphertext (hex): {}", hex_encode(&ct));
                                    println!("Nonce (hex): {}", hex_encode(&nonce));
                                }
                                Err(e) => println!("Error: {}", e),
                            }
                        } else if parts.len() > 3 && parts[1] == "quantum" {
                            let algo = match parts[2].to_lowercase().as_str() {
                                "kyber" => QuantumAlgorithm::MlKemKyber1024,
                                "dilithium" => QuantumAlgorithm::MlDsaDilithium,
                                _ => QuantumAlgorithm::Pfe969HyperLattice,
                            };
                            let msg = parts[3..].join(" ");
                            match service.encrypt_quantum(algo.clone(), msg.as_bytes()) {
                                Ok((ct, sk, pk)) => {
                                    println!("Quantum Encryption Successful ({:?})", algo);
                                    println!("Public Key (hex): {}", hex_encode(&pk));
                                    println!("Secret Key (hex): {}", hex_encode(&sk));
                                    println!("Ciphertext (hex): {}", hex_encode(&ct));
                                }
                                Err(e) => println!("Error: {}", e),
                            }
                        } else {
                            println!("Usage: crypto vault-list | crypto domestic <aes|chacha> <key> <msg> | crypto quantum <pfe969|kyber> <msg>");
                        }
                    }
                    "server" => {
                        if parts.len() > 1 && parts[1] == "list" {
                            let s = service.clone();
                            if let Err(e) = rt.block_on(async {
                                let list = s.list_server_launches().await?;
                                println!("Launched Servers ({})", list.len());
                                for srv in list {
                                    println!("- [{}] Subdomain: {} | Proto: {:?} | Crypto: {:?} | Public: {}", srv.id, srv.subdomain, srv.protocol, srv.crypto_requirement, srv.public_internet_launch);
                                }
                                Ok::<(), String>(())
                            }) {
                                println!("Error: {}", e);
                            }
                        } else if parts.len() > 3 && parts[1] == "launch" {
                            let subdomain = parts[2].to_string();
                            let port = parts[3].parse::<u16>().unwrap_or(8080);
                            let protocol_str = if parts.len() > 4 { parts[4].as_str() } else { "http" };
                            let crypto_str = if parts.len() > 5 { parts[5].as_str() } else { "pfe969" };
                            
                            let proto = match protocol_str.to_lowercase().as_str() {
                                "https" => NetworkProtocol::Https,
                                "quic" => NetworkProtocol::Quic,
                                "tcp" => NetworkProtocol::Tcp,
                                "tor" | "onion" => NetworkProtocol::TorOnionV3,
                                "i2p" => NetworkProtocol::I2PStream,
                                "freenet" => NetworkProtocol::FreenetSst,
                                _ => NetworkProtocol::Http,
                            };
                            let crypto_req = match crypto_str.to_lowercase().as_str() {
                                "aes" => CryptoRequirement::DomesticAes256,
                                "chacha" => CryptoRequirement::DomesticChaCha20,
                                "kyber" => CryptoRequirement::QuantumKyber1024,
                                "dilithium" => CryptoRequirement::QuantumDilithium,
                                _ => CryptoRequirement::QuantumPfe969Lattice,
                            };

                            let s = service.clone();
                            if let Err(e) = rt.block_on(async {
                                let config = s.launch_server(subdomain, port, proto, crypto_req, vec![], false, true).await?;
                                println!("Secure Server Launched Successfully!");
                                println!("  ID: {}", config.id);
                                println!("  Subdomain: {}", config.subdomain);
                                println!("  Port: {}", config.target_port);
                                println!("  Protocol: {:?}", config.protocol);
                                println!("  Crypto Protection: {:?}", config.crypto_requirement);
                                Ok::<(), String>(())
                            }) {
                                println!("Error: {}", e);
                            }
                        } else {
                            println!("Usage: server list | server launch <subdomain> <port> [protocol] [crypto]");
                        }
                    }
                    "create" => {
                        if parts.len() > 2 {
                            let subdomain = parts[1].to_string();
                            let port = parts[2].parse::<u16>().unwrap_or(8080);
                            let protocol = if parts.len() > 3 { parts[3].clone() } else { "http".to_string() };
                            let s = service.clone();
                            if let Err(e) = rt.block_on(async {
                                let session = s.create_tunnel(subdomain, port, protocol).await?;
                                println!("Tunnel Created Successfully: ID={}, Subdomain={}, Port={}", session.id, session.subdomain, session.target_port);
                                Ok::<(), String>(())
                            }) {
                                println!("Error: {}", e);
                            }
                        } else {
                            println!("Usage: create <subdomain> <port> [protocol]");
                        }
                    }
                    _ => {
                        let refs: Vec<&str> = parts.iter().map(|s| s.as_str()).collect();
                        execute_system_command(&refs);
                    }
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(e) => { println!("Error: {:?}", e); break; }
        }
    }
    let _ = rl.save_history(".iori_history");
    Ok(())
}

fn execute_system_command(parts: &[&str]) {
    let output = Command::new(parts[0]).args(&parts[1..]).output();
    match output {
        Ok(o) => {
            if !o.stdout.is_empty() { print!("{}", String::from_utf8_lossy(&o.stdout)); }
            if !o.stderr.is_empty() { print!("{}", String::from_utf8_lossy(&o.stderr)); }
        }
        Err(e) => println!("Failed to execute command: {}", e),
    }
}

fn hex_encode(data: &[u8]) -> String {
    data.iter().map(|b| format!("{:02x}", b)).collect()
}
