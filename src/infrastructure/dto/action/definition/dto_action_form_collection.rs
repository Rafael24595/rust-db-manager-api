use rust_db_manager_core::domain::action::definition::action_form_collection::ActionFormCollection;
use serde::Serialize;

use super::dto_action_form::DTOActionForm;

#[derive(Clone, Serialize)]
pub struct DTOActionFormCollection {
    sw_query: bool,
    forms: Vec<DTOActionForm>
}

impl DTOActionFormCollection {
    
    pub fn from(collection: &ActionFormCollection) -> Self {
        Self {
            sw_query: collection.is_query(),
            forms: collection.forms().iter()
                .map(|f| DTOActionForm::from(f))
                .collect()
        }
    }

}