use rust_db_manager_core::domain::collection::collections_reference_definition::CollectionReferenceDefinition;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DtoCollectionReferenceDefinition {
    collection: String,
    fields: Vec<String>,
}

impl DtoCollectionReferenceDefinition {
    
    pub fn from(reference: &CollectionReferenceDefinition) -> Self {
        Self {
            collection: reference.collection(),
            fields: reference.fields().clone()
        }
    }

}