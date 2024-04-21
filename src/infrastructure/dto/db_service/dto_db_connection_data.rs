use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct DTOConnectionData {
    pub category: String,
    pub connection: String
}