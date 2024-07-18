use rust_db_manager_core::domain::action::generate::action::Action;
use serde::Deserialize;

use super::dto_action_form::DTOActionForm;

#[derive(Clone, Deserialize)]
pub struct DTOAction {
    action: String,
    form: Vec<DTOActionForm>
}

impl DTOAction {
    
    pub fn from_dto(&self) -> Action {
        Action::new(
            self.action.clone(), 
            self.form.iter()
                .map(|f| f.from_dto())
                .collect())
    }

}