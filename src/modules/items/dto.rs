use serde::Deserialize;

use crate::common::error::AppError;

#[derive(Debug, Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
}

impl CreateItemRequest {
    pub fn validate(&self) -> Result<(), AppError> {
        if self.name.trim().is_empty() {
            return Err(AppError::bad_request("Item name is required"));
        }
        if self.name.chars().count() > 120 {
            return Err(AppError::bad_request(
                "Item name must be at most 120 characters",
            ));
        }
        if let Some(description) = &self.description {
            if description.chars().count() > 500 {
                return Err(AppError::bad_request(
                    "Item description must be at most 500 characters",
                ));
            }
        }
        if !self.price.is_finite() || self.price < 0.0 {
            return Err(AppError::bad_request(
                "Item price must be a non-negative finite number",
            ));
        }
        Ok(())
    }
}
