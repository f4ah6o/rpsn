use crate::api::RepsonaClient;
use crate::telemetry_span;
use anyhow::Result;
use colored::Colorize;

fn phase_attrs(phase: &str) -> Vec<(&'static str, String)> {
    vec![
        ("command.group", "util".to_string()),
        ("op.phase", phase.to_string()),
    ]
}

pub fn handle_version() {
    let attrs = phase_attrs("render_output");
    telemetry_span::with_span("render_output", &attrs, || {
        println!("rpsn 0.1.0");
    });
}

pub async fn handle_ping(client: &RepsonaClient) -> Result<()> {
    let render_attrs = phase_attrs("render_output");
    telemetry_span::with_span("render_output", &render_attrs, || {
        println!("{}", "Pinging Repsona API...".dimmed());
    });

    let exec_attrs = phase_attrs("execute_operation");
    telemetry_span::with_span_async_result("execute_operation", &exec_attrs, || async {
        client.get::<serde_json::Value>("me").await
    })
    .await?;

    telemetry_span::with_span("render_output", &render_attrs, || {
        println!("{}", "✓ API is reachable".green().bold());
    });
    Ok(())
}
