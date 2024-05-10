use rust_db_manager_core::domain::field::generate::field_attribute::FieldAttribute;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct DTOFieldAttribute {
    key: String,
    value: String,
}

impl DTOFieldAttribute {
    
    pub fn from(attribute: &FieldAttribute) -> Self {
        Self {
            key: attribute.key(),
            value: attribute.value()
        }
    }

    pub fn from_dto(&self) -> FieldAttribute {
        FieldAttribute::new(self.key.clone(), self.value.clone())
    }

}