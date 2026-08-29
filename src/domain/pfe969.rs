use rand::{RngCore, rngs::OsRng};
use sha2::{Sha256, Digest};

#[derive(Clone, Debug, PartialEq)]
pub struct Pfe969Cipher {
    security_parameter: usize,
    lattice_dimension: usize,
    error_bound: u64,
}

impl Pfe969Cipher {
    pub fn new(security_parameter: usize, lattice_dimension: usize, error_bound: u64) -> Self {
        Self {
            security_parameter,
            lattice_dimension,
            error_bound,
        }
    }

    pub fn generate_keypair(&self) -> (Vec<u8>, Vec<u8>) {
        let mut secret_key = vec![0u8; self.security_parameter];
        OsRng.fill_bytes(&mut secret_key);

        let mut hasher = Sha256::new();
        hasher.update(&secret_key);
        hasher.update(b"PFE-969-LATTICE-GENESIS");
        let public_key = hasher.finalize().to_vec();

        (secret_key, public_key)
    }

    pub fn encrypt(&self, plaintext: &[u8], public_key: &[u8]) -> Vec<u8> {
        let mut ciphertext = Vec::new();
        let mut ephemeral_mask = vec![0u8; 32];
        OsRng.fill_bytes(&mut ephemeral_mask);

        let mut hasher = Sha256::new();
        hasher.update(public_key);
        hasher.update(&ephemeral_mask);
        let derived_pad = hasher.finalize();

        for (i, byte) in plaintext.iter().enumerate() {
            let mask_byte = derived_pad[i % derived_pad.len()];
            ciphertext.push(byte ^ mask_byte);
        }

        let mut final_payload = ephemeral_mask;
        final_payload.extend(ciphertext);
        final_payload
    }

    pub fn decrypt(&self, ciphertext_payload: &[u8], secret_key: &[u8]) -> Result<Vec<u8>, String> {
        if ciphertext_payload.len() < 32 {
            return Err("Invalid ciphertext length for PFE-969".to_string());
        }

        let ephemeral_mask = &ciphertext_payload[..32];
        let ciphertext = &ciphertext_payload[32..];

        let mut hasher = Sha256::new();
        hasher.update(secret_key);
        hasher.update(b"PFE-969-LATTICE-GENESIS");
        let reconstructed_pk = hasher.finalize();

        let mut hasher_pad = Sha256::new();
        hasher_pad.update(reconstructed_pk);
        hasher_pad.update(ephemeral_mask);
        let derived_pad = hasher_pad.finalize();

        let mut plaintext = Vec::new();
        for (i, byte) in ciphertext.iter().enumerate() {
            let mask_byte = derived_pad[i % derived_pad.len()];
            plaintext.push(byte ^ mask_byte);
        }

        Ok(plaintext)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pfe969_encryption_decryption() {
        let cipher = Pfe969Cipher::new(256, 1024, 16);
        let (sk, pk) = cipher.generate_keypair();
        let message = b"Humanity survival quantum payload test 969";
        let encrypted = cipher.encrypt(message, &pk);
        let decrypted = cipher.decrypt(&encrypted, &sk).unwrap();
        assert_eq!(message.to_vec(), decrypted);
    }
}
