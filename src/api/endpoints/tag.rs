use crate::api::types::*;
use anyhow::Result;

impl crate::api::RepsonaClient {
    pub async fn list_tags(&self) -> Result<ApiResponse<TagsData>> {
        self.get("tag/all").await
    }
}
