use crate::ai::{AiClient, AnthropicClient};
use crate::api::{endpoints::me::TaskFilter, endpoints::task::*, types::TaskStatus, RepsonaClient};
use crate::cli::TaskCommands;
use crate::commands::tag::parse_tags;
use crate::config;
use crate::output::{print, print_success, OutputFormat};
use crate::telemetry_span;
use anyhow::Result;
use colored::Colorize;
use std::io::{self, Write};

fn phase_attrs(phase: &str) -> Vec<(&'static str, String)> {
    vec![
        ("command.group", "task".to_string()),
        ("op.phase", phase.to_string()),
    ]
}

pub async fn handle(client: &RepsonaClient, command: TaskCommands, json: bool) -> Result<()> {
    let format = if json {
        OutputFormat::Json
    } else {
        OutputFormat::Human
    };

    match command {
        TaskCommands::List { project_id } => {
            let prepare_attrs = phase_attrs("prepare_request");
            let filter =
                telemetry_span::with_span("prepare_request", &prepare_attrs, TaskFilter::default);
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.list_tasks(project_id, &filter).await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.tasks, format)
            })?;
        }
        TaskCommands::Get {
            project_id,
            task_id,
        } => {
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.get_task(project_id, task_id).await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.task, format)
            })?;
        }
        TaskCommands::Create {
            project_id,
            title,
            description,
            status,
            priority,
            due,
            assignee,
            tags,
        } => {
            let validate_attrs = phase_attrs("validate_input");
            let tags_vec = telemetry_span::with_span("validate_input", &validate_attrs, || {
                tags.map(|t| parse_tags(&t))
            });
            let prepare_attrs = phase_attrs("prepare_request");
            let request = telemetry_span::with_span("prepare_request", &prepare_attrs, || {
                CreateTaskRequest {
                    name: title,
                    description,
                    status,
                    priority,
                    due_date: due,
                    responsible_user: assignee,
                    tags: tags_vec,
                    ..Default::default()
                }
            });
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.create_task(project_id, &request).await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.task, format)
            })?;
            telemetry_span::with_span("render_output", &render_attrs, || {
                print_success(&format!("Task '{}' created", response.data.task.name));
            });
        }
        TaskCommands::Update {
            project_id,
            task_id,
            title,
            description,
            status,
            priority,
            due,
            assignee,
            tags,
        } => {
            let validate_attrs = phase_attrs("validate_input");
            let tags_vec = telemetry_span::with_span("validate_input", &validate_attrs, || {
                tags.map(|t| parse_tags(&t))
            });
            let prepare_attrs = phase_attrs("prepare_request");
            let request = telemetry_span::with_span("prepare_request", &prepare_attrs, || {
                UpdateTaskRequest {
                    name: title,
                    description,
                    status,
                    priority,
                    due_date: due,
                    start_date: None,
                    responsible_user: assignee,
                    ball_holding_user: None,
                    milestone: None,
                    parent: None,
                    tags: tags_vec,
                }
            });
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.update_task(project_id, task_id, &request).await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.task, format)
            })?;
            telemetry_span::with_span("render_output", &render_attrs, || {
                print_success(&format!("Task '{}' updated", response.data.task.name));
            });
        }
        TaskCommands::Done {
            project_id,
            task_id,
        } => {
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async {
                    client
                        .set_task_status(project_id, task_id, TaskStatus::Done.id())
                        .await
                },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.task, format)
            })?;
            telemetry_span::with_span("render_output", &render_attrs, || {
                print_success("Task marked as done");
            });
        }
        TaskCommands::Reopen {
            project_id,
            task_id,
        } => {
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async {
                    client
                        .set_task_status(project_id, task_id, TaskStatus::Open.id())
                        .await
                },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.task, format)
            })?;
            telemetry_span::with_span("render_output", &render_attrs, || {
                print_success("Task reopened");
            });
        }
        TaskCommands::Delete {
            project_id,
            task_id,
        } => {
            let exec_attrs = phase_attrs("execute_operation");
            telemetry_span::with_span_async_result("execute_operation", &exec_attrs, || async {
                client.delete_task(project_id, task_id).await
            })
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span("render_output", &render_attrs, || {
                print_success("Task deleted");
            });
        }
        TaskCommands::Children {
            project_id,
            task_id,
        } => {
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.get_task_children(project_id, task_id).await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.tasks, format)
            })?;
        }
        TaskCommands::CommentList {
            project_id,
            task_id,
        } => {
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.list_task_comments(project_id, task_id).await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.task_comments, format)
            })?;
        }
        TaskCommands::CommentAdd {
            project_id,
            task_id,
            comment,
            reply_to,
        } => {
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async {
                    client
                        .add_task_comment(project_id, task_id, comment, reply_to)
                        .await
                },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.task_comment, format)
            })?;
            telemetry_span::with_span("render_output", &render_attrs, || {
                print_success("Comment added");
            });
        }
        TaskCommands::CommentUpdate {
            project_id,
            comment_id,
            comment,
        } => {
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async {
                    client
                        .update_task_comment(project_id, comment_id, comment)
                        .await
                },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.task_comment, format)
            })?;
            telemetry_span::with_span("render_output", &render_attrs, || {
                print_success("Comment updated");
            });
        }
        TaskCommands::CommentDelete {
            project_id,
            comment_id,
        } => {
            let exec_attrs = phase_attrs("execute_operation");
            telemetry_span::with_span_async_result("execute_operation", &exec_attrs, || async {
                client.delete_task_comment(project_id, comment_id).await
            })
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span("render_output", &render_attrs, || {
                print_success("Comment deleted");
            });
        }
        TaskCommands::Activity {
            project_id,
            task_id,
        } => {
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.get_task_activity(project_id, task_id).await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.activity, format)
            })?;
        }
        TaskCommands::History {
            project_id,
            task_id,
        } => {
            let exec_attrs = phase_attrs("execute_operation");
            let response = telemetry_span::with_span_async_result(
                "execute_operation",
                &exec_attrs,
                || async { client.get_task_history(project_id, task_id).await },
            )
            .await?;
            let render_attrs = phase_attrs("render_output");
            telemetry_span::with_span_result("render_output", &render_attrs, || {
                print(&response.data.history, format)
            })?;
        }
        TaskCommands::Generate {
            project_id,
            goal,
            count,
            model,
            interactive,
            status,
            assignee,
        } => {
            let prepare_attrs = phase_attrs("prepare_request");
            let options = telemetry_span::with_span("prepare_request", &prepare_attrs, || {
                GenerateTaskOptions {
                    project_id,
                    goal,
                    count,
                    model,
                    interactive,
                    status,
                    assignee,
                    json,
                }
            });
            let exec_attrs = phase_attrs("execute_operation");
            telemetry_span::with_span_async_result("execute_operation", &exec_attrs, || async {
                handle_generate(client, options).await
            })
            .await?;
        }
    }

    Ok(())
}

struct GenerateTaskOptions {
    project_id: u64,
    goal: String,
    count: usize,
    model: Option<String>,
    interactive: bool,
    status: Option<u64>,
    assignee: Option<u64>,
    json: bool,
}

async fn handle_generate(client: &RepsonaClient, options: GenerateTaskOptions) -> Result<()> {
    let prepare_attrs = phase_attrs("prepare_request");
    let api_key = telemetry_span::with_span_result(
        "prepare_request",
        &prepare_attrs,
        config::load_anthropic_api_key,
    )?;
    let ai_client = telemetry_span::with_span("prepare_request", &prepare_attrs, || {
        AnthropicClient::new(api_key, options.model)
    });

    let render_attrs = phase_attrs("render_output");
    telemetry_span::with_span("render_output", &render_attrs, || {
        eprintln!(
            "{}",
            format!(
                "Generating {} tasks for goal: \"{}\"",
                options.count, options.goal
            )
            .dimmed()
        );
    });

    let generate_attrs = vec![
        ("command.group", "task".to_string()),
        ("op.phase", "execute_operation".to_string()),
    ];
    let tasks =
        telemetry_span::with_span_async_result("generate_candidates", &generate_attrs, || async {
            ai_client
                .generate_tasks_from_goal(&options.goal, options.count)
                .await
                .map_err(|e| {
                    eprintln!("{} {}", "Error generating tasks:".red(), e);
                    eprintln!("\nHint: Make sure ANTHROPIC_API_KEY is set correctly:");
                    eprintln!("  export ANTHROPIC_API_KEY=sk-ant-...");
                    eprintln!("  Or add it to your config.toml under [ai.anthropic_api_key]");
                    e
                })
        })
        .await?;

    if tasks.is_empty() {
        telemetry_span::with_span("render_output", &render_attrs, || {
            eprintln!("{}", "No tasks were generated".yellow());
        });
        return Ok(());
    }

    // プレビュー表示
    if !options.json {
        telemetry_span::with_span("render_output", &render_attrs, || {
            eprintln!("\n{}", "Preview of tasks to be created:".bold());
            for (i, task) in tasks.iter().enumerate() {
                let priority_str = task
                    .priority
                    .map(|p| format!(" [Priority: {}]", p))
                    .unwrap_or_default();
                eprintln!("  {}. {}{}", i + 1, task.title, priority_str);
                if let Some(desc) = &task.description {
                    for line in desc.lines().take(2) {
                        eprintln!("       {}", line.dimmed());
                    }
                    if desc.lines().count() > 2 {
                        eprintln!("       ...");
                    }
                }
            }
            eprintln!();
        });
    }

    // インタラクティブモードまたは確認プロンプト
    let tasks_to_create = if options.interactive {
        let attrs = vec![
            ("command.group", "task".to_string()),
            ("op.phase", "validate_input".to_string()),
        ];
        telemetry_span::with_span_result("interactive_confirm", &attrs, || {
            confirm_interactive(&tasks)
        })?
    } else {
        if !options.json {
            let attrs = vec![
                ("command.group", "task".to_string()),
                ("op.phase", "validate_input".to_string()),
            ];
            let proceed = telemetry_span::with_span_result("interactive_confirm", &attrs, || {
                print!(
                    "Create these {} tasks in project {}? [y/N]: ",
                    tasks.len(),
                    options.project_id
                );
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let input = input.trim().to_lowercase();

                Ok::<bool, anyhow::Error>(input == "y" || input == "yes")
            })?;
            if !proceed {
                telemetry_span::with_span("render_output", &render_attrs, || {
                    eprintln!("{}", "Cancelled".yellow());
                });
                return Ok(());
            }
        }
        tasks
    };

    // タスクを作成
    let mut created_count = 0;
    let mut failed_count = 0;

    for task in &tasks_to_create {
        let prepare_create_attrs = phase_attrs("prepare_request");
        let request = telemetry_span::with_span("prepare_request", &prepare_create_attrs, || {
            CreateTaskRequest {
                name: task.title.clone(),
                description: task.description.clone(),
                status: options.status,
                priority: task.priority,
                responsible_user: options.assignee,
                ..Default::default()
            }
        });

        let create_attrs = vec![
            ("command.group", "task".to_string()),
            ("op.phase", "execute_operation".to_string()),
        ];
        match telemetry_span::with_span_async_result(
            "create_generated_task",
            &create_attrs,
            || async { client.create_task(options.project_id, &request).await },
        )
        .await
        {
            Ok(response) => {
                if options.json {
                    telemetry_span::with_span_result("render_output", &render_attrs, || {
                        println!("{}", serde_json::to_string_pretty(&response.data.task)?);
                        Ok::<(), anyhow::Error>(())
                    })?;
                } else {
                    telemetry_span::with_span("render_output", &render_attrs, || {
                        eprintln!(
                            "{} Created: \"{}\" (ID: {})",
                            "✓".green(),
                            task.title,
                            response.data.task.id
                        );
                    });
                }
                created_count += 1;
            }
            Err(e) => {
                if !options.json {
                    telemetry_span::with_span("render_output", &render_attrs, || {
                        eprintln!(
                            "{} {}",
                            "✗".red(),
                            format!("Failed to create \"{}\": {}", task.title, e).red()
                        );
                    });
                }
                failed_count += 1;
            }
        }
    }

    if !options.json {
        if created_count > 0 {
            telemetry_span::with_span("render_output", &render_attrs, || {
                print_success(&format!("Successfully created {} task(s)", created_count));
            });
        }
        if failed_count > 0 {
            telemetry_span::with_span("render_output", &render_attrs, || {
                eprintln!(
                    "{}",
                    format!("Failed to create {} task(s)", failed_count).red()
                );
            });
        }
    }

    Ok(())
}

fn confirm_interactive(
    tasks: &[crate::ai::GeneratedTask],
) -> Result<Vec<crate::ai::GeneratedTask>> {
    let mut confirmed = Vec::new();

    for (i, task) in tasks.iter().enumerate() {
        eprintln!("\n{} {}/{}", "Generated task".bold(), i + 1, tasks.len());
        eprintln!("  Title: {}", task.title.bold());
        if let Some(desc) = &task.description {
            eprintln!("  Description: {}", desc);
        }
        if let Some(p) = task.priority {
            eprintln!("  Priority: {}", p);
        }

        eprint!("  Create this task? [Y/n/s/q]: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "" | "y" | "yes" => {
                confirmed.push(task.clone());
            }
            "n" | "no" => {
                eprintln!("  {}", "Skipped".yellow());
            }
            "s" | "skip" => {
                eprintln!("  {}", "Skipped".yellow());
            }
            "q" | "quit" => {
                eprintln!("\n{}", "Cancelled".yellow());
                break;
            }
            _ => {
                eprintln!("  {}", "Unknown command, skipping".yellow());
            }
        }
    }

    Ok(confirmed)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_handle_task_commands_compile() {
        // This test ensures the handle function compiles correctly
        // Actual testing requires mocking RepsonaClient
    }
}
