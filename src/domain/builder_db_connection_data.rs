use rust_db_manager_core::{domain::connection_data::ConnectionData, infrastructure::repository::e_db_repository::EDBRepository};
use url::Url;

use crate::{commons::exception::api_exception::ApiException, infrastructure::dto::service::generate::dto_db_connection_data::DTODBConnectionData};

pub struct BuilderConnectionData {
}

impl BuilderConnectionData {
    
    pub fn make(dto: DTODBConnectionData) -> Result<ConnectionData, ApiException> {
        let category = Self::identify(&dto);
        let data = ConnectionData::new(category.unwrap(), dto.connection);
        Ok(data)
    }

    fn identify(dto: &DTODBConnectionData) -> Result<EDBRepository, ApiException> {
        let category = &dto.category;
        if !category.is_empty() {
            let o_category = EDBRepository::from_string(category);
            if let None = o_category {
                let exception = ApiException::new(404, format!("Invalid scheme '{}'.", category));
                return Err(exception);    
            }
            return Ok(o_category.unwrap());
        }

        let url = Url::parse(&dto.connection);
        if let Err(err) = url {
            let exception = ApiException::new(404, err.to_string());
            return Err(exception);
        }
        
        let url = url.unwrap();
    
        match url.scheme().to_lowercase().as_str() {
            "mongodb" => Ok(EDBRepository::MongoDB),
            "postgres" => Ok(EDBRepository::Postgres),
            _ => {
                let exception = ApiException::new(404, format!("Invalid scheme '{}'.", url.scheme()));
                Err(exception)
            }
        }
    }

}