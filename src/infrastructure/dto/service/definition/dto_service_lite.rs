use rust_db_manager_core::infrastructure::db_service_lite::DBServiceLite;
use serde::Serialize;

use super::dto_service_category_lite::DTOServiceCategoryLite;

#[derive(Clone, Serialize)]
pub struct DTOServiceLite {
    pub name: String,
    pub category: DTOServiceCategoryLite
}

impl DTOServiceLite {
    
    pub fn from(service: &DBServiceLite) -> Self {
        Self {
            name: service.name(), 
            category: DTOServiceCategoryLite::from(&service.category())
        }
    }

}