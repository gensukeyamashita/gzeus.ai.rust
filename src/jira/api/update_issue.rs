use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::settings::Settings;
use reqwest::Client;
use crate::jira::dto::update_issue::UpdateIssue;

pub async fn update_issue(
    settings: web::Data<Arc<Mutex<Settings>>>,
    path: web::Path<UpdateIssue>,
    issue: web::Json<UpdateIssue>,
) -> impl Responder {
    let client = Client::new();
    let settings = settings.lock().await;
    let response = client
        .put(format!("{}/jira/rest/api/2/issue/{}", settings.jira.base_url, path.issue_id))
        .header("Authorization", format!("Bearer {}", settings.jira.token))
        .json(&issue.fields)
        .send()
        .await;

    match response {
        Ok(resp) => HttpResponse::Ok().body(resp.text().await.unwrap_or_default()),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update issue"),
    }
}
