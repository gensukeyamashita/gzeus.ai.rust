use crate::settings::Settings;
use crate::utils::http_request_handler::HttpRequestHandler;
use actix_web::{web, HttpResponse, Responder};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/post_query")
            .route(web::post().to(post_query))
    );
    cfg.service(
        web::resource("/get_messages")
            .route(web::post().to(get_messages))
    );
}

pub(crate) fn common_headers(settings: &Settings) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", settings.api.token)).unwrap());
    headers.insert("Accept", HeaderValue::from_static("application/json, text/plain, */*"));
    headers.insert("Accept-Language", HeaderValue::from_static("en-US,en;q=0.9,ja;q=0.8"));
    headers.insert("Connection", HeaderValue::from_static("keep-alive"));
    headers.insert("Content-Length", HeaderValue::from_static("0"));
    headers.insert("Origin", HeaderValue::from_str(&settings.api.base_url).unwrap());
    headers.insert("Referer", HeaderValue::from_str(&settings.api.base_url).unwrap());
    headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("empty"));
    headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("cors"));
    headers.insert("Sec-Fetch-Site", HeaderValue::from_static("same-site"));
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36"));
    headers.insert("sec-ch-ua", HeaderValue::from_static("\"Not/A)Brand\";v=\"8\", \"Chromium\";v=\"126\", \"Google Chrome\";v=\"126\""));
    headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
    headers.insert("sec-ch-ua-platform", HeaderValue::from_static("\"Windows\""));
    headers
}

async fn post_query(
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
        .headers(crate::gzeus_ai::gzeus_ai::common_headers(&settings))
        .body(request_body)
        .send()
        .await;
    match response {
        Ok(resp) => {
            let body = resp.text().await.unwrap_or_else(|_| "Failed to read response body"
                .to_string());
            HttpResponse::Ok().body(body)
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(stringify!(e))
        }
    }
}

pub(crate) async fn get_messages(settings: web::Data<Arc<Mutex<Settings>>>) -> impl Responder {
    let handler = HttpRequestHandler;
    let client = Client::new();
    let settings = settings.lock().await;
    let url = format!(
        "{}{}",
        settings.api.base_url,
        settings.api.msgs_by_thread_id_endpoint,
    );

    let resp = client.post(&url)
        .headers(crate::gzeus_ai::gzeus_ai::common_headers(&settings))
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
