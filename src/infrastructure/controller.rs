use axum::{body::Body, extract::{Path, Query}, http::{header::SET_COOKIE, HeaderMap, Response, StatusCode}, middleware, response::IntoResponse, routing::{delete, get, post}, Json, Router};

use rust_db_manager_core::{commons::configuration::configuration::Configuration, infrastructure::{db_service::DBService, repository::e_db_repository::EDBRepository}};

use crate::{commons::{configuration::web_configuration::WebConfiguration, exception::{api_exception::ApiException, auth_exception::AuthException}}, domain::{builder_db_service::BuilderDBService, cookie::cookie::Cookie}};

use super::{db_assets::WebEDBRepository, dto::{db_service::{dto_db_service::DTODBService, dto_db_service_lite::DTODBServiceLite, dto_db_service_suscribe::DTODBServiceSuscribe, dto_db_service_web_category::DTODBServiceWebCategory}, dto_server_status::DTOServerStatus, pagination::{dto_paginated_collection::DTOPaginatedCollection, dto_query_pagination::DTOQueryPagination}}, handler, pagination::Pagination, services_jwt::ServicesJWT, utils::find_token};

pub struct Controller{
}

impl Controller {
    
    pub fn route(router: Router) -> Router {
        router
            .route("/:service/status", get(Controller::service_status))
            .route("/:service", delete(Controller::service_remove))
            .route_layer(middleware::from_fn(handler::autentication_handler))
            .route("/status", get(Controller::status))
            .route("/support", get(Controller::support))
            .route("/services", get(Controller::services))
            .route("/publish", post(Controller::publish))
            .route("/suscribe", post(Controller::suscribe))
    }

    async fn status() -> (StatusCode, Json<DTOServerStatus>) {
        let result = WebConfiguration::as_dto();
        (StatusCode::ACCEPTED, Json(result))
    }

    async fn support() -> (StatusCode, Json<Vec<DTODBServiceWebCategory>>) {
        let dto = EDBRepository::supported();
        (StatusCode::ACCEPTED, Json(dto))
    }

    async fn services(Query(params): Query<DTOQueryPagination>) -> (StatusCode, Json<DTOPaginatedCollection<DTODBServiceLite>>) {
        let services = Configuration::find_services();
        let dto = DTODBServiceLite::from_vec(services);
        let result = Pagination::paginate(params, dto);
        (StatusCode::ACCEPTED, Json(result))
    }

    async fn publish(headers: HeaderMap, Json(dto): Json<DTODBService>) -> impl IntoResponse {
        let o_service = BuilderDBService::make(dto);
        if let Err(error) = o_service {
            return Err(error.into_response());
        }
        
        let service = &o_service.unwrap();

        let r_cookie = Controller::make_token(headers, service);
        if let Err(exception) = r_cookie {
            return Err(exception.into_response());
        }

        let db_service = Configuration::push_service(service);
        if let Err(exception) = db_service {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), exception);
            return Err(exception.into_response());
        }

        Ok(Self::build_token_response(r_cookie.unwrap(), Body::empty()))
    }

    async fn suscribe(headers: HeaderMap, Json(dto): Json<DTODBServiceSuscribe>) -> impl IntoResponse {
        let o_db_service = Configuration::find_service(dto.name);
        if o_db_service.is_none() {
            let exception = ApiException::new(StatusCode::NOT_FOUND.as_u16(), String::from("Service not found."));
            return Err(exception.into_response());
        }
        
        let db_service = &o_db_service.unwrap();

        if db_service.is_authorized(dto.password).is_err() {
            let exception = ApiException::new(StatusCode::UNAUTHORIZED.as_u16(), String::from("Authentication error."));
            return Err(exception.into_response());
        }

        let r_cookie = Controller::make_token(headers, db_service);
        if let Err(exception) = r_cookie {
            return Err(exception.into_response());
        }

        Ok(Self::build_token_response(r_cookie.unwrap(), Body::empty()))
    }

    async fn service_status(Path(service): Path<String>) -> Result<(StatusCode, String), impl IntoResponse> {
        let o_db_service = Configuration::find_service(service);
        if o_db_service.is_none() {
            return Err(Controller::not_found());
        }
        
        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let status = result.unwrap().status().await;
        if let Err(error) = status {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        Ok((StatusCode::ACCEPTED, String::from("Service up.")))
    }

    async fn service_remove(headers: HeaderMap, Path(service): Path<String>) -> impl IntoResponse {
        let o_db_service = Configuration::find_service(service);
        if o_db_service.is_none() {
            return Err(Controller::not_found());
        }

        let db_service = o_db_service.unwrap();

        let r_cookie = Controller::remove_token(headers, &db_service);
        if let Err(exception) = r_cookie {
            return Err(exception.into_response());
        }

        Configuration::remove_service(db_service);

        Ok(Self::build_token_response(r_cookie.unwrap(), Body::empty()))
    }

    fn make_token(headers: HeaderMap, service: &DBService) -> Result<Option<Cookie>, AuthException> {
        let o_cookie = find_token(headers);
        if o_cookie.is_err() {
            return Err(o_cookie.unwrap_err());
        }

        match o_cookie.unwrap() {
            Some(cookie) => {
                if !service.is_protected() {
                    return Ok(Some(cookie));
                }
                
                Ok(Some(ServicesJWT::update(&cookie.value, service)?))
            },
            None => {
                if !service.is_protected() {
                    return Ok(None);
                }
                Ok(Some(ServicesJWT::sign(service)?))
            },
        }
    }

    fn remove_token(headers: HeaderMap, service: &DBService) -> Result<Option<Cookie>, AuthException> {
        let o_cookie = find_token(headers);
        if o_cookie.is_err() {
            return Err(o_cookie.unwrap_err());
        }

        match o_cookie.unwrap() {
            Some(cookie) => {
                if !service.is_protected() {
                    return Ok(Some(cookie));
                }
                Ok(Some(ServicesJWT::remove(&cookie.value, service)?))
            },
            None => return Ok(None),
        }
    }
    
    fn build_token_response(cookie: Option<Cookie>, body: Body) -> impl IntoResponse {
        let mut builder = Response::builder();
        if cookie.is_some() {
            builder = builder.header(SET_COOKIE, cookie.unwrap().to_string());
        }

        builder.status(StatusCode::OK)
            .body(body)
            .unwrap()
    }

    fn not_found() -> Response<Body> {
        let error = ApiException::new(
            StatusCode::NOT_FOUND.as_u16(),
            String::from("Not found"));
        return error.into_response();
    }

}