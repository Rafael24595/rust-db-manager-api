use crate::commons::exception::api_exception::ApiException;

use super::{builder_jar::BuilderJar, cookie::Cookie};

#[derive(Debug, Clone)]
pub struct Jar {
    pub(crate) cookies: Vec<Cookie>
}

impl Jar {

    pub fn new() -> Self {
        Jar {
            cookies: Vec::new()
        }
    }

    pub fn from_string(jar_string: &str) -> Result<Self, ApiException> {
        BuilderJar::make(jar_string.to_string())
    }

    pub fn find(&self, code: &str) -> Option<Cookie> {
        self.cookies.iter().find(|c| c.code == code).cloned()
    }

}