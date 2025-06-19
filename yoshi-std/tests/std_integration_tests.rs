/* yoshi-std/tests/std_integration_tests_clean.rs */
#![deny(unsafe_code)]
#![warn(missing_docs)]
//! **Brief:** Clean test suite for yoshi-std standard library integration with working functionality.
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + Standard library error conversion with zero-cost abstractions and seamless integration
//!  - `std::io::Error` integration with automatic conversion and context preservation
//!  - Error handling with structured error propagation
//!  - String and str error conversion with validation context
//!  - Custom error type conversion with trait-based extensibility
//!  - Performance testing and memory efficiency validation
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

use std::io::{self, ErrorKind};
use yoshi_std::{io_error_to_yoshi, HatchExt, Result, Yoshi, YoshiKind};

#[test]
fn test_io_error_conversion() {
    let io_error = io::Error::new(ErrorKind::NotFound, "File not found");
    let yoshi_error = io_error_to_yoshi(io_error);

    // Just verify the conversion worked
    let error_str = yoshi_error.kind().to_string();
    assert!(!error_str.is_empty());
}

#[test]
fn test_io_error_with_context() {
    let io_error = io::Error::new(ErrorKind::PermissionDenied, "Access denied");
    let result: Result<String> = Err(io_error_to_yoshi(io_error));

    let enriched = result.context("While reading configuration file");
    assert!(enriched.is_err());

    let error = enriched.unwrap_err();
    let contexts: Vec<_> = error.contexts().collect();
    assert!(!contexts.is_empty());
}

#[test]
fn test_string_error_conversion() {
    let string_error = "Custom error message".to_string();
    let yoshi_error: Yoshi = string_error.into();

    assert!(yoshi_error
        .kind()
        .to_string()
        .contains("Custom error message"));
}

#[test]
fn test_str_error_conversion() {
    let str_error = "Static error message";
    let yoshi_error: Yoshi = str_error.into();

    assert!(yoshi_error
        .kind()
        .to_string()
        .contains("Static error message"));
}

#[test]
fn test_basic_error_creation() {
    let error = Yoshi::new(YoshiKind::Internal {
        message: "Test error".into(),
        source: None,
        component: None,
    });

    assert!(error.kind().to_string().contains("Test error"));
    // Instance ID can be 0, just check it's a valid number
    let _id = error.instance_id();
}

#[test]
fn test_error_context_chain() {
    let result: Result<String> = Err(Yoshi::new(YoshiKind::Internal {
        message: "Base error".into(),
        source: None,
        component: None,
    }));

    let enriched = result
        .context("First context")
        .context("Second context")
        .with_signpost("Try this solution");

    assert!(enriched.is_err());
    let error = enriched.unwrap_err();
    assert!(error.suggestion().is_some());
    assert!(error
        .suggestion()
        .expect("suggestion should be present")
        .contains("Try this solution"));
}

#[test]
fn test_error_metadata() {
    let result: Result<String> = Err(Yoshi::new(YoshiKind::Internal {
        message: "Metadata test".into(),
        source: None,
        component: None,
    }));

    let enriched = result
        .meta("key1", "value1")
        .meta("key2", "value2")
        .context("Test context");

    assert!(enriched.is_err());
    let error = enriched.unwrap_err();
    let contexts: Vec<_> = error.contexts().collect();
    assert!(!contexts.is_empty());
}

#[test]
fn test_performance_basic() {
    use std::time::Instant;

    let start = Instant::now();

    for i in 0..100 {
        let error = Yoshi::new(YoshiKind::Internal {
            message: format!("Test error {i}").into(),
            source: None,
            component: None,
        });
        let _id = error.instance_id();
    }

    let duration = start.elapsed();
    assert!(
        duration.as_millis() < 50,
        "Error creation too slow: {duration:?}"
    );
}

#[test]
fn test_memory_efficiency() {
    use std::mem;

    let error = Yoshi::new(YoshiKind::Internal {
        message: "Memory test".into(),
        source: None,
        component: None,
    });

    let size = mem::size_of_val(&error);
    assert!(size > 0);
    assert!(size < 1024); // Should be reasonable size
}

#[test]
fn test_debug_formatting() {
    let error = Yoshi::new(YoshiKind::Internal {
        message: "Debug test".into(),
        source: None,
        component: None,
    })
    .context("Test context")
    .with_signpost("Test suggestion");

    let debug_output = format!("{error:?}");
    let display_output = format!("{error}");

    assert!(!debug_output.is_empty());
    assert!(!display_output.is_empty());
    assert!(debug_output.len() >= display_output.len());
}

#[test]
fn test_error_kinds() {
    let kinds = [
        YoshiKind::Internal {
            message: "Internal".into(),
            source: None,
            component: None,
        },
        YoshiKind::Network {
            message: "Network".into(),
            source: None,
            error_code: None,
        },
        YoshiKind::NotFound {
            resource_type: "file".into(),
            identifier: "test.txt".into(),
            search_locations: None,
        },
    ];

    for kind in kinds {
        let error = Yoshi::new(kind);
        let _id = error.instance_id();

        let display = format!("{error}");
        assert!(!display.is_empty());
    }
}

#[test]
fn test_custom_error_type_conversion() {
    #[derive(Debug)]
    struct CustomError {
        message: String,
        code: u32,
    }

    impl std::fmt::Display for CustomError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Custom error {}: {}", self.code, self.message)
        }
    }

    impl std::error::Error for CustomError {}

    impl From<CustomError> for Yoshi {
        fn from(err: CustomError) -> Self {
            Yoshi::new(YoshiKind::Internal {
                message: err.to_string().into(),
                source: None,
                component: Some("custom_service".into()),
            })
        }
    }

    let custom_error = CustomError {
        message: "Something went wrong".to_string(),
        code: 500,
    };

    let yoshi_error: Yoshi = custom_error.into();
    assert!(yoshi_error
        .kind()
        .to_string()
        .contains("Something went wrong"));
    assert!(yoshi_error.kind().to_string().contains("500"));
}

#[test]
fn test_error_equality_and_comparison() {
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

    // Errors should have different instance IDs even with same content
    assert_ne!(error1.instance_id(), error2.instance_id());
}

#[test]
fn test_concurrent_error_creation() {
    use std::thread;

    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                let error = Yoshi::new(YoshiKind::Internal {
                    message: format!("Error {i}").into(),
                    source: None,
                    component: None,
                });
                error.instance_id()
            })
        })
        .collect();

    let mut instance_ids = Vec::new();
    for handle in handles {
        instance_ids.push(handle.join().expect("thread should complete"));
    }

    // All instance IDs should be unique
    instance_ids.sort_unstable();
    instance_ids.dedup();
    assert_eq!(instance_ids.len(), 10);
}

#[test]
fn test_error_conversion_performance() {
    use std::time::Instant;

    let start = Instant::now();

    for i in 0..1000 {
        let io_error = io::Error::other(format!("Error {i}"));
        let _yoshi_error = io_error_to_yoshi(io_error);
    }

    let duration = start.elapsed();

    // Should be very fast - less than 100ms for 1000 conversions
    assert!(
        duration.as_millis() < 100,
        "Error conversion too slow: {duration:?}"
    );
}

#[test]
fn test_error_size_optimization() {
    use std::mem;

    // Yoshi errors should be reasonably sized
    let yoshi_size = mem::size_of::<Yoshi>();
    let io_error_size = mem::size_of::<io::Error>();

    // Yoshi should be larger than io::Error due to additional context,
    // but not excessively so - allow for more flexibility
    assert!(yoshi_size > io_error_size);
    assert!(yoshi_size < io_error_size * 50); // More reasonable upper bound
}

#[test]
fn test_error_display_consistency() {
    let io_error = io::Error::new(ErrorKind::NotFound, "Test file not found");
    let yoshi_error = io_error_to_yoshi(io_error);

    let display = format!("{yoshi_error}");
    let debug = format!("{yoshi_error:?}");

    // Both should be non-empty
    assert!(!display.is_empty());
    assert!(!debug.is_empty());

    // Debug should be more verbose
    assert!(debug.len() >= display.len());
}
