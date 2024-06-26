use rust_db_manager_core::{domain::connection_data::ConnectionData, infrastructure::repository::e_db_repository::EDBRepository};

use crate::{commons::exception::api_exception::ApiException, infrastructure::dto::service::generate::dto_db_connection_data::DTODBConnectionData};

pub struct BuilderConnectionData {
}

impl BuilderConnectionData {
    
    pub fn make(dto: DTODBConnectionData) -> Result<ConnectionData, ApiException> {
        let category = EDBRepository::from_string(&dto.category);
        if let None = category {
            let message = format!("Data base type '{}' not supported.", dto.category);
            return Err(ApiException::new(404, message));
        }

        let data = ConnectionData::new(category.unwrap(), dto.connection);
        
        Ok(data)
    }

}