use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct DTODBResources {
    pub web_site: String,
    pub color: String,
    pub image: String
}

impl DTODBResources {
    
    pub fn new(web_site: String, color: String, image: String) -> DTODBResources {
        DTODBResources {
            web_site, color, image
        }
    }

}