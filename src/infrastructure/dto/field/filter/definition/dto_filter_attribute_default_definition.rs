use rust_db_manager_core::domain::filter::definition::filter_attribute_default_definition::FilterAttributeDefaultDefinition;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct DTOFilterAttributeDefaultDefinition {
    pub key: String,
    pub value: String,
    pub default: bool
}

impl DTOFilterAttributeDefaultDefinition {
    
    pub fn from(attribute: &FilterAttributeDefaultDefinition) -> Self {
        Self {
            key: attribute.key(),
            value: attribute.value(),
            default: attribute.default()
        }
    }

}