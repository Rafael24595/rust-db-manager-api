use std::sync::Arc;

use cargo_metadata::{CargoOpt, MetadataCommand};
use lazy_static::lazy_static;
use rust_db_manager_core::commons::{configuration::configuration::Configuration, exception::configuration_exception::ConfigurationException};
use tokio::sync::RwLock;

use crate::infrastructure::dto::dto_server_status::DTOServerStatus;

lazy_static! {
    static ref INSTANCE: Arc<RwLock<WebConfiguration>> = Arc::new(RwLock::new(WebConfiguration::empty()));
}

#[derive(Clone)]
pub struct WebConfiguration {
    web_app_name: String,
    web_app_version: String,
}

impl WebConfiguration {

    pub const COOKIE_NAME: &'static str = "DB_TOKEN";

    fn empty() -> WebConfiguration {
        WebConfiguration { 
            web_app_name: String::new(), 
            web_app_version: String::new()
        }
    }
    
    pub async fn initialize() -> Result<Arc<RwLock<WebConfiguration>>, ConfigurationException> {
        let _ = Configuration::initialize().await?;

        let mut instance = INSTANCE.write().await;
        if instance.is_initialized() {
            //TODO: Log.
            panic!("Configuration is already initialized.");
        }

        let metadata = MetadataCommand::new()
            .manifest_path("./Cargo.toml")
            .features(CargoOpt::AllFeatures)
            .exec()
            .unwrap();

        let root: &cargo_metadata::Package = metadata.packages.iter()
            .find(|i| i.name == "rust_db_manager_api").unwrap();

        let web_app_name = root.name.clone();
        let web_app_version = root.version.clone().to_string();

        *instance = WebConfiguration {
            web_app_name, web_app_version
        };
        
        Ok(Arc::clone(&INSTANCE))
    }

    pub async fn instance() -> Result<Arc<RwLock<WebConfiguration>>, ConfigurationException> {
        let instance = Arc::clone(&INSTANCE);
        if instance.read().await.is_not_initialized() {
            //TODO: Log.
            return Err(ConfigurationException::new("Web configuration is not initialized."))
        }

        Ok(Arc::clone(&INSTANCE))
    }

    fn is_initialized(&self) -> bool {
        !self.web_app_name.is_empty() && !self.web_app_name.is_empty()
    }

    fn is_not_initialized(&self) -> bool {
        !self.is_initialized()
    }

    pub fn name(&self) -> &str {
        &self.web_app_name
    }

    pub fn version(&self) -> &str {
        &self.web_app_version
    }

    pub async fn as_dto(&self) -> Result<DTOServerStatus, ConfigurationException> {
        let core =  Configuration::instance().await?;
        let locked_core = core.read().await;
        
        Ok(DTOServerStatus {
            rustc_version: locked_core.rustc_version().to_string(),
            cargo_version: locked_core.cargo_version().to_string(),
            core_name: locked_core.name().to_string(),
            core_version: locked_core.version().to_string(),
            web_name: self.web_app_name.clone(),
            web_version: self.web_app_version.clone(),
            session_id: locked_core.session_id().to_string(),
            timestamp: locked_core.timestamp(),
            services: locked_core.find_services().await.len()
        })
    }

}