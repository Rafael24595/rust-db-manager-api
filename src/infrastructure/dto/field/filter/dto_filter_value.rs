use rust_db_manager_core::domain::filter::{e_filter_category::EFilterCategory, filter_value::FilterValue};
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
        let category = EFilterCategory::from_string(&self.category);
        if let None = category {
            let exception = ApiException::new(422, String::from("Field category not recognized."));
            return Err(exception);
        }

        let mut children = Vec::new();
        for child in self.children.clone() {
            children.push(child.from_dto()?);
        }

        Ok(FilterValue::from(
            category.unwrap(), 
            self.value.clone(), 
            self.attributes.iter().map(|c| c.from_dto()).collect(), 
            children
        ))
    }

}