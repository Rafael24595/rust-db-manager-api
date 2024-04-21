use rust_db_manager_core::infrastructure::db_service::DBService;

use crate::infrastructure::dto::db_service::dto_db_service::DTODBService;

pub trait DTOBuilder<T, K> {
    fn from_dto(dto: K) -> T;
}

impl DTOBuilder<DBService, DTODBService> for DBService {

    fn from_dto(dto: DTODBService) -> DBService {
        
    }

}