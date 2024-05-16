use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DTOQueryPagination {
    #[serde(default = "default_limit")]
    pub limit: usize,
    #[serde(default = "default_offset")]
    pub offset: usize,
}

fn default_limit() -> usize {
    0
}

fn default_offset() -> usize {
    10
}

impl DTOQueryPagination {
    
    pub fn new(offset: usize, limit: usize) -> DTOQueryPagination {
        DTOQueryPagination {
            offset, limit
        }
    }

}