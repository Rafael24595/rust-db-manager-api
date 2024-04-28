use axum::{body::Body, extract::{Path, Query}, http::{HeaderMap, Response, StatusCode}, response::IntoResponse, routing::{get, post}, Json, Router};

use rust_db_manager_core::{commons::configuration::configuration::Configuration, infrastructure::db_service::DBService};

use crate::{commons::{configuration::web_configuration::WebConfiguration, exception::api_exception::ApiException}, domain::builder_db_service::BuilderDBService};

use super::{dto::{db_service::{dto_db_service::DTODBService, dto_db_service_lite::DTODBServiceLite, dto_db_service_suscribe::DTODBServiceSuscribe}, dto_server_status::DTOServerStatus, pagination::{dto_paginated_collection::DTOPaginatedCollection, dto_query_pagination::DTOQueryPagination}}, pagination::Pagination, services_jwt::ServicesJWT};

pub struct Controller{
}

impl Controller {
    
    pub fn route(router: Router) -> Router {
        router
            .route("/status", get(Controller::status))
            .route("/services", get(Controller::services))
            .route("/publish", post(Controller::service_publish))
            .route("/suscribe", post(Controller::service_suscribe))
            .route("/:service/status", get(Controller::service_status))
    }

    async fn status() -> (StatusCode, Json<DTOServerStatus>) {
        let result = WebConfiguration::as_dto();
        (StatusCode::ACCEPTED, Json(result))
    }

    async fn services(Query(params): Query<DTOQueryPagination>) -> (StatusCode, Json<DTOPaginatedCollection<DTODBServiceLite>>) {
        let services = Configuration::find_services();
        let dto = DTODBServiceLite::from_vec(services);
        let result = Pagination::paginate(params, dto);
        (StatusCode::ACCEPTED, Json(result))
    }

    async fn service_publish(headers: HeaderMap, Json(dto): Json<DTODBService>) -> impl IntoResponse {
        let o_service = BuilderDBService::make(dto);
        if let Err(error) = o_service {
            return Err(error.into_response());
        }
        
        let service = o_service.unwrap();

        let token = Controller::make_token(headers, service.clone());
        if token.is_err() {
            return Err(token.unwrap_err().into_response());
        }

        let db_service = Configuration::push_service(service.clone());
        if let Err(error) = db_service {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let response = Response::builder()
            .header("db-token", token.unwrap())
            .status(StatusCode::ACCEPTED)
            .body(Body::from(service.name()))
            .unwrap();

        Ok(response)
    }

    async fn service_suscribe(headers: HeaderMap, Json(dto): Json<DTODBServiceSuscribe>) -> impl IntoResponse {
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

        let token = Controller::make_token(headers, service.clone());
        if token.is_err() {
            return Err(token.unwrap_err().into_response());
        }

        let response = Response::builder()
            .header("db-token", token.unwrap())
            .status(StatusCode::ACCEPTED)
            .body(Body::from(service.name()))
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

    fn make_token(headers: HeaderMap, service: DBService) -> Result<String, ApiException> {
        match headers.get("db-token") {
            Some(header) => {
                let result = ServicesJWT::update(header.to_str().unwrap().to_owned(), service.clone());
                if result.is_err() {
                    return Err(result.unwrap_err());
                }
                
                Ok(result.unwrap())
            },
            None => {
                let result = ServicesJWT::sign(service.clone());
                if result.is_err() {
                    return Err(result.unwrap_err());
                }
    
                Ok(result.unwrap())
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