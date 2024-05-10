use axum::{extract::Path, http::StatusCode, middleware, response::IntoResponse, routing::{delete, get, post}, Json, Router};
use rust_db_manager_core::{commons::configuration::configuration::Configuration, domain::{collection::generate_collection_query::GenerateCollectionQuery, filter::data_base_query::DataBaseQuery}};

use crate::commons::exception::api_exception::ApiException;

use super::{dto::{collection::dto_generate_collection_query::DTOGenerateCollectionQuery, table::dto_table_data_group::DTOTableDataGroup}, handler, utils};

pub struct ControllerCollection {
}

impl ControllerCollection {
    
    pub fn route(router: Router) -> Router {
        router
            .route("/:service/data-base/:data_base/metadata", get(Self::metadata))
            .route("/:service/data-base/:data_base/collection", get(Self::find_all))
            .route("/:service/data-base/:data_base/collection", post(Self::insert))
            .route("/:service/data-base/:data_base/collection/:collection", delete(Self::delete))
            .route_layer(middleware::from_fn(handler::autentication_handler))
    }

    async fn metadata(Path((service, data_base)): Path<(String, String)>) -> Result<Json<Vec<DTOTableDataGroup>>, impl IntoResponse> {
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
            .map(|g| DTOTableDataGroup::from(g))
            .collect();

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

    async fn insert(Path((service, _)): Path<(String, String)>, Json(dto): Json<DTOGenerateCollectionQuery>) -> Result<StatusCode, impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }
        
        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let query = dto.from_dto();
        if let Err(exception) = query {
            return Err(exception.into_response());
        }

        let collection = result.unwrap().create_collection(&query.unwrap()).await;
        if let Err(error) = collection {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        Ok(StatusCode::ACCEPTED)
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

        let query = GenerateCollectionQuery::from_collection(data_base, collection);

        let collection = result.unwrap().drop_collection(&query).await;
        if let Err(error) = collection {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        Ok(StatusCode::ACCEPTED)
    }

}