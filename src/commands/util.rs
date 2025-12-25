use crate::api::RepsonaClient;
use anyhow::Result;
use colored::Colorize;

pub async fn handle_version() {
    println!("rpsn 0.1.0");
}

pub async fn handle_ping(client: &RepsonaClient) -> Result<()> {
    println!("{}", "Pinging Repsona API...".dim());
    client.get::<serde_json::Value>("me").await?;
    println!("{}", "✓ API is reachable".green().bold());
    Ok(())
}
