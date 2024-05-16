use rust_db_manager_core::domain::collection::collection_data::CollectionData;
use serde::Serialize;

use crate::infrastructure::dto::document::dto_document_data::DTODocumentData;

#[derive(Clone, Serialize)]
pub struct DTOCollectionData {
    total: usize,
    limit: Option<usize>,
    offset: Option<usize>,
    documents: Vec<DTODocumentData>
}

impl DTOCollectionData {
    
    pub fn from(data: &CollectionData) -> Self {
        Self {
            total: data.total(),
            limit: data.limit(),
            offset: data.offset(),
            documents: data.documents().iter()
                .map(|d| DTODocumentData::from(d))
                .collect()
        }
    }

}