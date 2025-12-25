use crate::api::RepsonaClient;
use crate::cli::ConfigCommands;
use crate::config::{Config, Profile};
use crate::output::{print, OutputFormat};
use anyhow::Result;
use colored::Colorize;

pub async fn handle(command: ConfigCommands) -> Result<()> {
    match command {
        ConfigCommands::Init => handle_init(),
        ConfigCommands::Get => handle_get(),
        ConfigCommands::Set { space, token } => handle_set(space, token),
        ConfigCommands::SetProfile { name, space, token } => handle_set_profile(name, space, token),
        ConfigCommands::Use { name } => handle_use(name),
        ConfigCommands::Whoami => handle_whoami().await,
    }
}

fn handle_init() -> Result<()> {
    let config = Config::default();
    config.save()?;
    println!("{}", "Configuration initialized at ~/.config/rpsn/config.toml".green().bold());
    println!("{}", "Use 'rpsn config set' to set your credentials".dimmed());
    Ok(())
}

fn handle_get() -> Result<()> {
    let config = Config::load()?;
    let current_profile = config.current_profile.clone();

    println!("{}", "Current Configuration:".bold());
    println!("  Profile: {}", current_profile);
    println!();
    println!("{}", "Profiles:".bold());

    for (name, profile) in &config.profiles {
        let is_current = name == &current_profile;
        let indicator = if is_current { "* " } else { "  " };
        let name_display = if is_current {
            name.cyan().bold().to_string()
        } else {
            name.dimmed().to_string()
        };

        println!("{}{}:", indicator, name_display);
        println!("    Space ID: {}", profile.space_id);
        println!("    Token: {}", if profile.api_token.is_empty() {
            "(not set)".dimmed().to_string()
        } else {
            format!("{}***", &profile.api_token[..8.min(profile.api_token.len())]).dimmed().to_string()
        });
    }

    Ok(())
}

fn handle_set(space_id: String, token: String) -> Result<()> {
    let mut config = Config::load()?;
    let profile = Profile { space_id, api_token: token };
    config.add_profile("default".to_string(), profile);
    config.save()?;
    println!("{}", "Credentials saved to 'default' profile".green().bold());
    Ok(())
}

fn handle_set_profile(name: String, space_id: String, token: String) -> Result<()> {
    let mut config = Config::load()?;
    let profile = Profile { space_id, api_token: token };
    config.add_profile(name.clone(), profile);
    config.save()?;
    println!("{}", format!("Credentials saved to '{}' profile", name).green().bold());
    Ok(())
}

fn handle_use(name: String) -> Result<()> {
    let mut config = Config::load()?;
    config.set_current_profile(name.clone())?;
    config.save()?;
    println!("{}", format!("Switched to profile '{}'", name).green().bold());
    Ok(())
}

async fn handle_whoami() -> Result<()> {
    let (space_id, token) = crate::config::load_credentials()?;
    let client = RepsonaClient::new(space_id, token, false, false);

    let response = client.get_me().await?;
    print(&response.data.user, OutputFormat::Human)?;

    Ok(())
}
