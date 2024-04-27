use rust_db_manager_core::infrastructure::repository::e_db_repository::EDBRepository;

use super::dto::dto_db_resources::DTODBResources;

pub trait WebEDBRepository {
    
    fn resources(&self) -> DTODBResources;

}

impl WebEDBRepository for EDBRepository {

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