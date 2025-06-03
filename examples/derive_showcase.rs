/* examples/derive_showcase.rs */
#![warn(missing_docs)]
#![deny(unsafe_code)]
//! **Brief:** Comprehensive showcase of the YoshiError derive macro capabilities.
//!
//! This example demonstrates the full power of the `#[derive(YoshiError)]` macro,
//! including advanced attributes, auto-inference, shorthand syntax, and integration
//! with the Yoshi error handling ecosystem.
//!
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + YoshiError derive macro comprehensive demonstration
//!  - Advanced attribute usage and auto-inference
//!  - Shorthand attribute syntax (y_net, y_timeout, etc.)
//!  - Automatic From conversions with #[yoshi(from)]
//!  - Rich context and metadata handling
//!  - Performance monitoring and tracing integration
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Author:** Lord Xyn

use std::io;
use std::time::Duration;
use yoshi::{yoshi, Arc, HatchExt, Result, Yoshi, YoshiKind};
use yoshi_derive::YoshiError;

/// Advanced application error with comprehensive YoshiError derive configuration.
///
/// This demonstrates the full feature set of the derive macro including:
/// - Global configuration with error code prefix and default severity
/// - Performance monitoring and tracing integration
/// - Precise capturing for Rust 1.87 features
#[derive(Debug, YoshiError)]
#[yoshi(error_code_prefix = "APP")]
#[yoshi(default_severity = 75)]
#[yoshi(performance_monitoring = true)]
#[yoshi(tracing_integration = true)]
#[yoshi(precise_capturing = true)]
#[yoshi(doc_prefix = "Advanced Application Error")]
pub enum AppError {
    /// Configuration error with enhanced context and automatic inference.
    ///
    /// This variant demonstrates:
    /// - Custom display format with field placeholders
    /// - Explicit YoshiKind mapping to Config
    /// - Custom error code assignment
    /// - Source error chaining with #[yoshi(source)]
    /// - Context metadata extraction from file path
    #[yoshi(display = "Failed to load configuration from {config_path}: {source}")]
    #[yoshi(kind = "Config")]
    #[yoshi(error_code = 1001)]
    #[yoshi(severity = 60)]
    #[yoshi(suggestion = "Check configuration file syntax and permissions")]
    ConfigError {
        /// The configuration file path that failed to load
        #[yoshi(context = "config_file")]
        config_path: String,
        /// The underlying I/O error that caused the failure
        #[yoshi(source)]
        source: io::Error,
    },

    /// Network error using shorthand syntax for rapid development.
    ///
    /// This demonstrates the `#[y_net]` shorthand which expands to:
    /// `#[yoshi(kind = "Network", display = "Network error: {message}")]`
    #[y_net]
    #[yoshi(error_code = 1002)]
    #[yoshi(transient = true)]
    NetworkFailure {
        message: String,
        #[yoshi(context = "endpoint")]
        url: String,
        #[yoshi(shell)]
        connection_info: ConnectionInfo,
    },

    /// Timeout error with comprehensive timing information.
    ///
    /// Uses the `#[y_timeout]` shorthand and demonstrates:
    /// - Duration fields for timing information
    /// - Automatic transient flag (timeouts are retryable)
    /// - Shell attachment for structured debugging data
    #[y_timeout]
    #[yoshi(error_code = 1003)]
    #[yoshi(suggestion = "Increase timeout duration or check service health")]
    OperationTimeout {
        operation: String,
        #[yoshi(context = "timeout_duration")]
        duration: Duration,
        expected_max: Duration,
        #[yoshi(shell)]
        performance_metrics: PerformanceMetrics,
    },

    /// User validation error with detailed field analysis.
    ///
    /// Demonstrates:
    /// - Validation kind mapping with expected/actual values
    /// - Multiple context metadata fields
    /// - Field-level suggestions
    /// - Skip annotation for internal debugging fields
    #[yoshi(display = "User validation failed for field '{field}': {message}")]
    #[yoshi(kind = "Validation")]
    #[yoshi(error_code = 1004)]
    #[yoshi(severity = 40)]
    UserValidationError {
        field: String,
        message: String,
        #[yoshi(context = "expected_format")]
        expected: String,
        #[yoshi(context = "actual_value")]
        actual: String,
        #[yoshi(suggestion = "Ensure input matches the expected format")]
        validation_rule: String,
        #[yoshi(skip)]
        internal_validation_id: u32,
    },

    /// Database connection error with automatic From conversion.
    ///
    /// This single-field tuple variant with #[yoshi(from)] enables:
    /// - Automatic `From<DatabaseConnectionError> for AppError`
    /// - Ergonomic `?` operator usage
    /// - Source error preservation
    #[yoshi(display = "Database connection failed")]
    #[yoshi(kind = "Network")]
    #[yoshi(error_code = 1005)]
    #[yoshi(severity = 90)]
    DatabaseError(#[yoshi(from)] DatabaseConnectionError),

    /// I/O error using shorthand with automatic inference.
    ///
    /// The `#[y_from_io]` shorthand provides:
    /// - Automatic From<io::Error> conversion
    /// - Io kind mapping
    /// - Source field marking
    #[y_from_io]
    #[yoshi(error_code = 1006)]
    IoError(#[yoshi(source)] io::Error),

    /// Parse error with custom formatting function.
    ///
    /// Demonstrates:
    /// - Custom format_with function for specialized display
    /// - Multiple shell attachments for debugging
    /// - Context extraction from multiple fields
    #[yoshi(display = "Parse operation failed: {formatted_input}")]
    #[yoshi(kind = "Validation")]
    #[yoshi(error_code = 1007)]
    ParseError {
        #[yoshi(format_with = "format_parse_input")]
        input: String,
        #[yoshi(context = "parser_type")]
        parser: String,
        #[yoshi(shell)]
        parse_context: ParseContext,
        #[yoshi(shell)]
        debug_info: Vec<String>,
    },

    /// System error with resource exhaustion details.
    ///
    /// Auto-inference will detect this as ResourceExhausted kind based on
    /// the variant name pattern and field names.
    ResourceExhausted {
        resource_type: String,
        #[yoshi(context = "current_usage")]
        current: u64,
        #[yoshi(context = "resource_limit")]
        limit: u64,
        usage_percentage: f64,
    },

    /// Internal error demonstrating multiple error aggregation.
    ///
    /// This shows how to wrap multiple related errors into a single variant.
    #[yoshi(display = "Multiple system failures occurred")]
    #[yoshi(kind = "Multiple")]
    #[yoshi(error_code = 1009)]
    #[yoshi(severity = 200)]
    MultipleFailures {
        #[yoshi(shell)]
        errors: Vec<Box<dyn std::error::Error + Send + Sync>>,
        primary_failure: String,
    },
}

/// Custom database connection error for From conversion demonstration.
#[derive(Debug)]
pub struct DatabaseConnectionError {
    pub host: String,
    pub port: u16,
    pub error_message: String,
}

impl std::fmt::Display for DatabaseConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Database connection to {}:{} failed: {}",
            self.host, self.port, self.error_message
        )
    }
}

impl std::error::Error for DatabaseConnectionError {}

/// Connection information for shell attachment.
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub host: String,
    pub port: u16,
    pub protocol: String,
    pub retry_count: u32,
}

/// Performance metrics for timing analysis.
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub start_time: std::time::Instant,
    pub cpu_usage: f64,
    pub memory_usage: usize,
}

/// Parse context for debugging parse errors.
#[derive(Debug, Clone)]
pub struct ParseContext {
    pub position: usize,
    pub line: u32,
    pub column: u32,
    pub expected_tokens: Vec<String>,
}

/// Custom formatting function for parse input display.
pub fn format_parse_input(input: &String) -> String {
    if input.len() > 50 {
        format!("{}... ({} chars)", &input[..47], input.len())
    } else {
        input.clone()
    }
}

/// Demonstrates the derive macro capabilities in a realistic application context.
fn main() {
    println!("🦕 YoshiError Derive Macro Showcase");
    println!("====================================");

    // Example 1: Configuration error with source chaining
    println!("\n1. Configuration Error with Source Chaining:");
    let config_error = AppError::ConfigError {
        config_path: "/etc/app/config.toml".to_string(),
        source: io::Error::new(io::ErrorKind::PermissionDenied, "access denied"),
    };

    println!("   Error: {}", config_error);
    println!("   Error Code: {:?}", config_error.error_code());
    println!("   Severity: {:?}", config_error.severity());
    println!("   Documentation: {}", config_error.documentation());

    // Convert to Yoshi and show enhanced context
    let yoshi_error: Yoshi = config_error.into();
    println!("   As Yoshi: {}", yoshi_error);

    // Example 2: Network error with shell data
    println!("\n2. Network Error with Shell Data:");
    let network_error = AppError::NetworkFailure {
        message: "Connection refused by remote server".to_string(),
        url: "https://api.example.com/v1/data".to_string(),
        connection_info: ConnectionInfo {
            host: "api.example.com".to_string(),
            port: 443,
            protocol: "HTTPS".to_string(),
            retry_count: 3,
        },
    };

    println!("   Error: {}", network_error);
    println!("   Is transient: {}", is_transient_error(&network_error));

    // Example 3: Automatic From conversion
    println!("\n3. Automatic From Conversion:");
    let db_connection_error = DatabaseConnectionError {
        host: "db.example.com".to_string(),
        port: 5432,
        error_message: "connection timeout".to_string(),
    };

    // This works because of #[yoshi(from)] on the DatabaseError variant
    let app_error: AppError = db_connection_error.into();
    println!("   Converted error: {}", app_error);
    println!("   Variant type: {}", app_error.variant_name());

    // Example 4: Using automatic From conversion with ? operator
    println!("\n4. Ergonomic Error Propagation:");
    match demonstrate_error_propagation() {
        Ok(result) => println!("   Success: {}", result),
        Err(error) => {
            println!("   Propagated error: {}", error);

            // Convert to Yoshi for full ecosystem benefits
            let yoshi_error: Yoshi = error.into();
            println!("   Enhanced error: {}", yoshi_error);

            // Show context analysis
            let analysis = yoshi_error.analyze_contexts();
            println!(
                "   Context analysis: {} contexts, {} metadata entries",
                analysis.total_contexts, analysis.metadata_entries
            );
        }
    }

    // Example 5: Parse error with custom formatting
    println!("\n5. Parse Error with Custom Formatting:");
    let parse_error = AppError::ParseError {
        input: "very_long_input_string_that_will_be_truncated_in_the_display_because_it_exceeds_fifty_characters".to_string(),
        parser: "JSON".to_string(),
        parse_context: ParseContext {
            position: 127,
            line: 15,
            column: 23,
            expected_tokens: vec!["}", "\",\"", "string"].into_iter().map(String::from).collect(),
        },
        debug_info: vec!["token_stack_depth: 5".to_string(), "last_valid_token: string_literal".to_string()],
    };

    println!("   Error: {}", parse_error);

    // Example 6: Performance monitoring (if enabled)
    #[cfg(feature = "performance-monitoring")]
    {
        println!("\n6. Performance Monitoring:");
        let perf_metrics = network_error.performance_metrics();
        println!("   Performance data: {:?}", perf_metrics);
        network_error.track_creation();
    }

    // Example 7: Tracing integration (if enabled)
    #[cfg(feature = "tracing")]
    {
        println!("\n7. Tracing Integration:");
        let span = network_error.create_span();
        println!("   Created tracing span: {:?}", span);
        network_error.trace_error();
    }

    println!("\n🎯 Derive macro showcase completed!");
    println!("   Generated implementations include:");
    println!("   - Display with custom formatting");
    println!("   - Error trait with source chaining");
    println!("   - From<AppError> for Yoshi conversion");
    println!("   - Automatic From conversions for marked fields");
    println!("   - Performance monitoring methods");
    println!("   - Tracing integration");
    println!("   - Comprehensive documentation");
}

/// Demonstrates error propagation using the automatic From conversions.
fn demonstrate_error_propagation() -> Result<String, AppError> {
    // This will use the automatic From<io::Error> for AppError conversion
    let _file_content = std::fs::read_to_string("nonexistent.txt")?;

    // This could also propagate database errors
    simulate_database_operation()?;

    Ok("Operation completed successfully".to_string())
}

/// Simulates a database operation that might fail.
fn simulate_database_operation() -> Result<(), DatabaseConnectionError> {
    // Simulate connection failure
    Err(DatabaseConnectionError {
        host: "primary-db.example.com".to_string(),
        port: 5432,
        error_message: "connection pool exhausted".to_string(),
    })
}

/// Helper function to check if an error is transient.
fn is_transient_error(error: &AppError) -> bool {
    // Use the generated variant checking method
    error.is_networkfailure() || error.is_operationtimeout()
}

/// Demonstrates advanced error handling patterns.
fn advanced_error_handling_demo() {
    println!("\n🔧 Advanced Error Handling Patterns:");

    // Pattern 1: Error variant checking
    let timeout_error = AppError::OperationTimeout {
        operation: "API call".to_string(),
        duration: Duration::from_secs(30),
        expected_max: Duration::from_secs(10),
        performance_metrics: PerformanceMetrics {
            start_time: std::time::Instant::now(),
            cpu_usage: 25.5,
            memory_usage: 1024,
        },
    };

    if timeout_error.is_operationtimeout() {
        println!("   Detected timeout error - implementing retry logic");
    }

    // Pattern 2: Error conversion and context extraction
    let yoshi_error: Yoshi = timeout_error.into();
    if let Some(metrics) = yoshi_error.shell::<PerformanceMetrics>() {
        println!(
            "   Performance impact: CPU {}%, Memory {} bytes",
            metrics.cpu_usage, metrics.memory_usage
        );
    }

    // Pattern 3: Error severity-based handling
    let validation_error = AppError::UserValidationError {
        field: "email".to_string(),
        message: "Invalid email format".to_string(),
        expected: "user@domain.com".to_string(),
        actual: "invalid-email".to_string(),
        validation_rule: "RFC 5322 compliance".to_string(),
        internal_validation_id: 12345,
    };

    if let Some(severity) = validation_error.severity() {
        match severity {
            0..=50 => println!("   Low severity error - logging only"),
            51..=100 => println!("   Medium severity error - user notification"),
            101..=200 => println!("   High severity error - admin alert"),
            201..=255 => println!("   Critical error - immediate escalation"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_macro_functionality() {
        // Test basic error creation
        let config_error = AppError::ConfigError {
            config_path: "/test/config.toml".to_string(),
            source: io::Error::new(io::ErrorKind::NotFound, "file not found"),
        };

        // Test Display implementation
        let display_output = format!("{}", config_error);
        assert!(display_output.contains("/test/config.toml"));
        assert!(display_output.contains("file not found"));

        // Test Error trait implementation
        use std::error::Error;
        assert!(config_error.source().is_some());

        // Test conversion to Yoshi
        let yoshi_error: Yoshi = config_error.into();
        assert!(matches!(yoshi_error.kind(), YoshiKind::Config { .. }));
    }

    #[test]
    fn test_automatic_from_conversion() {
        let db_error = DatabaseConnectionError {
            host: "test-db".to_string(),
            port: 5432,
            error_message: "timeout".to_string(),
        };

        // Test automatic From conversion
        let app_error: AppError = db_error.into();
        assert!(matches!(app_error, AppError::DatabaseError(_)));

        // Test variant checking method
        assert!(app_error.is_databaseerror());
    }

    #[test]
    fn test_error_metadata_and_shells() {
        let network_error = AppError::NetworkFailure {
            message: "Connection failed".to_string(),
            url: "https://test.com".to_string(),
            connection_info: ConnectionInfo {
                host: "test.com".to_string(),
                port: 443,
                protocol: "HTTPS".to_string(),
                retry_count: 1,
            },
        };

        // Convert to Yoshi and check for shell data
        let yoshi_error: Yoshi = network_error.into();
        assert!(yoshi_error.shell::<ConnectionInfo>().is_some());

        // Check metadata context
        let analysis = yoshi_error.analyze_contexts();
        assert!(analysis.metadata_entries > 0);
        assert!(analysis.typed_payloads > 0);
    }

    #[test]
    fn test_error_codes_and_severity() {
        let validation_error = AppError::UserValidationError {
            field: "test_field".to_string(),
            message: "test message".to_string(),
            expected: "expected".to_string(),
            actual: "actual".to_string(),
            validation_rule: "test_rule".to_string(),
            internal_validation_id: 1,
        };

        // Test error code
        assert_eq!(validation_error.error_code(), Some(1004));

        // Test severity
        assert_eq!(validation_error.severity(), Some(40));

        // Test documentation
        assert!(validation_error.documentation().contains("Severity: 40"));
    }

    #[test]
    fn test_io_error_propagation() {
        fn failing_operation() -> Result<(), AppError> {
            // This should automatically convert io::Error to AppError::IoError
            std::fs::read_to_string("nonexistent_file.txt")?;
            Ok(())
        }

        let result = failing_operation();
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(matches!(error, AppError::IoError(_)));
    }
}
