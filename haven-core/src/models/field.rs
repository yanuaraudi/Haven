use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum FieldType {
    Text,
    Password,
    Url,
    Note,
    File,
    Custom(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Field {
    pub id: Uuid,
    pub name: String,
    pub field_type: FieldType,
    pub value: String,
}

impl Field {
    pub fn new(name: String, field_type: FieldType, value: String) -> Self {
        Field {
            id: Uuid::new_v4(),
            name,
            field_type,
            value,
        }
    }
}