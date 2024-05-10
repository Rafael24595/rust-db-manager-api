use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct DTODBConnectionData {
    pub category: String,
    pub connection: String
}