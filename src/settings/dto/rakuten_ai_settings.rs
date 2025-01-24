use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RakutenAISettings {
    pub base_url: String,
    pub question_endpoint: String,
    pub token: String,
    pub jql_chatbot_id: String,
    pub msgs_by_thread_id_endpoint: String,
    pub confluence_query_endpoint: String,
    pub confluence_pages_endpoint: String,
}
