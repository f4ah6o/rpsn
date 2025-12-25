use crate::api::{RepsonaClient, endpoints::project::*};
use crate::cli::ProjectCommands;
use crate::output::{print, OutputFormat, print_success};
use anyhow::Result;
use colored::Colorize;

pub async fn handle(client: &RepsonaClient, command: ProjectCommands, json: bool) -> Result<()> {
    let format = if json { OutputFormat::Json } else { OutputFormat::Human };

    match command {
        ProjectCommands::List => {
            let response = client.list_projects().await?;
            print(&response.projects, format)?;
        }
        ProjectCommands::Get { project_id } => {
            let response = client.get_project(project_id).await?;
            print(&response.project, format)?;
        }
        ProjectCommands::Create { name, full_name, purpose } => {
            let request = CreateProjectRequest { name, full_name, purpose };
            let response = client.create_project(&request).await?;
            print(&response.project, format)?;
            print_success(&format!("Project '{}' created", response.project.name));
        }
        ProjectCommands::Update { project_id, name, purpose } => {
            let request = UpdateProjectRequest { name, full_name: None, purpose };
            let response = client.update_project(project_id, &request).await?;
            print(&response.project, format)?;
            print_success(&format!("Project '{}' updated", response.project.name));
        }
        ProjectCommands::MembersList { project_id } => {
            let response = client.list_project_members(project_id).await?;
            print(&response.users, format)?;
        }
        ProjectCommands::MembersAdd { project_id, user } => {
            let response = client.add_project_member(project_id, user).await?;
            print_success(&format!("User {} added to project", user));
        }
        ProjectCommands::MembersRemove { project_id, user } => {
            client.remove_project_member(project_id, user).await?;
            print_success(&format!("User {} removed from project", user));
        }
        ProjectCommands::Activity { project_id } => {
            let response = client.get_project_activity(project_id).await?;
            print(&response.activity, format)?;
        }
        ProjectCommands::StatusList { project_id } => {
            let response = client.list_project_statuses(project_id).await?;
            print(&response.statuses, format)?;
        }
        ProjectCommands::MilestoneList { project_id } => {
            let response = client.list_project_milestones(project_id).await?;
            print(&response.milestones, format)?;
        }
    }

    Ok(())
}
