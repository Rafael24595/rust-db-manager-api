use rust_db_manager_core::domain::filter::definition::filter_fields_definition::FilterFieldsDefinition;
use serde::Serialize;

use super::dto_filter_field_definition::DTOFilterFieldDefinition;

#[derive(Clone, Serialize)]
pub struct DTOFilterFieldsDefinition {
    category: String,
    fields: Vec<DTOFilterFieldDefinition>
}

impl DTOFilterFieldsDefinition {

    pub fn from(schema: &FilterFieldsDefinition) -> Self {
        Self {
            category: schema.category().to_string(),
            fields: schema.fields().iter()
                .map(|f| DTOFilterFieldDefinition::from(f))
                .collect(),
        }
    }

}