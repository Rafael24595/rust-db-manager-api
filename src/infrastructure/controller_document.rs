use axum::{
    extract::{Path, Query},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use rust_db_manager_core::{
    commons::{
        configuration::configuration::Configuration, utils::document_keys_to_filter_element,
    },
    domain::filter::{collection_query::CollectionQuery, document_query::DocumentQuery},
};

use crate::commons::exception::api_exception::ApiException;

use super::{
    dto::{
        collection::dto_collection_data::DTOCollectionData,
        document::{dto_document_data::DTODocumentData, dto_document_key::DTODocumentKey},
        dto_create_document::DTOCreateDocument,
        dto_update_document::DTOUpdateDocument,
        field::filter::dto_filter_element::DTOFilterElement,
        pagination::dto_query_pagination::DTOQueryPagination,
    },
    handler, utils,
};

pub struct ControllerDocument {
}

impl ControllerDocument {
    
    pub fn route(router: Router) -> Router {
        router
            .route("/api/v1/service/:service/data-base/:data_base/collection/:collection/document/find", get(Self::find_all))
            .route("/api/v1/service/:service/data-base/:data_base/collection/:collection/document/find", post(Self::find))
            .route("/api/v1/service/:service/data-base/:data_base/collection/:collection/document/query", post(Self::query))
            .route("/api/v1/service/:service/data-base/:data_base/collection/:collection/document/action", post(Self::insert))
            .route("/api/v1/service/:service/data-base/:data_base/collection/:collection/document/action", put(Self::update))
            .route("/api/v1/service/:service/data-base/:data_base/collection/:collection/document/action", delete(Self::delete))
            .route_layer(middleware::from_fn(handler::autentication_handler))
    }

    async fn find_all(Path((service, data_base, collection)): Path<(String, String, String)>, Query(params): Query<DTOQueryPagination>) -> Result<Json<DTOCollectionData>, impl IntoResponse> {
        let r_db_service = Configuration::find_service(&service);
        if let Err(error) = r_db_service {
            let exception = ApiException::from_configuration_exception(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let o_db_service = r_db_service.unwrap();
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }
        
        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let query = DocumentQuery::from(data_base, collection, Some(params.limit), Some(params.offset), None);

        let data = result.unwrap().find_all(&query).await;
        if let Err(error) = data {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        Ok(Json(DTOCollectionData::from(&data.unwrap())))
    }

    async fn find(Path((service, data_base, collection)): Path<(String, String, String)>, Json(dto): Json<Vec<DTODocumentKey>>) -> Result<Json<DTODocumentData>, impl IntoResponse> {
        let r_db_service = Configuration::find_service(&service);
        if let Err(error) = r_db_service {
            let exception = ApiException::from_configuration_exception(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let o_db_service = r_db_service.unwrap();
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }
        
        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let mut keys = Vec::new();
        for dto_key in dto {
            let key = dto_key.from_dto();
            if let Err(exception) = key {
                return Err(exception.into_response());
            }
            keys.push(key.unwrap());
        }

        let filter = document_keys_to_filter_element(keys);
        let query = DocumentQuery::from_filter(data_base, collection, filter);

        let r_document = result.unwrap().find(&query).await;
        if let Err(error) = r_document {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let document = r_document.unwrap();

        if let None = document {
            let exception = ApiException::new(StatusCode::NOT_FOUND.as_u16(), String::from("Document not found."));
            return Err(exception.into_response());
        }
    
        Ok(Json(DTODocumentData::from(&document.unwrap())))
    }

    async fn query(Path((service, data_base, collection)): Path<(String, String, String)>, Query(params): Query<DTOQueryPagination>, Json(dto): Json<DTOFilterElement>) -> Result<Json<DTOCollectionData>, impl IntoResponse> {
        let r_db_service = Configuration::find_service(&service);
        if let Err(error) = r_db_service {
            let exception = ApiException::from_configuration_exception(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let o_db_service = r_db_service.unwrap();
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }
        
        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let filter = dto.from_dto();
        if let Err(exception) = filter {
            return Err(exception.into_response());
        }

        let query = DocumentQuery::from(data_base, collection, Some(params.limit), Some(params.offset), Some(filter.unwrap()));

        let data = result.unwrap().find_query(&query).await;
        if let Err(error) = data {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        Ok(Json(DTOCollectionData::from(&data.unwrap())))
    }


    async fn insert(Path((service, data_base, collection)): Path<(String, String, String)>, Json(dto): Json<DTOCreateDocument>) -> Result<Json<DTODocumentData>, impl IntoResponse> {
        let r_db_service = Configuration::find_service(&service);
        if let Err(error) = r_db_service {
            let exception = ApiException::from_configuration_exception(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let o_db_service = r_db_service.unwrap();
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }
        
        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let query = CollectionQuery::from(data_base, collection);

        let document = result.unwrap().insert(&query, &dto.document).await;
        if let Err(error) = document {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        Ok(Json(DTODocumentData::from(&document.unwrap())))
    }

    async fn update(Path((service, data_base, collection)): Path<(String, String, String)>, Json(dto): Json<DTOUpdateDocument>) -> Result<Json<Vec<DTODocumentData>>, impl IntoResponse> {
        let r_db_service = Configuration::find_service(&service);
        if let Err(error) = r_db_service {
            let exception = ApiException::from_configuration_exception(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let o_db_service = r_db_service.unwrap();
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }
        
        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let mut keys = Vec::new();
        for dto_key in dto.keys {
            let key = dto_key.from_dto();
            if let Err(exception) = key {
                return Err(exception.into_response());
            }
            keys.push(key.unwrap());
        }

        let filter = document_keys_to_filter_element(keys);
        let query = DocumentQuery::from_filter(data_base, collection, filter);

        let documents = result.unwrap().update(&query, &dto.document).await;
        if let Err(error) = documents {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        Ok(Json(documents.unwrap().iter()
            .map(|d| DTODocumentData::from(d))
            .collect())
        )
    }

    async fn delete(Path((service, data_base, collection)): Path<(String, String, String)>, Json(dto): Json<Vec<DTODocumentKey>>) -> Result<Json<Vec<DTODocumentData>>, impl IntoResponse> {
        let r_db_service = Configuration::find_service(&service);
        if let Err(error) = r_db_service {
            let exception = ApiException::from_configuration_exception(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let o_db_service = r_db_service.unwrap();
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }
        
        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let mut keys = Vec::new();
        for dto_key in dto {
            let key = dto_key.from_dto();
            if let Err(exception) = key {
                return Err(exception.into_response());
            }
            keys.push(key.unwrap());
        }

        let filter = document_keys_to_filter_element(keys);
        let query = DocumentQuery::from_filter(data_base, collection, filter);

        let documents = result.unwrap().delete(&query).await;
        if let Err(error) = documents {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        Ok(Json(documents.unwrap().iter()
            .map(|d| DTODocumentData::from(d))
            .collect())
        )
    }

}