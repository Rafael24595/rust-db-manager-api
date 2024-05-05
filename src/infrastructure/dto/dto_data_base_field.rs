use rust_db_manager_core::domain::data_base_field::DataBaseField;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct DTODataBaseField {
    order: usize,
    name: String,
    value: String,
    json_type: String,
}

impl DTODataBaseField {
    
    pub fn from(data: &DataBaseField) -> DTODataBaseField {
        Self {
            order: data.order(),
            name: data.name(),
            value: data.value(),
            json_type: data.json_type(),
        }
    }

}