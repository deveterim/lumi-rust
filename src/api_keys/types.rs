use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    pub prefix: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApiKeyOptions {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyListResponse {
    pub data: Vec<ApiKey>,
}
