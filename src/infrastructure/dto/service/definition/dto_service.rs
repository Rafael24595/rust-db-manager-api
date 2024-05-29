use rust_db_manager_core::infrastructure::db_service::DBService;
use serde::Serialize;

use super::dto_service_category::DTOServiceCategory;

#[derive(Clone, Serialize)]
pub struct DTOService {
    pub name: String,
    pub owner: String,
    pub protected: bool,
    pub timestamp: u128,
    pub connection_data: DTOServiceCategory,
}

impl DTOService {
    
    pub fn from(service: DBService) -> Self {
        Self {
            name: service.name(),
            owner: service.owner(),
            protected: service.is_protected(),
            timestamp: service.timestamp(),
            connection_data: DTOServiceCategory::from(service.connection_data())
        }
    }

}