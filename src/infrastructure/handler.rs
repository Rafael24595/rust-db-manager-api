use axum::{extract::{Path, Request}, http::{HeaderMap, StatusCode}, middleware::Next, response::{IntoResponse, Response}};
use rust_db_manager_core::commons::configuration::configuration::Configuration;

use crate::commons::{configuration::web_configuration::WebConfiguration, exception::api_exception::ApiException};

use super::services_jwt::ServicesJWT;

pub(crate) async fn autentication_handler(headers: HeaderMap, Path(service): Path<String>, request: Request, next: Next) -> Result<Response, impl IntoResponse> {
    let o_service = Configuration::find_service(service);
    if o_service.is_none() {
        let exception = ApiException::new(StatusCode::NOT_FOUND.as_u16(), String::from("Service not found."));
        return Err(exception.into_response());
    }

    let service = o_service.unwrap();
    if service.is_protected() {
        let o_token = headers.get(String::from(WebConfiguration::COOKIE_NAME));
        if o_token.is_none() {
            let error = ApiException::new(StatusCode::UNAUTHORIZED.as_u16(), String::from("Token not found"));
            return Err(error.into_response());
        }

        let token = o_token.unwrap().to_str().unwrap().to_owned();

        let result = ServicesJWT::verify(token.clone());
        if result.is_err() {
            return Err(result.unwrap_err().into_response());
        }

        let services = result.unwrap();

        let authorized = services.iter().find(|s| s.is_same(service.clone()));
        if authorized.is_none() {
            let error = ApiException::new(StatusCode::UNAUTHORIZED.as_u16(), String::from("Token not found"));
            return Err(error.into_response());
        }
    }
    
    return Ok(next.run(request).await);
}