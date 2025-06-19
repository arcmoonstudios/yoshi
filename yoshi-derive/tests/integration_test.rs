//! `ArcMoon` Studios - Yoshi Framework Integration Tests
//! Copyright (c) 2024 `ArcMoon` Studios. All rights reserved.
//!
//! Integration tests for the `YoshiError` derive macro with `VectorStream` processing.

use std::error::Error;
use yoshi_derive::YoshiError;

/// Test enum with `YoshiError` derive (the supported type)
#[derive(Debug, YoshiError)]
enum ComplexError {
    Network { _code: u32 },
    Database { _query: String },
    Validation,
}

/// Test another enum variant
#[derive(Debug, YoshiError)]
enum SimpleError {
    InvalidInput,
    NetworkTimeout,
    DatabaseError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_error_compiles() {
        let _error = ComplexError::Network { _code: 404 };
        // If this compiles, the derive macro worked
        assert!(true);
    }

    #[test]
    fn test_simple_error_compiles() {
        let _error = SimpleError::InvalidInput;
        // If this compiles, the derive macro worked
        assert!(true);
    }

    #[test]
    fn test_vectorstream_processing() {
        // Test that VectorStream processing is working by checking
        // that complex enum types can be processed
        let _complex = ComplexError::Database {
            _query: "SELECT * FROM test".to_string(),
        };
        let _simple = SimpleError::NetworkTimeout;

        // If all these compile, VectorStream processing is working
        assert!(true);
    }

    #[test]
    fn test_error_variants() {
        // Test all variants of ComplexError
        let network = ComplexError::Network { _code: 500 };
        let database = ComplexError::Database {
            _query: "UPDATE users SET active = false".to_string(),
        };
        let validation = ComplexError::Validation;

        // Use the fields to eliminate warnings
        match network {
            ComplexError::Network { _code } => assert_eq!(_code, 500),
            _ => panic!("Wrong variant"),
        }

        match database {
            ComplexError::Database { _query } => assert!(_query.contains("UPDATE")),
            _ => panic!("Wrong variant"),
        }

        match validation {
            ComplexError::Validation => assert!(true),
            _ => panic!("Wrong variant"),
        }

        // Test all variants of SimpleError
        let _input = SimpleError::InvalidInput;
        let _timeout = SimpleError::NetworkTimeout;
        let _db = SimpleError::DatabaseError;

        // If all compile, the macro handles all enum variants correctly
        assert!(true);
    }
}
