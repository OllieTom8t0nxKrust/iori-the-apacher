# IORI THE APACHER

`iori-the-apacher` is a high-performance, multi-purpose secure network tunneling, secure DNS resolution, and post-quantum cryptographic management tool inspired by Apache software architecture, NGINX, and NGROK. Designed for both domestic computing and advanced quantum research.

---

## Architecture & Features

1. **Hexagonal Architecture**: Clean separation between domain models, application ports, adapters (SQLite storage, network tunneling), and CLI interfaces.
2. **Cybersecurity Mindset (360° Professional)**: Built with rigorous security practices, supporting Red/Blue/Purple team analysis, black/white/grey-box audit readiness.
3. **Dual Cryptography Submenus**:
   - **Domestic / Corp Ciphers**: AES-256-GCM, ChaCha20-Poly1305, RSA-4096, ECC P-384, Ed25519.
   - **Quantum-Resistant Ciphers**: ML-KEM (Kyber-1024), ML-DSA (Dilithium), and the novel **PFE-969** Hyper-Dimensional Lattice Cipher.
4. **Advanced Routing & Anonymity Suite**:
   - **Tor (.onion v3)**, **I2P**, and **Freenet SST** protocol integration options.
   - **Multi-Hop Relay Chaining** & **Proxychains** support (Strict, Dynamic, Random chains).
   - **Crypto Pre-Launch Verification**: Enforces mandatory cryptographic protection before public internet server deployment.
5. **Modern Web Management Console**: Built with Node.js, TypeScript, Vite, React, and Tailwind CSS.
6. **Enhanced Interactive REPL Shell**:
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

2. **Launch the Interactive Shell (with Random ASCII Art Banners)**:
   ```bash
   target/release/iori-the-apacher shell
   ```
    *Within the shell, use built-in app commands (`tunnel`, `crypto`, `server`, `create`, `help`) or execute native OS commands (e.g., `ls -la`, `pwd`). Command history is maintained.*

3. **Feature Usage Examples**:

   - **Network Tunnels (`tunnel` / `create`)**:
     Create and persist an NGINX/NGROK-style secure network tunnel session:
     ```bash
     target/release/iori-the-apacher create --subdomain my-app --port 8080 --protocol http
     target/release/iori-the-apacher tunnel list
     target/release/iori-the-apacher tunnel get --id <TUNNEL_ID>
     ```

   - **Dual Cryptography & Secure Vault (`crypto`)**:
     Encrypt data with domestic ciphers (AES/ChaCha) or post-quantum lattice cryptography (PFE-969/Kyber/Dilithium), and manage vault records:
     ```bash
     target/release/iori-the-apacher crypto domestic --algorithm aes --key "my-secret-key-32bytes-long-string!!" --message "confidential payload"
     target/release/iori-the-apacher crypto quantum --algorithm pfe969 --message "quantum secured message"
     target/release/iori-the-apacher crypto vault-save --algorithm pfe969 --ciphertext "deadbeef" --key "feedface" --metadata "Key vault entry #1"
     target/release/iori-the-apacher crypto vault-list
     ```

   - **Secure Server Launch with Tor / I2P & PQC Verification (`server`)**:
     Launch secure servers with Tor (.onion v3), multi-hop relay chaining, proxychains, and mandatory cryptographic pre-launch verification:
     ```bash
     target/release/iori-the-apacher server launch --subdomain secapp --port 8080 --protocol tor --crypto pfe969 --hops "node1,node2" --proxychains true --public true
     target/release/iori-the-apacher server list
     ```

   - **Persistent SQLite Database Storage (`--db`)**:
     Specify custom database paths across sessions:
     ```bash
     target/release/iori-the-apacher -d custom_apacher.db tunnel list
     ```

---

## Documentation Index
- [Changelog](CHANGELOG.md)
- [Frontend Changelog](FRONTEND-CHANGELOG.md)
- [PFE-969 Crypto Study](PFE-969-CRYPTO-CYPHER-STUDY.md)
- [Database Models](DATABASE-MODELS.md)
- [Interactive CLI Guide](INTERACTIVE-CLI-GUIDE.md)
- [Manual Testing Guide](MANUAL-TESTING.md)
