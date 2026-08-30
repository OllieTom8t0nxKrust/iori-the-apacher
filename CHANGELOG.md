# Changelog - Iori The Apacher

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to Semantic Versioning.

## [1.5.0] - 2026-08-30

### Removed
- **Forensic Intelligence & Grabify Suite**: Completely removed the forensic module, database table (`forensics`), domain model, CLI subcommands (`forensic`), and frontend UI components (`ForensicTracker.tsx`), streamlining the tool strictly around secure networking, tunnels, and post-quantum cryptography.

### Remade
- **Database Architecture**: Remade and cleaned up the SQLite database schema (`SqliteStorageAdapter`), retaining exclusively the core tables: `tunnels`, `crypto_vault`, and `server_launches`.

## [1.4.0] - 2026-08-30

### Added
- **Top-Level Create Command**: Added top-level `create` command shortcut alongside `tunnel create` for streamlined tunnel provisioning.
- **Google Maps Geolocation Compatibility**: Enabled `allow_hyphen_values = true` on forensic tracking (`track` and `update`), supporting negative coordinate formatting (e.g. `-23.6688938,-46.6801928`).
- **SQLite Schema Auto-Migration**: Added automatic column checks (`ALTER TABLE`) to SQLite storage adapter for seamless backward compatibility on existing databases.
- **Interactive REPL App Command Routing & Help**: Implemented direct app command dispatch (`tunnel`, `forensic`, `crypto`, `server`, `create`, `help`) inside the interactive shell, eliminating external execution errors. Added a dedicated interactive shell help directory.
- **Randomized ASCII Art Startup Banners**: Added 3 distinct ASCII art banners featuring "IORI THE APACHER" that randomly cycle upon launching the interactive shell.
- **Robust Network Launcher & Tor Support**: Enhanced `NetworkLauncher` with graceful fallback handling and notice logging for `tor` and `proxychains` daemon execution.
- **Comprehensive README Usage Examples**: Added step-by-step usage examples for every project feature in `README.md`.

## [1.3.0] - 2026-08-30

### Added
- **Enhanced Interactive REPL Shell**: Integrated `rustyline` for persistent command history across sessions (`.iori_history`).
- **OS Command Execution**: Enabled native Linux command execution (e.g., `ls`, `pwd`) directly from the interactive shell.
- **Security-First Execution Design**: Implemented command execution without shell invocation (direct binary spawning), preventing common shell injection vulnerabilities (piping/redirection is intentionally disabled).

## [1.2.0] - 2026-08-29

### Added
- **Advanced Routing & Anonymity Suite**: Added Tor (.onion v3), I2P stream, and Freenet SST protocol integration options for server tunneling and routing.
- **Multi-Hop & Proxychains Integration**: Added multi-hop relay node chaining and Proxychains configuration (Strict, Dynamic, Random chains) for enhanced investigation security.
- **Crypto Pre-Launch Verification**: Enforced mandatory cryptographic protection (AES, ChaCha, Kyber, Dilithium, PFE-969) before launching servers to the public internet.
- **Operational Forensic Grabify Suite**: Removed all simulation/mock stubs, replacing them with real operational risk scoring, User-Agent bot detection, Tor exit node recognition, and anomaly analytics.
- **Full CLI & REPL Parity**: Added `server launch`, `server list`, and `server delete` commands to both non-interactive CLI and interactive REPL shell with detailed help documentation.

## [1.1.0] - 2026-08-28

### Added
- Fully functional interactive CLI shell REPL supporting complete CRUD operations (create, list, stop/delete tunnels, track/list forensics, domestic and quantum cryptography).
- Distinct exit commands: `exit` (secure shutdown closing active ports and clearing resources) and `exit-background` / detach (exiting session while keeping services running in background).
- Consolidated "Crypto Vault" unified menu item.
- Official Caboclo-Iori logo asset integration with 30-second lightning bolt energization and Raiden eyes animation.
- Extracted Caboclo-Iori color palette across project styling and Tailwind config.

### Removed
- All simulation/mock pre-populated initial data (tunnels and forensic hits start clean).
- Random mock hashes replaced with real Web Crypto API SHA-256 calculation.

## [1.0.0] - 2026-08-28

### Added
- Initial release of `iori-the-apacher` CLI tool built in Rust with Hexagonal Architecture.
- Hexagonal domain models for Tunnels, Forensic Telemetries, Domestic Ciphers (AES-256-GCM, ChaCha20-Poly1305), and Quantum Ciphers (ML-KEM/Kyber-1024, ML-DSA/Dilithium, and novel **PFE-969** Hyper-Dimensional Lattice Cipher).
- SQLite storage adapter implementing physical and logical database schemas for high performance and reliability across binary and quantum architectures.
- Comprehensive unit and integration test suite covering cryptographic encryption/decryption flows and application service orchestration.
- CLI subcommands supporting tunnel creation/listing, forensic telemetry tracking, and cryptographic operations.
- Scientific study document for **PFE-969** quantum cryptographic cipher (`PFE-969-CRYPTO-CYPHER-STUDY.md`).
- Database physical and logical model documentation (`DATABASE-MODELS.md`).
- Modern React + TypeScript web management console with NGROK and Grabify inspired UI/UX.
