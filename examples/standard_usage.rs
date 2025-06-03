/* examples/standard_usage.rs */
#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
//! **Brief:** Demonstrates standard usage patterns for the Yoshi error handling framework.
//!
//! This module provides basic examples of creating, propagating, and enriching errors
//! using the `yoshi_std` crate. It showcases the new `Hatch` ecosystem and direct API usage.
//!
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Standard Error Usage Patterns]
//!  - [Basic Error Creation: Demonstrates internal and not-found errors]
//!  - [Error Propagation: Simple Result chaining with context using Hatch ecosystem]
//!  - [Foreign Error Integration: Wrapping external `std::error::Error` types]
//!  - [New Thematic Methods: Using lay(), laytext(), nest(), yum!]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Author:** Lord Xyn

use std::io::{self, ErrorKind};
use yoshi_std::{yum, Hatch, HatchExt, Hatchable, LayContext, Yoshi, YoshiKind};

/// Example 1: Creating a basic internal error.
///
/// This example demonstrates the simplest form of error creation for an
/// internal application fault using the updated API.
mod example_1_basic_internal_error {
    use super::*;

    /// Creates an internal error using direct Yoshi API calls.
    pub fn create_with_api() -> Yoshi {
        Yoshi::new(YoshiKind::Internal {
            message: "Something went wrong inside the system.".into(),
            source: None,
            component: Some("CoreService".into()),
        })
    }

    /// Creates an internal error and returns as Hatch<T>.
    pub fn create_as_hatch() -> Hatch<String> {
        Err(Yoshi::new(YoshiKind::Internal {
            message: "System initialization failed.".into(),
            source: None,
            component: Some("InitService".into()),
        }))
    }
}

/// Example 2: Creating a resource not found error.
///
/// This example shows how to create a structured error for a common
/// scenario where a requested resource cannot be found.
mod example_2_not_found_error {
    use super::*;

    /// Creates a not found error using direct Yoshi API calls.
    pub fn create_with_api() -> Yoshi {
        Yoshi::new(YoshiKind::NotFound {
            resource_type: "User Profile".into(),
            identifier: "john_doe".into(),
            search_locations: Some(vec!["/db/users".into(), "/cache/users".into()]),
        })
    }

    /// Creates a not found error as Hatch with context.
    pub fn create_as_hatch_with_context() -> Hatch<String> {
        Err(Yoshi::new(YoshiKind::NotFound {
            resource_type: "Configuration File".into(),
            identifier: "app.config".into(),
            search_locations: Some(vec!["/etc/app".into(), "~/.config/app".into()]),
        }))
        .lay("During application startup configuration loading")
    }
}

/// Example 3: Propagating an `std::io::Error` with context using new Hatch ecosystem.
///
/// This example demonstrates converting a standard library error into a Yoshi
/// error and adding contextual information using the new thematic methods.
mod example_3_io_error_propagation {
    use super::*;

    /// Simulates an I/O operation that fails.
    fn simulated_file_read() -> io::Result<String> {
        Err(io::Error::new(
            ErrorKind::PermissionDenied,
            "file system access denied",
        ))
    }

    /// Propagates an I/O error with context using new Hatch ecosystem.
    pub fn propagate_with_hatch_ecosystem() -> Hatch<String> {
        simulated_file_read()
            .hatch() // Convert to Hatch<T>
            .lay("Failed to load application configuration")
            .meta("config_path", "/etc/app/config.json")
            .help("Check file permissions and ensure the file exists")
    }

    /// Propagates an I/O error with context using direct API.
    pub fn propagate_with_api() -> Hatch<String> {
        simulated_file_read()
            .map_err(Yoshi::from)
            .context("Failed to load application configuration")
            .meta("config_path", "/etc/app/config.json")
    }

    /// Demonstrates thematic error methods.
    pub fn demonstrate_thematic_methods() -> Hatch<String> {
        let result = simulated_file_read().hatch();

        if let Err(error) = result {
            // Use new thematic methods
            let enhanced_error = error
                .lay("During configuration file processing")
                .with_suggestion("Verify file permissions and path");

            // Demonstrate yum! macro for debugging
            let debug_error = yum!(enhanced_error);

            // Show laytext and nest access
            println!("Context message: {:?}", debug_error.laytext());
            println!("Nested error: {:?}", debug_error.nest());

            return Err(debug_error);
        }

        Ok("Success".to_string())
    }
}

/// Example 4: Using the new Hatch ecosystem features.
///
/// This example showcases the new type aliases and traits.
mod example_4_hatch_ecosystem {
    use super::*;

    /// Demonstrates Hatch<T> type alias and LayContext trait.
    pub fn process_data(input: &str) -> Hatch<u32> {
        if input.is_empty() {
            return Err(Yoshi::new(YoshiKind::Validation {
                field: "input".into(),
                message: "Input cannot be empty".into(),
                expected: Some("non-empty string".into()),
                actual: Some("empty string".into()),
            }))
            .lay("Input validation failed during data processing");
        }

        // Simulate parsing that might fail
        input
            .parse::<u32>()
            .map_err(|e| e.to_string()) // Convert ParseIntError to String
            .hatch() // Convert to Hatch<u32>
            .lay("Failed to parse input as number")
            .help("Ensure input contains only numeric characters")
    }

    /// Demonstrates error chaining with thematic methods.
    pub fn complex_operation() -> Hatch<String> {
        let number = process_data("not_a_number")?;
        Ok(format!("Processed: {}", number))
    }

    /// Demonstrates the yum! macro for debugging.
    pub fn debug_errors() {
        match complex_operation() {
            Ok(result) => println!("Success: {}", result),
            Err(error) => {
                // yum! macro provides enhanced debug output
                let consumed_error = yum!(error);
                println!("Operation failed - see debug output above");
                println!("Error had {} contexts", consumed_error.contexts().count());
            }
        }
    }
}

// Test suite
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1_basic_internal_error() {
        let err = example_1_basic_internal_error::create_with_api();
        assert!(matches!(err.kind(), YoshiKind::Internal { .. }));
        assert!(format!("{}", err).contains("Something went wrong inside the system"));

        let hatch_result = example_1_basic_internal_error::create_as_hatch();
        assert!(hatch_result.is_err());
        let err = hatch_result.unwrap_err();
        assert!(matches!(err.kind(), YoshiKind::Internal { .. }));
    }

    #[test]
    fn test_example_2_not_found_error() {
        let err = example_2_not_found_error::create_with_api();
        assert!(matches!(err.kind(), YoshiKind::NotFound { .. }));
        assert!(format!("{}", err).contains("User Profile not found: john_doe"));

        let hatch_result = example_2_not_found_error::create_as_hatch_with_context();
        assert!(hatch_result.is_err());
        let err = hatch_result.unwrap_err();
        assert!(err.laytext().is_some());
        assert!(err.laytext().unwrap().contains("startup configuration"));
    }

    #[test]
    fn test_example_3_io_error_propagation() {
        let result = example_3_io_error_propagation::propagate_with_hatch_ecosystem();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(format!("{}", err).contains("Permission denied"));
        assert!(err.laytext().unwrap().contains("configuration"));
        assert!(err.suggestion().is_some());

        let result2 = example_3_io_error_propagation::propagate_with_api();
        assert!(result2.is_err());
        let err2 = result2.unwrap_err();
        assert!(format!("{}", err2).contains("Permission denied"));
    }

    #[test]
    fn test_example_4_hatch_ecosystem() {
        let result = example_4_hatch_ecosystem::process_data("");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err.kind(), YoshiKind::Validation { .. }));
        assert!(err.laytext().is_some());

        let result2 = example_4_hatch_ecosystem::process_data("not_a_number");
        assert!(result2.is_err());
        let err2 = result2.unwrap_err();
        assert!(err2.laytext().unwrap().contains("parse input"));

        let result3 = example_4_hatch_ecosystem::complex_operation();
        assert!(result3.is_err());

        // Test debug functionality (just ensure it doesn't panic)
        example_4_hatch_ecosystem::debug_errors();
    }
}
