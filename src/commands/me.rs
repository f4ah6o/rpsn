use crate::api::{RepsonaClient, endpoints::me::*};
use crate::cli::MeCommands;
use crate::output::{print, OutputFormat};
use anyhow::Result;
use colored::Colorize;

pub async fn handle(client: &RepsonaClient, command: MeCommands, json: bool) -> Result<()> {
    let format = if json { OutputFormat::Json } else { OutputFormat::Human };

    match command {
        MeCommands::Get => {
            let response = client.get_me().await?;
            print(&response.user, format)?;
        }
        MeCommands::Update { name, full_name, what_are_you_doing } => {
            let updates = MeUpdateRequest {
                name,
                full_name,
                what_are_you_doing,
            };
            let response = client.update_me(updates).await?;
            print(&response.user, format)?;
            println!("{}", "Profile updated".green().bold());
        }
        MeCommands::Tasks => {
            let filter = TaskFilter::default();
            let response = client.get_me_tasks(&filter).await?;
            print(&response.tasks, format)?;
        }
        MeCommands::TasksResponsible => {
            let filter = TaskFilter::default();
            let response = client.get_me_tasks_responsible(&filter).await?;
            print(&response.tasks, format)?;
        }
        MeCommands::TasksBallHolding => {
            let filter = TaskFilter::default();
            let response = client.get_me_tasks_ball_holding(&filter).await?;
            print(&response.tasks, format)?;
        }
        MeCommands::TasksFollowing => {
            let filter = TaskFilter::default();
            let response = client.get_me_tasks_following(&filter).await?;
            print(&response.tasks, format)?;
        }
        MeCommands::TasksCount => {
            let response = client.get_me_tasks_count().await?;
            println!("Total tasks: {}", response.count);
        }
        MeCommands::Projects => {
            let response = client.get_me_projects().await?;
            print(&response.projects, format)?;
        }
        MeCommands::Activity => {
            let response = client.get_me_activity().await?;
            print(&response.activity, format)?;
        }
    }

    Ok(())
}
