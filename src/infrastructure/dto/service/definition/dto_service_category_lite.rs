use rust_db_manager_core::infrastructure::repository::e_db_repository::EDBRepository;
use serde::Serialize;

use crate::infrastructure::db_assets::WebEDBRepository;

use super::dto_service_resources::DTOServiceResources;

#[derive(Clone, Serialize)]
pub struct DTOServiceCategoryLite {
    pub category: String,
    pub resources: DTOServiceResources
}

impl DTOServiceCategoryLite {
    
    pub fn from(category: &EDBRepository) -> Self {
        Self {
            category: category.to_string(),
            resources: category.resources()
        }
    }

}