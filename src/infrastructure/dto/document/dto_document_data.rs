use rust_db_manager_core::domain::document::document_data::DocumentData;
use serde::Serialize;

use super::dto_document_key::DTODocumentKey;


#[derive(Clone, Serialize)]
pub struct DTODocumentData {
    data_base: String,
    collection: String,
    document: String
}

impl DTODocumentData {
    
    pub fn from(document: &DocumentData) -> Self {
        Self {
            data_base: document.data_base(),
            collection: document.collection(),
            document: document.document(),
        }
    }

}