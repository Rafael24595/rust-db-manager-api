use std::sync::Mutex;

use cargo_metadata::{CargoOpt, MetadataCommand};
use lazy_static::lazy_static;
use rust_db_manager_core::commons::{configuration::configuration::Configuration, exception::configuration_exception::ConfigurationException};

use crate::infrastructure::dto::dto_server_status::DTOServerStatus;

lazy_static! {
    static ref INSTANCE: Mutex<Option<WebConfiguration>> = Mutex::new(None);
}

#[derive(Clone)]
pub struct WebConfiguration {
    web_app_name: String,
    web_app_version: String,
}

impl WebConfiguration {

    pub const COOKIE_NAME: &'static str = "DB_TOKEN";
    
    pub fn initialize() -> WebConfiguration {
        let _ = Configuration::initialize();

        let mut instance = INSTANCE.lock().expect("Could not lock mutex");
        if instance.is_some() {
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

        let web_app_name =root.name.clone();
        let web_app_version = root.version.clone().to_string();

        let config = WebConfiguration {
            web_app_name, web_app_version
        };

        *instance = Some(config);
        
        return instance.as_ref().unwrap().clone();
    }

    fn instance() -> WebConfiguration {
        let instance = INSTANCE.lock().expect("Could not lock mutex");
        if instance.is_none() {
            //TODO: Log.
            panic!("Configuration is not initialized.");
        }
        
        return instance.as_ref().unwrap().clone();
    }

    pub fn name() -> String {
        WebConfiguration::instance().web_app_name
    }

    pub fn version() -> String {
        WebConfiguration::instance().web_app_version
    }

    pub fn as_dto() -> Result<DTOServerStatus, ConfigurationException> {
        let web = WebConfiguration::instance();
        Ok(DTOServerStatus {
            rustc_version: Configuration::rustc_version()?,
            cargo_version: Configuration::cargo_version()?,
            core_name: Configuration::name()?,
            core_version: Configuration::version()?,
            web_name: web.web_app_name,
            web_version: web.web_app_version,
            session_id: Configuration::session_id()?,
            timestamp: Configuration::timestamp()?,
            services: Configuration::find_services()?.len()
        })
    }

}