use std::net::SocketAddr;

use tower_http::cors::CorsLayer;

use axum::Router;
use rust_db_manager_api::infrastructure::{controller::Controller, dto::dto_query_pagination::DTOQueryPagination, pagination::Pagination};
use rust_db_manager_core::commons::configuration::configuration::Configuration;

#[tokio::main]
async fn main() {
    let _ = Configuration::initialize();

    let router = Router::new();
    let app = Controller::route(router)
        .layer(CorsLayer::permissive())
        .into_make_service_with_connect_info::<SocketAddr>();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
