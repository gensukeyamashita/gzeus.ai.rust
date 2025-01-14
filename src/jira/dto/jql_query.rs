use serde::Deserialize;

#[derive(Deserialize)]
pub struct JqlQuery {
    pub jql: String,
    pub fields: Option<String>, // Optional parameter for fields
}
