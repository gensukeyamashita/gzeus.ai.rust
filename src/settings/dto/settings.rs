use serde::Deserialize;
use super::{ServerSettings, JiraSettings, RakutenAISettings};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub jira: JiraSettings,
    pub api: RakutenAISettings,
}
