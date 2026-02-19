use crate::api::endpoints::project::CreateProjectRequest;
use crate::api::endpoints::project::UpdateProjectRequest;
use crate::api::endpoints::task::CreateTaskRequest;
use crate::api::RepsonaClient;
use anyhow::{anyhow, Context, Result};
use serde_json::Value;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn load_live_client() -> Result<RepsonaClient> {
    let (space_id, api_token) = crate::config::load_credentials().context(
        "Failed to load credentials. Provide REPSONA_SPACE and REPSONA_TOKEN via `opz rpsn-dev -- ...`.",
    )?;
    Ok(RepsonaClient::new(space_id, api_token, false, false))
}

fn safe_unique_name(prefix: &str, max_len: usize) -> String {
    let cleaned_prefix: String = prefix
        .chars()
        .filter_map(|ch| {
            if ch.is_ascii_alphanumeric() {
                Some(ch.to_ascii_lowercase())
            } else if ch == '-' || ch == '_' || ch == ' ' {
                Some('-')
            } else {
                None
            }
        })
        .collect();
    let cleaned_prefix = cleaned_prefix.trim_matches('-');
    let base = if cleaned_prefix.is_empty() {
        "rpsn"
    } else {
        cleaned_prefix
    };

    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;
    let suffix = format!("{:x}", millis);

    if max_len <= suffix.len() + 1 {
        return suffix.chars().take(max_len.max(1)).collect();
    }

    let max_prefix_len = max_len - suffix.len() - 1;
    let short_prefix: String = base.chars().take(max_prefix_len).collect();
    format!("{short_prefix}-{suffix}")
}

fn is_invalid_parameter_error(err: &anyhow::Error) -> bool {
    let text = err.to_string();
    text.contains("400 Bad Request") && text.contains("Invalid parameters")
}

fn is_free_plan_project_limit_error(err: &anyhow::Error) -> bool {
    err.to_string()
        .contains("Only one project can be created in the free plan.")
}

async fn first_project_id(client: &RepsonaClient) -> Result<u64> {
    match client.list_projects().await {
        Ok(resp) => {
            if let Some(first) = resp.data.projects.first() {
                return Ok(first.id);
            }
        }
        Err(err) => {
            if !err.to_string().contains("404 Not Found") {
                return Err(err).context("Failed to list projects for free-plan fallback");
            }
        }
    }

    let me_projects = client
        .get_me_projects()
        .await
        .context("Failed to list me/projects for free-plan fallback")?
        .data
        .projects;
    let first = me_projects
        .first()
        .ok_or_else(|| anyhow!("No existing project available for free-plan fallback"))?;
    Ok(first.id)
}

async fn create_project_compat(client: &RepsonaClient, name: &str) -> Result<u64> {
    let variants = vec![
        (
            "name + fullName, no purpose",
            CreateProjectRequest {
                name: name.to_string(),
                full_name: Some(name.to_string()),
                purpose: None,
            },
        ),
        (
            "name only",
            CreateProjectRequest {
                name: name.to_string(),
                full_name: None,
                purpose: None,
            },
        ),
        (
            "name + fullName + short purpose",
            CreateProjectRequest {
                name: name.to_string(),
                full_name: Some(name.to_string()),
                purpose: Some("live test".to_string()),
            },
        ),
    ];

    let mut errors = Vec::new();

    for (variant_name, payload) in variants {
        match client.create_project(&payload).await {
            Ok(resp) => return Ok(resp.data.project.id),
            Err(err) => {
                let err_text = err.to_string();
                errors.push(format!("variant `{variant_name}` failed: {err_text}"));
                if !is_invalid_parameter_error(&err) {
                    return Err(anyhow!(
                        "Failed to create project with `{variant_name}`: {err_text}"
                    ));
                }
            }
        }
    }

    Err(anyhow!(
        "Failed to create project after all compatibility variants: {}",
        errors.join(" | ")
    ))
}

fn cleanup_error_message(cleanup_errors: &[String]) -> String {
    cleanup_errors.join(" | ")
}

fn extract_u64(value: &Value, path: &str) -> Result<u64> {
    value
        .pointer(path)
        .and_then(|v| v.as_u64())
        .ok_or_else(|| anyhow!("Missing or invalid u64 at JSON pointer `{path}`"))
}

fn extract_string(value: &Value, path: &str) -> Result<String> {
    value
        .pointer(path)
        .and_then(|v| v.as_str())
        .map(ToString::to_string)
        .ok_or_else(|| anyhow!("Missing or invalid string at JSON pointer `{path}`"))
}

async fn delete_task_with_probe(
    client: &RepsonaClient,
    project_id: u64,
    task_id: u64,
) -> Result<()> {
    match client.delete_task(project_id, task_id).await {
        Ok(()) => Ok(()),
        Err(err) => {
            let err_text = err.to_string();
            if !err_text.contains("Failed to parse response") {
                return Err(err);
            }

            let get_task_endpoint = format!("project/{project_id}/task/{task_id}");
            match client.get::<Value>(&get_task_endpoint).await {
                Err(probe_err) if probe_err.to_string().contains("404 Not Found") => Ok(()),
                _ => Err(anyhow!(
                    "task {task_id} delete returned parse error and task still appears accessible: {err_text}"
                )),
            }
        }
    }
}

fn combine_test_and_cleanup_error(
    test_err: anyhow::Error,
    cleanup_errors: &[String],
) -> Result<()> {
    if cleanup_errors.is_empty() {
        Err(test_err)
    } else {
        Err(anyhow!(
            "{test_err}; cleanup errors: {}",
            cleanup_error_message(cleanup_errors)
        ))
    }
}

fn fail_on_cleanup_errors(cleanup_errors: &[String]) -> Result<()> {
    if cleanup_errors.is_empty() {
        Ok(())
    } else {
        Err(anyhow!(
            "Cleanup failed: {}",
            cleanup_error_message(cleanup_errors)
        ))
    }
}

#[tokio::test]
#[ignore]
async fn live_api_read_me_ok() -> Result<()> {
    println!("[live_api] start: read me");
    let client = load_live_client()?;
    println!("[live_api] call: GET /me");
    let response = client.get_me().await.context("GET /me failed")?;
    assert!(response.data.user.id > 0, "expected user.id > 0");
    assert!(
        !response.data.user.email.is_empty(),
        "expected non-empty user email"
    );
    println!("[live_api] done: read me");
    Ok(())
}

#[tokio::test]
#[ignore]
async fn live_api_write_project_create_delete_ok() -> Result<()> {
    println!("[live_api] start: project write flow");
    let client = load_live_client()?;
    let mut cleanup_errors = Vec::new();
    let mut created_project_id: Option<u64> = None;
    let mut revert_project_purpose: Option<(u64, Option<String>)> = None;
    let project_name = safe_unique_name(
        &env::var("RPSN_LIVE_NAME_PREFIX").unwrap_or_else(|_| "rpsn-p".to_string()),
        24,
    );

    let test_result: Result<()> = async {
        println!("[live_api] step: create project (compat)");
        match create_project_compat(&client, &project_name).await {
            Ok(created_id) => {
                created_project_id = Some(created_id);
                println!("[live_api] step: project created id={created_id}");

                let fetched = client
                    .get_project(created_id)
                    .await
                    .context("Failed to fetch created project")?;
                println!("[live_api] step: verify created project id={created_id}");
                assert_eq!(fetched.data.project.id, created_id);
                assert_eq!(fetched.data.project.name, project_name);
            }
            Err(err) if is_free_plan_project_limit_error(&err) => {
                println!("[live_api] step: free-plan fallback to existing project");
                let existing_project_id = first_project_id(&client).await?;
                let existing = client
                    .get_project(existing_project_id)
                    .await
                    .context("Failed to fetch fallback project")?;
                let original_purpose = existing.data.project.purpose.clone();
                revert_project_purpose = Some((existing_project_id, original_purpose.clone()));

                let marker = format!("live-test-{}", safe_unique_name("p", 12));
                client
                    .update_project(
                        existing_project_id,
                        &UpdateProjectRequest {
                            name: None,
                            full_name: None,
                            purpose: Some(marker.clone()),
                        },
                    )
                    .await
                    .context("Failed to update fallback project purpose")?;
                println!("[live_api] step: fallback project updated id={existing_project_id}");

                let updated = client
                    .get_project(existing_project_id)
                    .await
                    .context("Failed to verify fallback project update")?;
                assert_eq!(updated.data.project.id, existing_project_id);
                assert_eq!(
                    updated.data.project.purpose.as_deref(),
                    Some(marker.as_str())
                );
            }
            Err(err) => return Err(err).context("Failed to create project"),
        }
        Ok(())
    }
    .await;

    if let Some((pid, original_purpose)) = revert_project_purpose {
        if let Err(err) = client
            .update_project(
                pid,
                &UpdateProjectRequest {
                    name: None,
                    full_name: None,
                    purpose: original_purpose,
                },
            )
            .await
        {
            cleanup_errors.push(format!("project {pid} purpose revert failed: {err}"));
        }
    }

    if let Some(id) = created_project_id {
        println!("[live_api] cleanup: delete created project id={id}");
        if let Err(err) = client.delete_project(id).await {
            cleanup_errors.push(format!("project {id} delete failed: {err}"));
        }
    }

    if let Err(test_err) = test_result {
        return combine_test_and_cleanup_error(test_err, &cleanup_errors);
    }

    println!("[live_api] done: project write flow");
    fail_on_cleanup_errors(&cleanup_errors)
}

#[tokio::test]
#[ignore]
async fn live_api_write_task_create_delete_ok() -> Result<()> {
    println!("[live_api] start: task write flow");
    let client = load_live_client()?;
    let mut cleanup_errors = Vec::new();
    let mut project_id: Option<u64> = None;
    let mut task_id: Option<u64> = None;
    let mut owns_project = false;
    let project_name = safe_unique_name(
        &env::var("RPSN_LIVE_NAME_PREFIX").unwrap_or_else(|_| "rpsn-tp".to_string()),
        24,
    );
    let task_name = safe_unique_name("rpsn-t", 24);

    let test_result: Result<()> = async {
        println!("[live_api] step: ensure target project (create/fallback)");
        let target_project_id = match create_project_compat(&client, &project_name).await {
            Ok(created_project_id) => {
                owns_project = true;
                println!("[live_api] step: created temporary project id={created_project_id}");
                created_project_id
            }
            Err(err) if is_free_plan_project_limit_error(&err) => {
                owns_project = false;
                let pid = first_project_id(&client).await?;
                println!("[live_api] step: free-plan fallback project id={pid}");
                pid
            }
            Err(err) => return Err(err).context("Failed to create temporary project"),
        };
        project_id = Some(target_project_id);

        println!("[live_api] step: create task");
        let create_task_endpoint = format!("project/{target_project_id}/task");
        let created_task: Value = client
            .post(
                &create_task_endpoint,
                &CreateTaskRequest {
                    name: task_name.clone(),
                    ..Default::default()
                },
            )
            .await
            .context("Failed to create task")?;
        let created_task_id = extract_u64(&created_task, "/task/id")
            .context("Failed to read created task id from response")?;
        task_id = Some(created_task_id);
        println!("[live_api] step: task created id={created_task_id}");

        println!("[live_api] step: fetch created task id={created_task_id}");
        let get_task_endpoint = format!("project/{target_project_id}/task/{created_task_id}");
        let fetched_task: Value = client
            .get(&get_task_endpoint)
            .await
            .context("Failed to fetch created task")?;

        let fetched_task_id = extract_u64(&fetched_task, "/task/id")
            .context("Failed to read fetched task id from response")?;
        let fetched_task_name = extract_string(&fetched_task, "/task/name")
            .context("Failed to read fetched task name from response")?;
        let fetched_project_id = extract_u64(&fetched_task, "/task/project/id")
            .context("Failed to read fetched task project id from response")?;

        assert_eq!(fetched_task_id, created_task_id);
        assert_eq!(fetched_task_name, task_name);
        assert_eq!(fetched_project_id, target_project_id);
        Ok(())
    }
    .await;

    if let (Some(pid), Some(tid)) = (project_id, task_id) {
        println!("[live_api] cleanup: delete task id={tid} project_id={pid}");
        if let Err(err) = delete_task_with_probe(&client, pid, tid).await {
            cleanup_errors.push(format!("task {tid} in project {pid} delete failed: {err}"));
        }
    }

    if owns_project {
        if let Some(pid) = project_id {
            println!("[live_api] cleanup: delete temporary project id={pid}");
            if let Err(err) = client.delete_project(pid).await {
                cleanup_errors.push(format!("project {pid} delete failed: {err}"));
            }
        }
    }

    if !owns_project {
        // No project cleanup required when using an existing free-plan project.
    }

    if let Err(test_err) = test_result {
        return combine_test_and_cleanup_error(test_err, &cleanup_errors);
    }

    println!("[live_api] done: task write flow");
    fail_on_cleanup_errors(&cleanup_errors)
}
