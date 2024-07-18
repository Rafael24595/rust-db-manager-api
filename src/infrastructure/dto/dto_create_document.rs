use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct DTOCreateDocument {
    pub document: String
}