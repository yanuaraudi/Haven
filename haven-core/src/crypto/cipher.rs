use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce, Key
};
use crate::crypto::DerivedKey;

pub fn encrypt_payload(key: &DerivedKey, nonce_bytes: &[u8; 12], plaintext: &[u8]) -> Result<Vec<u8>, String> {
    let cipher_key = Key::from_slice(&key.bytes);
    let cipher = ChaCha20Poly1305::new(cipher_key);
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher.encrypt(nonce, plaintext).map_err(|_| "Encryption failed".to_string())
}

pub fn decrypt_payload(key: &DerivedKey, nonce_bytes: &[u8; 12], ciphertext: &[u8]) -> Result<Vec<u8>, String> {
    let cipher_key = Key::from_slice(&key.bytes);
    let cipher = ChaCha20Poly1305::new(cipher_key);
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher.decrypt(nonce, ciphertext).map_err(|_| "Decryption failed: invalid key or corrupted data".to_string())
}