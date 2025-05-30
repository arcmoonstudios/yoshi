/* examples/expert_usage.rs */
#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
//! **Brief:** Demonstrates expert-level usage patterns for the Yoshi error handling framework.
//!
//! This module explores advanced error composition, detailed introspection, and
//! integration with features like `YoshiError` derive and error recovery strategies.
//! Examples are provided with both the `yoshi!` macro and explicit API calls.
//!
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Expert Error Usage Patterns]
//!  - [Chained Yoshi Errors: Nesting Yoshi errors as sources for complex traces]
//!  - [Multiple Errors Aggregation: Using `YoshiKind::Multiple` for batch failures]
//!  - [Custom Error Derive: Defining structured errors with `#[derive(YoshiError)]`]
//!  - [Detailed Introspection: Accessing specific contexts, metadata, and typed payloads]
//!  - [Error Recovery Strategy: Attaching recovery instructions as payloads]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** Business Source License 1.1 (BSL-1.1)
// **License Terms:** Non-production use only; commercial/production use requires paid license.
// **Effective Date:** 2025-05-25 | **Change License:** GPL v3
// **License File:** /LICENSE
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn
// **Last Validation:** 2025-05-30

use yoshi::{yoshi, Result, Yoshi, YoshiKind, YoshiContext, YoshiLocation, YoshiContextExt};
// Conditional compilation for yoshi-derive and serde if the user's Cargo.toml has features
#[cfg(feature = "derive")]
use yoshi_derive::YoshiError; 
use std::collections::HashMap; // For metadata inspection
use std::sync::Arc; // For Arc<str> in YoshiKind fields
use std::time::Duration; // For YoshiKind::Timeout

/// Custom struct to use as a payload.
#[derive(Debug, PartialEq, Clone)]
struct CustomErrorState {
    retries: u32,
    service: String,
}

/// Recovery strategy payload to attach to errors.
#[derive(Debug, PartialEq, Clone)]
enum RecoveryStrategy {
    RetryWithDelay(Duration),
    FallbackToCache,
    ManualIntervention,
}

/// Example 1: Error with a chain of Yoshi errors (nested Yoshi as source).
///
/// This demonstrates building a call stack of Yoshi errors where each layer
/// provides additional context or wraps a lower-level Yoshi error.
mod example_1_chained_yoshi_errors {
    use super::*;

    /// Simulates a low-level network error.
    fn low_level_network_error() -> Yoshi {
        yoshi!(kind: YoshiKind::Network {
            message: "TCP connection reset by peer".into(),
            source: None,
            error_code: Some(104),
        })
    }

    /// Simulates a middle-level API error wrapping the network error.
    fn middle_level_api_error_with_macro() -> Yoshi {
        let network_err = low_level_network_error();
        yoshi!(kind: YoshiKind::Internal {
            message: "Failed to communicate with authentication service".into(),
            source: Some(Box::new(network_err)),
            component: Some("AuthService".into()),
        },
        with_metadata = ("api_endpoint", "/auth/login"),
        with_suggestion = "Verify authentication service is running.")
    }

    /// Simulates a middle-level API error wrapping the network error using direct API.
    fn middle_level_api_error_with_api() -> Yoshi {
        let network_err = low_level_network_error();
        Yoshi::new(YoshiKind::Internal {
            message: "Failed to communicate with authentication service".into(),
            source: Some(Box::new(network_err)),
            component: Some("AuthService".into()),
        })
        .with_metadata("api_endpoint", "/auth/login".to_string())
        .with_suggestion("Verify authentication service is running.".to_string())
    }

    /// Creates a high-level user-facing error wrapping the API error.
    pub fn create_chained_with_macro() -> Yoshi {
        let api_err = middle_level_api_error_with_macro();
        yoshi!(kind: YoshiKind::Internal { // Using Internal as a general app error
            message: "User login failed unexpectedly".into(),
            source: Some(Box::new(api_err)),
            component: Some("UserService".into()),
        },
        with_metadata = ("user_id", "test_user_id"),
        with_suggestion = "Please try logging in again later. If the issue persists, contact support.")
    }

    /// Creates a high-level user-facing error wrapping the API error using direct API.
    pub fn create_chained_with_api() -> Yoshi {
        let api_err = middle_level_api_error_with_api();
        Yoshi::new(YoshiKind::Internal {
            message: "User login failed unexpectedly".into(),
            source: Some(Box::new(api_err)),
            component: Some("UserService".into()),
        })
        .with_metadata("user_id", "test_user_id".to_string())
        .with_suggestion("Please try logging in again later. If the issue persists, contact support.".to_string())
    }
}

/// Example 2: Aggregating multiple errors into a single `YoshiKind::Multiple`.
///
/// This is useful for scenarios like batch processing or form validation where
/// multiple distinct failures can occur.
mod example_2_multiple_errors {
    use super::*;

    /// Creates individual errors to aggregate.
    fn create_individual_errors() -> Vec<Yoshi> {
        vec![
            yoshi!(kind: YoshiKind::Validation {
                field: "username".into(),
                message: "Username already taken".into(),
                expected: None, actual: Some("existing_user".into())
            }),
            yoshi!(kind: YoshiKind::Validation {
                field: "email".into(),
                message: "Invalid email format".into(),
                expected: Some("user@domain.com".into()), actual: Some("invalid".into())
            }),
            yoshi!(kind: YoshiKind::Internal {
                message: "Internal processing error for profile creation".into(),
                source: None, component: Some("ProfileService".into())
            }),
        ]
    }

    /// Aggregates multiple errors using the `yoshi!` macro.
    ///
    /// Note: The `yoshi!` macro itself doesn't directly support `YoshiKind::Multiple`
    /// as a simple `message` or `error` parameter. It must be explicitly constructed.
    /// However, the example will show how the `yoshi!` macro *can be used to create the inner errors*.
    pub fn aggregate_with_macro_components() -> Yoshi {
        let errors = create_individual_errors();
        Yoshi::new(YoshiKind::Multiple {
            errors,
            primary_index: Some(1), // Email validation is primary
        })
        .context("Failed to register new user due to multiple issues.".to_string())
        .with_suggestion("Review all error messages and correct input fields. Try again.".to_string())
    }

    /// Aggregates multiple errors using direct Yoshi API calls.
    pub fn aggregate_with_api() -> Yoshi {
        let errors = vec![
            Yoshi::new(YoshiKind::Validation {
                field: "username".into(),
                message: "Username already taken".into(),
                expected: None, actual: Some("existing_user".into())
            }),
            Yoshi::new(YoshiKind::Validation {
                field: "email".into(),
                message: "Invalid email format".into(),
                expected: Some("user@domain.com".into()), actual: Some("invalid".into())
            }),
            Yoshi::new(YoshiKind::Internal {
                message: "Internal processing error for profile creation".into(),
                source: None, component: Some("ProfileService".into())
            }),
        ];
        Yoshi::new(YoshiKind::Multiple {
            errors,
            primary_index: Some(1),
        })
        .context("Failed to register new user due to multiple issues.".to_string())
        .with_suggestion("Review all error messages and correct input fields. Try again.".to_string())
    }
}

/// Example 3: Using a custom error defined with `YoshiError` derive.
///
/// This demonstrates how a custom enum, annotated with `#[derive(YoshiError)]`,
/// integrates seamlessly with the Yoshi ecosystem, providing `From<CustomError> for Yoshi`
/// conversions.
#[cfg(feature = "derive")] // This module only compiles if `derive` feature is enabled
mod example_3_custom_derive_error {
    use super::*;

    // A custom error enum using YoshiError derive
    #[derive(Debug, YoshiError)]
    #[yoshi(error_code_prefix = "APP")]
    pub enum MyAppError {
        #[yoshi(display = "Failed to load {resource}: {message}")]
        #[yoshi(kind = "NotFound")]
        #[yoshi(error_code = 1001)]
        ResourceLoadError {
            resource: String,
            message: String,
            #[yoshi(context = "source_path")]
            path: String,
        },
        #[yoshi(display = "Database transaction failed: {operation}")]
        #[yoshi(kind = "Network")] // Mapping DB errors to Network kind
        #[yoshi(error_code = 1002)]
        #[yoshi(transient = true)]
        DbTransactionError {
            operation: String,
            #[yoshi(source)]
            source_err: std::io::Error, // Example of using std::io::Error as source
            #[yoshi(payload)]
            transaction_id: String,
        },
    }

    /// Creates and converts a custom derived error using direct enum construction.
    pub fn create_and_convert_with_derive() -> Yoshi {
        let custom_err = MyAppError::ResourceLoadError {
            resource: "User configuration".to_string(),
            message: "File not found".to_string(),
            path: "/home/user/.config/app.json".to_string(),
        };
        // The `YoshiError` derive automatically implements `From<MyAppError> for Yoshi`
        let yoshi_err: Yoshi = custom_err.into();
        yoshi_err.context("During application startup".to_string())
    }

    /// Creates and converts another custom derived error with a nested `std::io::Error`.
    pub fn create_and_convert_db_error() -> Yoshi {
        let io_err = std::io::Error::new(std::io::ErrorKind::TimedOut, "DB server did not respond");
        let custom_db_err = MyAppError::DbTransactionError {
            operation: "INSERT new record".to_string(),
            source_err: io_err,
            transaction_id: "tx_abc_123".to_string(),
        };
        let yoshi_err: Yoshi = custom_db_err.into();
        yoshi_err.with_suggestion("Check database server status and network connectivity.".to_string())
    }
}

/// Example 4: Accessing contexts, metadata, and typed payloads from an error.
///
/// This shows how to programmatically inspect the detailed information within a Yoshi error.
mod example_4_detailed_introspection {
    use super::*;
    use std::any::Any; // For Any::type_name()

    /// Creates a complex error for introspection.
    fn create_complex_error() -> Yoshi {
        yoshi!(kind: YoshiKind::Internal {
            message: "Service processing pipeline failed".into(),
            source: None,
            component: Some("DataPipeline".into()),
        },
        with_metadata = ("pipeline_id", "pipe_001"),
        with_suggestion = "Inspect pipeline logs for details.",
        with_payload = CustomErrorState { retries: 3, service: "ProcessorA".to_string() }
        )
        .context("Error occurred during data transformation step.".to_string())
        .with_metadata("step", "transformation")
        .with_payload(vec![10, 20, 30]) // Another payload
        .context("Input validation failed before transformation.".to_string())
        .with_priority(250) // High priority for this context
        .with_metadata("validation_rule", "format_check")
    }

    /// Introspects a complex error created with `yoshi!` macro.
    pub fn introspect_with_macro_created_error() {
        let error = create_complex_error();

        println!("\n--- Introspection of macro-created error ---");
        println!("Error Display: {}", error);
        println!("Error Debug: {:?}", error);
        println!("Instance ID: {}", error.instance_id());
        println!("Severity: {}", error.severity());
        println!("Is Transient: {}", error.is_transient());

        // Accessing primary context (highest priority)
        if let Some(primary_ctx) = error.primary_context() {
            println!("\nPrimary Context (Priority {}):", primary_ctx.priority);
            println!("  Message: {:?}", primary_ctx.message.as_deref());
            if let Some(loc) = primary_ctx.location {
                println!("  Location: {}", loc);
            }
            println!("  Metadata: {:?}", primary_ctx.metadata);
            println!("  Suggestion: {:?}", primary_ctx.suggestion.as_deref());

            // Accessing typed payloads in primary context
            if let Some(state) = primary_ctx.payload::<CustomErrorState>() {
                println!("  Payload (CustomErrorState): {:?}", state);
            }
        }

        // Iterating through all contexts (in order of addition, or sorted by priority by iterators)
        println!("\nAll Contexts:");
        for (i, ctx) in error.contexts().rev().enumerate() { // .rev() to show in display order
            println!("  Context {}: Message={:?}, Priority={}", i, ctx.message.as_deref(), ctx.priority);
            if let Some(loc) = ctx.location {
                println!("    Location: {}", loc);
            }
            if !ctx.metadata.is_empty() {
                println!("    Metadata: {:?}", ctx.metadata);
            }
            if !ctx.payloads.is_empty() {
                println!("    Payloads (raw): {} items", ctx.payloads.len());
                for (p_idx, payload_arc) in ctx.payloads.iter().enumerate() {
                    println!("      Payload {}: Type={}, Value={:?}", p_idx, payload_arc.type_id(), payload_arc);
                }
            }
        }

        // Accessing any payload across all contexts (via convenience method)
        if let Some(vec_payload) = error.payload::<Vec<i32>>() {
            println!("\nFound Vec<i32> payload anywhere: {:?}", vec_payload);
        }
    }

    /// Introspects a complex error created using direct API calls.
    pub fn introspect_with_api_created_error() {
        let error = Yoshi::new(YoshiKind::Internal {
            message: "Service processing pipeline failed".into(),
            source: None,
            component: Some("DataPipeline".into()),
        })
        .with_metadata("pipeline_id", "pipe_001".to_string())
        .with_suggestion("Inspect pipeline logs for details.".to_string())
        .with_payload(CustomErrorState { retries: 3, service: "ProcessorA".to_string() })
        .context("Error occurred during data transformation step.".to_string())
        .with_metadata("step", "transformation".to_string())
        .with_payload(vec![10, 20, 30]) // Another payload
        .context("Input validation failed before transformation.".to_string())
        .with_priority(250) // High priority for this context
        .with_metadata("validation_rule", "format_check".to_string());

        // Same introspection logic as above, demonstrating API consistency
        println!("\n--- Introspection of API-created error ---");
        println!("Error Display: {}", error);
        println!("Error Debug: {:?}", error);
        println!("Instance ID: {}", error.instance_id());
        println!("Severity: {}", error.severity());
        println!("Is Transient: {}", error.is_transient());

        // Accessing primary context (highest priority)
        if let Some(primary_ctx) = error.primary_context() {
            println!("\nPrimary Context (Priority {}):", primary_ctx.priority);
            println!("  Message: {:?}", primary_ctx.message.as_deref());
            if let Some(loc) = primary_ctx.location {
                println!("  Location: {}", loc);
            }
            println!("  Metadata: {:?}", primary_ctx.metadata);
            println!("  Suggestion: {:?}", primary_ctx.suggestion.as_deref());

            // Accessing typed payloads in primary context
            if let Some(state) = primary_ctx.payload::<CustomErrorState>() {
                println!("  Payload (CustomErrorState): {:?}", state);
            }
        }

        // Iterating through all contexts (in order of addition, or sorted by priority by iterators)
        println!("\nAll Contexts:");
        for (i, ctx) in error.contexts().rev().enumerate() { // .rev() to show in display order
            println!("  Context {}: Message={:?}, Priority={}", i, ctx.message.as_deref(), ctx.priority);
            if let Some(loc) = ctx.location {
                println!("    Location: {}", loc);
            }
            if !ctx.metadata.is_empty() {
                println!("    Metadata: {:?}", ctx.metadata);
            }
            if !ctx.payloads.is_empty() {
                println!("    Payloads (raw): {} items", ctx.payloads.len());
                for (p_idx, payload_arc) in ctx.payloads.iter().enumerate() {
                    println!("      Payload {}: Type={}, Value={:?}", p_idx, payload_arc.type_id(), payload_arc);
                }
            }
        }

        // Accessing any payload across all contexts (via convenience method)
        if let Some(vec_payload) = error.payload::<Vec<i32>>() {
            println!("\nFound Vec<i32> payload anywhere: {:?}", vec_payload);
        }
    }
}

/// Example 5: Demonstrating error recovery strategy.
///
/// This shows how to attach a specific `RecoveryStrategy` as a typed payload
/// to an error, allowing higher-level error handlers to react appropriately.
mod example_5_error_recovery_strategy {
    use super::*;

    /// Creates an error with a recovery strategy using the `yoshi!` macro.
    pub fn create_with_macro() -> Yoshi {
        yoshi!(kind: YoshiKind::Network {
            message: "Third-party service unavailable".into(),
            source: None,
            error_code: Some(503),
        },
        with_payload = RecoveryStrategy::RetryWithDelay(Duration::from_secs(5)))
    }

    /// Creates an error with a recovery strategy using direct API calls.
    pub fn create_with_api() -> Yoshi {
        Yoshi::new(YoshiKind::Network {
            message: "Third-party service unavailable".into(),
            source: None,
            error_code: Some(503),
        })
        .with_payload(RecoveryStrategy::RetryWithDelay(Duration::from_secs(5)))
    }

    /// Consumes the error and attempts recovery.
    pub fn handle_error(error: Yoshi) {
        if let Some(strategy) = error.payload::<RecoveryStrategy>() {
            match strategy {
                RecoveryStrategy::RetryWithDelay(delay) => {
                    println!("\nError suggests retry after: {:?}", delay);
                    // Simulate retry logic
                }
                RecoveryStrategy::FallbackToCache => {
                    println!("\nError suggests falling back to cache.");
                    // Simulate fallback logic
                }
                RecoveryStrategy::ManualIntervention => {
                    println!("\nError requires manual intervention.");
                    // Alert human operator
                }
            }
        } else {
            println!("\nNo specific recovery strategy found for error: {}", error);
        }
    }
}


// Main function to run examples (for testing/demonstration)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1_chained_yoshi_errors() {
        let err1 = example_1_chained_yoshi_errors::create_chained_with_macro();
        assert!(format!("{}", err1).contains("User login failed unexpectedly"));
        assert!(format!("{}", err1).contains("Failed to communicate with authentication service"));
        assert!(format!("{}", err1).contains("TCP connection reset by peer"));
        // Check metadata from original creation
        assert!(err1.primary_context().unwrap().metadata.get(&"user_id".into()).is_some());

        let err2 = example_1_chained_yoshi_errors::create_chained_with_api();
        assert!(format!("{}", err2).contains("User login failed unexpectedly"));
        assert!(format!("{}", err2).contains("Failed to communicate with authentication service"));
        assert!(format!("{}", err2).contains("TCP connection reset by peer"));
        assert!(err2.primary_context().unwrap().metadata.get(&"user_id".into()).is_some());
    }

    #[test]
    fn test_example_2_multiple_errors() {
        let err1 = example_2_multiple_errors::aggregate_with_macro_components();
        assert!(matches!(err1.kind(), YoshiKind::Multiple { .. }));
        assert!(format!("{}", err1).contains("Multiple errors (3 total)"));
        assert!(format!("{}", err1).contains("email"));
        assert!(err1.primary_context().unwrap().suggestion.is_some());

        let err2 = example_2_multiple_errors::aggregate_with_api();
        assert!(matches!(err2.kind(), YoshiKind::Multiple { .. }));
        assert!(format!("{}", err2).contains("Multiple errors (3 total)"));
        assert!(format!("{}", err2).contains("email"));
        assert!(err2.primary_context().unwrap().suggestion.is_some());
    }

    #[test]
    #[cfg(feature = "derive")]
    fn test_example_3_custom_derive_error() {
        let err1 = example_3_custom_derive_error::create_and_convert_with_derive();
        assert!(matches!(err1.kind(), YoshiKind::NotFound { .. }));
        assert!(format!("{}", err1).contains("APP-1001")); // Check for error code prefix
        assert!(err1.primary_context().unwrap().metadata.get(&"source_path".into()).is_some());

        let err2 = example_3_custom_derive_error::create_and_convert_db_error();
        assert!(matches!(err2.kind(), YoshiKind::Network { .. }));
        assert!(err2.is_transient());
        assert!(err2.suggestion().is_some());
        assert!(err2.payload::<String>().is_some()); // Check for transaction_id payload
    }

    #[test]
    fn test_example_4_detailed_introspection() {
        // This test primarily relies on manual inspection of `println!` output
        // for comprehensive verification, but basic checks are added.
        let error_macro = example_4_detailed_introspection::create_complex_error();
        assert!(error_macro.primary_context().unwrap().metadata.get(&"pipeline_id".into()).is_some());
        assert!(error_macro.payload::<CustomErrorState>().is_some());
        assert!(error_macro.payload::<Vec<i32>>().is_some());

        let error_api = example_4_detailed_introspection::create_complex_error();
        assert!(error_api.primary_context().unwrap().metadata.get(&"pipeline_id".into()).is_some());
        assert!(error_api.payload::<CustomErrorState>().is_some());
        assert!(error_api.payload::<Vec<i32>>().is_some());

        // For full demo, uncomment these and run `cargo test -- --nocapture`
        // example_4_detailed_introspection::introspect_with_macro_created_error();
        // example_4_detailed_introspection::introspect_with_api_created_error();
    }

    #[test]
    fn test_example_5_error_recovery_strategy() {
        let err_macro = example_5_error_recovery_strategy::create_with_macro();
        let strategy_macro = err_macro.payload::<RecoveryStrategy>().unwrap();
        assert_eq!(*strategy_macro, RecoveryStrategy::RetryWithDelay(Duration::from_secs(5)));
        example_5_error_recovery_strategy::handle_error(err_macro);

        let err_api = example_5_error_recovery_strategy::create_with_api();
        let strategy_api = err_api.payload::<RecoveryStrategy>().unwrap();
        assert_eq!(*strategy_api, RecoveryStrategy::RetryWithDelay(Duration::from_secs(5)));
        example_5_error_recovery_strategy::handle_error(err_api);
    }
}