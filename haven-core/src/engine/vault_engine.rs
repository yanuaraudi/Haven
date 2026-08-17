use rand::RngCore;
use uuid::Uuid;
use crate::crypto::{derive_key, encrypt_payload, decrypt_payload};
use crate::models::VaultPayload;
use crate::storage::{VaultHeader, save_vault_file, read_vault_file};
use crate::engine::VaultSession;

pub struct VaultEngine;

impl VaultEngine {
    pub fn create_vault(path: &str, password: &str, vault_name: String) -> Result<VaultSession, String> {
        let mut salt = [0u8; 16];
        let mut nonce = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut salt);
        rand::thread_rng().fill_bytes(&mut nonce);
        let vault_id = Uuid::new_v4();
        let key = derive_key(password, &salt)?;
        let header = VaultHeader::new(vault_id, salt, nonce);
        let payload = VaultPayload::new(vault_name);
        let json_bytes = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let ciphertext = encrypt_payload(&key, &nonce, &json_bytes)?;
        save_vault_file(path, &header, &ciphertext)?;
        return Ok(VaultSession::new(header, payload, key))
    }

    pub fn unlock_vault(path: &str, password: &str) -> Result<VaultSession, String> {
        let (header, ciphertext) = read_vault_file(path)?;
        let key = derive_key(password, &header.salt)?;
        let decrypted_bytes = decrypt_payload(&key, &header.nonce, &ciphertext)?;
        let payload: VaultPayload = serde_json::from_slice(&decrypted_bytes).map_err(|e| e.to_string())?;
        return Ok(VaultSession::new(header, payload, key))
    }

    pub fn save_session(session: &VaultSession, path: &str) -> Result<(), String> {
        if !session.is_unlocked {
            return Err("Vault is locked".to_string());
        }
        let json_bytes = serde_json::to_vec(&session.payload).map_err(|e| e.to_string())?;
        let ciphertext = encrypt_payload(&session.key, &session.header.nonce, &json_bytes)?;
        save_vault_file(path, &session.header, &ciphertext)?;
        return Ok(())
    }
}