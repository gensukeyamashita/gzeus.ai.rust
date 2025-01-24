use crate::settings::Settings;
use crate::utils::http_request_handler::HttpRequestHandler;
use crate::dto::common_headers;
use actix_web::{web, HttpResponse, Responder};
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn get_messages(settings: web::Data<Arc<Mutex<Settings>>>) -> impl Responder {
    let handler = HttpRequestHandler;
    let client = Client::new();
    let settings = settings.lock().await;
    let url = format!(
        "{}{}",
        settings.api.base_url,
        settings.api.msgs_by_thread_id_endpoint,
    );

    let resp = client.post(&url)
        .headers(common_headers(&settings))
        .send()
        .await;

    match resp {
        Ok(resp) => {
            let body = resp.text().await.unwrap_or_else(|_| "Failed to read response body".to_string());
            handler.log_output(&body);
            HttpResponse::Ok().body(body)
        }
        Err(e) => {
            handler.handle_error(&e);
            HttpResponse::InternalServerError().body("Failed to make request")
        }
    }
}
