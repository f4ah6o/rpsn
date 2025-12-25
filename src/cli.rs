use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "rpsn")]
#[command(about = "Repsona Task Management CLI")]
#[command(version = "0.1.0")]
#[command(long_about = "rpsn provides a command-line interface for Repsona task management")]
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
    /// Utility commands
    #[command(subcommand)]
    Util(UtilCommands),

    /// Configuration management
    #[command(subcommand)]
    Config(ConfigCommands),

    /// Personal operations (me)
    #[command(subcommand)]
    Me(MeCommands),

    /// Project management
    #[command(subcommand)]
    Project(ProjectCommands),

    /// Task operations
    #[command(subcommand)]
    Task(TaskCommands),

    /// Note operations
    #[command(subcommand)]
    Note(NoteCommands),

    /// File operations
    #[command(subcommand)]
    File(FileCommands),

    /// Tag operations
    #[command(subcommand)]
    Tag(TagCommands),

    /// Inbox operations
    #[command(subcommand)]
    Inbox(InboxCommands),

    /// Space operations
    #[command(subcommand)]
    Space(SpaceCommands),

    /// User operations
    #[command(subcommand)]
    User(UserCommands),

    /// Webhook operations
    #[command(subcommand)]
    Webhook(WebhookCommands),

    /// ID Link operations
    #[command(subcommand)]
    Idlink(IdlinkCommands),

    /// Generate shell completion script
    Completion {
        /// Shell type
        #[arg(value_enum)]
        shell: Shell,
    },

    /// Generate agent skill file
    SkillGenerate {
        /// Output file path
        #[arg(long)]
        output: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum UtilCommands {
    /// Show version information
    Version,
    /// Show help
    Help,
    /// Ping the API
    Ping,
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Initialize configuration
    Init,
    /// Show current configuration
    Get,
    /// Set credentials
    Set {
        /// Space ID
        #[arg(long)]
        space: String,
        /// API Token
        #[arg(long)]
        token: String,
    },
    /// Set profile credentials
    SetProfile {
        /// Profile name
        name: String,
        /// Space ID
        #[arg(long)]
        space: String,
        /// API Token
        #[arg(long)]
        token: String,
    },
    /// Switch profile
    Use {
        /// Profile name
        name: String,
    },
    /// Show current user information
    Whoami,
}

#[derive(Subcommand)]
pub enum MeCommands {
    /// Get your user information
    Get,
    /// Update your profile
    Update {
        /// Display name
        #[arg(long)]
        name: Option<String>,
        /// Full name
        #[arg(long)]
        full_name: Option<String>,
        /// Status message
        #[arg(long)]
        what_are_you_doing: Option<String>,
    },
    /// List your tasks
    Tasks,
    /// List tasks you're responsible for
    TasksResponsible,
    /// List tasks you're holding the ball for
    TasksBallHolding,
    /// List tasks you're following
    TasksFollowing,
    /// Get your task count
    TasksCount,
    /// List your projects
    Projects,
    /// Get your activity log
    Activity,
}

#[derive(Subcommand)]
pub enum ProjectCommands {
    /// List all projects
    List,
    /// Get project details
    Get {
        /// Project ID
        project_id: u64,
    },
    /// Create a new project
    Create {
        /// Project name
        #[arg(long)]
        name: String,
        /// Full name
        #[arg(long)]
        full_name: Option<String>,
        /// Project purpose
        #[arg(long)]
        purpose: Option<String>,
    },
    /// Update project
    Update {
        /// Project ID
        project_id: u64,
        /// Project name
        #[arg(long)]
        name: Option<String>,
        /// Project purpose
        #[arg(long)]
        purpose: Option<String>,
    },
    /// List project members
    MembersList {
        /// Project ID
        project_id: u64,
    },
    /// Add member to project
    MembersAdd {
        /// Project ID
        project_id: u64,
        /// User ID
        #[arg(long)]
        user: u64,
    },
    /// Remove member from project
    MembersRemove {
        /// Project ID
        project_id: u64,
        /// User ID
        #[arg(long)]
        user: u64,
    },
    /// Get project activity
    Activity {
        /// Project ID
        project_id: u64,
    },
    /// List project statuses
    StatusList {
        /// Project ID
        project_id: u64,
    },
    /// List project milestones
    MilestoneList {
        /// Project ID
        project_id: u64,
    },
}

#[derive(Subcommand)]
pub enum TaskCommands {
    /// List tasks in project
    List {
        /// Project ID
        project_id: u64,
    },
    /// Get task details
    Get {
        /// Project ID
        project_id: u64,
        /// Task ID
        task_id: u64,
    },
    /// Create a task
    Create {
        /// Project ID
        project_id: u64,
        /// Task title
        #[arg(long)]
        title: String,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// Status ID
        #[arg(long)]
        status: Option<u64>,
        /// Priority
        #[arg(long)]
        priority: Option<u32>,
        /// Due date (timestamp)
        #[arg(long)]
        due: Option<u64>,
        /// Assignee (user ID)
        #[arg(long)]
        assignee: Option<u64>,
        /// Comma-separated tag IDs
        #[arg(long)]
        tags: Option<String>,
    },
    /// Update task
    Update {
        /// Project ID
        project_id: u64,
        /// Task ID
        task_id: u64,
        /// Task title
        #[arg(long)]
        title: Option<String>,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// Status ID
        #[arg(long)]
        status: Option<u64>,
        /// Priority
        #[arg(long)]
        priority: Option<u32>,
        /// Due date (timestamp)
        #[arg(long)]
        due: Option<u64>,
        /// Assignee (user ID)
        #[arg(long)]
        assignee: Option<u64>,
        /// Comma-separated tag IDs
        #[arg(long)]
        tags: Option<String>,
    },
    /// Mark task as done
    Done {
        /// Project ID
        project_id: u64,
        /// Task ID
        task_id: u64,
    },
    /// Reopen task
    Reopen {
        /// Project ID
        project_id: u64,
        /// Task ID
        task_id: u64,
    },
    /// List subtasks
    Children {
        /// Project ID
        project_id: u64,
        /// Task ID
        task_id: u64,
    },
    /// List task comments
    CommentList {
        /// Project ID
        project_id: u64,
        /// Task ID
        task_id: u64,
    },
    /// Add comment to task
    CommentAdd {
        /// Project ID
        project_id: u64,
        /// Task ID
        task_id: u64,
        /// Comment text
        #[arg(long)]
        comment: String,
        /// Reply to comment ID
        #[arg(long)]
        reply_to: Option<u64>,
    },
    /// Get task activity
    Activity {
        /// Project ID
        project_id: u64,
        /// Task ID
        task_id: u64,
    },
    /// Get task history
    History {
        /// Project ID
        project_id: u64,
        /// Task ID
        task_id: u64,
    },
}

#[derive(Subcommand)]
pub enum NoteCommands {
    /// List notes in project
    List {
        /// Project ID
        project_id: u64,
    },
    /// Get note details
    Get {
        /// Project ID
        project_id: u64,
        /// Note ID
        note_id: u64,
    },
    /// Create a note
    Create {
        /// Project ID
        project_id: u64,
        /// Note name
        #[arg(long)]
        name: String,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// Parent note ID
        #[arg(long)]
        parent: Option<u64>,
        /// Comma-separated tag IDs
        #[arg(long)]
        tags: Option<String>,
        /// Add to bottom
        #[arg(long)]
        add_to_bottom: bool,
    },
    /// Update note
    Update {
        /// Project ID
        project_id: u64,
        /// Note ID
        note_id: u64,
        /// Note name
        #[arg(long)]
        name: Option<String>,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// Comma-separated tag IDs
        #[arg(long)]
        tags: Option<String>,
    },
    /// Delete note
    Delete {
        /// Project ID
        project_id: u64,
        /// Note ID
        note_id: u64,
    },
    /// List subnotes
    Children {
        /// Project ID
        project_id: u64,
        /// Note ID
        note_id: u64,
    },
    /// List note comments
    CommentList {
        /// Project ID
        project_id: u64,
        /// Note ID
        note_id: u64,
    },
    /// Add comment to note
    CommentAdd {
        /// Project ID
        project_id: u64,
        /// Note ID
        note_id: u64,
        /// Comment text
        #[arg(long)]
        comment: String,
    },
    /// Update note comment
    CommentUpdate {
        /// Project ID
        project_id: u64,
        /// Note ID
        note_id: u64,
        /// Comment ID
        comment_id: u64,
        /// Comment text
        #[arg(long)]
        comment: String,
    },
    /// Delete note comment
    CommentDelete {
        /// Project ID
        project_id: u64,
        /// Note ID
        note_id: u64,
        /// Comment ID
        comment_id: u64,
    },
    /// Get note activity
    Activity {
        /// Project ID
        project_id: u64,
        /// Note ID
        note_id: u64,
    },
    /// Get note history
    History {
        /// Project ID
        project_id: u64,
        /// Note ID
        note_id: u64,
    },
}

#[derive(Subcommand)]
pub enum FileCommands {
    /// Upload file to project
    Upload {
        /// Project ID
        project_id: u64,
        /// File path
        path: String,
    },
    /// Download file
    Download {
        /// File hash
        #[arg(long)]
        hash: String,
        /// Output path
        #[arg(long)]
        out: Option<String>,
    },
    /// Attach file
    Attach {
        /// Project ID
        project_id: u64,
        /// Model type (task, task_comment, note, note_comment)
        #[arg(long)]
        model: String,
        /// Model ID
        #[arg(long)]
        id: u64,
        /// File ID
        #[arg(long)]
        file: u64,
    },
    /// Detach file
    Detach {
        /// Project ID
        project_id: u64,
        /// Model type (task, task_comment, note, note_comment)
        #[arg(long)]
        model: String,
        /// Model ID
        #[arg(long)]
        id: u64,
        /// File ID
        #[arg(long)]
        file: u64,
    },
    /// Delete file
    Delete {
        /// File ID
        file_id: u64,
    },
}

#[derive(Subcommand)]
pub enum TagCommands {
    /// List all tags
    List,
}

#[derive(Subcommand)]
pub enum InboxCommands {
    /// List inbox items
    List,
    /// Update inbox item
    Update {
        /// Inbox ID
        inbox_id: u64,
    },
    /// Mark all inbox as read
    ReadAll,
    /// Get unread count
    UnreadCount,
}

#[derive(Subcommand)]
pub enum SpaceCommands {
    /// Get space information
    Get,
    /// Invite user to space
    Invite {
        /// Email address
        #[arg(long)]
        email: String,
        /// Role
        #[arg(long)]
        role: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum UserCommands {
    /// List all users
    List,
    /// Get user details
    Get {
        /// User ID
        user_id: u64,
    },
    /// Set user role
    RoleSet {
        /// User ID
        user_id: u64,
        /// Role
        #[arg(long)]
        role: String,
    },
    /// Set user payment type
    PaymentSet {
        /// User ID
        user_id: u64,
        /// Payment type
        #[arg(long)]
        r#type: String,
    },
    /// Get user activity
    Activity {
        /// User ID
        user_id: u64,
    },
}

#[derive(Subcommand)]
pub enum WebhookCommands {
    /// List webhooks
    List,
    /// Create webhook
    Create {
        /// Webhook name
        #[arg(long)]
        name: String,
        /// Webhook URL
        #[arg(long)]
        url: String,
        /// Comma-separated events
        #[arg(long)]
        events: String,
    },
    /// Update webhook
    Update {
        /// Webhook ID
        webhook_id: u64,
        /// Webhook name
        #[arg(long)]
        name: Option<String>,
        /// Webhook URL
        #[arg(long)]
        url: Option<String>,
        /// Comma-separated events
        #[arg(long)]
        events: Option<String>,
    },
    /// Delete webhook
    Delete {
        /// Webhook ID
        webhook_id: u64,
    },
}

#[derive(Subcommand)]
pub enum IdlinkCommands {
    /// List ID links
    List,
    /// Create ID link
    Create {
        /// Link name
        #[arg(long)]
        name: String,
        /// Link URL
        #[arg(long)]
        url: String,
    },
    /// Delete ID link
    Delete {
        /// ID Link ID
        idlink_id: u64,
    },
}

#[derive(ValueEnum, Clone, Copy)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    Elvish,
    Powershell,
}
