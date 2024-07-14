use std::fmt;
use std::error::Error;

use rust_db_manager_core::commons::exception::configuration_exception::ConfigurationException;

use super::api_exception::ApiException;

#[derive(Debug, Clone)]
pub struct AuthException {
    status: u16,
    message: String,
    reset: bool,
}

impl fmt::Display for AuthException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AuthException: {}", self.message)
    }
}

impl Error for AuthException {}


impl AuthException {
    
    pub fn from(status: u16, exception: ApiException, reset: bool) -> AuthException {
        AuthException {
            status: status,
            message: exception.message(),
            reset: reset
        }
    }

    pub fn from_configuration_exception(status: u16, exception: ConfigurationException, reset: bool) -> AuthException {
        AuthException {
            status: status,
            message: exception.message(),
            reset: reset
        }
    }

    pub fn new(status: u16, message: String) -> AuthException {
        AuthException {
            status,
            message,
            reset: false
        }
    }

    pub fn new_reset(status: u16, message: String) -> AuthException {
        AuthException {
            status,
            message,
            reset: true
        }
    }
    
    pub fn status(&self) -> u16 {
        return self.status;
    }

    pub fn message(&self) -> String {
        return self.message.clone();
    }

    pub fn reset(&self) -> bool {
        return self.reset;
    }

}