use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/jira/issues")
            .route(web::get().to(super::functions::fetch_issues::fetch_issues))
            .route(web::post().to(super::functions::create_issue::create_issue)),
    )
    .service(
        web::resource("/jira/issues/{issue_id}")
            .route(web::put().to(super::functions::update_issue::update_issue))
            .route(web::delete().to(super::functions::delete_issue::delete_issue)),
    )
    .service(
        web::resource("/jira/issues/{issue_id}/transitions")
            .route(web::post().to(super::functions::transition_issue::transition_issue)),
    );
}
