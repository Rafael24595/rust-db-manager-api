use rust_db_manager_core::domain::definition::field::field_attribute_default_definition::FieldAttributeDefaultDefinition;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct DTOFieldAttributeDefaultDefinition {
    key: String,
    value: String
}

impl DTOFieldAttributeDefaultDefinition {
    
    pub fn from(default: &FieldAttributeDefaultDefinition) -> Self {
        Self {
            key: default.key(),
            value: default.value()
        }
    }

}