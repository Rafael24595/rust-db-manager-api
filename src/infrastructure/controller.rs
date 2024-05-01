use axum::{body::Body, extract::{Path, Query}, http::{HeaderMap, Response, StatusCode}, middleware, response::IntoResponse, routing::{delete, get, post}, Json, Router};

use rust_db_manager_core::{commons::configuration::configuration::Configuration, infrastructure::{db_service::DBService, repository::e_db_repository::EDBRepository}};

use crate::{commons::{configuration::web_configuration::WebConfiguration, exception::api_exception::ApiException}, domain::builder_db_service::BuilderDBService};

use super::{db_assets::WebEDBRepository, dto::{db_service::{dto_db_service::DTODBService, dto_db_service_lite::DTODBServiceLite, dto_db_service_suscribe::DTODBServiceSuscribe, dto_db_service_web_category::DTODBServiceWebCategory}, dto_server_status::DTOServerStatus, pagination::{dto_paginated_collection::DTOPaginatedCollection, dto_query_pagination::DTOQueryPagination}}, handler, pagination::Pagination, services_jwt::ServicesJWT};

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
        
        let service = o_service.unwrap();

        let r_token = Controller::make_token(headers, service.clone());
        if r_token.is_err() {
            return Err(r_token.unwrap_err().into_response());
        }

        let token = r_token.unwrap();

        let db_service = Configuration::push_service(service.clone());
        if let Err(error) = db_service {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let mut builder = Response::builder();
        if token.is_some() {
            let cookie = format!("{}={}; Path=/; Secure; HttpOnly; SameSite=Lax", WebConfiguration::COOKIE_NAME, token.unwrap());
            builder = builder.header("Set-Cookie", cookie);
        } 

        let response = builder
            .status(StatusCode::ACCEPTED)
            .body(Body::empty())
            .unwrap();

        Ok(response)
    }

    async fn suscribe(headers: HeaderMap, Json(dto): Json<DTODBServiceSuscribe>) -> impl IntoResponse {
        let o_service = Configuration::find_service(dto.name);
        if o_service.is_none() {
            let exception = ApiException::new(StatusCode::NOT_FOUND.as_u16(), String::from("Service not found."));
            return Err(exception.into_response());
        }
        
        let service = o_service.unwrap();

        if service.is_authorized(dto.password).is_err() {
            let exception = ApiException::new(StatusCode::UNAUTHORIZED.as_u16(), String::from("Authentication error."));
            return Err(exception.into_response());
        }

        let r_token = Controller::make_token(headers, service.clone());
        if r_token.is_err() {
            return Err(r_token.unwrap_err().into_response());
        }

        let token = r_token.unwrap();

        let mut builder = Response::builder();
        if token.is_some() {
            let cookie = format!("{}={}; Path=/; Secure; HttpOnly; SameSite=Lax", WebConfiguration::COOKIE_NAME, token.unwrap());
            builder = builder.header("Set-Cookie", cookie);
        } 

        let response = builder
            .status(StatusCode::ACCEPTED)
            .body(Body::empty())
            .unwrap();

        Ok(response)
    }

    async fn service_status(Path(service): Path<String>) -> Result<(StatusCode, String), impl IntoResponse> {
        let db_service = Configuration::find_service(service);
        if db_service.is_none() {
            return Err(Controller::not_found());
        }
        
        let result = db_service.unwrap().instance().await;
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

    async fn service_remove(Path(service): Path<String>) -> Result<StatusCode, impl IntoResponse> {
        let db_service = Configuration::find_service(service);
        if db_service.is_none() {
            return Err(Controller::not_found());
        }

        Configuration::remove_service(db_service.unwrap());
        
        Ok(StatusCode::ACCEPTED)
    }

    fn make_token(headers: HeaderMap, service: DBService) -> Result<Option<String>, ApiException> {
        match headers.get(WebConfiguration::COOKIE_NAME) {
            Some(header) => {
                let token = header.to_str().unwrap().to_owned();
                if !service.is_protected() {
                    return Ok(Some(token));
                }

                let result = ServicesJWT::update(token, service.clone());
                if result.is_err() {
                    return Err(result.unwrap_err());
                }
                
                Ok(Some(result.unwrap()))
            },
            None => {
                if !service.is_protected() {
                    return Ok(None);
                }

                let result = ServicesJWT::sign(service.clone());
                if result.is_err() {
                    return Err(result.unwrap_err());
                }
    
                Ok(Some(result.unwrap()))
            },
        }
    }
    
    fn not_found() -> Response<Body> {
        let error = ApiException::new(
            StatusCode::NOT_FOUND.as_u16(),
            String::from("Not found"));
        return error.into_response();
    }

}