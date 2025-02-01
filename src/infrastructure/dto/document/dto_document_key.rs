use rust_db_manager_core::domain::{document::{document_key::DocumentKey, document_key_attribute::DocumentKeyAttribute}, e_json_type::EJSONType};
use serde::{Deserialize, Serialize};

use crate::commons::exception::api_exception::ApiException;

use super::dto_document_key_attribute::DTODocumentKeyAttribute;


#[derive(Clone, Serialize, Deserialize)]
pub struct DTODocumentKey {
    name: String,
    value: String,
    json_type: String,
    attributes: Vec<DTODocumentKeyAttribute>
}

impl DTODocumentKey {

    pub fn from(key: &DocumentKey) -> Self {
        Self {
            name: key.name().to_string(),
            value: key.value().to_string(),
            json_type: key.json_type().to_string(),
            attributes: key.attributes().iter()
                .map(|a| DTODocumentKeyAttribute::from(a))
                .collect()
        }
    }

    pub fn from_dto(&self) -> Result<DocumentKey, ApiException> {
        let jstype = EJSONType::from_string(&self.json_type);
        if let None = jstype {
            let exception = ApiException::new(422, String::from("Field type not recognized."));
            return Err(exception);
        }

        Ok(DocumentKey::new(
            self.name.clone(),
            self.value.clone(),
            jstype.unwrap(),
            self.attributes.iter()
                .map(|a| DocumentKeyAttribute::new(a.key.clone(), a.value.clone()))
                .collect()
        ))
    }

}