use rust_db_manager_core::domain::filter::filter_element::FilterElement;
use serde::Deserialize;

use crate::commons::exception::api_exception::ApiException;

use super::dto_filter_value::DTOFilterValue;

#[derive(Clone, Deserialize)]
pub struct DTOFilterElement {
    key: String,
    value: DTOFilterValue,
    direction: bool,
    negation: bool,
}

impl DTOFilterElement {
    
    pub fn from_dto(&self) -> Result<FilterElement, ApiException> {
        Ok(FilterElement::from(
            self.key.clone(), 
            self.value.from_dto()?, 
            self.direction, 
            self.negation
        ))
    }

}