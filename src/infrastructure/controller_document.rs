use axum::{extract::Path, http::StatusCode, middleware, response::IntoResponse, routing::get, Json, Router};
use rust_db_manager_core::{commons::configuration::configuration::Configuration, domain::filter::data_base_query::DataBaseQuery};

use crate::commons::exception::api_exception::ApiException;

use super::{handler, utils};

pub struct ControllerDocument {
}

impl ControllerDocument {
    
    pub fn route(router: Router) -> Router {
        router
            .route("/:service/data-base/:data_base/collection/:collection", get(Self::find_all))
            .route_layer(middleware::from_fn(handler::autentication_handler))
    }

    async fn find_all(Path((service, data_base, collection)): Path<(String, String, String)>) -> Result<Json<Vec<String>>, impl IntoResponse> {
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
    
        Ok(Json(documents.unwrap()))
    }

}