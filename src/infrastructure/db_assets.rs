use rust_db_manager_core::infrastructure::repository::e_db_repository::EDBRepository;

use super::dto::db_service::{dto_db_resources::DTODBResources, dto_db_service_web_category::DTODBServiceWebCategory};

pub trait WebEDBRepository {
    
    fn supported() -> Vec<DTODBServiceWebCategory>;
    fn resources(&self) -> DTODBResources;

}

impl WebEDBRepository for EDBRepository {

    fn supported() -> Vec<DTODBServiceWebCategory> {
        EDBRepository::items().iter()
            .map(|e| DTODBServiceWebCategory::from(e.clone()))
            .collect()
    }

    fn resources(&self) -> DTODBResources {
        match self {
            EDBRepository::MongoDB => DTODBResources::new(
                String::from("https://www.mongodb.com/"),
                String::from("#00ED64"),
                String::from("https://thumbs.bfldr.com/at/hj345wvxsvpbc82vchqcj9qh?expiry=1714820885&fit=bounds&height=162&sig=NTU3MjQ1YzFjYzljYzFhY2UxNjI0ZDA4ZjhjNTc4ZDI2YzViNmMxOQ%3D%3D&width=262")
            ),
        }
    }

}