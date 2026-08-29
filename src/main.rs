use std::sync::Arc;
use clap::Parser;
use tokio::runtime::Runtime;

use iori_the_apacher::{
    adapters::db_adapter::SqliteStorageAdapter,
    application::service::ApplicationService,
    cli::parser::{CliArgs, Commands, TunnelAction, ForensicAction, CryptoAction, ServerAction},
    cli::shell,
    domain::crypto_config::{DomesticAlgorithm, QuantumAlgorithm},
    domain::routing::{NetworkProtocol, CryptoRequirement},
};

fn main() -> Result<(), String> {
    let args = CliArgs::parse();
    let db_path = args.db.clone();
    let storage = Arc::new(SqliteStorageAdapter::new(&db_path)?);
    let service = ApplicationService::new(storage);

    let rt = Runtime::new().map_err(|e| e.to_string())?;

    match args.command {
        Commands::Tunnel { action } => {
            rt.block_on(async {
                match action {
                    TunnelAction::Create { subdomain, port, protocol } => {
                        let session = service.create_tunnel(subdomain, port, protocol).await?;
                        println!("Tunnel Created Successfully: ID={}, Subdomain={}, Port={}", session.id, session.subdomain, session.target_port);
                    }
                    TunnelAction::List => {
                        let tunnels = service.list_tunnels().await?;
                        println!("Active Tunnels ({})", tunnels.len());
                        for t in tunnels {
                            println!("- [{}] {} -> localhost:{} ({}) [Active: {}]", t.id, t.subdomain, t.target_port, t.protocol, t.active);
                        }
                    }
                    TunnelAction::Get { id } => {
                        match service.get_tunnel(&id).await? {
                            Some(t) => println!("Tunnel Details: ID={}, Subdomain={}, Port={}, Protocol={}, Active={}, Created={}", t.id, t.subdomain, t.target_port, t.protocol, t.active, t.created_at),
                            None => println!("Tunnel with ID {} not found", id),
                        }
                    }
                    TunnelAction::Update { id, subdomain, port, protocol, active } => {
                        service.update_tunnel(id.clone(), subdomain, port, protocol, active).await?;
                        println!("Tunnel Updated Successfully: ID={}", id);
                    }
                    TunnelAction::Delete { id } => {
                        service.delete_tunnel(id.clone()).await?;
                        println!("Tunnel Deleted Successfully: ID={}", id);
                    }
                }
                Ok::<(), String>(())
            })?;
        }
        Commands::Forensic { action } => {
            rt.block_on(async {
                match action {
                    ForensicAction::Track { ip, user_agent, hardware, geo } => {
                        let tele = service.record_forensic(ip, user_agent, hardware, geo).await?;
                        println!("Forensic Telemetry Recorded: TrackingID={}, IP={}, RiskScore={}, AnomalyFlags={:?}", tele.tracking_id, tele.source_ip, tele.risk_score, tele.anomaly_flags);
                    }
                    ForensicAction::List => {
                        let teles = service.list_forensics().await?;
                        println!("Recorded Forensic Telemetries ({})", teles.len());
                        for ft in teles {
                            println!("- [{}] IP: {} | Risk: {}/100 | Flags: {:?} | Geo: {}", ft.tracking_id, ft.source_ip, ft.risk_score, ft.anomaly_flags, ft.geo_location);
                        }
                    }
                    ForensicAction::Get { id } => {
                        match service.get_forensic(&id).await? {
                            Some(ft) => println!("Forensic Telemetry Details: ID={}, IP={}, RiskScore={}, Flags={:?}, Geo={}, Timestamp={}", ft.tracking_id, ft.source_ip, ft.risk_score, ft.anomaly_flags, ft.geo_location, ft.timestamp),
                            None => println!("Forensic telemetry with ID {} not found", id),
                        }
                    }
                    ForensicAction::Update { id, ip, user_agent, hardware, geo } => {
                        service.update_forensic(id.clone(), ip, user_agent, hardware, geo).await?;
                        println!("Forensic Telemetry Updated Successfully: ID={}", id);
                    }
                    ForensicAction::Delete { id } => {
                        service.delete_forensic(id.clone()).await?;
                        println!("Forensic Telemetry Deleted Successfully: ID={}", id);
                    }
                }
                Ok::<(), String>(())
            })?;
        }
        Commands::Crypto { action } => {
            match action {
                CryptoAction::Domestic { algorithm, key, message } => {
                    let algo = match algorithm.to_lowercase().as_str() {
                        "chacha" | "chacha20" => DomesticAlgorithm::ChaCha20Poly1305,
                        _ => DomesticAlgorithm::Aes256Gcm,
                    };
                    let (ciphertext, nonce) = service.encrypt_domestic(algo, key.as_bytes(), message.as_bytes())?;
                    println!("Domestic Encryption Successful");
                    println!("Ciphertext (hex): {}", hex_encode(&ciphertext));
                    println!("Nonce (hex): {}", hex_encode(&nonce));
                }
                CryptoAction::Quantum { algorithm, message } => {
                    let algo = match algorithm.to_lowercase().as_str() {
                        "kyber" | "ml-kem" => QuantumAlgorithm::MlKemKyber1024,
                        "dilithium" | "ml-dsa" => QuantumAlgorithm::MlDsaDilithium,
                        _ => QuantumAlgorithm::Pfe969HyperLattice,
                    };
                    let (ct, sk, pk) = service.encrypt_quantum(algo.clone(), message.as_bytes())?;
                    println!("Quantum-Resistant Encryption Successful (Algorithm: {:?})", algo);
                    println!("Public Key (hex): {}", hex_encode(&pk));
                    println!("Secret Key (hex): {}", hex_encode(&sk));
                    println!("Ciphertext (hex): {}", hex_encode(&ct));
                }
                CryptoAction::VaultSave { algorithm, ciphertext, key, metadata } => {
                    rt.block_on(async {
                        let record = service.save_crypto_vault(algorithm, ciphertext, key, metadata).await?;
                        println!("Crypto Vault Record Saved Successfully: ID={}", record.id);
                        Ok::<(), String>(())
                    })?;
                }
                CryptoAction::VaultList => {
                    rt.block_on(async {
                        let list = service.list_crypto_vaults().await?;
                        println!("Stored Crypto Vault Records ({})", list.len());
                        for r in list {
                            println!("- [{}] Algo: {} | Metadata: {} | Created: {}", r.id, r.algorithm, r.metadata, r.created_at);
                        }
                        Ok::<(), String>(())
                    })?;
                }
                CryptoAction::VaultGet { id } => {
                    rt.block_on(async {
                        match service.get_crypto_vault(&id).await? {
                            Some(r) => {
                                println!("Crypto Vault Record Details:");
                                println!("  ID: {}", r.id);
                                println!("  Algorithm: {}", r.algorithm);
                                println!("  Ciphertext (hex): {}", r.ciphertext_hex);
                                println!("  Key (hex): {}", r.key_hex);
                                println!("  Metadata: {}", r.metadata);
                                println!("  Created At: {}", r.created_at);
                            }
                            None => println!("Crypto vault record with ID {} not found", id),
                        }
                        Ok::<(), String>(())
                    })?;
                }
                CryptoAction::VaultDelete { id } => {
                    rt.block_on(async {
                        service.delete_crypto_vault(id.clone()).await?;
                        println!("Crypto Vault Record Deleted Successfully: ID={}", id);
                        Ok::<(), String>(())
                    })?;
                }
            }
        }
        Commands::Server { action } => {
            rt.block_on(async {
                match action {
                    ServerAction::Launch { subdomain, port, protocol, crypto, hops, proxychains, public } => {
                        let proto = parse_protocol(&protocol);
                        let crypto_req = parse_crypto_req(&crypto);
                        let multi_hop_nodes: Vec<String> = if hops.is_empty() {
                            vec![]
                        } else {
                            hops.split(',').map(|s| s.trim().to_string()).collect()
                        };

                        let config = service.launch_server(subdomain, port, proto, crypto_req, multi_hop_nodes, proxychains, public).await?;
                        println!("Secure Server Launched Successfully!");
                        println!("  ID: {}", config.id);
                        println!("  Subdomain: {}", config.subdomain);
                        println!("  Port: {}", config.target_port);
                        println!("  Protocol: {:?}", config.protocol);
                        println!("  Crypto Protection: {:?}", config.crypto_requirement);
                        println!("  Multi-hop Nodes: {:?}", config.multi_hop_nodes);
                        println!("  Proxychains Enabled: {}", config.proxychains_enabled);
                        println!("  Public Internet Launch: {}", config.public_internet_launch);
                    }
                    ServerAction::List => {
                        let list = service.list_server_launches().await?;
                        println!("Launched Servers ({})", list.len());
                        for s in list {
                            println!("- [{}] Subdomain: {} | Proto: {:?} | Crypto: {:?} | Hops: {:?} | Proxychains: {} | Public: {}", s.id, s.subdomain, s.protocol, s.crypto_requirement, s.multi_hop_nodes, s.proxychains_enabled, s.public_internet_launch);
                        }
                    }
                    ServerAction::Get { id } => {
                        match service.get_server_launch(&id).await? {
                            Some(s) => println!("Server Launch Details: ID={}, Subdomain={}, Port={}, Proto={:?}, Crypto={:?}, Hops={:?}, Proxychains={}, Public={}, Status={}, Created={}", s.id, s.subdomain, s.target_port, s.protocol, s.crypto_requirement, s.multi_hop_nodes, s.proxychains_enabled, s.public_internet_launch, s.status, s.created_at),
                            None => println!("Server launch with ID {} not found", id),
                        }
                    }
                    ServerAction::Update { id, subdomain, port, protocol, crypto, hops, proxychains, public } => {
                        let proto = parse_protocol(&protocol);
                        let crypto_req = parse_crypto_req(&crypto);
                        let multi_hop_nodes: Vec<String> = if hops.is_empty() {
                            vec![]
                        } else {
                            hops.split(',').map(|s| s.trim().to_string()).collect()
                        };
                        service.update_server_launch(id.clone(), subdomain, port, proto, crypto_req, multi_hop_nodes, proxychains, public).await?;
                        println!("Server Launch Updated Successfully: ID={}", id);
                    }
                    ServerAction::Delete { id } => {
                        service.delete_server_launch(id.clone()).await?;
                        println!("Server Launch Deleted Successfully: ID={}", id);
                    }
                }
                Ok::<(), String>(())
            })?;
        }
        Commands::Shell => {
            shell::run_interactive_shell(service, &rt)?;
        }
    }

    Ok(())
}

fn hex_encode(data: &[u8]) -> String {
    data.iter().map(|b| format!("{:02x}", b)).collect()
}

fn parse_protocol(s: &str) -> NetworkProtocol {
    match s.to_lowercase().as_str() {
        "https" => NetworkProtocol::Https,
        "quic" | "http3" => NetworkProtocol::Quic,
        "tcp" => NetworkProtocol::Tcp,
        "tor" | "onion" => NetworkProtocol::TorOnionV3,
        "i2p" => NetworkProtocol::I2PStream,
        "freenet" => NetworkProtocol::FreenetSst,
        _ => NetworkProtocol::Http,
    }
}

fn parse_crypto_req(s: &str) -> CryptoRequirement {
    match s.to_lowercase().as_str() {
        "aes" | "aes256" => CryptoRequirement::DomesticAes256,
        "chacha" | "chacha20" => CryptoRequirement::DomesticChaCha20,
        "kyber" | "ml-kem" => CryptoRequirement::QuantumKyber1024,
        "dilithium" | "ml-dsa" => CryptoRequirement::QuantumDilithium,
        "pfe969" | "lattice" => CryptoRequirement::QuantumPfe969Lattice,
        _ => CryptoRequirement::None,
    }
}
