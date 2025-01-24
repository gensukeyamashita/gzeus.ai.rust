use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JiraSettings {
    pub base_url: String,
    pub token: String,
}
