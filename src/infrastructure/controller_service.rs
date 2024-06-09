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
        collection::dto_collection_definition::DTOCollectionDefinition,
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
        table::dto_table_data_group::DTOTableDataGroup,
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
            .route("/api/v1/service/:service/schema", get(Self::schema))
            .route("/api/v1/service/:service/schema-filter", get(Self::schema_filter))
            .route_layer(middleware::from_fn(handler::autentication_handler))

            .route("/api/v1/service", get(Self::find_all))
            .route("/api/v1/service", post(Self::insert))
            .route("/api/v1/service", patch(Self::suscribe))
    }

    async fn find(Path(service): Path<String>) -> Result<Json<DTOService>,impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }
        
        Ok(Json(DTOService::from(o_db_service.unwrap())))
    } 

    async fn delete(headers: HeaderMap, Path(service): Path<String>) -> impl IntoResponse {
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

    async fn status(Path(service): Path<String>) -> Result<(StatusCode, String), impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
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
    
        Ok((StatusCode::ACCEPTED, String::from("listening")))
    }

    async fn metadata(Path(service): Path<String>) -> Result<Json<Vec<DTOTableDataGroup>>, impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }
        
        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let metadata = result.unwrap().metadata().await;
        if let Err(error) = metadata {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        let dto = metadata.unwrap().iter()
            .map(|g| DTOTableDataGroup::from(g))
            .collect();

        Ok(Json(dto))
    }

    async fn schema(Path(service): Path<String>) -> Result<Json<DTOCollectionDefinition>, impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }
        
        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let definition = result.unwrap().collection_accept_schema().await;
        if let Err(error) = definition {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
        
        Ok(Json(DTOCollectionDefinition::from(definition.unwrap())))
    }

    async fn schema_filter(Path(service): Path<String>) -> Result<Json<DTOFilterDefinition>, impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }
        
        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let schema = result.unwrap().filter_schema().await;
        if let Err(error) = schema {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        Ok(Json(DTOFilterDefinition::from(&schema.unwrap())))
    }

    async fn find_all(Query(params): Query<DTOQueryPagination>) -> (StatusCode, Json<DTOPaginatedCollection<DTOServiceLite>>) {
        let services = Configuration::find_services();
        let dto = services.iter().map(|s| DTOServiceLite::from(s)).collect();
        let result = Pagination::paginate(params, dto);
        (StatusCode::ACCEPTED, Json(result))
    }

    async fn insert(headers: HeaderMap, Json(dto): Json<DTOServiceRequest>) -> impl IntoResponse {
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