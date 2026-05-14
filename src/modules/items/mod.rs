pub mod dto;
pub mod handlers;
pub mod model;
pub mod repository;
pub mod service;

use actix_web::web;

use crate::app::method_not_allowed;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/items")
            .route(web::get().to(handlers::list_items))
            .route(web::post().to(handlers::create_item))
            .route(web::route().to(method_not_allowed)),
    )
    .service(
        web::resource("/items/{id}")
            .route(web::get().to(handlers::get_item))
            .route(web::route().to(method_not_allowed)),
    );
}
