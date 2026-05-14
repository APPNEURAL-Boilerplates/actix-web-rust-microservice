use actix_cors::Cors;
use actix_web::{
    error::InternalError,
    http::{header, header::HeaderName},
    web, HttpRequest, HttpResponse, ResponseError,
};
use serde_json::json;

use crate::{
    common::error::AppError,
    config::Settings,
    modules::{health, items, root},
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(root::service_info))
            .route(web::route().to(method_not_allowed)),
    )
    .service(
        web::scope("/api/v1")
            .configure(health::configure)
            .configure(items::configure),
    )
    .default_service(web::route().to(not_found));
}

pub fn json_config() -> web::JsonConfig {
    web::JsonConfig::default()
        .limit(1024 * 1024)
        .error_handler(|err, _req| {
            let app_error = AppError::bad_request("Invalid JSON body").with_details(json!({
                "reason": err.to_string(),
            }));
            InternalError::from_response(err, app_error.error_response()).into()
        })
}

pub fn build_cors(settings: &Settings) -> Cors {
    if settings.cors_allowed_origin == "*" {
        Cors::permissive()
    } else {
        Cors::default()
            .allowed_origin(&settings.cors_allowed_origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
                HeaderName::from_static("x-request-id"),
            ])
            .expose_headers(vec![HeaderName::from_static("x-request-id")])
            .max_age(3600)
    }
}

pub async fn not_found(req: HttpRequest) -> HttpResponse {
    AppError::not_found(format!("Route {} was not found", req.path())).error_response()
}

pub async fn method_not_allowed(req: HttpRequest) -> HttpResponse {
    AppError::method_not_allowed(format!(
        "Method {} is not allowed for {}",
        req.method(),
        req.path()
    ))
    .error_response()
}
