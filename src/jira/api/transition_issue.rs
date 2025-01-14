use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::settings::Settings;
use reqwest::Client;
use crate::jira::dto::transition_issue::TransitionIssue;

pub async fn transition_issue(
    settings: web::Data<Arc<Mutex<Settings>>>,
    path: web::Path<TransitionIssue>,
    transition: web::Json<TransitionIssue>,
) -> impl Responder {
    let client = Client::new();
    let settings = settings.lock().await;
    let response = client
        .post(format!("{}/jira/rest/api/2/issue/{}/transitions", settings.jira.base_url, path.issue_id))
        .header("Authorization", format!("Bearer {}", settings.jira.token))
        .json(&transition.transition)
        .send()
        .await;

    match response {
        Ok(resp) => HttpResponse::Ok().body(resp.text().await.unwrap_or_default()),
        Err(_) => HttpResponse::InternalServerError().body("Failed to transition issue"),
    }
}
