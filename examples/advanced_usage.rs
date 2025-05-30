/* examples/advanced_usage.rs */
#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
//! **Brief:** Demonstrates advanced usage patterns for the Yoshi error handling framework.
//!
//! This module delves into more complex error types, rich contextualization, and
//! basic error analysis capabilities provided by the `yoshi` crate. Examples are
//! presented with both the `yoshi!` macro and direct API calls.
//!
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Advanced Error Usage Patterns]
//!  - [Validation Error: Detailed input validation with expected/actual values]
//!  - [Timeout Error: Capturing operation duration and suggestions]
//!  - [Rich Contextualization: Attaching multiple metadata and custom payloads]
//!  - [Error Priority: Setting severity for different contexts]
//!  - [Basic Analysis: Checking error severity and transience]
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

use yoshi::{yoshi, Result, Yoshi, YoshiKind, YoshiContext, YoshiContextExt};
use std::time::Duration;
use std::sync::Arc; // Needed for Arc<str> in YoshiKind fields

/// Custom struct to use as a payload.
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

    /// Creates a validation error using the `yoshi!` macro.
    ///
    /// Shows specifying the `YoshiKind` and chaining metadata/suggestion directly.
    pub fn create_with_macro() -> Yoshi {
        yoshi!(kind: YoshiKind::Validation {
            field: "password".into(),
            message: "Password does not meet complexity requirements.".into(),
            expected: Some("Minimum 8 characters, 1 uppercase, 1 number".into()),
            actual: Some("shortpass".into()),
        },
        with_suggestion = "Choose a stronger password containing diverse characters.")
    }

    /// Creates a validation error using direct Yoshi API calls.
    ///
    /// Demonstrates explicit `YoshiKind::Validation` creation and chaining.
    pub fn create_with_api() -> Yoshi {
        Yoshi::new(YoshiKind::Validation {
            field: "password".into(),
            message: "Password does not meet complexity requirements.".into(),
            expected: Some("Minimum 8 characters, 1 uppercase, 1 number".into()),
            actual: Some("shortpass".into()),
        })
        .with_suggestion("Choose a stronger password containing diverse characters.".to_string())
    }
}

/// Example 2: Creating a timeout error with custom duration and suggestion.
///
/// This example focuses on the `YoshiKind::Timeout` variant and demonstrates
/// adding specific timing information and a recovery suggestion.
mod example_2_timeout_with_suggestion {
    use super::*;

    /// Creates a timeout error using the `yoshi!` macro.
    ///
    /// Shows specifying `YoshiKind::Timeout` with a duration and a suggestion.
    pub fn create_with_macro() -> Yoshi {
        yoshi!(kind: YoshiKind::Timeout {
            operation: "External API call".into(),
            duration: Duration::from_secs(5),
            expected_max: Some(Duration::from_secs(3)),
        },
        with_suggestion = "Retry the API call after a short delay or check network connectivity.")
    }

    /// Creates a timeout error using direct Yoshi API calls.
    ///
    /// Demonstrates explicit `YoshiKind::Timeout` construction and chaining.
    pub fn create_with_api() -> Yoshi {
        Yoshi::new(YoshiKind::Timeout {
            operation: "External API call".into(),
            duration: Duration::from_secs(5),
            expected_max: Some(Duration::from_secs(3)),
        })
        .with_suggestion("Retry the API call after a short delay or check network connectivity.".to_string())
    }
}

/// Example 3: Attaching a custom struct as a payload and setting error priority.
///
/// This example demonstrates how to embed arbitrary typed data (payloads)
/// within an error's context and assign a priority level.
mod example_3_payload_and_priority {
    use super::*;

    /// Creates an error with a custom payload and priority using the `yoshi!` macro.
    ///
    /// The payload and priority are applied to the initial context created by the macro.
    pub fn create_with_macro() -> Yoshi {
        yoshi!(message: "Request processing failed due to an unexpected state.",
            with_payload = RequestInfo {
                id: "req_xyz_789".to_string(),
                method: "POST".to_string(),
                path: "/api/v1/data".to_string(),
            },
            with_priority = 200 // Higher priority for critical issues
        )
    }

    /// Creates an error with a custom payload and priority using direct API calls.
    ///
    /// Shows explicit `YoshiContext` creation and applying payload and priority.
    pub fn create_with_api() -> Yoshi {
        Yoshi::new(YoshiKind::Internal {
            message: "Request processing failed due to an unexpected state.".into(),
            source: None,
            component: None,
        })
        .with_payload(RequestInfo {
            id: "req_xyz_789".to_string(),
            method: "POST".to_string(),
            path: "/api/v1/data".to_string(),
        })
        .with_priority(200) // Higher priority for critical issues
    }
}

/// Example 4: Basic error analysis.
///
/// This example demonstrates how to retrieve intrinsic properties of a Yoshi
/// error, such as its severity and transience.
mod example_4_basic_analysis {
    use super::*;

    /// Analyzes an error created with the `yoshi!` macro.
    pub fn analyze_with_macro() -> (u8, bool) {
        let err = yoshi!(kind: YoshiKind::Network {
            message: "Connection lost temporarily.".into(),
            source: None,
            error_code: Some(10053),
        });

        // Accessing properties directly
        (err.severity(), err.is_transient())
    }

    /// Analyzes an error created with direct API calls.
    pub fn analyze_with_api() -> (u8, bool) {
        let err = Yoshi::new(YoshiKind::Network {
            message: "Connection lost temporarily.".into(),
            source: None,
            error_code: Some(10053),
        });

        // Accessing properties directly
        (err.severity(), err.is_transient())
    }
}


// Main function to run examples (for testing/demonstration)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1_detailed_validation_error() {
        let err1 = example_1_detailed_validation_error::create_with_macro();
        assert!(matches!(err1.kind(), YoshiKind::Validation { .. }));
        assert!(format!("{}", err1).contains("Password does not meet complexity requirements."));
        assert!(err1.suggestion().is_some());

        let err2 = example_1_detailed_validation_error::create_with_api();
        assert!(matches!(err2.kind(), YoshiKind::Validation { .. }));
        assert!(format!("{}", err2).contains("Password does not meet complexity requirements."));
        assert!(err2.suggestion().is_some());
    }

    #[test]
    fn test_example_2_timeout_with_suggestion() {
        let err1 = example_2_timeout_with_suggestion::create_with_macro();
        assert!(matches!(err1.kind(), YoshiKind::Timeout { .. }));
        assert!(err1.suggestion().is_some());
        assert!(err1.is_transient());

        let err2 = example_2_timeout_with_suggestion::create_with_api();
        assert!(matches!(err2.kind(), YoshiKind::Timeout { .. }));
        assert!(err2.suggestion().is_some());
        assert!(err2.is_transient());
    }

    #[test]
    fn test_example_3_payload_and_priority() {
        let err1 = example_3_payload_and_priority::create_with_macro();
        assert!(err1.primary_context().unwrap().priority == 200);
        assert!(err1.payload::<RequestInfo>().is_some());
        assert_eq!(err1.payload::<RequestInfo>().unwrap().id, "req_xyz_789");

        let err2 = example_3_payload_and_priority::create_with_api();
        assert!(err2.primary_context().unwrap().priority == 200);
        assert!(err2.payload::<RequestInfo>().is_some());
        assert_eq!(err2.payload::<RequestInfo>().unwrap().id, "req_xyz_789");
    }

    #[test]
    fn test_example_4_basic_analysis() {
        let (severity_macro, transient_macro) = example_4_basic_analysis::analyze_with_macro();
        assert_eq!(severity_macro, 50); // Network error severity
        assert!(transient_macro); // Network errors are transient

        let (severity_api, transient_api) = example_4_basic_analysis::analyze_with_api();
        assert_eq!(severity_api, 50);
        assert!(transient_api);
    }
}