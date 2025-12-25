//! Error report command handlers.

use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::io::{self, Read};

use crate::cli::ReportCommands;
use crate::error_report::{ErrorReport, SensitiveData};

/// Handle report subcommands.
pub async fn handle(cmd: ReportCommands) -> Result<()> {
    match cmd {
        ReportCommands::Generate { error, command, output } => {
            handle_generate(error, command, output).await
        }
        ReportCommands::Test => handle_test().await,
        ReportCommands::Info => handle_info().await,
    }
}

/// Generate an error report from an error message.
async fn handle_generate(
    error_msg: Option<String>,
    command: Option<String>,
    output: Option<String>,
) -> Result<()> {
    // Get error message from argument or stdin
    let error_text = if let Some(msg) = error_msg {
        msg
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer.trim().to_string()
    };

    if error_text.is_empty() {
        return Err(anyhow::anyhow!("No error message provided. Use --error or pipe from stdin."));
    }

    // Build sensitive data registry
    let mut sensitive = SensitiveData::new();
    sensitive.load_from_environment();

    // Try to load from config as well
    if let Ok((space_id, api_token)) = crate::config::load_credentials() {
        sensitive.load_from_profile(&space_id, &api_token);
    }

    // Create the error and report
    let error = anyhow::anyhow!("{}", error_text);
    let report = ErrorReport::new(&error, command.as_deref(), &sensitive);

    // Verify the report is safe
    if !report.verify_no_sensitive_data(&sensitive) {
        eprintln!("{}", "Warning: Report may still contain sensitive data after sanitization.".yellow());
        eprintln!("{}", "Please review carefully before submitting.".yellow());
    }

    let markdown = report.to_markdown();

    // Output the report
    if let Some(path) = output {
        fs::write(&path, &markdown)?;
        println!("{}", format!("Report saved to: {}", path).green());
    } else {
        println!("{}", markdown);
    }

    println!();
    println!("{}", "Note: This report has been sanitized to remove sensitive data.".dimmed());
    println!("{}", "Please review before posting to GitHub issues.".dimmed());

    Ok(())
}

/// Generate a test report to show what information is collected.
async fn handle_test() -> Result<()> {
    let mut sensitive = SensitiveData::new();

    // Register some test secrets
    sensitive.register("test-api-token-12345");
    sensitive.register("test-space-id");

    // Create a sample error that might contain sensitive data
    let sample_error = anyhow::anyhow!(
        "API error (500): Internal server error at https://test-space-id.repsona.com/api/tasks. \
         Authorization: Bearer test-api-token-12345. Request ID: 550e8400-e29b-41d4-a716-446655440000"
    );

    let mut report = ErrorReport::new(
        &sample_error,
        Some("task create --title 'Secret Project' --description 'Confidential info'"),
        &sensitive,
    );
    report.add_context("Retry attempt: 3", &sensitive);
    report.add_context("Connection timeout after 30s", &sensitive);

    let markdown = report.to_markdown();

    println!("{}", "=== TEST ERROR REPORT ===".bold().cyan());
    println!();
    println!("{}", markdown);
    println!();

    // Show what was redacted
    println!("{}", "=== REDACTION SUMMARY ===".bold().cyan());
    println!();
    println!("{}", "The following types of data are automatically redacted:".yellow());
    println!("  • API tokens and credentials");
    println!("  • Space IDs (in URLs and text)");
    println!("  • UUIDs and request IDs");
    println!("  • Bearer tokens");
    println!("  • Base64-encoded tokens (32+ chars)");
    println!("  • Command arguments (only command name is kept)");
    println!();

    if report.verify_no_sensitive_data(&sensitive) {
        println!("{}", "✓ Report verified: No registered sensitive data found.".green());
    } else {
        println!("{}", "✗ Warning: Some sensitive data may still be present.".red());
    }

    Ok(())
}

/// Show information about error reporting.
async fn handle_info() -> Result<()> {
    println!("{}", "=== Error Report Information ===".bold().cyan());
    println!();

    println!("{}", "WHAT IS COLLECTED:".bold());
    println!("  • rpsn version");
    println!("  • Operating system and architecture");
    println!("  • Error category (Network, Auth, API, Parse, Config, etc.)");
    println!("  • HTTP status codes (if applicable)");
    println!("  • Command name (without arguments)");
    println!("  • Sanitized error message");
    println!();

    println!("{}", "WHAT IS NEVER COLLECTED:".bold().red());
    println!("  • API tokens or credentials");
    println!("  • Space IDs");
    println!("  • Environment variable values (REPSONA_SPACE, REPSONA_TOKEN)");
    println!("  • Configuration file contents");
    println!("  • Data from Repsona (task names, notes, user info, etc.)");
    println!("  • Command arguments (may contain sensitive data)");
    println!("  • UUIDs, request IDs, or any unique identifiers");
    println!("  • File paths (may contain usernames)");
    println!();

    println!("{}", "HOW TO USE:".bold());
    println!("  1. Run: rpsn report generate --error \"<error message>\"");
    println!("  2. Or pipe: echo \"<error>\" | rpsn report generate");
    println!("  3. Review the output for any remaining sensitive data");
    println!("  4. Copy and paste into a GitHub issue");
    println!();

    println!("{}", "TESTING:".bold());
    println!("  Run 'rpsn report test' to see a sample report with redactions.");
    println!();

    println!("{}", "SAFETY GUARANTEES:".bold().green());
    println!("  • Property-based tests ensure secrets are always redacted");
    println!("  • Multiple layers of sanitization (registered secrets + pattern matching)");
    println!("  • Reports are verified before output");
    println!("  • Open source: audit the code at src/error_report.rs");

    Ok(())
}
