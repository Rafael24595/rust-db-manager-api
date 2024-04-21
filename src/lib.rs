pub mod commons {
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
            pub mod dto_db_service;
        }
    }
    pub mod controller;
    pub mod utils;
}