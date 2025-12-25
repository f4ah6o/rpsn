use crate::api::{RepsonaClient, endpoints::idlink::CreateIdLinkRequest};
use crate::cli::IdlinkCommands;
use crate::output::{print, OutputFormat, print_success};
use anyhow::Result;

pub async fn handle(client: &RepsonaClient, command: IdlinkCommands, json: bool) -> Result<()> {
    let format = if json { OutputFormat::Json } else { OutputFormat::Human };

    match command {
        IdlinkCommands::List => {
            let response = client.list_idlinks().await?;
            print(&response.data.idlinks, format)?;
        }
        IdlinkCommands::Create { name, url } => {
            let request = CreateIdLinkRequest { name, url };
            let response = client.create_idlink(&request).await?;
            print(&response.data.idlink, format)?;
            print_success(&format!("ID link '{}' created", response.data.idlink.name));
        }
        IdlinkCommands::Delete { idlink_id } => {
            client.delete_idlink(idlink_id).await?;
            print_success("ID link deleted");
        }
    }

    Ok(())
}
