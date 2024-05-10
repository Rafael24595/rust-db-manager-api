use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct DTOServiceResources {
    pub web_site: String,
    pub color: String,
    pub image: String
}

impl DTOServiceResources {
    
    pub fn new(web_site: String, color: String, image: String) -> Self {
        Self {
            web_site, color, image
        }
    }

}