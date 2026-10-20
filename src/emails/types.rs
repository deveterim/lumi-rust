use serde::{Deserialize, Serialize};
use crate::types::{Attachment, Tag};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEmailBaseOptions {
    pub from: String,
    pub to: Vec<String>,
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
}

impl CreateEmailBaseOptions {
    pub fn new<F, T, S>(from: F, to: Vec<T>, subject: S) -> Self
    where
        F: Into<String>,
        T: Into<String>,
        S: Into<String>,
    {
        Self {
            from: from.into(),
            to: to.into_iter().map(Into::into).collect(),
            subject: subject.into(),
            html: None,
            text: None,
            cc: None,
            bcc: None,
            reply_to: None,
            attachments: None,
            tags: None,
            scheduled_at: None,
        }
    }

    pub fn with_html<H: Into<String>>(mut self, html: H) -> Self {
        self.html = Some(html.into());
        self
    }

    pub fn with_text<T: Into<String>>(mut self, text: T) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn with_reply_to<R: Into<String>>(mut self, reply_to: R) -> Self {
        self.reply_to = Some(reply_to.into());
        self
    }

    pub fn with_cc(mut self, cc: Vec<String>) -> Self {
        self.cc = Some(cc);
        self
    }

    pub fn with_bcc(mut self, bcc: Vec<String>) -> Self {
        self.bcc = Some(bcc);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEmailResponse {
    pub id: String,
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    pub id: String,
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub to: serde_json::Value,
    #[serde(default)]
    pub subject: String,
    #[serde(default)]
    pub status: String,
    #[serde(default, alias = "createdAt")]
    pub created_at: String,
}
