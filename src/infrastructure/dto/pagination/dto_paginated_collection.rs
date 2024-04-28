use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct DTOPaginatedCollection<T> {
    pub total: usize,
    pub previous: usize,
    pub next: usize,
    pub services: Vec<T>,
}

impl <T> DTOPaginatedCollection<T> {
    
    pub fn new(total: usize, previous: usize, next: usize, services: Vec<T>) -> DTOPaginatedCollection<T> {
        DTOPaginatedCollection {total, previous, next, services}
    }

}