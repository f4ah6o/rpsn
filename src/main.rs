mod ai;
mod api;
mod cli;
mod commands;
mod config;
mod error_report;
mod output;
mod skills;
mod telemetry;
mod telemetry_span;

use anyhow::Result;
use clap::{ArgMatches, CommandFactory, FromArgMatches};
use clap_complete::{generate, Shell};
use colored::Colorize;
use std::ffi::OsString;

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
        Commands::Util(UtilCommands::Version) => {
            let attrs = vec![
                ("command.group", command_group.clone()),
                ("op.phase", "execute_operation".to_string()),
            ];
            telemetry_span::with_span("main_operation", &attrs, || {
                util::handle_version();
            });
            Ok(RunOutcome::Success)
        }
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
        Commands::Skills { output } => {
            let attrs = vec![
                ("command.group", command_group.clone()),
                ("op.phase", "execute_operation".to_string()),
            ];
            telemetry_span::with_span_result("main_operation", &attrs, || skills::emit(output))?;
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
                    Commands::Util(UtilCommands::Version) => unreachable!(),
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
                    Commands::Skills { .. } => unreachable!(),
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
