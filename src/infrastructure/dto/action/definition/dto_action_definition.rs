use rust_db_manager_core::domain::action::definition::action_definition::ActionDefinition;
use serde::Serialize;

use crate::infrastructure::dto::table::definition::dto_table_definition::DTOTableDefinition;

use super::dto_action_form_collection::DTOActionFormCollection;

#[derive(Clone, Serialize)]
pub struct DTOActionDefinition {
    action: String,
    title: String,
    data: Option<Vec<DTOTableDefinition>>,
    form: Option<DTOActionFormCollection>
}

impl DTOActionDefinition {
    
    pub fn from(action: &ActionDefinition) -> Self {
        let mut data = None;
        if let Some(e_data) = action.data() {
            data = Some(e_data.iter()
                .map(|t| DTOTableDefinition::from(t))
                .collect()
            );
        }

        let mut form = None;
        if let Some(e_form) = action.form() {
            form = Some(DTOActionFormCollection::from(&e_form));
        }

        Self {
            action: action.action(),
            title: action.title(),
            data: data,
            form: form
        }
    }

}