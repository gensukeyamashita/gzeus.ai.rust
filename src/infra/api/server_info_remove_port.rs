use crate::infra::dto::server_info::ServerInfo;

impl ServerInfo {
    pub fn remove_port(&mut self, port: u16) {
        self.ports.retain(|&p| p != port);
    }
}
