use rust_db_manager_core::domain::filter::definition::filter_definition_query::FilterDefinitionQuery;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct DTOFilterDefinitionQuery {
    category: String,
    json_type: String,
    example: String
}

impl DTOFilterDefinitionQuery {

    pub fn from(schema: &FilterDefinitionQuery) -> Self {
        Self {
            category: schema.category().to_string(),
            json_type: schema.json_type().to_string(),
            example: schema.example().to_string()
        }
    }

}