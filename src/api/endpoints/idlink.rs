use crate::api::types::*;
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CreateIdLinkRequest {
    pub name: String,
    pub url: String,
}

impl crate::api::RepsonaClient {
    pub async fn list_idlinks(&self) -> Result<ApiResponse<IdLinksData>> {
        self.get("idlink").await
    }

    pub async fn create_idlink(&self, request: &CreateIdLinkRequest) -> Result<ApiResponse<IdLinkData>> {
        self.post("idlink", request).await
    }

    pub async fn delete_idlink(&self, idlink_id: u64) -> Result<()> {
        self.delete(&format!("idlink/{}", idlink_id)).await
    }
}
