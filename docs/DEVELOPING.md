# Development Guide

This guide covers setting up a development environment and contributing to rpsn.

## Prerequisites

- Rust 1.70+ (edition 2021)
- `uv` for Python tool management (if needed)

## Setup

```bash
# Clone the repository
git clone https://github.com/your-org/rpsn.git
cd rpsn

# Install dependencies (managed via Cargo)
cargo build
```

## Development Workflow

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_task_status_done_id

# Run property-based tests (proptest)
cargo test prop_
```

### Formatting

```bash
# Format code
cargo fmt

# Check formatting without changes
cargo fmt -- --check
```

### Linting

```bash
# Run clippy
cargo clippy

# Run clippy with all targets
cargo clippy --all-targets

# Run clippy with pedantic lints
cargo clippy -- -W clippy::pedantic
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run the CLI
cargo run -- --help
```

## Configuration Files

- `rustfmt.toml`: Code formatting rules
- `clippy.toml`: Lint configuration
- `.github/workflows/ci.yaml`: CI pipeline

## Testing Strategy

### Unit Tests
- Test individual functions and modules
- Located in `#[cfg(test)]` modules within each file

### Property-Based Tests
- Use `proptest` for testing with randomized inputs
- Test invariants and properties that should always hold
- Located in the same test modules

### Integration Tests
- (TODO) Located in `tests/` directory
- Test end-to-end workflows

## Adding a New Command

1. Define the command in `cli.rs` using ` clap::Parser`
2. Create a handler in `commands/<name>.rs`
3. Add API endpoints in `api/endpoints/` if needed
4. Wire up the handler in `main.rs`
5. Add tests (unit and/or property-based)

## Adding a New API Endpoint

1. Define request/response types in `api/types.rs`
2. Add the endpoint function in `api/endpoints/<resource>.rs`
3. Implement the method in `RepsonaClient`
4. Add tests for serialization/deserialization

## Code Style Guidelines

- Use `cargo fmt` for formatting
- Address clippy warnings
- Add rustdoc comments for public APIs
- Use property-based testing for functions with inputs
- Use `anyhow::Result` for error handling

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create git tag: `git tag -a v<version> -m "v<version>"`
4. Push: `git push && git push --tags`
5. GitHub Actions will build and publish releases
