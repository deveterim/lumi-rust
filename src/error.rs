use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Mailifica API error ({status}): {message}")]
    Api {
        status: reqwest::StatusCode,
        message: String,
    },

    #[error("Missing API key")]
    MissingApiKey,

    #[error("Invalid webhook signature")]
    InvalidWebhookSignature,
}

pub type Result<T> = std::result::Result<T, Error>;
