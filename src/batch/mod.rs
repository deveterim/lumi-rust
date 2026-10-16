pub mod types;

use crate::{client::Client, emails::CreateEmailBaseOptions, error::Result};
pub use types::*;

#[derive(Clone)]
pub struct Batch {
    client: Client,
}

impl Batch {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn send(&self, emails: Vec<CreateEmailBaseOptions>) -> Result<BatchSendResponse> {
        let mut processed_emails = Vec::with_capacity(emails.len());
        for mut email in emails {
            let to_value = if email.to.len() == 1 {
                serde_json::Value::String(email.to.remove(0))
            } else {
                serde_json::to_value(&email.to)?
            };
            let mut val = serde_json::to_value(&email)?;
            if let Some(map) = val.as_object_mut() {
                map.insert("to".to_string(), to_value);
            }
            processed_emails.push(val);
        }
        let payload = serde_json::json!({
            "emails": processed_emails
        });
        self.client.post("/emails/batch", &payload).await
    }
}
