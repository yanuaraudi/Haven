use crate::crypto::DerivedKey;
use crate::models::VaultPayload;
use crate::storage::VaultHeader;

#[derive(Clone, Debug)]
pub struct VaultSession {
    pub header: VaultHeader,
    pub payload: VaultPayload,
    pub key: DerivedKey,
    pub is_unlocked: bool,
}

impl VaultSession {
    pub fn new(header: VaultHeader, payload: VaultPayload, key: DerivedKey) -> Self {
        Self {
            header,
            payload,
            key,
            is_unlocked: true,
        }
    }

    pub fn lock(&mut self) {
        self.is_unlocked = false;
        self.payload.items.clear();
    }
}