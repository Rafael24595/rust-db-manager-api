use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DTOQueryPagination {
    pub offset: usize,
    pub limit: usize,
}