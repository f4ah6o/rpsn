use crate::api::types::*;
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct InviteRequest {
    pub email: String,
    pub role: String,
}

impl crate::api::RepsonaClient {
    pub async fn get_space(&self) -> Result<ApiResponse<SpaceData>> {
        self.get("space/base").await
    }

    pub async fn invite_to_space(&self, request: &InviteRequest) -> Result<ApiResponse<InviteData>> {
        self.post("space/invite", request).await
    }
}
