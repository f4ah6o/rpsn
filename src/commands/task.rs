use crate::api::{RepsonaClient, endpoints::task::*, endpoints::me::TaskFilter};
use crate::cli::TaskCommands;
use crate::output::{print, OutputFormat, print_success};
use anyhow::Result;

pub async fn handle(client: &RepsonaClient, command: TaskCommands, json: bool) -> Result<()> {
    let format = if json { OutputFormat::Json } else { OutputFormat::Human };

    match command {
        TaskCommands::List { project_id } => {
            let filter = TaskFilter::default();
            let response = client.list_tasks(project_id, &filter).await?;
            print(&response.data.tasks, format)?;
        }
        TaskCommands::Get { project_id, task_id } => {
            let response = client.get_task(project_id, task_id).await?;
            print(&response.data.task, format)?;
        }
        TaskCommands::Create { project_id, title, description, status, priority, due, assignee, tags } => {
            let tags_vec = tags.map(|t| t.split(',').filter_map(|s| s.trim().parse().ok()).collect());
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
        TaskCommands::Update { project_id, task_id, title, description, status, priority, due, assignee, tags } => {
            let tags_vec = tags.map(|t| t.split(',').filter_map(|s| s.trim().parse().ok()).collect());
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
        TaskCommands::Done { project_id, task_id } => {
            let response = client.set_task_status(project_id, task_id, 0).await?;
            print(&response.data.task, format)?;
            print_success("Task marked as done");
        }
        TaskCommands::Reopen { project_id, task_id } => {
            let response = client.set_task_status(project_id, task_id, 1).await?;
            print(&response.data.task, format)?;
            print_success("Task reopened");
        }
        TaskCommands::Children { project_id, task_id } => {
            let response = client.get_task_children(project_id, task_id).await?;
            print(&response.data.tasks, format)?;
        }
        TaskCommands::CommentList { project_id, task_id } => {
            let response = client.list_task_comments(project_id, task_id).await?;
            print(&response.data.task_comments, format)?;
        }
        TaskCommands::CommentAdd { project_id, task_id, comment, reply_to } => {
            let response = client.add_task_comment(project_id, task_id, comment, reply_to).await?;
            print(&response.data.task_comment, format)?;
            print_success("Comment added");
        }
        TaskCommands::Activity { project_id, task_id } => {
            let response = client.get_task_activity(project_id, task_id).await?;
            print(&response.data.activity, format)?;
        }
        TaskCommands::History { project_id, task_id } => {
            let response = client.get_task_history(project_id, task_id).await?;
            print(&response.data.history, format)?;
        }
    }

    Ok(())
}
