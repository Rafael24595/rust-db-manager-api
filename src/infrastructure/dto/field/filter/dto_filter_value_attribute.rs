use rust_db_manager_core::domain::filter::filter_value_attribute::FilterValueAttribute;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DTOFilterValueAttribute {
    key: String,
    value: String
}

impl DTOFilterValueAttribute {
    
    pub fn from_dto(&self) -> FilterValueAttribute {
        FilterValueAttribute::new(self.key.clone(), self.value.clone())
    }

}