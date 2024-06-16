use rust_db_manager_core::domain::table::definition::table_row_definition::TableRowDefinition;
use serde::Serialize;

use super::dto_table_field_definition::DTOTableFieldDefinition;

#[derive(Debug, Clone, Serialize)]
pub struct DTOTableRowDefinition {
    fields: Vec<DTOTableFieldDefinition>
}

impl DTOTableRowDefinition {
    
    pub fn from(rows: &TableRowDefinition) -> Self {
        Self {
            fields: rows.fields().iter()
                .map(|f| DTOTableFieldDefinition::from(f))
                .collect()
        }
    }

}