use std::{collections::HashMap, sync::RwLock};

use uuid::Uuid;

use super::model::Item;

#[derive(Default)]
pub struct InMemoryItemRepository {
    items: RwLock<HashMap<Uuid, Item>>,
}

impl InMemoryItemRepository {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn list(&self) -> Vec<Item> {
        let items = self.items.read().expect("items read lock poisoned");
        let mut values = items.values().cloned().collect::<Vec<_>>();
        values.sort_by_key(|a| a.created_at);
        values
    }

    pub fn get(&self, id: Uuid) -> Option<Item> {
        let items = self.items.read().expect("items read lock poisoned");
        items.get(&id).cloned()
    }

    pub fn create(&self, item: Item) -> Item {
        let mut items = self.items.write().expect("items write lock poisoned");
        items.insert(item.id, item.clone());
        item
    }
}
