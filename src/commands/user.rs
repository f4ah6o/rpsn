use crate::api::{RepsonaClient, endpoints::user::*};
use crate::cli::UserCommands;
use crate::output::{print, OutputFormat, print_success};
use anyhow::Result;

pub async fn handle(client: &RepsonaClient, command: UserCommands, json: bool) -> Result<()> {
    let format = if json { OutputFormat::Json } else { OutputFormat::Human };

    match command {
        UserCommands::List => {
            let response = client.list_users().await?;
            print(&response.users, format)?;
        }
        UserCommands::Get { user_id } => {
            let response = client.get_user(user_id).await?;
            print(&response.user, format)?;
        }
        UserCommands::RoleSet { user_id, role } => {
            let request = SetUserRoleRequest { role };
            let response = client.set_user_role(user_id, &request).await?;
            print(&response.user, format)?;
            print_success(&format!("User {} role updated", user_id));
        }
        UserCommands::PaymentSet { user_id, r#type } => {
            let request = SetPaymentRequest { payment_type: r#type };
            let response = client.set_user_payment(user_id, &request).await?;
            print(&response.user, format)?;
            print_success(&format!("User {} payment type updated", user_id));
        }
        UserCommands::Activity { user_id } => {
            let response = client.get_user_activity(user_id).await?;
            print(&response.activity, format)?;
        }
    }

    Ok(())
}
