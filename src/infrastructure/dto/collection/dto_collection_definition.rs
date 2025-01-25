use rust_db_manager_core::domain::collection::collection_definition::CollectionDefinition;
use serde::Serialize;

use crate::infrastructure::dto::field::{definition::{dto_field_attribute_definition::DTOFieldAttributeDefinition, dto_field_definition::DTOFieldDefinition}, generate::dto_field_data::DTOFieldData};

use super::dto_collection_reference_definition::DtoCollectionReferenceDefinition;

#[derive(Clone, Serialize)]
pub struct DTOCollectionDefinition {
    swrelational: bool,
    definition: Vec<DTOFieldDefinition>,
    defaults: Vec<DTOFieldData>,
    global_attributes: Vec<DTOFieldAttributeDefinition>,
    references: Vec<DtoCollectionReferenceDefinition>
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
                .collect(),
            global_attributes: definition.global_attributes().iter()
                .map(|d| DTOFieldAttributeDefinition::from(d))
                .collect(),
            references: definition.references().iter()
                .map(|r| DtoCollectionReferenceDefinition::from(r))
                .collect()
        }
    }

}