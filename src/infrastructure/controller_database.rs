use axum::{
    extract::Path,
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use rust_db_manager_core::{
    commons::configuration::configuration::Configuration,
    domain::{
        data_base::generate_database_query::GenerateDatabaseQuery,
        filter::data_base_query::DataBaseQuery,
    },
};

use crate::commons::exception::api_exception::ApiException;

use super::{
    dto::{
        data_base::dto_generate_data_base_query::DTOGenerateDatabaseQuery,
        table::group::dto_table_data_group::DTOTableDataGroup,
    },
    handler, utils,
};

pub struct ControllerDataBase {
}

impl ControllerDataBase {
    
    pub fn route(router: Router) -> Router {
        router
            .route("/api/v1/service/:service/data-base", get(Self::find_all))
            .route("/api/v1/service/:service/data-base", post(Self::insert))
            .route("/api/v1/service/:service/data-base/:data_base", delete(Self::delete))
            .route("/api/v1/service/:service/data-base/:data_base/metadata", get(Self::metadata))
            .route_layer(middleware::from_fn(handler::autentication_handler))
    }

    async fn find_all(Path(service): Path<String>) -> Result<Json<Vec<String>>, impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }
        
        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let collection = result.unwrap().data_base_find_all().await;
        if let Err(error) = collection {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        Ok(Json(collection.unwrap()))
    }

    async fn insert(Path(service): Path<String>, Json(dto): Json<DTOGenerateDatabaseQuery>) -> Result<StatusCode, impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }
        
        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let query = GenerateDatabaseQuery::new(dto.data_base);

        let collection = result.unwrap().data_base_create(&query).await;
        if let Err(error) = collection {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        Ok(StatusCode::ACCEPTED)
    }

    async fn delete(Path((service, data_base)): Path<(String, String)>) -> Result<StatusCode, impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }
        
        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let query = GenerateDatabaseQuery::new(data_base);

        let collection = result.unwrap().data_base_drop(&query).await;
        if let Err(error) = collection {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        Ok(StatusCode::ACCEPTED)
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

        let query = DataBaseQuery::from(data_base);

        let metadata = result.unwrap().data_base_metadata(&query).await;
        if let Err(error) = metadata {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        let dto = metadata.unwrap().iter()
            .map(|g| DTOTableDataGroup::from(g))
            .collect();

        Ok(Json(dto))
    }

}