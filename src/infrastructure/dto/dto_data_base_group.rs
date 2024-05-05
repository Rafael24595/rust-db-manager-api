use rust_db_manager_core::domain::data_base_group_data::DataBaseDataGroup;
use serde::Serialize;

use super::dto_data_base_field::DTODataBaseField;

#[derive(Clone, Serialize)]
pub struct DTODataBaseGroup {
    order: usize,
    name: String,
    fields: Vec<DTODataBaseField>,
}

impl DTODataBaseGroup {
    
    pub fn from(data: &DataBaseDataGroup) -> DTODataBaseGroup {
        Self {
            order: data.order(),
            name: data.name(),
            fields: data.fields().iter()
                .map(|f| DTODataBaseField::from(f))
                .collect()
        }
    }

}