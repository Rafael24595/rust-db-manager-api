use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct DTODBCreate {
    pub data_base: String
}