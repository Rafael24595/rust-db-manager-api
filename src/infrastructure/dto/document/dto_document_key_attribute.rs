use rust_db_manager_core::domain::document::document_key_attribute::DocumentKeyAttribute;
use serde::Serialize;


#[derive(Clone, Serialize)]
pub struct DTODocumentKeyAttribute {
    key: String,
    value: String
}

impl DTODocumentKeyAttribute {
    
    pub fn from(attribute: &DocumentKeyAttribute) -> Self {
        Self {
            key: attribute.key(),
            value: attribute.value()
        }
    }

}