use crate::api::RepsonaClient;
use crate::cli::TagCommands;
use crate::output::{print, OutputFormat};
use anyhow::Result;

pub async fn handle(client: &RepsonaClient, command: TagCommands, json: bool) -> Result<()> {
    let format = if json { OutputFormat::Json } else { OutputFormat::Human };

    match command {
        TagCommands::List => {
            let response = client.list_tags().await?;
            print(&response.data.tags, format)?;
        }
    }

    Ok(())
}
