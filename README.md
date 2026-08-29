# Iori The Apacher

`iori-the-apacher` is a high-performance, multi-purpose forensic tunneling, secure DNS resolution, and post-quantum cryptographic management tool inspired by Apache software architecture, NGINX, NGROK, and Grabify. Designed for both domestic computing and advanced quantum research.

---

## Architecture & Features

1. **Hexagonal Architecture**: Clean separation between domain models, application ports, adapters (SQLite / SurrealDB storage, network tunneling), and CLI interfaces.
2. **Cybersecurity Mindset (360° Professional)**: Built with rigorous security practices, supporting Red/Blue/Purple team telemetry analysis, black/white/grey-box audit readiness.
3. **Dual Cryptography Submenus**:
   - **Domestic / Corp Ciphers**: AES-256-GCM, ChaCha20-Poly1305, RSA-4096, ECC P-384, Ed25519.
   - **Quantum-Resistant Ciphers**: ML-KEM (Kyber-1024), ML-DSA (Dilithium), and the novel **PFE-969** Hyper-Dimensional Lattice Cipher.
4. **Forensic Telemetry & Tunnels**: Grabify-inspired real-time IP tracking, hardware fingerprinting, geolocation resolution, and NGROK-style secure tunneling over HTTP/HTTPS/QUIC.
5. **Modern Web Management Console**: Built with Node.js, TypeScript, Vite, React, and Tailwind CSS, featuring a beautiful dark-mode glassmorphism interface inspired by NGROK and Grabify.

---

## Step-by-Step Installation & Walkthrough

### Prerequisites
- **Rust Toolchain** (v1.75+ or newer with Cargo)
- **Node.js** (v18+ or v20+ recommended) & **npm**

---

### Part 1: Running the CLI Mode

1. **Build the Rust Release Binary**:
   ```bash
   cargo build --release
   ```

2. **Create an HTTP/HTTPS Tunnel**:
   ```bash
   target/release/iori-the-apacher tunnel create --subdomain myapp --port 8080 --protocol https
   ```

3. **List Active Tunnels**:
   ```bash
   target/release/iori-the-apacher tunnel list
   ```

4. **Record Forensic Telemetry (Grabify Style)**:
   target/release/iori-the-apacher forensic track --ip "192.168.1.100" --user-agent "Mozilla/5.0" --hardware "x86_64-avx512" --geo "Zurich, CH"

5. **List Recorded Forensic Telemetries**:
   ```bash
   target/release/iori-the-apacher forensic list
   ```

6. **Execute Domestic Encryption (AES-256-GCM)**:
   ```bash
   target/release/iori-the-apacher crypto domestic --algorithm aes --key "0123456789abcdef0123456789abcdef" --message "Classified Domestic Payload"
   ```

7. **Execute Quantum-Resistant Encryption (PFE-969 Lattice)**:
   ```bash
   target/release/iori-the-apacher crypto quantum --algorithm pfe969 --message "Classified Quantum-Resistant Payload"
   ```

---

### Part 2: Running the Web Frontend Panel

1. **Navigate to the Frontend Directory**:
   ```bash
   cd frontend
   ```

2. **Install Node Dependencies**:
   ```bash
   npm install
   ```

3. **Launch the Development Server**:
   ```bash
   npm run dev
   ```
   *The web console will be accessible at `http://localhost:3000` with live glassmorphism UI, tunneling management, forensic telemetry streams, and dual crypto submenus.*

4. **Build for Production**:
   ```bash
   npm run build
   ```

---

## Documentation Index
- [Changelog](CHANGELOG.md)
- [Frontend Changelog](FRONTEND-CHANGELOG.md)
- [PFE-969 Crypto Study](PFE-969-CRYPTO-CYPHER-STUDY.md)
- [Database Models](DATABASE-MODELS.md)
