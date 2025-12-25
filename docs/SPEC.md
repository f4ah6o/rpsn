# rpsn - Repsona Task Management CLI
# Specification Document
# ============================

## Overview
rpsn is a command-line interface for Repsona task management system, providing 1:1 mapping to the Repsona REST API with human-friendly operations.

## Design Principles
- 1:1 correspondence with Repsona REST API
- Human-focused operations (list/get/create/update)
- Explicit execution of dangerous operations
- No AI subcommands (current version)

## Project Structure

```
rpsn/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── cli.rs                    # Clap CLI structure
│   ├── config.rs                 # Config management
│   ├── output.rs                 # Output formatting
│   ├── completion.rs             # Shell completion generation
│   ├── skill_gen.rs              # Auto-generate skills from CLI
│   ├── api/
│   │   ├── mod.rs
│   │   ├── client.rs             # HTTP client with --trace support
│   │   ├── types.rs              # API response types
│   │   ├── error.rs              # API error handling
│   │   └── endpoints/
│   │       ├── mod.rs
│   │       ├── me.rs
│   │       ├── project.rs
│   │       ├── task.rs
│   │       ├── note.rs
│   │       ├── file.rs
│   │       ├── tag.rs
│   │       ├── inbox.rs
│   │       ├── space.rs
│   │       ├── user.rs
│   │       ├── webhook.rs
│   │       └── idlink.rs
│   └── commands/
│       ├── mod.rs
│       ├── util.rs
│       ├── config.rs
│       ├── me.rs
│       ├── project.rs
│       ├── task.rs
│       ├── note.rs
│       ├── file.rs
│       ├── tag.rs
│       ├── inbox.rs
│       ├── space.rs
│       ├── user.rs
│       └── webhook.rs
├── .claude/
│   └── skills/
│       └── rpsn/
│           └── SKILL.md          # Auto-generated
└── completions/
    ├── rpsn.bash
    ├── rpsn.zsh
    └── rpsn.fish
```

## Dependencies

```toml
[package]
name = "rpsn"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4.5", features = ["derive", "env"] }
reqwest = { version = "0.12", features = ["json", "multipart"] }
tokio = { version = "1.42", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"
dirs = "5.0"
anyhow = "1.0"
thiserror = "2.0"
chrono = { version = "0.4", features = ["serde"] }
comfy-table = "7.1"
clap_complete = "4.5"
```

## Phase 1: Foundation

### 1.1 Project Initialization
- Initialize Cargo project: `cargo new rpsn`
- Configure Cargo.toml with dependencies
- Set up workspace structure
- Initialize git repository

### 1.2 Configuration System (src/config.rs)

**Purpose:** Manage profiles and API credentials

**Components:**
```rust
pub struct Config {
    pub profiles: HashMap<String, Profile>,
    pub current_profile: String,
}

pub struct Profile {
    pub space_id: String,
    pub api_token: String,
}

impl Config {
    pub fn load() -> Result<Self>;
    pub fn save(&self) -> Result<()>;
    pub fn get_profile(&self, name: &str) -> Option<&Profile>;
    pub fn add_profile(&mut self, name: String, profile: Profile);
}
```

**Behavior:**
- Load from `~/.config/rpsn/config.toml`
- Create default config if not exists
- Respect `REPSONA_SPACE` and `REPSONA_TOKEN` environment variables
- Support multiple profiles
- Default profile: `default`

### 1.3 API Client (src/api/client.rs)

**Purpose:** HTTP client for Repsona REST API

**Components:**
```rust
pub struct RepsonaClient {
    space_id: String,
    api_token: String,
    dry_run: bool,
    trace: bool,
    client: reqwest::Client,
}

impl RepsonaClient {
    pub fn new(space_id: String, api_token: String, dry_run: bool, trace: bool) -> Self;

    pub async fn get<T>(&self, endpoint: &str) -> Result<T>;
    pub async fn post<T>(&self, endpoint: &str, body: impl Serialize) -> Result<T>;
    pub async fn patch<T>(&self, endpoint: &str, body: impl Serialize) -> Result<T>;
    pub async fn delete<T>(&self, endpoint: &str) -> Result<T>;
    pub async fn post_multipart<T>(&self, endpoint: &str, form: reqwest::multipart::Form) -> Result<T>;

    fn build_request(&self, method: Method, endpoint: &str) -> RequestBuilder;
    fn handle_rate_limits(&self, headers: &HeaderMap);
    fn log_trace(&self, method: Method, endpoint: &str, request: Option<&serde_json::Value>, response: &Response);
}
```

**Features:**
- Base URL: `https://<space_id>.repsona.com/api`
- Auth via `Authorization: Bearer <token>` header
- Rate limit handling (respect `RateLimit-Limit`, `RateLimit-Remaining`, `RateLimit-Reset` headers)
- Dry-run mode: log requests without executing
- Trace mode: log all HTTP requests/responses for debugging
- JSON and FormData support
- Automatic retry on 429 (Too Many Requests) with exponential backoff

### 1.4 API Types (src/api/types.rs)

**Purpose:** Type-safe API response structures

**Key Types:**
```rust
pub struct ApiResponse<T> {
    pub requestedBy: u64,
    #[serde(flatten)]
    pub data: T,
}

pub struct User {
    pub id: u64,
    pub email: String,
    pub name: String,
    pub fullName: String,
    pub avatarUrl: Option<String>,
    pub role: String,
    pub billingStatus: String,
    pub createdAt: u64,
    pub updatedAt: u64,
}

pub struct Project {
    pub id: u64,
    pub name: String,
    pub fullName: String,
    pub purpose: Option<String>,
    pub avatarUrl: Option<String>,
    pub isClosed: bool,
    pub isPublic: bool,
    pub createdAt: u64,
    pub updatedAt: u64,
}

pub struct Task {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub status: Status,
    pub priority: u32,
    pub dueDate: Option<u64>,
    pub startDate: Option<u64>,
    pub responsibleUser: Option<User>,
    pub ballHoldingUser: Option<User>,
    pub tags: Vec<Tag>,
    pub project: ProjectSummary,
    pub milestone: Option<Milestone>,
    pub parent: Option<u64>,
    pub sortOrder: u32,
    pub createdAt: u64,
    pub updatedAt: u64,
}

pub struct Note {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub tags: Vec<Tag>,
    pub parent: Option<u64>,
    pub project: ProjectSummary,
    pub sortOrder: u32,
    pub createdAt: u64,
    pub updatedAt: u64,
}

pub struct File {
    pub id: u64,
    pub hash: String,
    pub filename: String,
    pub size: u64,
    pub file_type: String,
}

pub struct Tag {
    pub id: u64,
    pub name: String,
    pub color: String,
}

pub struct InboxItem {
    pub id: u64,
    pub task: Option<Task>,
    pub note: Option<Note>,
    pub comment: Option<Comment>,
    pub readAt: Option<u64>,
    pub createdAt: u64,
}
```

### 1.5 CLI Structure (src/cli.rs)

**Purpose:** Define CLI interface using Clap derive macros

**Components:**
```rust
#[derive(Parser)]
#[command(name = "rpsn")]
#[command(about = "Repsona Task Management CLI")]
#[command(version)]
struct Cli {
    #[arg(long, env = "REPSONA_SPACE")]
    space: Option<String>,

    #[arg(long, env = "REPSONA_TOKEN")]
    token: Option<String>,

    #[arg(long)]
    profile: Option<String>,

    #[arg(long)]
    json: bool,

    #[arg(long)]
    dry_run: bool,

    #[arg(long)]
    yes: bool,

    #[arg(long)]
    trace: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Version,
    Help,
    Ping,
    Config(ConfigCommands),
    Me(MeCommands),
    Project(ProjectCommands),
    Task(TaskCommands),
    Note(NoteCommands),
    File(FileCommands),
    Tag(TagCommands),
    Inbox(InboxCommands),
    Space(SpaceCommands),
    User(UserCommands),
    Webhook(WebhookCommands),
}
```

### 1.6 Output Formatting (src/output.rs)

**Purpose:** Format output for human and machine consumption

**Components:**
```rust
pub enum OutputFormat {
    Human,
    Json,
}

pub fn print<T: Serialize>(data: &T, format: OutputFormat) -> Result<()>;
pub fn print_table<T: Display>(headers: &[&str], rows: &[Vec<T>]) -> Result<()>;
pub fn print_error(error: &anyhow::Error) -> Result<()>;
pub fn print_progress(message: &str);
pub fn print_success(message: &str);
```

**Features:**
- Human-readable tables via `comfy-table`
- JSON output mode for programmatic use
- Colored error messages
- Progress indicators for file uploads
- Success confirmations

## Phase 2: API Endpoint Implementations

### 2.1 me (src/api/endpoints/me.rs)

**Endpoints:**
```rust
impl RepsonaClient {
    pub async fn get_me(&self) -> Result<ApiResponse<User>>;
    pub async fn update_me(&self, updates: MeUpdateRequest) -> Result<ApiResponse<User>>;
    pub async fn get_me_tasks(&self, filter: TaskFilter) -> Result<ApiResponse<Vec<Task>>>;
    pub async fn get_me_tasks_responsible(&self, filter: TaskFilter) -> Result<ApiResponse<Vec<Task>>>;
    pub async fn get_me_tasks_ball_holding(&self, filter: TaskFilter) -> Result<ApiResponse<Vec<Task>>>;
    pub async fn get_me_tasks_following(&self, filter: TaskFilter) -> Result<ApiResponse<Vec<Task>>>;
    pub async fn get_me_tasks_count(&self) -> Result<ApiResponse<TaskCount>>;
    pub async fn get_me_projects(&self) -> Result<ApiResponse<Vec<Project>>>;
    pub async fn get_me_activity(&self) -> Result<ApiResponse<Vec<Activity>>>;
}
```

### 2.2 project (src/api/endpoints/project.rs)

**Endpoints:**
```rust
impl RepsonaClient {
    pub async fn list_projects(&self) -> Result<ApiResponse<Vec<Project>>>;
    pub async fn get_project(&self, project_id: u64) -> Result<ApiResponse<Project>>;
    pub async fn create_project(&self, request: CreateProjectRequest) -> Result<ApiResponse<Project>>;
    pub async fn update_project(&self, project_id: u64, request: UpdateProjectRequest) -> Result<ApiResponse<Project>>;
    pub async fn list_project_members(&self, project_id: u64) -> Result<ApiResponse<Vec<User>>>;
    pub async fn add_project_member(&self, project_id: u64, user_id: u64) -> Result<ApiResponse<Project>>;
    pub async fn remove_project_member(&self, project_id: u64, user_id: u64) -> Result<()>;
    pub async fn get_project_activity(&self, project_id: u64) -> Result<ApiResponse<Vec<Activity>>>;
    pub async fn list_project_statuses(&self, project_id: u64) -> Result<ApiResponse<Vec<Status>>>;
    pub async fn list_project_milestones(&self, project_id: u64) -> Result<ApiResponse<Vec<Milestone>>>;
}
```

### 2.3 task (src/api/endpoints/task.rs)

**Endpoints:**
```rust
impl RepsonaClient {
    pub async fn list_tasks(&self, project_id: u64, filter: TaskFilter) -> Result<ApiResponse<Vec<Task>>>;
    pub async fn get_task(&self, project_id: u64, task_id: u64) -> Result<ApiResponse<Task>>;
    pub async fn create_task(&self, project_id: u64, request: CreateTaskRequest) -> Result<ApiResponse<Task>>;
    pub async fn update_task(&self, project_id: u64, task_id: u64, request: UpdateTaskRequest) -> Result<ApiResponse<Task>>;
    pub async fn set_task_status(&self, project_id: u64, task_id: u64, status_id: u64) -> Result<ApiResponse<Task>>;
    pub async fn get_task_children(&self, project_id: u64, task_id: u64) -> Result<ApiResponse<Vec<Task>>>;
    pub async fn list_task_comments(&self, project_id: u64, task_id: u64) -> Result<ApiResponse<Vec<TaskComment>>>;
    pub async fn add_task_comment(&self, project_id: u64, task_id: u64, comment: String, reply_to: Option<u64>) -> Result<ApiResponse<TaskComment>>;
    pub async fn get_task_activity(&self, project_id: u64, task_id: u64) -> Result<ApiResponse<Vec<Activity>>>;
    pub async fn get_task_history(&self, project_id: u64, task_id: u64) -> Result<ApiResponse<Vec<History>>>;
}
```

### 2.4 note (src/api/endpoints/note.rs)

**Endpoints:**
```rust
impl RepsonaClient {
    pub async fn list_notes(&self, project_id: u64) -> Result<ApiResponse<Vec<Note>>>;
    pub async fn get_note(&self, project_id: u64, note_id: u64) -> Result<ApiResponse<Note>>;
    pub async fn create_note(&self, project_id: u64, request: CreateNoteRequest) -> Result<ApiResponse<Note>>;
    pub async fn update_note(&self, project_id: u64, note_id: u64, request: UpdateNoteRequest) -> Result<ApiResponse<Note>>;
    pub async fn delete_note(&self, project_id: u64, note_id: u64) -> Result<()>;
    pub async fn get_note_children(&self, project_id: u64, note_id: u64) -> Result<ApiResponse<Vec<Note>>>;
    pub async fn list_note_comments(&self, project_id: u64, note_id: u64) -> Result<ApiResponse<Vec<NoteComment>>>;
    pub async fn add_note_comment(&self, project_id: u64, note_id: u64, comment: String) -> Result<ApiResponse<NoteComment>>;
    pub async fn update_note_comment(&self, project_id: u64, note_id: u64, comment_id: u64, comment: String) -> Result<ApiResponse<NoteComment>>;
    pub async fn delete_note_comment(&self, project_id: u64, note_id: u64, comment_id: u64) -> Result<()>;
    pub async fn get_note_activity(&self, project_id: u64, note_id: u64) -> Result<ApiResponse<Vec<Activity>>>;
    pub async fn get_note_history(&self, project_id: u64, note_id: u64) -> Result<ApiResponse<Vec<History>>>;
}
```

### 2.5 file (src/api/endpoints/file.rs)

**Endpoints:**
```rust
impl RepsonaClient {
    pub async fn upload_file(&self, project_id: u64, file_path: &Path) -> Result<ApiResponse<Vec<File>>>;
    pub async fn download_file(&self, file_hash: &str, output_path: &Path) -> Result<()>;
    pub async fn attach_file(&self, project_id: u64, model: AttachModel, model_id: u64, file_id: u64) -> Result<()>;
    pub async fn detach_file(&self, project_id: u64, model: AttachModel, model_id: u64, file_id: u64) -> Result<()>;
    pub async fn delete_file(&self, file_id: u64) -> Result<()>;
}

pub enum AttachModel {
    Task,
    TaskComment,
    Note,
    NoteComment,
}
```

### 2.6 tag (src/api/endpoints/tag.rs)

**Endpoints:**
```rust
impl RepsonaClient {
    pub async fn list_tags(&self) -> Result<ApiResponse<Vec<Tag>>>;
}
```

### 2.7 inbox (src/api/endpoints/inbox.rs)

**Endpoints:**
```rust
impl RepsonaClient {
    pub async fn list_inbox(&self) -> Result<ApiResponse<Vec<InboxItem>>>;
    pub async fn update_inbox(&self, inbox_id: u64, read: bool) -> Result<ApiResponse<InboxItem>>;
    pub async fn mark_inbox_all_read(&self) -> Result<()>;
    pub async fn get_inbox_unread_count(&self) -> Result<ApiResponse<u64>>;
}
```

### 2.8 space (src/api/endpoints/space.rs)

**Endpoints:**
```rust
impl RepsonaClient {
    pub async fn get_space(&self) -> Result<ApiResponse<Space>>;
    pub async fn invite_to_space(&self, email: String, role: String) -> Result<ApiResponse<Invite>>;
}
```

### 2.9 user (src/api/endpoints/user.rs)

**Endpoints:**
```rust
impl RepsonaClient {
    pub async fn list_users(&self) -> Result<ApiResponse<Vec<User>>>;
    pub async fn get_user(&self, user_id: u64) -> Result<ApiResponse<User>>;
    pub async fn set_user_role(&self, user_id: u64, role: String) -> Result<ApiResponse<User>>;
    pub async fn set_user_payment(&self, user_id: u64, payment_type: String) -> Result<ApiResponse<User>>;
    pub async fn get_user_activity(&self, user_id: u64) -> Result<ApiResponse<Vec<Activity>>>;
}
```

### 2.10 webhook (src/api/endpoints/webhook.rs)

**Endpoints:**
```rust
impl RepsonaClient {
    pub async fn list_webhooks(&self) -> Result<ApiResponse<Vec<Webhook>>>;
    pub async fn create_webhook(&self, request: CreateWebhookRequest) -> Result<ApiResponse<Webhook>>;
    pub async fn update_webhook(&self, webhook_id: u64, request: UpdateWebhookRequest) -> Result<ApiResponse<Webhook>>;
    pub async fn delete_webhook(&self, webhook_id: u64) -> Result<()>;
}
```

### 2.11 idlink (src/api/endpoints/idlink.rs)

**Endpoints:**
```rust
impl RepsonaClient {
    pub async fn list_idlinks(&self) -> Result<ApiResponse<Vec<IdLink>>>;
    pub async fn create_idlink(&self, request: CreateIdLinkRequest) -> Result<ApiResponse<IdLink>>;
    pub async fn delete_idlink(&self, idlink_id: u64) -> Result<()>;
}
```

## Phase 3: Command Implementations

Each command module implements logic to:
1. Parse CLI arguments
2. Load configuration and initialize API client
3. Call appropriate API endpoint
4. Format and display output

### 3.1 util (src/commands/util.rs)

```rust
pub async fn handle_version() -> Result<()>;
pub async fn handle_help() -> Result<()>;
pub async fn handle_ping(client: &RepsonaClient) -> Result<()>;
```

### 3.2 config (src/commands/config.rs)

```rust
pub async fn handle_init() -> Result<()>;
pub async fn handle_get() -> Result<()>;
pub async fn handle_set(space_id: String, token: String) -> Result<()>;
pub async fn handle_set_profile(name: String, space_id: String, token: String) -> Result<()>;
pub async fn handle_use(name: String) -> Result<()>;
pub async fn handle_whoami(client: &RepsonaClient) -> Result<()>;
```

### 3.3 me (src/commands/me.rs)

```rust
pub async fn handle_get(client: &RepsonaClient, format: OutputFormat) -> Result<()>;
pub async fn handle_update(client: &RepsonaClient, updates: MeUpdateRequest) -> Result<()>;
pub async fn handle_tasks(client: &RepsonaClient, format: OutputFormat) -> Result<()>;
pub async fn handle_tasks_responsible(client: &RepsonaClient, format: OutputFormat) -> Result<()>;
pub async fn handle_tasks_ball_holding(client: &RepsonaClient, format: OutputFormat) -> Result<()>;
pub async fn handle_tasks_following(client: &RepsonaClient, format: OutputFormat) -> Result<()>;
pub async fn handle_tasks_count(client: &RepsonaClient, format: OutputFormat) -> Result<()>;
pub async fn handle_projects(client: &RepsonaClient, format: OutputFormat) -> Result<()>;
pub async fn handle_activity(client: &RepsonaClient, format: OutputFormat) -> Result<()>;
```

### 3.4 project (src/commands/project.rs)

```rust
pub async fn handle_list(client: &RepsonaClient, format: OutputFormat) -> Result<()>;
pub async fn handle_get(client: &RepsonaClient, project_id: u64, format: OutputFormat) -> Result<()>;
pub async fn handle_create(client: &RepsonaClient, request: CreateProjectRequest, format: OutputFormat) -> Result<()>;
pub async fn handle_update(client: &RepsonaClient, project_id: u64, request: UpdateProjectRequest) -> Result<()>;
pub async fn handle_members_list(client: &RepsonaClient, project_id: u64, format: OutputFormat) -> Result<()>;
pub async fn handle_members_add(client: &RepsonaClient, project_id: u64, user_id: u64, confirm: bool) -> Result<()>;
pub async fn handle_members_remove(client: &RepsonaClient, project_id: u64, user_id: u64, confirm: bool) -> Result<()>;
pub async fn handle_activity(client: &RepsonaClient, project_id: u64, format: OutputFormat) -> Result<()>;
pub async fn handle_status_list(client: &RepsonaClient, project_id: u64, format: OutputFormat) -> Result<()>;
pub async fn handle_milestone_list(client: &RepsonaClient, project_id: u64, format: OutputFormat) -> Result<()>;
```

### 3.5 task (src/commands/task.rs)

```rust
pub async fn handle_list(client: &RepsonaClient, project_id: u64, filter: TaskFilter, format: OutputFormat) -> Result<()>;
pub async fn handle_get(client: &RepsonaClient, project_id: u64, task_id: u64, format: OutputFormat) -> Result<()>;
pub async fn handle_create(client: &RepsonaClient, project_id: u64, request: CreateTaskRequest, format: OutputFormat) -> Result<()>;
pub async fn handle_update(client: &RepsonaClient, project_id: u64, task_id: u64, request: UpdateTaskRequest) -> Result<()>;
pub async fn handle_done(client: &RepsonaClient, project_id: u64, task_id: u64) -> Result<()>;
pub async fn handle_reopen(client: &RepsonaClient, project_id: u64, task_id: u64) -> Result<()>;
pub async fn handle_children(client: &RepsonaClient, project_id: u64, task_id: u64, format: OutputFormat) -> Result<()>;
pub async fn handle_comment_list(client: &RepsonaClient, project_id: u64, task_id: u64, format: OutputFormat) -> Result<()>;
pub async fn handle_comment_add(client: &RepsonaClient, project_id: u64, task_id: u64, comment: String, reply_to: Option<u64>) -> Result<()>;
pub async fn handle_activity(client: &RepsonaClient, project_id: u64, task_id: u64, format: OutputFormat) -> Result<()>;
pub async fn handle_history(client: &RepsonaClient, project_id: u64, task_id: u64, format: OutputFormat) -> Result<()>;
```

### 3.6 note (src/commands/note.rs)

```rust
pub async fn handle_list(client: &RepsonaClient, project_id: u64, format: OutputFormat) -> Result<()>;
pub async fn handle_get(client: &RepsonaClient, project_id: u64, note_id: u64, format: OutputFormat) -> Result<()>;
pub async fn handle_create(client: &RepsonaClient, project_id: u64, request: CreateNoteRequest, format: OutputFormat) -> Result<()>;
pub async fn handle_update(client: &RepsonaClient, project_id: u64, note_id: u64, request: UpdateNoteRequest) -> Result<()>;
pub async fn handle_delete(client: &RepsonaClient, project_id: u64, note_id: u64, confirm: bool) -> Result<()>;
pub async fn handle_children(client: &RepsonaClient, project_id: u64, note_id: u64, format: OutputFormat) -> Result<()>;
pub async fn handle_comment_list(client: &RepsonaClient, project_id: u64, note_id: u64, format: OutputFormat) -> Result<()>;
pub async fn handle_comment_add(client: &RepsonaClient, project_id: u64, note_id: u64, comment: String) -> Result<()>;
pub async fn handle_comment_update(client: &RepsonaClient, project_id: u64, note_id: u64, comment_id: u64, comment: String) -> Result<()>;
pub async fn handle_comment_delete(client: &RepsonaClient, project_id: u64, note_id: u64, comment_id: u64, confirm: bool) -> Result<()>;
pub async fn handle_activity(client: &RepsonaClient, project_id: u64, note_id: u64, format: OutputFormat) -> Result<()>;
pub async fn handle_history(client: &RepsonaClient, project_id: u64, note_id: u64, format: OutputFormat) -> Result<()>;
```

### 3.7 file (src/commands/file.rs)

```rust
pub async fn handle_upload(client: &RepsonaClient, project_id: u64, file_path: PathBuf, format: OutputFormat) -> Result<()>;
pub async fn handle_download(client: &RepsonaClient, file_hash: String, output_path: Option<PathBuf>) -> Result<()>;
pub async fn handle_attach(client: &RepsonaClient, project_id: u64, model: AttachModel, model_id: u64, file_id: u64) -> Result<()>;
pub async fn handle_detach(client: &RepsonaClient, project_id: u64, model: AttachModel, model_id: u64, file_id: u64) -> Result<()>;
pub async fn handle_delete(client: &RepsonaClient, file_id: u64, confirm: bool) -> Result<()>;
```

### 3.8 tag (src/commands/tag.rs)

```rust
pub async fn handle_list(client: &RepsonaClient, format: OutputFormat) -> Result<()>;
```

### 3.9 inbox (src/commands/inbox.rs)

```rust
pub async fn handle_list(client: &RepsonaClient, format: OutputFormat) -> Result<()>;
pub async fn handle_update(client: &RepsonaClient, inbox_id: u64) -> Result<()>;
pub async fn handle_read_all(client: &RepsonaClient) -> Result<()>;
pub async fn handle_unread_count(client: &RepsonaClient, format: OutputFormat) -> Result<()>;
```

### 3.10 space (src/commands/space.rs)

```rust
pub async fn handle_get(client: &RepsonaClient, format: OutputFormat) -> Result<()>;
pub async fn handle_invite(client: &RepsonaClient, email: String, role: String) -> Result<()>;
```

### 3.11 user (src/commands/user.rs)

```rust
pub async fn handle_list(client: &RepsonaClient, format: OutputFormat) -> Result<()>;
pub async fn handle_get(client: &RepsonaClient, user_id: u64, format: OutputFormat) -> Result<()>;
pub async fn handle_role_set(client: &RepsonaClient, user_id: u64, role: String) -> Result<()>;
pub async fn handle_payment_set(client: &RepsonaClient, user_id: u64, payment_type: String) -> Result<()>;
pub async fn handle_activity(client: &RepsonaClient, user_id: u64, format: OutputFormat) -> Result<()>;
```

### 3.12 webhook (src/commands/webhook.rs)

```rust
pub async fn handle_list(client: &RepsonaClient, format: OutputFormat) -> Result<()>;
pub async fn handle_create(client: &RepsonaClient, request: CreateWebhookRequest, format: OutputFormat) -> Result<()>;
pub async fn handle_update(client: &RepsonaClient, webhook_id: u64, request: UpdateWebhookRequest) -> Result<()>;
pub async fn handle_delete(client: &RepsonaClient, webhook_id: u64, confirm: bool) -> Result<()>;
```

### 3.13 idlink (src/commands/idlink.rs)

```rust
pub async fn handle_list(client: &RepsonaClient, format: OutputFormat) -> Result<()>;
pub async fn handle_create(client: &RepsonaClient, request: CreateIdLinkRequest, format: OutputFormat) -> Result<()>;
pub async fn handle_delete(client: &RepsonaClient, idlink_id: u64, confirm: bool) -> Result<()>;
```

## Phase 4: Auto-Generated Agent Skills

### 4.1 Skill Generator (src/skill_gen.rs)

**Purpose:** Generate `.claude/skills/rpsn/SKILL.md` from CLI structure

**Components:**
```rust
pub struct SkillGenerator {
    cli: Command,
}

impl SkillGenerator {
    pub fn from_cli(cli: Command) -> Self;
    pub fn generate(&self) -> Result<String>;

    fn extract_commands(&self) -> Vec<CommandInfo>;
    fn format_frontmatter(&self) -> String;
    fn format_categories(&self, commands: &[CommandInfo]) -> String;
    fn format_workflows(&self, commands: &[CommandInfo]) -> String;
    fn format_safety_notes(&self) -> String;
}

struct CommandInfo {
    category: String,
    name: String,
    description: String,
    usage: String,
    required_args: Vec<String>,
    optional_args: Vec<String>,
}
```

**Generated Skill Structure:**
```markdown
---
name: rpsn
description: Interact with Repsona task management via rpsn CLI
---

## Overview

rpsn provides a command-line interface for Repsona task management, mapping 1:1 to the Repsona REST API with human-friendly operations.

## Categories

### me - Personal Operations
`rpsn me get` - Get your user information
`rpsn me tasks` - List your tasks
`rpsn me tasks responsible` - List tasks you're responsible for
`rpsn me tasks ball-holding` - List tasks you're holding the ball for
`rpsn me tasks following` - List tasks you're following
`rpsn me projects` - List your participating projects
`rpsn me activity` - Get your activity log

### project - Project Management
`rpsn project list` - List all projects
`rpsn project get <projectId>` - Get project details
`rpsn project create --name <name> [--full-name <full>] [--purpose <text>]` - Create a project
`rpsn project update <projectId> [--name <name>] [--purpose <text>]` - Update project
`rpsn project members list <projectId>` - List project members
`rpsn project members add <projectId> --user <userId>` - Add member to project
`rpsn project members remove <projectId> --user <userId>` - Remove member from project
`rpsn project status list <projectId>` - List project statuses
`rpsn project milestone list <projectId>` - List project milestones

### task - Task Operations
`rpsn task list <projectId>` - List tasks in a project
`rpsn task get <projectId> <taskId>` - Get task details
`rpsn task create <projectId> --title <title> [--description <text>] [--status <status>] [--priority <p>] [--due <date>] [--assignee <userId>] [--tags <tag,tag>]` - Create a task
`rpsn task update <projectId> <taskId> [--title <title>] [--description <text>] [--status <status>] [--priority <p>] [--due <date>] [--assignee <userId>] [--tags <tag,tag>]` - Update task
`rpsn task done <projectId> <taskId>` - Mark task as done
`rpsn task reopen <projectId> <taskId>` - Reopen task
`rpsn task comment add <projectId> <taskId> --comment <text> [--reply-to <commentId>]` - Add comment to task

### note - Note Operations
`rpsn note list <projectId>` - List notes in a project
`rpsn note get <projectId> <noteId>` - Get note details
`rpsn note create <projectId> --name <name> [--description <text>] [--parent <noteId>] [--tags <tag,tag>]` - Create a note
`rpsn note delete <projectId> <noteId>` - Delete a note
`rpsn note comment add <projectId> <noteId> --comment <text>` - Add comment to note

### file - File Operations
`rpsn file upload <projectId> <path>` - Upload file to project
`rpsn file download --hash <fileHash> [--out <path>]` - Download file
`rpsn file attach <projectId> --model task|note --id <modelId> --file <fileId>` - Attach file to entity
`rpsn file detach <projectId> --model task|note --id <modelId> --file <fileId>` - Detach file from entity

### Other Categories
`rpsn tag list` - List all tags
`rpsn inbox list` - List inbox items
`rpsn space get` - Get space information
`rpsn user list` - List users
`rpsn user get <userId>` - Get user details

## Common Workflows

### Create a New Task
```bash
# 1. Find the project
rpsn project list

# 2. Create the task
rpsn task create <projectId> --title "Task title" --description "Description"

# 3. Verify creation
rpsn task get <projectId> <taskId>
```

### Project Member Management
```bash
# List users to find user ID
rpsn user list

# Add user to project
rpsn project members add <projectId> --user <userId>

# Verify addition
rpsn project members list <projectId>
```

### Task Lifecycle
```bash
# Create task
rpsn task create <projectId> --title "New task"

# Update status
rpsn task update <projectId> <taskId> --status "in-progress"

# Mark as done
rpsn task done <projectId> <taskId>

# Reopen if needed
rpsn task reopen <projectId> <taskId>
```

### File Management
```bash
# Upload file
rpsn file upload <projectId> /path/to/file.png

# Get file ID from response, then attach to task
rpsn file attach <projectId> --model task --id <taskId> --file <fileId>
```

## Global Options

- `--space <space_id>` - Override Repsona Space ID
- `--token <api_key>` - Override API Token
- `--profile <name>` - Use specific config profile
- `--json` - Output as JSON
- `--dry-run` - Show request only, don't execute
- `--yes` - Skip confirmation prompts
- `--trace` - Show HTTP trace for debugging

## Safety Notes

- **Always use `--dry-run`** first to preview changes, especially for create/update operations
- **Destructive operations** (delete, remove) require `--yes` flag or confirmation prompt
- **Test with `--trace`** if you encounter issues to see full HTTP request/response details
- **Use `--json`** output for programmatic integration with other tools

## Configuration

```bash
# Initialize configuration
rpsn config init

# Set credentials
rpsn config set --space <space_id> --token <api_key>

# Create named profile
rpsn config set-profile work --space <space_id> --token <api_token>

# Switch profile
rpsn config use work

# Verify configuration
rpsn config whoami
```
```

### 4.2 Skill Generation Command

Add to CLI:
```rust
#[derive(Subcommand)]
enum Commands {
    // ... other commands
    SkillGenerate {
        #[arg(long)]
        output: Option<PathBuf>,
    },
}
```

Usage:
```bash
rpsn skill-generate
rpsn skill-generate --output /custom/path/SKILL.md
```

### 4.3 Integration with Build

Optionally add to `build.rs` to auto-generate on build:
```rust
fn main() {
    // Auto-generate skill file during build
    // Only run if SKILL.md doesn't exist or CLI changed
}
```

## Phase 5: Shell Completions

### 5.1 Completion Generator (src/completion.rs)

**Purpose:** Generate shell completion scripts

**Components:**
```rust
pub struct CompletionGenerator;

impl CompletionGenerator {
    pub fn generate(shell: Shell, cmd: &Command, buf: &mut dyn Write) -> Result<()>;
    pub fn print_completion_instructions(shell: Shell) -> String;
}
```

**Supported Shells:**
- bash
- zsh
- fish
- elvish
- powershell

### 5.2 Integration

Add to CLI:
```rust
#[derive(Subcommand)]
enum Commands {
    // ... other commands
    Completion {
        shell: Shell,
    },
}

#[derive(ValueEnum, Clone, Copy)]
enum Shell {
    Bash,
    Zsh,
    Fish,
    Elvish,
    Powershell,
}
```

Usage:
```bash
# Generate completion for current shell
rpsn completion bash > ~/.local/share/bash-completion/completions/rpsn

# Or print to stdout
rpsn completion bash
```

### 5.3 Installation Instructions

Output included in `rpsn completion --help`:
```bash
# Bash
rpsn completion bash | sudo tee /usr/share/bash-completion/completions/rpsn

# Zsh
rpsn completion zsh > ~/.zsh/completion/_rpsn

# Fish
rpsn completion fish > ~/.config/fish/completions/rpsn.fish

# Powershell
rpsn completion powershell > rpsn.ps1
# Then add to profile: . rpsn.ps1
```

### 5.4 Dynamic Completions

Implement smart completions for:
- **Project IDs**: Cache from `rpsn project list`
- **User IDs**: Cache from `rpsn user list`
- **Task IDs**: Cache from project context
- **Tag names**: Cache from `rpsn tag list`
- **Status names**: Cache from project status list

## Phase 6: Testing

### 6.1 Unit Tests

**API Client Tests** (`src/api/client.rs` tests):
- Request building
- Header injection
- Rate limit handling
- Error parsing

**Config Tests** (`src/config.rs` tests):
- Loading from file
- Saving to file
- Profile management
- Environment variable overrides

**Type Tests** (`src/api/types.rs` tests):
- JSON deserialization
- Serialization
- Edge cases

### 6.2 Integration Tests

**Mock Server Setup:**
- Use `wiremock` or `httpmock` for mocking Repsona API
- Test all endpoint calls
- Test error scenarios (401, 404, 429, 500)

**Command Tests:**
- Test each command with mocked responses
- Test JSON vs human output formats
- Test `--dry-run` mode
- Test `--yes` flag behavior

### 6.3 Manual Testing Checklist

**Configuration:**
- [ ] Initialize config
- [ ] Set credentials
- [ ] Create profiles
- [ ] Switch profiles
- [ ] Environment variable overrides
- [ ] whoami verification

**Project Operations:**
- [ ] List projects
- [ ] Get project details
- [ ] Create project
- [ ] Update project
- [ ] List members
- [ ] Add member
- [ ] Remove member
- [ ] List statuses
- [ ] List milestones

**Task Operations:**
- [ ] List tasks (with filters)
- [ ] Get task details
- [ ] Create task
- [ ] Update task
- [ ] Mark as done
- [ ] Reopen task
- [ ] List subtasks
- [ ] Add comment
- [ ] View activity log
- [ ] View history

**Note Operations:**
- [ ] List notes
- [ ] Get note details
- [ ] Create note
- [ ] Update note
- [ ] Delete note
- [ ] List subnotes
- [ ] Add comment
- [ ] Delete comment
- [ ] View activity log
- [ ] View history

**File Operations:**
- [ ] Upload file
- [ ] Download file
- [ ] Attach to task
- [ ] Attach to note
- [ ] Detach from task
- [ ] Detach from note
- [ ] Delete file

**Other:**
- [ ] List tags
- [ ] Inbox operations
- [ ] Space info
- [ ] User operations
- [ ] Webhook operations
- [ ] Idlink operations

**Global Features:**
- [ ] JSON output mode
- [ ] Dry-run mode
- [ ] Trace mode
- [ ] Confirmation prompts
- [ ] Error messages
- [ ] Rate limit handling

## Implementation Order

### Stage 1: Foundation (Priority: High)
1. Initialize Cargo project
2. Add dependencies to Cargo.toml
3. Create project structure (directories)
4. Implement `src/config.rs` - Configuration system
5. Implement `src/api/types.rs` - API response types
6. Implement `src/api/client.rs` - HTTP client base
7. Implement `src/output.rs` - Output formatting
8. Implement `src/cli.rs` - Basic CLI structure with utility commands

### Stage 2: Core Operations (Priority: High)
9. Implement `src/commands/util.rs` - version, help, ping
10. Implement `src/commands/config.rs` - config management commands
11. Implement `src/api/endpoints/me.rs` - me endpoints
12. Implement `src/commands/me.rs` - me commands
13. Implement `src/api/endpoints/project.rs` - project endpoints
14. Implement `src/commands/project.rs` - project commands

### Stage 3: Task & Note (Priority: High)
15. Implement `src/api/endpoints/task.rs` - task endpoints
16. Implement `src/commands/task.rs` - task commands
17. Implement `src/api/endpoints/note.rs` - note endpoints
18. Implement `src/commands/note.rs` - note commands

### Stage 4: File & Other (Priority: Medium)
19. Implement `src/api/endpoints/file.rs` - file endpoints
20. Implement `src/commands/file.rs` - file commands
21. Implement `src/api/endpoints/tag.rs` - tag endpoints
22. Implement `src/commands/tag.rs` - tag commands
23. Implement `src/api/endpoints/inbox.rs` - inbox endpoints
24. Implement `src/commands/inbox.rs` - inbox commands

### Stage 5: Advanced Features (Priority: Medium)
25. Implement `src/api/endpoints/space.rs` - space endpoints
26. Implement `src/api/endpoints/user.rs` - user endpoints
27. Implement `src/commands/space.rs` - space commands
28. Implement `src/commands/user.rs` - user commands
29. Implement `src/api/endpoints/webhook.rs` - webhook endpoints
30. Implement `src/commands/webhook.rs` - webhook commands
31. Implement `src/api/endpoints/idlink.rs` - idlink endpoints
32. Implement `src/commands/idlink.rs` - idlink commands

### Stage 6: Agent Skills (Priority: High)
33. Implement `src/skill_gen.rs` - Skill generator
34. Add `skill-generate` command to CLI
35. Test skill generation
36. Generate initial `SKILL.md`
37. Document skill usage

### Stage 7: Shell Completions (Priority: Medium)
38. Implement `src/completion.rs` - Completion generator
39. Add `completion` command to CLI
40. Generate completion scripts for all shells
41. Test completions
42. Document installation

### Stage 8: Testing & Documentation (Priority: High)
43. Write unit tests for config, types, client
44. Write integration tests with mock server
45. Manual testing - Configuration
46. Manual testing - Project operations
47. Manual testing - Task operations
48. Manual testing - Note operations
49. Manual testing - File operations
50. Manual testing - All other operations
51. Manual testing - Global features
52. Write README.md
53. Add usage examples
54. Add troubleshooting guide

### Stage 9: Polish (Priority: Low)
55. Add caching for dynamic completions
56. Improve error messages
57. Add color support configuration
58. Add progress bars for long operations
59. Optimize performance
60. Release notes and changelog

## Success Criteria

- ✅ All commands from PLAN.md are implemented
- ✅ 1:1 mapping with Repsona REST API
- ✅ Configuration system with profiles working
- ✅ JSON and human-readable output modes
- ✅ Auto-generated agent skill file
- ✅ Shell completions for bash, zsh, fish
- ✅ Comprehensive error handling
- ✅ Rate limit handling
- ✅ Dry-run mode for all write operations
- ✅ Trace mode for debugging
- ✅ Unit tests passing
- ✅ Integration tests passing
- ✅ Manual testing checklist completed
- ✅ Documentation complete

---

*This specification serves as the blueprint for implementing rpsn CLI with agent skills support. Implementation should follow the order specified in the Implementation Order section.*
