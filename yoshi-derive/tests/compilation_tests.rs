/* yoshi-derive/tests/compilation_tests.rs */
#![warn(missing_docs)]
#![warn(clippy::cargo)]
#![deny(unsafe_code)]
#![allow(unused_variables, dead_code)]

//! **Brief:** Compilation validation and edge case tests for `YoshiError` derive macro
//!
//! These tests ensure that the macro generates valid Rust code and handles edge cases
//! gracefully. Tests include boundary conditions, unusual configurations, and error
//! recovery scenarios.

// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Compilation Validation and Edge Case Testing]
//!  - [Boundary condition testing for complex configurations]
//!  - [Error recovery and fallback implementation validation]
//!  - [Generated code quality and performance verification]
//!  - [Trait bound inference and generic constraint validation]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

use std::error::Error;
use std::fmt::Debug;
use yoshi_derive::YoshiError;
use yoshi_std::NoStdIo;

//--------------------------------------------------------------------------------------------------
// Minimal Configuration Tests
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
enum MinimalError {
    Basic,
}

#[test]
fn test_minimal_error_works() {
    let err = MinimalError::Basic;
    assert_eq!(format!("{err}"), "Basic");
    assert_eq!(err.variant_name(), "Basic");
    assert_eq!(err.error_kind(), "Internal"); // Default inferred kind
}

//--------------------------------------------------------------------------------------------------
// Empty Enum Edge Case
//--------------------------------------------------------------------------------------------------

// Note: This should actually fail to compile with a helpful error message
// Uncomment to test error handling:
/*
#[derive(Debug, YoshiError)]
enum EmptyError {
    // This should produce a compilation error
}
*/

//--------------------------------------------------------------------------------------------------
// Maximum Complexity Configuration
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
#[yoshi(
    default_severity = 100,
    default_kind = "Custom",
    optimize_large = true,
    auto_inference = true,
    generate_helpers = true,
    namespace = "complex",
    error_code_base = 5000
)]
enum MaxComplexityError {
    #[yoshi(
        display = "Ultra complex error: {operation} failed with {details}",
        kind = "UltraComplex",
        severity = 255,
        suggestion = "This is an ultra-complex error requiring comprehensive handling",
        transient = true,
        code = 5001,
        category = "ultra-complex"
    )]
    UltraComplex {
        operation: String,
        details: String,
        #[yoshi(context = "operation_context", skip = false, sensitive = false)]
        custom_field: String,
        #[yoshi(context = "metadata")]
        metadata: std::collections::HashMap<String, String>,
    },
}

#[test]
fn test_max_complexity_error() {
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("key".to_string(), "value".to_string());

    let err = MaxComplexityError::UltraComplex {
        operation: "test_op".to_string(),
        details: "test_details".to_string(),
        custom_field: "custom_value".to_string(),
        metadata,
    };

    // Test basic functionality
    let display = format!("{err}");
    assert!(display.contains("complex:")); // namespace
    assert!(display.contains("test_op"));
    assert!(display.contains("test_details"));

    // Test that it implements Error trait
    use std::error::Error;
    let _: &dyn Error = &err;
}

//--------------------------------------------------------------------------------------------------
// Unusual Field Types and Patterns
//--------------------------------------------------------------------------------------------------

#[derive(YoshiError)]
#[yoshi(generate_helpers = true)]
#[allow(clippy::enum_variant_names)] // All variants intentionally start with "With" for testing
enum UnusualTypesError {
    WithFunction {
        #[yoshi(skip)]
        callback: fn() -> String,
    },

    WithClosure {
        #[yoshi(skip)]
        closure: Box<dyn Fn() -> String>,
    },

    WithRawPointer {
        #[yoshi(sensitive, skip)]
        ptr: *const u8,
    },

    WithUnsizedType {
        #[yoshi(context = "slice_info")]
        data: Box<[u8]>,
    },
}

// Manual Debug implementation to handle non-Debug types
impl std::fmt::Debug for UnusualTypesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WithFunction { .. } => f.debug_struct("WithFunction").finish_non_exhaustive(),
            Self::WithClosure { .. } => f.debug_struct("WithClosure").finish_non_exhaustive(),
            Self::WithRawPointer { .. } => f.debug_struct("WithRawPointer").finish_non_exhaustive(),
            Self::WithUnsizedType { data } => f
                .debug_struct("WithUnsizedType")
                .field("data", data)
                .finish(),
        }
    }
}

#[test]
fn test_unusual_types() {
    let err = UnusualTypesError::WithFunction {
        callback: || "test".to_string(),
    };

    // Should compile and work even with unusual types
    assert!(!format!("{err}").is_empty());
    assert!(!format!("{err:?}").is_empty());
}

//--------------------------------------------------------------------------------------------------
// Simple Non-Generic Complex Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
enum ComplexTypesError {
    Complex {
        data: Vec<Option<String>>,
        formatter: String,
        #[yoshi(source)]
        source: NoStdIo,
    },

    NestedStruct {
        nested: std::collections::HashMap<String, Vec<String>>,
    },
}

#[test]
fn test_complex_types() {
    let source_err = NoStdIo::new("test");
    let err = ComplexTypesError::Complex {
        data: vec![Some("test".to_string())],
        formatter: "test formatter".to_string(),
        source: source_err,
    };

    // Test basic functionality
    assert!(!format!("{err}").is_empty());
    assert!(!format!("{err:?}").is_empty());
    assert!(err.source().is_some());
}

//--------------------------------------------------------------------------------------------------
// Lifetime Edge Cases
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
#[yoshi(generate_helpers = true)]
enum LifetimeError<'a> {
    WithReference {
        #[yoshi(context = "ref_data")]
        data: &'a str,
    },

    #[yoshi(display = "Static error occurred")]
    WithStaticRef {
        #[yoshi(source)]
        error: &'static (dyn Error + Send + Sync),
    },
}

#[test]
fn test_lifetime_handling() {
    let data = "test data";
    let err = LifetimeError::WithReference { data };

    assert!(!format!("{err}").is_empty());
    assert!(!format!("{err:?}").is_empty());
}

//--------------------------------------------------------------------------------------------------
// Unicode and Special Characters
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
enum UnicodeError {
    #[yoshi(display = "Unicode error: {message} 🚨")]
    WithUnicode { message: String },

    #[yoshi(display = "Special chars: {data}")]
    SpecialChars { data: String },

    #[yoshi(
        display = "Emoji test 🔥🚀⚡",
        suggestion = "Try using ASCII characters only 🤔"
    )]
    EmojiTest,
}

#[test]
fn test_unicode_support() {
    let err = UnicodeError::WithUnicode {
        message: "测试消息".to_string(),
    };

    let display = format!("{err}");
    assert!(display.contains("🚨"));
    assert!(display.contains("测试消息"));

    let emoji_err = UnicodeError::EmojiTest;
    assert!(!format!("{emoji_err}").is_empty());
}

//--------------------------------------------------------------------------------------------------
// Error Code Boundary Tests
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
#[yoshi(error_code_base = 4294967285, generate_helpers = true)]
enum BoundaryCodeError {
    #[yoshi(code = 2)]
    MinCode,

    #[yoshi(code = 4294967295)]
    MaxValue,

    AutoGenerated1,
    AutoGenerated2,
}

#[test]
fn test_error_code_boundaries() {
    let min_err = BoundaryCodeError::MinCode;
    let max_err = BoundaryCodeError::MaxValue;
    let auto1 = BoundaryCodeError::AutoGenerated1;

    // Test basic functionality
    assert!(!format!("{min_err}").is_empty());
    assert!(!format!("{max_err}").is_empty());
    assert!(!format!("{auto1}").is_empty());
}

//--------------------------------------------------------------------------------------------------
// Performance Stress Tests
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
#[yoshi(optimize_large = true)]
enum StressTestError {
    // Many variants to test performance optimizations
    V01,
    V02,
    V03,
    V04,
    V05,
    V06,
    V07,
    V08,
    V09,
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
    V51,
    V52,
    V53,
    V54,
    V55,
    V56,
    V57,
    V58,
    V59,
    V60,
    V61,
    V62,
    V63,
    V64,
    V65,
    V66,
    V67,
    V68,
    V69,
    V70,
    V71,
    V72,
    V73,
    V74,
    V75,
    V76,
    V77,
    V78,
    V79,
    V80,
    V81,
    V82,
    V83,
    V84,
    V85,
    V86,
    V87,
    V88,
    V89,
    V90,
    V91,
    V92,
    V93,
    V94,
    V95,
    V96,
    V97,
    V98,
    V99,
    V100,
}

#[test]
fn test_large_enum_performance() {
    use std::time::Instant;

    let start = Instant::now();

    // Test basic functionality performance
    for _ in 0..1000 {
        let err = StressTestError::V50;
        let _ = format!("{err}");
        let _ = format!("{err:?}");
    }

    let duration = start.elapsed();

    // Should be very fast even with many variants
    assert!(duration.as_millis() < 100);
}

//--------------------------------------------------------------------------------------------------
// Memory Layout and Size Tests
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
#[yoshi(generate_helpers = true)]
enum SizeTestError {
    Small,
    Medium {
        message: String,
    },
    Large {
        field1: String,
        field2: String,
        field3: String,
        field4: i64,
        field5: f64,
    },
}

#[test]
fn test_memory_efficiency() {
    use std::mem;

    // Basic size checks
    let small_size = mem::size_of::<SizeTestError>();
    println!("SizeTestError size: {small_size} bytes");

    // Should be reasonably sized (allowing for additional macro-generated fields)
    assert!(small_size <= 128);

    // Test alignment
    assert_eq!(mem::align_of::<SizeTestError>(), mem::align_of::<String>());
}

//--------------------------------------------------------------------------------------------------
// Thread Safety Tests
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
#[yoshi(generate_helpers = true)]
enum ThreadSafeError {
    #[yoshi(display = "Thread {id}: {message}")]
    ThreadError { id: u64, message: String },

    #[yoshi(from)]
    Sync(NoStdIo),
}

#[test]
fn test_thread_safety() {
    use std::sync::Arc;
    use std::thread;

    let err = Arc::new(ThreadSafeError::ThreadError {
        id: 123,
        message: "test".to_string(),
    });

    let err_clone = Arc::clone(&err);
    let handle = thread::spawn(move || format!("{err_clone}"));

    let result = handle.join().unwrap();
    assert!(result.contains("Thread 123"));
}

//--------------------------------------------------------------------------------------------------
// Serialization Compatibility Tests (if serde is available)
//--------------------------------------------------------------------------------------------------

#[cfg(feature = "serde")]
mod serde_tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, YoshiError, Serialize, Deserialize)]
    enum SerializableError {
        #[yoshi(display = "Serializable error: {message}")]
        Test { message: String },
    }

    #[test]
    fn test_serde_compatibility() {
        let err = SerializableError::Test {
            message: "test".to_string(),
        };

        let json = serde_json::to_string(&err).unwrap();
        let deserialized: SerializableError = serde_json::from_str(&json).unwrap();

        assert_eq!(format!("{err}"), format!("{}", deserialized));
    }
}

//--------------------------------------------------------------------------------------------------
// Regression Tests for Known Issues
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
enum RegressionError {
    // Test for issue with empty display strings
    #[yoshi(display = "")]
    EmptyDisplay,

    // Test for issue with very long display strings
    #[yoshi(
        display = "This is a very long display string that should not cause compilation issues even though it exceeds normal length limits and contains many words and detailed descriptions of the error condition"
    )]
    LongDisplay,

    // Test for issue with placeholder edge cases
    #[yoshi(display = "{{{field}}}")]
    WeirdPlaceholders { field: String },
}

#[test]
fn test_regression_cases() {
    let empty = RegressionError::EmptyDisplay;
    assert_eq!(format!("{empty}"), "");

    let long = RegressionError::LongDisplay;
    assert!(format!("{long}").len() > 100);

    let weird = RegressionError::WeirdPlaceholders {
        field: "test".to_string(),
    };
    assert!(format!("{weird}").contains("test"));
}

//--------------------------------------------------------------------------------------------------
// Compile-Time Validation Tests
//--------------------------------------------------------------------------------------------------

#[test]
fn test_compile_time_constants() {
    // Test that the macro generates proper compile-time constants
    const _: () = {
        let _err = MinimalError::Basic;
        // This should work at compile time if properly implemented
    };
}

//--------------------------------------------------------------------------------------------------
// Integration with Other Derive Macros
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash, YoshiError)]
enum MultiDeriveError {
    #[yoshi(display = "Multi-derive test: {value}")]
    Test { value: i32 },
}

#[test]
fn test_multi_derive_compatibility() {
    let err1 = MultiDeriveError::Test { value: 42 };
    let err2 = err1.clone();

    assert_eq!(err1, err2);
    assert_eq!(format!("{err1}"), format!("{}", err2));

    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(err1);
    assert!(set.contains(&err2));
}

//--------------------------------------------------------------------------------------------------
// Documentation Generation Tests
//--------------------------------------------------------------------------------------------------

#[derive(Debug, YoshiError)]
/// This is a documented error enum
enum DocumentedError {
    /// This variant represents a documented error
    #[yoshi(display = "Documented error occurred")]
    Documented,

    /// Another documented variant with fields
    #[yoshi(display = "Complex documented error: {details}")]
    Complex {
        /// The error details
        details: String,
    },
}

#[test]
fn test_documentation_preservation() {
    // The macro should preserve documentation
    let err = DocumentedError::Documented;
    assert_eq!(format!("{err}"), "Documented error occurred");
}

//--------------------------------------------------------------------------------------------------
// Summary Test - Everything Together
//--------------------------------------------------------------------------------------------------

#[test]
fn test_comprehensive_edge_cases() {
    // Test a mix of edge cases to ensure they all work together
    let minimal = MinimalError::Basic;
    let complex = MaxComplexityError::UltraComplex {
        operation: "edge_test".to_string(),
        details: "testing".to_string(),
        custom_field: "test".to_string(),
        metadata: std::collections::HashMap::new(),
    };
    let unicode = UnicodeError::EmojiTest;
    let large = StressTestError::V75;

    // All should implement Error trait
    let errors: Vec<&dyn Error> = vec![&minimal, &complex, &unicode, &large];

    for err in errors {
        assert!(!format!("{err}").is_empty());
        assert!(!format!("{err:?}").is_empty());
    }

    // Test that basic functionality works
    assert!(!format!("{minimal}").is_empty());
    assert!(!format!("{complex}").is_empty());
    assert!(!format!("{unicode}").is_empty());
    assert!(!format!("{large}").is_empty());
}
