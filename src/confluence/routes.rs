use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/post_confluence_query")
            .route(web::post().to(super::apis::post_query::post_confluence_query))
    );
    cfg.service(
        web::resource("/get_confluence_pages")
            .route(web::post().to(super::apis::get_pages::get_confluence_pages))
    );
    cfg.service(
        web::resource("/create_confluence_page")
            .route(web::post().to(super::apis::create_page::create_confluence_page))
    );
    cfg.service(
        web::resource("/update_confluence_page")
            .route(web::post().to(super::apis::update_page::update_confluence_page))
    );
    cfg.service(
        web::resource("/delete_confluence_page")
            .route(web::post().to(super::apis::delete_page::delete_confluence_page))
    );
    cfg.service(
        web::resource("/search_confluence")
            .route(web::post().to(super::apis::search::search_confluence))
    );
}
