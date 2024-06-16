use rust_db_manager_core::domain::table::definition::table_field_definition::TableFieldDefinition;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DTOTableFieldDefinition {
    data: String,
    sw_title: bool
}

impl DTOTableFieldDefinition {
    
    pub fn from(field: &TableFieldDefinition) -> Self {
        Self {
            data: field.data(),
            sw_title: field.is_title()
        }
    }

}