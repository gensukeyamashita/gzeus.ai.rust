use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateIssue {
    pub issue_id: String,
    pub fields: serde_json::Value,
}
