use crate::api::{endpoints::me::*, RepsonaClient};
use crate::cli::MeCommands;
use crate::output::{print, OutputFormat};
use anyhow::Result;
use colored::Colorize;

pub async fn handle(client: &RepsonaClient, command: MeCommands, json: bool) -> Result<()> {
    let format = if json {
        OutputFormat::Json
    } else {
        OutputFormat::Human
    };

    match command {
        MeCommands::Get => {
            let response = client.get_me().await?;
            print(&response.data.user, format)?;
        }
        MeCommands::Update {
            name,
            full_name,
            what_are_you_doing,
        } => {
            let updates = MeUpdateRequest {
                name,
                full_name,
                what_are_you_doing,
            };
            let response = client.update_me(updates).await?;
            print(&response.data.user, format)?;
            println!("{}", "Profile updated".green().bold());
        }
        MeCommands::Tasks => {
            let filter = TaskFilter::default();
            let response = client.get_me_tasks(&filter).await?;
            print(&response.data.tasks, format)?;
        }
        MeCommands::TasksResponsible => {
            let filter = TaskFilter::default();
            let response = client.get_me_tasks_responsible(&filter).await?;
            print(&response.data.tasks, format)?;
        }
        MeCommands::TasksBallHolding => {
            let filter = TaskFilter::default();
            let response = client.get_me_tasks_ball_holding(&filter).await?;
            print(&response.data.tasks, format)?;
        }
        MeCommands::TasksFollowing => {
            let filter = TaskFilter::default();
            let response = client.get_me_tasks_following(&filter).await?;
            print(&response.data.tasks, format)?;
        }
        MeCommands::TasksCount => {
            let response = client.get_me_task_count().await?;
            if json {
                print(&response.data, format)?;
            } else {
                println!("Tasks: {}", response.data.count);
            }
        }
        MeCommands::Projects => {
            let response = client.get_me_projects().await?;
            print(&response.data.projects, format)?;
        }
        MeCommands::Activity => {
            let response = client.get_me_activity().await?;
            print(&response.data.activity, format)?;
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
