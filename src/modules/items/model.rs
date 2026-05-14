use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub created_at: DateTime<Utc>,
}
