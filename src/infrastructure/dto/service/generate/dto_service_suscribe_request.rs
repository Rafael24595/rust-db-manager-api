use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct DTOServiceSuscribeRequest {
    pub name: String,
    pub password: String,
    pub owner: String,
}