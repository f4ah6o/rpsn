use crate::api::{endpoints::file::AttachModel, RepsonaClient};
use crate::cli::FileCommands;
use crate::output::{print, print_success, OutputFormat};
use crate::telemetry_span;
use anyhow::Result;
use std::path::{Path, PathBuf};

fn phase_attrs(phase: &str) -> Vec<(&'static str, String)> {
    vec![
        ("command.group", "file".to_string()),
        ("op.phase", phase.to_string()),
    ]
}

pub async fn handle(client: &RepsonaClient, command: FileCommands, json: bool) -> Result<()> {
    let format = if json {
        OutputFormat::Json
    } else {
        OutputFormat::Human
    };

    match command {
        FileCommands::Upload { project_id, path } => {
            let prepare_attrs = phase_attrs("prepare_request");
            let file_path =
                telemetry_span::with_span("prepare_request", &prepare_attrs, || Path::new(&path));
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.upload_file(project_id, file_path).await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.files, format)
            })?;
            telemetry_span::with_span("render_output", &render_attrs, || {
                print_success(&format!("File '{}' uploaded", path));
            });
        }
        FileCommands::Download { hash, out } => {
            let prepare_attrs = phase_attrs("prepare_request");
            let output_path = telemetry_span::with_span("prepare_request", &prepare_attrs, || {
                out.map(PathBuf::from)
            });
            let exec_attrs = phase_attrs("execute_operation");
            telemetry_span::with_span_async_result("execute_operation", &exec_attrs, || async {
                client.download_file(&hash, output_path.as_deref()).await
            })
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span("render_output", &render_attrs, || {
                print_success("File downloaded");
            });
        }
        FileCommands::Attach {
            project_id,
            model,
            id,
            file,
        } => {
            let validate_attrs = phase_attrs("validate_input");
            let attach_model = telemetry_span::with_span_result(
                "validate_input",
                &validate_attrs,
                || match model.as_str() {
                    "task" => Ok(AttachModel::Task),
                    "task_comment" => Ok(AttachModel::TaskComment),
                    "note" => Ok(AttachModel::Note),
                    "note_comment" => Ok(AttachModel::NoteComment),
                    _ => Err(anyhow::anyhow!("Invalid model: {}", model)),
                },
            )?;
            let exec_attrs = phase_attrs("execute_operation");
            telemetry_span::with_span_async_result("execute_operation", &exec_attrs, || async {
                client.attach_file(project_id, attach_model, id, file).await
            })
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span("render_output", &render_attrs, || {
                print_success("File attached");
            });
        }
        FileCommands::Detach {
            project_id,
            model,
            id,
            file,
        } => {
            let validate_attrs = phase_attrs("validate_input");
            let attach_model = telemetry_span::with_span_result(
                "validate_input",
                &validate_attrs,
                || match model.as_str() {
                    "task" => Ok(AttachModel::Task),
                    "task_comment" => Ok(AttachModel::TaskComment),
                    "note" => Ok(AttachModel::Note),
                    "note_comment" => Ok(AttachModel::NoteComment),
                    _ => Err(anyhow::anyhow!("Invalid model: {}", model)),
                },
            )?;
            let exec_attrs = phase_attrs("execute_operation");
            telemetry_span::with_span_async_result("execute_operation", &exec_attrs, || async {
                client.detach_file(project_id, attach_model, id, file).await
            })
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span("render_output", &render_attrs, || {
                print_success("File detached");
            });
        }
        FileCommands::Delete { file_id } => {
            let exec_attrs = phase_attrs("execute_operation");
            telemetry_span::with_span_async_result("execute_operation", &exec_attrs, || async {
                client.delete_file(file_id).await
            })
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span("render_output", &render_attrs, || {
                print_success("File deleted");
            });
        }
    }

    Ok(())
}
