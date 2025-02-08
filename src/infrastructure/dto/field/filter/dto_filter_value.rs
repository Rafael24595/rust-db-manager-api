use rust_db_manager_core::domain::filter::filter_value::FilterValue;
use serde::Deserialize;

use crate::commons::exception::api_exception::ApiException;

use super::{dto_filter_element::DTOFilterElement, dto_filter_value_attribute::DTOFilterValueAttribute};

#[derive(Clone, Deserialize)]
pub struct DTOFilterValue {
    category: String,
    value: String,
    attributes: Vec<DTOFilterValueAttribute>,
    children: Vec<DTOFilterElement>
}

impl DTOFilterValue {
    
    pub fn from_dto(&self) -> Result<FilterValue, ApiException> {
        let mut children = Vec::new();
        for child in self.children.to_vec() {
            children.push(child.from_dto()?);
        }

        Ok(FilterValue::from(
            self.category.clone(), 
            self.value.clone(), 
            self.attributes.iter().map(|c| c.from_dto()).collect(), 
            children
        ))
    }

}