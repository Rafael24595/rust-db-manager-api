use serde::Deserialize;

use super::document::dto_document_key::DTODocumentKey;

#[derive(Clone, Deserialize)]
pub struct DTOCreateDocument {
    pub document: String
}