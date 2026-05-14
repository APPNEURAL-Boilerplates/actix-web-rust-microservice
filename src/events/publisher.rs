use serde_json::Value;

use crate::common::error::AppError;

#[derive(Clone, Default)]
pub struct EventPublisher;

impl EventPublisher {
    pub fn new() -> Self {
        Self
    }

    pub async fn publish(&self, topic: &str, payload: Value) -> Result<(), AppError> {
        tracing::info!(topic, payload = %payload, "event publish placeholder");
        Ok(())
    }
}
