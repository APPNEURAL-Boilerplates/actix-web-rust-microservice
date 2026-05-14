use chrono::{DateTime, Utc};

use crate::{
    clients::http_client::HttpClient, config::Settings, events::publisher::EventPublisher,
    modules::items::service::ItemService,
};

#[derive(Clone)]
pub struct AppState {
    pub settings: Settings,
    pub item_service: ItemService,
    pub http_client: HttpClient,
    pub event_publisher: EventPublisher,
    pub started_at: DateTime<Utc>,
}

impl AppState {
    pub fn new(settings: Settings) -> Self {
        let item_service = ItemService::new();
        let http_client = HttpClient::new(&settings);
        let event_publisher = EventPublisher::new();

        Self {
            settings,
            item_service,
            http_client,
            event_publisher,
            started_at: Utc::now(),
        }
    }

    pub fn uptime_seconds(&self) -> i64 {
        Utc::now()
            .signed_duration_since(self.started_at)
            .num_seconds()
            .max(0)
    }
}
