# 🛡️ IORI THE APACHER // Enterprise Post-Quantum Secure Tunneling & Cryptographic Suite

`iori-the-apacher` is an elite, high-performance, multi-purpose secure network tunneling, encrypted DNS resolution, and post-quantum cryptographic management suite inspired by legendary Apache software architecture, NGINX reliability, and NGROK agility. Engineered meticulously for both domestic enterprise computing and advanced quantum research environments.

---

## 🏛️ Comprehensive Architecture & Advanced Features

1. **Hexagonal Domain-Driven Architecture**:
   - **Domain Layer**: Core business logic encapsulating Tunnels (`tunnels`), Cryptographic Vault (`crypto_vault`), and Server Launches (`server_launches`) with zero external coupling.
   - **Application Ports & Services**: Clean dependency inversion defining storage and network launch boundaries.
   - **Adapters**: SQLite physical persistence (`SqliteStorageAdapter`), robust network launcher (`NetworkLauncher`), and REST API server (`ApiServer`).
   - **CLI & REPL Shell**: Interactive REPL with persistent history (`.iori_history`), randomized startup ASCII banners, and safe OS process spawning.

2. **Cybersecurity Mindset (360° Professional Red/Blue/Purple Readiness)**:
   - Built to withstand rigorous black/white/grey-box audits.
   - Strict input validation, zero-shell-injection execution model, and mandatory pre-launch cryptographic verification.

3. **Dual Cryptography Submenus & Post-Quantum Vault**:
   - **Domestic / Corporate Ciphers**: AES-256-GCM, ChaCha20-Poly1305, RSA-4096, ECC P-384, Ed25519.
   - **Quantum-Resistant Ciphers**: ML-KEM (Kyber-1024), ML-DSA (Dilithium), and the proprietary **PFE-969** Hyper-Dimensional Lattice Cipher.
   - **Unified Crypto Vault**: Securely stores public keys and encrypted secret keys in SQLite with cryptographic metadata tagging.

4. **Advanced Routing, Anonymity & Onion Network Suite**:
   - **Tor (.onion v3)** hidden services integration with automated ephemeral directory provisioning.
   - **I2P** stream and **Freenet SST** protocol routing support.
   - **Multi-Hop Relay Chaining** & **Proxychains** integration (Strict, Dynamic, and Random chains).
   - **Crypto Pre-Launch Verification**: Enforces cryptographic cipher validation before allowing public internet server binding.

5. **Modern React Web Management Console**:
   - Built with Node.js, TypeScript, Vite, React, and Tailwind CSS.
   - Glassmorphism dark-mode control panel featuring real-time tunnel monitoring, server launcher management, quantum cipher validation, and Caboclo-Iori branding assets.
   - **Responsive UX**: Fully responsive sidebar with collapsible design featuring an interactive skull transformation logo, optimized for desktop and mobile devices.

6. **Enhanced Interactive REPL Shell**:
   - **Persistent Command History**: Automatically saves and loads session history across restarts (`.iori_history`).
   - **Direct Binary Execution**: Executes native Linux system commands (`ls`, `pwd`, `ps`) securely as direct child processes without shell interpreter injection vulnerabilities.

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
