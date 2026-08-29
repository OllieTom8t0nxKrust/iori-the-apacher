use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "iori-the-apacher")]
#[command(about = "High-performance Apache/Ngrok/Grabify multi-purpose forensic tunneling & quantum crypto tool", long_about = None)]
pub struct CliArgs {
    #[arg(short, long, default_value = "iori_apacher.db")]
    pub db: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Tunnel {
        #[command(subcommand)]
        action: TunnelAction,
    },
    Forensic {
        #[command(subcommand)]
        action: ForensicAction,
    },
    Crypto {
        #[command(subcommand)]
        action: CryptoAction,
    },
    Shell,
}

#[derive(Subcommand, Debug)]
pub enum TunnelAction {
    Create {
        #[arg(short, long)]
        subdomain: String,
        #[arg(short, long)]
        port: u16,
        #[arg(short, long, default_value = "http")]
        protocol: String,
    },
    List,
}

#[derive(Subcommand, Debug)]
pub enum ForensicAction {
    Track {
        #[arg(short, long)]
        ip: String,
        #[arg(short, long)]
        user_agent: String,
        #[arg(short, long)]
        hardware: String,
        #[arg(short, long)]
        geo: String,
    },
    List,
}

#[derive(Subcommand, Debug)]
pub enum CryptoAction {
    Domestic {
        #[arg(short, long, default_value = "aes")]
        algorithm: String,
        #[arg(short, long)]
        key: String,
        #[arg(short, long)]
        message: String,
    },
    Quantum {
        #[arg(short, long, default_value = "pfe969")]
        algorithm: String,
        #[arg(short, long)]
        message: String,
    },
}
