use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use serde::{de::DeserializeOwned, Serialize};
use crate::error::{Error, Result};

#[derive(Clone)]
pub struct Client {
    http: reqwest::Client,
    base_url: String,
}

impl Client {
    pub fn new<K: Into<String>>(api_key: K) -> Result<Self> {
        let key = api_key.into();
        if key.is_empty() {
            return Err(Error::MissingApiKey);
        }

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", key))
                .map_err(|_| Error::MissingApiKey)?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(USER_AGENT, HeaderValue::from_static("lumi-rust/1.0.0"));

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        let base_url = std::env::var("LUMI_BASE_URL")
            .unwrap_or_else(|_| "https://api.lumi.ao".to_string())
            .trim_end_matches('/')
            .to_string();

        Ok(Self { http, base_url })
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, if path.starts_with('/') { path.to_string() } else { format!("/{}", path) });
        let resp = self.http.get(&url).send().await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let message = resp.text().await.unwrap_or_default();
            return Err(Error::Api { status, message });
        }

        let val: serde_json::Value = resp.json().await?;
        if let Some(data) = val.get("data") {
            if let Ok(deserialized) = serde_json::from_value::<T>(data.clone()) {
                return Ok(deserialized);
            }
        }
        Ok(serde_json::from_value(val)?)
    }

    pub async fn post<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T> {
        let url = format!("{}{}", self.base_url, if path.starts_with('/') { path.to_string() } else { format!("/{}", path) });
        let resp = self.http.post(&url).json(body).send().await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let message = resp.text().await.unwrap_or_default();
            return Err(Error::Api { status, message });
        }

        let val: serde_json::Value = resp.json().await?;
        if let Some(data) = val.get("data") {
            if let Ok(deserialized) = serde_json::from_value::<T>(data.clone()) {
                return Ok(deserialized);
            }
        }
        Ok(serde_json::from_value(val)?)
    }

    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, if path.starts_with('/') { path.to_string() } else { format!("/{}", path) });
        let resp = self.http.delete(&url).send().await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let message = resp.text().await.unwrap_or_default();
            return Err(Error::Api { status, message });
        }

        let val: serde_json::Value = resp.json().await?;
        if let Some(data) = val.get("data") {
            if let Ok(deserialized) = serde_json::from_value::<T>(data.clone()) {
                return Ok(deserialized);
            }
        }
        Ok(serde_json::from_value(val)?)
    }
}
