use crate::infra::dto::server_info::ServerInfo;

impl ServerInfo {
    pub fn add_port(&mut self, port: u16) {
        self.ports.push(port);
    }
}
