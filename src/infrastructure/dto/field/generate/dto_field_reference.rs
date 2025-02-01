use rust_db_manager_core::domain::field::generate::field_reference::FieldReference;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct DTOFieldReference {
    collection: String,
    field: String,
    cascade: bool
}

impl DTOFieldReference {
    
    pub fn from(reference: &FieldReference) -> Self {
        Self {
            collection: reference.collection().to_string(),
            field: reference.field().to_string(),
            cascade: reference.cascade()
        }
    }

    pub fn from_dto(&self) -> FieldReference {
        FieldReference::new(self.collection.clone(), self.field.clone(), self.cascade)
    }

}