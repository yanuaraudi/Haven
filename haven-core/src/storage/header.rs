use uuid::Uuid;

pub const MAGIC_BYTES: &[u8; 5] = b"HAVEN";
pub const HEADER_SIZE: usize = 67;

#[derive(Debug, Clone, PartialEq)]
pub struct VaultHeader {
    pub magic: [u8; 5],
    pub version: u16,
    pub vault_id: Uuid,
    pub salt: [u8; 16],
    pub nonce: [u8; 12],
    pub reserved: [u8; 16],
}

impl VaultHeader {
    pub fn new(vault_id: Uuid, salt:[u8; 16], nonce: [u8; 12]) -> Self {
        Self {
            magic: *MAGIC_BYTES,
            version: 1,
            vault_id,
            salt,
            nonce,
            reserved: [0u8; 16],
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut header_bytes = Vec::with_capacity(HEADER_SIZE);
        header_bytes.extend_from_slice(&self.magic);
        header_bytes.extend_from_slice(&self.version.to_le_bytes());
        header_bytes.extend_from_slice(self.vault_id.as_bytes());
        header_bytes.extend_from_slice(&self.salt);
        header_bytes.extend_from_slice(&self.nonce);
        header_bytes.extend_from_slice(&self.reserved);
        return header_bytes;
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<VaultHeader, String> {
        if bytes.len() < HEADER_SIZE {
            return Err("Header too short".to_string());
        }
        let magic: [u8; 5] = bytes[0..5].try_into().unwrap();
        if &magic != MAGIC_BYTES {
            return Err("Invalid vault header magic bytes".to_string());
        }
        let version_bytes: [u8; 2] = bytes[5..7].try_into().unwrap();
        let version = u16::from_le_bytes(version_bytes);
        let vault_id = Uuid::from_slice(&bytes[7..23]).map_err(|e| format!("Invalid Vault UUID: {}", e))?;
        let salt: [u8; 16] = bytes[23..39].try_into().unwrap();
        let nonce: [u8; 12] = bytes[39..51].try_into().unwrap();
        let reserved: [u8; 16] = bytes[51..67].try_into().unwrap();
        return Ok(VaultHeader {magic, version, vault_id, salt, nonce, reserved})
    } 

}