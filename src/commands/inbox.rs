use crate::api::RepsonaClient;
use crate::cli::InboxCommands;
use crate::output::{print, OutputFormat, print_success};
use anyhow::Result;

pub async fn handle(client: &RepsonaClient, command: InboxCommands, json: bool) -> Result<()> {
    let format = if json { OutputFormat::Json } else { OutputFormat::Human };

    match command {
        InboxCommands::List => {
            let response = client.list_inbox().await?;
            print(&response.data.inbox, format)?;
        }
        InboxCommands::Update { inbox_id } => {
            let response = client.update_inbox(inbox_id, true).await?;
            print(&response.data.inbox, format)?;
            print_success("Inbox item marked as read");
        }
        InboxCommands::ReadAll => {
            client.mark_inbox_all_read().await?;
            print_success("All inbox items marked as read");
        }
        InboxCommands::UnreadCount => {
            let response = client.get_inbox_unread_count().await?;
            println!("Unread items: {}", response.data.count);
        }
    }

    Ok(())
}
