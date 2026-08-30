use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "iori-the-apacher")]
#[command(about = "IORI THE APACHER - High-performance Apache/Ngrok/Grabify multi-purpose forensic tunneling & quantum crypto suite", long_about = "
IORI THE APACHER is a comprehensive multi-purpose cryptographic and forensic tunneling tool.
It supports full SQLite database CRUD operations for tunnels, forensic telemetry, cryptographic vault records,
and secure server launching with Tor, I2P, Freenet, multi-hop routing, proxychains, and mandatory PQC crypto verification.
")]
pub struct CliArgs {
    #[arg(short, long, default_value = "iori_apacher.db", help = "Path to the SQLite database file used for persistent storage across sessions")]
    pub db: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Manage NGINX/NGROK-style secure network tunnels and custom DNS routing")]
    Tunnel {
        #[command(subcommand)]
        action: TunnelAction,
    },
    #[command(about = "Manage domestic enterprise ciphers (AES/ChaCha) and post-quantum lattice cryptography (PFE-969/Kyber/Dilithium) + Vault CRUD")]
    Crypto {
        #[command(subcommand)]
        action: CryptoAction,
    },
    #[command(about = "Launch secure servers with Tor, I2P, Freenet, multi-hop routing, proxychains, and crypto verification")]
    Server {
        #[command(subcommand)]
        action: ServerAction,
    },
    #[command(about = "Create and persist a new network tunnel session (top-level shortcut)")]
    Create {
        #[arg(short, long, help = "Subdomain prefix for the tunnel endpoint (e.g. 'my-app')")]
        subdomain: String,
        #[arg(short, long, help = "Local target port to proxy traffic to (e.g. 8080)")]
        port: u16,
        #[arg(short, long, default_value = "http", help = "Protocol type: http, https, quic, or tcp")]
        protocol: String,
    },
    #[command(about = "Launch the interactive REPL shell for continuous command execution with active background state")]
    Shell,
}

#[derive(Subcommand, Debug)]
pub enum TunnelAction {
    #[command(about = "Create and persist a new network tunnel session")]
    Create {
        #[arg(short, long, help = "Subdomain prefix for the tunnel endpoint (e.g. 'my-app')")]
        subdomain: String,
        #[arg(short, long, help = "Local target port to proxy traffic to (e.g. 8080)")]
        port: u16,
        #[arg(short, long, default_value = "http", help = "Protocol type: http, https, quic, or tcp")]
        protocol: String,
    },
    #[command(about = "List all active and stored tunnel sessions from the database")]
    List,
    #[command(about = "Retrieve details of a specific tunnel session by its unique ID")]
    Get {
        #[arg(short, long, help = "Unique UUID of the tunnel session")]
        id: String,
    },
    #[command(about = "Update an existing tunnel session configuration")]
    Update {
        #[arg(short, long, help = "Unique UUID of the tunnel session to update")]
        id: String,
        #[arg(short, long, help = "Updated subdomain name")]
        subdomain: String,
        #[arg(short, long, help = "Updated local target port")]
        port: u16,
        #[arg(short, long, default_value = "http", help = "Updated protocol")]
        protocol: String,
        #[arg(short, long, default_value = "true", help = "Active state status (true/false)")]
        active: bool,
    },
    #[command(about = "Delete/stop a tunnel session from the database")]
    Delete {
        #[arg(short, long, help = "Unique UUID of the tunnel session to delete")]
        id: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum CryptoAction {
    #[command(about = "Execute domestic enterprise encryption (AES-256-GCM / ChaCha20)")]
    Domestic {
        #[arg(short, long, default_value = "aes", help = "Algorithm: 'aes' or 'chacha'")]
        algorithm: String,
        #[arg(short, long, help = "Encryption key string")]
        key: String,
        #[arg(short, long, help = "Plaintext message payload to encrypt")]
        message: String,
    },
    #[command(about = "Execute quantum-resistant encryption (PFE-969 Lattice / Kyber / Dilithium)")]
    Quantum {
        #[arg(short, long, default_value = "pfe969", help = "Quantum algorithm: 'pfe969', 'kyber', or 'dilithium'")]
        algorithm: String,
        #[arg(short, long, help = "Plaintext message payload to protect")]
        message: String,
    },
    #[command(about = "Save/persist a cryptographic record into the secure Vault database")]
    VaultSave {
        #[arg(short, long, help = "Algorithm identifier name")]
        algorithm: String,
        #[arg(short, long, help = "Hex-encoded ciphertext string")]
        ciphertext: String,
        #[arg(short, long, help = "Hex-encoded key string")]
        key: String,
        #[arg(short, long, default_value = "Standard Vault Entry", help = "Metadata or description")]
        metadata: String,
    },
    #[command(about = "List all stored cryptographic vault records from the database")]
    VaultList,
    #[command(about = "Retrieve a specific cryptographic vault record by its ID")]
    VaultGet {
        #[arg(short, long, help = "Unique UUID of the vault record")]
        id: String,
    },
    #[command(about = "Delete a cryptographic vault record from the database")]
    VaultDelete {
        #[arg(short, long, help = "Unique UUID of the vault record to delete")]
        id: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum ServerAction {
    #[command(about = "Launch a secure server with Tor, I2P, Freenet, multi-hop routing, and pre-launch crypto verification")]
    Launch {
        #[arg(short, long, help = "Subdomain or service name")]
        subdomain: String,
        #[arg(short, long, help = "Local port")]
        port: u16,
        #[arg(short, long, default_value = "http", help = "Protocol: http, https, quic, tcp, tor, i2p, freenet")]
        protocol: String,
        #[arg(short, long, default_value = "pfe969", help = "Crypto requirement: none, aes, chacha, kyber, dilithium, pfe969")]
        crypto: String,
        #[arg(short, long, default_value = "", help = "Multi-hop relay nodes separated by comma (e.g. 'node1,node2')")]
        hops: String,
        #[arg(short, long, default_value = "false", help = "Enable proxychains routing (true/false)")]
        proxychains: bool,
        #[arg(short, long, default_value = "true", help = "Public internet launch flag (requires crypto verification) (true/false)")]
        public: bool,
    },
    #[command(about = "List all launched servers")]
    List,
    #[command(about = "Retrieve details of a specific server launch by ID")]
    Get {
        #[arg(short, long, help = "Unique UUID of the launched server")]
        id: String,
    },
    #[command(about = "Update an existing server launch configuration")]
    Update {
        #[arg(short, long, help = "Unique UUID of the server launch to update")]
        id: String,
        #[arg(short, long, help = "Updated subdomain or service name")]
        subdomain: String,
        #[arg(short, long, help = "Updated local port")]
        port: u16,
        #[arg(short, long, default_value = "http", help = "Updated protocol: http, https, quic, tcp, tor, i2p, freenet")]
        protocol: String,
        #[arg(short, long, default_value = "pfe969", help = "Updated crypto requirement")]
        crypto: String,
        #[arg(short, long, default_value = "", help = "Updated multi-hop relay nodes (comma-separated)")]
        hops: String,
        #[arg(short, long, default_value = "false", help = "Updated proxychains routing flag")]
        proxychains: bool,
        #[arg(short, long, default_value = "true", help = "Updated public internet launch flag")]
        public: bool,
    },
    #[command(about = "Delete/stop a launched server")]
    Delete {
        #[arg(short, long, help = "Unique UUID of the launched server")]
        id: String,
    },
}
