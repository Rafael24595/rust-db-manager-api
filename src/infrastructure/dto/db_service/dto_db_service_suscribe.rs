use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct DTODBServiceSuscribe {
    pub name: String,
    pub password: String,
    pub owner: String,
}