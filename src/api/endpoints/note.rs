use crate::api::types::*;
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateNoteRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "addToBottom")]
    pub add_to_bottom: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateNoteRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<u64>>,
}

impl crate::api::RepsonaClient {
    pub async fn list_notes(&self, project_id: u64) -> Result<ApiResponse<Vec<Note>>> {
        self.get(&format!("project/{}/note", project_id)).await
    }

    pub async fn get_note(&self, project_id: u64, note_id: u64) -> Result<ApiResponse<Note>> {
        self.get(&format!("project/{}/note/{}", project_id, note_id)).await
    }

    pub async fn create_note(&self, project_id: u64, request: &CreateNoteRequest) -> Result<ApiResponse<Note>> {
        self.post(&format!("project/{}/note", project_id), request).await
    }

    pub async fn update_note(&self, project_id: u64, note_id: u64, request: &UpdateNoteRequest) -> Result<ApiResponse<Note>> {
        self.patch(&format!("project/{}/note/{}", project_id, note_id), request).await
    }

    pub async fn delete_note(&self, project_id: u64, note_id: u64) -> Result<()> {
        self.delete(&format!("project/{}/note/{}", project_id, note_id)).await
    }

    pub async fn get_note_children(&self, project_id: u64, note_id: u64) -> Result<ApiResponse<Vec<Note>>> {
        self.get(&format!("project/{}/note/{}/children", project_id, note_id)).await
    }

    pub async fn list_note_comments(&self, project_id: u64, note_id: u64) -> Result<ApiResponse<Vec<NoteComment>>> {
        self.get(&format!("project/{}/note/{}/note_comment", project_id, note_id)).await
    }

    pub async fn add_note_comment(&self, project_id: u64, note_id: u64, comment: String) -> Result<ApiResponse<NoteComment>> {
        self.post(&format!("project/{}/note/{}/note_comment", project_id, note_id), &serde_json::json!({ "comment": comment })).await
    }

    pub async fn update_note_comment(&self, project_id: u64, note_id: u64, comment_id: u64, comment: String) -> Result<ApiResponse<NoteComment>> {
        self.patch(&format!("project/{}/note/{}/note_comment/{}", project_id, note_id, comment_id), &serde_json::json!({ "comment": comment })).await
    }

    pub async fn delete_note_comment(&self, project_id: u64, note_id: u64, comment_id: u64) -> Result<()> {
        self.delete(&format!("project/{}/note/{}/note_comment/{}", project_id, note_id, comment_id)).await
    }

    pub async fn get_note_activity(&self, project_id: u64, note_id: u64) -> Result<ApiResponse<Vec<Activity>>> {
        self.get(&format!("project/{}/note/{}/activity", project_id, note_id)).await
    }

    pub async fn get_note_history(&self, project_id: u64, note_id: u64) -> Result<ApiResponse<Vec<History>>> {
        self.get(&format!("project/{}/note/{}/history", project_id, note_id)).await
    }
}
