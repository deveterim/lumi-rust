pub mod types;

use crate::{client::Client, error::Result};
pub use types::*;

#[derive(Clone)]
pub struct Domains {
    client: Client,
}

impl Domains {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, options: CreateDomainOptions) -> Result<Domain> {
        self.client.post("/domains", &options).await
    }

    pub async fn list(&self) -> Result<DomainListResponse> {
        self.client.get("/domains").await
    }

    pub async fn get(&self, domain_id: &str) -> Result<Domain> {
        self.client.get(&format!("/domains/{}", domain_id)).await
    }

    pub async fn verify(&self, domain_id: &str) -> Result<Domain> {
        self.client.post(&format!("/domains/{}/verify", domain_id), &serde_json::json!({})).await
    }

    pub async fn remove(&self, domain_id: &str) -> Result<serde_json::Value> {
        self.client.delete(&format!("/domains/{}", domain_id)).await
    }
}
