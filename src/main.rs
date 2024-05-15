use std::net::SocketAddr;

use tower_http::cors::CorsLayer;

use axum::Router;
use rust_db_manager_api::{commons::configuration::web_configuration::WebConfiguration, infrastructure::{controller_collection::ControllerCollection, controller_database::ControllerDataBase, controller_document::ControllerDocument, controller_server::ControllerServer, controller_service::ControllerService}};

#[tokio::main]
async fn main() {
    let _ = WebConfiguration::initialize();
    
    let app = Router::new()
        .merge(ControllerServer::route(Router::new()))
        .merge(ControllerService::route(Router::new()))
        .merge(ControllerDataBase::route(Router::new()))
        .merge(ControllerCollection::route(Router::new()))
        .merge(ControllerDocument::route(Router::new()))
        .layer(CorsLayer::very_permissive())
        .into_make_service_with_connect_info::<SocketAddr>();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}