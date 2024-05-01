use serde::Deserialize;

use super::dto_db_connection_data::DTOConnectionData;

#[derive(Clone, Deserialize)]
pub struct DTODBService {
    pub name: String,
    pub owner: String,
    pub protected: bool,
    pub password: String,
    pub connection_data: DTOConnectionData
}