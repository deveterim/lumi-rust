use serde::{Deserialize, Serialize};
use crate::emails::CreateEmailResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchSendResponse {
    pub data: Vec<CreateEmailResponse>,
}
