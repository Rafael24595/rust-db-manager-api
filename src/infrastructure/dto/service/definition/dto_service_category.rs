use rust_db_manager_core::domain::connection_data::ConnectionData;
use serde::Serialize;

use crate::infrastructure::db_assets::WebEDBRepository;

use super::dto_service_resources::DTOServiceResources;

#[derive(Clone, Serialize)]
pub struct DTOServiceCategory {
    pub category: String,
    pub connection: String,
    pub resources: DTOServiceResources
}

impl DTOServiceCategory {
    
    pub fn from(connection: ConnectionData) -> Self {
        Self {
            category: connection.category().to_string(),
            connection: connection.connection(),
            resources: connection.category().resources()
        }
    }

}