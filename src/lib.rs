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
        pub mod db_service {
            pub mod db {
                pub mod dto_db_create;
            }
            pub mod dto_db_connection_data;
            pub mod dto_db_resources;
            pub mod dto_db_service_web_category;
            pub mod dto_db_service_lite;
            pub mod dto_db_service_response_connection;
            pub mod dto_db_service_response;
            pub mod dto_db_service_suscribe;
            pub mod dto_db_service;
        }
        pub mod definition {
            pub mod dto_collection_definition;
            pub mod dto_field_atribute;
            pub mod dto_field_attribute_default_definition;
            pub mod dto_field_attribute_definition;
            pub mod dto_field_definition;
            pub mod dto_field_reference;
            pub mod dto_field_data;
        }
        pub mod pagination {
            pub mod dto_paginated_collection;
            pub mod dto_query_pagination;
        }
        pub mod request {
            pub mod dto_generate_collection_query;
        }
        pub mod dto_data_base_field;
        pub mod dto_data_base_group;
        pub mod dto_server_status;
    }
    pub mod controller_collection;
    pub mod controller_database;
    pub mod controller_document;
    pub mod controller_server;
    pub mod db_assets;
    pub mod handler;
    pub mod pagination;
    pub mod services_jwt;
    pub mod utils;
}