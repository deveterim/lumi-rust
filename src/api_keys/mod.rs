pub mod types;

use crate::{client::Client, error::Result};
pub use types::*;

#[derive(Clone)]
pub struct ApiKeys {
    client: Client,
}

impl ApiKeys {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, options: CreateApiKeyOptions) -> Result<ApiKey> {
        self.client.post("/api-keys", &options).await
    }

    pub async fn list(&self) -> Result<ApiKeyListResponse> {
        self.client.get("/api-keys").await
    }

    pub async fn remove(&self, api_key_id: &str) -> Result<serde_json::Value> {
        self.client.delete(&format!("/api-keys/{}", api_key_id)).await
    }
}
