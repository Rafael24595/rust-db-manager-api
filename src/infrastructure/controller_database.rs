use axum::{extract::Path, http::StatusCode, middleware, response::IntoResponse, routing::{delete, get, post}, Json, Router};
use rust_db_manager_core::{commons::configuration::configuration::Configuration, domain::generate::generate_database_query::GenerateDatabaseQuery};

use crate::commons::exception::api_exception::ApiException;

use super::{dto::{db_service::db::dto_db_create::DTODBCreate, dto_data_base_group::DTODataBaseGroup}, handler, utils};

pub struct ControllerDataBase {
}

impl ControllerDataBase {
    
    pub fn route(router: Router) -> Router {
        router
            .route("/:service/status", get(Self::status))
            .route("/:service/metadata", get(Self::metadata))
            .route("/:service/data-base", get(Self::find_all))
            .route("/:service/data-base", post(Self::insert))
            .route("/:service/data-base/:data_base", delete(Self::delete))
            .route_layer(middleware::from_fn(handler::autentication_handler))
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

    async fn metadata(Path(service): Path<String>) -> Result<Json<Vec<DTODataBaseGroup>>, impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }
        
        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let metadata = result.unwrap().data_base_metadata().await;
        if let Err(error) = metadata {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        let dto = metadata.unwrap().iter()
            .map(|g| DTODataBaseGroup::from(g))
            .collect::<Vec<DTODataBaseGroup>>();

        Ok(Json(dto))
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

        let collection = result.unwrap().list_data_bases().await;
        if let Err(error) = collection {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        Ok(Json(collection.unwrap()))
    }

    async fn insert(Path(service): Path<String>, Json(dto): Json<DTODBCreate>) -> Result<StatusCode, impl IntoResponse> {
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

        let collection = result.unwrap().create_data_base(&query).await;
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

        let collection = result.unwrap().drop_data_base(&query).await;
        if let Err(error) = collection {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        Ok(StatusCode::ACCEPTED)
    }

}