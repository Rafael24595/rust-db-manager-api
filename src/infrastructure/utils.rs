use axum::{body::Body, http::{header::COOKIE, HeaderMap, Response, StatusCode}, response::IntoResponse};

use crate::{commons::{configuration::web_configuration::WebConfiguration, exception::api_exception::ApiException}, domain::cookie::jar::Jar};

impl IntoResponse for ApiException {

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

pub(crate) fn find_token_strict(headers: HeaderMap) -> Result<String, ApiException> {
    let o_cookies = headers.get(COOKIE);
    if o_cookies.is_none() {
        let error = ApiException::new(StatusCode::UNAUTHORIZED.as_u16(), String::from("Cookies not found"));
        return Err(error);
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
        let exception = ApiException::new(StatusCode::UNAUTHORIZED.as_u16(), String::from("Token not found"));
        return Err(exception);
    }

    Ok(o_cookie.unwrap().value)
}