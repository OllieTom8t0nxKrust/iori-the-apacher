use clap::{Subcommand};

#[derive(Subcommand, Debug, Clone)]
pub enum ShellCommand {
    Tunnel {
        #[command(subcommand)]
        action: TunnelShellAction,
    },
    Forensic {
        #[command(subcommand)]
        action: ForensicShellAction,
    },
    Crypto {
        #[command(subcommand)]
        action: CryptoShellAction,
    },
    Help,
    ExitBackground,
    Exit,
}

#[derive(Subcommand, Debug, Clone)]
pub enum TunnelShellAction {
    Create {
        subdomain: String,
        port: u16,
        #[arg(default_value = "http")]
        protocol: String,
    },
    List,
    Stop {
        id: String,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum ForensicShellAction {
    Track {
        ip: String,
        user_agent: String,
        hardware: String,
        geo: String,
    },
    List,
}

#[derive(Subcommand, Debug, Clone)]
pub enum CryptoShellAction {
    Domestic {
        #[arg(default_value = "aes")]
        algorithm: String,
        key: String,
        message: String,
    },
    Quantum {
        #[arg(default_value = "pfe969")]
        algorithm: String,
        message: String,
    },
}
