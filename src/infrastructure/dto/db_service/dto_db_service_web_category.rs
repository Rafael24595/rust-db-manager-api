use rust_db_manager_core::infrastructure::{db_service_lite::DBServiceLite, repository::e_db_repository::EDBRepository};
use serde::{Deserialize, Serialize};

use crate::infrastructure::{db_assets::WebEDBRepository, dto::dto_db_resources::DTODBResources};

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