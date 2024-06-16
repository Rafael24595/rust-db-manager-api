use rust_db_manager_core::domain::action::definition::action_form::ActionForm;
use serde::Serialize;

use super::dto_form_field_definition::DTOFormFieldDefinition;

#[derive(Clone, Serialize)]
pub struct DTOActionForm {
    title: Option<String>,
    sw_vector: bool,
    fields: Vec<DTOFormFieldDefinition>
}

impl DTOActionForm {
    
    pub fn from(action: &ActionForm) -> Self {
        Self {
            title: action.title(),
            sw_vector: action.is_vector(),
            fields: action.fields().iter()
                .map(|f| DTOFormFieldDefinition::from(f))
                .collect()
        }
    }

}