#[cfg(test)]
mod tests {
    use iori_the_apacher::domain::pfe969::Pfe969Cipher;

    #[test]
    fn test_mutation_corrupted_quantum_pfe969() {
        let cipher = Pfe969Cipher::new(256, 2048, 32);
        let (sk, pk) = cipher.generate_keypair();
        let msg = b"quantum mutation test";
        let mut ct = cipher.encrypt(msg, &pk);

        if !ct.is_empty() {
            ct[0] ^= 0x55;
        }

        let pt = cipher.decrypt(&ct, &sk).unwrap();
        assert_ne!(msg.to_vec(), pt);
    }
}
