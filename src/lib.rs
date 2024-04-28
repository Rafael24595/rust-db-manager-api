pub mod commons {
    pub mod configuration {
        pub mod web_configuration;
    }
    pub mod exception {
        pub mod api_exception;
    }
}
pub mod domain {
    pub mod builder_db_connection_data;
    pub mod builder_db_service;
}
pub mod infrastructure {
    pub mod dto {
        pub mod db_service {
            pub mod dto_db_connection_data;
            pub mod dto_db_resources;
            pub mod dto_db_service_web_category;
            pub mod dto_db_service_lite;
            pub mod dto_db_service;
        }
        pub mod pagination {
            pub mod dto_paginated_collection;
            pub mod dto_query_pagination;
        }
        pub mod dto_server_status;
    }
    pub mod controller;
    pub mod db_assets;
    pub mod pagination;
    pub mod utils;
}