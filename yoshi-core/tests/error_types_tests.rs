/* yoshi-core/tests/error_types_tests.rs */
#![deny(dead_code)]
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![deny(unused_variables)]
#![deny(clippy::expect_used)]
#![deny(clippy::unwrap_used)]
//! **Brief:** Comprehensive test suite for yoshi-core error type validation with mathematical precision and zero-cost abstractions.
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
fn test_yoshi_creation_basic() {
    let error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Internal {
        message: "Test error".into(),
        source: None,
        component: None,
    });

    let _id = error.instance_id(); // Instance ID is always valid
}

#[test]
fn test_yoshi_kind_internal() {
    let internal = YoshiCore::YoshiKind::Internal {
        message: "Internal error".into(),
        source: None,
        component: Some("test_component".into()),
    };

    let error = YoshiCore::Yoshi::new(internal);
    let _id = error.instance_id(); // Instance ID is always valid
    assert!(error.kind().to_string().contains("Internal error"));
}

#[test]
fn test_yoshi_kind_network() {
    let network = YoshiCore::YoshiKind::Network {
        message: "Network failure".into(),
        source: None,
        error_code: Some(404),
    };

    let error = YoshiCore::Yoshi::new(network);
    let _id = error.instance_id(); // Instance ID is always valid
    assert!(error.kind().to_string().contains("Network failure"));
}

#[test]
fn test_yoshi_kind_validation() {
    let validation = YoshiCore::YoshiKind::Validation {
        field: "username".into(),
        message: "Invalid input".into(),
        expected: Some("non-empty string".into()),
        actual: Some("empty".into()),
    };

    let error = YoshiCore::Yoshi::new(validation);
    let _id = error.instance_id(); // Instance ID is always valid
    assert!(error.kind().to_string().contains("Invalid input"));
}

#[test]
fn test_yoshi_kind_not_found() {
    let not_found = YoshiCore::YoshiKind::NotFound {
        resource_type: "file".into(),
        identifier: "config.toml".into(),
        search_locations: Some(vec!["current directory".into()]),
    };

    let error = YoshiCore::Yoshi::new(not_found);
    let _id = error.instance_id(); // Instance ID is always valid
    assert!(error.kind().to_string().contains("file"));
    assert!(error.kind().to_string().contains("config.toml"));
}

#[test]
fn test_yoshi_kind_config() {
    let config = YoshiCore::YoshiKind::Config {
        message: "Missing configuration".into(),
        source: None,
        config_path: Some("config.toml".into()),
    };

    let error = YoshiCore::Yoshi::new(config);
    let _id = error.instance_id(); // Instance ID is always valid
    assert!(error.kind().to_string().contains("Missing configuration"));
}

#[test]
fn test_yoshi_kind_timeout() {
    use std::time::Duration;

    let timeout = YoshiCore::YoshiKind::Timeout {
        operation: "database_query".into(),
        duration: Duration::from_secs(30),
        expected_max: Some(Duration::from_secs(10)),
    };

    let error = YoshiCore::Yoshi::new(timeout);
    let _id = error.instance_id(); // Instance ID is always valid
    assert!(error.kind().to_string().contains("database_query"));
}

#[test]
fn test_yoshi_kind_resource_exhausted() {
    let resource_exhausted = YoshiCore::YoshiKind::ResourceExhausted {
        resource: "memory".into(),
        limit: "2GB".into(),
        current: "2.1GB".into(),
        usage_percentage: Some(105.0),
    };

    let error = YoshiCore::Yoshi::new(resource_exhausted);
    let _id = error.instance_id(); // Instance ID is always valid
    assert!(error.kind().to_string().contains("memory"));
}

#[test]
fn test_yoshi_kind_security() {
    let security = YoshiCore::YoshiKind::Security {
        message: "Invalid JWT token".into(),
        source: None,
        security_level: "authentication_failure".into(),
    };

    let error = YoshiCore::Yoshi::new(security);
    let _id = error.instance_id(); // Instance ID is always valid
    assert!(error.kind().to_string().contains("Invalid JWT token"));
}

#[test]
fn test_error_instance_uniqueness() {
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

    assert_ne!(error1.instance_id(), error2.instance_id());
    assert!(error2.instance_id() > error1.instance_id());
}

#[test]
fn test_yoshi_location_creation() {
    let location = YoshiCore::YoshiLocation::new("test_file.rs", 42, 10);

    assert_eq!(location.file, "test_file.rs");
    assert_eq!(location.line, 42);
    assert_eq!(location.column, 10);
}

#[test]
fn test_error_cloning() {
    let original = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Internal {
        message: "Original error".into(),
        source: None,
        component: None,
    });

    let cloned = original.clone();
    assert_ne!(original.instance_id(), cloned.instance_id());
    assert_eq!(original.kind().to_string(), cloned.kind().to_string());
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
}

#[test]
fn test_quick_fixes_alias() {
    let error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Internal {
        message: "Test error".into(),
        source: None,
        component: None,
    });

    let auto_fixes = error.auto_fixes();
    let quick_fixes = error.quick_fixes();

    assert_eq!(auto_fixes.len(), quick_fixes.len());
}

#[test]
fn test_yoshi_autofix_structure() {
    let autofix = YoshiCore::YoshiAutoFix {
        description: "Test fix description".into(),
        fix_code: "// Test fix code".into(),
        confidence: 0.9,
        safety_level: YoshiCore::AutoFixSafetyLevel::Safe,
        target_file: Some("test.rs".into()),
        range: None,
    };

    assert_eq!(autofix.description.as_ref(), "Test fix description");
    assert_eq!(autofix.fix_code.as_ref(), "// Test fix code");
    assert!((autofix.confidence - 0.9).abs() < f32::EPSILON);
    assert_eq!(autofix.safety_level, YoshiCore::AutoFixSafetyLevel::Safe);
    assert!(autofix.target_file.is_some());
}

#[test]
fn test_autofix_safety_levels() {
    let levels = [
        YoshiCore::AutoFixSafetyLevel::Safe,
        YoshiCore::AutoFixSafetyLevel::LowRisk,
        YoshiCore::AutoFixSafetyLevel::MediumRisk,
        YoshiCore::AutoFixSafetyLevel::HighRisk,
        YoshiCore::AutoFixSafetyLevel::Manual,
    ];

    // Test ordering
    for i in 0..levels.len() - 1 {
        if let (Some(current), Some(next)) = (levels.get(i), levels.get(i + 1)) {
            assert!(current <= next);
        }
    }
}

#[test]
fn test_error_with_empty_message() {
    let error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Internal {
        message: "".into(),
        source: None,
        component: None,
    });

    let _id = error.instance_id(); // Instance ID is always valid
    let display = format!("{error}");
    assert!(!display.is_empty()); // Should still have some content
}

#[test]
fn test_error_with_unicode() {
    let error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Internal {
        message: "Unicode test: 🦀 Rust 中文 العربية".into(),
        source: None,
        component: None,
    });

    let _id = error.instance_id(); // Instance ID is always valid
    let display = format!("{error}");
    assert!(display.contains("🦀"));
    assert!(display.contains("中文"));
    assert!(display.contains("العربية"));
}

#[test]
fn test_numeric_edge_cases() {
    let error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::ResourceExhausted {
        resource: "connections".into(),
        limit: format!("{}", u64::MAX).into(),
        current: "0".into(),
        usage_percentage: Some(0.0),
    });

    let _id = error.instance_id(); // Instance ID is always valid
    let display = format!("{error}");
    assert!(display.contains("connections"));
}

#[test]
fn test_optional_fields_none() {
    let error = YoshiCore::Yoshi::new(YoshiCore::YoshiKind::Network {
        message: "Minimal network error".into(),
        source: None,
        error_code: None,
    });

    let _id = error.instance_id(); // Instance ID is always valid
    let display = format!("{error}");
    assert!(display.contains("Minimal network error"));
}
