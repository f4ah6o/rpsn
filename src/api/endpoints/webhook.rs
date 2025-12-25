use crate::api::types::*;
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CreateWebhookRequest {
    pub name: String,
    pub url: String,
    pub events: Vec<String>,
}

#[derive(Debug, Serialize, Default)]
pub struct UpdateWebhookRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
}

impl crate::api::RepsonaClient {
    pub async fn list_webhooks(&self) -> Result<ApiResponse<Vec<Webhook>>> {
        self.get("webhook").await
    }

    pub async fn create_webhook(&self, request: &CreateWebhookRequest) -> Result<ApiResponse<Webhook>> {
        self.post("webhook", request).await
    }

    pub async fn update_webhook(&self, webhook_id: u64, request: &UpdateWebhookRequest) -> Result<ApiResponse<Webhook>> {
        self.patch(&format!("webhook/{}", webhook_id), request).await
    }

    pub async fn delete_webhook(&self, webhook_id: u64) -> Result<()> {
        self.delete(&format!("webhook/{}", webhook_id)).await
    }
}
