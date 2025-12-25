use crate::api::{RepsonaClient, endpoints::webhook::*};
use crate::cli::WebhookCommands;
use crate::output::{print, OutputFormat, print_success};
use anyhow::Result;

pub async fn handle(client: &RepsonaClient, command: WebhookCommands, json: bool) -> Result<()> {
    let format = if json { OutputFormat::Json } else { OutputFormat::Human };

    match command {
        WebhookCommands::List => {
            let response = client.list_webhooks().await?;
            print(&response.data.webhooks, format)?;
        }
        WebhookCommands::Create { name, url, events } => {
            let events_vec: Vec<String> = events.split(',').map(|s| s.trim().to_string()).collect();
            let request = CreateWebhookRequest { name, url, events: events_vec };
            let response = client.create_webhook(&request).await?;
            print(&response.data.webhook, format)?;
            print_success(&format!("Webhook '{}' created", response.data.webhook.name));
        }
        WebhookCommands::Update { webhook_id, name, url, events } => {
            let events_vec = events.map(|e| e.split(',').map(|s| s.trim().to_string()).collect());
            let request = UpdateWebhookRequest { name, url, events: events_vec };
            let response = client.update_webhook(webhook_id, &request).await?;
            print(&response.data.webhook, format)?;
            print_success(&format!("Webhook '{}' updated", response.data.webhook.name));
        }
        WebhookCommands::Delete { webhook_id } => {
            client.delete_webhook(webhook_id).await?;
            print_success("Webhook deleted");
        }
    }

    Ok(())
}
