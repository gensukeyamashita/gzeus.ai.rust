use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::settings::Settings;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct JqlQuery {
    jql: String,
    fields: Option<String>, // Optional parameter for fields
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/jira/issues")
            .route(web::get().to(fetch_issues)),
    );
}

async fn fetch_issues(
    settings: web::Data<Arc<Mutex<Settings>>>,
    query: web::Query<JqlQuery>,
) -> impl Responder {
    let client = Client::new();
    let settings = settings.lock().await;
    let mut request = client
        .get(format!("{}/jira/rest/api/2/search", settings.jira.base_url))
        .header("Authorization", format!("Bearer {}", settings.jira.token))
        .query(&[("jql", &query.jql)]);

    if let Some(fields) = &query.fields {
        request = request.query(&[("fields", fields)]);
    }

    let response = request.send().await;

    match response {
        Ok(resp) => HttpResponse::Ok().body(resp.text().await.unwrap_or_default()),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch issues"),
    }
}
