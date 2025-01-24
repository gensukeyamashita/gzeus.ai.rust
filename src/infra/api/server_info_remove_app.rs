use crate::infra::dto::server_info::ServerInfo;

impl ServerInfo {
    pub fn remove_app(&mut self, app_name: &str) {
        self.apps_running.retain(|app| app != app_name);
    }
}
