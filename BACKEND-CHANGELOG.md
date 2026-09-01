# Backend Changelog - IORI THE APACHER // Rust Core & Hexagonal Architecture

All notable changes to the backend Rust core engine, domain models, ports, adapters, SQLite persistence layer, interactive REPL shell, and REST API server are documented in this file.

## [1.5.1] - 2026-09-01

### Fixed (CLI REPL Shell)
- **Curated ASCII Art Banner**: Replaced randomized startup banners with a single curated and corrected ASCII art banner featuring "IORI THE APACHER" with the letter `E` correctly included in `THE`.

## [1.5.0] - 2026-08-30

### Removed
- **Forensic Intelligence Module**: Completely decommissioned the forensic telemetry domain model, database table (`forensics`), and related execution handlers, narrowing the backend domain strictly to secure network tunneling, server launches, and post-quantum cryptographic vault management.

### Remade
- **SQLite Storage Schema (`SqliteStorageAdapter`)**: Cleaned and optimized database migration scripts, retaining exclusively the core tables: `tunnels`, `crypto_vault`, and `server_launches`. Added automatic column migration checks (`ALTER TABLE`) ensuring zero-downtime upgrades.

## [1.4.0] - 2026-08-30

### Added
- **Top-Level Create Command**: Implemented top-level `create` command parser shortcut alongside `tunnel create` for streamlined network tunnel provisioning.
- **Interactive REPL App Command Routing & Help**: Implemented direct command dispatch (`tunnel`, `crypto`, `server`, `create`, `help`) within the interactive shell REPL with comprehensive built-in help text.
- **Randomized ASCII Art Startup Banners**: Added 3 distinct ASCII art banners featuring "IORI THE APACHER" that randomly cycle upon launching the interactive shell.
- **Robust Network Launcher (`NetworkLauncher`)**: Enhanced background process management and fallback handling for Tor, I2P, and Proxychains daemon execution.

## [1.3.0] - 2026-08-30

### Added
- **Persistent REPL Command History**: Integrated `rustyline` library to automatically persist and load command history across terminal sessions (`.iori_history`).
- **Secure OS Command Execution**: Enabled native Linux command execution (`ls`, `pwd`, `ps`) directly from the interactive shell.
- **Security-First Process Spawning**: Designed OS command execution using direct child process spawning (`std::process::Command`) without shell interpretation (`/bin/sh` or `/bin/bash`), entirely eliminating shell injection vulnerabilities while intentionally restricting shell pipes/redirection.

## [1.2.0] - 2026-08-29

### Added
- **Advanced Routing & Anonymity Suite**: Added backend protocol handlers for Tor (.onion v3), I2P streams, and Freenet SST.
- **Multi-Hop Relay & Proxychains**: Added multi-hop relay node chaining and Proxychains configuration support (Strict, Dynamic, Random chains).
- **Crypto Pre-Launch Verification**: Enforced strict cryptographic protection checks before binding network servers to public interfaces.
- **Full CLI & REPL Parity**: Implemented `server launch`, `server list`, and `server delete` command handlers across non-interactive CLI arguments and the interactive REPL shell.

## [1.1.0] - 2026-08-28

### Added
- Fully functional interactive CLI shell REPL supporting CRUD operations for tunnels and cryptographic vault.
- Distinct termination commands: `exit` (clean resource shutdown) and `exit-background` (daemon detach).
- Unified "Crypto Vault" management handlers in storage adapter and application services.

## [1.0.0] - 2026-08-28

### Added
- Initial release of `iori-the-apacher` core engine built in Rust adhering to Hexagonal Architecture.
- Domain models for Tunnels, Domestic Ciphers (AES-256-GCM, ChaCha20-Poly1305), and Quantum Ciphers (ML-KEM/Kyber-1024, ML-DSA/Dilithium, and proprietary **PFE-969** Hyper-Dimensional Lattice Cipher).
- SQLite storage adapter providing ACID-compliant persistence.
- Comprehensive unit and integration test suites covering cryptographic operations and tunnel service coordination.
- REST API server adapter facilitating frontend web console communication.
