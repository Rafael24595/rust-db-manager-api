use rust_db_manager_core::domain::document::document_key_attribute::DocumentKeyAttribute;
use serde::{Deserialize, Serialize};


#[derive(Clone, Serialize, Deserialize)]
pub struct DTODocumentKeyAttribute {
    pub key: String,
    pub value: String
}

impl DTODocumentKeyAttribute {
    
    pub fn from(attribute: &DocumentKeyAttribute) -> Self {
        Self {
            key: attribute.key(),
            value: attribute.value()
        }
    }

}