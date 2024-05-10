use axum::{body::Body, extract::{Path, Query}, http::{header::SET_COOKIE, HeaderMap, Response, StatusCode}, middleware, response::IntoResponse, routing::{delete, get, post}, Json, Router};

use rust_db_manager_core::{commons::configuration::configuration::Configuration, infrastructure::{db_service::DBService, repository::e_db_repository::EDBRepository}};

use crate::{commons::{configuration::web_configuration::WebConfiguration, exception::{api_exception::ApiException, auth_exception::AuthException}}, domain::{builder_db_service::BuilderDBService, cookie::cookie::Cookie}};

use super::{db_assets::WebEDBRepository, dto::{dto_server_status::DTOServerStatus, pagination::{dto_paginated_collection::DTOPaginatedCollection, dto_query_pagination::DTOQueryPagination}, service::{definition::{dto_service::DTOService, dto_service_category_lite::DTOServiceCategoryLite, dto_service_lite::DTOServiceLite}, generate::{dto_service_create_request::DTOServiceRequest, dto_service_suscribe_request::DTOServiceSuscribeRequest}}}, handler, pagination::Pagination, services_jwt::ServicesJWT, utils::{self, find_token}};

pub struct ControllerServer {
}

impl ControllerServer {
    
    pub fn route(router: Router) -> Router {
        router
            .route("/:service", get(Self::service_find))
            .route("/:service", delete(Self::service_remove))
            .route_layer(middleware::from_fn(handler::autentication_handler))
            .route("/metadata", get(Self::metadata))
            .route("/support", get(Self::support))
            .route("/services", get(Self::services))
            .route("/publish", post(Self::publish))
            .route("/suscribe", post(Self::suscribe))
    }

    async fn service_find(Path(service): Path<String>) -> Result<Json<DTOService>,impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }
        
        Ok(Json(DTOService::from(o_db_service.unwrap())))
    } 

    async fn service_remove(headers: HeaderMap, Path(service): Path<String>) -> impl IntoResponse {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }

        let db_service = o_db_service.unwrap();

        let r_cookie = Self::remove_token(headers, &db_service);
        if let Err(exception) = r_cookie {
            return Err(exception.into_response());
        }

        Configuration::remove_service(db_service);

        Ok(Self::build_token_response(r_cookie.unwrap(), Body::empty()))
    }

    async fn metadata() -> (StatusCode, Json<DTOServerStatus>) {
        let result = WebConfiguration::as_dto();
        (StatusCode::ACCEPTED, Json(result))
    }

    async fn support() -> (StatusCode, Json<Vec<DTOServiceCategoryLite>>) {
        let dto = EDBRepository::supported();
        (StatusCode::ACCEPTED, Json(dto))
    }

    async fn services(Query(params): Query<DTOQueryPagination>) -> (StatusCode, Json<DTOPaginatedCollection<DTOServiceLite>>) {
        let services = Configuration::find_services();
        let dto = services.iter().map(|s| DTOServiceLite::from(s)).collect();
        let result = Pagination::paginate(params, dto);
        (StatusCode::ACCEPTED, Json(result))
    }

    async fn publish(headers: HeaderMap, Json(dto): Json<DTOServiceRequest>) -> impl IntoResponse {
        let o_service = BuilderDBService::make(dto);
        if let Err(error) = o_service {
            return Err(error.into_response());
        }
        
        let service = &o_service.unwrap();

        let r_cookie = Self::make_token(headers, service);
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

    async fn suscribe(headers: HeaderMap, Json(dto): Json<DTOServiceSuscribeRequest>) -> impl IntoResponse {
        let o_db_service = Configuration::find_service(&dto.name);
        if o_db_service.is_none() {
            let exception = ApiException::new(StatusCode::NOT_FOUND.as_u16(), String::from("Service not found."));
            return Err(exception.into_response());
        }
        
        let db_service = &o_db_service.unwrap();

        if db_service.is_authorized(dto.password).is_err() {
            let exception = ApiException::new(StatusCode::UNAUTHORIZED.as_u16(), String::from("Authentication error."));
            return Err(exception.into_response());
        }

        let r_cookie = Self::make_token(headers, db_service);
        if let Err(exception) = r_cookie {
            return Err(exception.into_response());
        }

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

}