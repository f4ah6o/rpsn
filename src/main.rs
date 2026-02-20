mod ai;
mod api;
mod cli;
mod commands;
mod config;
mod error_report;
mod output;
mod telemetry;
mod telemetry_span;

use anyhow::Result;
use clap::{ArgMatches, CommandFactory, FromArgMatches};
use clap_complete::{generate, Shell};
use std::ffi::OsString;
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

enum RunOutcome {
    Success,
    Exit(i32),
}

fn command_path_from_matches(matches: &ArgMatches) -> Vec<String> {
    let mut path = Vec::new();
    let mut current = matches;

    while let Some((name, sub)) = current.subcommand() {
        path.push(name.to_string());
        current = sub;
    }

    path
}

fn command_group_from_path(path: &[String]) -> String {
    path.first()
        .cloned()
        .unwrap_or_else(|| "unknown".to_string())
}

fn sanitize_cli_args(args: Vec<OsString>) -> String {
    let mut sanitized = Vec::new();
    let mut mask_next = false;

    for arg in args {
        let arg = arg.to_string_lossy().to_string();

        if mask_next {
            sanitized.push("***REDACTED***".to_string());
            mask_next = false;
            continue;
        }

        if arg == "--token" {
            sanitized.push(arg);
            mask_next = true;
            continue;
        }

        if let Some((key, _)) = arg.split_once('=') {
            if key == "--token" {
                sanitized.push(format!("{}=***REDACTED***", key));
                continue;
            }
        }

        sanitized.push(arg);
    }

    sanitized.join(" ")
}

async fn run_cli() -> Result<RunOutcome> {
    let mut root_attrs = vec![(
        "cli.args",
        sanitize_cli_args(std::env::args_os().skip(1).collect()),
    )];
    if let Ok(cwd) = std::env::current_dir() {
        root_attrs.push(("cwd", cwd.display().to_string()));
    }

    let root_span = telemetry_span::new_span("cli.unknown", &root_attrs);
    let _root_entered = root_span.enter();

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

    let (matches, cli) = telemetry_span::with_span_result("parse_args", &[], || {
        let matches = cmd.get_matches();
        let cli = Cli::from_arg_matches(&matches)?;
        Ok::<(ArgMatches, Cli), clap::Error>((matches, cli))
    })?;

    let command_path = command_path_from_matches(&matches);
    let command = command_path.join(".");
    let command_group = command_group_from_path(&command_path);
    let root_name = if command.is_empty() {
        "cli.unknown".to_string()
    } else {
        format!("cli.{}", command)
    };

    telemetry_span::set_span_attr(&root_span, "otel.name", &root_name);
    telemetry_span::set_span_attr(&root_span, "cli.command", &command);
    telemetry_span::set_span_attr(&root_span, "command.group", &command_group);

    let result = match cli.command {
        Commands::Completion { shell } => {
            let attrs = vec![
                ("command.group", command_group.clone()),
                ("op.phase", "execute_operation".to_string()),
            ];
            telemetry_span::with_span("main_operation", &attrs, || {
                generate_shell_completion(shell);
            });
            Ok(RunOutcome::Success)
        }
        Commands::SkillGenerate { output } => {
            let attrs = vec![
                ("command.group", command_group.clone()),
                ("op.phase", "execute_operation".to_string()),
            ];
            telemetry_span::with_span_result("main_operation", &attrs, || {
                generate_skill_file(output)
            })?;
            Ok(RunOutcome::Success)
        }
        Commands::Report(cmd) => {
            let attrs = vec![
                ("command.group", command_group.clone()),
                ("op.phase", "execute_operation".to_string()),
            ];
            telemetry_span::with_span_async_result("main_operation", &attrs, || {
                report::handle(cmd)
            })
            .await?;
            Ok(RunOutcome::Success)
        }
        command => {
            let (space_id, api_token) =
                telemetry_span::with_span_result("load_config", &[], config::load_credentials)?;

            if space_id.is_empty() || api_token.is_empty() {
                eprintln!("{}", "Error: No credentials configured".red().bold());
                eprintln!("{}", "Run 'rpsn config init' to initialize, then 'rpsn config set --space <id> --token <token>' to set credentials".dimmed());
                telemetry_span::mark_span_error(&root_span, "no credentials configured");
                return Ok(RunOutcome::Exit(1));
            }

            let client = RepsonaClient::new(space_id, api_token, cli.dry_run, cli.trace);

            let attrs = vec![
                ("command.group", command_group.clone()),
                ("op.phase", "execute_operation".to_string()),
            ];
            telemetry_span::with_span_async_result("main_operation", &attrs, || async {
                match command {
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

                Ok::<(), anyhow::Error>(())
            })
            .await?;

            Ok(RunOutcome::Success)
        }
    };

    if let Err(err) = &result {
        telemetry_span::mark_span_error(&root_span, err);
    }

    result
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut telemetry = telemetry::init_telemetry();
    telemetry_span::set_enabled(telemetry.enabled());

    let run_result = run_cli().await;
    telemetry.shutdown();

    match run_result? {
        RunOutcome::Success => Ok(()),
        RunOutcome::Exit(code) => std::process::exit(code),
    }
}
