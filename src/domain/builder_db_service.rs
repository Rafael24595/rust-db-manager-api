use rust_db_manager_core::infrastructure::db_service::DBService;

use crate::{commons::exception::api_exception::ApiException, infrastructure::dto::db_service::dto_db_service::DTODBService};

use super::builder_db_connection_data::BuilderConnectionData;

pub struct BuilderDBService {
}

impl BuilderDBService {
    
    pub fn make(dto: DTODBService) -> Result<DBService, ApiException> {
        let connection_data = BuilderConnectionData::make(dto.connection_data)?;
        let service = DBService::new(dto.name, dto.owner, connection_data);
        Ok(service)
    }

}