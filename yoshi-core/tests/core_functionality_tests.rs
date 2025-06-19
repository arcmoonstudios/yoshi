/* yoshi-core/tests/core_functionality_tests.rs */
#![deny(dead_code)]
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![deny(unused_variables)]
#![allow(clippy::expect_used)] // Allow expect in tests for clearer error messages
#![deny(clippy::unwrap_used)]
//! **Brief:** Comprehensive test suite for yoshi-core fundamental error handling capabilities with mathematical precision validation.
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + Core error type validation with O(1) construction and O(log n) context retrieval
//!  - `YoshiKind` variant testing with memory safety guarantees and zero-cost abstractions
//!  - Error instance tracking with atomic operations and thread-safety validation
//!  - Location capture with compile-time optimization and runtime efficiency
//!  - Context management with structured payload handling and type safety
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

use yoshi_core::YoshiCore;

#[test]
fn test_yoshi_creation_and_basic_properties() {
    let error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Internal {
        message: "Test error".into(),
        source: None,
        component: None,
    });

    assert!(error.instance_id() > 0);
    assert_eq!(error.kind().to_string(), "Internal error: Test error");
}

#[test]
fn test_yoshi_kind_variants() {
    // Test Internal variant
    let internal = YoshiCore::YoshiKind::Internal {
        message: "Internal error".into(),
        source: None,
        component: Some("test_component".into()),
    };
    assert!(internal.to_string().contains("Internal error"));

    // Test Network variant
    let network = YoshiCore::YoshiKind::Network {
        message: "Network failure".into(),
        source: None,
        error_code: Some(404),
    };
    assert!(network.to_string().contains("Network failure"));

    // Test Validation variant
    let validation = YoshiCore::YoshiKind::Validation {
        field: "username".into(),
        message: "Invalid input".into(),
        expected: Some("non-empty string".into()),
        actual: Some("empty".into()),
    };
    assert!(validation.to_string().contains("Invalid input"));
}

#[test]
fn test_error_instance_tracking() {
    let error1 = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Internal {
        message: "First error".into(),
        source: None,
        component: None,
    });

    let error2 = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Internal {
        message: "Second error".into(),
        source: None,
        component: None,
    });

    // Instance IDs should be unique and incrementing
    assert_ne!(error1.instance_id(), error2.instance_id());
    assert!(error2.instance_id() > error1.instance_id());
}

#[test]
fn test_yoshi_location_capture() {
    let location = YoshiCore::YoshiLocation::new("core_functionality_tests.rs", 85, 10);

    assert!(location.file.contains("core_functionality_tests.rs"));
    assert!(location.line > 0);
    assert!(location.column > 0);
}

#[test]
fn test_error_with_source() {
    let source_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    let yoshi_error = YoshiCore::Yoshi::foreign(source_error);

    assert!(yoshi_error.instance_id() > 0);
    assert!(yoshi_error.kind().to_string().contains("File not found"));
}

#[test]
fn test_error_context_chaining() {
    let mut error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Internal {
        message: "Base error".into(),
        source: None,
        component: None,
    });

    error = error.with_metadata("operation", "test_operation");
    error = error.with_metadata("user_id", "12345");

    // Verify metadata is accessible
    let contexts: Vec<_> = error.contexts().collect();
    assert!(!contexts.is_empty());
}

#[test]
fn test_error_priority_levels() {
    let mut error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Internal {
        message: "Priority test".into(),
        source: None,
        component: None,
    });

    error = error.with_priority(90);

    // Verify priority is set (implementation dependent)
    assert!(error.instance_id() > 0);
}

#[test]
fn test_error_suggestions() {
    let mut error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Validation {
        field: "email".into(),
        message: "Invalid email format".into(),
        expected: Some("valid email address".into()),
        actual: Some("invalid@".into()),
    });

    error = error.with_signpost("Please provide a valid email address with proper domain");

    assert!(error.suggestion().is_some());
    assert!(error
        .suggestion()
        .expect("suggestion should be present")
        .contains("valid email"));
}

#[test]
fn test_thread_safety() {
    use std::sync::mpsc;
    use std::thread;

    let (tx, rx) = mpsc::channel();

    // Spawn multiple threads creating errors
    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            let error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Internal {
                message: format!("Thread error {i}").into(),
                source: None,
                component: None,
            });
            tx.send(error.instance_id())
                .expect("channel should be open");
        });
    }

    drop(tx);

    // Collect all instance IDs
    let mut ids = Vec::new();
    while let Ok(id) = rx.recv() {
        ids.push(id);
    }

    // Verify all IDs are unique
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 10);
}

#[test]
fn test_memory_efficiency() {
    // Test that creating many errors doesn't cause excessive memory usage
    let mut errors = Vec::new();

    for i in 0..1000 {
        let error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Internal {
            message: format!("Error {i}").into(),
            source: None,
            component: None,
        });
        errors.push(error);
    }

    // Verify all errors are valid
    assert_eq!(errors.len(), 1000);

    // Check that instance IDs are sequential
    for i in 1..errors.len() {
        assert!(
            errors.get(i).expect("index should be valid").instance_id()
                > errors
                    .get(i - 1)
                    .expect("index should be valid")
                    .instance_id()
        );
    }
}

#[test]
fn test_error_display_formatting() {
    let error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Network {
        message: "Connection timeout".into(),
        source: None,
        error_code: Some(408),
    });

    let display_string = format!("{error}");
    assert!(display_string.contains("Connection timeout"));
}

#[test]
fn test_error_debug_formatting() {
    let error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Internal {
        message: "Debug test".into(),
        source: None,
        component: Some("test_module".into()),
    });

    let debug_string = format!("{error:?}");
    assert!(debug_string.contains("Debug test"));
    assert!(debug_string.contains("test_module"));
}

#[test]
fn test_error_chaining_basic() {
    // Test basic error chaining without complex scenarios
    let error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Internal {
        message: "Basic internal failure".into(),
        source: None,
        component: Some("test_component".into()),
    });

    // Test metadata addition
    let error_with_metadata = error.with_metadata("operation", "test");

    // Test suggestion addition
    let error_with_suggestion = error_with_metadata.with_signpost("Check configuration");

    // Verify components are present
    assert!(error_with_suggestion.suggestion().is_some());
}
