use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use tokio::sync::Mutex;

mod settings;
mod utils;
mod jira;
mod gzeus_ai;
mod confluence;
mod routes1;
mod routes2;
mod routes3;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = settings::init::Settings::new().expect("Failed to load settings");
    let settings = Arc::new(Mutex::new(settings));

    // Clone the settings Arc before using it in the workers and bind methods
    let settings_clone = settings.clone();
    let server_threads = settings_clone.lock().await.server.threads;
    let server_address = settings_clone.lock().await.server.address.clone();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(&mut settings))
            .configure(jira::routes::init)
            .configure(gzeus_ai::routes::init)
            .configure(confluence::routes::init)
    })
    .workers(server_threads)
    .bind(server_address)?
    .run()
    .await
}
