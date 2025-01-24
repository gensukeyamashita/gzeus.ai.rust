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

    pub fn update_health_check(&mut self, new_health_check: String) {
        self.health_check = new_health_check;
    }

    pub fn add_app(&mut self, app_name: String) {
        self.apps_running.push(app_name);
    }

    pub fn remove_app(&mut self, app_name: &str) {
        self.apps_running.retain(|app| app != app_name);
    }

    pub fn add_port(&mut self, port: u16) {
        self.ports.push(port);
    }

    pub fn remove_port(&mut self, port: u16) {
        self.ports.retain(|&p| p != port);
    }
}
