use axum::{extract::Path, http::StatusCode, middleware, response::IntoResponse, routing::{delete, get}, Json, Router};
use rust_db_manager_core::{commons::configuration::configuration::Configuration, domain::{filter::data_base_query::DataBaseQuery, generate::generate_collection_query::GenerateCollectionQuery}};

use crate::commons::exception::api_exception::ApiException;

use super::{dto::dto_data_base_group::DTODataBaseGroup, handler, utils};

pub struct ControllerCollection {
}

impl ControllerCollection {
    
    pub fn route(router: Router) -> Router {
        router
            .route("/:service/data-base/:data_base/metadata", get(Self::metadata))
            .route("/:service/data-base/:data_base/collection", get(Self::find_all))
            .route("/:service/data-base/:data_base/collection/:collection", delete(Self::delete))
            .route_layer(middleware::from_fn(handler::autentication_handler))
    }

    async fn metadata(Path((service, data_base)): Path<(String, String)>) -> Result<Json<Vec<DTODataBaseGroup>>, impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }

        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let query = DataBaseQuery::from_data_base(data_base);

        let metadata = result.unwrap().data_base_collections_metadata(&query).await;
        if let Err(error) = metadata {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        let dto = metadata.unwrap().iter()
            .map(|g| DTODataBaseGroup::from(g))
            .collect::<Vec<DTODataBaseGroup>>();

        Ok(Json(dto))
    }

    async fn find_all(Path((service, data_base)): Path<(String, String)>) -> Result<Json<Vec<String>>, impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }
        
        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let query = DataBaseQuery::from_data_base(data_base);

        let collections = result.unwrap().list_collections(&query).await;
        if let Err(error) = collections {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        Ok(Json(collections.unwrap()))
    }

    async fn delete(Path((service, data_base, collection)): Path<(String, String, String)>) -> Result<StatusCode, impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }
        
        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let query = GenerateCollectionQuery::new(data_base, collection);

        let collection = result.unwrap().drop_collection(&query).await;
        if let Err(error) = collection {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        Ok(StatusCode::ACCEPTED)
    }

}