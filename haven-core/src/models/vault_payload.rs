use serde::{Deserialize, Serialize};
use crate::models::Tag;
use crate::models::Category;
use crate::models::Item;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VaultPayload {
    pub vault_name: String,
    pub items: Vec<Item>,
    pub categories: Vec<Category>,
    pub tags: Vec<Tag>,
}

impl VaultPayload{
    pub fn new(vault_name: String) -> Self {
        VaultPayload {
            vault_name,
            items: Vec::new(),
            categories: Vec::new(),
            tags: Vec::new(),
            }
    }
}