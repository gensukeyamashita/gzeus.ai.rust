use actix_web::web;
use crate::gzeus_ai::api::{post_query, get_messages};

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
