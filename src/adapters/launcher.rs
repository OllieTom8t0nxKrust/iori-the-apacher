use std::process::{Command, Child};
use crate::domain::routing::{ServerLaunchConfig, NetworkProtocol};

pub struct NetworkLauncher;

impl NetworkLauncher {
    pub fn launch(config: &ServerLaunchConfig) -> Result<Child, String> {
        let mut cmd = match config.protocol {
            NetworkProtocol::TorOnionV3 => {
                let mut c = Command::new("tor");
                c.arg("--HiddenServiceDir").arg(format!("./services/tor/{}", config.subdomain))
                 .arg("--HiddenServicePort").arg(format!("80 127.0.0.1:{}", config.target_port));
                c
            }
            NetworkProtocol::I2PStream => {
                // Simplified I2P launch wrapper
                Command::new("i2p-router")
            }
            _ => Command::new("echo"), // Placeholder for other protocols
        };

        if config.proxychains_enabled {
            cmd = Command::new("proxychains");
            cmd.arg("-f").arg("./proxychains.conf");
            // Wrap the actual launch command
        }

        cmd.spawn().map_err(|e| format!("Failed to launch network protocol: {}", e))
    }
}
