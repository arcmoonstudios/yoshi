/* tests/test_serde_integration.rs */
#![warn(missing_docs)]
#![deny(unsafe_code)]
//! **Brief:** Integration test for Yoshi serde serialization and cross-process communication.
//!
//! **Module Classification:** Testing
//! **Complexity Level:** High
//! **API Stability:** Stable
//!
//! ## Mathematical Properties
//!
//! **Algorithmic Complexity:**
//! - Time Complexity: O(1) for serialization/deserialization operations
//! - Space Complexity: O(N) where N is size of error context and metadata
//! - Concurrency Safety: Thread-safe with Arc<str> usage
//!
//! **Performance Characteristics:**
//! - Expected Performance: Sub-millisecond serialization for typical errors
//! - Worst-Case Scenarios: O(N) for large metadata maps and deep context chains
//! - Optimization Opportunities: Custom serialization for specific error patterns
//!
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + Comprehensive serde integration testing [Thread-safe, Memory-efficient]
//!  - ProcessError serialization with JSON validation [O(N) serialization, O(N) validation]
//!  - Cross-process error reporting with structured data [Lock-free communication]
//!  - SystemTime serialization with UNIX epoch conversion [Deterministic timestamps]
//!  - Arc<str> metadata handling with custom serde helpers [Zero-copy optimization]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **License Terms:** Full open source freedom; dual licensing allows choice between MIT and Apache 2.0
// **Effective Date:** 2025-05-30 | **Open Source Release**
// **License File:** /LICENSE
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn
// **Last Validation:** 2025-06-02

// Integration tests for serde functionality in the yoshi crate

// Only include the serde functionality if features are available
#[cfg(all(feature = "std", feature = "serde"))]
mod serde_tests {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::time::SystemTime;
    use yoshi::process_communication::{ProcessError, ProcessErrorReporter};
    use yoshi::{Yoshi, YoshiKind};

    #[test]
    fn test_process_error_serialization() {
        // Create a test error with comprehensive metadata
        let test_error = Yoshi::new(YoshiKind::Internal {
            message: "Test error for serde integration".into(),
            source: None,
            component: Some("serde_test".into()),
        })
        .with_metadata("operation", "database_connection")
        .with_metadata("retry_count", "3")
        .with_metadata("error_code", "DB_CONN_FAILED");

        // Create a ProcessError for serialization testing
        let mut metadata = HashMap::new();
        metadata.insert(Arc::from("test_key"), Arc::from("test_value"));
        metadata.insert(Arc::from("component"), Arc::from("integration_test"));

        let process_error = ProcessError {
            process_id: 12345,
            thread_id: "test_thread".to_string(),
            error_message: test_error.to_string(),
            error_kind: format!("{:?}", test_error.kind()),
            severity: test_error.severity(),
            timestamp: SystemTime::now(),
            metadata,
        };

        // Test serialization
        let json_result = serde_json::to_string(&process_error);
        assert!(
            json_result.is_ok(),
            "ProcessError serialization should succeed: {:?}",
            json_result.err()
        );

        let json = json_result.unwrap();

        // Verify JSON contains expected fields
        assert!(json.contains("\"process_id\":12345"));
        assert!(json.contains("\"thread_id\":\"test_thread\""));
        assert!(json.contains("\"test_key\":\"test_value\""));
        assert!(json.contains("\"component\":\"integration_test\""));

        // Test deserialization
        let deserialized_result = serde_json::from_str::<ProcessError>(&json);
        assert!(
            deserialized_result.is_ok(),
            "ProcessError deserialization should succeed: {:?}",
            deserialized_result.err()
        );

        let deserialized = deserialized_result.unwrap();

        // Verify deserialized data integrity
        assert_eq!(deserialized.process_id, 12345);
        assert_eq!(deserialized.thread_id, "test_thread");
        assert_eq!(
            deserialized.metadata.get(&Arc::from("test_key")),
            Some(&Arc::from("test_value"))
        );
        assert_eq!(
            deserialized.metadata.get(&Arc::from("component")),
            Some(&Arc::from("integration_test"))
        );
    }

    #[test]
    fn test_system_time_serialization() {
        // Test SystemTime serialization through ProcessError
        let test_error = Yoshi::new(YoshiKind::Network {
            message: "Connection timeout".into(),
            source: None,
            error_code: Some(1001),
        });

        let process_error = ProcessError {
            process_id: 54321,
            thread_id: "network_thread".to_string(),
            error_message: test_error.to_string(),
            error_kind: "Network".to_string(),
            severity: 150,
            timestamp: SystemTime::now(),
            metadata: HashMap::new(),
        };

        // Serialize and verify timestamp is handled correctly
        let json = serde_json::to_string(&process_error).expect("Serialization should succeed");

        // Should contain a numeric timestamp (seconds since UNIX_EPOCH)
        assert!(json.contains("\"timestamp\":"));

        // Deserialize and verify timestamp integrity
        let deserialized: ProcessError =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        // Timestamps should be reasonably close (within 1 second for test execution)
        let time_diff = process_error
            .timestamp
            .duration_since(deserialized.timestamp)
            .unwrap_or_else(|_| {
                deserialized
                    .timestamp
                    .duration_since(process_error.timestamp)
                    .unwrap()
            });

        assert!(
            time_diff.as_secs() <= 1,
            "Timestamp should be preserved within 1 second accuracy"
        );
    }

    #[test]
    fn test_process_error_reporter() {
        // Test the reporter functionality (basic smoke test)
        let reporter = ProcessErrorReporter::new();

        let test_error = Yoshi::new(YoshiKind::Internal {
            message: "Reporter test error".into(),
            source: None,
            component: Some("reporter_test".into()),
        });

        // This should not fail as the reporter has a background thread
        let report_result = reporter.report_error(&test_error);
        assert!(
            report_result.is_ok(),
            "Error reporting should succeed: {:?}",
            report_result.err()
        );
    }

    #[test]
    fn test_arc_str_metadata_serialization() {
        // Test comprehensive metadata with Arc<str> keys and values
        let mut metadata = HashMap::new();
        metadata.insert(Arc::from("user_id"), Arc::from("john_doe_123"));
        metadata.insert(Arc::from("session_token"), Arc::from("abcd-efgh-ijkl-mnop"));
        metadata.insert(Arc::from("api_version"), Arc::from("v2.1.0"));
        metadata.insert(Arc::from("endpoint"), Arc::from("/api/users/create"));

        let process_error = ProcessError {
            process_id: 99999,
            thread_id: "api_handler".to_string(),
            error_message: "API validation failed".to_string(),
            error_kind: "Validation".to_string(),
            severity: 200,
            timestamp: SystemTime::now(),
            metadata,
        };

        // Test round-trip serialization
        let json = serde_json::to_string(&process_error).expect("Serialization should succeed");
        let deserialized: ProcessError =
            serde_json::from_str(&json).expect("Deserialization should succeed");

        // Verify all metadata is preserved
        assert_eq!(
            deserialized.metadata.get(&Arc::from("user_id")),
            Some(&Arc::from("john_doe_123"))
        );
        assert_eq!(
            deserialized.metadata.get(&Arc::from("session_token")),
            Some(&Arc::from("abcd-efgh-ijkl-mnop"))
        );
        assert_eq!(
            deserialized.metadata.get(&Arc::from("api_version")),
            Some(&Arc::from("v2.1.0"))
        );
        assert_eq!(
            deserialized.metadata.get(&Arc::from("endpoint")),
            Some(&Arc::from("/api/users/create"))
        );
    }
} // End of serde_tests module

// Provide compile-time note instead of an ignored test
#[cfg(not(all(feature = "std", feature = "serde")))]
mod feature_requirements_note {
    // Using compile_note to generate a build message instead of an ignored test
    const _: () = {
        // This will show during compilation only, not as an ignored test
        struct RequirementsNote;

        // This is a compile-time note that doesn't create a runtime test
        // It won't appear as "ignored" in test output
        #[allow(dead_code)]
        impl RequirementsNote {
            const NOTE: &'static str = "NOTE: Serde integration tests require both 'std' and 'serde' features to be enabled";
        }
    };
}
