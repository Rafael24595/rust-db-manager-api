use axum::{
    body::Body,
    extract::{Path, Query},
    http::{header::SET_COOKIE, HeaderMap, Response, StatusCode},
    middleware,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};

use rust_db_manager_core::{
    commons::configuration::configuration::Configuration, infrastructure::db_service::DBService,
};

use crate::{
    commons::exception::{api_exception::ApiException, auth_exception::AuthException},
    domain::{builder_db_service::BuilderDBService, cookie::cookie::Cookie},
};

use super::{
    dto::{
        field::filter::definition::dto_filter_definition::DTOFilterDefinition,
        pagination::{
            dto_paginated_collection::DTOPaginatedCollection,
            dto_query_pagination::DTOQueryPagination,
        },
        service::{
            definition::{dto_service::DTOService, dto_service_lite::DTOServiceLite},
            generate::{
                dto_service_create_request::DTOServiceRequest,
                dto_service_suscribe_request::DTOServiceSuscribeRequest,
            },
        },
        table::group::dto_table_data_group::DTOTableDataGroup,
    },
    handler,
    pagination::Pagination,
    services_jwt::ServicesJWT,
    utils::{self, find_token},
};

pub struct ControllerService {
}

impl ControllerService {
    
    pub fn route(router: Router) -> Router {
        router
            .route("/api/v1/service/:service", get(Self::find))
            .route("/api/v1/service/:service", delete(Self::delete))
            .route("/api/v1/service/:service/status", get(Self::status))
            .route("/api/v1/service/:service/metadata", get(Self::metadata))
            .route("/api/v1/service/:service/schema-filter", get(Self::schema_filter))
            .route_layer(middleware::from_fn(handler::autentication_handler))

            .route("/api/v1/service", get(Self::find_all))
            .route("/api/v1/service", post(Self::insert))
            .route("/api/v1/service", patch(Self::suscribe))
    }

    async fn find(Path(service): Path<String>) -> Result<Json<DTOService>,impl IntoResponse> {
        let result = utils::find_service_schema(&service).await;
        if let Err(error) = result {
            return Err(error.into_response());
        }    
        
        Ok(Json(DTOService::from(result.unwrap())))
    } 

    async fn delete(headers: HeaderMap, Path(service): Path<String>) -> impl IntoResponse {
        let result = utils::find_service_schema(&service).await;
        if let Err(error) = result {
            return Err(error.into_response());
        }

        let db_service = result.unwrap();

        let r_cookie = Self::remove_token(headers, &db_service).await;
        if let Err(exception) = r_cookie {
            return Err(exception.into_response());
        }

        let config = Configuration::instance().await;
        if let Err(error) = config {
            let exception = ApiException::from_configuration_exception(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let config = config.unwrap();
        let mut locked_config = config.write().await;

        if let Err(error) = locked_config.remove_service(db_service).await {
            let exception = ApiException::from_configuration_exception(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        Ok(Self::build_token_response(r_cookie.unwrap(), Body::empty()))
    }

    async fn status(Path(service): Path<String>) -> Result<(StatusCode, String), impl IntoResponse> {
        let result = utils::find_service(&service).await;
        if let Err(error) = result {
            return Err(error.into_response());
        }

        let result = result.unwrap();
        let locked_result = result.lock().await;

        let status = locked_result.status().await;
        if let Err(error) = status {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        Ok((StatusCode::ACCEPTED, String::from("listening")))
    }

    async fn metadata(Path(service): Path<String>) -> Result<Json<Vec<DTOTableDataGroup>>, impl IntoResponse> {
        let result = utils::find_service(&service).await;
        if let Err(error) = result {
            return Err(error.into_response());
        }

        let result = result.unwrap();
        let locked_result = result.lock().await;

        let metadata = locked_result.metadata().await;
        if let Err(error) = metadata {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        let dto = metadata.unwrap().iter()
            .map(|g| DTOTableDataGroup::from(g))
            .collect();

        Ok(Json(dto))
    }

    async fn schema_filter(Path(service): Path<String>) -> Result<Json<DTOFilterDefinition>, impl IntoResponse> {
        let result = utils::find_service(&service).await;
        if let Err(error) = result {
            return Err(error.into_response());
        }

        let result = result.unwrap();
        let locked_result = result.lock().await;

        let schema = locked_result.filter_schema().await;
        if let Err(error) = schema {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        Ok(Json(DTOFilterDefinition::from(&schema.unwrap())))
    }

    async fn find_all(Query(params): Query<DTOQueryPagination>) -> Result<Json<DTOPaginatedCollection<DTOServiceLite>>, impl IntoResponse> {
        let config = Configuration::instance().await;
        if let Err(error) = config {
            let exception = ApiException::from_configuration_exception(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception);
        }

        let config = config.unwrap();
        let locked_config = config.read().await;

        let dto = locked_config.find_services().await.iter().map(|s| DTOServiceLite::from(s)).collect();
        let result = Pagination::paginate(params, dto);
        Ok(Json(result))
    }

    async fn insert(headers: HeaderMap, Json(dto): Json<DTOServiceRequest>) -> impl IntoResponse {
        let o_service = BuilderDBService::make(dto);
        if let Err(error) = o_service {
            return Err(error.into_response());
        }
        
        let service = o_service.unwrap();

        let r_cookie = Self::make_token(headers, &service).await;
        if let Err(exception) = r_cookie {
            return Err(exception.into_response());
        }

        let config = Configuration::instance().await;
        if let Err(error) = config {
            let exception = ApiException::from_configuration_exception(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let config = config.unwrap();
        let mut locked_config = config.write().await;

        let db_service = locked_config.push_service(service).await;
        if let Err(error) = db_service {
            let exception = ApiException::from_configuration_exception(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        Ok(Self::build_token_response(r_cookie.unwrap(), Body::empty()))
    }

    async fn suscribe(headers: HeaderMap, Json(dto): Json<DTOServiceSuscribeRequest>) -> impl IntoResponse {
        let result = utils::find_service_schema(&dto.name).await;
        if let Err(error) = result {
            return Err(error.into_response());
        }
        
        let db_service = &result.unwrap();

        if db_service.is_authorized(dto.password).is_err() {
            let exception = ApiException::new(StatusCode::UNAUTHORIZED.as_u16(), String::from("Authentication error."));
            return Err(exception.into_response());
        }

        let r_cookie = Self::make_token(headers, db_service).await;
        if let Err(exception) = r_cookie {
            return Err(exception.into_response());
        }

        Ok(Self::build_token_response(r_cookie.unwrap(), Body::empty()))
    }

    async fn make_token(headers: HeaderMap, service: &DBService) -> Result<Option<Cookie>, AuthException> {
        let o_cookie = find_token(headers);
        if o_cookie.is_err() {
            return Err(o_cookie.unwrap_err());
        }

        match o_cookie.unwrap() {
            Some(cookie) => {
                if !service.is_protected() {
                    return Ok(Some(cookie));
                }
                Ok(Some(ServicesJWT::update(&cookie.value, service).await?))
            },
            _ => {
                if !service.is_protected() {
                    return Ok(None);
                }
                Ok(Some(ServicesJWT::sign(service)?))
            },
        }
    }

    async fn remove_token(headers: HeaderMap, service: &DBService) -> Result<Option<Cookie>, AuthException> {
        let o_cookie = find_token(headers);
        if o_cookie.is_err() {
            return Err(o_cookie.unwrap_err());
        }

        match o_cookie.unwrap() {
            Some(cookie) => {
                if !service.is_protected() {
                    return Ok(Some(cookie));
                }
                Ok(Some(ServicesJWT::remove(&cookie.value, service).await?))
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