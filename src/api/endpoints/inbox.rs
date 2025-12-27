use crate::api::types::*;
use anyhow::Result;

impl crate::api::RepsonaClient {
    pub async fn list_inbox(&self) -> Result<ApiResponse<InboxData>> {
        self.get("inbox").await
    }

    pub async fn update_inbox(&self, inbox_id: u64, read: bool) -> Result<ApiResponse<InboxItemData>> {
        self.patch(&format!("inbox/{}", inbox_id), &serde_json::json!({ "read": read })).await
    }

    pub async fn mark_inbox_all_read(&self) -> Result<()> {
        self.patch("inbox/readAll", &serde_json::json!({})).await
    }

    pub async fn get_inbox_unread_count(&self) -> Result<ApiResponse<UnreadCountData>> {
        self.get("inbox/unread_count").await
    }
}
