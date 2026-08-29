# Iori The Apacher

`iori-the-apacher` is a high-performance, multi-purpose forensic tunneling, secure DNS resolution, and post-quantum cryptographic management tool inspired by Apache software architecture, NGINX, NGROK, and Grabify. Designed for both domestic computing and advanced quantum research.

---

## Architecture & Features

1. **Hexagonal Architecture**: Clean separation between domain models, application ports, adapters (SQLite / SurrealDB storage, network tunneling), and CLI interfaces.
2. **Cybersecurity Mindset (360° Professional)**: Built with rigorous security practices, supporting Red/Blue/Purple team telemetry analysis, black/white/grey-box audit readiness.
3. **Dual Cryptography Submenus**:
   - **Domestic / Corp Ciphers**: AES-256-GCM, ChaCha20-Poly1305, RSA-4096, ECC P-384, Ed25519.
   - **Quantum-Resistant Ciphers**: ML-KEM (Kyber-1024), ML-DSA (Dilithium), and the novel **PFE-969** Hyper-Dimensional Lattice Cipher.
4. **Advanced Routing & Anonymity Suite**:
   - **Tor (.onion v3)**, **I2P**, and **Freenet SST** protocol integration options.
   - **Multi-Hop Relay Chaining** & **Proxychains** support (Strict, Dynamic, Random chains).
   - **Crypto Pre-Launch Verification**: Enforces mandatory cryptographic protection before public internet server deployment.
5. **Operational Forensic Grabify Suite**: Real-time IP tracking, risk scoring, User-Agent bot detection, Tor exit node identification, hardware fingerprinting, and geolocation resolution.
6. **Modern Web Management Console**: Built with Node.js, TypeScript, Vite, React, and Tailwind CSS.
7. **Enhanced Interactive REPL Shell**:
   - **Persistent History**: Command history saved across sessions (`.iori_history`).
   - **OS Command Execution**: Executes native Linux commands (e.g., `ls`, `pwd`) directly, secured against shell injection.

---

## Step-by-Step Installation & Walkthrough

### Prerequisites
- **Rust Toolchain** (v1.75+ or newer with Cargo)
- **Node.js** (v18+ or v20+ recommended) & **npm**

---

### Part 1: Running the CLI & Advanced Routing Mode

1. **Build the Rust Release Binary**:
   ```bash
   cargo build --release
   ```

2. **Launch the Interactive Shell**:
   ```bash
   target/release/iori-the-apacher shell
   ```
   *Within the shell, you can use built-in app commands or execute native OS commands (e.g., `ls -la`, `pwd`). Command history is maintained.*

3. **Launch a Secure Server with Tor & PFE-969 Quantum Protection**:
   ```bash
   target/release/iori-the-apacher server launch --subdomain secapp --port 8080 --protocol tor --crypto pfe969 --hops "relay1,relay2" --proxychains true --public true
   ```

---

## Documentation Index
- [Changelog](CHANGELOG.md)
- [Frontend Changelog](FRONTEND-CHANGELOG.md)
- [PFE-969 Crypto Study](PFE-969-CRYPTO-CYPHER-STUDY.md)
- [Database Models](DATABASE-MODELS.md)
- [Interactive CLI Guide](INTERACTIVE-CLI-GUIDE.md)
- [Manual Testing Guide](MANUAL-TESTING.md)
