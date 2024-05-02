use axum::{body::Body, http::{header::{COOKIE, SET_COOKIE}, HeaderMap, Response, StatusCode}, response::IntoResponse};

use crate::{commons::{configuration::web_configuration::WebConfiguration, exception::{api_exception::ApiException, auth_exception::AuthException}}, domain::cookie::jar::Jar};

use super::services_jwt::ServicesJWT;

impl IntoResponse for ApiException {

    fn into_response(self) -> Response<Body> {
        let mut builder = Response::builder();
        if let Ok(token) = ServicesJWT::sign_empty() {
            let cookie = format!("{}={}; Path=/; HttpOnly", WebConfiguration::COOKIE_NAME, token);
            builder = builder.header(SET_COOKIE, cookie);
        }
        
        builder
        .status(self.status())
        .body(Body::from(self.message()))
        .unwrap()
    }

}

impl IntoResponse for AuthException {

    fn into_response(self) -> Response<Body> {
        Response::builder()
        .status(self.status())
        .body(Body::from(self.message()))
        .unwrap()
    }

}

pub(crate) fn find_token(headers: HeaderMap) -> Result<Option<String>, ApiException> {
    let o_cookies = headers.get(COOKIE);
    if o_cookies.is_none() {
        return Ok(None);
    }

    let cookies = o_cookies.unwrap().to_str();
    if cookies.is_err() {
        let exception = ApiException::new(StatusCode::UNAUTHORIZED.as_u16(), String::from("Token has non valid format"));
        return Err(exception);
    }

    let jar = Jar::from_string(cookies.unwrap());
    if jar.is_err() {
        return Err(jar.unwrap_err());
    }

    let o_cookie = jar.unwrap().find(WebConfiguration::COOKIE_NAME);
    if o_cookie.is_none() {
        return Ok(None);
    }

    Ok(Some(o_cookie.unwrap().value))
}