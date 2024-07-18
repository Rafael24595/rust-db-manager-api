use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct DTOFilterResources {
    root_category: String,
    query_category: String,
    categories: Vec<String>
}

impl DTOFilterResources {
    
    pub fn new(root_category: String, query_category: String, categories: Vec<String>) -> Self {
        Self {
            root_category,
            query_category,
            categories
        }
    }

}