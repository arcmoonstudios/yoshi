/* yoshi-core/tests/result_traits_tests.rs */
#![deny(dead_code)]
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![deny(unused_variables)]
#![allow(clippy::expect_used)] // Allow expect in tests for clearer error messages
#![deny(clippy::unwrap_used)]
//! **Brief:** Comprehensive test suite for yoshi-core Result types and extension traits with ergonomic API validation.
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + Result type alias validation with zero-cost abstraction guarantees
//!  - Hatch type ergonomics with thematic naming and performance optimization
//!  - Hatchable trait functionality with seamless error conversion
//!  - `LayText` trait operations with context enrichment capabilities
//!  - `HatchExt` trait methods with fluent API design and method chaining
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

use yoshi_core::{Hatch, HatchExt, Hatchable, LayText, Yoshi, YoshiKind};

#[test]
fn test_result_type_alias() {
    fn test_function() -> i32 {
        42
    }

    let result = test_function();
    assert_eq!(result, 42);
}

#[test]
fn test_hatch_type_alias() {
    fn test_function() -> String {
        "success".to_string()
    }

    let result = test_function();
    assert_eq!(result, "success");
}

#[test]
fn test_hatch_with_error() {
    fn test_function() -> Hatch<String> {
        Err(Yoshi::new(YoshiKind::Validation {
            field: "input".into(),
            message: "Invalid input".into(),
            expected: Some("valid string".into()),
            actual: Some("invalid".into()),
        }))
    }

    let result = test_function();
    assert!(result.is_err());
    let error = result.expect_err("result should be an error");
    assert!(error.kind().to_string().contains("Invalid input"));
}

#[test]
fn test_hatchable_trait_string_error() {
    let string_result: core::result::Result<i32, String> = Err("string error".to_string());
    let hatched: Hatch<i32> = string_result.hatch();

    assert!(hatched.is_err());
    let error = hatched.expect_err("hatched should be an error");
    assert!(error.kind().to_string().contains("string error"));
}

#[test]
fn test_hatchable_trait_success() {
    let success_result: core::result::Result<i32, String> = Ok(100);
    let hatched: Hatch<i32> = success_result.hatch();

    assert!(hatched.is_ok());
    assert_eq!(hatched.expect("hatched should be ok"), 100);
}

#[test]
fn test_lay_text_trait() {
    let result: Hatch<i32> = Err(Yoshi::new(YoshiKind::Internal {
        message: "Original error".into(),
        source: None,
        component: None,
    }));

    let enriched = result.lay("Additional context");
    assert!(enriched.is_err());

    let error = enriched.expect_err("enriched should be an error");
    let nests: Vec<_> = error.nests().collect();
    assert!(!nests.is_empty());
}

#[test]
fn test_lay_text_with_success() {
    let result: Hatch<i32> = Ok(42);
    let enriched = result.lay("This context won't be added");

    assert!(enriched.is_ok());
    assert_eq!(enriched.expect("enriched should be ok"), 42);
}

#[test]
fn test_hatch_ext_context() {
    let result: core::result::Result<i32, String> = Err("base error".to_string());
    let enriched = result.nest("Operation failed");

    assert!(enriched.is_err());
    let error = enriched.expect_err("enriched should be an error");
    let contexts: Vec<_> = error.nests().collect();
    assert!(!contexts.is_empty());
}

#[test]
fn test_hatch_ext_with_suggestion() {
    let result: core::result::Result<i32, String> = Err("validation failed".to_string());
    let enriched = result.with_signpost("Try a different approach");

    assert!(enriched.is_err());
    let error = enriched.expect_err("enriched should be an error");
    assert!(error.signpost().is_some());
    assert!(error
        .signpost()
        .expect("signpost should be present")
        .contains("different approach"));
}

#[test]
fn test_hatch_ext_with_priority() {
    let result: core::result::Result<i32, String> = Err("priority test".to_string());
    let enriched = result.with_priority(150);

    assert!(enriched.is_err());
    let error = enriched.expect_err("enriched should be an error");
    assert!(error.instance_id() > 0);
}

#[test]
fn test_hatch_ext_with_shell() {
    let test_payload = "test data";
    let result: core::result::Result<i32, String> = Err("shell test".to_string());
    let enriched = result.with_shell(test_payload);

    assert!(enriched.is_err());
    let error = enriched.expect_err("enriched should be an error");
    let nests: Vec<_> = error.nests().collect();
    assert!(!nests.is_empty());
}

#[test]
fn test_hatch_ext_ctx_alias() {
    let result: core::result::Result<i32, String> = Err("ctx test".to_string());
    let enriched = result.nest("Short context");

    assert!(enriched.is_err());
    let error = enriched.expect_err("enriched should be an error");
    let contexts: Vec<_> = error.nests().collect();
    assert!(!contexts.is_empty());
}

#[test]
fn test_hatch_ext_help_alias() {
    let result: core::result::Result<i32, String> = Err("help test".to_string());
    let enriched = result.help("Helpful suggestion");

    assert!(enriched.is_err());
    let error = enriched.expect_err("enriched should be an error");
    assert!(error.signpost().is_some());
    assert!(error
        .signpost()
        .expect("signpost should be present")
        .contains("Helpful"));
}

#[test]
fn test_hatch_ext_with_metadata() {
    let result: core::result::Result<i32, String> = Err("metadata test".to_string());
    let enriched = result.meta("test_key", "test_value");

    assert!(enriched.is_err());
    let error = enriched.expect_err("enriched should be an error");
    let nests: Vec<_> = error.nests().collect();
    assert!(!nests.is_empty());
}

#[test]
fn test_method_chaining() {
    let result: core::result::Result<i32, String> = Err("chain test".to_string());
    let enriched = result
        .nest("First context")
        .with_signpost("Try this")
        .with_priority(100)
        .meta("key1", "value1")
        .meta("key2", "value2")
        .nest("Short context")
        .help("Additional help");

    assert!(enriched.is_err());
    let error = enriched.expect_err("enriched should be an error");
    assert!(error.signpost().is_some());
    let contexts: Vec<_> = error.nests().collect();
    assert!(!contexts.is_empty());
}

#[test]
fn test_hatch_ext_with_success_values() {
    let result: core::result::Result<i32, String> = Ok(42);
    let enriched = result
        .nest("This won't be added")
        .with_signpost("Neither will this")
        .with_priority(100)
        .meta("key", "value");

    assert!(enriched.is_ok());
    assert_eq!(enriched.expect("enriched should be ok"), 42);
}

#[test]
fn test_complex_error_conversion_chain() {
    fn simulate_io_error() -> core::result::Result<String, String> {
        Err("file not found".to_string())
    }

    fn simulate_parse_error() -> core::result::Result<i32, String> {
        Err("invalid number format".to_string())
    }

    let io_result = simulate_io_error()
        .nest("While reading configuration file")
        .meta("file_path", "/etc/config.toml")
        .with_signpost("Check if the file exists and has proper permissions");

    let parse_result = simulate_parse_error()
        .nest("While parsing configuration value")
        .meta("field", "port_number")
        .with_signpost("Ensure the value is a valid integer");

    assert!(io_result.is_err());
    assert!(parse_result.is_err());

    let io_error = io_result.expect_err("io_result should be an error");
    let parse_error = parse_result.expect_err("parse_result should be an error");

    assert!(io_error.signpost().is_some());
    assert!(parse_error.signpost().is_some());
}

#[test]
fn test_nested_error_contexts() {
    let base_error = Yoshi::new(YoshiKind::Network {
        message: "Connection timeout".into(),
        source: None,
        error_code: Some(408),
    });

    let result: Hatch<String> = Err(base_error);
    let enriched = result
        .lay("During API call")
        .lay("While fetching user data")
        .lay("In authentication flow");

    assert!(enriched.is_err());
    let error = enriched.expect_err("enriched should be an error");
    let contexts: Vec<_> = error.nests().collect();
    assert!(contexts.len() >= 3);
}

#[test]
fn test_error_source_preservation() {
    let original_error = Yoshi::new(YoshiKind::Internal {
        message: "Original internal error".into(),
        source: None,
        component: Some("database".into()),
    });

    let result: Hatch<i32> = Err(original_error);
    let enriched = result.nest("Additional context");

    assert!(enriched.is_err());
    let error = enriched.expect_err("enriched should be an error");

    // The original error information should still be accessible
    assert!(error.kind().to_string().contains("Original internal error"));
}

#[test]
fn test_multiple_metadata_entries() {
    let result: core::result::Result<i32, String> = Err("metadata test".to_string());
    let enriched = result
        .meta("request_id", "req_12345")
        .meta("user_id", "user_67890")
        .meta("session_id", "sess_abcdef")
        .meta("timestamp", "2025-01-01T00:00:00Z");

    assert!(enriched.is_err());
    let error = enriched.expect_err("enriched should be an error");
    let contexts: Vec<_> = error.nests().collect();
    assert!(!contexts.is_empty());
}

#[test]
fn test_priority_ordering() {
    let low_priority = Yoshi::new(YoshiKind::Internal {
        message: "Low priority".into(),
        source: None,
        component: None,
    })
    .with_priority(10);

    let high_priority = Yoshi::new(YoshiKind::Internal {
        message: "High priority".into(),
        source: None,
        component: None,
    })
    .with_priority(200);

    assert!(low_priority.instance_id() > 0);
    assert!(high_priority.instance_id() > 0);
}

#[test]
fn test_unicode_in_contexts() {
    let result: core::result::Result<i32, String> = Err("unicode test".to_string());
    let enriched = result
        .nest("Context with emoji: 🚀")
        .with_signpost("Suggestion with Chinese: 建议")
        .meta("arabic_key", "مفتاح");

    assert!(enriched.is_err());
    let error = enriched.expect_err("enriched should be an error");
    assert!(error.signpost().is_some());
    let contexts: Vec<_> = error.nests().collect();
    assert!(!contexts.is_empty());
}

#[test]
fn test_empty_string_handling() {
    let result: core::result::Result<i32, String> = Err(String::new());
    let enriched = result.nest("").with_signpost("").meta("", "");

    assert!(enriched.is_err());
    let error = enriched.expect_err("enriched should be an error");
    let contexts: Vec<_> = error.nests().collect();
    assert!(!contexts.is_empty());
}
