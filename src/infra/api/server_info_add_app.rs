use crate::infra::dto::server_info::ServerInfo;

impl ServerInfo {
    pub fn add_app(&mut self, app_name: String) {
        self.apps_running.push(app_name);
    }
}
