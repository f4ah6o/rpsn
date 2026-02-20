use crate::api::{endpoints::me::*, RepsonaClient};
use crate::cli::MeCommands;
use crate::output::{print, OutputFormat};
use crate::telemetry_span;
use anyhow::Result;
use colored::Colorize;

fn phase_attrs(phase: &str) -> Vec<(&'static str, String)> {
    vec![
        ("command.group", "me".to_string()),
        ("op.phase", phase.to_string()),
    ]
}

pub async fn handle(client: &RepsonaClient, command: MeCommands, json: bool) -> Result<()> {
    let format = if json {
        OutputFormat::Json
    } else {
        OutputFormat::Human
    };

    match command {
        MeCommands::Get => {
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.get_me().await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.user, format)
            })?;
        }
        MeCommands::Update {
            name,
            full_name,
            what_are_you_doing,
        } => {
            let prepare_attrs = phase_attrs("prepare_request");
            let updates =
                telemetry_span::with_span("prepare_request", &prepare_attrs, || MeUpdateRequest {
                    name,
                    full_name,
                    what_are_you_doing,
                });
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.update_me(updates).await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.user, format)
            })?;
            telemetry_span::with_span("render_output", &render_attrs, || {
                println!("{}", "Profile updated".green().bold());
            });
        }
        MeCommands::Tasks => {
            let prepare_attrs = phase_attrs("prepare_request");
            let filter =
                telemetry_span::with_span("prepare_request", &prepare_attrs, TaskFilter::default);
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.get_me_tasks(&filter).await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.tasks, format)
            })?;
        }
        MeCommands::TasksResponsible => {
            let prepare_attrs = phase_attrs("prepare_request");
            let filter =
                telemetry_span::with_span("prepare_request", &prepare_attrs, TaskFilter::default);
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.get_me_tasks_responsible(&filter).await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.tasks, format)
            })?;
        }
        MeCommands::TasksBallHolding => {
            let prepare_attrs = phase_attrs("prepare_request");
            let filter =
                telemetry_span::with_span("prepare_request", &prepare_attrs, TaskFilter::default);
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.get_me_tasks_ball_holding(&filter).await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.tasks, format)
            })?;
        }
        MeCommands::TasksFollowing => {
            let prepare_attrs = phase_attrs("prepare_request");
            let filter =
                telemetry_span::with_span("prepare_request", &prepare_attrs, TaskFilter::default);
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.get_me_tasks_following(&filter).await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.tasks, format)
            })?;
        }
        MeCommands::TasksCount => {
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.get_me_task_count().await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            if json {
                telemetry_span::with_span_result("render_output", &render_attrs, || {
                    print(&response.data, format)
                })?;
            } else {
                telemetry_span::with_span("render_output", &render_attrs, || {
                    println!("Tasks: {}", response.data.count);
                });
            }
        }
        MeCommands::Projects => {
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.get_me_projects().await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.projects, format)
            })?;
        }
        MeCommands::Activity => {
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.get_me_activity().await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.activity, format)
            })?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_handle_me_commands_compile() {
        // This test ensures the handle function compiles correctly
        // Actual testing requires mocking RepsonaClient
    }
}
