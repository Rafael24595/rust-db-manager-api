use rust_db_manager_core::domain::table::group::table_data_field::TableDataField;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct DTOTableDataField {
    order: usize,
    name: String,
    value: String,
    data_type: String,
    json_type: String,
}

impl DTOTableDataField {
    
    pub fn from(data: &TableDataField) -> Self {
        Self {
            order: data.order(),
            name: data.name(),
            value: data.value(),
            data_type: data.data_type().to_string(),
            json_type: data.json_type(),
        }
    }

}