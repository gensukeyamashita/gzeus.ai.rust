use crate::infra::dto::server_info::ServerInfo;

impl ServerInfo {
    pub fn update_health_check(&mut self, new_health_check: String) {
        self.health_check = new_health_check;
    }
}
