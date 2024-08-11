use serde::Deserialize;
use config::{Config, ConfigError, File, Environment};

#[derive(Debug, Deserialize)]
pub struct RakutenAISettings {
    pub base_url: String,
    pub question_endpoint: String,
    pub token: String,
    pub jql_chatbot_id: String,
    pub msgs_by_thread_id_endpoint: String,
}

#[derive(Debug, Deserialize)]
pub struct JiraSettings {
    pub base_url: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerSettings {
    pub threads: usize,
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub jira: JiraSettings,
    pub api: RakutenAISettings,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("settings"))
            .add_source(Environment::with_prefix("APP"))
            .build()?;
        s.try_deserialize()
    }
}
