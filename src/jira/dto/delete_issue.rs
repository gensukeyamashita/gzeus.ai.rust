use serde::Deserialize;

#[derive(Deserialize)]
pub struct DeleteIssue {
    pub issue_id: String,
}
