/* examples/basic_error_handling.rs */
#![deny(dead_code)]
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![allow(unused_variables)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
//! **Brief:** Basic error handling patterns with the Yoshi framework.
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + Fundamental error handling concepts and patterns
//!  - Simple error creation and propagation
//!  - Basic context addition and error chaining
//!  - File I/O and parsing error handling
//!  - Result type usage and error conversion
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

use tracing;
use yoshi::*;
//--------------------------------------------------------------------------------------------------
// Basic Error Types
//--------------------------------------------------------------------------------------------------

/// Simple error types for basic operations.
///
/// This enum demonstrates fundamental error handling patterns with clear,
/// descriptive error variants for common scenarios.
#[derive(YoshiError, Debug)]
#[allow(dead_code)]
pub enum BasicError {
    /// File operation failed.
    ///
    /// Contains the file path and error description for debugging.
    #[yoshi(display = "File operation failed: {path} - {message}")]
    #[yoshi(suggestion = "Check file permissions and path validity")]
    FileError {
        /// Path to the file that caused the error
        path: String,
        /// Detailed error message
        message: String,
    },

    /// Data parsing failed.
    ///
    /// Provides information about what data failed to parse and why.
    #[yoshi(display = "Failed to parse {data_type}: {reason}")]
    #[yoshi(suggestion = "Verify data format and structure")]
    ParseError {
        /// Type of data that failed to parse
        data_type: String,
        /// Reason for the parsing failure
        reason: String,
    },

    /// Configuration is invalid.
    ///
    /// Contains the configuration key and validation error details.
    #[yoshi(display = "Invalid configuration for '{key}': {details}")]
    #[yoshi(suggestion = "Check configuration documentation and fix the value")]
    ConfigError {
        /// Configuration key that is invalid
        key: String,
        /// Details about why the configuration is invalid
        details: String,
    },

    /// Network operation failed.
    ///
    /// Provides network-specific error information for troubleshooting.
    #[yoshi(display = "Network error: {operation} failed - {reason}")]
    #[yoshi(suggestion = "Check network connectivity and retry")]
    NetworkError {
        /// The network operation that failed
        operation: String,
        /// Reason for the network failure
        reason: String,
    },
}

//--------------------------------------------------------------------------------------------------
// Basic Error Handling Functions
//--------------------------------------------------------------------------------------------------

/// Reads a configuration file and parses it as JSON.
///
/// This function demonstrates basic error handling with file I/O and JSON parsing,
/// showing how to convert different error types into the Yoshi ecosystem.
///
/// # Arguments
///
/// * `path` - Path to the configuration file
///
/// # Returns
///
/// A `Hatch<HashMap<String, String>>` containing the parsed configuration
/// or an error if reading or parsing fails.
///
/// # Examples
///
/// ```rust,no_run
/// use yoshi::*;
/// # use std::collections::HashMap;
/// # fn main() -> Hatch<()> {
/// let config = read_config_file("config.json")?;
/// println!("Loaded {} configuration entries", config.len());
/// # Ok(())
/// # }
/// ```
pub fn read_config_file(path: &str) -> Hatch<HashMap<String, String>> {
    // Read the file content
    let content = fs::read_to_string(path).map_err(|e| BasicError::FileError {
        path: path.to_string(),
        message: e.to_string(),
    })?;

    // Parse as JSON
    let config: HashMap<String, String> =
        serde_json::from_str(&content).map_err(|e| BasicError::ParseError {
            data_type: "JSON".to_string(),
            reason: e.to_string(),
        })?;

    Ok(config)
}

/// Validates a configuration value against basic rules.
///
/// Demonstrates simple validation logic with descriptive error messages
/// for different validation failure scenarios.
///
/// # Arguments
///
/// * `key` - The configuration key being validated
/// * `value` - The value to validate
///
/// # Returns
///
/// A `Hatch<()>` indicating success or validation failure.
pub fn validate_config_value(key: &str, value: &str) -> Hatch<()> {
    if value.is_empty() {
        return Err(BasicError::ConfigError {
            key: key.to_string(),
            details: "Value cannot be empty".to_string(),
        }
        .into());
    }

    if key == "port" {
        let _port: u16 = value.parse().map_err(|_| BasicError::ConfigError {
            key: key.to_string(),
            details: format!("'{value}' is not a valid port number"),
        })?;
    }

    if key == "timeout" {
        let timeout: u64 = value.parse().map_err(|_| BasicError::ConfigError {
            key: key.to_string(),
            details: format!("'{value}' is not a valid timeout value"),
        })?;

        if timeout > 300 {
            return Err(BasicError::ConfigError {
                key: key.to_string(),
                details: "Timeout cannot exceed 300 seconds".to_string(),
            }
            .into());
        }
    }

    Ok(())
}

/// Processes a list of files with error aggregation.
///
/// Shows how to handle multiple operations where some may fail,
/// collecting errors while continuing to process other items.
///
/// # Arguments
///
/// * `file_paths` - List of file paths to process
///
/// # Returns
///
/// A `Hatch<Vec<String>>` containing successfully processed file contents
/// or an error if critical failures occur.
pub fn process_files(file_paths: &[&str]) -> Hatch<Vec<String>> {
    let mut results = Vec::with_capacity(10);
    let mut errors = Vec::with_capacity(10);

    for path in file_paths {
        match fs::read_to_string(path) {
            Ok(content) => {
                results.push(content);
            }
            Err(e) => {
                errors.push(format!("{path}: {e}"));
            }
        }
    }

    // If more than half the files failed, consider it a critical error
    if errors.len() > file_paths.len() / 2 {
        return Err(BasicError::FileError {
            path: "multiple".to_string(),
            message: format!("Too many failures: {}", errors.join(", ")),
        }
        .into());
    }

    Ok(results)
}

/// Demonstrates basic error context addition.
///
/// Shows how to add contextual information to errors as they propagate
/// up the call stack, making debugging easier.
///
/// # Arguments
///
/// * `operation` - Description of the operation being performed
/// * `data` - Data to process
///
/// # Returns
///
/// A `Hatch<String>` with the processed result or contextual error information.
pub fn process_with_context(operation: &str, data: &str) -> Hatch<String> {
    if data.is_empty() {
        return Err(yoshi!(message: "Cannot process empty data"));
    }

    // Simulate processing that might fail
    if data.contains("invalid") {
        return Err(BasicError::ParseError {
            data_type: "user_input".to_string(),
            reason: "Contains invalid characters".to_string(),
        }
        .into())
        .lay(format!("During operation: {operation}"))?;
    }

    Ok(format!("Processed: {}", data.to_uppercase()))
}

//--------------------------------------------------------------------------------------------------
// Example Usage and Demonstrations
//--------------------------------------------------------------------------------------------------

/// Demonstrates basic error handling patterns.
///
/// This function shows various ways to create, handle, and propagate errors
/// using the Yoshi framework's basic features.
pub fn demonstrate_basic_patterns() -> Hatch<()> {
    tracing::error!("=== Basic Error Handling Demonstration ===");

    // Example 1: Simple error creation
    let simple_error = yoshi!(message: "Something went wrong");
    tracing::error!("Simple error: {simple_error}");

    // Example 2: Error with context
    let contextual_error = yoshi!(message: "Operation failed");
    tracing::error!("Contextual error: {contextual_error}");

    // Example 3: Converting std::io::Error
    match fs::read_to_string("nonexistent.txt") {
        Ok(_) => println!("File read successfully"),
        Err(e) => {
            let yoshi_error = io_error_to_yoshi(e).lay("Failed to read configuration file");
            tracing::error!("Converted I/O error: {yoshi_error}");
        }
    }

    // Example 4: Error propagation with ?
    match process_with_context("demo", "test_data") {
        Ok(result) => println!("Processing result: {result}"),
        Err(e) => println!("Processing error: {e}"),
    }

    Ok(())
}

/// Main function demonstrating basic error handling concepts.
///
/// Runs through various examples showing fundamental error handling
/// patterns and best practices with the Yoshi framework.
pub fn main() -> Hatch<()> {
    demonstrate_basic_patterns()?;

    // Example configuration processing
    let config_data = vec![("port", "8080"), ("timeout", "30"), ("host", "localhost")];

    for (key, value) in config_data {
        match validate_config_value(key, value) {
            Ok(()) => println!("✓ {key} = {value} is valid"),
            Err(e) => println!("✗ Validation error: {e}"),
        }
    }

    tracing::error!("\n=== Basic Error Handling Complete ===");
    Ok(())
}
