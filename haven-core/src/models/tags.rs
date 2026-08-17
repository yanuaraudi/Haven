use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub color: Option<String>,
}

impl Tag {
    pub fn new(name: String) -> Self {
        Tag {
            id: Uuid::new_v4(),
            name,
            color: None,
        }
    }
}