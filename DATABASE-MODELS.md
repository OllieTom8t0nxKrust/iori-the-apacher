# Database Models & Architecture Study - IORI THE APACHER // Enterprise Post-Quantum Secure Tunneling & Cryptographic Suite

## 1. Database Technology Selection & Architecture
For `iori-the-apacher`, the database architecture fulfills two distinct operating environments:
1. **Domestic & Binary Computing**: High-speed embedded relational storage via SQLite (`Rusqlite`), requiring zero external daemons, ensuring ACID compliance, instantaneous startup, and cross-compilation reliability.
2. **Quantum Computing & Distributed Research**: Multi-model data structures capable of handling high-dimensional lattice cryptographic keys, quantum entanglement state metadata, and distributed synchronization.

### Selected Database Engine: SQLite Embedded (Domestic Core) & SurrealDB (Quantum Research Roadmap)
- **SQLite (Bundled via Rusqlite)**: Serves as our robust embedded domestic storage backend in `SqliteStorageAdapter`, providing zero-latency file-based storage with automated schema migration (`ALTER TABLE`).
- **SurrealDB (Hybrid & Multi-Model)**: Designated for advanced quantum research nodes, supporting document, graph, and relational paradigms with quantum-safe encryption at rest.

---

## 2. Logical Database Model

The logical schema consists of three core entity domains:

### Entity 1: Tunnels (`tunnels`)
- `id` (UUID Primary Key)
- `subdomain` (String, unique routing identifier for NGINX/NGROK style tunneling)
- `target_port` (Integer, local application port forwarding target)
- `protocol` (String, e.g., http, https, tcp, tor, i2p)
- `active` (Boolean, connection status indicator)
- `created_at` (Timestamp, RFC3339 formatted creation time)

### Entity 2: Cryptographic Vault (`crypto_vault`)
- `key_id` (UUID Primary Key)
- `algorithm_category` (Enum: `Domestic`, `Quantum`)
- `algorithm_name` (String, e.g., `aes-256-gcm`, `pfe-969`, `kyber-1024`, `dilithium`)
- `public_key` (Binary/Blob, cryptographic public key payload)
- `secret_key_encrypted` (Binary/Blob, master-key encrypted private/secret key payload)

### Entity 3: Server Launches (`server_launches`)
- `id` (UUID Primary Key)
- `subdomain` (String, target routing subdomain)
- `port` (Integer, target port)
- `protocol` (String, routing protocol: http, tor, i2p)
- `crypto_algorithm` (String, enforced encryption cipher)
- `hops` (String, multi-hop relay node chaining configuration)
- `proxychains` (Boolean, proxychains integration flag)
- `public` (Boolean, public internet exposure flag)
- `created_at` (Timestamp, RFC3339)

---

## 3. Physical Database Model (SQL DDL)

```sql
CREATE TABLE IF NOT EXISTS tunnels (
    id TEXT PRIMARY KEY,
    subdomain TEXT NOT NULL UNIQUE,
    target_port INTEGER NOT NULL,
    protocol TEXT NOT NULL,
    active BOOLEAN NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS crypto_vault (
    key_id TEXT PRIMARY KEY,
    algorithm_category TEXT NOT NULL,
    algorithm_name TEXT NOT NULL,
    public_key BLOB NOT NULL,
    secret_key_encrypted BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS server_launches (
    id TEXT PRIMARY KEY,
    subdomain TEXT NOT NULL,
    port INTEGER NOT NULL,
    protocol TEXT NOT NULL,
    crypto_algorithm TEXT NOT NULL,
    hops TEXT NOT NULL,
    proxychains BOOLEAN NOT NULL,
    public BOOLEAN NOT NULL,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_tunnels_subdomain ON tunnels(subdomain);
CREATE INDEX IF NOT EXISTS idx_crypto_vault_alg ON crypto_vault(algorithm_name);
```
