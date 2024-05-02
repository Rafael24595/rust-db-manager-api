use std::fmt;
use std::error::Error;

use super::api_exception::ApiException;

#[derive(Debug, Clone)]
pub struct AuthException {
    status: u16,
    message: String,
}

impl fmt::Display for AuthException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AuthException: {}", self.message)
    }
}

impl Error for AuthException {}


impl AuthException {
    
    pub fn from(status: u16, exception: ApiException) -> AuthException {
        AuthException {
            status: status,
            message: exception.message()
        }
    }

    pub fn new(status: u16, message: String) -> AuthException {
        AuthException {
            status,
            message
        }
    }
    
    pub fn status(&self) -> u16 {
        return self.status;
    }

    pub fn message(&self) -> String {
        return self.message.clone();
    }

}