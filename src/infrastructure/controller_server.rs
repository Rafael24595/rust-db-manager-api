use axum::{
    http::StatusCode,
    routing::get,
    Json, Router,
};

use rust_db_manager_core::infrastructure::repository::e_db_repository::EDBRepository;

use crate::commons::configuration::web_configuration::WebConfiguration;

use super::{
    db_assets::WebEDBRepository,
    dto::{dto_server_status::DTOServerStatus, service::definition::dto_service_category_lite::DTOServiceCategoryLite}
};

pub struct ControllerServer {
}

impl ControllerServer {
    
    pub fn route(router: Router) -> Router {
        router
            .route("/api/v1/metadata", get(Self::metadata))
            .route("/api/v1/available", get(Self::available))
    }

    async fn metadata() -> (StatusCode, Json<DTOServerStatus>) {
        let result = WebConfiguration::as_dto();
        (StatusCode::OK, Json(result))
    }

    async fn available() -> (StatusCode, Json<Vec<DTOServiceCategoryLite>>) {
        let dto = EDBRepository::availables();
        (StatusCode::OK, Json(dto))
    }

}