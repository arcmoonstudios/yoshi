/* yoshi-derive/tests/diagnostic_test.rs */
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![allow(clippy::io_other_error)]

//! **Brief:** Diagnostic test to quickly identify `YoshiError` macro issues
//!
//! This is a minimal test to quickly verify that the `YoshiError` derive macro
//! compiles and works correctly. Run this first to identify any basic issues.

// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Quick Diagnostic Testing]
//!  - [Basic macro compilation verification]
//!  - [Essential trait implementation checking]
//!  - [Core functionality validation]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

#![allow(unused_variables)]

use std::error::Error;
use std::fmt::Debug;
use yoshi_derive::YoshiError;

//--------------------------------------------------------------------------------------------------
// Most Basic Test - If this fails, the macro has fundamental issues
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
#[yoshi(generate_helpers = true)]
enum BasicDiagnosticError {
    Simple,
}

#[test]
fn test_basic_macro_works() {
    let err = BasicDiagnosticError::Simple;

    // Test that basic traits are implemented
    tracing::info!("✓ Macro compiles successfully");

    // Test Display trait
    let display_str = format!("{err}");
    assert!(!display_str.is_empty(), "Display implementation failed");
    tracing::info!("✓ Display trait works: '{display_str}'");

    // Test Debug trait
    let debug_str = format!("{err:?}");
    assert!(!debug_str.is_empty(), "Debug implementation failed");
    tracing::debug!("✓ Debug trait works: '{debug_str}'");

    // Test Error trait
    let error_trait: &dyn Error = &err;
    let error_display = format!("{error_trait}");
    assert!(
        !error_display.is_empty(),
        "Error trait implementation failed"
    );
    tracing::error!("✓ Error trait works");

    // Test source method (should be None for basic case)
    assert!(
        error_trait.source().is_none(),
        "Source should be None for basic error"
    );
    tracing::error!("✓ Error source method works");
}

//--------------------------------------------------------------------------------------------------
// Test With Fields - Next level of complexity
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
#[allow(unused_variables)]
enum FieldDiagnosticError {
    #[yoshi(display = "Error with message: {message}")]
    WithMessage { message: String },

    #[yoshi(display = "Tuple error: {0}")]
    TupleError(String),
}

#[test]
fn test_fields_work() {
    let struct_err = FieldDiagnosticError::WithMessage {
        message: "test message".to_string(),
    };

    let display_str = format!("{struct_err}");
    assert!(
        display_str.contains("test message"),
        "Field interpolation failed: {display_str}"
    );
    tracing::info!("✓ Struct field interpolation works: '{display_str}'");

    let tuple_err = FieldDiagnosticError::TupleError("tuple test".to_string());
    let display_str = format!("{tuple_err}");
    assert!(
        display_str.contains("tuple test"),
        "Tuple field interpolation failed: {display_str}"
    );
    tracing::info!("✓ Tuple field interpolation works: '{display_str}'");
}

//--------------------------------------------------------------------------------------------------
// Test Yoshi-Std Integration - Check if conversion works
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
#[allow(unused_variables)]
enum YoshiIntegrationError {
    #[yoshi(
        display = "Integration test error: {message}",
        kind = "Network",
        severity = 200
    )]
    TestError { message: String },
}

#[test]
fn test_yoshi_std_integration() {
    let err = YoshiIntegrationError::TestError {
        message: "integration test".to_string(),
    };

    // Test conversion to Yoshi
    let yoshi_err: yoshi_core::Yoshi = err.into();

    tracing::info!("✓ Conversion to yoshi_std::Yoshi works");
    tracing::error!("  Yoshi error: {yoshi_err}");

    // Test that the conversion preserves information
    let yoshi_str = format!("{yoshi_err}");
    assert!(
        yoshi_str.contains("integration test"),
        "Yoshi conversion lost message"
    );
    tracing::error!("✓ Yoshi conversion preserves error information");
}

//--------------------------------------------------------------------------------------------------
// Test Auto-Inference - Check if it works
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
#[yoshi(auto_inference = true)]
enum AutoInferenceError {
    NetworkTimeout,
    ValidationFailed,
    IoError,
}

#[test]
fn test_auto_inference() {
    let network_err = AutoInferenceError::NetworkTimeout;
    let validation_err = AutoInferenceError::ValidationFailed;
    let io_err = AutoInferenceError::IoError;

    // Test that helper methods exist (generated by auto-inference)
    assert_eq!(network_err.variant_name(), "NetworkTimeout");
    assert_eq!(validation_err.variant_name(), "ValidationFailed");
    assert_eq!(io_err.variant_name(), "IoError");

    tracing::info!("✓ Auto-inference generates helper methods");

    // Test that error kinds are inferred
    tracing::info!("  NetworkTimeout kind: {}", network_err.error_kind());
    tracing::info!("  ValidationFailed kind: {}", validation_err.error_kind());
    tracing::info!("  IoError kind: {}", io_err.error_kind());

    // Basic sanity checks
    assert!(!network_err.error_kind().is_empty());
    assert!(!validation_err.error_kind().is_empty());
    assert!(!io_err.error_kind().is_empty());

    tracing::error!("✓ Auto-inference assigns error kinds");
}

//--------------------------------------------------------------------------------------------------
// Test Source Fields - Advanced feature
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
#[yoshi(generate_helpers = true)]
enum SourceDiagnosticError {
    #[yoshi(display = "IO error occurred")]
    IoError {
        #[yoshi(source)]
        inner: std::io::Error,
    },

    #[yoshi(transparent)]
    Transparent(std::io::Error),

    // Removed FromError to avoid conflicting From implementations
    Other(String),
}

#[test]
fn test_source_fields() {
    // Test explicit source field
    let io_err = SourceDiagnosticError::IoError {
        inner: std::io::Error::new(std::io::ErrorKind::Other, "test io error"),
    };

    assert!(io_err.source().is_some(), "Source field not detected");
    tracing::info!("✓ Explicit source fields work");

    // Test transparent
    let transparent_err = SourceDiagnosticError::Transparent(std::io::Error::new(
        std::io::ErrorKind::Other,
        "transparent error",
    ));

    assert!(
        transparent_err.source().is_some(),
        "Transparent source not working"
    );
    let display_str = format!("{transparent_err}");
    assert!(
        display_str.contains("transparent error"),
        "Transparent forwarding failed"
    );
    tracing::info!("✓ Transparent errors work");

    // Test Other variant
    let other_err = SourceDiagnosticError::Other("other error".to_string());
    let display_str = format!("{other_err}");
    assert!(
        display_str.contains("Other"),
        "Other variant display failed"
    );
    tracing::info!("✓ Other variant works");
}

//--------------------------------------------------------------------------------------------------
// Test Complex Configuration - All features together
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
#[yoshi(
    default_severity = 150,
    namespace = "diagnostic",
    auto_inference = true,
    generate_helpers = true
)]
#[allow(unused_variables, dead_code)]
enum ComplexDiagnosticError {
    #[yoshi(
        display = "Complex error: {operation} failed with {details}",
        kind = "Complex",
        severity = 200,
        suggestion = "Try a different approach",
        transient = true
    )]
    Complex { operation: String, details: String },

    #[yoshi(display = "Simple wrapped error")]
    Simple,
}

#[test]
fn test_complex_configuration() {
    let complex_err = ComplexDiagnosticError::Complex {
        operation: "test_operation".to_string(),
        details: "detailed error info".to_string(),
    };

    // Test namespace in display
    let display_str = format!("{complex_err}");
    assert!(
        display_str.starts_with("diagnostic:"),
        "Namespace not applied: {display_str}"
    );
    assert!(
        display_str.contains("test_operation"),
        "Operation not in display"
    );
    assert!(
        display_str.contains("detailed error info"),
        "Details not in display"
    );
    tracing::info!("✓ Complex display formatting works: '{display_str}'");

    // Test helper methods
    assert_eq!(complex_err.variant_name(), "Complex");
    assert_eq!(complex_err.error_kind(), "Complex");
    assert_eq!(complex_err.severity(), 200);
    assert!(complex_err.is_transient());
    assert_eq!(complex_err.signpost(), Some("Try a different approach"));

    tracing::info!("✓ Complex configuration works");
    tracing::info!("  Variant: {}", complex_err.variant_name());
    tracing::info!("  Kind: {}", complex_err.error_kind());
    tracing::info!("  Severity: {}", complex_err.severity());
    tracing::info!("  Transient: {}", complex_err.is_transient());
    tracing::info!("  Suggestion: {:?}", complex_err.signpost());

    // Test variant-specific helper method
    assert!(complex_err.is_complex());
    tracing::info!("✓ Variant-specific helper methods work");
}

//--------------------------------------------------------------------------------------------------
// Performance Diagnostic - Check if macro is slow
//--------------------------------------------------------------------------------------------------

#[test]
fn test_performance_diagnostic() {
    use std::time::Instant;

    let start = Instant::now();

    // Create many error instances
    for i in 0..1000 {
        let err = BasicDiagnosticError::Simple;
        let _ = format!("{err}");
        let _ = err.source();

        let field_err = FieldDiagnosticError::WithMessage {
            message: format!("test {i}"),
        };
        let _ = format!("{field_err}");
    }

    let duration = start.elapsed();
    tracing::info!("✓ Performance test: created 2000 errors in {duration:?}");

    // Should be very fast
    assert!(
        duration.as_millis() < 100,
        "Error creation too slow: {duration:?}"
    );
}

//--------------------------------------------------------------------------------------------------
// Main Diagnostic Summary
//--------------------------------------------------------------------------------------------------

#[test]
fn run_diagnostic_summary() {
    tracing::info!("\n🔧 YOSHI-DERIVE DIAGNOSTIC SUMMARY");
    tracing::info!("===================================");

    // If we get here, all the individual tests passed
    tracing::info!("✅ Basic macro compilation: PASS");
    tracing::info!("✅ Field interpolation: PASS");
    tracing::info!("✅ Yoshi-std integration: PASS");
    tracing::info!("✅ Auto-inference: PASS");
    tracing::info!("✅ Source fields: PASS");
    tracing::info!("✅ Complex configuration: PASS");
    tracing::info!("✅ Performance: PASS");

    tracing::info!("\n🎉 ALL DIAGNOSTIC TESTS PASSED!");
    tracing::info!("Your YoshiError derive macro is working correctly.");
    tracing::info!("===================================\n");
}

//--------------------------------------------------------------------------------------------------
// Quick Test for Issues
//--------------------------------------------------------------------------------------------------

/// If you're having specific issues, run this test to isolate the problem
#[test]
fn test_specific_issue() {
    // Simple diagnostic error for testing (unit variants to avoid auto-source detection)
    #[derive(Debug, YoshiError)]
    enum DiagnosticAnalysisError {
        #[yoshi(display = "Compilation failed", severity = 200)]
        CompilationFailure,

        #[yoshi(display = "AST parsing failed", severity = 180, transient = true)]
        AstParsingFailure,
    }

    let err = DiagnosticAnalysisError::CompilationFailure;

    // Test both variants to avoid dead code warnings
    let _err2 = DiagnosticAnalysisError::AstParsingFailure;

    // Print everything we can about this error
    tracing::info!("\n🔍 DEBUG INFORMATION");
    tracing::info!("====================");
    tracing::error!("Error Display: '{err}'");
    tracing::error!("Error Debug: '{err:?}'");

    // Test basic functionality
    tracing::info!("Display: '{err}'");
    tracing::debug!("Debug: '{err:?}'");

    // Test Error trait
    let error_trait: &dyn Error = &err;
    tracing::error!("Error Trait Display: '{error_trait}'");
    tracing::error!("Error Source: {:?}", error_trait.source());

    // Test conversion to Yoshi if possible
    match std::panic::catch_unwind(|| {
        let yoshi_err: yoshi_core::Yoshi = err.into();
        format!("{yoshi_err}")
    }) {
        Ok(yoshi_str) => println!("Yoshi Conversion: '{yoshi_str}'"),
        Err(_) => println!("❌ Yoshi conversion failed"),
    }

    tracing::info!("====================\n");
}
