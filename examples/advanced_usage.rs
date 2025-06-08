/* examples/advanced_usage.rs */
#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
//! **Brief:** Demonstrates advanced usage patterns for the Yoshi error handling framework.
//!
//! This module delves into more complex error types, rich contextualization, and
//! advanced features provided by the `yoshi_std` crate using the updated API.
//!
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Advanced Error Usage Patterns]
//!  - [Validation Error: Detailed input validation with expected/actual values]
//!  - [Timeout Error: Capturing operation duration and suggestions]
//!  - [Rich Contextualization: Attaching multiple metadata and custom payloads]
//!  - [Error Priority: Setting severity for different contexts]
//!  - [Advanced Analysis: Comprehensive error introspection]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Author:** Lord Xyn

use std::time::Duration;
use yoshi_std::{yum, Hatch, HatchExt, Hatchable, LayText, YoContext, Yoshi, YoshiKind};

/// Custom struct to use as a shell.
#[derive(Debug, PartialEq, Clone)]
struct RequestInfo {
    id: String,
    method: String,
    path: String,
}

/// Example 1: Creating a detailed validation error.
///
/// This example shows how to create a `YoshiKind::Validation` error with
/// specific details about what was expected versus what was received.
mod example_1_detailed_validation_error {
    use super::*;

    /// Creates a validation error using direct API calls.
    pub fn create_validation_error() -> Yoshi {
        Yoshi::new(YoshiKind::Validation {
            field: "password".into(),
            message: "Password does not meet complexity requirements.".into(),
            expected: Some("Minimum 8 characters, 1 uppercase, 1 number".into()),
            actual: Some("shortpass".into()),
        })
        .with_suggestion("Choose a stronger password containing diverse characters.")
    }

    /// Creates a validation error and returns as Hatch with context.
    pub fn create_as_hatch_with_context() -> Hatch<()> {
        Err(Yoshi::new(YoshiKind::Validation {
            field: "email".into(),
            message: "Invalid email format detected".into(),
            expected: Some("user@domain.com format".into()),
            actual: Some("invalid_email".into()),
        }))
        .lay("During user registration validation")
        .help("Provide a valid email address in the format user@domain.com")
        .meta("validation_step", "email_format_check")
    }
}

/// Example 2: Creating a timeout error with custom duration and suggestion.
///
/// This example focuses on the `YoshiKind::Timeout` variant and demonstrates
/// adding specific timing information and a recovery suggestion.
mod example_2_timeout_with_suggestion {
    use super::*;

    /// Creates a timeout error using direct API calls.
    pub fn create_timeout_error() -> Yoshi {
        Yoshi::new(YoshiKind::Timeout {
            operation: "External API call".into(),
            duration: Duration::from_secs(5),
            expected_max: Some(Duration::from_secs(3)),
        })
        .with_suggestion("Retry the API call after a short delay or check network connectivity.")
    }

    /// Creates a timeout error as Hatch with enhanced context.
    pub fn create_as_hatch_with_enhanced_context() -> Hatch<String> {
        Err(Yoshi::new(YoshiKind::Timeout {
            operation: "Database query".into(),
            duration: Duration::from_secs(30),
            expected_max: Some(Duration::from_secs(10)),
        }))
        .lay("Database operation exceeded timeout threshold")
        .help("Check database server status and optimize query performance")
        .meta("query_type", "user_lookup")
        .meta("timeout_threshold", "10s")
        .with_priority(200) // High priority timeout
    }
}

/// Example 3: Attaching a custom struct as a shell and setting error priority.
///
/// This example demonstrates how to embed arbitrary typed data (payloads)
/// within an error's context and assign a priority level.
mod example_3_payload_and_priority {
    use super::*;

    /// Creates an error with a custom shell and priority using direct API.
    pub fn create_with_payload() -> Yoshi {
        Yoshi::new(YoshiKind::Internal {
            message: "Request processing failed due to an unexpected state.".into(),
            source: None,
            component: Some("RequestProcessor".into()),
        })
        .with_shell(RequestInfo {
            id: "req_xyz_789".to_string(),
            method: "POST".to_string(),
            path: "/api/v1/data".to_string(),
        })
        .with_priority(200) // Higher priority for critical issues
    }

    /// Creates an error as Hatch with multiple shells and metadata.
    pub fn create_as_hatch_with_multiple_shells() -> Hatch<()> {
        Err(Yoshi::new(YoshiKind::Network {
            message: "Service communication failure".into(),
            source: None,
            error_code: Some(503),
        }))
        .lay("Failed to communicate with downstream service")
        .with_shell(RequestInfo {
            id: "req_abc_123".to_string(),
            method: "GET".to_string(),
            path: "/api/v2/status".to_string(),
        })
        .with_shell(vec!["retry_attempt_1", "retry_attempt_2"])
        .meta("service_name", "user_service")
        .meta("region", "us-east-1")
        .help("Check service health dashboard and retry after brief delay")
        .with_priority(250)
    }
}

/// Example 4: Advanced error analysis and introspection.
///
/// This example demonstrates how to retrieve and analyze detailed properties
/// of a Yoshi error, including context analysis and shell inspection.
mod example_4_advanced_analysis {
    use super::*;

    /// Creates a complex error for analysis.
    fn create_complex_error() -> Yoshi {
        Yoshi::new(YoshiKind::Internal {
            message: "Multi-stage processing pipeline failure".into(),
            source: None,
            component: Some("DataPipeline".into()),
        })
        .with_metadata("pipeline_id", "pipe_001")
        .with_shell(RequestInfo {
            id: "req_pipeline_001".to_string(),
            method: "POST".to_string(),
            path: "/api/v1/process".to_string(),
        })
        .lay("Error during data transformation stage")
        .meta("stage", "transformation")
        .meta("batch_size", "1000")
        .with_shell(vec![1, 2, 3, 4, 5])
        .lay("Input validation failed before processing")
        .help("Verify input data format and schema compliance")
        .with_priority(230)
    }

    /// Analyzes an error's properties and context.
    pub fn analyze_error_properties() -> (u8, bool, usize) {
        let error = create_complex_error();

        let severity = error.severity();
        let is_transient = error.is_transient();
        let context_count = error.contexts().count();

        // Perform context analysis
        let analysis = error.analyze_contexts();
        println!("Error Analysis:");
        println!("  Total contexts: {}", analysis.total_contexts);
        println!("  Has suggestions: {}", analysis.has_suggestions);
        println!("  Has location info: {}", analysis.has_location_info);
        println!("  Metadata entries: {}", analysis.metadata_entries);
        println!("  Typed payloads: {}", analysis.typed_payloads);
        println!(
            "  Primary context priority: {}",
            analysis.primary_context_priority
        );

        (severity, is_transient, context_count)
    }

    /// Demonstrates comprehensive error introspection.
    pub fn introspect_error_details() {
        let error = create_complex_error();

        // Use yum! for enhanced debugging
        let debug_error = yum!(error);

        // Access thematic methods
        println!("Context (laytext): {:?}", debug_error.laytext());
        println!("Nested error (nest): {:?}", debug_error.nest());
        println!("Suggestion: {:?}", debug_error.suggestion());

        // Access typed shells
        if let Some(request_info) = debug_error.shell::<RequestInfo>() {
            println!("Request info: {:?}", request_info);
        }

        if let Some(numbers) = debug_error.shell::<Vec<i32>>() {
            println!("Number sequence: {:?}", numbers);
        }

        // Iterate through all contexts
        for (i, context) in debug_error.contexts().enumerate() {
            println!("Context {}: {:?}", i, context.message);
            if !context.metadata.is_empty() {
                println!("  Metadata: {:?}", context.metadata);
            }
        }
    }
}

/// Example 5: Error chaining and propagation with enhanced context.
///
/// This example shows complex error propagation scenarios using the new
/// Hatch ecosystem.
mod example_5_error_chaining {
    use super::*;

    /// Simulates a low-level error.
    fn database_connection_error() -> Hatch<String> {
        Err(Yoshi::new(YoshiKind::Network {
            message: "Database connection lost".into(),
            source: None,
            error_code: Some(2006),
        }))
        .lay("Connection to primary database failed")
        .meta("database_host", "db-primary.example.com")
        .meta("connection_pool", "primary_pool")
    }

    /// Simulates a mid-level service error.
    fn user_service_error() -> Hatch<serde_json::Value> {
        database_connection_error()?;
        unreachable!("This line should never be reached due to error propagation")
    }

    /// Simulates a high-level API error with full context chain.
    pub fn api_endpoint_error() -> Hatch<serde_json::Value> {
        user_service_error()
            .lay("User service operation failed during API request")
            .help("Check database connectivity and service health")
            .meta("api_endpoint", "/api/v1/users/profile")
            .meta("request_id", "req_12345")
            .with_priority(220)
    }

    /// Demonstrates error recovery and fallback patterns.
    pub fn handle_with_fallback() -> Hatch<String> {
        match api_endpoint_error() {
            Err(error) => {
                // Check if error is transient and suggests retry
                if error.is_transient() {
                    // Log error with yum! and attempt fallback
                    let logged_error = yum!(error);
                    println!("Attempting fallback due to transient error");

                    // Return fallback result
                    Ok("Fallback data from cache".to_string())
                } else {
                    // Re-propagate non-transient errors with additional context
                    Err(logged_error)
                        .lay("Fallback recovery failed - non-transient error")
                        .help("Manual intervention may be required")
                }
            }
            Ok(data) => Ok(data.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1_detailed_validation_error() {
        let err = example_1_detailed_validation_error::create_validation_error();
        assert!(matches!(err.kind(), YoshiKind::Validation { .. }));
        assert!(format!("{}", err).contains("Password does not meet complexity"));
        assert!(err.suggestion().is_some());

        let hatch_result = example_1_detailed_validation_error::create_as_hatch_with_context();
        assert!(hatch_result.is_err());
        let err = hatch_result.unwrap_err();
        assert!(err.laytext().unwrap().contains("registration validation"));
    }

    #[test]
    fn test_example_2_timeout_with_suggestion() {
        let err = example_2_timeout_with_suggestion::create_timeout_error();
        assert!(matches!(err.kind(), YoshiKind::Timeout { .. }));
        assert!(err.suggestion().is_some());
        assert!(err.is_transient());

        let hatch_result =
            example_2_timeout_with_suggestion::create_as_hatch_with_enhanced_context();
        assert!(hatch_result.is_err());
        let err = hatch_result.unwrap_err();
        assert_eq!(err.primary_context().unwrap().priority, 200);
    }

    #[test]
    fn test_example_3_payload_and_priority() {
        let err = example_3_payload_and_priority::create_with_payload();
        assert_eq!(err.primary_context().unwrap().priority, 200);
        assert!(err.shell::<RequestInfo>().is_some());
        assert_eq!(err.shell::<RequestInfo>().unwrap().id, "req_xyz_789");

        let hatch_result = example_3_payload_and_priority::create_as_hatch_with_multiple_shells();
        assert!(hatch_result.is_err());
        let err = hatch_result.unwrap_err();
        assert!(err.shell::<RequestInfo>().is_some());
        assert!(err.shell::<Vec<&str>>().is_some());
    }

    #[test]
    fn test_example_4_advanced_analysis() {
        let (severity, transient, context_count) =
            example_4_advanced_analysis::analyze_error_properties();
        assert_eq!(severity, 80); // Internal error severity
        assert!(!transient); // Internal errors are not transient
        assert!(context_count > 0);

        // Test introspection (mainly for ensuring no panics)
        example_4_advanced_analysis::introspect_error_details();
    }

    #[test]
    fn test_example_5_error_chaining() {
        let result = example_5_error_chaining::api_endpoint_error();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.laytext().unwrap().contains("service operation failed"));

        // Test fallback handling
        let fallback_result = example_5_error_chaining::handle_with_fallback();
        // This might succeed due to fallback logic
        if let Err(err) = fallback_result {
            assert!(err.laytext().is_some());
        }
    }
}
