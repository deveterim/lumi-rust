use hmac::{Hmac, Mac};
use sha2::Sha256;
use crate::error::{Error, Result};

type HmacSha256 = Hmac<Sha256>;

pub struct Webhooks;

impl Webhooks {
    pub fn verify_signature(payload: &[u8], signature: &str, secret: &str) -> bool {
        if signature.is_empty() || secret.is_empty() {
            return false;
        }

        let Ok(mut mac) = HmacSha256::new_from_slice(secret.as_bytes()) else {
            return false;
        };

        mac.update(payload);

        let Ok(signature_bytes) = hex::decode(signature) else {
            return false;
        };

        mac.verify_slice(&signature_bytes).is_ok()
    }

    pub fn construct_event<T: serde::de::DeserializeOwned>(
        payload: &[u8],
        signature: &str,
        secret: &str,
    ) -> Result<T> {
        if !Self::verify_signature(payload, signature, secret) {
            return Err(Error::InvalidWebhookSignature);
        }

        Ok(serde_json::from_slice(payload)?)
    }
}
