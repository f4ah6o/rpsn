use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse<T> {
    #[serde(rename = "requestedBy")]
    pub requested_by: u64,
    #[serde(flatten)]
    pub data: T,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
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
