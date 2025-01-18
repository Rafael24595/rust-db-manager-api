use rust_db_manager_core::domain::document::document_data::DocumentData;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct DTODocumentData {
    format: String,
    data_base: String,
    collection: String,
    document: String
}

impl DTODocumentData {
    
    pub fn from(document: &DocumentData) -> Self {
        Self {
            format: document.format().to_string(),
            data_base: document.data_base(),
            collection: document.collection(),
            document: document.document(),
        }
    }

}