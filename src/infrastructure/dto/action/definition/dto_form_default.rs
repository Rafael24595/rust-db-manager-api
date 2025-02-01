use rust_db_manager_core::domain::action::definition::form_default::FormDefault;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct DTOFormDefault {
    key: String,
    value: String
}

impl DTOFormDefault {
    
    pub fn from(default: &FormDefault) -> Self {
        Self {
            key: default.key().to_string(),
            value: default.value().to_string()
        }
    }

}