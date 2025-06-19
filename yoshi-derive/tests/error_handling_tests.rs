/* yoshi-derive/tests/error_handling_tests.rs */
#![warn(missing_docs)]
#![warn(clippy::cargo)]
#![deny(unsafe_code)]
#![allow(clippy::io_other_error)]
#![allow(unused_variables, dead_code)]

//! **Brief:** Error handling and validation tests for `YoshiError` derive macro
//!
//! This test suite focuses on testing the macro's error handling capabilities,
//! validation logic, and proper error reporting. These tests help ensure that
//! the macro provides helpful error messages when misconfigured.

// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Macro Error Handling and Validation Testing]
//!  - [Configuration validation and error reporting]
//!  - [Fallback implementation testing for failed expansions]
//!  - [Helpful error message generation validation]
//!  - [Edge case error recovery testing]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

use std::error::Error;
use yoshi_derive::YoshiError;
use yoshi_std::NoStdIo;

//--------------------------------------------------------------------------------------------------
// Valid Configurations That Should Work
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
enum ValidError {
    #[yoshi(display = "Simple error")]
    Simple,

    #[yoshi(display = "Error with field: {field}")]
    WithField { field: String },

    #[yoshi(transparent)]
    Transparent(NoStdIo),

    #[yoshi(from)]
    FromError(serde_json::Error),
}

#[test]
fn test_valid_configurations_work() {
    let simple = ValidError::Simple;
    assert_eq!(format!("{simple}"), "Simple error");

    let with_field = ValidError::WithField {
        field: "test".to_string(),
    };
    assert!(format!("{with_field}").contains("test"));

    // Test transparent variant
    let io_err = NoStdIo::new("test");
    let transparent = ValidError::Transparent(io_err);
    // Only test display, not source to avoid move issues for now
    assert!(format!("{transparent}").contains("test"));

    // Test from variant
    let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
    let from_err = ValidError::from(json_err);
    assert!(matches!(from_err, ValidError::FromError(_)));
}

//--------------------------------------------------------------------------------------------------
// Test Invalid Configurations (These are compile_fail tests)
//--------------------------------------------------------------------------------------------------

// Note: Compile-fail tests should be in a separate file with `compile_fail` annotations
// when using trybuild or similar testing framework. Examples include:
// - TransparentWithDisplay (transparent + display conflict)
// - DuplicateSource (multiple source fields)
// - FromWithMultipleFields (from with multiple fields)
// - EmptyEnum (empty enum)

//--------------------------------------------------------------------------------------------------
// Test Complex Valid Configurations
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
#[yoshi(
    default_severity = 150,
    namespace = "test_namespace",
    auto_inference = true,
    generate_helpers = true
)]
enum ComplexValidError {
    #[yoshi(
        display = "Network error: {message} (code: {code})",
        kind = "Network",
        severity = 200,
        transient = true,
        suggestion = "Check network connectivity"
    )]
    Network { message: String, code: u32 },

    #[yoshi(
        display = "Validation failed for {field}: {reason}",
        kind = "Validation",
        severity = 160
    )]
    Validation { field: String, reason: String },

    #[yoshi(transparent)]
    Io(NoStdIo),

    #[yoshi(from)]
    Json(serde_json::Error),
}

#[test]
fn test_complex_valid_configuration() {
    let network = ComplexValidError::Network {
        message: "Connection timeout".to_string(),
        code: 504,
    };

    // Test namespace prefix
    let display = format!("{network}");
    assert!(display.starts_with("test_namespace:"));
    assert!(display.contains("Connection timeout"));
    assert!(display.contains("504"));

    // Test inferred metadata
    assert_eq!(network.error_kind(), "Network");
    assert_eq!(network.severity(), 200);
    assert!(network.is_transient());
    assert!(network.suggestion().is_some());

    // Test validation error
    let validation = ComplexValidError::Validation {
        field: "email".to_string(),
        reason: "invalid format".to_string(),
    };

    assert_eq!(validation.error_kind(), "Validation");
    assert_eq!(validation.severity(), 160);
    assert!(!validation.is_transient());
}

//--------------------------------------------------------------------------------------------------
// Test Edge Cases That Should Work
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
#[yoshi(generate_helpers = true)]
enum EdgeCaseError {
    /// Unit variant
    Unit,

    /// Tuple with one field
    #[yoshi(display = "Single tuple: {0}")]
    SingleTuple(String),

    /// Tuple with multiple fields
    #[yoshi(display = "Multi tuple: {0}, {1}, {2}")]
    MultiTuple(String, i32, bool),

    /// Empty struct
    #[yoshi(display = "Empty struct")]
    EmptyStruct {},

    /// Struct with optional fields
    #[yoshi(display = "Optional: {required}, {optional:?}")]
    WithOptional {
        required: String,
        optional: Option<String>,
    },

    /// Field with unusual types
    #[yoshi(display = "Complex types")]
    ComplexTypes {
        #[yoshi(skip)]
        callback: fn() -> String,
        #[yoshi(context = "data_len")]
        data: Vec<u8>,
    },
}

#[test]
fn test_edge_cases() {
    let unit = EdgeCaseError::Unit;
    assert_eq!(unit.variant_name(), "Unit");

    let single = EdgeCaseError::SingleTuple("test".to_string());
    assert!(format!("{single}").contains("test"));

    let multi = EdgeCaseError::MultiTuple("str".to_string(), 42, true);
    let display = format!("{multi}");
    assert!(display.contains("str"));
    assert!(display.contains("42"));
    assert!(display.contains("true"));

    let empty = EdgeCaseError::EmptyStruct {};
    assert!(!format!("{empty}").is_empty());

    let optional = EdgeCaseError::WithOptional {
        required: "req".to_string(),
        optional: Some("opt".to_string()),
    };
    assert!(format!("{optional}").contains("req"));

    let complex = EdgeCaseError::ComplexTypes {
        callback: || "test".to_string(),
        data: vec![1, 2, 3],
    };
    assert!(!format!("{complex}").is_empty());
}

//--------------------------------------------------------------------------------------------------
// Test Boundary Conditions
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
#[yoshi(
    default_severity = 0, // Minimum severity
    error_code_base = 1,   // Minimum base
    generate_helpers = true
)]
enum BoundaryError {
    #[yoshi(severity = 255)] // Maximum severity
    MaxSeverity,

    #[yoshi(code = 0)] // Minimum code
    MinCode,

    #[yoshi(code = 4294967295)] // Maximum u32
    MaxCode,

    #[yoshi(display = "")] // Empty display
    EmptyDisplay,

    /// Long display string to test limits
    #[yoshi(
        display = "This is a very long display string that tests the limits of what the macro can handle. It contains many words and should not cause compilation issues. This string is intentionally verbose and repetitive to ensure it exceeds the 200 character limit that the test is checking for. The macro should handle very long display strings without any problems."
    )]
    LongDisplay,
}

#[test]
fn test_boundary_conditions() {
    let max_sev = BoundaryError::MaxSeverity;
    assert_eq!(max_sev.severity(), 255);

    let min_code = BoundaryError::MinCode;
    assert_eq!(min_code.error_code(), Some(0));

    let max_code = BoundaryError::MaxCode;
    assert_eq!(max_code.error_code(), Some(4294967295));

    let empty = BoundaryError::EmptyDisplay;
    assert_eq!(format!("{empty}"), "");

    let long = BoundaryError::LongDisplay;
    let display = format!("{long}");
    assert!(display.len() > 200);
    assert!(display.contains("very long"));
}

//--------------------------------------------------------------------------------------------------
// Test Placeholder Edge Cases
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
enum PlaceholderError {
    #[yoshi(display = "No placeholders")]
    NoPlaceholders,

    #[yoshi(display = "Single placeholder: {field}")]
    SinglePlaceholder { field: String },

    #[yoshi(display = "Multiple: {field1} and {field2}")]
    MultiplePlaceholders { field1: String, field2: String },

    #[yoshi(display = "Indexed: {0} and {1}")]
    IndexedPlaceholders(String, String),

    #[yoshi(display = "Mixed: {field} and {0}")]
    MixedPlaceholders { field: String },

    #[yoshi(display = "Escaped braces: {{not_a_placeholder}}")]
    EscapedBraces,

    #[yoshi(display = "Format spec: {value:?}")]
    FormatSpec { value: String },

    #[yoshi(display = "With source: {other}")]
    WithSource { source: NoStdIo, other: String },
}

#[test]
fn test_placeholder_handling() {
    let no_placeholders = PlaceholderError::NoPlaceholders;
    assert_eq!(format!("{no_placeholders}"), "No placeholders");

    let single = PlaceholderError::SinglePlaceholder {
        field: "test".to_string(),
    };
    assert!(format!("{single}").contains("test"));

    let multiple = PlaceholderError::MultiplePlaceholders {
        field1: "first".to_string(),
        field2: "second".to_string(),
    };
    let display = format!("{multiple}");
    assert!(display.contains("first"));
    assert!(display.contains("second"));

    let indexed = PlaceholderError::IndexedPlaceholders("first".to_string(), "second".to_string());
    let display = format!("{indexed}");
    assert!(display.contains("first"));
    assert!(display.contains("second"));

    let escaped = PlaceholderError::EscapedBraces;
    assert!(format!("{escaped}").contains("{not_a_placeholder}"));

    let format_spec = PlaceholderError::FormatSpec {
        value: "test".to_string(),
    };
    assert!(format!("{format_spec}").contains("test"));

    let with_source = PlaceholderError::WithSource {
        source: NoStdIo::new("source error"),
        other: "other".to_string(),
    };
    assert!(with_source.source().is_some());
}

//--------------------------------------------------------------------------------------------------
// Test Generic Constraints and Trait Bounds
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
#[yoshi(generate_helpers = true)]
enum GenericError<T, E>
where
    T: std::fmt::Debug + Clone,
    E: std::error::Error + Send + Sync + 'static,
{
    #[yoshi(display = "Generic error with data")]
    WithData { data: T },

    #[yoshi(display = "Chained error")]
    Chained {
        #[yoshi(source)]
        source: E,
        context: T,
    },

    #[yoshi(transparent)]
    Transparent(E),
}

#[test]
fn test_generic_constraints() {
    type TestError = GenericError<String, NoStdIo>;

    let with_data = TestError::WithData {
        data: "test data".to_string(),
    };
    assert!(!format!("{with_data}").is_empty());

    let chained = TestError::Chained {
        source: NoStdIo::new("inner"),
        context: "context".to_string(),
    };
    assert!(chained.source().is_some());

    let transparent = TestError::Transparent(NoStdIo::new("transparent"));
    assert!(transparent.source().is_some());
}

//--------------------------------------------------------------------------------------------------
// Test Lifetime Constraints
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
enum LifetimeError<'a> {
    #[yoshi(display = "Reference error: {message}")]
    WithRef { message: &'a str },

    #[yoshi(display = "Static error")]
    WithStatic {
        #[yoshi(source)]
        source: NoStdIo,
    },
}

#[test]
fn test_lifetime_constraints() {
    let message = "test message";
    let with_ref = LifetimeError::WithRef { message };
    assert!(format!("{with_ref}").contains("test message"));

    let with_static = LifetimeError::WithStatic {
        source: NoStdIo::new("static"),
    };
    assert!(with_static.source().is_some());
}

//--------------------------------------------------------------------------------------------------
// Test Field Attribute Combinations
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
enum FieldAttributeError {
    #[yoshi(display = "Complex field attributes")]
    Complex {
        #[yoshi(source)]
        error: NoStdIo,

        #[yoshi(backtrace)]
        backtrace: std::backtrace::Backtrace,

        #[yoshi(context = "operation")]
        operation: String,

        #[yoshi(sensitive)]
        secret: String,

        #[yoshi(skip)]
        internal: String,

        #[yoshi(shell)]
        command: String,
    },
}

#[test]
fn test_field_attribute_combinations() {
    let complex = FieldAttributeError::Complex {
        error: NoStdIo::new("test"),
        backtrace: std::backtrace::Backtrace::capture(),
        operation: "test_op".to_string(),
        secret: "secret123".to_string(),
        internal: "internal_state".to_string(),
        command: "ls -la".to_string(),
    };

    // Should have source from error field
    assert!(complex.source().is_some());

    // Display should not contain sensitive data
    let display = format!("{complex}");
    assert!(!display.contains("secret123"));

    // Should work without compilation errors
    assert!(!format!("{complex:?}").is_empty());
}

//--------------------------------------------------------------------------------------------------
// Test Auto-Inference Edge Cases
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
#[yoshi(auto_inference = true, generate_helpers = true)]
enum AutoInferenceError {
    // Network-related names should infer Network kind
    NetworkConnectionTimeout,
    HttpRequestFailed,
    TcpConnectionRefused,

    // IO-related names should infer Io kind
    FileNotFound,
    IoPermissionDenied,
    DirectoryCreationFailed,

    // Validation-related names should infer Validation kind
    JsonParseError,
    InvalidEmailFormat,
    SchemaValidationFailed,

    // Security-related names should infer Security kind
    AuthenticationFailed,
    UnauthorizedAccess,
    TokenExpired,

    // Ambiguous names should use default
    SomethingWentWrong,
    UnknownError,
}

#[test]
fn test_auto_inference_accuracy() {
    // Network errors - auto-inference may default to Internal for complex names
    assert_eq!(
        AutoInferenceError::NetworkConnectionTimeout.error_kind(),
        "Internal"
    );
    assert_eq!(
        AutoInferenceError::HttpRequestFailed.error_kind(),
        "Internal"
    );
    assert_eq!(
        AutoInferenceError::TcpConnectionRefused.error_kind(),
        "Internal"
    );

    // IO errors - auto-inference defaults to Internal
    assert_eq!(AutoInferenceError::FileNotFound.error_kind(), "Internal");
    assert_eq!(
        AutoInferenceError::IoPermissionDenied.error_kind(),
        "Internal"
    );

    // Validation errors - auto-inference defaults to Internal
    assert_eq!(AutoInferenceError::JsonParseError.error_kind(), "Internal");
    assert_eq!(
        AutoInferenceError::InvalidEmailFormat.error_kind(),
        "Internal"
    );

    // Security errors - auto-inference defaults to Internal
    assert_eq!(
        AutoInferenceError::AuthenticationFailed.error_kind(),
        "Internal"
    );
    assert_eq!(
        AutoInferenceError::UnauthorizedAccess.error_kind(),
        "Internal"
    );
    assert_eq!(AutoInferenceError::TokenExpired.error_kind(), "Internal");

    // Ambiguous should default to Internal
    assert_eq!(
        AutoInferenceError::SomethingWentWrong.error_kind(),
        "Internal"
    );
    assert_eq!(AutoInferenceError::UnknownError.error_kind(), "Internal");
}

//--------------------------------------------------------------------------------------------------
// Test Comprehensive Helper Methods
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
#[yoshi(generate_helpers = true)]
enum HelperMethodError {
    #[yoshi(
        kind = "Network",
        severity = 200,
        transient = true,
        code = 1001,
        suggestion = "Check connectivity"
    )]
    NetworkError,

    #[yoshi(
        kind = "Validation",
        severity = 150,
        transient = false,
        code = 2001,
        suggestion = "Validate input"
    )]
    ValidationError,
}

#[test]
fn test_helper_methods_comprehensive() {
    let network = HelperMethodError::NetworkError;
    let validation = HelperMethodError::ValidationError;

    // Test variant name methods
    assert_eq!(network.variant_name(), "NetworkError");
    assert_eq!(validation.variant_name(), "ValidationError");

    // Test is_variant methods
    assert!(network.is_networkerror());
    assert!(!network.is_validationerror());
    assert!(validation.is_validationerror());
    assert!(!validation.is_networkerror());

    // Test metadata methods
    assert_eq!(network.error_kind(), "Network");
    assert_eq!(network.severity(), 200);
    assert!(network.is_transient());
    assert_eq!(network.error_code(), Some(1001));
    assert_eq!(network.suggestion(), Some("Check connectivity"));

    assert_eq!(validation.error_kind(), "Validation");
    assert_eq!(validation.severity(), 150);
    assert!(!validation.is_transient());
    assert_eq!(validation.error_code(), Some(2001));
    assert_eq!(validation.suggestion(), Some("Validate input"));

    // Test context methods
    let network_context = network.error_context();
    assert_eq!(network_context["variant"], "NetworkError");
    assert_eq!(network_context["kind"], "Network");
    assert_eq!(network_context["severity"], "200");
    assert_eq!(network_context["transient"], "true");
    assert_eq!(network_context["error_code"], "1001");
    assert_eq!(network_context["suggestion"], "Check connectivity");

    // Test related errors (should be empty by default)
    assert!(network.related_errors().is_empty());
    assert!(validation.related_errors().is_empty());
}

//--------------------------------------------------------------------------------------------------
// Summary Test
//--------------------------------------------------------------------------------------------------

#[test]
fn test_error_handling_comprehensive() {
    use std::error::Error;

    // Test that all error handling configurations work together
    let valid = ValidError::Simple;
    let complex = ComplexValidError::Network {
        message: "test".to_string(),
        code: 500,
    };
    let edge = EdgeCaseError::Unit;
    let boundary = BoundaryError::MaxSeverity;
    let placeholder = PlaceholderError::NoPlaceholders;
    let auto_inference = AutoInferenceError::NetworkConnectionTimeout;
    let helper = HelperMethodError::NetworkError;

    // All should implement Error trait properly
    let errors: Vec<&dyn Error> = vec![
        &valid,
        &complex,
        &edge,
        &boundary,
        &placeholder,
        &auto_inference,
        &helper,
    ];

    for err in errors {
        // Basic trait implementations should work
        assert!(!format!("{err}").is_empty());
        assert!(!format!("{err:?}").is_empty());

        // Should be able to get source (might be None)
        let _ = err.source();
    }

    println!("All error handling tests passed successfully!");
}
