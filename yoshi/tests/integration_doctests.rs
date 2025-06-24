/* yoshi/tests/integration_doctests.rs */
//! Integration tests that validate the doctests from err.rs example
//!
//! This file ensures that all the doctests in the err.rs example actually work
//! and can be executed as part of the test suite.

use yoshi::*;

/// Test the basic Hatch<T> usage patterns from err.rs
#[test]
fn test_basic_hatch_usage() -> Hatch<()> {
    // Test divide function
    fn divide(a: f64, b: f64) -> Hatch<f64> {
        if b == 0.0 {
            Err(yopost!(message: "Division by zero"))
        } else {
            Ok(a / b)
        }
    }

    // Test calculate function with error propagation
    fn calculate() -> Hatch<f64> {
        let result = divide(10.0, 2.0)?;
        Ok(result * 2.0)
    }

    // Test successful division
    let result = divide(10.0, 2.0)?;
    assert_eq!(result, 5.0);

    // Test successful calculation
    let calc_result = calculate()?;
    assert_eq!(calc_result, 10.0);

    // Test error case
    let error_result = divide(10.0, 0.0);
    assert!(error_result.is_err());

    let error_msg = format!("{}", error_result.unwrap_err());
    assert!(error_msg.contains("Division by zero"));

    Ok(())
}

/// Test file operations with error handling
#[test]
fn test_file_operations() -> Hatch<()> {
    use std::fs;

    fn read_config_sync() -> Hatch<String> {
        let content = fs::read_to_string("nonexistent_config.toml").map_err(|e| {
            yopost!(
                error: e,
                with_signpost = "Create config.toml or check file permissions"
            )
        })?;
        Ok(content)
    }

    // Test with a file that doesn't exist - should return error with suggestion
    let result = read_config_sync();
    assert!(result.is_err());

    // The error should contain our suggestion or file reference
    let error_msg = format!("{}", result.unwrap_err());
    assert!(
        error_msg.contains("config")
            || error_msg.contains("file")
            || error_msg.contains("nonexistent")
    );

    Ok(())
}

/// Test error creation patterns
#[test]
fn test_error_patterns() -> Hatch<()> {
    // Test basic error creation
    let error = yopost!(message: "Configuration error in app.toml: missing required field");
    let error_str = format!("{error}");
    assert!(error_str.contains("Configuration error"));
    assert!(error_str.contains("app.toml"));

    // Test error with suggestion
    let error_with_suggestion = yopost!(message: "Validation error: email invalid format");
    let error_str = format!("{error_with_suggestion}");
    assert!(error_str.contains("Validation error"));
    assert!(error_str.contains("email"));

    // Test business rule error
    let business_error = yopost!(message: "Business rule violation: max_orders - limit exceeded");
    let error_str = format!("{business_error}");
    assert!(error_str.contains("Business rule violation"));
    assert!(error_str.contains("max_orders"));

    // Test timeout error
    let timeout_error = yopost!(message: "Operation 'database_query' timed out after 5000ms");
    let error_str = format!("{timeout_error}");
    assert!(error_str.contains("timed out"));
    assert!(error_str.contains("5000ms"));

    Ok(())
}

/// Test optimization patterns
#[test]
fn test_optimization_patterns() -> Hatch<()> {
    // Test optimization-related error messages
    let optimization_error =
        yopost!(message: "Optimization opportunity: Vec::new() could use with_capacity()");
    let error_str = format!("{optimization_error}");
    assert!(error_str.contains("Optimization"));
    assert!(error_str.contains("Vec::new"));

    // Test auto-correction suggestion format
    let suggestion_error =
        yopost!(message: "Auto-correction: Replace .unwrap() with proper error handling");
    let error_str = format!("{suggestion_error}");
    assert!(error_str.contains("Auto-correction"));
    assert!(error_str.contains("unwrap"));

    Ok(())
}

/// Test error chaining and context
#[test]
fn test_error_chaining() -> Hatch<()> {
    fn load_raw_data() -> Hatch<String> {
        Ok("1,2,3,4,5".to_string())
    }

    fn parse_data(data: &str) -> Hatch<Vec<i32>> {
        data.split(',')
            .map(|s| s.parse().map_err(|e| yopost!(error: e)))
            .collect()
    }

    fn process_data() -> Hatch<Vec<i32>> {
        let raw_data = load_raw_data()?;
        let processed = parse_data(&raw_data)?;
        Ok(processed)
    }

    // Test successful processing
    let result = process_data()?;
    assert_eq!(result, vec![1, 2, 3, 4, 5]);

    // Test error case with invalid data
    let invalid_result = parse_data("1,2,invalid,4,5");
    assert!(invalid_result.is_err());

    Ok(())
}

/// Test that all the example patterns compile and work
#[test]
fn test_comprehensive_patterns() -> Hatch<()> {
    // Test various error creation patterns
    let _config_err = yopost!(message: "Configuration error");
    let _validation_err = yopost!(message: "Validation failed - check input format");

    // Test error conversion
    let std_err = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    let _yoshi_err = yopost!(error: std_err, with_signpost = "Create the missing file");

    // Test Result<T> alias
    fn example_function() -> Result<String> {
        Ok("success".to_string())
    }

    let result = example_function()?;
    assert_eq!(result, "success");

    Ok(())
}
