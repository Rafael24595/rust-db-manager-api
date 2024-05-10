use rust_db_manager_core::domain::collection::collection_definition::CollectionDefinition;
use serde::Serialize;

use crate::infrastructure::dto::field::{definition::dto_field_definition::DTOFieldDefinition, generate::dto_field_data::DTOFieldData};

#[derive(Clone, Serialize)]
pub struct DTOCollectionDefinition {
    swrelational: bool,
    definition: Vec<DTOFieldDefinition>,
    defaults: Vec<DTOFieldData>
}

impl DTOCollectionDefinition {
    
    pub fn from(definition: CollectionDefinition) -> Self {
        Self {
            swrelational: definition.is_relational(),
            definition: definition.definition().iter()
                .map(|f| DTOFieldDefinition::from(f))
                .collect(),
            defaults: definition.defaults().iter()
                .map(|d| DTOFieldData::from(d))
                .collect()
        }
    }

}