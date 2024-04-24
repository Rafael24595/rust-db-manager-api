use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DTOQueryPagination {
    #[serde(default = "default_offset")]
    pub offset: usize,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_offset() -> usize {
    0
}

fn default_limit() -> usize {
    10
}

impl DTOQueryPagination {
    
    pub fn new(offset: usize, limit: usize) -> DTOQueryPagination {
        DTOQueryPagination {
            offset, limit
        }
    }

}