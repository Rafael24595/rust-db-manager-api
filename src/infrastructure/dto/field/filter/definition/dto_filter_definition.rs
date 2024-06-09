use rust_db_manager_core::domain::filter::definition::filter_definition::FilterDefinition;
use serde::Serialize;

use super::dto_filter_attribute_definition::DTOFilterAttributeDefinition;

#[derive(Clone, Serialize)]
pub struct DTOFilterDefinition {
    attributes: Vec<DTOFilterAttributeDefinition>
}

impl DTOFilterDefinition {

    pub fn from(schema: &FilterDefinition) -> Self {
        Self {
            attributes: schema.attributes().iter()
                .map(|f| DTOFilterAttributeDefinition::from(f))
                .collect()
        }
    }

}