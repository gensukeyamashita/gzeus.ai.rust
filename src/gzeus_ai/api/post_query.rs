use crate::settings::Settings;
use crate::dto::common_headers;
use actix_web::{web, HttpResponse, Responder};
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn post_query(
    settings: web::Data<Arc<Mutex<Settings>>>,
    request_body: String,
) -> impl Responder {
    let client = Client::new();
    let settings = settings.lock().await;
    let url = format!(
        "{}{}?id={}",
        settings.api.base_url,
        settings.api.question_endpoint,
        settings.api.jql_chatbot_id // Assuming `id` is the `jql_chatbot_id`
    );

    let response = client.post(&url)
        .headers(common_headers(&settings))
        .body(request_body)
        .send()
        .await;

    match response {
        Ok(resp) => {
            match resp.text().await {
                Ok(body) => HttpResponse::Ok().body(body),
                Err(_) => HttpResponse::InternalServerError().body("Failed to read response body"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to make request"),
    }
}
