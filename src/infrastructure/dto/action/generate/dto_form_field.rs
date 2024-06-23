use rust_db_manager_core::domain::action::generate::form_field::FormField;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct DTOFormField {
    code: String,
    value: Vec<String>,
}

impl DTOFormField {
    
    pub fn from_dto(&self) -> FormField {
        FormField::new(self.code.clone(), self.value.clone())
    }

}