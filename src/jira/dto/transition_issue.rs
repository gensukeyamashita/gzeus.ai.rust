use serde::Deserialize;

#[derive(Deserialize)]
pub struct TransitionIssue {
    pub issue_id: String,
    pub transition: serde_json::Value,
}
