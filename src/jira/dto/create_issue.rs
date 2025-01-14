use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateIssue {
    pub fields: serde_json::Value,
}
