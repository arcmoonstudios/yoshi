/* examples/simple_demo.rs */
#![warn(missing_docs)]
#![deny(unsafe_code)]
//! **Brief:** Simple demonstration of core Yoshi functionality.
//!
//! This replaces the derive functionality demo since derive macros
//! are not yet implemented. Shows the essential Yoshi patterns.
//!
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + Simple Yoshi demonstration
//!  - Basic error creation and handling
//!  - New Hatch ecosystem usage
//!  - Thematic methods showcase
//!  - Debug output with yum! macro
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Author:** Lord Xyn

use std::io;
use yoshi_std::{yum, Hatch, Hatchable, LayText, Yoshi, YoshiKind};

/// Simple error types for demonstration.
#[derive(Debug)]
pub enum SimpleError {
    Io(io::Error),
    Validation(String),
}

impl std::fmt::Display for SimpleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "I/O error: {}", err),
            Self::Validation(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for SimpleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::Validation(_) => None,
        }
    }
}

impl From<SimpleError> for Yoshi {
    fn from(err: SimpleError) -> Self {
        match err {
            SimpleError::Io(io_err) => Yoshi::from(io_err),
            SimpleError::Validation(msg) => Yoshi::new(YoshiKind::Validation {
                field: "input".into(),
                message: msg.into(),
                expected: None,
                actual: None,
            }),
        }
    }
}

/// Demonstrates file operations with Yoshi error handling.
fn read_config_file(path: &str) -> Hatch<String> {
    if path.is_empty() {
        return Err(Yoshi::new(YoshiKind::Validation {
            field: "file_path".into(),
            message: "File path cannot be empty".into(),
            expected: Some("valid file path".into()),
            actual: Some("empty string".into()),
        }))
        .lay("Validating configuration file path");
    }

    // Simulate file reading that might fail
    if path == "missing.txt" {
        return Err(SimpleError::Io(io::Error::new(
            io::ErrorKind::NotFound,
            "Configuration file not found",
        )))
        .hatch() // Convert to Hatch<String>
        .lay("Reading configuration file from disk")
        .help("Ensure the configuration file exists and is readable");
    }

    Ok(format!("Configuration loaded from {}", path))
}

/// Demonstrates network operations with recovery strategies.
fn fetch_remote_data(url: &str) -> Hatch<String> {
    if url.starts_with("http://") {
        return Err(Yoshi::new(YoshiKind::Network {
            message: "Insecure connection rejected".into(),
            source: None,
            error_code: Some(400),
        }))
        .lay("Establishing secure connection to remote service")
        .help("Use HTTPS instead of HTTP for secure communication")
        .meta("provided_url", url)
        .meta("security_policy", "https_only");
    }

    if url.contains("timeout") {
        return Err(Yoshi::new(YoshiKind::Timeout {
            operation: "HTTP request".into(),
            duration: std::time::Duration::from_secs(30),
            expected_max: Some(std::time::Duration::from_secs(10)),
        }))
        .lay("Remote data fetch operation exceeded timeout")
        .help("Check network connectivity and service availability");
    }

    Ok(format!("Data fetched from {}", url))
}

/// Comprehensive workflow demonstrating Yoshi ecosystem.
fn application_workflow() -> Hatch<String> {
    // Step 1: Load configuration
    let config = read_config_file("app.config")?;

    // Step 2: Fetch remote data
    let data = fetch_remote_data("https://api.example.com/data")?;

    // Step 3: Process results
    Ok(format!("Workflow completed: {} -> {}", config, data))
}

/// Main demonstration function.
fn main() {
    println!("🦕 Yoshi Error Handling Demo");
    println!("============================");

    // Demo 1: Successful operation
    println!("\n1. Successful operation:");
    match application_workflow() {
        Ok(result) => println!("✅ {}", result),
        Err(error) => {
            let debug_error = yum!(error);
            println!("❌ Workflow failed: {}", debug_error);
        }
    }

    // Demo 2: Validation error
    println!("\n2. Validation error:");
    match read_config_file("") {
        Ok(result) => println!("✅ {}", result),
        Err(error) => {
            let debug_error = yum!(error);
            println!("❌ Validation failed");
            println!("   Context: {:?}", debug_error.laytext());
            println!("   Suggestion: {:?}", debug_error.suggestion());
        }
    }

    // Demo 3: File not found error
    println!("\n3. File not found error:");
    match read_config_file("missing.txt") {
        Ok(result) => println!("✅ {}", result),
        Err(error) => {
            let debug_error = yum!(error);
            println!("❌ File operation failed");
            println!("   Context: {:?}", debug_error.laytext());
            println!("   Nested error: {:?}", debug_error.nest());
            println!("   Help: {:?}", debug_error.suggestion());
        }
    }

    // Demo 4: Network security error
    println!("\n4. Network security error:");
    match fetch_remote_data("http://insecure.example.com") {
        Ok(result) => println!("✅ {}", result),
        Err(error) => {
            let debug_error = yum!(error);
            println!("❌ Network operation rejected");
            println!("   Severity: {}", debug_error.severity());
            println!("   Is transient: {}", debug_error.is_transient());

            // Access metadata
            if let Some(ctx) = debug_error.primary_context() {
                if let Some(url) = ctx.metadata.get("provided_url") {
                    println!("   Rejected URL: {}", url);
                }
            }
        }
    }

    // Demo 5: Timeout error
    println!("\n5. Timeout error:");
    match fetch_remote_data("https://slow.timeout.com") {
        Ok(result) => println!("✅ {}", result),
        Err(error) => {
            let debug_error = yum!(error);
            println!("❌ Operation timed out");

            // Analyze error contexts
            let analysis = debug_error.analyze_contexts();
            println!(
                "   Analysis: {} contexts, {} metadata entries",
                analysis.total_contexts, analysis.metadata_entries
            );
            println!("   Has suggestions: {}", analysis.has_suggestions);
        }
    }

    println!("\n🎯 Demo completed! Check the enhanced debug output above.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successful_config_read() {
        let result = read_config_file("valid.config");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Configuration loaded"));
    }

    #[test]
    fn test_empty_path_validation() {
        let result = read_config_file("");
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(matches!(error.kind(), YoshiKind::Validation { .. }));
        assert!(error.laytext().is_some());
    }

    #[test]
    fn test_file_not_found() {
        let result = read_config_file("missing.txt");
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.nest().is_some()); // Should have nested I/O error
        assert!(error.suggestion().is_some());
    }

    #[test]
    fn test_insecure_url_rejection() {
        let result = fetch_remote_data("http://insecure.com");
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(matches!(error.kind(), YoshiKind::Network { .. }));

        // Check metadata
        if let Some(ctx) = error.primary_context() {
            assert!(ctx.metadata.contains_key(&"provided_url".into()));
        }
    }

    #[test]
    fn test_timeout_error() {
        let result = fetch_remote_data("https://timeout.example.com");
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(matches!(error.kind(), YoshiKind::Timeout { .. }));
        assert!(error.is_transient()); // Timeouts are transient
    }

    #[test]
    fn test_error_analysis() {
        let result = fetch_remote_data("http://test.com");
        assert!(result.is_err());
        let error = result.unwrap_err();

        let analysis = error.analyze_contexts();
        assert!(analysis.total_contexts > 0);
        assert!(analysis.has_suggestions);
        assert!(analysis.metadata_entries > 0);
    }
}
