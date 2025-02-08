use rust_db_manager_core::domain::filter::definition::filter_definition::FilterDefinition;
use serde::Serialize;

use super::{dto_filter_attribute_definition::DTOFilterAttributeDefinition, dto_filter_definition_query::DTOFilterDefinitionQuery, dto_filter_fields_definition::DTOFilterFieldsDefinition};

#[derive(Clone, Serialize)]
pub struct DTOFilterDefinition {
    category_root: String,
    category_query: DTOFilterDefinitionQuery,
    categories: Vec<String>,
    fields: Vec<DTOFilterFieldsDefinition>,
    attributes: Vec<DTOFilterAttributeDefinition>
}

impl DTOFilterDefinition {

    pub fn from(schema: &FilterDefinition) -> Self {
        Self {
            category_root: schema.category_root().to_string(),
            category_query: DTOFilterDefinitionQuery::from(schema.category_query()),
            categories: schema.categories().clone(),
            fields: schema.fields().iter()
                .map(|f| DTOFilterFieldsDefinition::from(f))
                .collect(),
            attributes: schema.attributes().iter()
                .map(|f| DTOFilterAttributeDefinition::from(f))
                .collect()
        }
    }

}