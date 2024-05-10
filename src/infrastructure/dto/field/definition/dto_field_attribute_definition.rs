use rust_db_manager_core::domain::field::definition::field_attribute_definition::FieldAttributeDefinition;
use serde::Serialize;

use super::dto_field_attribute_default_definition::DTOFieldAttributeDefaultDefinition;

#[derive(Clone, Serialize)]
pub struct DTOFieldAttributeDefinition {
    name: String,
    code: String,
    values: Vec<DTOFieldAttributeDefaultDefinition>,
}

impl DTOFieldAttributeDefinition {
    
    pub fn from(attribute: &FieldAttributeDefinition) -> Self {
        Self {
            name: attribute.name(),
            code: attribute.code(),
            values: attribute.values().iter()
                .map(|a| DTOFieldAttributeDefaultDefinition::from(a))
                .collect()
        }
    }

}