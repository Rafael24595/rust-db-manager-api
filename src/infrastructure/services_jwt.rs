use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

use rust_db_manager_core::{commons::configuration::configuration::Configuration, infrastructure::db_service::DBService};

use crate::commons::exception::api_exception::ApiException;

pub struct ServicesJWT {

}

impl ServicesJWT {
    
    pub fn sign(service: &DBService) -> Result<String, ApiException> {
        ServicesJWT::sign_services(Vec::from(vec![service.clone()]))
    }

    pub fn sign_empty() -> Result<String, ApiException> {
        ServicesJWT::sign_services(Vec::new())
    }

    pub fn sign_services(services: Vec<DBService>) -> Result<String, ApiException> {
        let s_key = services.iter()
            .map(|s| s.salt())
            .collect::<Vec<String>>()
            .join("#");

        let key: Result<Hmac<Sha256>, hmac::digest::InvalidLength> = Hmac::new_from_slice(s_key.as_bytes());
        if key.is_err() {
            let exception = ApiException::new(500, key.unwrap_err().to_string());
            return Err(exception);
        }
        
        let collection = services.iter()
            .map(|s| s.name())
            .collect::<Vec<String>>()
            .join("-");
        
        let mut claims = BTreeMap::new();
        claims.insert("sub", collection);

        let token_str = claims.sign_with_key(&key.unwrap());
        if token_str.is_err() {
            let exception = ApiException::new(500, token_str.unwrap_err().to_string());
            return Err(exception);
        }
        
        Ok(token_str.unwrap())
    }

    pub fn update(token: &str, service: &DBService) -> Result<String, ApiException> {
        let _ = ServicesJWT::verify(token)?;
        
        let mut services = ServicesJWT::find_services(token)?;
        if services.iter().find(|s| s.name() == service.name()).is_some() {
            let exception = ApiException::new(500, String::from("This token is already subscribed to the service."));
            return Err(exception);
        }
        
        services.push(service.clone());

        ServicesJWT::sign_services(services)
    }

    pub fn remove(token: &str, service: &DBService) -> Result<String, ApiException> {
        let _ = ServicesJWT::verify(token)?;
        
        let mut services = ServicesJWT::find_services(token)?;
        if let Some(position) = services.iter().position(|s| s.name() == service.name()) {
            services.remove(position);
        }

        ServicesJWT::sign_services(services)
    }

    pub fn verify(token: &str) -> Result<Vec<DBService>, ApiException> {
        let services = ServicesJWT::find_services(token)?;
        let salt = services.iter()
            .map(|s| s.salt())
            .collect::<Vec<String>>()
            .join("#");

        let key: Result<Hmac<Sha256>, hmac::digest::InvalidLength> = Hmac::new_from_slice(salt.as_bytes());
        if key.is_err() {
            let exception = ApiException::new(500, key.unwrap_err().to_string());
            return Err(exception);
        }

        let result: Result<BTreeMap<String, String>, jwt::Error> = token.verify_with_key(&key.unwrap());
        if result.is_err() {
            let exception = ApiException::new(500, result.unwrap_err().to_string());
            return Err(exception);
        }

        Ok(services)
    }

    fn find_services(token: &str) -> Result<Vec<DBService>, ApiException> {
        let fragments = token.split(".").collect::<Vec<&str>>();
        if fragments.len() != 3 {
            let exception = ApiException::new(401, String::from("Invalid token."));
            return Err(exception);
        }

        let claims = fragments.get(1).unwrap().trim();
        let b_services = base64::decode_config(claims, base64::URL_SAFE_NO_PAD);
        if b_services.is_err() {
            let exception = ApiException::new(500, b_services.unwrap_err().to_string());
            return Err(exception);
        }

        let s_services = String::from_utf8(b_services.unwrap());
        if s_services.is_err() {
            let exception = ApiException::new(500, s_services.unwrap_err().to_string());
            return Err(exception);
        }

        let m_services: Result<BTreeMap<String, String>, serde_json::Error> = serde_json::from_str(&s_services.unwrap());
        if m_services.is_err() {
            let exception = ApiException::new(500, m_services.unwrap_err().to_string());
            return Err(exception);
        }

        let b_m_services = m_services.unwrap();
        let v_services = b_m_services.get("sub");

        if v_services.is_none() {
            let exception = ApiException::new(500, String::from("No services found."));
            return Err(exception);
        }

        let mut collection = Vec::new();

        for s_service in v_services.unwrap().split("-").filter(|s| !s.is_empty()).collect::<Vec<&str>>() {
            let service =  Configuration::find_service(String::from(s_service));
            if service.is_none() {
                let exception = ApiException::new(500, String::from("Unknown service."));
                return Err(exception);
            }
            collection.push(service.unwrap());
        }

        Ok(collection)
    }

}