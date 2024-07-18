use rust_db_manager_core::domain::table::definition::table_definition::TableDefinition;
use serde::Serialize;

use super::dto_table_row_definition::DTOTableRowDefinition;

#[derive(Debug, Clone, Serialize)]
pub struct DTOTableDefinition {
    title: String,
    rows: Vec<DTOTableRowDefinition>,
}

impl DTOTableDefinition {

    pub fn from(table: &TableDefinition) -> Self {
        Self {
            title: table.title(),
            rows: table.rows().iter()
                .map(|r| DTOTableRowDefinition::from(r))
                .collect()
        }
    }

}