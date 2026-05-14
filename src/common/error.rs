use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde_json::Value;
use thiserror::Error;

use crate::common::response;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{message}")]
    BadRequest {
        message: String,
        details: Option<Value>,
    },
    #[error("{message}")]
    NotFound {
        message: String,
        details: Option<Value>,
    },
    #[error("{message}")]
    MethodNotAllowed {
        message: String,
        details: Option<Value>,
    },
    #[error("{message}")]
    Upstream {
        message: String,
        details: Option<Value>,
    },
    #[error("{message}")]
    Internal {
        message: String,
        details: Option<Value>,
    },
}

impl AppError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest {
            message: message.into(),
            details: None,
        }
    }
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound {
            message: message.into(),
            details: None,
        }
    }
    pub fn method_not_allowed(message: impl Into<String>) -> Self {
        Self::MethodNotAllowed {
            message: message.into(),
            details: None,
        }
    }
    pub fn upstream(message: impl Into<String>) -> Self {
        Self::Upstream {
            message: message.into(),
            details: None,
        }
    }
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
            details: None,
        }
    }
    pub fn with_details(self, details: Value) -> Self {
        match self {
            Self::BadRequest { message, .. } => Self::BadRequest {
                message,
                details: Some(details),
            },
            Self::NotFound { message, .. } => Self::NotFound {
                message,
                details: Some(details),
            },
            Self::MethodNotAllowed { message, .. } => Self::MethodNotAllowed {
                message,
                details: Some(details),
            },
            Self::Upstream { message, .. } => Self::Upstream {
                message,
                details: Some(details),
            },
            Self::Internal { message, .. } => Self::Internal {
                message,
                details: Some(details),
            },
        }
    }
    pub fn code(&self) -> &'static str {
        match self {
            Self::BadRequest { .. } => "BAD_REQUEST",
            Self::NotFound { .. } => "NOT_FOUND",
            Self::MethodNotAllowed { .. } => "METHOD_NOT_ALLOWED",
            Self::Upstream { .. } => "UPSTREAM_ERROR",
            Self::Internal { .. } => "INTERNAL_ERROR",
        }
    }
    fn public_message(&self) -> String {
        match self {
            Self::BadRequest { message, .. }
            | Self::NotFound { message, .. }
            | Self::MethodNotAllowed { message, .. }
            | Self::Upstream { message, .. } => message.clone(),
            Self::Internal { .. } => "An unexpected error occurred".to_owned(),
        }
    }
    fn details(&self) -> Option<Value> {
        match self {
            Self::BadRequest { details, .. }
            | Self::NotFound { details, .. }
            | Self::MethodNotAllowed { details, .. }
            | Self::Upstream { details, .. }
            | Self::Internal { details, .. } => details.clone(),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest { .. } => StatusCode::BAD_REQUEST,
            Self::NotFound { .. } => StatusCode::NOT_FOUND,
            Self::MethodNotAllowed { .. } => StatusCode::METHOD_NOT_ALLOWED,
            Self::Upstream { .. } => StatusCode::BAD_GATEWAY,
            Self::Internal { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse {
        response::error(
            self.status_code(),
            self.code(),
            self.public_message(),
            self.details(),
        )
    }
}

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        Self::upstream("Upstream HTTP request failed").with_details(serde_json::json!({
            "reason": error.to_string(),
        }))
    }
}
