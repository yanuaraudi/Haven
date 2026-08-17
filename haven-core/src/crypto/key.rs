use zeroize::{Zeroize, ZeroizeOnDrop};
use argon2::Argon2;

#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct DerivedKey {
    pub bytes: [u8; 32],
}

pub fn derive_key(password: &str, salt: &[u8; 16]) -> Result<DerivedKey, String> {
    let mut key_bytes = [0u8; 32];

    Argon2::default().hash_password_into(password.as_bytes(), salt, &mut key_bytes)
        .map_err(|e| e.to_string())?;
    
        Ok(DerivedKey { bytes: key_bytes})
}