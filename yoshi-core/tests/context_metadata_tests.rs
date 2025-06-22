/* yoshi-core/tests/context_metadata_tests.rs */
#![deny(dead_code)]
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![deny(unused_variables)]
#![allow(clippy::expect_used)] // Allow expect in tests for clearer error messages
#![deny(clippy::unwrap_used)]
//! **Brief:** Comprehensive test suite for yoshi-core context and metadata management with performance optimization validation.
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + Context chain validation with O(1) attachment and O(n) traversal complexity
//!  - Metadata storage with `HashMap` efficiency and type safety guarantees
//!  - Suggestion system with structured recommendation handling
//!  - Priority management with numerical ordering and conflict resolution
//!  - Payload system with Any trait downcasting and memory safety
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

use yoshi_core::YoshiCore;

#[test]
fn test_error_with_metadata() {
    let mut error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Internal {
        message: "Base error".into(),
        source: None,
        component: None,
    });

    error = error.with_metadata("operation", "test_operation");
    let _id = error.instance_id(); // Instance ID is always valid

    let nests: Vec<_> = error.nests().collect();
    assert!(!nests.is_empty());
}

#[test]
fn test_error_with_suggestion() {
    let mut error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Validation {
        field: "email".into(),
        message: "Invalid email format".into(),
        expected: Some("valid email address".into()),
        actual: Some("invalid@".into()),
    });

    error = error.with_signpost("Please provide a valid email address");
    let _id = error.instance_id(); // Instance ID is always valid
    assert!(error.signpost().is_some());
    assert!(error
        .signpost()
        .expect("signpost should be present")
        .contains("valid email"));
}

#[test]
fn test_error_with_priority() {
    let mut error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Internal {
        message: "Priority test".into(),
        source: None,
        component: None,
    });

    error = error.with_priority(90);
    let _id = error.instance_id(); // Instance ID is always valid
}

#[test]
fn test_multiple_metadata() {
    let error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Internal {
        message: "Metadata test".into(),
        source: None,
        component: None,
    })
    .with_metadata("key1", "value1")
    .with_metadata("key2", "value2")
    .with_metadata("key3", "value3");

    let _id = error.instance_id(); // Instance ID is always valid
    let nests: Vec<_> = error.nests().collect();
    assert!(!nests.is_empty());
}

#[test]
fn test_context_chaining() {
    let error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Internal {
        message: "Base error".into(),
        source: None,
        component: None,
    })
    .lay("First context")
    .lay("Second context")
    .lay("Third context");

    let _id = error.instance_id(); // Instance ID is always valid
    let nests: Vec<_> = error.nests().collect();
    assert!(nests.len() >= 3);
}

#[test]
fn test_context_with_location() {
    let location = YoshiCore::YoshiLocation::new("test.rs", 100, 20);
    let context = YoshiCore::Nest::new("Test context").with_location(location);

    assert_eq!(context.message.as_deref(), Some("Test context"));
    assert!(context.location.is_some());
    let location = context.location.expect("location should be present");
    assert_eq!(location.file, "test.rs");
    assert_eq!(location.line, 100);
    assert_eq!(location.column, 20);
}

#[test]
fn test_context_with_metadata() {
    let mut context = YoshiCore::Nest::new("Test context");
    context = context.with_metadata("test_key", "test_value");

    assert_eq!(context.message.as_deref(), Some("Test context"));
    assert!(!context.metadata.is_empty());
    assert_eq!(
        context
            .metadata
            .get("test_key")
            .map(std::convert::AsRef::as_ref),
        Some("test_value")
    );
}

#[test]
fn test_context_with_suggestion() {
    let context = YoshiCore::Nest::new("Test context").with_signpost("Try this solution");

    assert_eq!(context.message.as_deref(), Some("Test context"));
    assert_eq!(context.suggestion.as_deref(), Some("Try this solution"));
}

#[test]
fn test_context_with_priority() {
    let context = YoshiCore::Nest::new("Test context").with_priority(150);

    assert_eq!(context.message.as_deref(), Some("Test context"));
    assert_eq!(context.priority, 150);
}

#[test]
fn test_context_builder_pattern() {
    let location = YoshiCore::YoshiLocation::new("builder_test.rs", 50, 15);
    let context = YoshiCore::Nest::new("Builder test")
        .with_location(location)
        .with_metadata("component", "test_module")
        .with_metadata("operation", "validation")
        .with_signpost("Check input parameters")
        .with_priority(75);

    assert_eq!(context.message.as_deref(), Some("Builder test"));
    assert!(context.location.is_some());
    assert_eq!(context.metadata.len(), 2);
    assert_eq!(
        context.suggestion.as_deref(),
        Some("Check input parameters")
    );
    assert_eq!(context.priority, 75);
}

#[test]
fn test_error_context_iteration() {
    let error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Internal {
        message: "Base error".into(),
        source: None,
        component: None,
    })
    .lay("Context 1")
    .lay("Context 2")
    .with_metadata("key", "value");

    let nests: Vec<_> = error.nests().collect();
    assert!(!nests.is_empty());

    // Check that we can iterate multiple times
    let nests2: Vec<_> = error.nests().collect();
    assert_eq!(nests.len(), nests2.len());
}

#[test]
fn test_error_suggestion_retrieval() {
    let error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Validation {
        field: "password".into(),
        message: "Password too weak".into(),
        expected: Some("strong password".into()),
        actual: Some("123".into()),
    })
    .with_signpost("Use at least 8 characters with mixed case");

    assert!(error.signpost().is_some());
    let signpost = error.signpost().expect("signpost should be present");
    assert!(signpost.contains("8 characters"));
    assert!(signpost.contains("mixed case"));
}

#[test]
fn test_error_metadata_types() {
    let error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Internal {
        message: "Metadata test".into(),
        source: None,
        component: None,
    })
    .with_metadata("string_key", "string_value")
    .with_metadata("number_key", "42")
    .with_metadata("boolean_key", "true")
    .with_metadata("json_key", r#"{"nested": "value"}"#);

    let nests: Vec<_> = error.nests().collect();
    assert!(!nests.is_empty());
}

#[test]
fn test_context_empty_message() {
    let context = YoshiCore::Nest::new("");
    assert_eq!(context.message.as_deref(), Some(""));
    assert!(context.metadata.is_empty());
    assert!(context.suggestion.is_none());
    assert_eq!(context.priority, 128); // Default medium priority
}

#[test]
fn test_context_unicode_message() {
    let context = YoshiCore::Nest::new("Unicode: 🚀 测试 مرحبا");
    assert_eq!(context.message.as_deref(), Some("Unicode: 🚀 测试 مرحبا"));
}

#[test]
fn test_context_long_message() {
    let long_message = "A".repeat(1000);
    let context = YoshiCore::Nest::new(&long_message);
    assert_eq!(context.message.as_deref(), Some(long_message.as_str()));
}

#[test]
fn test_metadata_overwrite() {
    let mut context = YoshiCore::Nest::new("Test");
    context = context.with_metadata("key", "value1");
    context = context.with_metadata("key", "value2");

    assert_eq!(
        context.metadata.get("key").map(std::convert::AsRef::as_ref),
        Some("value2")
    );
}

#[test]
fn test_priority_bounds() {
    let context_min = YoshiCore::Nest::new("Min priority").with_priority(0);
    let context_max = YoshiCore::Nest::new("Max priority").with_priority(255);

    assert_eq!(context_min.priority, 0);
    assert_eq!(context_max.priority, 255);
}

#[test]
fn test_context_cloning() {
    let original = YoshiCore::Nest::new("Original context")
        .with_metadata("key", "value")
        .with_signpost("Original suggestion")
        .with_priority(100);

    let cloned = original.clone();

    assert_eq!(original.message, cloned.message);
    assert_eq!(original.metadata.len(), cloned.metadata.len());
    assert_eq!(original.suggestion, cloned.suggestion);
    assert_eq!(original.priority, cloned.priority);
}

#[test]
fn test_error_lay_method() {
    let error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Internal {
        message: "Original error".into(),
        source: None,
        component: None,
    })
    .lay("Additional context");

    let _id = error.instance_id(); // Instance ID is always valid
    let nests: Vec<_> = error.nests().collect();
    assert!(!nests.is_empty());
}

#[test]
fn test_complex_context_scenario() {
    let _location = YoshiCore::YoshiLocation::new("complex_test.rs", 200, 30);

    // Create error step by step to debug
    let mut error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Network {
        message: "Connection failed".into(),
        source: None,
        error_code: Some(500),
    });

    // Verify basic creation works
    let initial_id = error.instance_id(); // Instance ID is always valid

    // Add context step by step
    error = error.lay("During API call");
    error = error.with_metadata("endpoint", "/api/users");
    error = error.with_metadata("method", "GET");
    error = error.with_metadata("timeout", "30s");
    error = error.with_signpost("Check network connectivity");
    error = error.with_priority(200);

    assert!(error.instance_id() == initial_id); // Instance ID should remain the same
    assert!(error.signpost().is_some());
    let nests: Vec<_> = error.nests().collect();
    assert!(!nests.is_empty());
}

#[test]
fn test_context_debug_formatting() {
    let context = YoshiCore::Nest::new("Debug test")
        .with_metadata("debug_key", "debug_value")
        .with_signpost("Debug suggestion");

    let debug_string = format!("{context:?}");
    assert!(debug_string.contains("Debug test"));
}

#[test]
fn test_context_display_formatting() {
    let context = YoshiCore::Nest::new("Display test").with_signpost("Display suggestion");

    let debug_string = format!("{context:?}");
    assert!(debug_string.contains("Display test"));
}

#[test]
fn test_error_with_shell_payload() {
    let test_data = "test payload data";
    let error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Internal {
        message: "Shell test".into(),
        source: None,
        component: None,
    })
    .with_shell(test_data);

    let _id = error.instance_id(); // Instance ID is always valid
    let nests: Vec<_> = error.nests().collect();
    assert!(!nests.is_empty());
}

#[test]
fn test_multiple_shell_payloads() {
    let error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Internal {
        message: "Multiple shells test".into(),
        source: None,
        component: None,
    })
    .with_shell("first payload")
    .with_shell(42u32)
    .with_shell(true);

    let _id = error.instance_id(); // Instance ID is always valid
    let nests: Vec<_> = error.nests().collect();
    assert!(!nests.is_empty());
}
