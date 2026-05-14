use actix_web::{web, HttpResponse};
use serde_json::json;

use crate::{common::response, state::AppState};

pub async fn service_info(state: web::Data<AppState>) -> HttpResponse {
    response::ok(json!({
        "service": state.settings.service_name.as_str(),
        "environment": state.settings.environment.as_str(),
        "version": env!("CARGO_PKG_VERSION"),
        "api": {
            "base_path": "/api/v1",
            "health": "/api/v1/health",
            "ready": "/api/v1/ready",
            "items": "/api/v1/items"
        }
    }))
}
