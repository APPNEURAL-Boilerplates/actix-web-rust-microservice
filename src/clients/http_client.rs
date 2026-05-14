use std::time::Duration;

use serde::de::DeserializeOwned;

use crate::{common::error::AppError, config::Settings};

#[derive(Clone)]
pub struct HttpClient {
    client: reqwest::Client,
    base_url: String,
}

impl HttpClient {
    pub fn new(settings: &Settings) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(settings.request_timeout_seconds))
            .build()
            .expect("failed to build reqwest client");

        Self {
            client,
            base_url: settings.external_api_base_url.clone(),
        }
    }

    pub async fn get_json<T>(&self, path: &str) -> Result<T, AppError>
    where
        T: DeserializeOwned,
    {
        if self.base_url.trim().is_empty() {
            return Err(AppError::upstream(
                "EXTERNAL_API_BASE_URL is not configured",
            ));
        }

        let url = format!(
            "{}{}",
            self.base_url.trim_end_matches('/'),
            ensure_leading_slash(path)
        );

        let response = self.client.get(url).send().await?;
        let status = response.status();

        if !status.is_success() {
            return Err(AppError::upstream("Upstream service returned an error")
                .with_details(serde_json::json!({ "status": status.as_u16() })));
        }

        Ok(response.json::<T>().await?)
    }
}

fn ensure_leading_slash(path: &str) -> String {
    if path.starts_with('/') {
        path.to_owned()
    } else {
        format!("/{path}")
    }
}
