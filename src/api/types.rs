use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse<T> {
    #[serde(rename = "requestedBy")]
    pub requested_by: u64,
    #[serde(flatten)]
    pub data: T,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct User {
    pub id: u64,
    pub email: String,
    pub name: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: Option<String>,
    pub role: String,
    #[serde(rename = "billingStatus")]
    pub billing_status: String,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    #[serde(rename = "updatedAt")]
    pub updated_at: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectSummary {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project {
    pub id: u64,
    pub name: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
    pub purpose: Option<String>,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: Option<String>,
    #[serde(rename = "isClosed")]
    pub is_closed: bool,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    #[serde(rename = "updatedAt")]
    pub updated_at: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Status {
    pub id: u64,
    pub name: String,
    #[serde(rename = "isClosed")]
    pub is_closed: bool,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Milestone {
    pub id: u64,
    pub name: String,
    #[serde(rename = "dueDate")]
    pub due_date: Option<u64>,
    #[serde(rename = "isClosed")]
    pub is_closed: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tag {
    pub id: u64,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Task {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub status: Status,
    pub priority: u32,
    #[serde(rename = "dueDate")]
    pub due_date: Option<u64>,
    #[serde(rename = "startDate")]
    pub start_date: Option<u64>,
    #[serde(rename = "responsibleUser")]
    pub responsible_user: Option<User>,
    #[serde(rename = "ballHoldingUser")]
    pub ball_holding_user: Option<User>,
    pub tags: Vec<Tag>,
    pub project: ProjectSummary,
    pub milestone: Option<Milestone>,
    pub parent: Option<u64>,
    #[serde(rename = "sortOrder")]
    pub sort_order: u32,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    #[serde(rename = "updatedAt")]
    pub updated_at: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Note {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub tags: Vec<Tag>,
    pub parent: Option<u64>,
    pub project: ProjectSummary,
    #[serde(rename = "sortOrder")]
    pub sort_order: u32,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    #[serde(rename = "updatedAt")]
    pub updated_at: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct File {
    pub id: u64,
    pub hash: String,
    pub filename: String,
    pub size: u64,
    #[serde(rename = "type")]
    pub file_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Comment {
    pub id: u64,
    pub comment: String,
    pub user: User,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskComment {
    pub id: u64,
    pub comment: String,
    pub user: User,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NoteComment {
    pub id: u64,
    pub comment: String,
    pub user: User,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InboxItem {
    pub id: u64,
    pub task: Option<Task>,
    pub note: Option<Note>,
    pub comment: Option<Comment>,
    #[serde(rename = "readAt")]
    pub read_at: Option<u64>,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Space {
    pub id: u64,
    pub name: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
    pub information: Option<String>,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: Option<String>,
    pub status: String,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    #[serde(rename = "updatedAt")]
    pub updated_at: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Webhook {
    pub id: u64,
    pub name: String,
    pub url: String,
    pub events: Vec<String>,
    pub active: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IdLink {
    pub id: u64,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Activity {
    pub id: u64,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    pub action: String,
    pub user: Option<User>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct History {
    pub id: u64,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    pub action: String,
    pub user: Option<User>,
    pub changes: Option<Vec<Change>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Change {
    pub field: String,
    pub from: Option<String>,
    pub to: Option<String>,
}

// Response wrapper types for flattened ApiResponse
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserData {
    pub user: User,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectData {
    pub project: Project,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskData {
    pub task: Task,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NoteData {
    pub note: Note,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpaceData {
    pub space: Space,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectsData {
    pub projects: Vec<Project>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TasksData {
    pub tasks: Vec<Task>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotesData {
    pub notes: Vec<Note>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UsersData {
    pub users: Vec<User>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TagsData {
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StatusesData {
    pub statuses: Vec<Status>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MilestonesData {
    pub milestones: Vec<Milestone>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskCommentsData {
    pub task_comments: Vec<TaskComment>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskCommentData {
    pub task_comment: TaskComment,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NoteCommentsData {
    pub note_comments: Vec<NoteComment>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NoteCommentData {
    pub note_comment: NoteComment,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActivityData {
    pub activity: Vec<Activity>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HistoryData {
    pub history: Vec<History>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InboxData {
    pub inbox: Vec<InboxItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InboxItemData {
    pub inbox: InboxItem,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UnreadCountData {
    pub count: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskCountData {
    pub count: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhooksData {
    pub webhooks: Vec<Webhook>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookData {
    pub webhook: Webhook,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IdLinksData {
    pub idlinks: Vec<IdLink>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IdLinkData {
    pub idlink: IdLink,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FilesData {
    pub files: Vec<File>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileData {
    pub file: File,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Invite {
    pub id: u64,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InviteData {
    pub invite: Invite,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_deserialization() {
        let json = r#"{
            "id": 123,
            "email": "user@example.com",
            "name": "testuser",
            "fullName": "Test User",
            "avatarUrl": "https://example.com/avatar.png",
            "role": "admin",
            "billingStatus": "active",
            "createdAt": 1640000000,
            "updatedAt": 1640001000
        }"#;

        let user: User = serde_json::from_str(json).unwrap();
        assert_eq!(user.id, 123);
        assert_eq!(user.email, "user@example.com");
        assert_eq!(user.name, "testuser");
        assert_eq!(user.full_name, "Test User");
        assert_eq!(user.avatar_url, Some("https://example.com/avatar.png".to_string()));
        assert_eq!(user.role, "admin");
        assert_eq!(user.billing_status, "active");
        assert_eq!(user.created_at, 1640000000);
        assert_eq!(user.updated_at, 1640001000);
    }

    #[test]
    fn test_user_with_null_avatar() {
        let json = r#"{
            "id": 123,
            "email": "user@example.com",
            "name": "testuser",
            "fullName": "Test User",
            "avatarUrl": null,
            "role": "member",
            "billingStatus": "active",
            "createdAt": 1640000000,
            "updatedAt": 1640001000
        }"#;

        let user: User = serde_json::from_str(json).unwrap();
        assert_eq!(user.avatar_url, None);
    }

    #[test]
    fn test_project_deserialization() {
        let json = r#"{
            "id": 456,
            "name": "project1",
            "fullName": "My Project",
            "purpose": "Testing purposes",
            "avatarUrl": null,
            "isClosed": false,
            "isPublic": true,
            "createdAt": 1640000000,
            "updatedAt": 1640001000
        }"#;

        let project: Project = serde_json::from_str(json).unwrap();
        assert_eq!(project.id, 456);
        assert_eq!(project.name, "project1");
        assert_eq!(project.full_name, "My Project");
        assert_eq!(project.purpose, Some("Testing purposes".to_string()));
        assert_eq!(project.is_closed, false);
        assert_eq!(project.is_public, true);
    }

    #[test]
    fn test_task_deserialization_full() {
        let json = r##"{
            "id": 789,
            "name": "Implement feature X",
            "description": "Add new feature",
            "status": {
                "id": 1,
                "name": "In Progress",
                "isClosed": false,
                "color": "#ff0000"
            },
            "priority": 2,
            "dueDate": 1640005000,
            "startDate": 1640000000,
            "responsibleUser": {
                "id": 123,
                "email": "user@example.com",
                "name": "testuser",
                "fullName": "Test User",
                "avatarUrl": null,
                "role": "member",
                "billingStatus": "active",
                "createdAt": 1640000000,
                "updatedAt": 1640001000
            },
            "ballHoldingUser": null,
            "tags": [
                {"id": 1, "name": "bug", "color": "#ff0000"},
                {"id": 2, "name": "urgent", "color": "#00ff00"}
            ],
            "project": {
                "id": 456,
                "name": "project1"
            },
            "milestone": {
                "id": 10,
                "name": "v1.0",
                "dueDate": 1650000000,
                "isClosed": false
            },
            "parent": 100,
            "sortOrder": 5,
            "createdAt": 1640000000,
            "updatedAt": 1640002000
        }"##;

        let task: Task = serde_json::from_str(json).unwrap();
        assert_eq!(task.id, 789);
        assert_eq!(task.name, "Implement feature X");
        assert_eq!(task.description, Some("Add new feature".to_string()));
        assert_eq!(task.status.name, "In Progress");
        assert_eq!(task.priority, 2);
        assert_eq!(task.due_date, Some(1640005000));
        assert_eq!(task.start_date, Some(1640000000));
        assert!(task.responsible_user.is_some());
        assert_eq!(task.responsible_user.unwrap().id, 123);
        assert_eq!(task.tags.len(), 2);
        assert_eq!(task.tags[0].name, "bug");
        assert_eq!(task.project.id, 456);
        assert!(task.milestone.is_some());
        assert_eq!(task.parent, Some(100));
        assert_eq!(task.sort_order, 5);
    }

    #[test]
    fn test_task_deserialization_minimal() {
        let json = r#"{
            "id": 789,
            "name": "Simple task",
            "description": null,
            "status": {
                "id": 1,
                "name": "Open",
                "isClosed": false,
                "color": null
            },
            "priority": 0,
            "dueDate": null,
            "startDate": null,
            "responsibleUser": null,
            "ballHoldingUser": null,
            "tags": [],
            "project": {
                "id": 456,
                "name": "project1"
            },
            "milestone": null,
            "parent": null,
            "sortOrder": 0,
            "createdAt": 1640000000,
            "updatedAt": 1640000000
        }"#;

        let task: Task = serde_json::from_str(json).unwrap();
        assert_eq!(task.id, 789);
        assert_eq!(task.name, "Simple task");
        assert_eq!(task.description, None);
        assert_eq!(task.due_date, None);
        assert_eq!(task.responsible_user, None);
        assert_eq!(task.tags.len(), 0);
        assert_eq!(task.milestone, None);
        assert_eq!(task.parent, None);
    }

    #[test]
    fn test_note_deserialization() {
        let json = r##"{
            "id": 999,
            "name": "Meeting notes",
            "description": "Notes from the meeting",
            "tags": [{"id": 3, "name": "notes", "color": "#0000ff"}],
            "parent": null,
            "project": {"id": 456, "name": "project1"},
            "sortOrder": 1,
            "createdAt": 1640000000,
            "updatedAt": 1640001000
        }"##;

        let note: Note = serde_json::from_str(json).unwrap();
        assert_eq!(note.id, 999);
        assert_eq!(note.name, "Meeting notes");
        assert_eq!(note.description, Some("Notes from the meeting".to_string()));
        assert_eq!(note.tags.len(), 1);
        assert_eq!(note.parent, None);
    }

    #[test]
    fn test_status_deserialization() {
        let json = r##"{
            "id": 1,
            "name": "Done",
            "isClosed": true,
            "color": "#00ff00"
        }"##;

        let status: Status = serde_json::from_str(json).unwrap();
        assert_eq!(status.id, 1);
        assert_eq!(status.name, "Done");
        assert_eq!(status.is_closed, true);
        assert_eq!(status.color, Some("#00ff00".to_string()));
    }

    #[test]
    fn test_tag_serialization_roundtrip() {
        let tag = Tag {
            id: 42,
            name: "important".to_string(),
            color: "#ff0000".to_string(),
        };

        let json = serde_json::to_string(&tag).unwrap();
        let deserialized: Tag = serde_json::from_str(&json).unwrap();

        assert_eq!(tag.id, deserialized.id);
        assert_eq!(tag.name, deserialized.name);
        assert_eq!(tag.color, deserialized.color);
    }

    #[test]
    fn test_api_response_wrapper() {
        let json = r#"{
            "requestedBy": 123,
            "user": {
                "id": 123,
                "email": "user@example.com",
                "name": "testuser",
                "fullName": "Test User",
                "avatarUrl": null,
                "role": "member",
                "billingStatus": "active",
                "createdAt": 1640000000,
                "updatedAt": 1640001000
            }
        }"#;

        let response: ApiResponse<UserData> = serde_json::from_str(json).unwrap();
        assert_eq!(response.requested_by, 123);
        assert_eq!(response.data.user.id, 123);
        assert_eq!(response.data.user.email, "user@example.com");
    }

    #[test]
    fn test_tasks_data_list() {
        let json = r#"{
            "tasks": [
                {
                    "id": 1,
                    "name": "Task 1",
                    "description": null,
                    "status": {"id": 1, "name": "Open", "isClosed": false, "color": null},
                    "priority": 0,
                    "dueDate": null,
                    "startDate": null,
                    "responsibleUser": null,
                    "ballHoldingUser": null,
                    "tags": [],
                    "project": {"id": 1, "name": "p1"},
                    "milestone": null,
                    "parent": null,
                    "sortOrder": 0,
                    "createdAt": 1640000000,
                    "updatedAt": 1640000000
                },
                {
                    "id": 2,
                    "name": "Task 2",
                    "description": null,
                    "status": {"id": 1, "name": "Open", "isClosed": false, "color": null},
                    "priority": 1,
                    "dueDate": null,
                    "startDate": null,
                    "responsibleUser": null,
                    "ballHoldingUser": null,
                    "tags": [],
                    "project": {"id": 1, "name": "p1"},
                    "milestone": null,
                    "parent": null,
                    "sortOrder": 1,
                    "createdAt": 1640000000,
                    "updatedAt": 1640000000
                }
            ]
        }"#;

        let tasks_data: TasksData = serde_json::from_str(json).unwrap();
        assert_eq!(tasks_data.tasks.len(), 2);
        assert_eq!(tasks_data.tasks[0].id, 1);
        assert_eq!(tasks_data.tasks[1].id, 2);
    }

    #[test]
    fn test_webhook_deserialization() {
        let json = r#"{
            "id": 555,
            "name": "Deploy webhook",
            "url": "https://example.com/webhook",
            "events": ["task.created", "task.updated"],
            "active": true
        }"#;

        let webhook: Webhook = serde_json::from_str(json).unwrap();
        assert_eq!(webhook.id, 555);
        assert_eq!(webhook.name, "Deploy webhook");
        assert_eq!(webhook.url, "https://example.com/webhook");
        assert_eq!(webhook.events.len(), 2);
        assert_eq!(webhook.active, true);
    }

    #[test]
    fn test_file_deserialization() {
        let json = r#"{
            "id": 777,
            "hash": "abc123def456",
            "filename": "document.pdf",
            "size": 1024000,
            "type": "application/pdf"
        }"#;

        let file: File = serde_json::from_str(json).unwrap();
        assert_eq!(file.id, 777);
        assert_eq!(file.hash, "abc123def456");
        assert_eq!(file.filename, "document.pdf");
        assert_eq!(file.size, 1024000);
        assert_eq!(file.file_type, "application/pdf");
    }

    #[test]
    fn test_inbox_item_with_task() {
        let json = r#"{
            "id": 888,
            "task": {
                "id": 789,
                "name": "Task in inbox",
                "description": null,
                "status": {"id": 1, "name": "Open", "isClosed": false, "color": null},
                "priority": 0,
                "dueDate": null,
                "startDate": null,
                "responsibleUser": null,
                "ballHoldingUser": null,
                "tags": [],
                "project": {"id": 1, "name": "p1"},
                "milestone": null,
                "parent": null,
                "sortOrder": 0,
                "createdAt": 1640000000,
                "updatedAt": 1640000000
            },
            "note": null,
            "comment": null,
            "readAt": null,
            "createdAt": 1640000000
        }"#;

        let inbox_item: InboxItem = serde_json::from_str(json).unwrap();
        assert_eq!(inbox_item.id, 888);
        assert!(inbox_item.task.is_some());
        assert_eq!(inbox_item.task.unwrap().id, 789);
        assert!(inbox_item.note.is_none());
        assert!(inbox_item.comment.is_none());
        assert_eq!(inbox_item.read_at, None);
    }

    #[test]
    fn test_change_deserialization() {
        let json = r#"{
            "field": "status",
            "from": "Open",
            "to": "In Progress"
        }"#;

        let change: Change = serde_json::from_str(json).unwrap();
        assert_eq!(change.field, "status");
        assert_eq!(change.from, Some("Open".to_string()));
        assert_eq!(change.to, Some("In Progress".to_string()));
    }

    #[test]
    fn test_space_deserialization() {
        let json = r#"{
            "id": 111,
            "name": "myspace",
            "fullName": "My Workspace",
            "information": "Team workspace",
            "avatarUrl": "https://example.com/space.png",
            "status": "active",
            "createdAt": 1640000000,
            "updatedAt": 1640001000
        }"#;

        let space: Space = serde_json::from_str(json).unwrap();
        assert_eq!(space.id, 111);
        assert_eq!(space.name, "myspace");
        assert_eq!(space.full_name, "My Workspace");
        assert_eq!(space.information, Some("Team workspace".to_string()));
        assert_eq!(space.status, "active");
    }
}
