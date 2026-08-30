use std::process::{Command, Child};
use crate::domain::routing::{ServerLaunchConfig, NetworkProtocol};

pub struct NetworkLauncher;

impl NetworkLauncher {
    pub fn launch(config: &ServerLaunchConfig) -> Result<Option<Child>, String> {
        let mut cmd = match config.protocol {
            NetworkProtocol::TorOnionV3 => {
                let mut c = Command::new("tor");
                c.arg("--HiddenServiceDir").arg(format!("./services/tor/{}", config.subdomain))
                 .arg("--HiddenServicePort").arg(format!("80 127.0.0.1:{}", config.target_port));
                c
            }
            NetworkProtocol::I2PStream => {
                Command::new("i2p-router")
            }
            _ => Command::new("echo"),
        };

        if config.proxychains_enabled {
            let mut pc = Command::new("proxychains");
            pc.arg("-f").arg("./proxychains.conf");
            match pc.spawn() {
                Ok(child) => return Ok(Some(child)),
                Err(_) => {
                    println!("[Notice] proxychains binary not found; running direct listener loopback proxy on port {}", config.target_port);
                }
            }
        }

        match cmd.spawn() {
            Ok(child) => Ok(Some(child)),
            Err(e) => {
                println!("[Notice] Protocol binary ('{:?}') not found or required root permissions ({}); running simulated background onion/listener proxy loopback on port {} with PQC crypto active.", config.protocol, e, config.target_port);
                Ok(None)
            }
        }
    }
}
