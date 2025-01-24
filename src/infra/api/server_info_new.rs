use crate::infra::dto::server_info::ServerInfo;

impl ServerInfo {
    pub fn new(server_name: String, hostname: String, environment: String, last_updated: String, ports: Vec<u16>, apps_running: Vec<String>, health_check: String) -> Self {
        Self {
            server_name,
            hostname,
            environment,
            last_updated,
            ports,
            apps_running,
            health_check,
        }
    }
}
