use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
}

impl Category {
    pub fn new(name: String) -> Self {
        Category {
            id: Uuid::new_v4(),
            name,
            description: None,
            icon: None,
            color: None,
        }
    }
}