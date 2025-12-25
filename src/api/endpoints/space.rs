use crate::api::types::*;
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct InviteRequest {
    pub email: String,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct Invite {
    pub id: u64,
    pub email: String,
    pub role: String,
}

impl crate::api::RepsonaClient {
    pub async fn get_space(&self) -> Result<ApiResponse<Space>> {
        self.get("space").await
    }

    pub async fn invite_to_space(&self, request: &InviteRequest) -> Result<ApiResponse<Invite>> {
        self.post("space/invite", request).await
    }
}
