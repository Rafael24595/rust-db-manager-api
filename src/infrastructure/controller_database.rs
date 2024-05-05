use axum::{extract::Path, http::StatusCode, middleware, response::IntoResponse, routing::get, Json, Router};
use rust_db_manager_core::commons::configuration::configuration::Configuration;

use crate::commons::exception::api_exception::ApiException;

use super::{handler, utils};

pub struct ControllerDataBase {
}

impl ControllerDataBase {
    
    pub fn route(router: Router) -> Router {
        router
            .route("/:service/status", get(Self::status))
            .route("/:service/data-base", get(Self::data_bases))
            .route_layer(middleware::from_fn(handler::autentication_handler))
    }

    async fn status(Path(service): Path<String>) -> Result<(StatusCode, String), impl IntoResponse> {
        let o_db_service = Configuration::find_service(service);
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

    async fn data_bases(Path(service): Path<String>) -> Result<Json<Vec<String>>, impl IntoResponse> {
        let o_db_service = Configuration::find_service(service);
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

}