# Test Changelog - IORI THE APACHER

All notable changes to the test suite (unit, integration, integrity, mutation, and chaos tests) are documented in this file.

## [1.0.0] - 2026-09-02

### Added
- **Test Suite Structure**: Established comprehensive test directories under `tests/` (`unitests`, `integration-tests`, `integrity-tests`, `mutation-tests`, `chaos-tests`).
- **Unit Tests**: Added thorough unit tests covering tunnel creation/validation, domestic ciphers (AES, ChaCha), quantum ciphers (Kyber, Dilithium, PFE-969), and routing configurations.
- **Integration Tests**: Added end-to-end integration tests for `ApplicationService`, database persistence, quantum encryption flows, and server launch cryptographic enforcement.
- **Integrity Tests**: Added SQLite schema migration integrity checks, foreign key/data consistency validations, and crypto vault integrity assertions.
- **Mutation Tests**: Added fault injection tests testing corrupted ciphertexts, invalid nonce lengths, truncated key bytes, and cryptographic tampering resistance.
- **Chaos Tests**: Added concurrent database write stress tests, rapid tunnel session churn simulations, and simulated resource contention scenarios.
