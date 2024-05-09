use rust_db_manager_core::domain::generate::generate_collection_query::GenerateCollectionQuery;
use serde::Deserialize;

use crate::{commons::exception::api_exception::ApiException, infrastructure::dto::definition::dto_field_data::DTOFieldData};

#[derive(Clone, Deserialize)]
pub struct DTOGenerateCollectionQuery {
    data_base: String,
    collection: String,
    fields: Option<Vec<DTOFieldData>>
}

impl DTOGenerateCollectionQuery {
    
    pub fn from_dto(&self) -> Result<GenerateCollectionQuery, ApiException> {
        let mut fields = Vec::new();
        if let Some(dtos) = self.fields.clone() {
            for dto in dtos {
                fields.push(dto.from_dto()?);
            }
        }
        Ok(GenerateCollectionQuery::new(self.data_base.clone(), self.collection.clone(), fields))
    }

}