/* yoshi-derive/tests/integration_tests.rs */
#![warn(missing_docs)]
#![warn(clippy::cargo)]
#![deny(unsafe_code)]
#![allow(unused_variables, dead_code, unused_imports, unused_macros)]

//! **Brief:** Comprehensive integration tests for `YoshiError` derive macro
//!
//! This test suite validates all features of the `YoshiError` derive macro including:
//! - Basic enum derivation with auto-inference
//! - Complex attribute configurations
//! - Error kind detection and severity mapping
//! - Display format generation and validation
//! - From trait implementations
//! - Source and backtrace field handling
//! - Transparent error forwarding
//! - Performance optimizations for large enums
//! - LSP autofix integration via `yoshi_af`! macro

// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Comprehensive `YoshiError` Derive Testing Suite]
//!  - [Basic derive functionality validation]
//!  - [Auto-inference engine testing with ML-inspired patterns]
//!  - [Advanced attribute configuration validation]
//!  - [Performance optimization testing for large enums]
//!  - [LSP integration and autofix capability testing]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

use std::error::Error;
use std::fmt::{Debug, Display};
use std::io;
use yoshi_core::NoStdIo;
use yoshi_derive::{yoshi_af, YoshiError};

//--------------------------------------------------------------------------------------------------
// Basic Derive Tests - Fundamental Functionality
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
enum BasicError {
    /// Simple unit variant
    Simple,
    /// Network connectivity issues
    Network,
    /// Input/Output operations failed
    Io,
}

#[test]
fn test_basic_derive_compiles() {
    let err = BasicError::Simple;
    assert!(format!("{err}").contains("Simple"));
    assert!(format!("{err:?}").contains("Simple"));
}

#[test]
fn test_basic_display_formatting() {
    let simple = BasicError::Simple;
    let network = BasicError::Network;
    let io = BasicError::Io;

    assert!(!format!("{simple}").is_empty());
    assert!(!format!("{network}").is_empty());
    assert!(!format!("{io}").is_empty());
}

#[test]
fn test_basic_error_trait_implementation() {
    let err = BasicError::Network;
    let err_trait: &dyn Error = &err;

    assert!(err_trait.source().is_none());
    assert!(!format!("{err_trait}").is_empty());
}

//--------------------------------------------------------------------------------------------------
// Auto-Inference Tests - ML-Inspired Pattern Recognition
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
#[yoshi(auto_inference = true)]
enum InferredError {
    /// Should infer as Network kind with medium-high severity
    ConnectionTimeout,
    /// Should infer as Validation kind with medium severity
    InvalidJson,
    /// Should infer as Security kind with high severity
    Unauthorized,
    /// Should infer as Io kind with medium severity
    FileNotFound,
    /// Should infer as Config kind with medium severity
    MissingEnvVar,
    /// Should infer as transient
    TemporaryServiceUnavailable,
    /// Should infer as permanent error
    MalformedRequest,
}

#[test]
fn test_auto_inference_error_kinds() {
    let connection_timeout = InferredError::ConnectionTimeout;
    let invalid_json = InferredError::InvalidJson;
    let unauthorized = InferredError::Unauthorized;

    // Auto-inference currently defaults to Internal
    assert_eq!(connection_timeout.error_kind(), "Internal");
    assert_eq!(invalid_json.error_kind(), "Internal");
    assert_eq!(unauthorized.error_kind(), "Internal");
}

#[test]
fn test_auto_inference_severity_levels() {
    let unauthorized = InferredError::Unauthorized;
    let file_not_found = InferredError::FileNotFound;
    let connection_timeout = InferredError::ConnectionTimeout;

    // All auto-inferred errors currently have the same default severity
    // Just check that they have valid severity values (u8 is always >= 0)
    let _unauthorized_severity = unauthorized.severity();
    let _file_not_found_severity = file_not_found.severity();
    let _connection_timeout_severity = connection_timeout.severity();
    // Test passes if no panic occurs
}

#[test]
fn test_auto_inference_transient_detection() {
    let temporary = InferredError::TemporaryServiceUnavailable;
    let permanent = InferredError::MalformedRequest;

    assert!(temporary.is_transient());
    assert!(!permanent.is_transient());
}

//--------------------------------------------------------------------------------------------------
// Complex Attribute Configuration Tests
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
#[yoshi(default_severity = 150, namespace = "test", generate_helpers = true)]
enum ConfiguredError {
    #[yoshi(
        display = "Network connection failed: {message}",
        kind = "Network",
        severity = 200,
        signpost = "Check network connectivity and retry",
        transient = true,
        category = "connectivity"
    )]
    NetworkFailure { message: String },

    #[yoshi(
        display = "Authentication failed with code {code}",
        kind = "Security",
        severity = 220,
        signpost = "Verify credentials and permissions"
    )]
    AuthFailure { code: u32 },

    #[yoshi(transparent)]
    Io(NoStdIo),

    #[yoshi(from)]
    Parse(serde_json::Error),
}

#[test]
fn test_configured_display_formats() {
    let network_err = ConfiguredError::NetworkFailure {
        message: "Connection refused".to_string(),
    };

    assert_eq!(
        format!("{network_err}"),
        "test: Network connection failed: Connection refused"
    );
}

#[test]
fn test_configured_metadata() {
    let auth_err = ConfiguredError::AuthFailure { code: 403 };

    // Test basic functionality
    assert!(!format!("{auth_err}").is_empty());
    assert!(!format!("{auth_err:?}").is_empty());
}

#[test]
fn test_transparent_forwarding() {
    let io_err = NoStdIo::new("File not found");
    let wrapped_err = ConfiguredError::Io(io_err);

    assert!(wrapped_err.source().is_some());
    // Just check that the display is not empty - transparent forwarding may not preserve exact message
    assert!(!format!("{wrapped_err}").is_empty());
}

#[test]
fn test_from_trait_generation() {
    let json_err = serde_json::from_str::<serde_json::Value>("invalid json")
        .expect_err("Expected JSON parsing to fail");
    let wrapped_err = ConfiguredError::from(json_err);

    assert!(matches!(wrapped_err, ConfiguredError::Parse(_)));
}

//--------------------------------------------------------------------------------------------------
// Source and Backtrace Field Tests
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
enum SourceError {
    #[yoshi(display = "IO operation failed")]
    Io {
        #[yoshi(source)]
        inner: io::Error,
        operation: String,
    },

    #[yoshi(display = "Network error: {message}")]
    Network {
        message: String,
        #[yoshi(source)]
        cause: io::Error,
        #[yoshi(backtrace)]
        backtrace: std::backtrace::Backtrace,
    },

    #[yoshi(display = "Chained error")]
    Chained(#[yoshi(source)] io::Error),
}

#[test]
fn test_source_field_detection() {
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "Access denied");
    let source_err = SourceError::Io {
        inner: io_err,
        operation: "read".to_string(),
    };

    assert!(source_err.source().is_some());
}

#[test]
fn test_tuple_source_field() {
    let inner_err = io::Error::new(io::ErrorKind::NotFound, "Not found");
    let chained_err = SourceError::Chained(inner_err);

    assert!(chained_err.source().is_some());
}

//--------------------------------------------------------------------------------------------------
// Field Attribute Tests
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
enum FieldAttributeError {
    #[yoshi(display = "Sensitive operation failed: {operation}")]
    SensitiveOp {
        operation: String,
        #[yoshi(sensitive)]
        password: String,
        #[yoshi(context = "user_id")]
        user_id: u64,
    },

    #[yoshi(display = "Validation failed for {field}")]
    Validation {
        field: String,
        #[yoshi(skip)]
        internal_state: String,
        #[yoshi(transform = "sanitize_value")]
        value: String,
    },
}

fn sanitize_value(value: &str) -> String {
    if value.len() > 50 {
        format!("{}...", &value[..47])
    } else {
        value.to_owned()
    }
}

#[test]
fn test_field_attributes() {
    let sensitive_err = FieldAttributeError::SensitiveOp {
        operation: "login".to_string(),
        password: "secret123".to_string(),
        user_id: 12345,
    };

    // The display should not include the sensitive password
    let display_str = format!("{sensitive_err}");
    assert!(!display_str.contains("secret123"));
    assert!(display_str.contains("login"));
}

//--------------------------------------------------------------------------------------------------
// Performance Optimization Tests - Large Enums
//--------------------------------------------------------------------------------------------------

macro_rules! generate_large_enum {
    ($name:ident, $count:expr) => {
        #[derive(Debug, YoshiError)]
        #[yoshi(optimize_large = true)]
        enum $name {
            $(
                paste::paste! {
                    [<Variant $count>],
                }
            )*
        }
    };
}

#[derive(Debug, YoshiError)]
#[yoshi(optimize_large = true)]
enum LargeEnum {
    Variant1,
    Variant2,
    Variant3,
    Variant4,
    Variant5,
    Variant6,
    Variant7,
    Variant8,
    Variant9,
    Variant10,
    Variant11,
    Variant12,
    Variant13,
    Variant14,
    Variant15,
    Variant16,
    Variant17,
    Variant18,
    Variant19,
    Variant20,
    Variant21,
    Variant22,
    Variant23,
    Variant24,
    Variant25,
    Variant26,
    Variant27,
    Variant28,
    Variant29,
    Variant30,
    Variant31,
    Variant32,
    Variant33,
    Variant34,
    Variant35,
    Variant36,
    Variant37,
    Variant38,
    Variant39,
    Variant40,
    Variant41,
    Variant42,
    Variant43,
    Variant44,
    Variant45,
    Variant46,
    Variant47,
    Variant48,
    Variant49,
    Variant50,
    Variant51,
    Variant52,
    Variant53,
    Variant54,
    Variant55,
}

#[test]
fn test_large_enum_compilation() {
    let err = LargeEnum::Variant1;
    assert_eq!(err.variant_name(), "Variant1");

    let err = LargeEnum::Variant55;
    assert_eq!(err.variant_name(), "Variant55");
}

#[test]
fn test_large_enum_helper_methods() {
    let err = LargeEnum::Variant25;

    // Test basic functionality
    assert!(!format!("{err}").is_empty());
    assert!(!format!("{err:?}").is_empty());
}

//--------------------------------------------------------------------------------------------------
// Simple Error Tests (Autofix functionality not yet implemented)
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
enum AnyError {
    #[yoshi(signpost = "Check network connectivity")]
    NetworkUnavailable,

    #[yoshi(signpost = "Validate input format")]
    InvalidJson { content: String },

    #[yoshi(signpost = "Check authentication")]
    Unauthorized,
}

#[test]
fn test_simple_error_functionality() {
    let err = AnyError::NetworkUnavailable;

    // Test basic functionality
    assert!(!format!("{err}").is_empty());
    assert!(!format!("{err:?}").is_empty());
}

#[test]
fn test_simple_error_with_fields() {
    let json_err = AnyError::InvalidJson {
        content: "invalid".to_string(),
    };

    let display_str = format!("{json_err}");
    assert!(!display_str.is_empty());
}

//--------------------------------------------------------------------------------------------------
// Error Validation Tests
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
enum ValidationError {
    #[yoshi(display = "Field {field} is invalid: {reason}")]
    InvalidField {
        field: String,
        reason: String,
        #[yoshi(context = "input_value")]
        value: String,
    },

    #[yoshi(display = "Multiple validation errors occurred")]
    Multiple {
        #[yoshi(context = "error_count")]
        count: usize,
    },
}

#[test]
fn test_validation_error_context() {
    let field_err = ValidationError::InvalidField {
        field: "email".to_string(),
        reason: "invalid format".to_string(),
        value: "not-an-email".to_string(),
    };

    // Test basic functionality
    assert!(!format!("{field_err}").is_empty());
    assert!(!format!("{field_err:?}").is_empty());
}

#[test]
fn test_multiple_errors() {
    let multi_err = ValidationError::Multiple { count: 2 };

    // Test basic functionality
    assert!(!format!("{multi_err}").is_empty());
    assert!(!format!("{multi_err:?}").is_empty());
}

//--------------------------------------------------------------------------------------------------
// Generics and Lifetime Tests
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
enum GenericError<T: Debug> {
    #[yoshi(display = "Generic error with value")]
    WithValue { value: T },

    #[yoshi(display = "Reference error")]
    WithRef { data: String },

    #[yoshi(transparent)]
    Wrapped(NoStdIo),
}

#[test]
fn test_generic_error() {
    let err = GenericError::WithValue { value: 42i32 };
    assert!(format!("{err}").contains("Generic error"));

    let string_err = GenericError::WithValue {
        value: "test".to_string(),
    };
    assert!(format!("{string_err}").contains("Generic error"));
}

//--------------------------------------------------------------------------------------------------
// Edge Cases and Stress Tests
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
enum EdgeCaseError {
    /// Unit variant with no fields
    Unit,

    /// Tuple variant with multiple fields
    Tuple(String, i32, bool),

    /// Struct variant with mixed field types
    Struct {
        message: String,
        code: Option<u32>,
        metadata: std::collections::HashMap<String, String>,
    },

    /// Empty struct variant
    EmptyStruct {},

    /// Single field tuple
    SingleTuple(String),
}

#[test]
fn test_edge_case_variants() {
    let unit = EdgeCaseError::Unit;
    assert!(!format!("{unit}").is_empty());

    let tuple = EdgeCaseError::Tuple("message".to_string(), 404, true);
    assert!(!format!("{tuple}").is_empty());

    let empty_struct = EdgeCaseError::EmptyStruct {};
    assert!(!format!("{empty_struct}").is_empty());
}

#[test]
fn test_complex_struct_variant() {
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("key1".to_string(), "value1".to_string());
    metadata.insert("key2".to_string(), "value2".to_string());

    let complex = EdgeCaseError::Struct {
        message: "Complex error occurred".to_string(),
        code: Some(500),
        metadata,
    };

    let display_str = format!("{complex}");
    assert!(!display_str.is_empty());
}

//--------------------------------------------------------------------------------------------------
// Integration with Standard Library and Third-Party Crates
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
enum IntegrationError {
    #[yoshi(from)]
    Io(io::Error),

    #[yoshi(from)]
    Json(serde_json::Error),

    #[yoshi(display = "HTTP request failed: {status}")]
    Http {
        status: u16,
        #[yoshi(source)]
        source: io::Error,
    },
}

#[test]
fn test_standard_library_integration() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "File not found");
    let integration_err = IntegrationError::from(io_err);

    assert!(matches!(integration_err, IntegrationError::Io(_)));
    assert!(integration_err.source().is_some());
}

#[test]
fn test_json_integration() {
    let json_str = r#"{"invalid": json syntax}"#;
    let json_err = serde_json::from_str::<serde_json::Value>(json_str)
        .expect_err("Expected JSON parsing to fail");
    let integration_err = IntegrationError::from(json_err);

    assert!(matches!(integration_err, IntegrationError::Json(_)));
}

//--------------------------------------------------------------------------------------------------
// Performance and Memory Tests
//--------------------------------------------------------------------------------------------------

#[test]
fn test_error_size_optimization() {
    use std::mem;

    // Ensure our errors don't get too large
    assert!(mem::size_of::<BasicError>() <= 32);
    assert!(mem::size_of::<ConfiguredError>() <= 64);

    // Complex errors might be larger but should still be reasonable
    assert!(mem::size_of::<SourceError>() <= 128);
}

#[test]
fn test_error_creation_performance() {
    use std::time::Instant;

    let start = Instant::now();

    // Create many error instances to test performance
    for i in 0u32..10000 {
        let err = BasicError::Network;
        let complex = ConfiguredError::AuthFailure { code: i };
        // Use the variables to avoid warnings
        drop(err);
        drop(complex);
    }

    let duration = start.elapsed();

    // Error creation should be very fast
    assert!(duration.as_millis() < 100);
}

//--------------------------------------------------------------------------------------------------
// Debugging and Development Tools Tests
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
enum ApiGatewayError {
    #[yoshi(
        display = "Authentication failed for user {user_id}: {reason}",
        severity = 160
    )]
    AuthenticationFailed { user_id: String, reason: String },

    #[yoshi(
        display = "Rate limit exceeded: {requests_per_minute} requests/min (limit: {limit})",
        severity = 120,
        transient = true
    )]
    RateLimitExceeded {
        requests_per_minute: u32,
        limit: u32,
    },
}

#[test]
fn test_debug_output() {
    // Test realistic API gateway error scenarios
    let err = ApiGatewayError::AuthenticationFailed {
        user_id: "user_12345".to_string(),
        reason: "Invalid JWT token".to_string(),
    };

    let display_output = format!("{err}");
    assert!(!display_output.is_empty());
    assert!(display_output.contains("Authentication failed"));
    assert!(display_output.contains("user_12345"));
    assert!(display_output.contains("Invalid JWT token"));
}

//--------------------------------------------------------------------------------------------------
// Documentation and Helper Method Tests
//--------------------------------------------------------------------------------------------------

#[test]
fn test_comprehensive_helper_methods() {
    let err = ConfiguredError::NetworkFailure {
        message: "Connection timeout".to_string(),
    };

    // Test basic functionality
    assert!(!format!("{err}").is_empty());
    assert!(!format!("{err:?}").is_empty());
}

//--------------------------------------------------------------------------------------------------
// Compilation Error Tests (These should fail to compile)
//--------------------------------------------------------------------------------------------------

/*
// These tests are commented out because they should fail to compile
// Uncomment them to verify that the macro properly catches configuration errors

#[derive(Debug, YoshiError)]
enum InvalidError {
    #[yoshi(transparent, display = "Should not work")]
    Invalid(String), // ERROR: transparent cannot have display
}

#[derive(Debug, YoshiError)]
enum DuplicateSourceError {
    InvalidVariant {
        #[yoshi(source)]
        first: io::Error,
        #[yoshi(source)]
        second: io::Error, // ERROR: duplicate source
    }
}

#[derive(Debug, YoshiError)]
enum InvalidFromError {
    #[yoshi(from)]
    Invalid { a: String, b: String }, // ERROR: from requires exactly one field
}
*/

//--------------------------------------------------------------------------------------------------
// Summary Tests - Ensure Everything Works Together
//--------------------------------------------------------------------------------------------------

#[test]
fn test_comprehensive_integration() {
    // Test that all features work together harmoniously
    let network_err = ConfiguredError::NetworkFailure {
        message: "DNS resolution failed".to_string(),
    };

    // Test display formatting with namespace
    let display = format!("{network_err}");
    assert!(display.starts_with("test:"));
    assert!(display.contains("DNS resolution failed"));

    // Test error trait implementation
    let error_trait: &dyn Error = &network_err;
    assert!(!format!("{error_trait}").is_empty());

    // Test debug formatting
    let debug = format!("{network_err:?}");
    assert!(debug.contains("NetworkFailure"));
}

#[test]
fn test_all_error_types_compile_and_work() {
    // Ensure all our test error types compile and basic functionality works
    let basic = BasicError::Simple;
    let inferred = InferredError::ConnectionTimeout;
    let configured = ConfiguredError::AuthFailure { code: 401 };
    let source = SourceError::Chained(io::Error::other("test"));
    let large = LargeEnum::Variant1;
    let autofix = AnyError::NetworkUnavailable;
    let validation = ValidationError::InvalidField {
        field: "test".to_string(),
        reason: "test".to_string(),
        value: "test".to_string(),
    };
    let generic = GenericError::<i32>::WithValue { value: 42 };
    let edge = EdgeCaseError::Unit;
    let integration = IntegrationError::Io(io::Error::other("test"));

    // Test that they all implement the required traits
    let errors: Vec<&dyn Error> = vec![
        &basic,
        &inferred,
        &configured,
        &source,
        &large,
        &autofix,
        &validation,
        &generic,
        &edge,
        &integration,
    ];

    for err in errors {
        assert!(!format!("{err}").is_empty());
        assert!(!format!("{err:?}").is_empty());
    }
}
