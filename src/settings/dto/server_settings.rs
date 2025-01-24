use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerSettings {
    pub threads: usize,
    pub address: String,
}
