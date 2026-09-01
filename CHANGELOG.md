# Master Changelog - IORI THE APACHER // Enterprise Post-Quantum Secure Tunneling & Cryptographic Suite

All notable changes to the master codebase (backend Rust core and frontend web panel) are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to Semantic Versioning.

## [1.6.3] - 2026-09-01

### Fixed & Refined (CLI & Frontend)
- **Curated ASCII Art Banner**: Replaced randomized startup banners with a single curated ASCII art banner featuring "IORI THE APACHER", correcting the letter `E` in `THE` to ensure a fully formed capital `E` (fixing previous incomplete rendering resembling `THF`).
- **Logo & Electric Animations**: Upgraded `CabocloIoriLogo` with interactive click-to-energize capabilities, persistent lightning flash effects across both expanded and collapsed modes, and refined sidebar branding hierarchy.

## [1.6.2] - 2026-09-01

### Fixed (Frontend)
- **Logo Proportions**: Adjusted `CabocloIoriLogo` sizing (56px collapsed / 96px expanded) and sidebar container padding for improved visual hierarchy and UI consistency.

### Fixed (CLI)
- **Prompt Consistency**: Updated interactive shell prompt from `iori-apacher>` to `iori-the-apacher>` for consistent project branding.

## [1.6.1] - 2026-09-01

### Fixed (Frontend)
- **Logo Optimization & Cleanup**: Processed and formatted logo assets (`caboclo-iori.jpg` and `caboclo-iori-skull.png`) to 1:1 square aspect ratio with entirely black backgrounds, centered indigenous figures, and removed the Gemini star watermark.

## [1.6.0] - 2026-08-31

### Added (Frontend)
- **Responsive UI & Sidebar Toggle**: Added sidebar collapsibility for improved screen real-estate management, optimized for all screen sizes including smartphones.
- **Dynamic Logo Interaction**: Integrated interactive logo functionality where clicking the logo toggles sidebar state and visually transforms the logo into a white skull with glowing red eyes when collapsed.

## [1.5.0] - 2026-08-30

### Removed (Backend & Frontend)
- **Forensic Intelligence & Grabify Suite**: Completely decommissioned and removed the forensic module, database table (`forensics`), domain model, CLI subcommands (`forensic`), and frontend UI components (`ForensicTracker.tsx`), refining the suite strictly around secure networking, tunnels, and post-quantum cryptography.

### Remade (Backend)
- **Database Architecture**: Streamlined SQLite database schema (`SqliteStorageAdapter`), retaining exclusively the core tables: `tunnels`, `crypto_vault`, and `server_launches`. Added automatic column migration (`ALTER TABLE`) for seamless backward compatibility.

## [1.4.0] - 2026-08-30

### Added
- **Top-Level Create Command**: Added top-level `create` command shortcut alongside `tunnel create` for streamlined tunnel provisioning.
- **SQLite Schema Auto-Migration**: Added automatic column checks (`ALTER TABLE`) to SQLite storage adapter for seamless backward compatibility on existing databases.
- **Interactive REPL App Command Routing & Help**: Implemented direct app command dispatch (`tunnel`, `crypto`, `server`, `create`, `help`) inside the interactive shell.
- **Randomized ASCII Art Startup Banners**: Added 3 distinct ASCII art banners featuring "IORI THE APACHER" that randomly cycle upon launching the interactive shell.
- **Robust Network Launcher & Tor Support**: Enhanced `NetworkLauncher` with graceful fallback handling and notice logging for `tor` and `proxychains` daemon execution.
- **Comprehensive README Usage Examples**: Added step-by-step usage examples for every project feature in `README.md`.

## [1.3.0] - 2026-08-30

### Added
- **Enhanced Interactive REPL Shell**: Integrated `rustyline` for persistent command history across sessions (`.iori_history`).
- **OS Command Execution**: Enabled native Linux command execution (`ls`, `pwd`) directly from the interactive shell.
- **Security-First Execution Design**: Implemented command execution without shell invocation (direct binary spawning), preventing common shell injection vulnerabilities.

## [1.2.0] - 2026-08-29

### Added
- **Advanced Routing & Anonymity Suite**: Added Tor (.onion v3), I2P stream, and Freenet SST protocol integration options for server tunneling and routing.
- **Multi-Hop & Proxychains Integration**: Added multi-hop relay node chaining and Proxychains configuration (Strict, Dynamic, Random chains) for enhanced investigation security.
- **Crypto Pre-Launch Verification**: Enforced mandatory cryptographic protection (AES, ChaCha, Kyber, Dilithium, PFE-969) before launching servers to the public internet.
- **Full CLI & REPL Parity**: Added `server launch`, `server list`, and `server delete` commands to both non-interactive CLI and interactive REPL shell with detailed help documentation.

## [1.1.0] - 2026-08-28

### Added
- Fully functional interactive CLI shell REPL supporting complete CRUD operations (create, list, stop/delete tunnels, domestic and quantum cryptography).
- Distinct exit commands: `exit` (secure shutdown closing active ports and clearing resources) and `exit-background` / detach.
- Consolidated "Crypto Vault" unified menu item.
- Official Caboclo-Iori logo asset integration with 30-second lightning bolt energization and Raiden eyes animation.
- Extracted Caboclo-Iori color palette across project styling and Tailwind config.

## [1.0.0] - 2026-08-28

### Added
- Initial release of `iori-the-apacher` CLI tool built in Rust with Hexagonal Architecture.
- Hexagonal domain models for Tunnels, Domestic Ciphers (AES-256-GCM, ChaCha20-Poly1305), and Quantum Ciphers (ML-KEM/Kyber-1024, ML-DSA/Dilithium, and novel **PFE-969** Hyper-Dimensional Lattice Cipher).
- SQLite storage adapter implementing physical and logical database schemas for high performance and reliability.
- Comprehensive unit and integration test suite covering cryptographic encryption/decryption flows and application service orchestration.
- Modern React + TypeScript web management console with NGROK inspired UI/UX.
