use rust_db_manager_core::domain::filter::definition::filter_attribute_definition::FilterAttributeDefinition;
use serde::Serialize;

use super::dto_filter_attribute_default_definition::DTOFilterAttributeDefaultDefinition;

#[derive(Clone, Serialize)]
pub struct DTOFilterAttributeDefinition {
    pub code: String,
    pub name: String,
    pub description: String,
    pub values: Vec<DTOFilterAttributeDefaultDefinition>,
    pub applies: Vec<String>,
}

impl DTOFilterAttributeDefinition {
    
    pub fn from(attribute: &FilterAttributeDefinition) -> Self {
        Self {
            code: attribute.code(),
            name: attribute.name(),
            description: attribute.description(),
            values: attribute.values().iter()
                .map(|a| DTOFilterAttributeDefaultDefinition::from(a))
                .collect(),
            applies: attribute.applies().iter()
                .map(|a| a.to_string())
                .collect(),
        }
    }

}