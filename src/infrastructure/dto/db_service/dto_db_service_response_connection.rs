use rust_db_manager_core::domain::connection_data::ConnectionData;
use serde::{Deserialize, Serialize};

use super::dto_db_resources::DTODBResources;
use crate::infrastructure::db_assets::WebEDBRepository;

#[derive(Clone, Serialize, Deserialize)]
pub struct DTODBServiceResponseConnection {
    pub category: String,
    pub connection: String,
    pub resources: DTODBResources
}

impl DTODBServiceResponseConnection {
    
    pub fn from(connection: ConnectionData) -> DTODBServiceResponseConnection {
        DTODBServiceResponseConnection {
            category: connection.category().to_string(),
            connection: connection.connection(),
            resources: connection.category().resources()
        }
    }

}