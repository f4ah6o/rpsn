use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "rpsn")]
#[command(about = "Repsona Task Management CLI - Manage tasks, projects, and notes from the command line")]
#[command(version = "0.1.0")]
#[command(long_about = r#"rpsn - Repsona Task Management CLI

A command-line interface for Repsona (https://repsona.com) that provides 1:1 mapping
to the Repsona REST API with human-friendly operations.

GETTING STARTED:
  1. Initialize configuration:  rpsn config init
  2. Set credentials:           rpsn config set --space <ID> --token <TOKEN>
  3. Verify connection:         rpsn config whoami

COMMON WORKFLOWS:
  List your tasks:              rpsn me tasks
  Create a task:                rpsn task create <PROJECT_ID> --title "Task title"
  Mark task as done:            rpsn task done <PROJECT_ID> <TASK_ID>
  List projects:                rpsn project list

For more information, see: https://github.com/your-org/rpsn"#)]
pub struct Cli {
    /// Repsona Space ID (overrides config)
    #[arg(long, env = "REPSONA_SPACE")]
    pub space: Option<String>,

    /// API Token (overrides config)
    #[arg(long, env = "REPSONA_TOKEN")]
    pub token: Option<String>,

    /// Config profile to use
    #[arg(long)]
    pub profile: Option<String>,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,

    /// Show request only, don't execute
    #[arg(long)]
    pub dry_run: bool,

    /// Skip confirmation prompts
    #[arg(long)]
    pub yes: bool,

    /// Show HTTP trace for debugging
    #[arg(long)]
    pub trace: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Utility commands (version, help, ping)
    #[command(subcommand)]
    Util(UtilCommands),

    /// Configuration management - Initialize, set credentials, manage profiles
    #[command(subcommand)]
    Config(ConfigCommands),

    /// Personal operations - Your tasks, projects, and activity
    #[command(subcommand)]
    Me(MeCommands),

    /// Project management - List, create, update projects and manage members
    #[command(subcommand)]
    Project(ProjectCommands),

    /// Task operations - Create, update, complete tasks and manage comments
    #[command(subcommand)]
    Task(TaskCommands),

    /// Note operations - Create, update, delete notes and manage comments
    #[command(subcommand)]
    Note(NoteCommands),

    /// File operations - Upload, download, attach/detach files
    #[command(subcommand)]
    File(FileCommands),

    /// Tag operations - List available tags
    #[command(subcommand)]
    Tag(TagCommands),

    /// Inbox operations - View notifications and mark as read
    #[command(subcommand)]
    Inbox(InboxCommands),

    /// Space operations - View space info and invite users
    #[command(subcommand)]
    Space(SpaceCommands),

    /// User operations - List users, manage roles and permissions
    #[command(subcommand)]
    User(UserCommands),

    /// Webhook operations - Create, update, delete webhooks for integrations
    #[command(subcommand)]
    Webhook(WebhookCommands),

    /// ID Link operations - Manage external ID link integrations
    #[command(subcommand)]
    Idlink(IdlinkCommands),

    /// Error reporting - Generate safe error reports for GitHub issues
    #[command(subcommand)]
    Report(ReportCommands),

    /// Generate shell completion script for bash, zsh, fish, etc.
    Completion {
        /// Shell type (bash, zsh, fish, elvish, powershell)
        #[arg(value_enum)]
        shell: Shell,
    },

    /// Generate agent skill file for AI assistants
    SkillGenerate {
        /// Output file path (default: ~/.config/rpsn/.claude/skills/rpsn/SKILL.md)
        #[arg(long)]
        output: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum UtilCommands {
    /// Show version information (rpsn version number)
    Version,
    /// Show detailed help information
    Help,
    /// Ping the API to verify connection and credentials
    Ping,
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Initialize configuration file (~/.config/rpsn/config.toml)
    Init,
    /// Show current configuration (space ID and profile)
    Get,
    /// Set credentials for the default profile
    Set {
        /// Repsona Space ID (found in your Repsona settings)
        #[arg(long)]
        space: String,
        /// API Token (generate from Repsona settings)
        #[arg(long)]
        token: String,
    },
    /// Create or update a named profile with credentials
    SetProfile {
        /// Profile name (e.g., "work", "personal")
        name: String,
        /// Repsona Space ID for this profile
        #[arg(long)]
        space: String,
        /// API Token for this profile
        #[arg(long)]
        token: String,
    },
    /// Switch to a different profile
    Use {
        /// Profile name to switch to
        name: String,
    },
    /// Show current user information (verify credentials)
    Whoami,
}

#[derive(Subcommand)]
pub enum MeCommands {
    /// Get your user information (name, email, role, etc.)
    Get,
    /// Update your profile information
    Update {
        /// Display name shown in the interface
        #[arg(long)]
        name: Option<String>,
        /// Your full name
        #[arg(long)]
        full_name: Option<String>,
        /// Status message ("What are you doing?")
        #[arg(long)]
        what_are_you_doing: Option<String>,
    },
    /// List all tasks assigned to you
    Tasks,
    /// List tasks where you are the responsible person
    TasksResponsible,
    /// List tasks where you are holding the ball (awaiting your action)
    TasksBallHolding,
    /// List tasks you are following for updates
    TasksFollowing,
    /// Get a count of your tasks by status
    TasksCount,
    /// List all projects you are a member of
    Projects,
    /// Get your recent activity log
    Activity,
}

#[derive(Subcommand)]
pub enum ProjectCommands {
    /// List all projects in the space
    List,
    /// Get detailed information about a project
    Get {
        /// Project ID (numeric)
        project_id: u64,
    },
    /// Create a new project in the space
    Create {
        /// Project name (short identifier)
        #[arg(long)]
        name: String,
        /// Full project name (display name)
        #[arg(long)]
        full_name: Option<String>,
        /// Project purpose or description
        #[arg(long)]
        purpose: Option<String>,
    },
    /// Update an existing project's information
    Update {
        /// Project ID to update
        project_id: u64,
        /// New project name
        #[arg(long)]
        name: Option<String>,
        /// New project purpose
        #[arg(long)]
        purpose: Option<String>,
    },
    /// List all members of a project
    MembersList {
        /// Project ID
        project_id: u64,
    },
    /// Add a user as a member of the project
    MembersAdd {
        /// Project ID
        project_id: u64,
        /// User ID to add (use 'user list' to find IDs)
        #[arg(long)]
        user: u64,
    },
    /// Remove a user from the project
    MembersRemove {
        /// Project ID
        project_id: u64,
        /// User ID to remove
        #[arg(long)]
        user: u64,
    },
    /// Get recent activity log for a project
    Activity {
        /// Project ID
        project_id: u64,
    },
    /// List available task statuses in a project
    StatusList {
        /// Project ID
        project_id: u64,
    },
    /// List milestones defined in a project
    MilestoneList {
        /// Project ID
        project_id: u64,
    },
}

#[derive(Subcommand)]
pub enum TaskCommands {
    /// List all tasks in a project
    List {
        /// Project ID containing the tasks
        project_id: u64,
    },
    /// Get detailed information about a specific task
    Get {
        /// Project ID
        project_id: u64,
        /// Task ID to retrieve
        task_id: u64,
    },
    /// Create a new task in a project
    Create {
        /// Project ID to create the task in
        project_id: u64,
        /// Task title (required)
        #[arg(long)]
        title: String,
        /// Task description (supports markdown)
        #[arg(long)]
        description: Option<String>,
        /// Status ID (use 'project status-list' to see available statuses)
        #[arg(long)]
        status: Option<u64>,
        /// Priority level (1-5, where 5 is highest)
        #[arg(long)]
        priority: Option<u32>,
        /// Due date as Unix timestamp
        #[arg(long)]
        due: Option<u64>,
        /// Assignee user ID (use 'user list' to find IDs)
        #[arg(long)]
        assignee: Option<u64>,
        /// Comma-separated tag IDs (e.g., "1,2,3")
        #[arg(long)]
        tags: Option<String>,
    },
    /// Update an existing task's properties
    Update {
        /// Project ID containing the task
        project_id: u64,
        /// Task ID to update
        task_id: u64,
        /// New task title
        #[arg(long)]
        title: Option<String>,
        /// New description
        #[arg(long)]
        description: Option<String>,
        /// New status ID
        #[arg(long)]
        status: Option<u64>,
        /// New priority level (1-5)
        #[arg(long)]
        priority: Option<u32>,
        /// New due date as Unix timestamp
        #[arg(long)]
        due: Option<u64>,
        /// New assignee user ID
        #[arg(long)]
        assignee: Option<u64>,
        /// New comma-separated tag IDs
        #[arg(long)]
        tags: Option<String>,
    },
    /// Mark a task as completed/done
    Done {
        /// Project ID
        project_id: u64,
        /// Task ID to mark as done
        task_id: u64,
    },
    /// Reopen a completed task
    Reopen {
        /// Project ID
        project_id: u64,
        /// Task ID to reopen
        task_id: u64,
    },
    /// List subtasks (child tasks) of a task
    Children {
        /// Project ID
        project_id: u64,
        /// Parent task ID
        task_id: u64,
    },
    /// List all comments on a task
    CommentList {
        /// Project ID
        project_id: u64,
        /// Task ID
        task_id: u64,
    },
    /// Add a comment to a task
    CommentAdd {
        /// Project ID
        project_id: u64,
        /// Task ID to comment on
        task_id: u64,
        /// Comment text (supports markdown)
        #[arg(long)]
        comment: String,
        /// Reply to an existing comment (comment ID)
        #[arg(long)]
        reply_to: Option<u64>,
    },
    /// Get activity log for a task
    Activity {
        /// Project ID
        project_id: u64,
        /// Task ID
        task_id: u64,
    },
    /// Get change history for a task
    History {
        /// Project ID
        project_id: u64,
        /// Task ID
        task_id: u64,
    },
}

#[derive(Subcommand)]
pub enum NoteCommands {
    /// List all notes in a project
    List {
        /// Project ID containing the notes
        project_id: u64,
    },
    /// Get detailed information about a note
    Get {
        /// Project ID
        project_id: u64,
        /// Note ID to retrieve
        note_id: u64,
    },
    /// Create a new note in a project
    Create {
        /// Project ID to create the note in
        project_id: u64,
        /// Note name/title
        #[arg(long)]
        name: String,
        /// Note description/content (supports markdown)
        #[arg(long)]
        description: Option<String>,
        /// Parent note ID (to create a subnote)
        #[arg(long)]
        parent: Option<u64>,
        /// Comma-separated tag IDs (e.g., "1,2,3")
        #[arg(long)]
        tags: Option<String>,
        /// Add note at the bottom of the list (default: top)
        #[arg(long)]
        add_to_bottom: bool,
    },
    /// Update an existing note
    Update {
        /// Project ID
        project_id: u64,
        /// Note ID to update
        note_id: u64,
        /// New note name
        #[arg(long)]
        name: Option<String>,
        /// New description
        #[arg(long)]
        description: Option<String>,
        /// New comma-separated tag IDs
        #[arg(long)]
        tags: Option<String>,
    },
    /// Delete a note (requires confirmation unless --yes is used)
    Delete {
        /// Project ID
        project_id: u64,
        /// Note ID to delete
        note_id: u64,
    },
    /// List subnotes (child notes) of a note
    Children {
        /// Project ID
        project_id: u64,
        /// Parent note ID
        note_id: u64,
    },
    /// List all comments on a note
    CommentList {
        /// Project ID
        project_id: u64,
        /// Note ID
        note_id: u64,
    },
    /// Add a comment to a note
    CommentAdd {
        /// Project ID
        project_id: u64,
        /// Note ID to comment on
        note_id: u64,
        /// Comment text (supports markdown)
        #[arg(long)]
        comment: String,
    },
    /// Update an existing comment on a note
    CommentUpdate {
        /// Project ID
        project_id: u64,
        /// Note ID
        note_id: u64,
        /// Comment ID to update
        comment_id: u64,
        /// New comment text
        #[arg(long)]
        comment: String,
    },
    /// Delete a comment from a note
    CommentDelete {
        /// Project ID
        project_id: u64,
        /// Note ID
        note_id: u64,
        /// Comment ID to delete
        comment_id: u64,
    },
    /// Get activity log for a note
    Activity {
        /// Project ID
        project_id: u64,
        /// Note ID
        note_id: u64,
    },
    /// Get change history for a note
    History {
        /// Project ID
        project_id: u64,
        /// Note ID
        note_id: u64,
    },
}

#[derive(Subcommand)]
pub enum FileCommands {
    /// Upload a file to a project
    Upload {
        /// Project ID to upload to
        project_id: u64,
        /// Local file path to upload
        path: String,
    },
    /// Download a file by its hash
    Download {
        /// File hash (obtained from upload or file list)
        #[arg(long)]
        hash: String,
        /// Output path (default: current directory with original filename)
        #[arg(long)]
        out: Option<String>,
    },
    /// Attach an uploaded file to a task, note, or comment
    Attach {
        /// Project ID
        project_id: u64,
        /// Model type: task, task_comment, note, or note_comment
        #[arg(long)]
        model: String,
        /// Model ID (task ID, note ID, or comment ID)
        #[arg(long)]
        id: u64,
        /// File ID to attach
        #[arg(long)]
        file: u64,
    },
    /// Detach a file from a task, note, or comment
    Detach {
        /// Project ID
        project_id: u64,
        /// Model type: task, task_comment, note, or note_comment
        #[arg(long)]
        model: String,
        /// Model ID (task ID, note ID, or comment ID)
        #[arg(long)]
        id: u64,
        /// File ID to detach
        #[arg(long)]
        file: u64,
    },
    /// Delete a file permanently
    Delete {
        /// File ID to delete
        file_id: u64,
    },
}

#[derive(Subcommand)]
pub enum TagCommands {
    /// List all tags available in the space
    List,
}

#[derive(Subcommand)]
pub enum InboxCommands {
    /// List all inbox notifications
    List,
    /// Mark an inbox item as read
    Update {
        /// Inbox item ID to mark as read
        inbox_id: u64,
    },
    /// Mark all inbox items as read
    ReadAll,
    /// Get count of unread inbox items
    UnreadCount,
}

#[derive(Subcommand)]
pub enum SpaceCommands {
    /// Get information about the current space
    Get,
    /// Invite a user to the space by email
    Invite {
        /// Email address to invite
        #[arg(long)]
        email: String,
        /// Role to assign (e.g., "member", "admin")
        #[arg(long)]
        role: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum UserCommands {
    /// List all users in the space
    List,
    /// Get detailed information about a user
    Get {
        /// User ID to retrieve
        user_id: u64,
    },
    /// Set a user's role in the space
    RoleSet {
        /// User ID to modify
        user_id: u64,
        /// New role (e.g., "member", "admin", "owner")
        #[arg(long)]
        role: String,
    },
    /// Set a user's payment/billing type
    PaymentSet {
        /// User ID to modify
        user_id: u64,
        /// Payment type (e.g., "paid", "free")
        #[arg(long)]
        r#type: String,
    },
    /// Get activity log for a specific user
    Activity {
        /// User ID
        user_id: u64,
    },
}

#[derive(Subcommand)]
pub enum WebhookCommands {
    /// List all configured webhooks
    List,
    /// Create a new webhook for event notifications
    Create {
        /// Webhook name for identification
        #[arg(long)]
        name: String,
        /// URL to receive webhook POST requests
        #[arg(long)]
        url: String,
        /// Comma-separated event types (e.g., "task.created,task.updated")
        #[arg(long)]
        events: String,
    },
    /// Update an existing webhook
    Update {
        /// Webhook ID to update
        webhook_id: u64,
        /// New webhook name
        #[arg(long)]
        name: Option<String>,
        /// New webhook URL
        #[arg(long)]
        url: Option<String>,
        /// New comma-separated event types
        #[arg(long)]
        events: Option<String>,
    },
    /// Delete a webhook
    Delete {
        /// Webhook ID to delete
        webhook_id: u64,
    },
}

#[derive(Subcommand)]
pub enum IdlinkCommands {
    /// List all ID link configurations
    List,
    /// Create a new ID link for external integrations
    Create {
        /// Link name/label
        #[arg(long)]
        name: String,
        /// URL pattern with {id} placeholder
        #[arg(long)]
        url: String,
    },
    /// Delete an ID link configuration
    Delete {
        /// ID Link ID to delete
        idlink_id: u64,
    },
}

#[derive(Subcommand)]
pub enum ReportCommands {
    /// Generate an error report from the last error (reads from stdin or file)
    Generate {
        /// Error message to report (or reads from stdin if not provided)
        #[arg(long)]
        error: Option<String>,
        /// Command that caused the error
        #[arg(long)]
        command: Option<String>,
        /// Output file path (prints to stdout if not provided)
        #[arg(long)]
        output: Option<String>,
    },
    /// Test error report generation with a sample error
    Test,
    /// Show information about what data is collected and excluded
    Info,
}

#[derive(ValueEnum, Clone, Copy)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    Elvish,
    Powershell,
}
