use rust_db_manager_core::domain::action::definition::form_field_definition::FormFieldDefinition;
use serde::Serialize;

use super::dto_form_default::DTOFormDefault;

#[derive(Clone, Serialize)]
pub struct DTOFormFieldDefinition {
    order: usize,
    code: String,
    name: String,
    sw_key: bool,
    values: Vec<DTOFormDefault>,
}

impl DTOFormFieldDefinition {
    
    pub fn from(field: &FormFieldDefinition) -> Self {
        Self {
            order: field.order(),
            code: field.code(),
            name: field.name(),
            sw_key: field.is_key(),
            values: field.values().iter()
                .map(|d| DTOFormDefault::from(d))
                .collect()
        }
    }

}