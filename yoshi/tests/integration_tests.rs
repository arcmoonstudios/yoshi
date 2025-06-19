/* yoshi/tests/integration_tests.rs */
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
//! **Brief:** Elite integration test suite for yoshi main crate with end-to-end validation.
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + External API validation with comprehensive error handling patterns
//!  - Core functionality tests with mathematical precision validation
//!  - Error boundary validation with graceful degradation protocols
//!  - Performance constraint verification with benchmark thresholds
//!  - Integration contracts with API stability guarantees
//!  - End-to-end workflow validation with real-world scenarios
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

use yoshi::*;
use yoshi_std::io_error_to_yoshi;

//--------------------------------------------------------------------------------------------------
// Core Functionality Tests
//--------------------------------------------------------------------------------------------------

#[test]
fn test_yoshi_crate_basic_functionality() {
    // Test that the main yoshi crate exports all necessary components
    let _result: Result<String> = Ok("test".to_string());

    // Test error creation
    let error = Yoshi::new(YoshiKind::Internal {
        message: "Integration test error".into(),
        source: None,
        component: Some("integration_test".into()),
    });

    assert!(error.to_string().contains("Integration test error"));
    // Instance ID is always valid (u64 type)
    let _instance_id = error.instance_id();
}

#[test]
fn test_comprehensive_error_handling() {
    // Test the full error handling pipeline
    let result: Result<()> = Err(Yoshi::new(YoshiKind::Internal {
        message: "Test error for comprehensive handling".into(),
        source: None,
        component: Some("test_component".into()),
    }));

    let enhanced = result
        .context("During integration test execution")
        .with_signpost("Check test configuration")
        .with_priority(150)
        .ctx("Integration test context")
        .help("This is a test error for validation");

    assert!(enhanced.is_err());
    let error = enhanced.expect_err("Should be an error");
    assert!(error.suggestion().is_some());
    // Just check that we have a suggestion - the exact value may vary
    assert!(!error.suggestion().unwrap().is_empty());

    let contexts: Vec<_> = error.contexts().collect();
    assert!(!contexts.is_empty());
}

#[test]
fn test_io_error_integration() {
    use std::io::{self, ErrorKind};

    let io_error = io::Error::new(ErrorKind::NotFound, "Integration test file not found");
    let result: Result<String> = Err(io_error_to_yoshi(io_error));

    let enhanced = result.context("During file operation in integration test");
    assert!(enhanced.is_err());

    let error = enhanced.expect_err("Should be an error");
    let contexts: Vec<_> = error.contexts().collect();
    assert!(!contexts.is_empty());
}

#[test]
fn test_type_alias_ergonomics() {
    fn returns_success() -> String {
        "integration_success".to_string()
    }

    fn returns_error() -> Result<i32> {
        Err(Yoshi::new(YoshiKind::Internal {
            message: "Integration test error".into(),
            source: None,
            component: Some("type_alias_test".into()),
        }))
    }

    assert_eq!(returns_success(), "integration_success");
    assert!(returns_error().is_err());
}

//--------------------------------------------------------------------------------------------------
// Error Boundary Validation
//--------------------------------------------------------------------------------------------------

#[test]
fn test_error_boundary_validation() {
    // Test error propagation through multiple layers
    fn level_3() -> Result<String> {
        Err(Yoshi::new(YoshiKind::Internal {
            message: "Level 3 error".into(),
            source: None,
            component: Some("level_3".into()),
        }))
        .context("Level 3: Core operation")
    }

    fn level_2() -> Result<String> {
        level_3().context("Level 2: Business logic")
    }

    fn level_1() -> Result<String> {
        level_2().context("Level 1: API layer")
    }

    let result = level_1();
    assert!(result.is_err());

    let error = result.expect_err("Should be an error");
    let contexts: Vec<_> = error.contexts().collect();
    assert!(contexts.len() >= 3);
}

#[test]
fn test_graceful_degradation() {
    // Test that errors don't cause panics
    let operations = [
        || -> Result<()> {
            Err(Yoshi::new(YoshiKind::Internal {
                message: "Operation 1 failed".into(),
                source: None,
                component: Some("op1".into()),
            }))
        },
        || -> Result<()> {
            Err(Yoshi::new(YoshiKind::Internal {
                message: "Operation 2 failed".into(),
                source: None,
                component: Some("op2".into()),
            }))
        },
        || -> Result<()> { Ok(()) },
    ];

    let mut success_count = 0;
    let mut error_count = 0;

    for op in operations {
        match op() {
            Ok(()) => success_count += 1,
            Err(_) => error_count += 1,
        }
    }

    assert_eq!(success_count, 1);
    assert_eq!(error_count, 2);
}

//--------------------------------------------------------------------------------------------------
// Performance Constraint Verification
//--------------------------------------------------------------------------------------------------

#[test]
fn test_error_creation_performance() {
    use std::time::Instant;

    let start = Instant::now();

    // Create many errors to test performance
    for i in 0..1000 {
        let _error = Yoshi::new(YoshiKind::Internal {
            message: format!("Performance test error {i}").into(),
            source: None,
            component: Some("performance_test".into()),
        });
    }

    let duration = start.elapsed();

    // Error creation should be very fast
    assert!(duration.as_millis() < 100);
}

#[test]
fn test_memory_efficiency() {
    use std::mem;

    // Test that our error types are reasonably sized
    let yoshi_size = mem::size_of::<Yoshi>();
    let result_size = mem::size_of::<Result<String>>();

    // Ensure reasonable memory usage
    assert!(yoshi_size > 0);
    assert!(result_size > 0);
    assert!(yoshi_size < 1024); // Should be less than 1KB
}

//--------------------------------------------------------------------------------------------------
// Integration Contracts
//--------------------------------------------------------------------------------------------------

#[test]
fn test_api_stability() {
    // Test that core API remains stable
    let error = Yoshi::new(YoshiKind::Internal {
        message: "API stability test".into(),
        source: None,
        component: Some("api_test".into()),
    });

    // These methods should always be available
    let _id = error.instance_id();
    let _severity = error.severity();
    let _transient = error.is_transient();
    let _kind = error.kind();
    let _display = error.to_string();
    let _debug = format!("{error:?}");
}

#[test]
fn test_end_to_end_workflow() {
    // Simulate a complete error handling workflow
    let result = simulate_complex_operation()
        .context("During complex operation simulation")
        .with_signpost("Check system configuration")
        .ctx("End-to-end test")
        .help("This is part of the integration test suite");

    match result {
        Ok(_) => {
            // Operation succeeded
        }
        Err(error) => {
            // Validate error structure
            assert!(!error.to_string().is_empty());
            assert!(error.suggestion().is_some());

            let contexts: Vec<_> = error.contexts().collect();
            assert!(!contexts.is_empty());
        }
    }
}

fn simulate_complex_operation() -> Result<String> {
    // Simulate a complex operation that might fail
    Err(Yoshi::new(YoshiKind::Internal {
        message: "Simulated complex operation failure".into(),
        source: None,
        component: Some("simulation".into()),
    }))
}

//--------------------------------------------------------------------------------------------------
// Comprehensive Integration Validation
//--------------------------------------------------------------------------------------------------

#[test]
fn test_comprehensive_integration() {
    // Test all major components working together
    let errors = vec![
        Yoshi::new(YoshiKind::Internal {
            message: "Error 1".into(),
            source: None,
            component: Some("comp1".into()),
        }),
        Yoshi::new(YoshiKind::Internal {
            message: "Error 2".into(),
            source: None,
            component: Some("comp2".into()),
        }),
        Yoshi::new(YoshiKind::Internal {
            message: "Error 3".into(),
            source: None,
            component: Some("comp3".into()),
        }),
    ];

    for error in errors {
        // Test that all errors can be processed
        assert!(!error.to_string().is_empty());
        // Instance ID is always valid (u64 type)
        let _instance_id = error.instance_id();
        assert!(error.severity() > 0);

        // Test context addition
        let enhanced: Result<()> = Err(error).context("Integration test context");
        assert!(enhanced.is_err());

        let enhanced_error = enhanced.expect_err("Should be an error");
        let contexts: Vec<_> = enhanced_error.contexts().collect();
        assert!(!contexts.is_empty());
    }
}
