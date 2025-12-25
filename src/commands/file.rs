use crate::api::{RepsonaClient, endpoints::file::AttachModel};
use crate::cli::FileCommands;
use crate::output::{print, OutputFormat, print_success};
use anyhow::Result;
use std::path::{Path, PathBuf};

pub async fn handle(client: &RepsonaClient, command: FileCommands, json: bool) -> Result<()> {
    let format = if json { OutputFormat::Json } else { OutputFormat::Human };

    match command {
        FileCommands::Upload { project_id, path } => {
            let file_path = Path::new(&path);
            let response = client.upload_file(project_id, file_path).await?;
            print(&response.data.files, format)?;
            print_success(&format!("File '{}' uploaded", path));
        }
        FileCommands::Download { hash, out } => {
            let output_path = out.map(|p| PathBuf::from(p));
            client.download_file(&hash, output_path.as_deref()).await?;
            print_success("File downloaded");
        }
        FileCommands::Attach { project_id, model, id, file } => {
            let attach_model = match model.as_str() {
                "task" => AttachModel::Task,
                "task_comment" => AttachModel::TaskComment,
                "note" => AttachModel::Note,
                "note_comment" => AttachModel::NoteComment,
                _ => return Err(anyhow::anyhow!("Invalid model: {}", model)),
            };
            client.attach_file(project_id, attach_model, id, file).await?;
            print_success("File attached");
        }
        FileCommands::Detach { project_id, model, id, file } => {
            let attach_model = match model.as_str() {
                "task" => AttachModel::Task,
                "task_comment" => AttachModel::TaskComment,
                "note" => AttachModel::Note,
                "note_comment" => AttachModel::NoteComment,
                _ => return Err(anyhow::anyhow!("Invalid model: {}", model)),
            };
            client.detach_file(project_id, attach_model, id, file).await?;
            print_success("File detached");
        }
        FileCommands::Delete { file_id } => {
            client.delete_file(file_id).await?;
            print_success("File deleted");
        }
    }

    Ok(())
}
