use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde_json::json;

use crate::{app::method_not_allowed, common::response, state::AppState};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/health")
            .route(web::get().to(health))
            .route(web::route().to(method_not_allowed)),
    )
    .service(
        web::resource("/ready")
            .route(web::get().to(ready))
            .route(web::route().to(method_not_allowed)),
    );
}

pub async fn health(state: web::Data<AppState>) -> HttpResponse {
    response::ok(json!({
        "status": "healthy",
        "service": state.settings.service_name.as_str(),
        "environment": state.settings.environment.as_str(),
        "uptime_seconds": state.uptime_seconds(),
        "timestamp": Utc::now(),
    }))
}

pub async fn ready(state: web::Data<AppState>) -> HttpResponse {
    response::ok(json!({
        "status": "ready",
        "service": state.settings.service_name.as_str(),
        "checks": {
            "http_client": "configured",
            "event_publisher": "configured",
            "repository": "in-memory"
        },
        "timestamp": Utc::now(),
    }))
}
