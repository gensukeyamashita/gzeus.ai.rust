use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerInfo {
    pub server_name: String,
    pub hostname: String,
    pub environment: String, // stg, prod, dev
    pub last_updated: String,
    pub ports: Vec<u16>,
    pub apps_running: Vec<String>,
    pub health_check: String,
}
