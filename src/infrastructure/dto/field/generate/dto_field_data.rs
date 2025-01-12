use rust_db_manager_core::domain::field::generate::field_data::FieldData;
use serde::{Deserialize, Serialize};

use crate::commons::exception::api_exception::ApiException;

use super::{dto_field_atribute::DTOFieldAttribute, dto_field_reference::DTOFieldReference};

#[derive(Clone, Serialize, Deserialize)]
pub struct DTOFieldData {
    order: i32,
    code: String,
    value: String,
    swsize: bool,
    size: i32,
    mutable: bool,
    attributes: Vec<DTOFieldAttribute>,
    reference: Vec<DTOFieldReference>
}

impl DTOFieldData {
    
    pub fn from(field: &FieldData) -> Self {
        Self {
            order: field.order(),
            code: field.code().to_string(),
            value: field.value().to_string(),
            swsize: field.is_resize(),
            size: field.size(),
            mutable: field.is_mutable(),
            attributes: field.attributes().iter()
                .map(|a| DTOFieldAttribute::from(a))
                .collect(),
            reference: field.reference().iter()
                .map(|r| DTOFieldReference::from(r))
                .collect()
        }
    }

    pub fn from_dto(&self) -> Result<FieldData, ApiException> {
        let attributes = self.attributes.iter()
            .map(|a| a.from_dto())
            .collect();

        let reference = self.reference.iter()
            .map(|a| a.from_dto())
            .collect();

        Ok(FieldData::new(
            self.order, self.code.clone(), self.value.clone(), 
            self.swsize, self.size, self.mutable, 
            attributes, reference
        ))
    }

}