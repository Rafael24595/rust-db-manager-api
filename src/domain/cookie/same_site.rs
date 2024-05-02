use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum SameSite {
    Strict, 
    Lax,
    None
}

impl Display for SameSite {

    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
    
}

impl SameSite {
    
    pub fn from_string(category: &str) -> Option<SameSite> {
        match category.to_lowercase().as_str() {
            "strict" => Some(SameSite::Strict),
            "lax" => Some(SameSite::Lax),
            "none" => Some(SameSite::None),
            _ => None,
        }
    }

}