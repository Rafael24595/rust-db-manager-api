use std::collections::HashMap;

use axum::{extract::{Path, Request}, http::{HeaderMap, StatusCode}, middleware::Next, response::{IntoResponse, Response}};
use rust_db_manager_core::commons::configuration::configuration::Configuration;

use crate::commons::exception::api_exception::ApiException;

use super::{services_jwt::ServicesJWT, utils::find_token};

pub(crate) async fn autentication_handler(headers: HeaderMap, Path(params): Path<HashMap<String, String>>, request: Request, next: Next) -> Result<Response, impl IntoResponse> {
    let service = params.get("service");
    if service.is_none() {
        let exception = ApiException::new(StatusCode::NOT_FOUND.as_u16(), String::from("Service not defined."));
        return Err(exception.into_response());
    }

    let r_service = Configuration::find_service(&service.unwrap());
    if let Err(error) = r_service {
        let exception = ApiException::from_configuration_exception(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
        return Err(exception.into_response());
    }

    let o_service = r_service.unwrap();
    if o_service.is_none() {
        let exception = ApiException::new(StatusCode::NOT_FOUND.as_u16(), String::from("Service not found."));
        return Err(exception.into_response());
    }

    let service = o_service.unwrap();
    if service.is_protected() {
        let o_token = find_token(headers);
        if let Err(exception) = o_token {
            return Err(exception.into_response());
        }

        let token = o_token.unwrap();
        if token.is_none() {
            let exception = ApiException::new(StatusCode::UNAUTHORIZED.as_u16(), String::from("Token not found"));
            return Err(exception.into_response());
        }

        let result = ServicesJWT::verify(&token.unwrap().value);
        if let Err(exception) = result {
            return Err(exception.into_response());
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