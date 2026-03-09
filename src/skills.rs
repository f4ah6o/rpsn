use anyhow::Result;
use clap::{Command, CommandFactory};
use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::cli::Cli;

const EXCLUDED_SKILL_COMMANDS: &[&str] = &["completion", "skills"];

pub fn render_markdown() -> String {
    render_markdown_from_command(Cli::command())
}

fn render_markdown_from_command(cmd: Command) -> String {
    let mut skill_content = String::new();
    skill_content.push_str("---\n");
    skill_content.push_str("name: rpsn\n");
    skill_content
        .push_str("description: Use the rpsn CLI to inspect and manage Repsona workspaces.\n");
    skill_content.push_str("---\n\n");
    skill_content.push_str("# rpsn\n\n");
    skill_content.push_str(
        "Use this skill when you need to inspect or modify Repsona projects, tasks, notes, files, users, webhooks, or inbox state through the `rpsn` CLI.\n\n",
    );
    skill_content.push_str("## Setup\n\n");
    skill_content.push_str("1. Initialize local configuration with `rpsn config init`.\n");
    skill_content.push_str(
        "2. Configure credentials with `rpsn config set --space <space_id> --token <api_token>`.\n",
    );
    skill_content.push_str("3. Verify access with `rpsn config whoami` or `rpsn util ping`.\n\n");
    skill_content.push_str("## Global Options\n\n");
    skill_content.push_str("- `--space <space_id>` overrides the configured Repsona Space ID.\n");
    skill_content.push_str("- `--token <api_token>` overrides the configured API token.\n");
    skill_content.push_str("- `--profile <name>` selects a named config profile.\n");
    skill_content.push_str(
        "- `--json` emits machine-readable JSON for scripting and follow-up processing.\n",
    );
    skill_content.push_str("- `--dry-run` shows the request without executing it.\n");
    skill_content.push_str("- `--yes` skips confirmation prompts.\n");
    skill_content.push_str("- `--trace` enables HTTP trace output for debugging.\n\n");
    skill_content.push_str("## Safety Notes\n\n");
    skill_content
        .push_str("- Prefer read-only commands first when the current Repsona state is unclear.\n");
    skill_content.push_str("- Use `--dry-run` before destructive or irreversible changes.\n");
    skill_content.push_str("- Provide explicit IDs and flags for write operations instead of relying on assumptions.\n\n");
    skill_content.push_str("## Command Catalog\n\n");

    for subcmd in cmd
        .get_subcommands()
        .filter(|subcmd| !EXCLUDED_SKILL_COMMANDS.contains(&subcmd.get_name()))
    {
        let name = subcmd.get_name();
        let description = subcmd
            .get_about()
            .map(std::string::ToString::to_string)
            .unwrap_or_default();

        skill_content.push_str(&format!("### `{}`\n\n", name));
        if !description.is_empty() {
            skill_content.push_str(&format!("{}\n\n", description));
        }
        skill_content.push_str("```bash\n");

        if subcmd.has_subcommands() {
            for nested in subcmd.get_subcommands() {
                let nested_name = nested.get_name();
                skill_content.push_str(&format!("rpsn {} {}\n", name, nested_name));
            }
        } else {
            skill_content.push_str(&format!("rpsn {}\n", name));
        }

        skill_content.push_str("```\n\n");
    }

    skill_content.push_str("## Output Modes\n\n");
    skill_content.push_str("- Run `rpsn skills` to print this `SKILL.md` content to stdout.\n");
    skill_content.push_str(
        "- Run `rpsn skills --output ./rpsn/SKILL.md` to write the same content to a file.\n",
    );

    skill_content
}

pub fn emit(output: Option<String>) -> Result<()> {
    let skill_content = render_markdown();

    if let Some(path) = output {
        write_to_path(Path::new(&path), &skill_content)?;
        eprintln!(
            "{}",
            format!("Skill file generated at: {}", path).green().bold()
        );
    } else {
        print!("{}", skill_content);
    }

    Ok(())
}

fn write_to_path(path: &Path, skill_content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, skill_content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::render_markdown;

    #[test]
    fn includes_frontmatter_and_setup() {
        let content = render_markdown();
        assert!(content.starts_with("---\nname: rpsn\n"));
        assert!(content.contains("## Setup"));
        assert!(content.contains("rpsn config init"));
    }

    #[test]
    fn includes_global_options() {
        let content = render_markdown();
        assert!(content.contains("`--json`"));
        assert!(content.contains("`--dry-run`"));
        assert!(content.contains("`--trace`"));
    }

    #[test]
    fn excludes_meta_commands() {
        let content = render_markdown();
        assert!(!content.contains("rpsn completion"));
        assert!(!content.contains("### `skills`"));
        assert!(!content.contains("skill-generate"));
    }
}
