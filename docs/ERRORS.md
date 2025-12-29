# Error Catalog

This document lists common errors and their solutions.

## Configuration Errors

### Config file has insecure permissions
```
Error: Config file has insecure permissions (644).
Please run: chmod 600 ~/.config/rpsn/config.toml
```
**Solution**: The config file contains sensitive API tokens and must only be readable by the owner.
```bash
chmod 600 ~/.config/rpsn/config.toml
```

### Profile not found
```
Error: Profile 'production' not found in config
```
**Solution**: Add the profile to your config file or use an existing profile.
```bash
rpsn config add-profile production <space_id> <api_token>
```

## API Errors

### Authentication failed
```
Error: API error (401): Unauthorized
```
**Solution**: Your API token is invalid or expired. Update your config.
```bash
rpsn config update-api-token <new_token>
```

### Rate limit exceeded
```
Error: API error (429): Too Many Requests
```
**Solution**: Wait before making more requests. Repsona has rate limits per space.

### Resource not found
```
Error: API error (404): Not Found
```
**Solution**: The resource (project, task, etc.) doesn't exist or you don't have access.

### Space not found
```
Error: API error (404): Space not found
```
**Solution**: Verify your space_id in the config is correct.

## Network Errors

### Failed to connect
```
Error: Failed to send request: error trying to connect
```
**Solution**: Check your internet connection and verify Repsona is accessible.

### TLS error
```
Error: error trying to connect: TLS handshake failed
```
**Solution**: Your system's CA certificates may be outdated. Update your system.

## Command Errors

### Invalid tag format
```
Error: Unable to parse tag ID
```
**Solution**: Tags must be numeric IDs separated by commas.
```bash
# Correct
rpsn task create --tags "1,2,3"

# Incorrect
rpsn task create --tags "bug,urgent"
```

### Missing required argument
```
Error: The following required arguments were not provided: <project_id>
```
**Solution**: Provide all required arguments as shown in the help text.
```bash
rpsn task --help
```

## Troubleshooting

### Debug mode
Enable trace logging to see API requests and responses:
```bash
rpsn --trace <command>
```

### Dry run mode
Preview what would happen without making changes:
```bash
rpsn --dry-run <command>
```

### Get more error details
Run with `RUST_BACKTRACE=1` to see a full stack trace:
```bash
RUST_BACKTRACE=1 rpsn <command>
```

## Reporting Issues

If you encounter an error not listed here:

1. Check the [GitHub Issues](https://github.com/your-org/rpsn/issues)
2. Run with `--trace` and include the output in your report
3. Use `rpsn report` to generate a sanitized error report template
