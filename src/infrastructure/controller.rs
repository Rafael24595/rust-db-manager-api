use axum::{body::Body, extract::{Path, Query}, http::{Response, StatusCode}, response::IntoResponse, routing::{get, post}, Json, Router};

use rust_db_manager_core::commons::configuration::configuration::Configuration;

use crate::{commons::exception::api_exception::ApiException, domain::builder_db_service::BuilderDBService};

use super::{dto::{db_service::{dto_db_service::DTODBService, dto_db_service_lite::DTODBServiceLite, dto_paginated_collection::DTOPaginatedCollection}, dto_query_pagination::DTOQueryPagination}, pagination::Pagination};

pub struct Controller{
}

impl Controller {
    
    pub fn route(router: Router) -> Router {
        router
            .route("/services", get(Controller::services))
            .route("/:service", post(Controller::insert_service))
            .route("/:service/status", get(Controller::status))
    }

    async fn services(Query(params): Query<DTOQueryPagination>) -> (StatusCode, Json<DTOPaginatedCollection<DTODBServiceLite>>) {
        let services = Configuration::find_services();
        let dto = DTODBServiceLite::from_vec(services);
        let result = Pagination::paginate(params, dto);
        (StatusCode::ACCEPTED, Json(result))
    }

    async fn insert_service(Json(dto): Json<DTODBService>) -> Result<(StatusCode, String), impl IntoResponse> {
        let o_service = BuilderDBService::make(dto);
        if let Err(error) = o_service {
            return Err(error.into_response());
        }

        let service = o_service.unwrap();

        let db_service = Configuration::push_service(service.clone());
        if let Err(error) = db_service {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        Ok((StatusCode::ACCEPTED, service.name()))
    }

    async fn status(Path(service): Path<String>) -> Result<(StatusCode, String), impl IntoResponse> {
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

    fn not_found() -> Response<Body> {
        let error = ApiException::new(
            StatusCode::NOT_FOUND.as_u16(),
            String::from("Not found"));
        return error.into_response();
    }

}