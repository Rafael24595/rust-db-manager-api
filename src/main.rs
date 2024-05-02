use std::net::SocketAddr;

use tower_http::cors::CorsLayer;

use axum::Router;
use rust_db_manager_api::{commons::configuration::web_configuration::WebConfiguration, infrastructure::controller::Controller};

#[tokio::main]
async fn main() {
    let _ = WebConfiguration::initialize();
    
    let router = Router::new();
    let app = Controller::route(router)
        .layer(CorsLayer::very_permissive())
        .into_make_service_with_connect_info::<SocketAddr>();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
