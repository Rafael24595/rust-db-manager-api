use rust_db_manager_core::domain::filter::definition::filter_field_definition::FilterFieldDefinition;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct DTOFilterFieldDefinition {
    field: String,
    json_type: String,
    defaults: Vec<String>
}

impl DTOFilterFieldDefinition {

    pub fn from(schema: &FilterFieldDefinition) -> Self {
        Self {
            field: schema.field().to_string(),
            json_type: schema.json_type().to_string(),
            defaults: schema.defaults().clone()
        }
    }

}