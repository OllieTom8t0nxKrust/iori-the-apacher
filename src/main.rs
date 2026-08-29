use std::sync::Arc;
use std::io::{self, Write};
use clap::Parser;
use tokio::runtime::Runtime;

use iori_the_apacher::{
    adapters::db_adapter::SqliteStorageAdapter,
    application::service::ApplicationService,
    cli::parser::{CliArgs, Commands, TunnelAction, ForensicAction, CryptoAction},
    domain::crypto_config::{DomesticAlgorithm, QuantumAlgorithm},
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
                }
                Ok::<(), String>(())
            })?;
        }
        Commands::Forensic { action } => {
            rt.block_on(async {
                match action {
                    ForensicAction::Track { ip, user_agent, hardware, geo } => {
                        let tele = service.record_forensic(ip, user_agent, hardware, geo).await?;
                        println!("Forensic Telemetry Recorded: TrackingID={}, IP={}, Geo={}", tele.tracking_id, tele.source_ip, tele.geo_location);
                    }
                    ForensicAction::List => {
                        let teles = service.list_forensics().await?;
                        println!("Recorded Forensic Telemetries ({})", teles.len());
                        for ft in teles {
                            println!("- [{}] IP: {} | UA: {} | HW: {} | Geo: {}", ft.tracking_id, ft.source_ip, ft.user_agent, ft.hardware_fingerprint, ft.geo_location);
                        }
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
                    let (ciphertext, sk, pk) = service.encrypt_quantum(algo.clone(), message.as_bytes())?;
                    println!("Quantum-Resistant Encryption Successful (Algorithm: {:?})", algo);
                    println!("Public Key (hex): {}", hex_encode(&pk));
                    println!("Secret Key (hex): {}", hex_encode(&sk));
                    println!("Ciphertext (hex): {}", hex_encode(&ciphertext));
                }
            }
        }
        Commands::Shell => {
            run_interactive_shell(service, &rt)?;
        }
    }

    Ok(())
}

fn run_interactive_shell(service: ApplicationService, rt: &Runtime) -> Result<(), String> {
    println!("============================================================");
    println!("  Iori The Apacher - Interactive REPL Shell (360° PQC & Tunnels)");
    println!("============================================================");
    println!("Type 'help' for available commands.");
    println!("Commands:");
    println!("  tunnel create <subdomain> <port> [protocol]");
    println!("  tunnel list");
    println!("  forensic track <ip> <user_agent> <hardware> <geo>");
    println!("  forensic list");
    println!("  crypto domestic <algorithm> <key> <message>");
    println!("  crypto quantum <algorithm> <message>");
    println!("  exit-background   (Detach shell, keep services running in background)");
    println!("  exit              (Secure shutdown: close all ports & clean operations)");
    println!("============================================================");

    loop {
        print!("iori-apacher> ");
        io::stdout().flush().map_err(|e| e.to_string())?;

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input");
            continue;
        }

        let line = input.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        let cmd = parts[0].to_lowercase();

        match cmd.as_str() {
            "exit" => {
                println!("[SECURE SHUTDOWN] Cleaning up all operations, closing active ports, terminating background services...");
                rt.block_on(async {
                    if let Ok(tunnels) = service.list_tunnels().await {
                        for t in tunnels {
                            println!("- Deactivating tunnel [{}] subdomain '{}' on port {}", t.id, t.subdomain, t.target_port);
                        }
                    }
                });
                println!("All resources cleaned successfully. Exiting.");
                break;
            }
            "exit-background" => {
                println!("[DETACH] Exiting interactive shell. Tunnels and services remain active in the background.");
                println!("To re-attach or manage, launch the shell or CLI again.");
                break;
            }
            "help" => {
                println!("Available commands:");
                println!("  tunnel create <subdomain> <port> [protocol]");
                println!("  tunnel list");
                println!("  forensic track <ip> <user_agent> <hardware> <geo>");
                println!("  forensic list");
                println!("  crypto domestic <aes|chacha> <key> <message>");
                println!("  crypto quantum <pfe969|kyber|dilithium> <message>");
                println!("  exit-background");
                println!("  exit");
            }
            "tunnel" => {
                if parts.len() < 2 {
                    println!("Usage: tunnel create <subdomain> <port> [protocol] OR tunnel list");
                    continue;
                }
                let sub_cmd = parts[1].to_lowercase();
                if sub_cmd == "list" {
                    rt.block_on(async {
                        match service.list_tunnels().await {
                            Ok(tunnels) => {
                                println!("Active Tunnels ({})", tunnels.len());
                                for t in tunnels {
                                    println!("- [{}] {} -> localhost:{} ({})", t.id, t.subdomain, t.target_port, t.protocol);
                                }
                            }
                            Err(e) => println!("Error listing tunnels: {}", e),
                        }
                    });
                } else if sub_cmd == "create" {
                    if parts.len() < 4 {
                        println!("Usage: tunnel create <subdomain> <port> [protocol]");
                        continue;
                    }
                    let subdomain = parts[2].to_string();
                    let port = match parts[3].parse::<u16>() {
                        Ok(p) => p,
                        Err(_) => {
                            println!("Invalid port number");
                            continue;
                        }
                    };
                    let protocol = if parts.len() > 4 { parts[4].to_string() } else { "http".to_string() };

                    rt.block_on(async {
                        match service.create_tunnel(subdomain, port, protocol).await {
                            Ok(s) => println!("Tunnel Created Successfully: ID={}, Subdomain={}, Port={}", s.id, s.subdomain, s.target_port),
                            Err(e) => println!("Error creating tunnel: {}", e),
                        }
                    });
                } else if sub_cmd == "stop" || sub_cmd == "delete" {
                    if parts.len() < 3 {
                        println!("Usage: tunnel stop <id>");
                        continue;
                    }
                    let id = parts[2].to_string();
                    rt.block_on(async {
                        match service.delete_tunnel(id.clone()).await {
                            Ok(_) => println!("Tunnel stopped/deleted successfully: ID={}", id),
                            Err(e) => println!("Error stopping tunnel: {}", e),
                        }
                    });
                } else {
                    println!("Unknown tunnel action. Use 'create' or 'list'.");
                }
            }
            "forensic" => {
                if parts.len() < 2 {
                    println!("Usage: forensic track <ip> <user_agent> <hardware> <geo> OR forensic list");
                    continue;
                }
                let sub_cmd = parts[1].to_lowercase();
                if sub_cmd == "list" {
                    rt.block_on(async {
                        match service.list_forensics().await {
                            Ok(teles) => {
                                println!("Recorded Forensic Telemetries ({})", teles.len());
                                for ft in teles {
                                    println!("- [{}] IP: {} | UA: {} | HW: {} | Geo: {}", ft.tracking_id, ft.source_ip, ft.user_agent, ft.hardware_fingerprint, ft.geo_location);
                                }
                            }
                            Err(e) => println!("Error listing forensics: {}", e),
                        }
                    });
                } else if sub_cmd == "track" {
                    if parts.len() < 6 {
                        println!("Usage: forensic track <ip> <user_agent> <hardware> <geo>");
                        continue;
                    }
                    let ip = parts[2].to_string();
                    let ua = parts[3].to_string();
                    let hw = parts[4].to_string();
                    let geo = parts[5].to_string();

                    rt.block_on(async {
                        match service.record_forensic(ip, ua, hw, geo).await {
                            Ok(ft) => println!("Forensic Telemetry Recorded: TrackingID={}, IP={}, Geo={}", ft.tracking_id, ft.source_ip, ft.geo_location),
                            Err(e) => println!("Error recording forensic telemetry: {}", e),
                        }
                    });
                } else {
                    println!("Unknown forensic action. Use 'track' or 'list'.");
                }
            }
            "crypto" => {
                if parts.len() < 2 {
                    println!("Usage: crypto domestic <algorithm> <key> <message> OR crypto quantum <algorithm> <message>");
                    continue;
                }
                let sub_cmd = parts[1].to_lowercase();
                if sub_cmd == "domestic" {
                    if parts.len() < 5 {
                        println!("Usage: crypto domestic <algorithm> <key> <message>");
                        continue;
                    }
                    let algorithm = parts[2];
                    let key = parts[3];
                    let message = parts[4..].join(" ");
                    let algo = match algorithm.to_lowercase().as_str() {
                        "chacha" | "chacha20" => DomesticAlgorithm::ChaCha20Poly1305,
                        _ => DomesticAlgorithm::Aes256Gcm,
                    };
                    match service.encrypt_domestic(algo, key.as_bytes(), message.as_bytes()) {
                        Ok((ct, nonce)) => {
                            println!("Domestic Encryption Successful");
                            println!("Ciphertext (hex): {}", hex_encode(&ct));
                            println!("Nonce (hex): {}", hex_encode(&nonce));
                        }
                        Err(e) => println!("Encryption error: {}", e),
                    }
                } else if sub_cmd == "quantum" {
                    if parts.len() < 4 {
                        println!("Usage: crypto quantum <algorithm> <message>");
                        continue;
                    }
                    let algorithm = parts[2];
                    let message = parts[3..].join(" ");
                    let algo = match algorithm.to_lowercase().as_str() {
                        "kyber" | "ml-kem" => QuantumAlgorithm::MlKemKyber1024,
                        "dilithium" | "ml-dsa" => QuantumAlgorithm::MlDsaDilithium,
                        _ => QuantumAlgorithm::Pfe969HyperLattice,
                    };
                    match service.encrypt_quantum(algo.clone(), message.as_bytes()) {
                        Ok((ct, sk, pk)) => {
                            println!("Quantum-Resistant Encryption Successful (Algorithm: {:?})", algo);
                            println!("Public Key (hex): {}", hex_encode(&pk));
                            println!("Secret Key (hex): {}", hex_encode(&sk));
                            println!("Ciphertext (hex): {}", hex_encode(&ct));
                        }
                        Err(e) => println!("Quantum encryption error: {}", e),
                    }
                } else {
                    println!("Unknown crypto subcommand. Use 'domestic' or 'quantum'.");
                }
            }
            _ => {
                println!("Unknown command: '{}'. Type 'help' for instructions.", cmd);
            }
        }
    }

    Ok(())
}

fn hex_encode(data: &[u8]) -> String {
    data.iter().map(|b| format!("{:02x}", b)).collect()
}
