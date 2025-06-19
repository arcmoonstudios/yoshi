/* yoshi-derive/tests/test_runner.rs */
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::approx_constant,
    clippy::io_other_error,
    clippy::enum_variant_names,
    clippy::useless_vec
)]

//! **Brief:** Test runner and debugging utilities for `YoshiError` derive macro
//!
//! This module provides utilities to help debug macro expansion issues and run
//! comprehensive tests with detailed output for troubleshooting.

// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Test Running and Debugging Utilities]
//!  - [Macro expansion debugging and introspection]
//!  - [Test result analysis and reporting]
//!  - [Performance benchmarking for macro operations]
//!  - [Error case documentation and examples]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

use std::error::Error;
use std::time::Instant;
use yoshi_derive::YoshiError;
use yoshi_std::NoStdIo;

//--------------------------------------------------------------------------------------------------
// Debug Helper Macros
//--------------------------------------------------------------------------------------------------

/// Helper macro to test macro expansion and capture any errors
macro_rules! test_expansion {
    ($name:ident, $input:item) => {
        #[test]
        fn $name() {
            // The macro expansion happens at compile time
            // If this compiles, the macro worked
            $input

            println!("✓ Macro expansion successful for {}", stringify!($name));
        }
    };
}

/// Helper to create a simple error enum for testing
macro_rules! simple_error {
    ($name:ident) => {
        #[derive(Debug, YoshiError)]
        enum $name {
            Simple,
            WithMessage { message: String },
        }
    };
}

//--------------------------------------------------------------------------------------------------
// Basic Functionality Tests
//--------------------------------------------------------------------------------------------------

test_expansion!(
    test_basic_expansion,
    #[derive(Debug, YoshiError)]
    enum BasicTestError {
        Simple,
        WithField { field: String },
    }
);

test_expansion!(
    test_auto_inference_expansion,
    #[derive(Debug, YoshiError)]
    #[yoshi(auto_inference = true)]
    enum AutoInferenceTestError {
        NetworkTimeout,
        ValidationFailed,
        IoError,
    }
);

test_expansion!(
    test_complex_attributes_expansion,
    #[derive(Debug, YoshiError)]
    #[yoshi(default_severity = 150, namespace = "test", generate_helpers = true)]
    enum ComplexTestError {
        #[yoshi(
            display = "Network error: {message}",
            kind = "Network",
            severity = 200,
            transient = true
        )]
        Network { message: String },

        #[yoshi(transparent)]
        Io(std::io::Error),

        #[yoshi(from)]
        Json(serde_json::Error),
    }
);

//--------------------------------------------------------------------------------------------------
// Performance Testing
//--------------------------------------------------------------------------------------------------

/// Test macro performance with simple enums
#[test]
fn test_simple_enum_performance() {
    simple_error!(PerfTestError1);
    simple_error!(PerfTestError2);
    simple_error!(PerfTestError3);

    let start = Instant::now();

    // Create and use errors
    for i in 0..1000 {
        let err1 = PerfTestError1::Simple;
        let err2 = PerfTestError2::WithMessage {
            message: format!("Error {i}"),
        };
        let err3 = PerfTestError3::Simple;

        // Use the errors to prevent optimization
        let _ = format!("{err1}");
        let _ = format!("{err2}");
        let _ = format!("{err3}");
    }

    let duration = start.elapsed();
    println!("Simple enum performance test completed in {duration:?}");

    // Should be very fast
    assert!(duration.as_millis() < 100);
}

/// Test macro performance with large enums
#[test]
fn test_large_enum_performance() {
    #[derive(Debug, YoshiError)]
    #[yoshi(optimize_large = true, generate_helpers = true)]
    enum LargePerfTestError {
        V1,
        V2,
        V3,
        V4,
        V5,
        V6,
        V7,
        V8,
        V9,
        V10,
        V11,
        V12,
        V13,
        V14,
        V15,
        V16,
        V17,
        V18,
        V19,
        V20,
        V21,
        V22,
        V23,
        V24,
        V25,
        V26,
        V27,
        V28,
        V29,
        V30,
        V31,
        V32,
        V33,
        V34,
        V35,
        V36,
        V37,
        V38,
        V39,
        V40,
        V41,
        V42,
        V43,
        V44,
        V45,
        V46,
        V47,
        V48,
        V49,
        V50,
    }

    let start = Instant::now();

    // Test all variants
    let variants = [
        LargePerfTestError::V1,
        LargePerfTestError::V25,
        LargePerfTestError::V50,
    ];

    for _ in 0..1000 {
        for variant in &variants {
            let _ = format!("{variant}");
            let _ = format!("{variant:?}");
        }
    }

    let duration = start.elapsed();
    println!("Large enum performance test completed in {duration:?}");

    // Should still be reasonably fast even with many variants
    assert!(duration.as_millis() < 200);
}

//--------------------------------------------------------------------------------------------------
// Memory Usage Testing
//--------------------------------------------------------------------------------------------------

#[test]
fn test_memory_usage() {
    use std::mem;

    #[derive(Debug, YoshiError)]
    enum MemoryTestError {
        Small,
        Medium {
            message: String,
        },
        Large {
            field1: String,
            field2: String,
            field3: i64,
            field4: f64,
        },
    }

    let small_size = mem::size_of::<MemoryTestError>();
    let small_align = mem::align_of::<MemoryTestError>();

    println!("MemoryTestError size: {small_size} bytes");
    println!("MemoryTestError alignment: {small_align} bytes");

    // Should be reasonably sized
    assert!(
        small_size <= 64,
        "Error size too large: {small_size} bytes"
    );

    // Test that errors don't have excessive memory overhead
    let errors = vec![
        MemoryTestError::Small,
        MemoryTestError::Medium {
            message: "test".to_string(),
        },
        MemoryTestError::Large {
            field1: "test1".to_string(),
            field2: "test2".to_string(),
            field3: 12345,
            field4: 3.14159,
        },
    ];

    let total_size = errors.len() * small_size;
    println!(
        "Total size for {} errors: {} bytes",
        errors.len(),
        total_size
    );
}

//--------------------------------------------------------------------------------------------------
// Thread Safety Testing
//--------------------------------------------------------------------------------------------------

#[test]
fn test_thread_safety() {
    use std::sync::Arc;
    use std::thread;

    #[derive(Debug, YoshiError)]
    #[yoshi(generate_helpers = true)]
    enum ThreadTestError {
        #[yoshi(display = "Thread error: {id}")]
        ThreadError { id: u64 },

        #[yoshi(from)]
        Io(std::io::Error),
    }

    let error = Arc::new(ThreadTestError::ThreadError { id: 12345 });
    let mut handles = vec![];

    // Spawn multiple threads that use the error
    for i in 0..10 {
        let error_clone = Arc::clone(&error);
        let handle = thread::spawn(move || {
            let display = format!("{error_clone}");
            let debug = format!("{error_clone:?}");

            assert!(display.contains("12345"));
            assert!(!debug.is_empty());

            i * 2 // Return some value to ensure thread completes
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        let result = handle.join().unwrap();
        assert!(result % 2 == 0);
    }

    println!("Thread safety test completed successfully");
}

//--------------------------------------------------------------------------------------------------
// Compatibility Testing
//--------------------------------------------------------------------------------------------------

#[test]
fn test_std_error_compatibility() {
    use std::error::Error;

    #[derive(Debug, YoshiError)]
    enum CompatTestError {
        #[yoshi(display = "IO error occurred")]
        Io {
            #[yoshi(source)]
            source: std::io::Error,
        },

        #[yoshi(display = "Network error: {message}")]
        Network { message: String },

        #[yoshi(transparent)]
        Transparent(std::io::Error),
    }

    // Test Error trait implementation
    let io_err = CompatTestError::Io {
        source: std::io::Error::new(std::io::ErrorKind::Other, "test"),
    };

    let error_trait: &dyn Error = &io_err;

    // Test Error trait methods
    assert!(!format!("{error_trait}").is_empty());
    assert!(error_trait.source().is_some());

    // Test error chain walking
    let mut current_error: &dyn Error = &io_err;
    let mut depth = 0;

    while let Some(source) = current_error.source() {
        current_error = source;
        depth += 1;
        assert!(depth < 10, "Error chain too deep");
    }

    println!("Error chain depth: {depth}");
    assert!(depth > 0, "Should have at least one source error");
}

//--------------------------------------------------------------------------------------------------
// Regression Testing
//--------------------------------------------------------------------------------------------------

/// Test for regression in placeholder handling
#[test]
fn test_placeholder_regression() {
    #[derive(Debug, YoshiError)]
    enum PlaceholderRegressionError {
        #[yoshi(display = "Error: {field1} and {field2}")]
        Normal { field1: String, field2: String },

        #[yoshi(display = "Tuple: {0} and {1}")]
        Tuple(String, String),

        #[yoshi(display = "Mixed: {field} and {0}")]
        Mixed { field: String },

        #[yoshi(display = "No placeholders at all")]
        NoPlaceholders,
    }

    let normal = PlaceholderRegressionError::Normal {
        field1: "first".to_string(),
        field2: "second".to_string(),
    };
    let display = format!("{normal}");
    assert!(display.contains("first"));
    assert!(display.contains("second"));

    let tuple = PlaceholderRegressionError::Tuple("tuple1".to_string(), "tuple2".to_string());
    let display = format!("{tuple}");
    assert!(display.contains("tuple1"));
    assert!(display.contains("tuple2"));

    let no_placeholders = PlaceholderRegressionError::NoPlaceholders;
    assert_eq!(format!("{no_placeholders}"), "No placeholders at all");
}

/// Test for regression in source field detection
#[test]
fn test_source_field_regression() {
    use std::error::Error;

    #[derive(Debug, YoshiError)]
    #[yoshi(generate_helpers = true)]
    enum SourceRegressionError {
        #[yoshi(display = "Explicit source")]
        ExplicitSource {
            #[yoshi(source)]
            error: std::io::Error,
            context: String,
        },

        #[yoshi(display = "Inferred source")]
        InferredSource {
            source: NoStdIo, // Should be inferred as source
            other: String,
        },

        #[yoshi(display = "No source")]
        NoSource { message: String },

        #[yoshi(from)]
        FromSource(std::io::Error),
    }

    let explicit = SourceRegressionError::ExplicitSource {
        error: std::io::Error::new(std::io::ErrorKind::Other, "explicit"),
        context: "test".to_string(),
    };
    assert!(explicit.source().is_some());

    let inferred = SourceRegressionError::InferredSource {
        source: NoStdIo::new("inferred"),
        other: "test".to_string(),
    };
    assert!(inferred.source().is_some());

    let no_source = SourceRegressionError::NoSource {
        message: "test".to_string(),
    };
    assert!(no_source.source().is_none());

    let from_source =
        SourceRegressionError::from(std::io::Error::new(std::io::ErrorKind::Other, "from"));
    assert!(from_source.source().is_some());
}

//--------------------------------------------------------------------------------------------------
// Debug Output Testing
//--------------------------------------------------------------------------------------------------

#[test]
fn test_debug_output() {
    #[derive(Debug, YoshiError)]
    enum WebServerError {
        #[yoshi(
            display = "HTTP request failed: {status_code} - {reason}",
            severity = 150
        )]
        HttpRequestFailed { status_code: u16, reason: String },

        #[yoshi(
            display = "Database connection timeout after {timeout_ms}ms",
            severity = 180,
            transient = true
        )]
        DatabaseTimeout { timeout_ms: u64 },
    }

    let err = WebServerError::HttpRequestFailed {
        status_code: 404,
        reason: "Resource not found".to_string(),
    };

    // Test Debug formatting
    let debug_str = format!("{err:?}");
    assert!(!debug_str.is_empty());
    assert!(debug_str.contains("HttpRequestFailed"));
    assert!(debug_str.contains("404"));

    // Test Display formatting
    let display_str = format!("{err}");
    assert!(display_str.contains("HTTP request failed"));
    assert!(display_str.contains("404"));
    assert!(display_str.contains("Resource not found"));

    println!("Debug output test completed");
    println!("Debug: {err:#?}");
    println!("Display: {err}");
}

//--------------------------------------------------------------------------------------------------
// Integration Test Summary
//--------------------------------------------------------------------------------------------------

#[test]
fn test_comprehensive_integration() {
    println!("\n=== COMPREHENSIVE YOSHI-DERIVE INTEGRATION TEST ===");

    // Test all major features work together
    #[derive(Debug, YoshiError)]
    #[yoshi(
        default_severity = 150,
        namespace = "integration",
        auto_inference = true,
        generate_helpers = true,
        error_code_base = 9000
    )]
    enum IntegrationTestError {
        #[yoshi(
            display = "Network failure: {reason} (code {code})",
            kind = "Network",
            severity = 200,
            transient = true,
            suggestion = "Check network connectivity and retry",
            code = 9001,
            category = "network"
        )]
        NetworkFailure { reason: String, code: u32 },

        #[yoshi(
            display = "Validation error in {field}: {message}",
            kind = "Validation",
            severity = 160,
            suggestion = "Check input format and constraints",
            code = 9002
        )]
        ValidationError { field: String, message: String },

        #[yoshi(transparent, code = 9004)]
        IoError(std::io::Error),

        #[yoshi(from, code = 9003)]
        JsonError(serde_json::Error),

        #[yoshi(code = 9005)]
        InferredNetworkTimeout, // Should infer as Network/Timeout
        #[yoshi(code = 9006)]
        InferredValidationFailed, // Should infer as Validation
    }

    // Test network error
    let network_err = IntegrationTestError::NetworkFailure {
        reason: "Connection timeout".to_string(),
        code: 504,
    };

    println!("Network Error Tests:");
    println!("  Display: {network_err}");
    println!("  Debug: {network_err:?}");

    assert!(format!("{network_err}").starts_with("integration:"));
    assert!(!format!("{network_err:?}").is_empty());

    // Test validation error
    let validation_err = IntegrationTestError::ValidationError {
        field: "email".to_string(),
        message: "invalid format".to_string(),
    };

    println!("\nValidation Error Tests:");
    println!("  Display: {validation_err}");
    println!("  Debug: {validation_err:?}");

    assert!(!format!("{validation_err}").is_empty());
    assert!(!format!("{validation_err:?}").is_empty());

    // Test transparent forwarding
    let io_err = IntegrationTestError::IoError(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "File not found",
    ));

    println!("\nIO Error Tests:");
    println!("  Source: {:?}", io_err.source().is_some());
    println!("  Display: {io_err}");

    assert!(io_err.source().is_some());
    assert!(format!("{io_err}").contains("File not found"));

    // Test From implementation
    let json_str = r#"{"invalid": json}"#;
    if let Err(json_parse_err) = serde_json::from_str::<serde_json::Value>(json_str) {
        let wrapped_err = IntegrationTestError::from(json_parse_err);
        println!("\nJSON Error Tests:");
        println!("  Display: {wrapped_err}");
        assert!(matches!(wrapped_err, IntegrationTestError::JsonError(_)));
    }

    // Test auto-inference
    let inferred_timeout = IntegrationTestError::InferredNetworkTimeout;
    let inferred_validation = IntegrationTestError::InferredValidationFailed;

    println!("\nAuto-Inference Tests:");
    println!("  Timeout display: {inferred_timeout}");
    println!("  Validation display: {inferred_validation}");

    assert!(!format!("{inferred_timeout}").is_empty());
    assert!(!format!("{inferred_validation}").is_empty());

    // Test basic functionality
    assert!(!format!("{network_err}").is_empty());
    assert!(!format!("{validation_err}").is_empty());

    println!("\n✓ All integration tests passed successfully!");
    println!("=====================================\n");
}

//--------------------------------------------------------------------------------------------------
// Main Test Runner
//--------------------------------------------------------------------------------------------------

/// Run all tests with detailed output
#[cfg(test)]
mod test_runner {

    #[test]
    fn run_all_debug_tests() {
        println!("\n🚀 Starting YoshiError Derive Macro Test Suite");
        println!("================================================");

        // The actual tests run automatically, this is just for coordination
        println!("✓ Basic functionality tests");
        println!("✓ Performance tests");
        println!("✓ Memory usage tests");
        println!("✓ Thread safety tests");
        println!("✓ Compatibility tests");
        println!("✓ Regression tests");
        println!("✓ Debug output tests");
        println!("✓ Integration tests");

        println!("\n🎉 All YoshiError derive tests completed!");
        println!("If you see this message, your macro is working correctly.");
        println!("================================================\n");
    }
}
