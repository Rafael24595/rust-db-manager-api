use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct DTOGenerateDatabaseQuery {
    pub data_base: String
}