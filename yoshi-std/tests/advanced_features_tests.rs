/* yoshi-std/tests/advanced_features_tests.rs */
#![deny(unsafe_code)]
#![warn(missing_docs)]
//! **Brief:** Elite test suite for yoshi-std advanced features with mathematical precision validation.
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + Advanced extension traits with fluent API design and method chaining
//!  - `HatchExt` trait functionality with nest enrichment capabilities
//!  - Error conversion and handling with seamless integration
//!  - Type alias ergonomics with Result type validation
//!  - Performance optimization with string interning and memory efficiency
//!  - Advanced error handling patterns and best practices
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

use std::io::{self, ErrorKind};
use yoshi_std::{io_error_to_yoshi, Hatch, HatchExt, Result, Yoshi, YoshiKind};

#[test]
fn test_hatch_ext_nest() {
    // Create a Hatch result first (Result<T, Yoshi>)
    let hatch_result: Hatch<String> = Err(Yoshi::new(YoshiKind::Internal {
        message: "Base error".into(),
        source: None,
        component: None,
    }));

    // Convert to Result<T, AnyError> using true dynamic adaptability
    let result: Result<String> = hatch_result.to_result();

    let enriched = result.nest("Operation failed");
    assert!(enriched.is_err());

    let error = enriched.unwrap_err();
    let nests: Vec<_> = error.nests().collect();
    assert!(!nests.is_empty());
}

#[test]
fn test_io_error_conversion_with_nest() {
    let io_error = io::Error::new(ErrorKind::NotFound, "File not found");

    // Create a Hatch result first, then convert to Result using true dynamic adaptability
    let hatch_result: Hatch<String> = Err(io_error_to_yoshi(io_error));
    let result: Result<String> = hatch_result.to_result();

    let enriched = result.nest("Loading application configuration");
    assert!(enriched.is_err());

    let error = enriched.unwrap_err();
    let nests: Vec<_> = error.nests().collect();
    assert!(!nests.is_empty());
}

#[test]
fn test_type_alias_ergonomics() {
    fn returns_result() -> String {
        "success".to_string()
    }

    fn returns_error() -> Result<i32> {
        // Create a Hatch result first, then convert to Result using true dynamic adaptability
        let hatch_result: Hatch<i32> = Err(Yoshi::new(YoshiKind::Internal {
            message: "Test error".into(),
            source: None,
            component: None,
        }));
        hatch_result.to_result()
    }

    assert_eq!(returns_result(), "success");
    assert!(returns_error().is_err());
}

#[test]
fn test_error_conversion_chain() {
    fn complex_operation() -> Result<String> {
        let io_error = io::Error::new(ErrorKind::NotFound, "config.toml not found");
        // Create a Hatch result first, then convert to Result using true dynamic adaptability
        let hatch_result: Hatch<String> = Err(io_error_to_yoshi(io_error));
        let result: Result<String> = hatch_result.to_result();

        result.nest("Reading configuration file")?;

        Ok("success".to_string())
    }

    let result = complex_operation();
    assert!(result.is_err());

    let error = result.unwrap_err();
    let nests: Vec<_> = error.nests().collect();
    assert!(!nests.is_empty());
}

#[test]
fn test_method_chaining_fluency() {
    // Create a Hatch result first, then convert to Result using true dynamic adaptability
    let hatch_result: Hatch<String> = Err(Yoshi::new(YoshiKind::Internal {
        message: "Base error".into(),
        source: None,
        component: None,
    }));
    let result: Result<String> = hatch_result.to_result();

    let enriched = result
        .nest("First nest")
        .with_signpost("Try this")
        .with_priority(100)
        .nest("Short nest")
        .help("Additional help");

    assert!(enriched.is_err());
    let error = enriched.unwrap_err();
    assert!(error.signpost().is_some());
}

#[test]
fn test_io_error_specific_handling() {
    let error_types = [
        (ErrorKind::NotFound, "File not found"),
        (ErrorKind::PermissionDenied, "Permission denied"),
        (ErrorKind::TimedOut, "Operation timed out"),
        (ErrorKind::Interrupted, "Operation interrupted"),
    ];

    for (kind, message) in error_types {
        let io_error = io::Error::new(kind, message);
        let error = io_error_to_yoshi(io_error);
        // Just check that the error was converted successfully
        assert!(error.instance_id() > 0);
        let error_str = error.kind().to_string();
        assert!(!error_str.is_empty());
    }
}

#[test]
fn test_nested_error_propagation() {
    fn level_3() -> Result<String> {
        let io_error = io::Error::new(ErrorKind::NotFound, "missing.txt");
        // Create a Hatch result first, then convert to Result using true dynamic adaptability
        let hatch_result: Hatch<String> = Err(io_error_to_yoshi(io_error));
        hatch_result.nest("Level 3: Reading file").to_result()
    }

    fn level_2() -> Result<String> {
        level_3()
            .to_hatch()
            .nest("Level 2: Processing data")
            .to_result()
    }

    fn level_1() -> Result<String> {
        level_2()
            .to_hatch()
            .nest("Level 1: Main operation")
            .to_result()
    }

    let result = level_1();
    assert!(result.is_err());

    let error = result.unwrap_err();
    let nests: Vec<_> = error.nests().collect();
    assert!(nests.len() >= 3);
}

#[test]
fn test_error_suggestion_system() {
    let suggestions = [
        "Check file permissions",
        "Verify network connectivity",
        "Increase timeout value",
        "Validate input parameters",
        "Review configuration settings",
    ];

    for suggestion in suggestions {
        // Create a Hatch result first, then convert to Result using true dynamic adaptability
        let hatch_result: Hatch<()> = Err(Yoshi::new(YoshiKind::Internal {
            message: "Test error".into(),
            source: None,
            component: None,
        }));
        let result: Result<()> = hatch_result.with_signpost(suggestion).to_result();

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(
            error.signpost().expect("suggestion should be present"),
            suggestion
        );
    }
}
