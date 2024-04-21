use rust_db_manager_core::infrastructure::db_service_lite::DBServiceLite;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct DTODBServiceLite {
    pub name: String,
    pub category: String
}

impl DTODBServiceLite {
    
    pub fn from_vec(collection: Vec<DBServiceLite>) -> Vec<DTODBServiceLite> {
        collection.iter().map(|s| DTODBServiceLite{name: s.name(), category: s.category().to_string()}).collect()
    }

}