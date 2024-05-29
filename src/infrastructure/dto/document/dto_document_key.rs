use rust_db_manager_core::domain::{document::{document_key::DocumentKey, document_key_attribute::DocumentKeyAttribute}, e_json_type::EJSONType};
use serde::{Deserialize, Serialize};

use crate::commons::exception::api_exception::ApiException;

use super::dto_document_key_attribute::DTODocumentKeyAttribute;


#[derive(Clone, Serialize, Deserialize)]
pub struct DTODocumentKey {
    name: String,
    value: String,
    jtype: String,
    attributes: Vec<DTODocumentKeyAttribute>
}

impl DTODocumentKey {

    pub fn from(key: &DocumentKey) -> Self {
        Self {
            name: key.name(),
            value: key.value(),
            jtype: key.jtype().to_string(),
            attributes: key.attributes().iter()
                .map(|a| DTODocumentKeyAttribute::from(a))
                .collect()
        }
    }

    pub fn from_dto(&self) -> Result<DocumentKey, ApiException> {
        let jstype = EJSONType::from_string(&self.jtype);
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