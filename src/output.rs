use colored::Colorize;
use comfy_table::{presets::UTF8_FULL, Attribute, Cell, Color, ContentArrangement, Table};
use serde::Serialize;

pub enum OutputFormat {
    Human,
    Json,
}

pub fn print<T: Serialize>(data: &T, format: OutputFormat) -> anyhow::Result<()> {
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(data)?);
        }
        OutputFormat::Human => {
            let json = serde_json::to_value(data)?;
            print_json_value(&json);
        }
    }
    Ok(())
}

fn print_json_value(value: &serde_json::Value) {
    if let Some(obj) = value.as_object() {
        if obj.contains_key("user") {
            print_user(obj);
        } else if obj.contains_key("project") {
            print_project(obj);
        } else if obj.contains_key("task") {
            print_task(obj);
        } else if obj.contains_key("note") {
            print_note(obj);
        } else if obj.contains_key("projects") {
            print_projects(obj);
        } else if obj.contains_key("tasks") {
            print_tasks(obj);
        } else if obj.contains_key("notes") {
            print_notes(obj);
        } else if obj.contains_key("users") {
            print_users(obj);
        } else if obj.contains_key("tags") {
            print_tags(obj);
        } else if obj.contains_key("space") {
            print_space(obj);
        } else {
            println!("{}", serde_json::to_string_pretty(value).unwrap_or_else(|_| "N/A".to_string()));
        }
    }
}

fn print_user(obj: &serde_json::Map<String, serde_json::Value>) {
    if let Some(user) = obj.get("user").and_then(|v| v.as_object()) {
        println!("Name: {}", user.get("fullName").and_then(|v| v.as_str()).unwrap_or("N/A"));
        println!("Email: {}", user.get("email").and_then(|v| v.as_str()).unwrap_or("N/A"));
        println!("Role: {}", user.get("role").and_then(|v| v.as_str()).unwrap_or("N/A"));
        println!("ID: {}", user.get("id").and_then(|v| v.as_u64()).unwrap_or(0));
    }
}

fn print_project(obj: &serde_json::Map<String, serde_json::Value>) {
    if let Some(project) = obj.get("project").and_then(|v| v.as_object()) {
        println!("Name: {}", project.get("name").and_then(|v| v.as_str()).unwrap_or("N/A"));
        println!("Full Name: {}", project.get("fullName").and_then(|v| v.as_str()).unwrap_or("N/A"));
        if let Some(purpose) = project.get("purpose").and_then(|v| v.as_str()) {
            println!("Purpose: {}", purpose);
        }
        println!("ID: {}", project.get("id").and_then(|v| v.as_u64()).unwrap_or(0));
    }
}

fn print_task(obj: &serde_json::Map<String, serde_json::Value>) {
    if let Some(task) = obj.get("task").and_then(|v| v.as_object()) {
        println!("Title: {}", task.get("name").and_then(|v| v.as_str()).unwrap_or("N/A"));
        if let Some(desc) = task.get("description").and_then(|v| v.as_str()) {
            println!("Description: {}", desc);
        }
        if let Some(status) = task.get("status").and_then(|v| v.as_object()) {
            println!("Status: {}", status.get("name").and_then(|v| v.as_str()).unwrap_or("N/A"));
        }
        println!("Priority: {}", task.get("priority").and_then(|v| v.as_u64()).unwrap_or(0));
        if let Some(due) = task.get("dueDate").and_then(|v| v.as_u64()) {
            println!("Due Date: {}", due);
        }
        if let Some(resp) = task.get("responsibleUser").and_then(|v| v.as_object()) {
            println!("Responsible: {}", resp.get("fullName").and_then(|v| v.as_str()).unwrap_or("N/A"));
        }
        println!("ID: {}", task.get("id").and_then(|v| v.as_u64()).unwrap_or(0));
    }
}

fn print_note(obj: &serde_json::Map<String, serde_json::Value>) {
    if let Some(note) = obj.get("note").and_then(|v| v.as_object()) {
        println!("Name: {}", note.get("name").and_then(|v| v.as_str()).unwrap_or("N/A"));
        if let Some(desc) = note.get("description").and_then(|v| v.as_str()) {
            println!("Description: {}", desc);
        }
        println!("ID: {}", note.get("id").and_then(|v| v.as_u64()).unwrap_or(0));
    }
}

fn print_projects(obj: &serde_json::Map<String, serde_json::Value>) {
    if let Some(projects) = obj.get("projects").and_then(|v| v.as_array()) {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["ID", "Name", "Status"]);

        for project in projects {
            let id = project.get("id").and_then(|v| v.as_u64()).unwrap_or(0).to_string();
            let name = project.get("name").and_then(|v| v.as_str()).unwrap_or("N/A").to_string();
            let status = if project.get("isClosed").and_then(|v| v.as_bool()).unwrap_or(false) {
                Cell::new("Closed".to_string()).add_attribute(Attribute::Bold).fg(Color::Red)
            } else {
                Cell::new("Open".to_string()).add_attribute(Attribute::Bold).fg(Color::Green)
            };

            table.add_row(vec![Cell::new(id), Cell::new(name), status]);
        }

        println!("{}", table);
    }
}

fn print_tasks(obj: &serde_json::Map<String, serde_json::Value>) {
    if let Some(tasks) = obj.get("tasks").and_then(|v| v.as_array()) {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["ID", "Title", "Status", "Priority", "Due"]);

        for task in tasks {
            let id = task.get("id").and_then(|v| v.as_u64()).unwrap_or(0).to_string();
            let title = task.get("name").and_then(|v| v.as_str()).unwrap_or("N/A").to_string();
            let status = task.get("status")
                .and_then(|v| v.as_object())
                .and_then(|s| s.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or("N/A")
                .to_string();
            let priority = task.get("priority").and_then(|v| v.as_u64()).unwrap_or(0).to_string();
            let due = task.get("dueDate")
                .and_then(|v| v.as_u64())
                .map(|d| d.to_string())
                .unwrap_or_else(|| "-".to_string());

            table.add_row(vec![id, title, status, priority, due]);
        }

        println!("{}", table);
    }
}

fn print_notes(obj: &serde_json::Map<String, serde_json::Value>) {
    if let Some(notes) = obj.get("notes").and_then(|v| v.as_array()) {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["ID", "Name", "Updated"]);

        for note in notes {
            let id = note.get("id").and_then(|v| v.as_u64()).unwrap_or(0).to_string();
            let name = note.get("name").and_then(|v| v.as_str()).unwrap_or("N/A").to_string();
            let updated = note.get("updatedAt")
                .and_then(|v| v.as_u64())
                .map(|d| d.to_string())
                .unwrap_or_else(|| "-".to_string());

            table.add_row(vec![id, name, updated]);
        }

        println!("{}", table);
    }
}

fn print_users(obj: &serde_json::Map<String, serde_json::Value>) {
    if let Some(users) = obj.get("users").and_then(|v| v.as_array()) {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["ID", "Name", "Email", "Role"]);

        for user in users {
            let id = user.get("id").and_then(|v| v.as_u64()).unwrap_or(0).to_string();
            let name = user.get("fullName").and_then(|v| v.as_str()).unwrap_or("N/A").to_string();
            let email = user.get("email").and_then(|v| v.as_str()).unwrap_or("N/A").to_string();
            let role = user.get("role").and_then(|v| v.as_str()).unwrap_or("N/A").to_string();

            table.add_row(vec![id, name, email, role]);
        }

        println!("{}", table);
    }
}

fn print_tags(obj: &serde_json::Map<String, serde_json::Value>) {
    if let Some(tags) = obj.get("tags").and_then(|v| v.as_array()) {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["ID", "Name", "Color"]);

        for tag in tags {
            let id = tag.get("id").and_then(|v| v.as_u64()).unwrap_or(0).to_string();
            let name = tag.get("name").and_then(|v| v.as_str()).unwrap_or("N/A").to_string();
            let color = tag.get("color").and_then(|v| v.as_str()).unwrap_or("N/A").to_string();

            table.add_row(vec![id, name, color]);
        }

        println!("{}", table);
    }
}

fn print_space(obj: &serde_json::Map<String, serde_json::Value>) {
    if let Some(space) = obj.get("space").and_then(|v| v.as_object()) {
        println!("Name: {}", space.get("name").and_then(|v| v.as_str()).unwrap_or("N/A"));
        println!("Full Name: {}", space.get("fullName").and_then(|v| v.as_str()).unwrap_or("N/A"));
        println!("Status: {}", space.get("status").and_then(|v| v.as_str()).unwrap_or("N/A"));
        println!("ID: {}", space.get("id").and_then(|v| v.as_u64()).unwrap_or(0));
    }
}

pub fn print_success(message: &str) {
    println!("{}", message.green().bold());
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_output_format_json() {
        #[derive(Serialize)]
        struct TestData {
            name: String,
            value: i32,
        }

        let data = TestData {
            name: "test".to_string(),
            value: 42,
        };

        let result = print(&data, OutputFormat::Json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_json_value_with_user() {
        let user_data = json!({
            "user": {
                "id": 123,
                "fullName": "Test User",
                "email": "test@example.com",
                "role": "admin"
            }
        });

        let result = print(&user_data, OutputFormat::Human);
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_json_value_with_project() {
        let project_data = json!({
            "project": {
                "id": 456,
                "name": "project1",
                "fullName": "My Project",
                "purpose": "Testing"
            }
        });

        let result = print(&project_data, OutputFormat::Human);
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_json_value_with_task() {
        let task_data = json!({
            "task": {
                "id": 789,
                "name": "Test Task",
                "description": "A test task",
                "status": {
                    "name": "Open"
                },
                "priority": 2
            }
        });

        let result = print(&task_data, OutputFormat::Human);
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_json_value_with_projects_list() {
        let projects_data = json!({
            "projects": [
                {
                    "id": 1,
                    "name": "project1",
                    "isClosed": false
                },
                {
                    "id": 2,
                    "name": "project2",
                    "isClosed": true
                }
            ]
        });

        let result = print(&projects_data, OutputFormat::Human);
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_json_value_with_tasks_list() {
        let tasks_data = json!({
            "tasks": [
                {
                    "id": 1,
                    "name": "Task 1",
                    "status": {"name": "Open"},
                    "priority": 0,
                    "dueDate": null
                },
                {
                    "id": 2,
                    "name": "Task 2",
                    "status": {"name": "Done"},
                    "priority": 1,
                    "dueDate": 1640000000
                }
            ]
        });

        let result = print(&tasks_data, OutputFormat::Human);
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_json_value_with_notes_list() {
        let notes_data = json!({
            "notes": [
                {
                    "id": 1,
                    "name": "Note 1",
                    "updatedAt": 1640000000
                },
                {
                    "id": 2,
                    "name": "Note 2",
                    "updatedAt": 1640001000
                }
            ]
        });

        let result = print(&notes_data, OutputFormat::Human);
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_json_value_with_users_list() {
        let users_data = json!({
            "users": [
                {
                    "id": 1,
                    "fullName": "User 1",
                    "email": "user1@example.com",
                    "role": "admin"
                },
                {
                    "id": 2,
                    "fullName": "User 2",
                    "email": "user2@example.com",
                    "role": "member"
                }
            ]
        });

        let result = print(&users_data, OutputFormat::Human);
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_json_value_with_tags_list() {
        let tags_data = json!({
            "tags": [
                {
                    "id": 1,
                    "name": "bug",
                    "color": "#ff0000"
                },
                {
                    "id": 2,
                    "name": "feature",
                    "color": "#00ff00"
                }
            ]
        });

        let result = print(&tags_data, OutputFormat::Human);
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_json_value_with_space() {
        let space_data = json!({
            "space": {
                "id": 111,
                "name": "myspace",
                "fullName": "My Workspace",
                "status": "active"
            }
        });

        let result = print(&space_data, OutputFormat::Human);
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_json_value_with_note() {
        let note_data = json!({
            "note": {
                "id": 999,
                "name": "My Note",
                "description": "Note description"
            }
        });

        let result = print(&note_data, OutputFormat::Human);
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_empty_tasks_list() {
        let empty_tasks = json!({
            "tasks": []
        });

        let result = print(&empty_tasks, OutputFormat::Human);
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_empty_projects_list() {
        let empty_projects = json!({
            "projects": []
        });

        let result = print(&empty_projects, OutputFormat::Human);
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_unknown_json_structure() {
        let unknown_data = json!({
            "unknown_field": "value",
            "another_field": 123
        });

        let result = print(&unknown_data, OutputFormat::Human);
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_task_with_null_fields() {
        let task_with_nulls = json!({
            "task": {
                "id": 1,
                "name": "Task",
                "description": null,
                "status": {"name": "Open"},
                "priority": 0,
                "responsibleUser": null,
                "dueDate": null
            }
        });

        let result = print(&task_with_nulls, OutputFormat::Human);
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_user_with_all_fields() {
        let full_user = json!({
            "user": {
                "id": 123,
                "fullName": "Complete User",
                "email": "complete@example.com",
                "role": "admin"
            }
        });

        let result = print(&full_user, OutputFormat::Human);
        assert!(result.is_ok());
    }
}
