use crate::api::types::*;
use anyhow::Result;
use reqwest::multipart;
use std::path::Path;

pub enum AttachModel {
    Task,
    TaskComment,
    Note,
    NoteComment,
}

impl AttachModel {
    fn as_str(&self) -> &str {
        match self {
            AttachModel::Task => "task",
            AttachModel::TaskComment => "task_comment",
            AttachModel::Note => "note",
            AttachModel::NoteComment => "note_comment",
        }
    }
}

impl crate::api::RepsonaClient {
    pub async fn upload_file(&self, project_id: u64, file_path: &Path) -> Result<ApiResponse<FilesData>> {
        let file_bytes = tokio::fs::read(file_path).await?;
        let file_name = file_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file");
        let file_part = multipart::Part::bytes(file_bytes)
            .file_name(file_name.to_string());
        let form = multipart::Form::new().part("file", file_part);
        self.post_multipart(&format!("project/{}/file", project_id), form).await
    }

    pub async fn download_file(&self, _file_hash: &str, _output_path: Option<&Path>) -> Result<()> {
        todo!("Download file implementation")
    }

    pub async fn attach_file(&self, project_id: u64, model: AttachModel, model_id: u64, file_id: u64) -> Result<()> {
        let model_str = model.as_str();
        self.post(
            &format!("project/{}/attach", project_id),
            &serde_json::json!({ "model": model_str, "id": model_id, "file": file_id }),
        ).await
    }

    pub async fn detach_file(&self, project_id: u64, model: AttachModel, model_id: u64, file_id: u64) -> Result<()> {
        let model_str = model.as_str();
        self.post(
            &format!("project/{}/detach", project_id),
            &serde_json::json!({ "model": model_str, "id": model_id, "file": file_id }),
        ).await
    }

    pub async fn delete_file(&self, file_id: u64) -> Result<()> {
        self.delete(&format!("file/{}", file_id)).await
    }
}
