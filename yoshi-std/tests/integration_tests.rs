/* tests/integration_tests.rs */
//! Core integration tests for the Yoshi error handling framework.
//! 
//! This module tests the fundamental functionality of Yoshi errors including
//! creation, chaining, conversion, and basic error handling patterns.

use yoshi_std::{Yoshi, YoshiKind, YoshiLocation, Result, error_instance_count};
use core::time::Duration;
use std::error::Error;

#[test]
fn test_basic_error_creation() {
    let initial_count = error_instance_count();
    
    let err = Yoshi::new(YoshiKind::Internal {
        message: "Critical system failure".into(),
        source: None,
        component: Some("DatabaseEngine".into()),
    });
    
    assert_eq!(error_instance_count(), initial_count + 1);
    assert_eq!(err.severity(), 80);
    assert!(!err.is_transient());
    assert!(err.instance_id() > 0);
}

#[test]
fn test_all_yoshikind_variants() {
    // Test I/O error variant
    #[cfg(feature = "std")]
    {
        use std::io::{Error as IoError, ErrorKind};
        let io_err = IoError::new(ErrorKind::NotFound, "file.txt");
        let yoshi_io = Yoshi::new(YoshiKind::Io(io_err));
        assert_eq!(yoshi_io.severity(), 40);
    }
    
    #[cfg(not(feature = "std"))]
    {
        use yoshi_std::NoStdIo;
        let no_std_io = NoStdIo::new("file not found");
        let yoshi_io = Yoshi::new(YoshiKind::Io(no_std_io));
        assert_eq!(yoshi_io.severity(), 40);
    }
    
    // Test Network error variant
    let network_err = Yoshi::new(YoshiKind::Network {
        message: "Connection timeout".into(),
        source: None,
        error_code: Some(408),
    });
    assert_eq!(network_err.severity(), 50);
    assert!(network_err.is_transient());
    
    // Test Config error variant
    let config_err = Yoshi::new(YoshiKind::Config {
        message: "Invalid configuration key".into(),
        source: None,
        config_path: Some("/etc/app/config.toml".into()),
    });
    assert_eq!(config_err.severity(), 30);
    assert!(!config_err.is_transient());
    
    // Test Validation error variant
    let validation_err = Yoshi::new(YoshiKind::Validation {
        field: "email".into(),
        message: "Invalid email format".into(),
        expected: Some("user@domain.com".into()),
        actual: Some("invalid-email".into()),
    });
    assert_eq!(validation_err.severity(), 20);
    
    // Test NotFound error variant
    let not_found_err = Yoshi::new(YoshiKind::NotFound {
        resource_type: "User".into(),
        identifier: "john.doe@example.com".into(),
        search_locations: Some(vec!["database".into(), "cache".into()]),
    });
    assert_eq!(not_found_err.severity(), 25);
    
    // Test Timeout error variant
    let timeout_err = Yoshi::new(YoshiKind::Timeout {
        operation: "API call to user service".into(),
        duration: Duration::from_secs(30),
        expected_max: Some(Duration::from_secs(5)),
    });
    assert_eq!(timeout_err.severity(), 45);
    assert!(timeout_err.is_transient());
    
    // Test ResourceExhausted error variant
    let resource_err = Yoshi::new(YoshiKind::ResourceExhausted {
        resource: "memory".into(),
        limit: "2GB".into(),
        current: "2.1GB".into(),
        usage_percentage: Some(105.0),
    });
    assert_eq!(resource_err.severity(), 70);
    assert!(resource_err.is_transient());
    
    // Test Multiple errors variant
    let errors = vec![
        Yoshi::new(YoshiKind::Internal { message: "Error 1".into(), source: None, component: None }),
        Yoshi::new(YoshiKind::Internal { message: "Error 2".into(), source: None, component: None }),
        Yoshi::new(YoshiKind::Internal { message: "Error 3".into(), source: None, component: None }),
    ];
    let multiple_err = Yoshi::new(YoshiKind::Multiple {
        errors,
        primary_index: Some(1),
    });
    assert_eq!(multiple_err.severity(), 65);
}

#[test]
fn test_foreign_error_integration() {
    #[derive(Debug)]
    struct CustomApiError {
        code: u16,
        message: String,
    }
    
    impl std::fmt::Display for CustomApiError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "API Error {}: {}", self.code, self.message)
        }
    }
    
    impl std::error::Error for CustomApiError {}
    
    let custom_err = CustomApiError {
        code: 500,
        message: "Internal server error".to_string(),
    };
    
    let yoshi_err = Yoshi::foreign(custom_err);
    assert_eq!(yoshi_err.severity(), 60);
    
    let err_string = format!("{}", yoshi_err);
    assert!(err_string.contains("integration_tests::CustomApiError"));
    assert!(err_string.contains("API Error 500: Internal server error"));
}

#[test]
fn test_error_source_chain() {
    // Create a nested error chain
    let root_cause = Yoshi::new(YoshiKind::Network {
        message: "Connection refused".into(),
        source: None,
        error_code: Some(111),
    });
    
    let intermediate_cause = Yoshi::new(YoshiKind::Internal {
        message: "Failed to establish database connection".into(),
        source: Some(Box::new(root_cause)),
        component: Some("DatabasePool".into()),
    });
    
    let top_level_error = Yoshi::new(YoshiKind::Internal {
        message: "User authentication failed".into(),
        source: Some(Box::new(intermediate_cause)),
        component: Some("AuthService".into()),
    });
    
    // Test source chain traversal
    let mut current_source = top_level_error.source();
    let mut chain_length = 0;
    
    while let Some(source) = current_source {
        chain_length += 1;
        current_source = source.source();
        
        // Prevent infinite loops in test
        if chain_length > 10 {
            break;
        }
    }
    
    assert_eq!(chain_length, 2); // intermediate_cause and root_cause
    
    // Test display includes full chain
    let error_display = format!("{}", top_level_error);
    assert!(error_display.contains("User authentication failed"));
    assert!(error_display.contains("Failed to establish database connection"));
}

#[test]
fn test_location_capture() {
    let location = YoshiLocation::new("src/lib.rs", 100, 25);
    assert_eq!(location.file, "src/lib.rs");
    assert_eq!(location.line, 100);
    assert_eq!(location.column, 25);
    assert_eq!(location.filename(), "lib.rs");
    assert_eq!(format!("{}", location), "lib.rs:100:25");
    
    // Test filename extraction with different path separators
    let windows_path = YoshiLocation::new("C:\\Users\\dev\\project\\src\\main.rs", 50, 10);
    assert_eq!(windows_path.filename(), "main.rs");
    
    let unix_path = YoshiLocation::new("/home/user/project/src/utils.rs", 75, 15);
    assert_eq!(unix_path.filename(), "utils.rs");
}

#[test]
fn test_macro_location_capture() {
    use yoshi_std::yoshi_location;
    
    let loc = yoshi_location!();
    assert!(loc.file.ends_with("integration_tests.rs"));
    assert!(loc.line > 0);
    assert!(loc.column > 0);
    
    // Verify the location points to this test
    let current_line = line!();
    // The macro call should be close to the current line
    assert!((loc.line as i32 - current_line as i32).abs() < 5);
}

#[test]
fn test_error_chain_with_cycle_protection() {    // Create a complex error chain to test cycle detection
    let mut errors: Vec<Yoshi> = Vec::new();
    
    for i in 0..50 {
        let err = Yoshi::new(YoshiKind::Internal {
            message: format!("Error level {}", i).into(),
            source: if i > 0 { Some(Box::new(errors[i-1].clone())) } else { None },
            component: Some(format!("Component{}", i).into()),
        });
        errors.push(err);
    }
    
    // The display should handle deep chains gracefully
    let deep_error = &errors[49];
    let error_display = format!("{}", deep_error);
    
    // Should contain truncation message for deep chains
    assert!(error_display.contains("Error level 49"));
    assert!(error_display.len() > 100); // Should have substantial content
    
    // Should not crash or cause stack overflow
    assert!(error_display.len() < 100_000); // But should be reasonable size
}

#[test]
fn test_multiple_errors_handling() {
    let errors = vec![
        Yoshi::new(YoshiKind::Validation {
            field: "username".into(),
            message: "Username too short".into(),
            expected: Some("at least 3 characters".into()),
            actual: Some("ab".into()),
        }),
        Yoshi::new(YoshiKind::Validation {
            field: "password".into(),
            message: "Password too weak".into(),
            expected: Some("8+ chars with special characters".into()),
            actual: Some("123".into()),
        }),
        Yoshi::new(YoshiKind::Validation {
            field: "email".into(),
            message: "Invalid email format".into(),
            expected: Some("valid email address".into()),
            actual: Some("not-an-email".into()),
        }),
    ];
    
    let multiple_err = Yoshi::new(YoshiKind::Multiple {
        errors: errors.clone(),
        primary_index: Some(1), // Password error is primary
    });
    
    let display = format!("{}", multiple_err);
    assert!(display.contains("Multiple errors (3 total)"));
    assert!(display.contains("Primary: Validation error for 'password'"));
    
    // Test without primary index
    let multiple_err_no_primary = Yoshi::new(YoshiKind::Multiple {
        errors,
        primary_index: None,
    });
    
    let display_no_primary = format!("{}", multiple_err_no_primary);
    assert!(display_no_primary.contains("Multiple errors (3 total)"));
}

#[test]
fn test_result_type_alias() {
    fn test_function() -> Result<String> {
        Ok("success".to_string())
    }
    
    fn failing_function() -> Result<String> {
        Err(Yoshi::new(YoshiKind::Internal {
            message: "Function failed".into(),
            source: None,
            component: None,
        }))
    }
    
    assert!(test_function().is_ok());
    assert_eq!(test_function().unwrap(), "success");
    
    assert!(failing_function().is_err());
    let err = failing_function().unwrap_err();
    assert!(format!("{}", err).contains("Function failed"));
}

#[cfg(feature = "std")]
#[test]
fn test_creation_timestamp() {
    use std::time::{SystemTime, Duration};
    
    let before = SystemTime::now();
    let err = Yoshi::new(YoshiKind::Internal {
        message: "Timestamp test".into(),
        source: None,
        component: None,
    });
    let after = SystemTime::now();
    
    let created_at = err.created_at();
    assert!(created_at >= before);
    assert!(created_at <= after);
    
    // Should be very close to creation time
    let elapsed = after.duration_since(before).unwrap_or(Duration::from_secs(0));
    assert!(elapsed < Duration::from_millis(100));
}

#[test]
fn test_error_severity_and_transience_classification() {
    let test_cases = vec![
        (YoshiKind::Validation { 
            field: "test".into(), 
            message: "test".into(), 
            expected: None, 
            actual: None 
        }, 20, false),
        (YoshiKind::NotFound { 
            resource_type: "test".into(), 
            identifier: "test".into(), 
            search_locations: None 
        }, 25, false),
        (YoshiKind::Config { 
            message: "test".into(), 
            source: None, 
            config_path: None 
        }, 30, false),
        (YoshiKind::Timeout { 
            operation: "test".into(), 
            duration: Duration::from_secs(1), 
            expected_max: None 
        }, 45, true),
        (YoshiKind::Network { 
            message: "test".into(), 
            source: None, 
            error_code: None 
        }, 50, true),
        (YoshiKind::Multiple { 
            errors: vec![], 
            primary_index: None 
        }, 65, false),
        (YoshiKind::ResourceExhausted { 
            resource: "test".into(), 
            limit: "test".into(), 
            current: "test".into(), 
            usage_percentage: None 
        }, 70, true),
        (YoshiKind::Internal { 
            message: "test".into(), 
            source: None, 
            component: None 
        }, 80, false),
    ];
    
    for (kind, expected_severity, expected_transient) in test_cases {
        let err = Yoshi::new(kind);
        assert_eq!(err.severity(), expected_severity);
        assert_eq!(err.is_transient(), expected_transient);
    }
}

#[test] 
fn test_instance_id_uniqueness() {
    let err1 = Yoshi::new(YoshiKind::Internal { message: "test1".into(), source: None, component: None });
    let err2 = Yoshi::new(YoshiKind::Internal { message: "test2".into(), source: None, component: None });
    let err3 = Yoshi::new(YoshiKind::Internal { message: "test3".into(), source: None, component: None });
    
    // Instance IDs should be unique and increasing
    assert_ne!(err1.instance_id(), err2.instance_id());
    assert_ne!(err2.instance_id(), err3.instance_id());
    assert_ne!(err1.instance_id(), err3.instance_id());
    
    // They should generally be increasing (allowing for potential reordering)
    assert!(err3.instance_id() > err1.instance_id());
}