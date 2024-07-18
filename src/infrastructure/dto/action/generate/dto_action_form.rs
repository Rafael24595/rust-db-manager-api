use rust_db_manager_core::domain::action::generate::action_form::ActionForm;
use serde::Deserialize;

use super::dto_form_field::DTOFormField;

#[derive(Clone, Deserialize)]
pub struct DTOActionForm {
    code: String,
    fields: Vec<Vec<DTOFormField>>
}

impl DTOActionForm {
    
    pub fn from_dto(&self) -> ActionForm {
        ActionForm::new(
            self.code.clone(), 
            self.fields.iter()
                .map(|c| 
                    c.iter()
                        .map(|f| f.from_dto())
                        .collect()
                ).collect())
    }

}