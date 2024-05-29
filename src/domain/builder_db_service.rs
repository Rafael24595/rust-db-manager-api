use rust_db_manager_core::infrastructure::db_service::DBService;

use crate::{commons::exception::api_exception::ApiException, infrastructure::dto::service::generate::dto_service_create_request::DTOServiceRequest};

use super::builder_db_connection_data::BuilderConnectionData;

pub struct BuilderDBService {
}

impl BuilderDBService {
    
    pub fn make(dto: DTOServiceRequest) -> Result<DBService, ApiException> {
        let connection_data = BuilderConnectionData::make(dto.connection_data)?;
        
        let service = DBService::new(dto.name, dto.owner, dto.protected, dto.password, connection_data);
        if service.is_err() {
            return Err(ApiException::from(500, service.err().unwrap()));
        }

        Ok(service.unwrap())
    }

}