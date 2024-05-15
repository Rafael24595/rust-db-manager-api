use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct DTORenameCollectionQuery {
    pub collection: String
}