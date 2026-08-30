use iori_the_apacher::domain::pfe969::Pfe969Cipher;
use iori_the_apacher::domain::crypto_config::QuantumAlgorithm;
use iori_the_apacher::domain::routing::{NetworkProtocol, CryptoRequirement};
use iori_the_apacher::application::service::ApplicationService;
use iori_the_apacher::adapters::db_adapter::SqliteStorageAdapter;
use std::sync::Arc;

#[test]
fn test_integration_pfe969_decrypt_flow() {
    let cipher = Pfe969Cipher::new(256, 2048, 32);
    let (sk, pk) = cipher.generate_keypair();
    let msg = b"Testing quantum PFE-969 end-to-end integration";
    let ct = cipher.encrypt(msg, &pk);
    let pt = cipher.decrypt(&ct, &sk).unwrap();
    assert_eq!(msg.to_vec(), pt);
}

#[tokio::test]
async fn test_application_service_quantum_crypto() {
    let db = SqliteStorageAdapter::new(":memory:").unwrap();
    let service = ApplicationService::new(Arc::new(db));
    let msg = b"Application service quantum test";
    let (ct, sk, _pk) = service.encrypt_quantum(QuantumAlgorithm::Pfe969HyperLattice, msg).unwrap();
    let pt = service.decrypt_quantum(QuantumAlgorithm::Pfe969HyperLattice, &ct, &sk).unwrap();
    assert_eq!(msg.to_vec(), pt);
}

#[tokio::test]
async fn test_server_launch_crypto_verification_enforcement() {
    let db = SqliteStorageAdapter::new(":memory:").unwrap();
    let service = ApplicationService::new(Arc::new(db));

    // Attempt public internet launch with no crypto -> Should fail security policy
    let res = service.launch_server(
        "pub-app".to_string(),
        80,
        NetworkProtocol::Https,
        CryptoRequirement::None,
        vec!["relay1".to_string()],
        true,
        true,
    ).await;
    assert!(res.is_err());

    // Launch with PFE-969 quantum crypto protection -> Should succeed
    let res_success = service.launch_server(
        "quantum-app".to_string(),
        443,
        NetworkProtocol::TorOnionV3,
        CryptoRequirement::QuantumPfe969Lattice,
        vec!["hop1".to_string(), "hop2".to_string()],
        true,
        true,
    ).await;
    assert!(res_success.is_ok());
    let launched = res_success.unwrap();
    assert_eq!(launched.subdomain, "quantum-app");
    assert_eq!(launched.protocol, NetworkProtocol::TorOnionV3);
    assert_eq!(launched.multi_hop_nodes.len(), 2);
}


