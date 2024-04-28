use rust_db_manager_core::infrastructure::repository::e_db_repository::EDBRepository;
use serde::{Deserialize, Serialize};

use super::dto_db_resources::DTODBResources;
use crate::infrastructure::db_assets::WebEDBRepository;

#[derive(Clone, Serialize, Deserialize)]
pub struct DTODBServiceWebCategory {
    pub category: String,
    pub resources: DTODBResources
}

impl DTODBServiceWebCategory {
    
    pub fn from(category: EDBRepository) -> DTODBServiceWebCategory {
        DTODBServiceWebCategory {
            category: category.to_string(),
            resources: category.resources()
        }
    }

}