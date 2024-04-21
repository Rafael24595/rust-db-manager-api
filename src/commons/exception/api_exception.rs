use std::fmt;
use std::error::Error;

use rust_db_manager_core::commons::exception::connect_exception::ConnectException;

pub(crate) const EXCEPTION_HEADER: &str = "Error-Code";

#[derive(Debug, Clone)]
pub struct ApiException {
    status: u16,
    message: String,
}

impl fmt::Display for ApiException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ApiException: {}", self.message)
    }
}

impl Error for ApiException {}


impl ApiException {
    
    pub fn from(status: u16, exception: ConnectException) -> ApiException {
        ApiException {
            status: status,
            message: exception.message()
        }
    }

    pub fn new(status: u16, message: String) -> ApiException {
        ApiException {
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