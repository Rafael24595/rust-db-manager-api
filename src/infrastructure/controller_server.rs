use axum::{
    http::StatusCode, response::IntoResponse, routing::get, Json, Router
};

use rust_db_manager_core::{domain::filter::e_filter_category::EFilterCategory, infrastructure::repository::e_db_repository::EDBRepository};

use crate::commons::{configuration::web_configuration::WebConfiguration, exception::api_exception::ApiException};

use super::{
    db_assets::WebEDBRepository,
    dto::{dto_server_status::DTOServerStatus, field::filter::dto_filter_resources::DTOFilterResources, service::definition::dto_service_category_lite::DTOServiceCategoryLite}
};

pub struct ControllerServer {
}

impl ControllerServer {
    
    pub fn route(router: Router) -> Router {
        router
            .route("/api/v1/metadata", get(Self::metadata))
            .route("/api/v1/available", get(Self::available))
            .route("/api/v1/resources/filter", get(Self::resources_filter))
    }

    async fn metadata() -> Result<Json<DTOServerStatus>, impl IntoResponse> {
        let result = WebConfiguration::as_dto();
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

    async fn resources_filter() -> (StatusCode, Json<DTOFilterResources>) {
        let dto = DTOFilterResources::new(
            EFilterCategory::root_category().to_string(),
            EFilterCategory::query_category().to_string(), 
            EFilterCategory::items().iter().map(|c| c.to_string()).collect()
        );
        (StatusCode::OK, Json(dto))
    }

}