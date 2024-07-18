use rust_db_manager_core::domain::filter::definition::filter_definition::FilterDefinition;
use serde::Serialize;

use super::dto_filter_attribute_definition::DTOFilterAttributeDefinition;

#[derive(Clone, Serialize)]
pub struct DTOFilterDefinition {
    query_type: String,
    query_example: String,
    attributes: Vec<DTOFilterAttributeDefinition>
}

impl DTOFilterDefinition {

    pub fn from(schema: &FilterDefinition) -> Self {
        Self {
            query_type: schema.query_type(),
            query_example: schema.query_example(),
            attributes: schema.attributes().iter()
                .map(|f| DTOFilterAttributeDefinition::from(f))
                .collect()
        }
    }

}