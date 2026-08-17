use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::Field;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub id: Uuid,
    pub title: String,
    pub category_id: Option<Uuid>,
    pub tag_ids: Vec<Uuid>,
    pub fields: Vec<Field>,
    pub is_favorite: bool,
    pub is_archived: bool,
    pub created_at: i64,
    pub updated_at: i64,
}



impl Item {
    pub fn new(title: String, category_id: Option<Uuid>) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        Item {
            id: Uuid::new_v4(),
            title,
            category_id,
            tag_ids: Vec::new(),
            fields: Vec::new(),
            is_favorite: false,
            is_archived: false,
            created_at: now,
            updated_at: now,
        }
    }
}
