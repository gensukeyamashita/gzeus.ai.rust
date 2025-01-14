use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::settings::Settings;
use reqwest::Client;
use crate::jira::dto::delete_issue::DeleteIssue;

pub async fn delete_issue(
    settings: web::Data<Arc<Mutex<Settings>>>,
    path: web::Path<DeleteIssue>,
) -> impl Responder {
    let client = Client::new();
    let settings = settings.lock().await;
    let response = client
        .delete(format!("{}/jira/rest/api/2/issue/{}", settings.jira.base_url, path.issue_id))
        .header("Authorization", format!("Bearer {}", settings.jira.token))
        .send()
        .await;

    match response {
        Ok(_) => HttpResponse::Ok().body("Issue deleted successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete issue"),
    }
}
