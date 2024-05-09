use rust_db_manager_core::domain::generate::field::field_reference::FieldReference;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct DTOFieldReference {
    collection: String,
    field: String
}

impl DTOFieldReference {
    
    pub fn from(reference: &FieldReference) -> Self {
        Self {
            collection: reference.collection(),
            field: reference.field()
        }
    }

    pub fn from_dto(&self) -> FieldReference {
        FieldReference::new(self.collection.clone(), self.field.clone())
    }

}