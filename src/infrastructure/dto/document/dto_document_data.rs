use rust_db_manager_core::domain::document::document_data::DocumentData;
use serde::Serialize;

use super::dto_document_key::DTODocumentKey;


#[derive(Clone, Serialize)]
pub struct DTODocumentData {
    data_base: String,
    collection: String,
    base_key: Option<DTODocumentKey>,
    keys: Vec<DTODocumentKey>,
    document: String
}

impl DTODocumentData {
    
    pub fn from(document: &DocumentData) -> Self {
        Self {
            data_base: document.data_base(),
            collection: document.collection(),
            base_key: match document.base_key() {
                Some(key) => Some(DTODocumentKey::from(&key)),
                None => None,
            },
            keys: document.keys().iter()
                .map(|k| DTODocumentKey::from(k))
                .collect(),
            document: document.document(),
        }
    }

}