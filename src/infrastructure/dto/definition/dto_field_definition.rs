use rust_db_manager_core::domain::definition::field::field_definition::FieldDefinition;
use serde::Serialize;

use super::dto_field_attribute_definition::DTOFieldAttributeDefinition;

#[derive(Clone, Serialize)]
pub struct DTOFieldDefinition {
    order: usize,
    name: String,
    code: String,
    category: String,
    size: bool,
    multiple: bool,
    attributes: Vec<DTOFieldAttributeDefinition>
}

impl DTOFieldDefinition {
    
    pub fn from(definition: &FieldDefinition) -> Self {
        Self {
            order: definition.order(),
            name: definition.name(),
            code: definition.code().to_string(),
            category: definition.category().to_string(),
            size: definition.size(),
            multiple: definition.multiple(),
            attributes: definition.attributes().iter()
                .map(|a| DTOFieldAttributeDefinition::from(a))
                .collect()
        }
    }

}