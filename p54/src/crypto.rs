// Source: https://backendengineer.io/aes-encryption-rust/

use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::{AeadCore, Aes256Gcm, Key, KeyInit, Nonce};

pub fn encrypt(key_str: &[u8; 32], plaintext: Vec<u8>) -> Vec<u8> {
    let key = Key::<Aes256Gcm>::from_slice(key_str);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let cipher = Aes256Gcm::new(key);

    let ciphered_data = cipher
        .encrypt(&nonce, &*plaintext)
        .expect("failed to encrypt");

    // combining nonce and encrypted data together
    // for storage purpose
    let mut encrypted_data: Vec<u8> = nonce.to_vec();
    encrypted_data.extend_from_slice(&ciphered_data);

    encrypted_data
}

pub fn decrypt(key_str: &[u8; 32], encrypted_data: Vec<u8>) -> Vec<u8> {
    let key = Key::<Aes256Gcm>::from_slice(key_str);

    let (nonce_arr, ciphered_data) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_arr);

    let cipher = Aes256Gcm::new(key);

    cipher
        .decrypt(nonce, ciphered_data)
        .expect("failed to decrypt data")
}

#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    pub fn expand_key() {}
}
