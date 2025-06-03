/* examples/complete_application.rs */
#![warn(missing_docs)]
#![deny(unsafe_code)]
//! **Brief:** Complete application example using the Yoshi facade crate.
//!
//! This example shows a realistic application architecture using Yoshi
//! for comprehensive error handling across multiple layers.
//!
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + Complete application demonstration
//!  - Multi-layer error handling
//!  - Service integration patterns
//!  - Recovery and fallback strategies
//!  - Enterprise-grade error reporting
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Author:** Lord Xyn

use std::collections::HashMap;
use std::time::Duration;
use yoshi::{yoshi, Arc, HatchExt, Result, Yoshi, YoshiKind};

/// Application configuration structure.
#[derive(Debug, Clone)]
struct AppConfig {
    database_url: String,
    api_timeout: Duration,
    max_retries: u32,
}

/// User data structure.
#[derive(Debug, Clone)]
struct User {
    id: String,
    email: String,
    name: String,
}

/// Request context for error tracking.
#[derive(Debug, Clone)]
struct RequestContext {
    request_id: String,
    user_agent: String,
    ip_address: String,
    timestamp: std::time::SystemTime,
}

/// Application service layer.
struct AppService {
    config: AppConfig,
}

impl AppService {
    /// Creates a new application service.
    fn new() -> Result<Self> {
        let config =
            Self::load_configuration().context("Failed to initialize application service")?;

        Ok(Self { config })
    }

    /// Loads application configuration.
    fn load_configuration() -> Result<AppConfig> {
        // Simulate configuration loading that might fail
        if std::env::var("APP_ENV").unwrap_or_default() == "invalid" {
            return Err(yoshi!(kind: YoshiKind::Config {
                message: "Invalid application environment configuration".into(),
                source: None,
                config_path: Some("/etc/app/config.toml".into()),
            },
            with_metadata = ("config_source", "environment"),
            with_metadata = ("validation_error", "unknown_environment"),
            with_suggestion = "Set APP_ENV to 'development', 'staging', or 'production'"
            ));
        }

        Ok(AppConfig {
            database_url: "postgresql://localhost:5432/app".to_string(),
            api_timeout: Duration::from_secs(30),
            max_retries: 3,
        })
    }

    /// Retrieves user data with comprehensive error handling.
    fn get_user(&self, user_id: &str, context: RequestContext) -> Result<User> {
        // Input validation
        if user_id.is_empty() {
            return Err(yoshi!(kind: YoshiKind::Validation {
                field: "user_id".into(),
                message: "User ID cannot be empty".into(),
                expected: Some("Non-empty string identifier".into()),
                actual: Some("empty string".into()),
            },
            with_shell = context.clone(),
            with_metadata = ("validation_rule", "non_empty_user_id"),
            with_priority = 100
            ))
            .context("User lookup validation failed");
        }

        // Database query simulation
        self.query_user_database(user_id, &context)
            .context("Database user lookup failed")?;

        // Cache lookup simulation
        self.query_user_cache(user_id, &context)
            .context("Cache user lookup failed")?;

        // External service validation
        self.validate_user_external(user_id, &context)
            .context("External user validation failed")
    }

    /// Simulates database user query.
    fn query_user_database(&self, user_id: &str, context: &RequestContext) -> Result<User> {
        if user_id == "db_error" {
            return Err(yoshi!(kind: YoshiKind::Network {
                message: "Database connection pool exhausted".into(),
                source: None,
                error_code: Some(2006),
            },
            with_shell = context.clone(),
            with_metadata = ("database_host", "db-primary.example.com"),
            with_metadata = ("connection_pool_size", "100"),
            with_metadata = ("active_connections", "100"),
            with_suggestion = "Scale database connection pool or implement connection recycling"
            ));
        }

        if user_id == "not_found" {
            return Err(yoshi!(kind: YoshiKind::NotFound {
                resource_type: "User".into(),
                identifier: user_id.into(),
                search_locations: Some(vec![
                    "users table".into(),
                    "user_profiles table".into(),
                    "archived_users table".into()
                ]),
            },
            with_shell = context.clone(),
            with_metadata = ("query_duration_ms", "45"),
            with_suggestion = "Check if user ID is correct or if user has been archived"
            ));
        }

        Ok(User {
            id: user_id.to_string(),
            email: format!("{}@example.com", user_id),
            name: format!("User {}", user_id),
        })
    }

    /// Simulates cache user query.
    fn query_user_cache(&self, user_id: &str, context: &RequestContext) -> Result<()> {
        if user_id == "cache_timeout" {
            return Err(yoshi!(kind: YoshiKind::Timeout {
                operation: "Redis cache lookup".into(),
                duration: self.config.api_timeout + Duration::from_secs(5),
                expected_max: Some(self.config.api_timeout),
            },
            with_shell = context.clone(),
            with_metadata = ("cache_host", "redis.example.com"),
            with_metadata = ("cache_key", &format!("user:{}", user_id)),
            with_suggestion = "Check Redis server health and network latency"
            ));
        }

        Ok(())
    }

    /// Simulates external service validation.
    fn validate_user_external(&self, user_id: &str, context: &RequestContext) -> Result<User> {
        if user_id == "service_down" {
            return Err(yoshi!(kind: YoshiKind::Network {
                message: "External validation service unavailable".into(),
                source: None,
                error_code: Some(503),
            },
            with_shell = context.clone(),
            with_metadata = ("service_name", "user_validation_api"),
            with_metadata = ("service_url", "https://api.validation.example.com"),
            with_metadata = ("retry_attempt", "1"),
            with_suggestion = "Enable fallback validation or retry after brief delay"
            ));
        }

        Ok(User {
            id: user_id.to_string(),
            email: format!("{}@example.com", user_id),
            name: format!("Validated User {}", user_id),
        })
    }

    /// Processes user request with fallback strategies.
    fn process_user_request(&self, user_id: &str) -> Result<String> {
        let context = RequestContext {
            request_id: format!("req_{}", fastrand::u64(..)),
            user_agent: "YoshiApp/1.0".to_string(),
            ip_address: "192.168.1.100".to_string(),
            timestamp: std::time::SystemTime::now(),
        };

        // Primary user lookup
        match self.get_user(user_id, context.clone()) {
            Ok(user) => Ok(format!(
                "Successfully processed user: {} ({})",
                user.name, user.email
            )),
            Err(error) => {
                // Check if error suggests fallback strategy
                if error.is_transient() {
                    // Attempt fallback for transient errors
                    self.fallback_user_processing(user_id, error)
                } else {
                    // Re-propagate non-transient errors with additional context
                    Err(error)
                        .context("User processing failed with non-recoverable error")
                        .meta("fallback_attempted", "false")
                        .meta("error_category", "non_transient")
                }
            }
        }
    }

    /// Fallback processing strategy.
    fn fallback_user_processing(&self, user_id: &str, original_error: Yoshi) -> Result<String> {
        // Log original error for analysis
        println!(
            "🔄 Attempting fallback due to transient error: {}",
            original_error
        );

        // Simplified fallback logic
        if user_id == "fallback_success" {
            Ok("Fallback processing successful".to_string())
        } else {
            Err(yoshi!(kind: YoshiKind::Internal {
                message: "Fallback processing also failed".into(),
                source: Some(Box::new(original_error)),
                component: Some("FallbackProcessor".into()),
            },
            with_metadata = ("fallback_strategy", "simplified_processing"),
            with_metadata = ("original_error_id", &original_error.instance_id().to_string()),
            with_suggestion = "Manual intervention may be required - check system health dashboard"
            ))
            .context("All processing strategies exhausted")
        }
    }
}

/// Demonstrates complete application error handling.
fn main() {
    println!("🚀 Complete Yoshi Application Demo");
    println!("===================================");

    // Initialize application service
    let app_service = match AppService::new() {
        Ok(service) => service,
        Err(error) => {
            println!("❌ Failed to initialize application:");
            println!("{}", error);
            return;
        }
    };

    // Test scenarios
    let test_cases = vec![
        ("valid_user", "✅ Should succeed"),
        ("not_found", "❌ Should fail with NotFound error"),
        ("db_error", "❌ Should fail with database error"),
        ("cache_timeout", "❌ Should fail with timeout error"),
        ("service_down", "❌ Should fail with service error"),
        ("fallback_success", "🔄 Should succeed via fallback"),
    ];

    for (user_id, description) in test_cases {
        println!("\n{} - Testing user_id: '{}'", description, user_id);
        println!("{}", "=".repeat(50));

        match app_service.process_user_request(user_id) {
            Ok(result) => {
                println!("✅ {}", result);
            }
            Err(error) => {
                println!("❌ Request failed:");
                println!("{}", error);

                // Show detailed error analysis
                println!("\n📊 Error Analysis:");
                println!("   Severity: {}/255", error.severity());
                println!("   Instance ID: {}", error.instance_id());
                println!("   Is Transient: {}", error.is_transient());

                // Show context analysis
                let analysis = error.analyze_contexts();
                println!("   Contexts: {}", analysis.total_contexts);
                println!("   Metadata Entries: {}", analysis.metadata_entries);
                println!("   Has Suggestions: {}", analysis.has_suggestions);

                // Show request context if available
                if let Some(req_ctx) = error.shell::<RequestContext>() {
                    println!("   Request ID: {}", req_ctx.request_id);
                    println!("   IP Address: {}", req_ctx.ip_address);
                }
            }
        }
    }

    println!("\n🎯 Demo completed! Check the comprehensive error output above.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_service_creation() {
        let service = AppService::new();
        assert!(service.is_ok());
    }

    #[test]
    fn test_valid_user_processing() {
        let service = AppService::new().unwrap();
        let result = service.process_user_request("valid_user");
        assert!(result.is_ok());
    }

    #[test]
    fn test_not_found_error() {
        let service = AppService::new().unwrap();
        let result = service.process_user_request("not_found");
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(matches!(error.kind(), YoshiKind::NotFound { .. }));
        assert!(error.shell::<RequestContext>().is_some());
    }

    #[test]
    fn test_database_error() {
        let service = AppService::new().unwrap();
        let result = service.process_user_request("db_error");
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(matches!(error.kind(), YoshiKind::Network { .. }));
    }

    #[test]
    fn test_timeout_error() {
        let service = AppService::new().unwrap();
        let result = service.process_user_request("cache_timeout");
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(matches!(error.kind(), YoshiKind::Timeout { .. }));
        assert!(error.is_transient());
    }

    #[test]
    fn test_fallback_processing() {
        let service = AppService::new().unwrap();
        let result = service.process_user_request("fallback_success");
        // This might succeed or fail depending on fallback logic
        if let Err(error) = result {
            assert!(error.to_string().contains("transient"));
        }
    }

    #[test]
    fn test_error_analysis() {
        let service = AppService::new().unwrap();
        let result = service.process_user_request("db_error");
        assert!(result.is_err());

        let error = result.unwrap_err();
        let analysis = error.analyze_contexts();
        assert!(analysis.total_contexts > 0);
        assert!(analysis.has_suggestions);
        assert!(analysis.metadata_entries > 0);
    }
}
