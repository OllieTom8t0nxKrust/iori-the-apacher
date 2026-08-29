# Database Models & Architecture Study

## 1. Database Technology Selection
For `iori-the-apacher`, the database architecture must fulfill two distinct operating environments:
1. **Domestic & Binary Computing**: High-speed embedded key-value and relational storage with zero external daemon configuration, supporting concurrent proxy tunneling and forensic log ingestion.
2. **Quantum Computing & Distributed Research**: Multi-model data structures (document, graph, and relational) capable of handling high-dimensional lattice cryptographic keys, quantum entanglement state metadata, and distributed cluster synchronization.

### Selected Database Engine: SurrealDB (Hybrid & Multi-Model) with SQLite Embedded Fallback
- **SurrealDB** is chosen as the primary database for quantum research and high-performance environments due to its native support for document, graph, and relational models, scalable distributed architecture, and quantum-safe cryptographic encryption at rest.
- **SQLite (Bundled via Rusqlite)** is chosen as the embedded domestic storage backend (implemented in our CLI adapter) because of its zero-latency file-based storage, ACID compliance, and flawless cross-compilation across binary and arm/quantum coprocessor boards.

---

## 2. Logical Database Model

The logical schema consists of three core entity domains:

### Entity 1: Tunnels (`tunnels`)
- `id` (UUID Primary Key)
- `subdomain` (String, unique routing identifier)
- `target_port` (Integer, local port forwarding target)
- `protocol` (String, e.g., http, https, tcp, quic)
- `active` (Boolean, connection status)
- `created_at` (Timestamp, RFC3339)

### Entity 2: Forensic Telemetry (`forensics`)
- `tracking_id` (UUID Primary Key)
- `source_ip` (String, client remote IP address)
- `user_agent` (String, browser/client agent signature)
- `hardware_fingerprint` (String, OS/hardware architecture hash)
- `geo_location` (String, resolved IP geolocation data)
- `timestamp` (Timestamp, RFC3339)

### Entity 3: Cryptographic Vault (`crypto_vault`)
- `key_id` (UUID Primary Key)
- `algorithm_category` (Enum: `Domestic`, `Quantum`)
- `algorithm_name` (String, e.g., `aes-256-gcm`, `pfe-969`)
- `public_key` (Binary/Blob)
- `secret_key_encrypted` (Binary/Blob, encrypted with master key)

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

CREATE TABLE IF NOT EXISTS forensics (
    tracking_id TEXT PRIMARY KEY,
    source_ip TEXT NOT NULL,
    user_agent TEXT NOT NULL,
    hardware_fingerprint TEXT NOT NULL,
    geo_location TEXT NOT NULL,
    timestamp TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS crypto_vault (
    key_id TEXT PRIMARY KEY,
    algorithm_category TEXT NOT NULL,
    algorithm_name TEXT NOT NULL,
    public_key BLOB NOT NULL,
    secret_key_encrypted BLOB NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_tunnels_subdomain ON tunnels(subdomain);
CREATE INDEX IF NOT EXISTS idx_forensics_ip ON forensics(source_ip);
```
