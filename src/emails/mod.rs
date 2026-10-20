pub mod types;

use crate::{client::Client, error::Result};
pub use types::*;

#[derive(Clone)]
pub struct Emails {
    client: Client,
}

impl Emails {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn send(&self, mut params: CreateEmailBaseOptions) -> Result<CreateEmailResponse> {
        let to_value = if params.to.len() == 1 {
            serde_json::Value::String(params.to.remove(0))
        } else {
            serde_json::to_value(&params.to)?
        };
        let mut body = serde_json::to_value(&params)?;
        if let Some(map) = body.as_object_mut() {
            map.insert("to".to_string(), to_value);
        }
        self.client.post("/api/v1/send", &body).await
    }

    pub async fn get(&self, email_id: &str) -> Result<Email> {
        self.client.get(&format!("/api/v1/send/{}", email_id)).await
    }
}
