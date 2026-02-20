use crate::api::{endpoints::project::*, RepsonaClient};
use crate::cli::ProjectCommands;
use crate::output::{print, print_success, OutputFormat};
use crate::telemetry_span;
use anyhow::Result;

fn phase_attrs(phase: &str) -> Vec<(&'static str, String)> {
    vec![
        ("command.group", "project".to_string()),
        ("op.phase", phase.to_string()),
    ]
}

pub async fn handle(client: &RepsonaClient, command: ProjectCommands, json: bool) -> Result<()> {
    let format = if json {
        OutputFormat::Json
    } else {
        OutputFormat::Human
    };

    match command {
        ProjectCommands::List => {
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.list_projects().await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.projects, format)
            })?;
        }
        ProjectCommands::Get { project_id } => {
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.get_project(project_id).await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.project, format)
            })?;
        }
        ProjectCommands::Create {
            name,
            full_name,
            purpose,
        } => {
            let prepare_attrs = phase_attrs("prepare_request");
            let request = telemetry_span::with_span("prepare_request", &prepare_attrs, || {
                CreateProjectRequest {
                    name,
                    full_name,
                    purpose,
                }
            });
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.create_project(&request).await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.project, format)
            })?;
            telemetry_span::with_span("render_output", &render_attrs, || {
                print_success(&format!("Project '{}' created", response.data.project.name));
            });
        }
        ProjectCommands::Update {
            project_id,
            name,
            purpose,
        } => {
            let prepare_attrs = phase_attrs("prepare_request");
            let request = telemetry_span::with_span("prepare_request", &prepare_attrs, || {
                UpdateProjectRequest {
                    name,
                    full_name: None,
                    purpose,
                }
            });
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.update_project(project_id, &request).await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.project, format)
            })?;
            telemetry_span::with_span("render_output", &render_attrs, || {
                print_success(&format!("Project '{}' updated", response.data.project.name));
            });
        }
        ProjectCommands::Delete { project_id } => {
            let exec_attrs = phase_attrs("execute_operation");
            telemetry_span::with_span_async_result("execute_operation", &exec_attrs, || async {
                client.delete_project(project_id).await
            })
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span("render_output", &render_attrs, || {
                print_success(&format!("Project {} deleted", project_id));
            });
        }
        ProjectCommands::MembersList { project_id } => {
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.list_project_members(project_id).await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.users, format)
            })?;
        }
        ProjectCommands::MembersAdd { project_id, user } => {
            let exec_attrs = phase_attrs("execute_operation");
            let _response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.add_project_member(project_id, user).await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span("render_output", &render_attrs, || {
                print_success(&format!("User {} added to project", user));
            });
        }
        ProjectCommands::MembersRemove { project_id, user } => {
            let exec_attrs = phase_attrs("execute_operation");
            telemetry_span::with_span_async_result("execute_operation", &exec_attrs, || async {
                client.remove_project_member(project_id, user).await
            })
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span("render_output", &render_attrs, || {
                print_success(&format!("User {} removed from project", user));
            });
        }
        ProjectCommands::Activity { project_id } => {
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.get_project_activity(project_id).await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.activity, format)
            })?;
        }
        ProjectCommands::StatusList { project_id } => {
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.list_project_statuses(project_id).await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.statuses, format)
            })?;
        }
        ProjectCommands::MilestoneList { project_id } => {
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.list_project_milestones(project_id).await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.milestones, format)
            })?;
        }
    }

    Ok(())
}
