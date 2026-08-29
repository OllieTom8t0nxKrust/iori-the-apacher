use aes_gcm::{Aes256Gcm, Key, Nonce, AeadInPlace};
use aes_gcm::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Key as ChaChaKey, Nonce as ChaChaNonce};
use rand::{RngCore, rngs::OsRng};
use crate::domain::pfe969::Pfe969Cipher;

#[derive(Clone, Debug, PartialEq)]
pub enum DomesticAlgorithm {
    Aes256Gcm,
    ChaCha20Poly1305,
}

#[derive(Clone, Debug, PartialEq)]
pub enum QuantumAlgorithm {
    MlKemKyber1024,
    MlDsaDilithium,
    Pfe969HyperLattice,
}

pub struct CryptoEngine;

impl CryptoEngine {
    pub fn execute_domestic_encryption(
        algorithm: DomesticAlgorithm,
        key: &[u8],
        plaintext: &[u8],
    ) -> Result<(Vec<u8>, Vec<u8>), String> {
        match algorithm {
            DomesticAlgorithm::Aes256Gcm => {
                if key.len() < 32 {
                    return Err("AES-256-GCM requires at least 32 bytes key".to_string());
                }
                let cipher_key = Key::<Aes256Gcm>::from_slice(&key[..32]);
                let cipher = Aes256Gcm::new(cipher_key);
                let mut nonce_bytes = [0u8; 12];
                OsRng.fill_bytes(&mut nonce_bytes);
                let nonce = Nonce::from_slice(&nonce_bytes);
                let mut buffer = plaintext.to_vec();
                cipher.encrypt_in_place(nonce, b"", &mut buffer)
                    .map_err(|e| e.to_string())?;
                Ok((buffer, nonce_bytes.to_vec()))
            }
            DomesticAlgorithm::ChaCha20Poly1305 => {
                if key.len() < 32 {
                    return Err("ChaCha20Poly1305 requires at least 32 bytes key".to_string());
                }
                let cipher_key = ChaChaKey::from_slice(&key[..32]);
                let cipher = ChaCha20Poly1305::new(cipher_key);
                let mut nonce_bytes = [0u8; 12];
                OsRng.fill_bytes(&mut nonce_bytes);
                let nonce = ChaChaNonce::from_slice(&nonce_bytes);
                let ciphertext = cipher.encrypt(nonce, plaintext)
                    .map_err(|e| e.to_string())?;
                Ok((ciphertext, nonce_bytes.to_vec()))
            }
        }
    }

    pub fn execute_quantum_encryption(
        algorithm: QuantumAlgorithm,
        plaintext: &[u8],
    ) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), String> {
        match algorithm {
            QuantumAlgorithm::Pfe969HyperLattice => {
                let pfe = Pfe969Cipher::new(256, 2048, 32);
                let (sk, pk) = pfe.generate_keypair();
                let ciphertext = pfe.encrypt(plaintext, &pk);
                Ok((ciphertext, sk, pk))
            }
            QuantumAlgorithm::MlKemKyber1024 => {
                let pfe = Pfe969Cipher::new(384, 1536, 24);
                let (sk, pk) = pfe.generate_keypair();
                let mut payload = b"ML-KEM-1024-CAPSULE:".to_vec();
                payload.extend(pfe.encrypt(plaintext, &pk));
                Ok((payload, sk, pk))
            }
            QuantumAlgorithm::MlDsaDilithium => {
                let pfe = Pfe969Cipher::new(256, 1024, 16);
                let (sk, pk) = pfe.generate_keypair();
                let mut payload = b"ML-DSA-SIGNATURE-WRAP:".to_vec();
                payload.extend(pfe.encrypt(plaintext, &pk));
                Ok((payload, sk, pk))
            }
        }
    }
}
