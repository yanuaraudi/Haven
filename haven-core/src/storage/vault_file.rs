use std::fs;
use crate::storage::header::{VaultHeader, HEADER_SIZE};

pub fn save_vault_file(path: &str, header: &VaultHeader, encrypted_payload: &[u8]) -> Result<(), String> {
    let mut full_data = header.to_bytes();
    full_data.extend_from_slice(encrypted_payload);

    let temp_path = format!("{}.tmp", path);
    fs::write(&temp_path, &full_data).map_err(|e| e.to_string())?;
    fs::rename(&temp_path, path).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn read_vault_file(path: &str) -> Result<(VaultHeader, Vec<u8>), String> {
    let file_bytes = fs::read(path).map_err(|e| e.to_string())?;
    let header = VaultHeader::from_bytes(&file_bytes)?;
    let ciphertext = file_bytes[HEADER_SIZE..].to_vec();
    Ok((header, ciphertext))
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_vault_file_save_and_read() {
        let vault_id = Uuid::new_v4();
        let salt = [1u8; 16];
        let nonce = [2u8; 12];
        let header = VaultHeader::new(vault_id, salt, nonce);
        let fake_payload = b"encrypted_payload_data";

        let test_file = "test_vault.hvlt";

        // 1. Save file
        save_vault_file(test_file, &header, fake_payload).unwrap();

        // 2. Read file back
        let (read_header, read_payload) = read_vault_file(test_file).unwrap();

        // 3. Assert header fields match!
        assert_eq!(read_header.vault_id, vault_id);
        assert_eq!(read_header.salt, salt);
        assert_eq!(read_header.nonce, nonce);
        assert_eq!(read_payload, fake_payload);

        // Clean up test file
        let _ = std::fs::remove_file(test_file);
    }
}