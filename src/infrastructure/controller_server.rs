use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};

use rust_db_manager_core::infrastructure::repository::e_db_repository::EDBRepository;

use crate::commons::{
    configuration::web_configuration::WebConfiguration, exception::api_exception::ApiException,
};

use super::{
    db_assets::WebEDBRepository,
    dto::{
        dto_server_status::DTOServerStatus,
        service::definition::dto_service_category_lite::DTOServiceCategoryLite,
    },
};

pub struct ControllerServer {
}

impl ControllerServer {
    
    pub fn route(router: Router) -> Router {
        router
            .route("/api/v1/metadata", get(Self::metadata))
            .route("/api/v1/available", get(Self::available))
    }

    async fn metadata() -> Result<Json<DTOServerStatus>, impl IntoResponse> {
        let config = WebConfiguration::instance().await;
        if let Err(error) = config {
            let exception = ApiException::from_configuration_exception(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
        
        let result = config.unwrap().read().await.as_dto().await;
        if let Err(error) = result {
            let exception = ApiException::from_configuration_exception(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        Ok(Json(result.unwrap()))
    }

    async fn available() -> (StatusCode, Json<Vec<DTOServiceCategoryLite>>) {
        let dto = EDBRepository::availables();
        (StatusCode::OK, Json(dto))
    }

}