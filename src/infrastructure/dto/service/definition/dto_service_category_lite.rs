use rust_db_manager_core::infrastructure::repository::e_db_repository::{EDBRepository, EDBRepositoryItem};
use serde::Serialize;

use crate::infrastructure::db_assets::WebEDBRepository;

use super::dto_service_resources::DTOServiceResources;

#[derive(Clone, Serialize)]
pub struct DTOServiceCategoryLite {
    pub category: String,
    pub default: bool,
    pub resources: DTOServiceResources
}

impl DTOServiceCategoryLite {
    
    pub fn from(category: &EDBRepository) -> Self {
        Self {
            category: category.to_string(),
            default: false,
            resources: category.resources()
        }
    }

    pub fn from_item(category: &EDBRepositoryItem) -> Self {
        Self {
            category: category.item.to_string(),
            default: category.default,
            resources: category.item.resources()
        }
    }

}