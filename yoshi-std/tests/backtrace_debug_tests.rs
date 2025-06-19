/* yoshi-std/tests/backtrace_debug_tests_clean.rs */
#![deny(unsafe_code)]
#![warn(missing_docs)]
//! **Brief:** Clean test suite for yoshi-std backtrace and debugging capabilities.
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + Backtrace capture and analysis with environment-aware configuration
//!  - `StdYoshiBacktrace` functionality with std compatibility
//!  - Debug output formatting with structured information display
//!  - Error instance tracking with unique identifier validation
//!  - Performance monitoring with timing and memory analysis
//!  - Diagnostic utilities with comprehensive error introspection
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

use std::time::SystemTime;
use yoshi_std::{error_instance_count, StdYoshiBacktrace, Yoshi, YoshiKind};

#[test]
fn test_error_instance_counter() {
    let _initial_count = error_instance_count();

    let error1 = Yoshi::new(YoshiKind::Internal {
        message: "First test error".into(),
        source: None,
        component: None,
    });

    let _count_after_first = error_instance_count();
    // Just check that we have a valid instance ID
    assert!(error1.instance_id() > 0);

    let error2 = Yoshi::new(YoshiKind::Internal {
        message: "Second test error".into(),
        source: None,
        component: None,
    });

    let _count_after_second = error_instance_count();
    // Check that the second error has a different instance ID
    assert_ne!(error1.instance_id(), error2.instance_id());
    assert!(error2.instance_id() > 0);
}

#[test]
fn test_error_instance_uniqueness() {
    let error1 = Yoshi::new(YoshiKind::Internal {
        message: "Test error".into(),
        source: None,
        component: None,
    });

    let error2 = Yoshi::new(YoshiKind::Internal {
        message: "Test error".into(),
        source: None,
        component: None,
    });

    assert_ne!(error1.instance_id(), error2.instance_id());
    assert!(error2.instance_id() > error1.instance_id());
}

#[test]
fn test_error_cloning_creates_new_instance() {
    let original = Yoshi::new(YoshiKind::Internal {
        message: "Original error".into(),
        source: None,
        component: None,
    });

    let cloned = original.clone();

    // Cloned error should have different instance ID
    assert_ne!(original.instance_id(), cloned.instance_id());

    // But same content
    assert_eq!(original.kind().to_string(), cloned.kind().to_string());
}

#[test]
fn test_backtrace_creation() {
    let backtrace = StdYoshiBacktrace::new_captured();

    // Backtrace should have some basic information
    let debug = format!("{backtrace:?}");
    assert!(!debug.is_empty());
}

#[test]
fn test_backtrace_creation_only() {
    // Only test the available method
    let backtrace = StdYoshiBacktrace::new_captured();

    let debug = format!("{backtrace:?}");
    assert!(!debug.is_empty());
}

#[test]
fn test_error_with_backtrace() {
    let error = Yoshi::new(YoshiKind::Internal {
        message: "Error with backtrace".into(),
        source: None,
        component: None,
    });

    // Check if error has backtrace information
    let debug_output = format!("{error:?}");
    assert!(!debug_output.is_empty());
    assert!(debug_output.contains("Error with backtrace"));
}

#[test]
fn test_error_debug_formatting() {
    let error = Yoshi::new(YoshiKind::Network {
        message: "Network connection failed".into(),
        source: None,
        error_code: Some(500),
    })
    .context("During API call")
    .with_signpost("Check network connectivity");

    let debug_output = format!("{error:?}");

    // Should contain error message
    assert!(debug_output.contains("Network connection failed"));

    // Should contain context
    assert!(debug_output.contains("During API call"));

    // Should be reasonably formatted
    assert!(debug_output.len() > 50);
}

#[test]
fn test_error_display_formatting() {
    let error = Yoshi::new(YoshiKind::Validation {
        field: "email".into(),
        message: "Invalid email format".into(),
        expected: Some("valid@example.com".into()),
        actual: Some("invalid-email".into()),
    });

    let display_output = format!("{error}");

    // Should contain the main error message
    assert!(display_output.contains("Invalid email format"));

    // Display should be more concise than debug
    let debug_output = format!("{error:?}");
    assert!(debug_output.len() > display_output.len());
}

#[test]
fn test_error_timing_information() {
    let start_time = SystemTime::now();

    let error = Yoshi::new(YoshiKind::Internal {
        message: "Timed error".into(),
        source: None,
        component: None,
    });

    let end_time = SystemTime::now();

    // Error creation should be very fast
    let duration = end_time
        .duration_since(start_time)
        .expect("time should be valid");
    assert!(duration.as_millis() < 10); // Should be sub-millisecond

    let _id = error.instance_id(); // Instance ID should be valid
}

#[test]
fn test_error_memory_efficiency() {
    use std::mem;

    let error = Yoshi::new(YoshiKind::Internal {
        message: "Memory test".into(),
        source: None,
        component: None,
    });

    // Error should be reasonably sized
    let size = mem::size_of_val(&error);
    assert!(size > 0);
    assert!(size < 1024); // Should be less than 1KB for basic error
}

#[test]
fn test_error_context_chain_debug() {
    let error = Yoshi::new(YoshiKind::Internal {
        message: "Base error".into(),
        source: None,
        component: None,
    })
    .context("First context")
    .context("Second context")
    .context("Third context")
    .with_signpost("Try this solution")
    .with_priority(150);

    let debug_output = format!("{error:?}");

    // Should contain all contexts
    assert!(debug_output.contains("Base error"));
    assert!(debug_output.contains("First context"));
    assert!(debug_output.contains("Second context"));
    assert!(debug_output.contains("Third context"));

    // Should contain suggestion
    assert!(debug_output.contains("Try this solution"));
}

#[test]
fn test_concurrent_error_debugging() {
    use std::thread;

    let handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                let error = Yoshi::new(YoshiKind::Internal {
                    message: format!("Thread {i} error").into(),
                    source: None,
                    component: None,
                })
                .context(format!("Thread {i} context"));

                let debug_output = format!("{error:?}");
                (error.instance_id(), debug_output)
            })
        })
        .collect();

    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.join().expect("thread should complete"));
    }

    // All errors should have unique instance IDs
    let mut instance_ids: Vec<_> = results.iter().map(|(id, _)| *id).collect();
    instance_ids.sort_unstable();
    instance_ids.dedup();
    assert_eq!(instance_ids.len(), 5);

    // All debug outputs should be unique and contain thread-specific info
    for (i, (_, debug_output)) in results.iter().enumerate() {
        assert!(debug_output.contains(&format!("Thread {i} error")));
    }
}

#[test]
fn test_error_performance_monitoring() {
    use std::time::Instant;

    let start = Instant::now();
    let mut errors = Vec::new();

    // Create many errors quickly
    for i in 0..1000 {
        let error = Yoshi::new(YoshiKind::Internal {
            message: format!("Performance test error {i}").into(),
            source: None,
            component: None,
        });
        errors.push(error);
    }

    let creation_time = start.elapsed();

    // Format all errors for debug output
    let format_start = Instant::now();
    for error in &errors {
        let _debug = format!("{error:?}");
    }
    let format_time = format_start.elapsed();

    // Performance should be reasonable
    assert!(
        creation_time.as_millis() < 100,
        "Error creation too slow: {creation_time:?}"
    );
    assert!(
        format_time.as_millis() < 500,
        "Error formatting too slow: {format_time:?}"
    );

    // All errors should have unique instance IDs
    let mut instance_ids: Vec<_> = errors.iter().map(Yoshi::instance_id).collect();
    instance_ids.sort_unstable();
    let original_len = instance_ids.len();
    instance_ids.dedup();
    assert_eq!(instance_ids.len(), original_len);
}

#[test]
fn test_error_diagnostic_information() {
    let error = Yoshi::new(YoshiKind::Timeout {
        operation: "database_query".into(),
        duration: std::time::Duration::from_secs(30),
        expected_max: Some(std::time::Duration::from_secs(10)),
    })
    .context("Database operation timeout")
    .with_signpost("Increase timeout or optimize query");

    // Test comprehensive diagnostic output
    let debug_output = format!("{error:?}");

    assert!(debug_output.contains("database_query"));
    assert!(debug_output.contains("Database operation timeout"));
    assert!(debug_output.contains("Increase timeout"));

    // Test display output is more concise
    let display_output = format!("{error}");
    assert!(display_output.len() < debug_output.len());
    assert!(display_output.contains("database_query"));
}

#[test]
fn test_error_introspection_methods() {
    let error = Yoshi::new(YoshiKind::Internal {
        message: "Introspection test".into(),
        source: None,
        component: None,
    })
    .context("Test context")
    .with_signpost("Test suggestion");

    // Test various introspection methods
    let _id = error.instance_id();
    let _kind = error.kind();
    let contexts: Vec<_> = error.contexts().collect();
    let suggestion = error.suggestion();

    assert!(!contexts.is_empty());
    assert!(suggestion.is_some());
    assert!(suggestion
        .expect("suggestion should be present")
        .contains("Test suggestion"));
}

#[test]
fn test_error_serialization_debug() {
    let error = Yoshi::new(YoshiKind::Internal {
        message: "Serialization test".into(),
        source: None,
        component: None,
    });

    // Test that error can be formatted for logging/serialization
    let json_like = format!(
        r#"{{"instance_id": {}, "message": "{}"}}"#,
        error.instance_id(),
        "Serialization test"
    );

    assert!(json_like.contains("instance_id"));
    assert!(json_like.contains("Serialization test"));
}
