use serde::Deserialize;

use super::document::dto_document_key::DTODocumentKey;

#[derive(Clone, Deserialize)]
pub struct DTOUpdateDocument {
    pub document: String,
    pub keys: Vec<DTODocumentKey>,
}