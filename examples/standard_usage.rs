/* examples/standard_usage.rs */
#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
//! **Brief:** Demonstrates standard usage patterns for the Yoshi error handling framework.
//!
//! This module provides basic examples of creating, propagating, and enriching errors
//! using the `yoshi` crate. It showcases both the convenient `yoshi!` macro and
//! the underlying direct API calls for clarity.
//!
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Standard Error Usage Patterns]
//!  - [Basic Error Creation: Demonstrates internal and not-found errors]
//!  - [Error Propagation: Simple Result chaining with context]
//!  - [Foreign Error Integration: Wrapping external `std::error::Error` types]
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

use yoshi::{yoshi, Result, Yoshi, YoshiKind, YoshiContextExt};
use std::io::{self, ErrorKind};

/// Example 1: Creating a basic internal error.
///
/// This example demonstrates the simplest form of error creation for an
/// internal application fault.
mod example_1_basic_internal_error {
    use super::*;

    /// Creates an internal error using the `yoshi!` macro.
    ///
    /// The macro automatically infers `YoshiKind::Internal` and captures
    /// the source code location.
    pub fn create_with_macro() -> Yoshi {
        yoshi!(message: "Something went wrong inside the system.")
    }

    /// Creates an internal error using direct Yoshi API calls.
    ///
    /// This shows the explicit construction of `YoshiKind::Internal` and
    /// `Yoshi::new`.
    pub fn create_with_api() -> Yoshi {
        Yoshi::new(YoshiKind::Internal {
            message: "Something went wrong inside the system.".into(),
            source: None,
            component: None,
        })
    }
}

/// Example 2: Creating a resource not found error.
///
/// This example shows how to create a structured error for a common
/// scenario where a requested resource cannot be found.
mod example_2_not_found_error {
    use super::*;

    /// Creates a not found error using the `yoshi!` macro.
    ///
    /// The macro allows specifying the exact `YoshiKind` variant and its fields.
    pub fn create_with_macro() -> Yoshi {
        yoshi!(kind: YoshiKind::NotFound {
            resource_type: "User Profile".into(),
            identifier: "john_doe".into(),
            search_locations: Some(vec!["/db/users".into(), "/cache/users".into()]),
        })
    }

    /// Creates a not found error using direct Yoshi API calls.
    ///
    /// This demonstrates explicit construction of the `YoshiKind::NotFound` variant.
    pub fn create_with_api() -> Yoshi {
        Yoshi::new(YoshiKind::NotFound {
            resource_type: "User Profile".into(),
            identifier: "john_doe".into(),
            search_locations: Some(vec!["/db/users".into(), "/cache/users".into()]),
        })
    }
}

/// Example 3: Propagating an `std::io::Error` with context.
///
/// This example demonstrates converting a standard library error into a Yoshi
/// error and adding contextual information as it propagates.
mod example_3_io_error_propagation {
    use super::*;

    /// Simulates an I/O operation that fails.
    fn simulated_file_read() -> io::Result<String> {
        Err(io::Error::new(ErrorKind::PermissionDenied, "file system access denied"))
    }

    /// Propagates an I/O error with context using the `yoshi!` macro and `YoshiContextExt`.
    ///
    /// The `yoshi!` macro wraps the `io::Error`, and `.context()` adds additional
    /// information to the propagating error.
    pub fn propagate_with_macro() -> Result<String> {
        simulated_file_read()
            .map_err(|e| yoshi!(error: e))
            .context("Failed to load application configuration.".to_string())
            .meta("config_path", "/etc/app/config.json".to_string())
    }

    /// Propagates an I/O error with context using direct Yoshi API calls.
    ///
    /// This explicitly converts `io::Error` to `Yoshi` using `From` and then
    /// uses the `YoshiContextExt` trait methods for chaining.
    pub fn propagate_with_api() -> Result<String> {
        simulated_file_read()
            .map_err(Yoshi::from) // Explicitly convert using From trait
            .context("Failed to load application configuration.".to_string())
            .meta("config_path", "/etc/app/config.json".to_string())
    }
}

// Main function to run examples (for testing/demonstration)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1_basic_internal_error() {
        let err1 = example_1_basic_internal_error::create_with_macro();
        assert!(matches!(err1.kind(), YoshiKind::Internal { .. }));
        assert!(format!("{}", err1).contains("internal"));

        let err2 = example_1_basic_internal_error::create_with_api();
        assert!(matches!(err2.kind(), YoshiKind::Internal { .. }));
        assert!(format!("{}", err2).contains("internal"));
    }

    #[test]
    fn test_example_2_not_found_error() {
        let err1 = example_2_not_found_error::create_with_macro();
        assert!(matches!(err1.kind(), YoshiKind::NotFound { .. }));
        assert!(format!("{}", err1).contains("User Profile not found: john_doe"));

        let err2 = example_2_not_found_error::create_with_api();
        assert!(matches!(err2.kind(), YoshiKind::NotFound { .. }));
        assert!(format!("{}", err2).contains("User Profile not found: john_doe"));
    }

    #[test]
    fn test_example_3_io_error_propagation() {
        let res1 = example_3_io_error_propagation::propagate_with_macro();
        assert!(res1.is_err());
        let err1 = res1.unwrap_err();
        assert!(format!("{}", err1).contains("Permission denied"));
        assert!(format!("{}", err1).contains("Failed to load application configuration."));
        assert!(err1.primary_context().unwrap().metadata.get(&"config_path".into()).is_some());

        let res2 = example_3_io_error_propagation::propagate_with_api();
        assert!(res2.is_err());
        let err2 = res2.unwrap_err();
        assert!(format!("{}", err2).contains("Permission denied"));
        assert!(format!("{}", err2).contains("Failed to load application configuration."));
        assert!(err2.primary_context().unwrap().metadata.get(&"config_path".into()).is_some());
    }
}