use iori_the_apacher::domain::pfe969::Pfe969Cipher;
use iori_the_apacher::domain::crypto_config::QuantumAlgorithm;
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
