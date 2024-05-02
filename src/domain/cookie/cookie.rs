use crate::commons::exception::api_exception::ApiException;

use super::{builder_cookie::BuilderCookie, same_site::SameSite};

#[derive(Debug, Clone)]
pub struct Cookie {
    pub code: String,
    pub value: String,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub expiration: Option<String>,
    pub max_age: Option<u32>,
    pub secure: Option<bool>,
    pub http_only: Option<bool>,
    pub same_site: Option<SameSite>,
}

impl Cookie {

    pub fn new(code: String, value: String) -> Self {
        Cookie {
            code: code, value: value, domain: None, 
            path: None, expiration: None, max_age: None,
            secure: None, http_only: None, same_site: None
        }
    }

    pub fn to_string(&self) -> String {
        let mut cookie_string = format!("{}", self.value);

        if let Some(domain) = &self.domain {
            cookie_string.push_str(&format!("; Domain={}", domain));
        }

        if let Some(path) = &self.path {
            cookie_string.push_str(&format!("; Path={}", path));
        }
        
        if let Some(expiration) = &self.expiration {
            cookie_string.push_str(&format!("; Expires={}", expiration));
        }
        
        if let Some(max_age) = self.max_age {
            cookie_string.push_str(&format!("; Max-Age={}", max_age));
        }

        if let Some(true) = self.secure {
            cookie_string.push_str("; Secure");
        }

        if let Some(true) = self.http_only {
            cookie_string.push_str("; HttpOnly");
        }

        if let Some(same_site) = &self.same_site {
            cookie_string.push_str(&format!("; SameSite={}", same_site));
        }

        cookie_string
    }

    pub fn from_string(cookie_string: &str) -> Result<Self, ApiException> {
        BuilderCookie::make(cookie_string)
    }

}