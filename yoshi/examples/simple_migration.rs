//! **Simple Migration Examples - Real-world scenarios**
//!
//! This file shows practical examples of migrating from thiserror/anyhow to Yoshi
//! with immediate benefits and gradual enhancement.

use yoshi::*;

/// **Example 1: File Operations (anyhow replacement)**
///
/// This shows how to replace anyhow with zero code changes
pub fn read_config_file(path: &str) -> Result<String> {
    // This is EXACTLY the same as anyhow code
    let content = fs::read_to_string(path).context("Failed to read configuration file")?;

    if content.is_empty() {
        return Err(error("Configuration file is empty"));
    }

    Ok(content)
}

/// **Example 2: Enhanced File Operations (with Yoshi features)**
///
/// Same function but with Yoshi's advanced features
pub fn read_config_file_enhanced(path: &str) -> Result<String> {
    let content = fs::read_to_string(path).context("Failed to read configuration file")?;

    // Add Yoshi's suggestions
    let validated = advanced::signpost(
        validate_config(&content),
        "Check the configuration file format and ensure all required fields are present",
    )?;

    // Add metadata for debugging
    advanced::metadata(Ok(validated), "config_path", path)
}

/// **Example 3: Network Operations**
///
/// Shows error handling for network operations with helpful suggestions
pub fn fetch_data(url: &str) -> Result<String> {
    // Simulate network request
    if url.is_empty() {
        return advanced::signpost(
            Err(error("Empty URL provided")),
            "Provide a valid HTTP/HTTPS URL",
        );
    }

    if !url.starts_with("http") {
        return advanced::signpost(
            Err(error("Invalid URL scheme")),
            "URL must start with 'http://' or 'https://'",
        );
    }

    // Simulate successful response
    Ok("response data".to_string())
}

/// **Example 4: Database Operations**
///
/// Shows how to handle database errors with context and suggestions
pub fn save_user(user_data: &UserData) -> Result<u64> {
    // Validate input
    let validated = advanced::nest(
        validate_user_data(user_data),
        "Validating user data before database save",
    )?;

    // Simulate database save
    let user_id = advanced::signpost(
        simulate_db_save(&validated),
        "Check database connection and ensure the users table exists",
    )?;

    // Add success metadata
    advanced::metadata(Ok(user_id), "operation", "user_save")
}

/// **Example 5: Parsing Operations**
///
/// Shows error handling for parsing with detailed suggestions
pub fn parse_json_config(json_str: &str) -> Result<Config> {
    if json_str.trim().is_empty() {
        return advanced::signpost(
            Err(error("Empty JSON string")),
            "Provide a valid JSON configuration string",
        );
    }

    // Simulate JSON parsing
    if !json_str.trim_start().starts_with('{') {
        return advanced::signpost(
            Err(error("Invalid JSON format")),
            "JSON must start with '{' for object format",
        );
    }

    // Add parsing context
    let config = advanced::nest(simulate_json_parse(json_str), "Parsing JSON configuration")?;

    // Validate parsed config
    advanced::signpost(
        validate_config_structure(&config),
        "Ensure all required configuration fields are present: 'host', 'port', 'database'",
    )?;

    Ok(config)
}

/// **Example 6: Gradual Migration Pattern**
///
/// Shows how to gradually migrate from existing error types
pub fn gradual_migration_example(input: &str) -> Result<ProcessedData> {
    // Step 1: Use existing validation (returns old error type)
    let validated =
        legacy_validate(input).map_err(|e| error(format!("Legacy validation failed: {e}")))?;

    // Step 2: Add Yoshi enhancements
    let enhanced = advanced::nest(Ok(validated), "Processing with enhanced error handling")?;

    // Step 3: Add suggestions for common issues
    if enhanced.is_empty() {
        return advanced::signpost(
            Err(error("Processed data is empty")),
            "Ensure input data contains valid content for processing",
        );
    }

    Ok(ProcessedData { data: enhanced })
}

/// **Example 7: Error Aggregation**
///
/// Shows how to collect and report multiple errors
pub fn process_multiple_files(paths: &[&str]) -> Result<Vec<String>> {
    let mut results = Vec::new();
    let mut errors = Vec::new();

    for path in paths {
        match read_config_file_enhanced(path) {
            Ok(content) => results.push(content),
            Err(e) => errors.push(format!("File {path}: {e}")),
        }
    }

    if !errors.is_empty() {
        return advanced::signpost(
            Err(error(format!(
                "Failed to process {} files: {}",
                errors.len(),
                errors.join("; ")
            ))),
            "Check file permissions and ensure all files exist and are readable",
        );
    }

    Ok(results)
}

// Helper types and functions for examples
#[derive(Debug)]
/// User data structure for migration example
pub struct UserData {
    /// User's full name
    pub name: String,
    /// User's email address
    pub email: String,
}

#[derive(Debug)]
/// Database configuration structure
pub struct Config {
    /// Database host address
    pub host: String,
    /// Database port number
    pub port: u16,
    /// Database name
    pub database: String,
}

#[derive(Debug)]
/// Processed data structure for migration example
pub struct ProcessedData {
    /// Processed data content
    pub data: String,
}

// Simulation functions
fn validate_config(content: &str) -> Result<String> {
    if content.len() < 10 {
        Err(error("Configuration too short"))
    } else {
        Ok(content.to_string())
    }
}

fn validate_user_data(user: &UserData) -> Result<UserData> {
    if user.name.is_empty() {
        return Err(error("User name cannot be empty"));
    }
    if !user.email.contains('@') {
        return Err(error("Invalid email format"));
    }
    Ok(UserData {
        name: user.name.clone(),
        email: user.email.clone(),
    })
}

const fn simulate_db_save(_user: &UserData) -> Result<u64> {
    Ok(12345) // Simulate successful save with user ID
}

fn simulate_json_parse(_json: &str) -> Result<Config> {
    Ok(Config {
        host: "localhost".to_string(),
        port: 5432,
        database: "myapp".to_string(),
    })
}

const fn validate_config_structure(_config: &Config) -> Result<()> {
    Ok(()) // Simulate successful validation
}

fn legacy_validate(input: &str) -> std::result::Result<String, String> {
    if input.is_empty() {
        Err("Input is empty".to_string())
    } else {
        Ok(input.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_file_reading() {
        // This would fail in real scenario, but shows the API
        let result = read_config_file("nonexistent.txt");
        assert!(result.is_err());

        // Error message includes context
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Failed to read configuration file"));
    }

    #[test]
    fn test_enhanced_features() {
        let user = UserData {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        };

        let result = save_user(&user);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 12345);
    }

    #[test]
    fn test_validation_with_suggestions() {
        let result = fetch_data("");
        assert!(result.is_err());

        // Check that the error contains helpful suggestions
        let error = result.unwrap_err();
        let yoshi_error = error.yoshi();
        assert!(yoshi_error.signpost().is_some());
    }
}

fn main() {
    tracing::info!("🚀 Yoshi Simple API Migration Examples");
    tracing::info!("Run with: cargo test --example simple_migration");
}
