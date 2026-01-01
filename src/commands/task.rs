use crate::ai::{AiClient, AnthropicClient};
use crate::api::{endpoints::me::TaskFilter, endpoints::task::*, types::TaskStatus, RepsonaClient};
use crate::cli::TaskCommands;
use crate::commands::tag::parse_tags;
use crate::config;
use crate::output::{print, print_success, OutputFormat};
use anyhow::Result;
use colored::Colorize;
use std::io::{self, Write};

pub async fn handle(client: &RepsonaClient, command: TaskCommands, json: bool) -> Result<()> {
    let format = if json {
        OutputFormat::Json
    } else {
        OutputFormat::Human
    };

    match command {
        TaskCommands::List { project_id } => {
            let filter = TaskFilter::default();
            let response = client.list_tasks(project_id, &filter).await?;
            print(&response.data.tasks, format)?;
        }
        TaskCommands::Get {
            project_id,
            task_id,
        } => {
            let response = client.get_task(project_id, task_id).await?;
            print(&response.data.task, format)?;
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
            let tags_vec = tags.map(|t| parse_tags(&t));
            let request = CreateTaskRequest {
                name: title,
                description,
                status,
                priority,
                due_date: due,
                responsible_user: assignee,
                tags: tags_vec,
                ..Default::default()
            };
            let response = client.create_task(project_id, &request).await?;
            print(&response.data.task, format)?;
            print_success(&format!("Task '{}' created", response.data.task.name));
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
            let tags_vec = tags.map(|t| parse_tags(&t));
            let request = UpdateTaskRequest {
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
            };
            let response = client.update_task(project_id, task_id, &request).await?;
            print(&response.data.task, format)?;
            print_success(&format!("Task '{}' updated", response.data.task.name));
        }
        TaskCommands::Done {
            project_id,
            task_id,
        } => {
            let response = client
                .set_task_status(project_id, task_id, TaskStatus::Done.id())
                .await?;
            print(&response.data.task, format)?;
            print_success("Task marked as done");
        }
        TaskCommands::Reopen {
            project_id,
            task_id,
        } => {
            let response = client
                .set_task_status(project_id, task_id, TaskStatus::Open.id())
                .await?;
            print(&response.data.task, format)?;
            print_success("Task reopened");
        }
        TaskCommands::Children {
            project_id,
            task_id,
        } => {
            let response = client.get_task_children(project_id, task_id).await?;
            print(&response.data.tasks, format)?;
        }
        TaskCommands::CommentList {
            project_id,
            task_id,
        } => {
            let response = client.list_task_comments(project_id, task_id).await?;
            print(&response.data.task_comments, format)?;
        }
        TaskCommands::CommentAdd {
            project_id,
            task_id,
            comment,
            reply_to,
        } => {
            let response = client
                .add_task_comment(project_id, task_id, comment, reply_to)
                .await?;
            print(&response.data.task_comment, format)?;
            print_success("Comment added");
        }
        TaskCommands::Activity {
            project_id,
            task_id,
        } => {
            let response = client.get_task_activity(project_id, task_id).await?;
            print(&response.data.activity, format)?;
        }
        TaskCommands::History {
            project_id,
            task_id,
        } => {
            let response = client.get_task_history(project_id, task_id).await?;
            print(&response.data.history, format)?;
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
            handle_generate(
                client,
                GenerateTaskOptions {
                    project_id,
                    goal,
                    count,
                    model,
                    interactive,
                    status,
                    assignee,
                    json,
                },
            )
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
    // APIキーをロード
    let api_key = config::load_anthropic_api_key()?;

    // AIクライアントを初期化
    let ai_client = AnthropicClient::new(api_key, options.model);

    eprintln!(
        "{}",
        format!(
            "Generating {} tasks for goal: \"{}\"",
            options.count, options.goal
        )
        .dimmed()
    );

    // タスクを生成
    let tasks = ai_client
        .generate_tasks_from_goal(&options.goal, options.count)
        .await
        .map_err(|e| {
            eprintln!("{} {}", "Error generating tasks:".red(), e);
            eprintln!("\nHint: Make sure ANTHROPIC_API_KEY is set correctly:");
            eprintln!("  export ANTHROPIC_API_KEY=sk-ant-...");
            eprintln!("  Or add it to your config.toml under [ai.anthropic_api_key]");
            e
        })?;

    if tasks.is_empty() {
        eprintln!("{}", "No tasks were generated".yellow());
        return Ok(());
    }

    // プレビュー表示
    if !options.json {
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
    }

    // インタラクティブモードまたは確認プロンプト
    let tasks_to_create = if options.interactive {
        confirm_interactive(&tasks)?
    } else {
        if !options.json {
            print!(
                "Create these {} tasks in project {}? [y/N]: ",
                tasks.len(),
                options.project_id
            );
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim().to_lowercase();

            if input != "y" && input != "yes" {
                eprintln!("{}", "Cancelled".yellow());
                return Ok(());
            }
        }
        tasks
    };

    // タスクを作成
    let mut created_count = 0;
    let mut failed_count = 0;

    for task in &tasks_to_create {
        let request = CreateTaskRequest {
            name: task.title.clone(),
            description: task.description.clone(),
            status: options.status,
            priority: task.priority,
            responsible_user: options.assignee,
            ..Default::default()
        };

        match client.create_task(options.project_id, &request).await {
            Ok(response) => {
                if options.json {
                    println!("{}", serde_json::to_string_pretty(&response.data.task)?);
                } else {
                    eprintln!(
                        "{} Created: \"{}\" (ID: {})",
                        "✓".green(),
                        task.title,
                        response.data.task.id
                    );
                }
                created_count += 1;
            }
            Err(e) => {
                if !options.json {
                    eprintln!(
                        "{} {}",
                        "✗".red(),
                        format!("Failed to create \"{}\": {}", task.title, e).red()
                    );
                }
                failed_count += 1;
            }
        }
    }

    if !options.json {
        if created_count > 0 {
            print_success(&format!("Successfully created {} task(s)", created_count));
        }
        if failed_count > 0 {
            eprintln!(
                "{}",
                format!("Failed to create {} task(s)", failed_count).red()
            );
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
