use rust_db_manager_core::infrastructure::db_service_lite::DBServiceLite;
use serde::{Deserialize, Serialize};

use super::dto_db_service_web_category::DTODBServiceWebCategory;

#[derive(Clone, Serialize, Deserialize)]
pub struct DTODBServiceLite {
    pub name: String,
    pub category: DTODBServiceWebCategory
}

impl DTODBServiceLite {
    
    pub fn from_vec(collection: Vec<DBServiceLite>) -> Vec<DTODBServiceLite> {
        collection.iter().map(|s| DTODBServiceLite{name: s.name(), category: DTODBServiceWebCategory::from(s.category())}).collect()
    }

}