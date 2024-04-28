use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct DTOServerStatus {
    pub rustc_version: String,
    pub cargo_version: String,
    pub core_name: String,
    pub core_version: String,
    pub web_name: String,
    pub web_version: String,
    pub session_id: String,
    pub timestamp: u128,
    pub services: usize
}