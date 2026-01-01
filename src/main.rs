mod ai;
mod api;
mod cli;
mod commands;
mod config;
mod error_report;
mod output;

use anyhow::Result;
use clap::{CommandFactory, FromArgMatches};
use clap_complete::{generate, Shell};
use std::fs;
use std::path::PathBuf;

use colored::Colorize;

use api::RepsonaClient;
use cli::{Cli, Commands, Shell as ClapShell, UtilCommands};
use commands::{
    config as config_cmd, file, idlink, inbox, me, note, project, report, space, tag, task, user,
    util, webhook,
};

fn generate_shell_completion(shell: ClapShell) {
    let mut cmd = Cli::command();
    let mut buf = vec![];
    match shell {
        ClapShell::Bash => generate(Shell::Bash, &mut cmd, "rpsn", &mut buf),
        ClapShell::Zsh => generate(Shell::Zsh, &mut cmd, "rpsn", &mut buf),
        ClapShell::Fish => generate(Shell::Fish, &mut cmd, "rpsn", &mut buf),
        ClapShell::Elvish => generate(Shell::Elvish, &mut cmd, "rpsn", &mut buf),
        ClapShell::Powershell => generate(Shell::PowerShell, &mut cmd, "rpsn", &mut buf),
    };
    print!("{}", String::from_utf8(buf).unwrap());
}

fn generate_skill_file(output: Option<String>) -> Result<()> {
    let cmd = Cli::command();

    let mut skill_content = String::new();
    skill_content.push_str("---\n");
    skill_content.push_str("name: rpsn\n");
    skill_content.push_str("description: Interact with Repsona task management via rpsn CLI\n");
    skill_content.push_str("---\n\n");
    skill_content.push_str("# rpsn Agent Skill\n\n");
    skill_content.push_str(
        "This skill provides access to rpsn CLI commands for Repsona task management.\n\n",
    );
    skill_content.push_str("## Categories\n\n");

    let subcommands = cmd.get_subcommands();

    for subcmd in subcommands {
        let name = subcmd.get_name();

        if name == "util" || name == "completion" || name == "skill-generate" {
            continue;
        }

        let description = subcmd
            .get_about()
            .map(|s| s.to_string())
            .unwrap_or_default();

        skill_content.push_str(&format!("### {} - {}\n", name, description));
        skill_content.push_str("```bash\n");

        for sub in subcmd.get_subcommands() {
            let sub_name = sub.get_name();
            let sub_desc = sub
                .get_about()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "".to_string());
            skill_content.push_str(&format!("rpsn {} {} - {}\n", name, sub_name, sub_desc));
        }

        skill_content.push_str("```\n\n");
    }

    skill_content.push_str("## Global Options\n\n");
    skill_content.push_str("- `--space <space_id>` - Override Repsona Space ID\n");
    skill_content.push_str("- `--token <api_key>` - Override API Token\n");
    skill_content.push_str("- `--profile <name>` - Use specific config profile\n");
    skill_content.push_str("- `--json` - Output as JSON\n");
    skill_content.push_str("- `--dry-run` - Show request only, don't execute\n");
    skill_content.push_str("- `--yes` - Skip confirmation prompts\n");
    skill_content.push_str("- `--trace` - Show HTTP trace for debugging\n\n");

    skill_content.push_str("## Configuration\n\n");
    skill_content.push_str("```bash\n");
    skill_content.push_str("# Initialize configuration\n");
    skill_content.push_str("rpsn config init\n\n");
    skill_content.push_str("# Set credentials\n");
    skill_content.push_str("rpsn config set --space <space_id> --token <api_key>\n\n");
    skill_content.push_str("# Verify configuration\n");
    skill_content.push_str("rpsn config whoami\n");
    skill_content.push_str("```\n\n");

    let output_path = if let Some(path) = output {
        PathBuf::from(path)
    } else {
        let mut skill_dir = dirs::config_dir().unwrap();
        skill_dir.push("rpsn");
        skill_dir.push(".claude");
        skill_dir.push("skills");
        skill_dir.push("rpsn");
        skill_dir.push("SKILL.md");
        skill_dir
    };

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&output_path, skill_content)?;
    println!(
        "{}",
        format!("Skill file generated at: {}", output_path.display())
            .green()
            .bold()
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut cmd = Cli::command();

    if std::env::var_os("REPSONA_TOKEN").is_some() {
        cmd = cmd.mut_arg("token", |arg| {
            arg.help("API Token (overrides config) [REPSONA_TOKEN is set]")
                .hide_env_values(true)
        });
    } else {
        cmd = cmd.mut_arg("token", |arg| {
            arg.help("API Token (overrides config)")
                .hide_env_values(true)
        });
    }

    let cli = Cli::from_arg_matches(&cmd.get_matches())?;

    match cli.command {
        Commands::Completion { shell } => {
            generate_shell_completion(shell);
            return Ok(());
        }
        Commands::SkillGenerate { output } => {
            generate_skill_file(output)?;
            return Ok(());
        }
        Commands::Report(cmd) => {
            // Report commands don't require credentials
            report::handle(cmd).await?;
            return Ok(());
        }
        _ => {}
    }

    let (space_id, api_token) = config::load_credentials()?;

    if space_id.is_empty() || api_token.is_empty() {
        eprintln!("{}", "Error: No credentials configured".red().bold());
        eprintln!("{}", "Run 'rpsn config init' to initialize, then 'rpsn config set --space <id> --token <token>' to set credentials".dimmed());
        std::process::exit(1);
    }

    let client = RepsonaClient::new(space_id, api_token, cli.dry_run, cli.trace);

    match cli.command {
        Commands::Util(UtilCommands::Version) => {
            util::handle_version();
        }
        Commands::Util(UtilCommands::Ping) => util::handle_ping(&client).await?,
        Commands::Config(cmd) => config_cmd::handle(cmd).await?,
        Commands::Me(cmd) => me::handle(&client, cmd, cli.json).await?,
        Commands::Project(cmd) => project::handle(&client, cmd, cli.json).await?,
        Commands::Task(cmd) => task::handle(&client, cmd, cli.json).await?,
        Commands::Note(cmd) => note::handle(&client, cmd, cli.json).await?,
        Commands::File(cmd) => file::handle(&client, cmd, cli.json).await?,
        Commands::Tag(cmd) => tag::handle(&client, cmd, cli.json).await?,
        Commands::Inbox(cmd) => inbox::handle(&client, cmd, cli.json).await?,
        Commands::Space(cmd) => space::handle(&client, cmd, cli.json).await?,
        Commands::User(cmd) => user::handle(&client, cmd, cli.json).await?,
        Commands::Webhook(cmd) => webhook::handle(&client, cmd, cli.json).await?,
        Commands::Idlink(cmd) => idlink::handle(&client, cmd, cli.json).await?,
        Commands::Completion { .. } => unreachable!(),
        Commands::SkillGenerate { .. } => unreachable!(),
        Commands::Report(_) => unreachable!(),
    }

    Ok(())
}
