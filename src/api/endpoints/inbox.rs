use crate::api::types::*;
use anyhow::Result;

impl crate::api::RepsonaClient {
    pub async fn list_inbox(&self) -> Result<ApiResponse<InboxData>> {
        self.get("inbox/unread").await
    }

    pub async fn update_inbox(
        &self,
        inbox_id: u64,
        status: &str,
    ) -> Result<ApiResponse<InboxItemData>> {
        self.patch(
            &format!("inbox/{}", inbox_id),
            &serde_json::json!({ "status": status }),
        )
        .await
    }

    pub async fn mark_inbox_all_read(&self) -> Result<()> {
        self.post("inbox/archive_all", &serde_json::json!({})).await
    }

    pub async fn get_inbox_unread_count(&self) -> Result<ApiResponse<UnreadCountData>> {
        self.get("inbox/unread_count").await
    }
}
