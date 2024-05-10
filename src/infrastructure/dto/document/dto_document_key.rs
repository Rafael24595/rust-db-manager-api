use rust_db_manager_core::domain::document::document_key::DocumentKey;
use serde::Serialize;

use super::dto_document_key_attribute::DTODocumentKeyAttribute;


#[derive(Clone, Serialize)]
pub struct DTODocumentKey {
    name: String,
    value: String,
    attributes: Vec<DTODocumentKeyAttribute>
}

impl DTODocumentKey {

    pub fn from(key: &DocumentKey) -> Self {
        Self {
            name: key.name(),
            value: key.value(),
            attributes: key.attributes().iter()
                .map(|a| DTODocumentKeyAttribute::from(a))
                .collect()
        }
    }

}