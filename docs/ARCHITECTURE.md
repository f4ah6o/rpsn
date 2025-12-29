# Architecture

This document describes the architecture of the rpsn CLI tool for interacting with Repsona.

## Project Structure

```
src/
├── main.rs              # Entry point
├── cli.rs               # Command-line argument parsing
├── config.rs            # Configuration file management
├── error_report.rs      # Error reporting for GitHub issues
├── output.rs            # Output formatting (JSON/Human)
├── api/                 # API client and endpoints
│   ├── client.rs        # HTTP client for Repsona API
│   ├── types.rs         # Shared data types
│   └── endpoints/       # API endpoint modules
│       ├── me.rs        # User-related endpoints
│       ├── task.rs      # Task-related endpoints
│       └── ...
└── commands/            # Command handlers
    ├── mod.rs
    ├── task.rs          # Task commands
    ├── me.rs            # User commands
    ├── tag.rs           # Tag commands
    └── ...
```

## Module Overview

### CLI Layer (`cli.rs`)
- Uses `clap` for argument parsing
- Defines command structure using derive APIs
- Handles global flags (`--json`, `--dry-run`, `--trace`)

### Config Layer (`config.rs`)
- Manages configuration file (`~/.config/rpsn/config.toml`)
- Supports multiple profiles (spaces)
- Validates file permissions (Unix: 0600)

### API Layer (`api/`)
- **client.rs**: HTTP client with TLS, rate limit handling, token redaction
- **types.rs**: Shared types for API requests/responses
- **endpoints/**: API endpoint groupings by resource

### Commands Layer (`commands/`)
- Each file handles a specific command group
- Uses output formatting for consistent display

### Output Layer (`output.rs`)
- Formats output as JSON or human-readable
- Handles tables for lists, detailed views for single items

### Error Reporting (`error_report.rs`)
- Generates GitHub issue templates
- Redacts sensitive information (tokens, URLs, IDs)

## Data Flow

```
┌─────────┐     ┌──────────┐     ┌───────────┐     ┌─────────┐
│  CLI    │────▶│ Commands │────▶│ API Client│────▶│ Repsona │
│ (clap)  │     │ Handlers │     │ (reqwest) │     │   API   │
└─────────┘     └──────────┘     └───────────┘     └─────────┘
                     │
                     ▼
              ┌───────────┐
              │  Output   │
              │ Formatting│
              └───────────┘
```

## Security Considerations

1. **Config File Permissions**: Config files must be 0600 on Unix systems
2. **Token Redaction**: Tokens are masked in debug/dry-run output
3. **TLS**: HTTPS with rustls-tls is enforced for all API calls
4. **Sanitization**: Error reports have sensitive data redacted

## Property-Based Testing

The following modules use `proptest` for property-based testing:
- `config.rs`: Config serialization/deserialization
- `api/types.rs`: Type serialization
- `output.rs`: Output formatting
- `error_report.rs`: Sanitization logic
- `commands/tag.rs`: Tag parsing
