use rust_db_manager_core::infrastructure::db_service::DBService;
use serde::{Deserialize, Serialize};

use super::dto_db_service_response_connection::DTODBServiceResponseConnection;

#[derive(Clone, Serialize, Deserialize)]
pub struct DTODBServiceResponse {
    pub name: String,
    pub owner: String,
    pub protected: bool,
    pub timestamp: u128,
    pub connection_data: DTODBServiceResponseConnection,
}

impl DTODBServiceResponse {
    
    pub fn from(service: DBService) -> DTODBServiceResponse {
        DTODBServiceResponse {
            name: service.name(),
            owner: service.owner(),
            protected: service.is_protected(),
            timestamp: service.timestamp(),
            connection_data: DTODBServiceResponseConnection::from(service.connection_data())
        }
    }

}