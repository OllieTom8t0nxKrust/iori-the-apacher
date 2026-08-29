# Changelog - Iori The Apacher

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0/).

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
