use axum::{extract::Path, http::StatusCode, middleware, response::IntoResponse, routing::{get, post}, Json, Router};
use rust_db_manager_core::{commons::{configuration::configuration::Configuration, utils::document_keys_to_filter_element}, domain::filter::data_base_query::DataBaseQuery};

use crate::commons::exception::api_exception::ApiException;

use super::{dto::{document::{dto_document_data::DTODocumentData, dto_document_key::DTODocumentKey}, table::dto_table_data_group::DTOTableDataGroup}, handler, utils};

pub struct ControllerDocument {
}

impl ControllerDocument {
    
    pub fn route(router: Router) -> Router {
        router
            .route("/:service/data-base/:data_base/collection/:collection/metadata", get(Self::metadata))
            .route("/:service/data-base/:data_base/collection/:collection", get(Self::find_all))
            .route("/:service/data-base/:data_base/collection/:collection/document", post(Self::find))
            .route_layer(middleware::from_fn(handler::autentication_handler))
    }

    async fn metadata(Path((service, data_base, collection)): Path<(String, String, String)>) -> Result<Json<Vec<DTOTableDataGroup>>, impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }

        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let query = DataBaseQuery::from(data_base, collection);

        let metadata = result.unwrap().collection_metadata(&query).await;
        if let Err(error) = metadata {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        let dto = metadata.unwrap().iter()
            .map(|g| DTOTableDataGroup::from(g))
            .collect();

        Ok(Json(dto))
    }

    async fn find_all(Path((service, data_base, collection)): Path<(String, String, String)>) -> Result<Json<Vec<DTODocumentData>>, impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }
        
        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let query = DataBaseQuery::from(data_base, collection);

        let documents = result.unwrap().find_all(&query).await;
        if let Err(error) = documents {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        Ok(Json(documents.unwrap().iter()
            .map(|d| DTODocumentData::from(d))
            .collect())
        )
    }

    async fn find(Path((service, data_base, collection)): Path<(String, String, String)>, Json(dto): Json<Vec<DTODocumentKey>>) -> Result<Json<DTODocumentData>, impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
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
        let query = DataBaseQuery::from_filter(data_base, collection, filter);

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

}