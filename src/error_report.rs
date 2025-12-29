//! Error reporting module for generating safe GitHub issue reports.
//!
//! This module ensures that sensitive information (credentials, user data from Repsona)
//! is NEVER included in error reports.

use once_cell::sync::Lazy;
use regex_lite::Regex;
use serde::Serialize;
use std::collections::HashSet;

// Pre-compiled regex patterns for sanitization
// Using Lazy ensures these are compiled once at first use, never panicking after successful compilation
static URL_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"https://[a-zA-Z0-9_-]+\.repsona\.com[^\s]*")
        .expect("URL pattern regex is valid")
});
static BEARER_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"Bearer\s+\S+")
        .expect("Bearer pattern regex is valid")
});
static UUID_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}")
        .expect("UUID pattern regex is valid")
});
static BASE64_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[A-Za-z0-9+_=-]{32,}")
        .expect("Base64 pattern regex is valid")
});

/// Sensitive data registry for sanitization.
///
/// This struct maintains a list of sensitive strings that must never
/// appear in error reports.
#[derive(Debug, Clone, Default)]
pub struct SensitiveData {
    /// Set of sensitive strings to redact
    secrets: HashSet<String>,
}

impl SensitiveData {
    /// Create a new SensitiveData registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a sensitive value that should be redacted from all output.
    ///
    /// Empty strings and whitespace-only strings are ignored.
    pub fn register(&mut self, secret: impl Into<String>) {
        let s = secret.into();
        // Only register non-empty, non-whitespace strings
        if !s.trim().is_empty() {
            self.secrets.insert(s);
        }
    }

    /// Load sensitive data from environment variables and config.
    pub fn load_from_environment(&mut self) {
        // Register environment variable values
        if let Ok(token) = std::env::var("REPSONA_TOKEN") {
            self.register(token);
        }
        if let Ok(space) = std::env::var("REPSONA_SPACE") {
            self.register(space);
        }
    }

    /// Load sensitive data from a config profile.
    pub fn load_from_profile(&mut self, space_id: &str, api_token: &str) {
        self.register(space_id);
        self.register(api_token);
    }

    /// Check if a string contains any registered sensitive data.
    pub fn contains_sensitive(&self, text: &str) -> bool {
        self.secrets.iter().any(|secret| text.contains(secret))
    }

    /// Sanitize a string by replacing all sensitive data with "[REDACTED]".
    pub fn sanitize(&self, text: &str) -> String {
        let mut result = text.to_string();
        for secret in &self.secrets {
            if !secret.is_empty() {
                result = result.replace(secret, "[REDACTED]");
            }
        }
        result
    }

    /// Get the number of registered secrets.
    #[cfg(test)]
    pub fn secret_count(&self) -> usize {
        self.secrets.len()
    }
}

/// Categories of errors for reporting.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub enum ErrorCategory {
    /// Network connectivity issues
    Network,
    /// Authentication/authorization failures
    Authentication,
    /// API returned an error status code
    ApiError,
    /// Failed to parse API response
    ParseError,
    /// Configuration issues
    Configuration,
    /// File system operations failed
    FileSystem,
    /// Unknown/other errors
    Unknown,
}

impl ErrorCategory {
    /// Categorize an error based on its message.
    ///
    /// Note: This only examines the error structure, not sensitive content.
    pub fn from_error(error: &anyhow::Error) -> Self {
        let msg = error.to_string().to_lowercase();

        if msg.contains("failed to send request") || msg.contains("connection") {
            ErrorCategory::Network
        } else if msg.contains("401") || msg.contains("403") || msg.contains("unauthorized") {
            ErrorCategory::Authentication
        } else if msg.contains("api error") {
            ErrorCategory::ApiError
        } else if msg.contains("failed to parse") || msg.contains("deserialize") {
            ErrorCategory::ParseError
        } else if msg.contains("config") || msg.contains("profile") {
            ErrorCategory::Configuration
        } else if msg.contains("file") || msg.contains("directory") || msg.contains("permission") {
            ErrorCategory::FileSystem
        } else {
            ErrorCategory::Unknown
        }
    }
}

/// Safe error report that can be posted to GitHub issues.
///
/// This struct intentionally only contains non-sensitive information.
/// It does NOT contain:
/// - API tokens or credentials
/// - Space IDs
/// - Any data from Repsona (task names, notes, user info, etc.)
/// - Configuration file contents
/// - Environment variable values
#[derive(Debug, Clone, Serialize)]
pub struct ErrorReport {
    /// Application version
    pub version: String,
    /// Operating system information
    pub os: String,
    /// Architecture
    pub arch: String,
    /// Error category
    pub category: ErrorCategory,
    /// HTTP status code if applicable (None for non-HTTP errors)
    pub http_status: Option<u16>,
    /// Command that was being executed (without arguments)
    pub command: Option<String>,
    /// Sanitized error message
    pub error_message: String,
    /// Additional context (sanitized)
    pub context: Vec<String>,
}

impl ErrorReport {
    /// Create a new error report from an error.
    ///
    /// # Arguments
    /// - `error`: The error to report
    /// - `command`: The command being executed (arguments will be stripped)
    /// - `sensitive`: Registry of sensitive data to redact
    pub fn new(
        error: &anyhow::Error,
        command: Option<&str>,
        sensitive: &SensitiveData,
    ) -> Self {
        let category = ErrorCategory::from_error(error);

        // Extract HTTP status code if present
        let http_status = Self::extract_http_status(error);

        // Sanitize the error message
        let error_message = Self::sanitize_error_message(error, sensitive);

        // Extract command name only (no arguments)
        let command = command.map(|c| {
            c.split_whitespace()
                .next()
                .unwrap_or(c)
                .to_string()
        });

        ErrorReport {
            version: env!("CARGO_PKG_VERSION").to_string(),
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            category,
            http_status,
            command,
            error_message,
            context: Vec::new(),
        }
    }

    /// Extract HTTP status code from error message.
    fn extract_http_status(error: &anyhow::Error) -> Option<u16> {
        let msg = error.to_string();
        // Look for patterns like "API error (404)" or "status: 500"
        for word in msg.split(|c: char| !c.is_ascii_digit()) {
            if let Ok(code) = word.parse::<u16>() {
                if (100..=599).contains(&code) {
                    return Some(code);
                }
            }
        }
        None
    }

    /// Create a safe, sanitized error message.
    ///
    /// This removes any sensitive data and replaces specific details with generic placeholders.
    fn sanitize_error_message(error: &anyhow::Error, sensitive: &SensitiveData) -> String {
        let msg = error.to_string();

        // First, apply registered sensitive data redaction
        let sanitized = sensitive.sanitize(&msg);

        // Additional sanitization patterns for common sensitive data formats
        Self::sanitize_common_patterns(&sanitized)
    }

    /// Sanitize common patterns that might contain sensitive data.
    fn sanitize_common_patterns(text: &str) -> String {
        let mut result = text.to_string();

        // Redact URLs with potential space_id (https://xxx.repsona.com/...)
        // This replaces the entire URL to avoid path leakage
        result = URL_PATTERN.replace_all(&result, "https://[REDACTED].repsona.com/[PATH]").to_string();

        // Redact Bearer tokens
        result = BEARER_PATTERN.replace_all(&result, "Bearer [REDACTED]").to_string();

        // Redact potential API tokens (common formats: UUID, base64-like strings)
        result = UUID_PATTERN.replace_all(&result, "[REDACTED-UUID]").to_string();

        // Redact base64-like tokens (32+ chars, excluding slashes to avoid matching URL paths)
        // This catches typical API tokens like JWT segments, API keys, etc.
        result = BASE64_PATTERN.replace_all(&result, "[REDACTED-TOKEN]").to_string();

        result
    }

    /// Add sanitized context to the report.
    pub fn add_context(&mut self, context: &str, sensitive: &SensitiveData) {
        let sanitized = sensitive.sanitize(context);
        let sanitized = Self::sanitize_common_patterns(&sanitized);
        self.context.push(sanitized);
    }

    /// Format the error report as markdown for GitHub issues.
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();

        md.push_str("## Error Report\n\n");

        md.push_str("### Environment\n");
        md.push_str(&format!("- **Version**: {}\n", self.version));
        md.push_str(&format!("- **OS**: {}\n", self.os));
        md.push_str(&format!("- **Architecture**: {}\n", self.arch));
        md.push_str("\n");

        md.push_str("### Error Details\n");
        md.push_str(&format!("- **Category**: {:?}\n", self.category));
        if let Some(status) = self.http_status {
            md.push_str(&format!("- **HTTP Status**: {}\n", status));
        }
        if let Some(ref cmd) = self.command {
            md.push_str(&format!("- **Command**: `{}`\n", cmd));
        }
        md.push_str("\n");

        md.push_str("### Error Message\n");
        md.push_str("```\n");
        md.push_str(&self.error_message);
        md.push_str("\n```\n");

        if !self.context.is_empty() {
            md.push_str("\n### Additional Context\n");
            for ctx in &self.context {
                md.push_str(&format!("- {}\n", ctx));
            }
        }

        md
    }

    /// Verify that the report contains no sensitive data.
    ///
    /// Returns true if the report is safe to publish.
    pub fn verify_no_sensitive_data(&self, sensitive: &SensitiveData) -> bool {
        let markdown = self.to_markdown();
        !sensitive.contains_sensitive(&markdown)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // =========================================================================
    // Unit Tests
    // =========================================================================

    #[test]
    fn test_sensitive_data_register() {
        let mut sd = SensitiveData::new();
        sd.register("my-secret-token");
        sd.register("my-space-id");

        assert_eq!(sd.secret_count(), 2);
        assert!(sd.contains_sensitive("contains my-secret-token here"));
        assert!(sd.contains_sensitive("space: my-space-id"));
        assert!(!sd.contains_sensitive("no secrets here"));
    }

    #[test]
    fn test_sensitive_data_empty_string() {
        let mut sd = SensitiveData::new();
        sd.register("");
        sd.register("   ");

        assert_eq!(sd.secret_count(), 0);
    }

    #[test]
    fn test_sanitize() {
        let mut sd = SensitiveData::new();
        sd.register("secret123");
        sd.register("myspace");

        let input = "Error at https://myspace.repsona.com with token secret123";
        let output = sd.sanitize(input);

        assert!(!output.contains("secret123"));
        assert!(!output.contains("myspace"));
        assert!(output.contains("[REDACTED]"));
    }

    #[test]
    fn test_error_category_from_error() {
        let network_err = anyhow::anyhow!("Failed to send request: connection refused");
        assert_eq!(ErrorCategory::from_error(&network_err), ErrorCategory::Network);

        let auth_err = anyhow::anyhow!("API error (401): Unauthorized");
        assert_eq!(ErrorCategory::from_error(&auth_err), ErrorCategory::Authentication);

        let api_err = anyhow::anyhow!("API error (500): Internal server error");
        assert_eq!(ErrorCategory::from_error(&api_err), ErrorCategory::ApiError);

        let parse_err = anyhow::anyhow!("Failed to parse response");
        assert_eq!(ErrorCategory::from_error(&parse_err), ErrorCategory::ParseError);
    }

    #[test]
    fn test_error_report_new() {
        let mut sd = SensitiveData::new();
        sd.register("mytoken123");
        sd.register("myspace");

        let error = anyhow::anyhow!("API error (404): Not found at https://myspace.repsona.com");
        let report = ErrorReport::new(&error, Some("task list --project myproject"), &sd);

        assert_eq!(report.http_status, Some(404));
        assert_eq!(report.command, Some("task".to_string()));
        assert!(!report.error_message.contains("myspace"));
    }

    #[test]
    fn test_sanitize_common_patterns_uuid() {
        let text = "Error with id 550e8400-e29b-41d4-a716-446655440000";
        let sanitized = ErrorReport::sanitize_common_patterns(text);
        assert!(!sanitized.contains("550e8400"));
        assert!(sanitized.contains("[REDACTED-UUID]"));
    }

    #[test]
    fn test_sanitize_common_patterns_bearer() {
        let text = "Header: Bearer abc123secrettoken456";
        let sanitized = ErrorReport::sanitize_common_patterns(text);
        assert!(!sanitized.contains("abc123secrettoken456"));
        assert!(sanitized.contains("Bearer [REDACTED]"));
    }

    #[test]
    fn test_sanitize_common_patterns_url() {
        let text = "Error at https://mycompany.repsona.com/api/task";
        let sanitized = ErrorReport::sanitize_common_patterns(text);
        assert!(!sanitized.contains("mycompany"));
        assert!(!sanitized.contains("/api/task"));
        assert!(sanitized.contains("https://[REDACTED].repsona.com/[PATH]"));
    }

    #[test]
    fn test_verify_no_sensitive_data() {
        let mut sd = SensitiveData::new();
        sd.register("supersecret");

        let error = anyhow::anyhow!("Connection failed");
        let report = ErrorReport::new(&error, Some("task"), &sd);

        assert!(report.verify_no_sensitive_data(&sd));
    }

    #[test]
    fn test_to_markdown_format() {
        let sd = SensitiveData::new();
        let error = anyhow::anyhow!("API error (500): Server error");
        let mut report = ErrorReport::new(&error, Some("project list"), &sd);
        report.add_context("Retry count: 3", &sd);

        let md = report.to_markdown();

        assert!(md.contains("## Error Report"));
        assert!(md.contains("### Environment"));
        assert!(md.contains("### Error Details"));
        assert!(md.contains("**Category**: ApiError"));
        assert!(md.contains("**HTTP Status**: 500"));
        assert!(md.contains("**Command**: `project`"));
        assert!(md.contains("### Additional Context"));
        assert!(md.contains("Retry count: 3"));
    }

    // =========================================================================
    // Property-Based Tests
    // =========================================================================

    proptest! {
        /// Property: Registered secrets are ALWAYS removed from sanitized output.
        #[test]
        fn prop_secrets_always_redacted(
            secret in "[a-zA-Z0-9]{8,32}",
            prefix in ".*",
            suffix in ".*"
        ) {
            let mut sd = SensitiveData::new();
            sd.register(&secret);

            let input = format!("{}{}{}", prefix, secret, suffix);
            let output = sd.sanitize(&input);

            // The secret must not appear in the output
            prop_assert!(!output.contains(&secret),
                "Secret '{}' was found in sanitized output: '{}'", secret, output);
        }

        /// Property: Multiple secrets are ALL removed.
        #[test]
        fn prop_multiple_secrets_all_redacted(
            secrets in prop::collection::vec("[a-zA-Z0-9]{8,16}", 1..5),
            text_template in ".{0,100}"
        ) {
            let mut sd = SensitiveData::new();
            for secret in &secrets {
                sd.register(secret);
            }

            // Create text containing all secrets
            let mut input = text_template.clone();
            for secret in &secrets {
                input.push_str(secret);
                input.push(' ');
            }

            let output = sd.sanitize(&input);

            // All secrets must be removed
            for secret in &secrets {
                prop_assert!(!output.contains(secret),
                    "Secret '{}' was found in output: '{}'", secret, output);
            }
        }

        /// Property: UUID patterns are always redacted.
        #[test]
        fn prop_uuid_patterns_redacted(
            uuid_parts in (
                "[a-fA-F0-9]{8}",
                "[a-fA-F0-9]{4}",
                "[a-fA-F0-9]{4}",
                "[a-fA-F0-9]{4}",
                "[a-fA-F0-9]{12}"
            ),
            prefix in ".{0,20}",
            suffix in ".{0,20}"
        ) {
            let uuid = format!("{}-{}-{}-{}-{}",
                uuid_parts.0, uuid_parts.1, uuid_parts.2, uuid_parts.3, uuid_parts.4);
            let input = format!("{}{}{}", prefix, uuid, suffix);
            let output = ErrorReport::sanitize_common_patterns(&input);

            prop_assert!(!output.contains(&uuid),
                "UUID '{}' was found in output: '{}'", uuid, output);
            prop_assert!(output.contains("[REDACTED-UUID]") || !input.contains(&uuid));
        }

        /// Property: Repsona URLs have their hostname and path redacted.
        #[test]
        fn prop_repsona_urls_redacted(
            space_id in "[a-zA-Z0-9_-]{3,20}",
            path in "[a-zA-Z0-9]{2,}(/[a-zA-Z0-9]+)*"
        ) {
            let url = format!("https://{}.repsona.com/{}", space_id, path);
            let output = ErrorReport::sanitize_common_patterns(&url);

            // The original URL should be completely redacted
            let original_host = format!("https://{}.repsona.com", space_id);
            prop_assert!(!output.contains(&original_host),
                "Original hostname '{}' was found in output: '{}'", original_host, output);
            // Meaningful path segments should be redacted
            prop_assert!(!output.contains(&path),
                "Original path '{}' was found in output: '{}'", path, output);
            prop_assert!(output.contains("https://[REDACTED].repsona.com/[PATH]"),
                "Expected redacted URL not found in output: '{}'", output);
        }

        /// Property: Bearer tokens are always redacted.
        #[test]
        fn prop_bearer_tokens_redacted(
            token in "[a-zA-Z0-9+/=]{10,64}"
        ) {
            let input = format!("Authorization: Bearer {}", token);
            let output = ErrorReport::sanitize_common_patterns(&input);

            prop_assert!(!output.contains(&token),
                "Token '{}' was found in output: '{}'", token, output);
            prop_assert!(output.contains("Bearer [REDACTED]"));
        }

        /// Property: Error reports never contain registered sensitive data.
        #[test]
        fn prop_error_report_no_sensitive_data(
            api_token in "[a-zA-Z0-9]{16,32}",
            space_id in "[a-zA-Z0-9]{4,12}",
            error_text in ".{0,100}"
        ) {
            let mut sd = SensitiveData::new();
            sd.register(&api_token);
            sd.register(&space_id);

            // Create an error containing sensitive data
            let error_msg = format!(
                "Error at https://{}.repsona.com with token {} - {}",
                space_id, api_token, error_text
            );
            let error = anyhow::anyhow!("{}", error_msg);

            let report = ErrorReport::new(&error, Some("task list"), &sd);
            let markdown = report.to_markdown();

            prop_assert!(!markdown.contains(&api_token),
                "API token found in markdown output");
            prop_assert!(!markdown.contains(&space_id),
                "Space ID found in markdown output");
            prop_assert!(report.verify_no_sensitive_data(&sd));
        }

        /// Property: Adding context also sanitizes sensitive data.
        #[test]
        fn prop_context_sanitized(
            secret in "[a-zA-Z0-9]{8,20}",
            _context_text in ".{0,50}"
        ) {
            let mut sd = SensitiveData::new();
            sd.register(&secret);

            let error = anyhow::anyhow!("Some error");
            let mut report = ErrorReport::new(&error, None, &sd);

            let context = format!("Context with {} inside", secret);
            report.add_context(&context, &sd);

            let markdown = report.to_markdown();
            prop_assert!(!markdown.contains(&secret),
                "Secret '{}' found in context", secret);
        }

        /// Property: Long base64-like tokens (without slashes) are redacted.
        /// Note: Slashes are excluded from the pattern to avoid matching URL paths.
        #[test]
        fn prop_long_base64_tokens_redacted(
            token in "[A-Za-z0-9+_=-]{32,128}"
        ) {
            let input = format!("Token: {}", token);
            let output = ErrorReport::sanitize_common_patterns(&input);

            prop_assert!(!output.contains(&token),
                "Base64 token found in output: '{}'", output);
        }
    }

    // =========================================================================
    // Exhaustive Tests for Known Sensitive Patterns
    // =========================================================================

    #[test]
    fn test_all_sensitive_patterns_redacted() {
        let test_cases = vec![
            // API tokens (various formats)
            ("api_token=abc123def456ghi789", "api_token"),
            ("Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9", "JWT token"),
            // Space IDs in URLs
            ("https://mycompany.repsona.com/api/tasks", "space in URL"),
            ("Error at https://test-space.repsona.com", "space in error"),
            // UUIDs
            ("task_id: 123e4567-e89b-12d3-a456-426614174000", "UUID"),
            // Environment variable values shouldn't leak
            ("REPSONA_TOKEN=secret123", "env var"),
        ];

        let mut sd = SensitiveData::new();
        sd.register("abc123def456ghi789");
        sd.register("mycompany");
        sd.register("test-space");
        sd.register("secret123");

        for (input, desc) in test_cases {
            let sanitized = sd.sanitize(input);
            let sanitized = ErrorReport::sanitize_common_patterns(&sanitized);

            // Check that no registered secrets appear
            assert!(!sd.contains_sensitive(&sanitized),
                "Failed for {}: output still contains sensitive data: {}", desc, sanitized);
        }
    }

    #[test]
    fn test_command_argument_stripping() {
        let sd = SensitiveData::new();
        let error = anyhow::anyhow!("Error");

        // Full command with arguments
        let report = ErrorReport::new(
            &error,
            Some("task create --name 'Secret Project Name' --description 'Confidential info'"),
            &sd
        );

        // Only command name should be present
        assert_eq!(report.command, Some("task".to_string()));

        // Arguments should not appear anywhere
        let md = report.to_markdown();
        assert!(!md.contains("Secret Project Name"));
        assert!(!md.contains("Confidential info"));
    }

    #[test]
    fn test_repsona_data_not_in_report() {
        // Simulate Repsona response data that should never appear in reports
        let repsona_data = vec![
            "My Important Task",
            "user@example.com",
            "John Doe",
            "Meeting notes: Q4 budget review",
            "Project Alpha - Phase 2",
        ];

        let mut sd = SensitiveData::new();
        for data in &repsona_data {
            sd.register(*data);
        }

        // Even if an error somehow contains this data, it should be redacted
        let error = anyhow::anyhow!(
            "Failed to update task 'My Important Task' for user@example.com"
        );
        let report = ErrorReport::new(&error, Some("task update"), &sd);
        let md = report.to_markdown();

        for data in &repsona_data {
            assert!(!md.contains(data),
                "Repsona data '{}' found in report", data);
        }
    }
}
