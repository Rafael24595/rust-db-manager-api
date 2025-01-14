use rust_db_manager_core::domain::{e_json_type::EJSONType, field::generate::field_data::FieldData};
use serde::{Deserialize, Serialize};

use crate::commons::exception::api_exception::ApiException;

use super::{dto_field_atribute::DTOFieldAttribute, dto_field_reference::DTOFieldReference};

#[derive(Clone, Serialize, Deserialize)]
pub struct DTOFieldData {
    order: i32,
    code: String,
    value: String,
    swkey: bool,
    swsize: bool,
    size: i32,
    mutable: bool,
    json_type: String,
    attributes: Vec<DTOFieldAttribute>,
    reference: Vec<DTOFieldReference>
}

impl DTOFieldData {
    
    pub fn from(field: &FieldData) -> Self {
        Self {
            order: field.order(),
            code: field.code().to_string(),
            value: field.value().to_string(),
            swkey: field.is_key(),
            swsize: field.is_resize(),
            size: field.size(),
            mutable: field.is_mutable(),
            json_type: field.json_type().to_string(),
            attributes: field.attributes().iter()
                .map(|a| DTOFieldAttribute::from(a))
                .collect(),
            reference: field.reference().iter()
                .map(|r| DTOFieldReference::from(r))
                .collect()
        }
    }

    pub fn from_dto(&self) -> Result<FieldData, ApiException> {
        let json_type = EJSONType::from_string(&self.json_type);
        if json_type.is_none() {
            let exception = ApiException::from_message(422, "Not valid JSON type");
            return Err(exception);
        }

        let attributes = self.attributes.iter()
            .map(|a| a.from_dto())
            .collect();

        let reference = self.reference.iter()
            .map(|a| a.from_dto())
            .collect();

        Ok(FieldData::new(
            self.order, self.code.clone(), self.value.clone(), 
            self.swkey, self.swsize, self.size, self.mutable, 
            json_type.unwrap(), attributes, reference
        ))
    }

}