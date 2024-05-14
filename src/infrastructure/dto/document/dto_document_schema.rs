use rust_db_manager_core::domain::document::document_schema::DocumentSchema;
use serde::Serialize;

use crate::infrastructure::dto::field::generate::dto_field_data::DTOFieldData;

#[derive(Clone, Serialize)]
pub struct DTODocumentSchema {
    comments: Vec<String>,
    fields: Vec<DTOFieldData>
}

impl DTODocumentSchema {
    
    pub fn from(schema: &DocumentSchema) -> Self {
        Self {
            comments: schema.comments(),
            fields: schema.fields().iter()
                .map(|f| DTOFieldData::from(f))
                .collect()
        }
    }

}