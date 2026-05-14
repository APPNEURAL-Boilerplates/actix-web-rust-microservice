use actix_web::{http::StatusCode, HttpResponse};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub ok: bool,
    pub data: T,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub ok: bool,
    pub error: ErrorDetails,
}

#[derive(Debug, Serialize)]
pub struct ErrorDetails {
    pub code: &'static str,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

pub fn ok<T: Serialize>(data: T) -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse { ok: true, data })
}

pub fn created<T: Serialize>(data: T) -> HttpResponse {
    HttpResponse::Created().json(ApiResponse { ok: true, data })
}

pub fn error(
    status: StatusCode,
    code: &'static str,
    message: String,
    details: Option<Value>,
) -> HttpResponse {
    HttpResponse::build(status).json(ErrorResponse {
        ok: false,
        error: ErrorDetails {
            code,
            message,
            details,
        },
    })
}
