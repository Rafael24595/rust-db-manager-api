use std::sync::Arc;

use axum::{
    body::Body,
    http::{
        header::{COOKIE, SET_COOKIE},
        HeaderMap, Response, StatusCode,
    },
    response::IntoResponse,
};
use rust_db_manager_core::{
    commons::configuration::configuration::Configuration, infrastructure::db_service::DBService,
    service::service::Service,
};
use tokio::sync::Mutex;

use crate::{
    commons::{
        configuration::web_configuration::WebConfiguration,
        exception::{api_exception::ApiException, auth_exception::AuthException},
    },
    domain::cookie::{cookie::Cookie, jar::Jar},
};

use super::services_jwt::ServicesJWT;

impl IntoResponse for ApiException {

    fn into_response(self) -> Response<Body> {
        Response::builder()
        .status(self.status())
        .body(Body::from(self.message()))
        .unwrap()
    }

}

impl IntoResponse for AuthException {

    fn into_response(self) -> Response<Body> {
        let mut builder = Response::builder();
        if let Ok(cookie) = ServicesJWT::sign_empty() {
            if self.reset() {
                builder = builder.header(SET_COOKIE, cookie.to_string());
            }
        }
        
        builder
        .status(self.status())
        .body(Body::from(self.message()))
        .unwrap()
    }

}

pub(crate) async fn find_service(service: &str) -> Result<Arc<Mutex<Service>>, ApiException> {
    let config = Configuration::instance().await;
    if let Err(error) = config {
        let exception = ApiException::from_configuration_exception(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
        return Err(exception);
    }

    let config = config.unwrap();
    let locked_config = config.read().await;

    let service = locked_config.find_service(&service);
    if service.is_none() {
        return Err(not_found_exception());
    }

    let service = service.unwrap();
    let mut locked_service = service.write().await;

    locked_service
        .connection()
        .await
        .map_err(|e| ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e))
}

pub(crate) async fn find_service_schema(service: &str) -> Result<DBService, ApiException> {
    let config = Configuration::instance().await;
    if let Err(error) = config {
        let exception = ApiException::from_configuration_exception(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
        return Err(exception);
    }

    let config = config.unwrap();
    let locked_config = config.read().await;

    let service = locked_config.find_service(&service);
    if service.is_none() {
        return Err(not_found_exception());
    }

    let service = service.unwrap();
    let locked_service = service.read().await;
    let config = locked_service.configuration().clone();

    Ok(config)
}

pub(crate) fn find_token(headers: HeaderMap) -> Result<Option<Cookie>, AuthException> {
    let o_cookies = headers.get(COOKIE);
    if o_cookies.is_none() {
        return Ok(None);
    }

    let cookies = o_cookies.unwrap().to_str();
    if cookies.is_err() {
        let exception = AuthException::new_reset(StatusCode::UNAUTHORIZED.as_u16(), String::from("Token has non valid format"));
        return Err(exception);
    }

    let jar = Jar::from_string(cookies.unwrap());
    if jar.is_err() {
        return Err(jar.unwrap_err());
    }

    Ok(jar.unwrap().find(WebConfiguration::COOKIE_NAME))
}

pub(crate) fn not_found_exception() -> ApiException {
    ApiException::new(
        StatusCode::NOT_FOUND.as_u16(),
        String::from("Not found"))
}

pub(crate) fn not_found() -> Response<Body> {
    return not_found_exception().into_response();
}