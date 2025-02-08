use commons::configuration::web_configuration::WebConfiguration;
use dotenv::dotenv;
use infrastructure::{controller_collection::ControllerCollection, controller_database::ControllerDataBase, controller_document::ControllerDocument, controller_server::ControllerServer, controller_service::ControllerService};

use std::net::SocketAddr;

use tower_http::cors::CorsLayer;

use axum::Router;

pub mod commons {
    pub mod configuration {
        pub mod web_configuration;
    }
    pub mod exception {
        pub mod api_exception;
        pub mod auth_exception;
    }
}
pub mod domain {
    pub mod builder_db_connection_data;
    pub mod builder_db_service;
    pub mod cookie {
        pub mod builder_cookie;
        pub mod builder_jar;
        pub mod cookie;
        pub mod jar;
        pub mod same_site;
    }
}
pub mod infrastructure {
    pub mod dto {
        pub mod action {
            pub mod definition {
                pub mod dto_action_definition;
                pub mod dto_action_form;
                pub mod dto_action_form_collection;
                pub mod dto_form_default;
                pub mod dto_form_field_definition;
            }
            pub mod generate {
                pub mod dto_action_form;
                pub mod dto_action;
                pub mod dto_form_field;
            }
        }
        pub mod collection {
            pub mod dto_collection_data;
            pub mod dto_collection_definition;
            pub mod dto_collection_reference_definition;
            pub mod dto_generate_collection_query;
            pub mod dto_rename_collection_query;
        }
        pub mod data_base {
            pub mod dto_generate_data_base_query;
        }
        pub mod document {
            pub mod dto_document_data;
            pub mod dto_document_key_attribute;
            pub mod dto_document_key;
            pub mod dto_document_schema;
        }
        pub mod field {
            pub mod definition {
                pub mod dto_field_attribute_default_definition;
                pub mod dto_field_attribute_definition;
                pub mod dto_field_definition;
            }
            pub mod filter {
                pub mod definition {
                    pub mod dto_filter_attribute_default_definition;
                    pub mod dto_filter_attribute_definition;
                    pub mod dto_filter_definition_query;
                    pub mod dto_filter_definition;
                    pub mod dto_filter_field_definition;
                    pub mod dto_filter_fields_definition;
                }
                pub mod dto_filter_element;
                pub mod dto_filter_value;
                pub mod dto_filter_value_attribute;
            }
            pub mod generate {
                pub mod dto_field_atribute;
                pub mod dto_field_data;
                pub mod dto_field_reference;
            }
        }
        pub mod pagination {
            pub mod dto_paginated_collection;
            pub mod dto_query_pagination;
        }
        pub mod service {
            pub mod definition {
                pub mod dto_service_category_lite;
                pub mod dto_service_category;
                pub mod dto_service_lite;
                pub mod dto_service_resources;
                pub mod dto_service;
            }
            pub mod generate {
                pub mod dto_db_connection_data;
                pub mod dto_service_create_request;
                pub mod dto_service_suscribe_request;
            }
        }
        pub mod table {
            pub mod definition {
                pub mod dto_table_definition;
                pub mod dto_table_field_definition;
                pub mod dto_table_row_definition;
            }
            pub mod group {
                pub mod dto_table_data_field;
                pub mod dto_table_data_group;
            }
        }
        pub mod dto_create_document;
        pub mod dto_server_status;
        pub mod dto_update_document;
    }
    pub mod controller_collection;
    pub mod controller_database;
    pub mod controller_document;
    pub mod controller_server;
    pub mod controller_service;
    pub mod db_assets;
    pub mod handler;
    pub mod pagination;
    pub mod services_jwt;
    pub mod utils;
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = WebConfiguration::initialize().await;
    if let Err(err) = config {
        panic!("{}", err.to_string());
    }
    
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