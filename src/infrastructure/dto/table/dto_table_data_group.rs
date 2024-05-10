use rust_db_manager_core::domain::table::table_data_group::TableDataGroup;
use serde::Serialize;

use super::dto_table_data_field::DTOTableDataField;

#[derive(Clone, Serialize)]
pub struct DTOTableDataGroup {
    order: usize,
    name: String,
    fields: Vec<DTOTableDataField>,
}

impl DTOTableDataGroup {
    
    pub fn from(data: &TableDataGroup) -> Self {
        Self {
            order: data.order(),
            name: data.name(),
            fields: data.fields().iter()
                .map(|f| DTOTableDataField::from(f))
                .collect()
        }
    }

}