pub mod api_keys;
pub mod batch;
pub mod client;
pub mod domains;
pub mod emails;
pub mod error;
pub mod types;
pub mod webhooks;

use api_keys::ApiKeys;
use batch::Batch;
use client::Client;
use domains::Domains;
pub use error::{Error, Result};
use emails::Emails;
pub use webhooks::Webhooks;

#[derive(Clone)]
pub struct Mailifica {
    pub emails: Emails,
    pub batch: Batch,
    pub domains: Domains,
    pub api_keys: ApiKeys,
}

impl Mailifica {
    pub fn new<K: Into<String>>(api_key: K) -> Self {
        let client = Client::new(api_key).expect("Failed to initialize Mailifica client");
        Self {
            emails: Emails::new(client.clone()),
            batch: Batch::new(client.clone()),
            domains: Domains::new(client.clone()),
            api_keys: ApiKeys::new(client),
        }
    }
}
