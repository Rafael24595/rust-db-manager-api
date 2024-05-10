use serde::Deserialize;

use super::dto_db_connection_data::DTODBConnectionData;

#[derive(Clone, Deserialize)]
pub struct DTOServiceRequest {
    pub name: String,
    pub owner: String,
    pub protected: bool,
    pub password: String,
    pub connection_data: DTODBConnectionData
}