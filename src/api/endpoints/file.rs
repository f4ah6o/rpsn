use crate::api::types::*;
use crate::telemetry_span;
use anyhow::Result;
use reqwest::multipart;
use std::path::{Path, PathBuf};

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
    pub async fn upload_file(
        &self,
        project_id: u64,
        file_path: &Path,
    ) -> Result<ApiResponse<FilesData>> {
        let span_attrs = vec![
            ("input_path", file_path.display().to_string()),
            ("payload.kind", "binary".to_string()),
            ("op.phase", "read_input".to_string()),
        ];
        let file_bytes =
            telemetry_span::with_span_async_result("read_input_file", &span_attrs, || async {
                tokio::fs::read(file_path)
                    .await
                    .map_err(anyhow::Error::from)
            })
            .await?;
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file");
        let file_part = multipart::Part::bytes(file_bytes).file_name(file_name.to_string());
        let form = multipart::Form::new().part("file", file_part);
        self.post_multipart(&format!("project/{}/file", project_id), form)
            .await
    }

    pub async fn download_file(&self, file_hash: &str, output_path: Option<&Path>) -> Result<()> {
        let bytes = self
            .get_bytes(&format!("file/{}/download", file_hash))
            .await?;

        let path = output_path
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from(file_hash));
        let span_attrs = vec![
            ("input_path", path.display().to_string()),
            ("payload.kind", "binary".to_string()),
            ("op.phase", "write_output".to_string()),
        ];
        telemetry_span::with_span_async_result("write_output_file", &span_attrs, || async {
            tokio::fs::write(path, bytes)
                .await
                .map_err(anyhow::Error::from)
        })
        .await?;
        Ok(())
    }

    pub async fn attach_file(
        &self,
        project_id: u64,
        model: AttachModel,
        model_id: u64,
        file_id: u64,
    ) -> Result<()> {
        let model_str = model.as_str();
        self.post(
            &format!("project/{}/attach", project_id),
            &serde_json::json!({ "model": model_str, "id": model_id, "file": file_id }),
        )
        .await
    }

    pub async fn detach_file(
        &self,
        project_id: u64,
        model: AttachModel,
        model_id: u64,
        file_id: u64,
    ) -> Result<()> {
        let model_str = model.as_str();
        self.post(
            &format!("project/{}/detach", project_id),
            &serde_json::json!({ "model": model_str, "id": model_id, "file": file_id }),
        )
        .await
    }

    pub async fn delete_file(&self, file_id: u64) -> Result<()> {
        self.delete(&format!("file/{}", file_id)).await
    }
}
