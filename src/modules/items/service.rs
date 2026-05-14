use std::sync::Arc;

use chrono::Utc;
use uuid::Uuid;

use super::{dto::CreateItemRequest, model::Item, repository::InMemoryItemRepository};
use crate::common::error::AppError;

#[derive(Clone)]
pub struct ItemService {
    repository: Arc<InMemoryItemRepository>,
}

impl ItemService {
    pub fn new() -> Self {
        Self {
            repository: Arc::new(InMemoryItemRepository::new()),
        }
    }

    pub fn list(&self) -> Vec<Item> {
        self.repository.list()
    }

    pub fn get(&self, id: Uuid) -> Result<Item, AppError> {
        self.repository
            .get(id)
            .ok_or_else(|| AppError::not_found("Item was not found"))
    }

    pub fn create(&self, payload: CreateItemRequest) -> Result<Item, AppError> {
        payload.validate()?;
        let item = Item {
            id: Uuid::new_v4(),
            name: payload.name.trim().to_owned(),
            description: payload
                .description
                .map(|value| value.trim().to_owned())
                .filter(|value| !value.is_empty()),
            price: payload.price,
            created_at: Utc::now(),
        };
        Ok(self.repository.create(item))
    }
}

impl Default for ItemService {
    fn default() -> Self {
        Self::new()
    }
}
