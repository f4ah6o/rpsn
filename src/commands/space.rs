use crate::api::{endpoints::space::InviteRequest, RepsonaClient};
use crate::cli::SpaceCommands;
use crate::output::{print, print_success, OutputFormat};
use anyhow::Result;

pub async fn handle(client: &RepsonaClient, command: SpaceCommands, json: bool) -> Result<()> {
    let format = if json {
        OutputFormat::Json
    } else {
        OutputFormat::Human
    };

    match command {
        SpaceCommands::Get => {
            let response = client.get_space().await?;
            print(&response.data.space, format)?;
        }
        SpaceCommands::Invite { email, role } => {
            let role = role.unwrap_or_else(|| "member".to_string());
            let request = InviteRequest {
                email: email.clone(),
                role,
            };
            client.invite_to_space(&request).await?;
            print_success(&format!("Invitation sent to {}", email));
        }
    }

    Ok(())
}
