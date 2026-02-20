# rpsn - Repsona Task Management CLI
<!-- bdg:begin -->
[![crates.io](https://img.shields.io/crates/v/rpsn.svg)](https://crates.io/crates/rpsn)
[![license](https://img.shields.io/github/license/f4ah6o/rpsn.svg)](https://github.com/f4ah6o/rpsn)
[![CI](https://github.com/f4ah6o/rpsn/actions/workflows/duplicate-issue-detection.yaml/badge.svg)](https://github.com/f4ah6o/rpsn/actions/workflows/duplicate-issue-detection.yaml)
[![CI](https://github.com/f4ah6o/rpsn/actions/workflows/ci.yaml/badge.svg)](https://github.com/f4ah6o/rpsn/actions/workflows/ci.yaml)
[![CI](https://github.com/f4ah6o/rpsn/actions/workflows/publish.yaml/badge.svg)](https://github.com/f4ah6o/rpsn/actions/workflows/publish.yaml)
<!-- bdg:end -->

A command-line interface for [Repsona](https://repsona.com) task management system.

## Features

- **1:1 API Mapping** - Direct correspondence with Repsona REST API
- **Human-Friendly** - Intuitive commands: list, get, create, update, delete
- **Multiple Output Formats** - Human-readable tables or JSON for scripting
- **Profile Support** - Manage multiple workspaces with named profiles
- **Shell Completions** - Auto-complete for bash, zsh, fish, and more
- **Agent Skills** - Auto-generated skill file for AI assistants

## Installation

### From Source

```bash
git clone https://github.com/your-org/rpsn.git
cd rpsn
cargo build --release
cp target/release/rpsn ~/.local/bin/
```

### Requirements

- Rust 1.70 or later
- Repsona account with API access

## Quick Start

### 1. Initialize Configuration

```bash
rpsn config init
```

### 2. Set Your Credentials

```bash
rpsn config set --space YOUR_SPACE_ID --token YOUR_API_TOKEN
```

You can find your Space ID and API Token in Repsona's settings page.

### 3. Verify Connection

```bash
rpsn config whoami
```

## Commands

### Global Options

| Option | Description |
|--------|-------------|
| `--space <id>` | Override Repsona Space ID |
| `--token <token>` | Override API Token |
| `--profile <name>` | Use specific config profile |
| `--json` | Output as JSON |
| `--dry-run` | Show request only, don't execute |
| `--yes` | Skip confirmation prompts |
| `--trace` | Show HTTP trace for debugging |

### Utility Commands

```bash
rpsn util version      # Show version information
rpsn util help         # Show help
rpsn util ping         # Ping the API to verify connection
```

### Configuration

```bash
rpsn config init                                      # Initialize configuration
rpsn config get                                       # Show current configuration
rpsn config set --space <id> --token <token>          # Set credentials
rpsn config set-profile <name> --space <id> --token <token>  # Create named profile
rpsn config use <name>                                # Switch to profile
rpsn config whoami                                    # Show current user information
```

### Personal Operations (me)

```bash
rpsn me get                # Get your user information
rpsn me update             # Update your profile
rpsn me tasks              # List your tasks
rpsn me tasks-responsible  # List tasks you're responsible for
rpsn me tasks-ball-holding # List tasks you're holding the ball for
rpsn me tasks-following    # List tasks you're following
rpsn me tasks-count        # Get your task count
rpsn me projects           # List your projects
rpsn me activity           # Get your activity log
```

### Project Management

```bash
rpsn project list                                     # List all projects
rpsn project get <project_id>                         # Get project details
rpsn project create --name <name>                     # Create a new project
rpsn project update <project_id> --name <name>        # Update project
rpsn project delete <project_id>                      # Delete project
rpsn project members-list <project_id>                # List project members
rpsn project members-add <project_id> --user <id>     # Add member to project
rpsn project members-remove <project_id> --user <id>  # Remove member from project
rpsn project activity <project_id>                    # Get project activity
rpsn project status-list <project_id>                 # List project statuses
rpsn project milestone-list <project_id>              # List project milestones
```

### Task Operations

```bash
rpsn task list <project_id>                           # List tasks in project
rpsn task get <project_id> <task_id>                  # Get task details
rpsn task create <project_id> --title <title>         # Create a task
rpsn task update <project_id> <task_id> --title <t>   # Update task
rpsn task done <project_id> <task_id>                 # Mark task as done
rpsn task reopen <project_id> <task_id>               # Reopen task
rpsn task delete <project_id> <task_id>               # Delete task
rpsn task children <project_id> <task_id>             # List subtasks
rpsn task comment-list <project_id> <task_id>         # List task comments
rpsn task comment-add <project_id> <task_id> --comment <text>  # Add comment
rpsn task comment-update <project_id> <comment_id> --comment <text> # Update comment
rpsn task comment-delete <project_id> <comment_id>    # Delete comment
rpsn task activity <project_id> <task_id>             # Get task activity
rpsn task history <project_id> <task_id>              # Get task history
```

### Note Operations

```bash
rpsn note list <project_id>                           # List notes in project
rpsn note get <project_id> <note_id>                  # Get note details
rpsn note create <project_id> --name <name>           # Create a note
rpsn note update <project_id> <note_id> --name <n>    # Update note
rpsn note delete <project_id> <note_id>               # Delete note
rpsn note children <project_id> <note_id>             # List subnotes
rpsn note comment-list <project_id> <note_id>         # List note comments
rpsn note comment-add <project_id> <note_id> --comment <text>  # Add comment
rpsn note comment-update <project_id> <note_id> <comment_id> --comment <text>
rpsn note comment-delete <project_id> <note_id> <comment_id>
rpsn note activity <project_id> <note_id>             # Get note activity
rpsn note history <project_id> <note_id>              # Get note history
```

### File Operations

```bash
rpsn file upload <project_id> <path>                  # Upload file to project
rpsn file download --hash <hash> --out <path>         # Download file
rpsn file attach <project_id> --model task --id <id> --file <file_id>
rpsn file detach <project_id> --model task --id <id> --file <file_id>
rpsn file delete <file_id>                            # Delete file
```

### Other Operations

```bash
# Tags
rpsn tag list                                         # List all tags

# Inbox
rpsn inbox list                                       # List inbox items
rpsn inbox update <inbox_id>                          # Update inbox item
rpsn inbox read-all                                   # Mark all as read
rpsn inbox unread-count                               # Get unread count

# Space
rpsn space get                                        # Get space information
rpsn space invite --email <email>                     # Invite user to space

# Users
rpsn user list                                        # List all users
rpsn user get <user_id>                               # Get user details
rpsn user role-set <user_id> --role <role>            # Set user role
rpsn user payment-set <user_id> --type <type>         # Set payment type
rpsn user activity <user_id>                          # Get user activity

# Webhooks
rpsn webhook list                                     # List webhooks
rpsn webhook create --name <n> --url <u> --events <e> # Create webhook
rpsn webhook update <id> --name <name>                # Update webhook
rpsn webhook delete <id>                              # Delete webhook

# ID Links
rpsn idlink list                                      # List ID links
rpsn idlink create --name <name> --url <url>          # Create ID link
rpsn idlink delete <id>                               # Delete ID link
```

### Shell Completions

```bash
# Generate completion script
rpsn completion bash   # For Bash
rpsn completion zsh    # For Zsh
rpsn completion fish   # For Fish
rpsn completion elvish # For Elvish
rpsn completion powershell  # For PowerShell

# Install (Bash)
rpsn completion bash > ~/.local/share/bash-completion/completions/rpsn

# Install (Zsh)
rpsn completion zsh > ~/.zsh/completion/_rpsn

# Install (Fish)
rpsn completion fish > ~/.config/fish/completions/rpsn.fish
```

### Agent Skill Generation

Generate a skill file for AI assistants:

```bash
rpsn skill-generate                    # Generate to default location
rpsn skill-generate --output ./SKILL.md  # Generate to custom path
```

## Configuration

Configuration is stored in `~/.config/rpsn/config.toml`.

### Example Configuration

```toml
[default]
space_id = "your-space-id"
api_token = "your-api-token"

[work]
space_id = "work-space-id"
api_token = "work-api-token"

[personal]
space_id = "personal-space-id"
api_token = "personal-api-token"
```

### Environment Variables

| Variable | Description |
|----------|-------------|
| `REPSONA_SPACE` | Override Space ID |
| `REPSONA_TOKEN` | Override API Token |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | Enable OTLP trace export (for example `http://localhost:4317`) |
| `OTEL_SERVICE_NAME` | Override service name used in traces (default: `rpsn`) |
| `OTEL_TRACES_SAMPLER` | Optional sampler override (`always_on`, `always_off`, `traceidratio`, `parentbased_traceidratio`, etc.) |
| `OTEL_TRACES_SAMPLER_ARG` | Optional sampler argument (ratio for `traceidratio` samplers) |

If `OTEL_EXPORTER_OTLP_ENDPOINT` is not set, tracing stays disabled (no-op).

## Development Tasks (just)

Use `opz rpsn-dev -- ...` to expand test credentials and other environment variables.

```bash
just test               # Run default test suite
just test-live-api      # Run ignored live API tests (includes write operations)
just coverage           # Generate lcov at coverage/lcov.info
just coverage-live-api  # Generate lcov for ignored live API tests at coverage/lcov-live.info
just jaeger-up          # Start Jaeger all-in-one with OTLP ports
just jaeger-down        # Stop Jaeger
just trace-ping         # Run a sample traced command
just trace-ui           # Open Jaeger UI (http://localhost:16686)
```

### Local Tracing with Jaeger

```bash
just jaeger-up
just trace-ping
just trace-ui
```

In Jaeger UI, search for service `rpsn` (or your `OTEL_SERVICE_NAME`) and inspect the span tree.

### Live API Test Notes

- Live tests are marked `#[ignore]` and run only via `just test-live-api`
- Write tests create temporary resources and always attempt cleanup (delete)
- Project creation in live tests uses compatibility payload fallbacks for API schema differences
- Temporary resource names are intentionally short and safe to reduce server-side validation failures
- If project creation is blocked by free-plan limits, tests fall back to using an existing project
- Repsona API rate limits:
  - `read: 1000/min/space`
  - `write: 150/min/space`

## Examples

### Create a Task with Full Details

```bash
rpsn task create 123 \
  --title "Implement login feature" \
  --description "Add OAuth2 login support" \
  --priority 3 \
  --assignee 456 \
  --tags "feature,auth"
```

### List Tasks as JSON

```bash
rpsn --json task list 123
```

### Preview a Request (Dry Run)

```bash
rpsn --dry-run task create 123 --title "Test task"
```

### Debug API Calls

```bash
rpsn --trace task get 123 456
```

### Use a Different Profile

```bash
rpsn --profile work project list
```

## Error Handling

- Rate limits are automatically handled with exponential backoff
- Detailed error messages with suggestions for common issues
- Use `--trace` to debug API issues

## License

MIT License. See [LICENSE](LICENSE) for details.

## Related

- [Repsona](https://repsona.com) - Task management service
- [Repsona API Documentation](https://repsona.com/api/docs)

