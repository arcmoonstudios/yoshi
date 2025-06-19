/* yoshi-benches/examples/derive_macro_showcase.rs */
#![deny(dead_code)]
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(unused_variables)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

//! **Brief:** Comprehensive showcase of `YoshiError` derive macro capabilities with 2025 enhancements.
//!
//! This example demonstrates the full power of the Yoshi ecosystem with derive macros,
//! auto-inference, shorthand syntax, and comprehensive error handling patterns.

//! **Brief:** Comprehensive showcase of `YoshiError` derive macro capabilities with 2025 enhancements.
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + Advanced `YoshiError` derive macro demonstration with comprehensive attribute support
//!  - Error code base configuration with hierarchical numbering system
//!  - Default severity levels with per-variant override capabilities
//!  - Rich display formatting with field interpolation and context preservation
//!  - Source error chaining with automatic From trait implementation
//! + Comprehensive error variant showcase with real-world patterns
//!  - `NetworkFailure` with endpoint context and retry suggestions
//!  - `DatabaseError` with metrics shell and connection information
//!  - `BusinessRuleViolation` with recovery actions and validation context
//!  - `IoError` with source chaining and permission-based error handling
//! + Framework comparison engine with multi-dimensional scoring system
//!  - Context richness evaluation with metadata and suggestion tracking
//!  - Ergonomics assessment with API usability and developer experience metrics
//!  - Derive capabilities analysis with macro feature completeness scoring
//!  - Comprehensive ecosystem comparison with category winners and overall rankings
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

use yoshi::*;
use yoshi_benches::EcosystemComparisonEngine;

/// Showcase all `YoshiError` derive macro features with 2025 enhancements
#[derive(Debug, YoshiError)]
#[yoshi(error_code_base = 1000)]
#[yoshi(default_severity = 75)]
pub enum ShowcaseError {
    /// Network error with comprehensive attributes
    #[yoshi(display = "Network operation failed: {message}")]
    #[yoshi(kind = "Network")]
    #[yoshi(code = 1001)]
    #[yoshi(severity = 90)]
    #[yoshi(suggestion = "Check network connectivity and retry")]
    NetworkFailure {
        /// Human-readable error message describing the network failure
        message: String,
        /// Target URL endpoint that failed to connect
        #[yoshi(context = "endpoint")]
        url: String,
    },

    /// Timeout with auto-inference
    #[yoshi(display = "Operation timed out: {operation}")]
    #[yoshi(kind = "Timeout")]
    #[yoshi(code = 1005)]
    #[yoshi(transient = true)]
    OperationTimeout {
        /// Name of the operation that timed out
        operation: String,
        /// Duration of the timeout period
        duration: Duration,
    },

    /// Database error with comprehensive attributes
    #[yoshi(display = "Database operation failed: {operation}")]
    #[yoshi(kind = "Internal")]
    #[yoshi(code = 2001)]
    #[yoshi(severity = 85)]
    #[yoshi(suggestion = "Check database connectivity and retry")]
    DatabaseError {
        /// Database operation that failed (e.g., SELECT, INSERT, UPDATE)
        operation: String,
        /// Underlying I/O error that caused the database failure
        #[yoshi(source)]
        cause: std::io::Error,
        /// Database connection string for debugging purposes
        #[yoshi(context = "connection_info")]
        connection_string: String,
        /// Performance metrics and connection statistics
        #[yoshi(shell)]
        metrics: DatabaseMetrics,
    },

    /// Business logic error with auto-inference
    #[yoshi(display = "Business rule violated: {rule}")]
    #[yoshi(kind = "Validation")]
    #[yoshi(code = 3001)]
    #[yoshi(severity = 75)]
    #[yoshi(suggestion = "Review business rules and retry")]
    BusinessRuleViolation {
        /// Business rule that was violated
        rule: String,
        /// Detailed explanation of the violation
        #[yoshi(context = "violation_context")]
        details: String,
        /// Optional recovery action suggestion
        recovery_action: Option<String>,
    },

    /// Simple I/O error with source chaining
    #[yoshi(kind = "Io")]
    #[yoshi(display = "I/O operation failed: {message}")]
    #[yoshi(code = 5002)]
    #[yoshi(severity = 60)]
    IoError {
        /// Human-readable I/O error message
        message: String,
        /// Underlying I/O error that caused the failure
        #[yoshi(source)]
        cause: std::io::Error,
    },

    /// Validation error with comprehensive context
    #[yoshi(display = "Validation failed for field '{field}': {message}")]
    #[yoshi(kind = "Validation")]
    #[yoshi(code = 4001)]
    #[yoshi(severity = 50)]
    ValidationFailed {
        /// Field name that failed validation
        field: String,
        /// Validation error message
        message: String,
        /// User input that caused the validation failure
        #[yoshi(context = "validation_context")]
        user_input: String,
        /// Validation rules that were applied
        #[yoshi(shell)]
        validation_rules: ValidationRules,
    },
}

/// Database performance metrics and connection statistics
#[derive(Debug, Clone)]
pub struct DatabaseMetrics {
    /// Query execution time in milliseconds
    pub query_time_ms: u64,
    /// Connection pool usage as a percentage (0.0 to 1.0)
    pub connection_pool_usage: f64,
    /// Number of database rows affected by the operation
    pub rows_affected: u64,
}

/// Validation rules configuration for field validation
#[derive(Debug, Clone)]
pub struct ValidationRules {
    /// List of required field names
    pub required_fields: Vec<String>,
    /// Regular expression patterns for format validation
    pub format_patterns: Vec<String>,
    /// Business constraint rules for validation
    pub business_constraints: Vec<String>,
}

fn main() {
    println!("🦀 Yoshi Error Handling Showcase 🦀");
    println!("══════════════════════════════════════");

    // Test 1: Shorthand syntax with auto-inference
    println!("\n📊 Test 1: Shorthand Syntax & Auto-Inference");
    test_shorthand_syntax();

    // Test 2: Comprehensive error with rich context
    println!("\n📊 Test 2: Comprehensive Error Context");
    test_comprehensive_error();

    // Test 3: Auto From conversion
    println!("\n📊 Test 3: Automatic From Conversion");
    test_auto_conversion();

    // Test 4: Business logic with suggestions
    println!("\n📊 Test 4: Business Logic with Recovery");
    test_business_logic();

    // Test 5: Run comprehensive comparison
    println!("\n📊 Test 5: Framework Comparison Analysis");
    run_framework_comparison();

    println!("\n🏆 Yoshi Ecosystem Demonstration Complete!");
    println!("✨ Unmatched error handling capabilities demonstrated!");
}

fn test_shorthand_syntax() {
    // Network error with comprehensive attributes
    let network_error = ShowcaseError::NetworkFailure {
        message: "Connection timeout to api.example.com".to_string(),
        url: "https://api.example.com/auth".to_string(),
    };

    let yoshi_error = Yoshi::from(network_error)
        .lay("API request failed during user authentication")
        .with_metadata("endpoint", "https://api.example.com/auth")
        .with_metadata("retry_count", "3")
        .with_signpost("Check network connectivity and retry with exponential backoff");

    println!("   Network Error: {yoshi_error}");
    println!("   Kind: {:?}", yoshi_error.kind());
    println!("   Suggestion: {:?}", yoshi_error.suggestion());
    assert!(yoshi_error.suggestion().is_some());
}

fn test_comprehensive_error() {
    let db_error = ShowcaseError::DatabaseError {
        operation: "SELECT user_profile".to_string(),
        cause: std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "Connection refused"),
        connection_string: "postgresql://localhost:5432/app".to_string(),
        metrics: DatabaseMetrics {
            query_time_ms: 5000,
            connection_pool_usage: 0.95,
            rows_affected: 0,
        },
    };

    let yoshi_error = Yoshi::from(db_error)
        .lay("User profile lookup failed")
        .with_metadata("user_id", "12345")
        .with_metadata("request_id", "req_abc123")
        .with_metadata("component", "user_service")
        .with_priority(200);

    println!("   Database Error: {yoshi_error}");
    println!(
        "   Has Shell: {:?}",
        yoshi_error.shell::<DatabaseMetrics>().is_some()
    );

    if let Some(metrics) = yoshi_error.shell::<DatabaseMetrics>() {
        println!("   Query Time: {}ms", metrics.query_time_ms);
        println!(
            "   Pool Usage: {:.1}%",
            metrics.connection_pool_usage * 100.0
        );
    }
}

fn test_auto_conversion() {
    // Create an I/O error and wrap it in our error structure
    let io_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Access denied");
    let showcase_error = ShowcaseError::IoError {
        message: "File access failed".to_string(),
        cause: io_error,
    };

    let yoshi_error = Yoshi::from(showcase_error)
        .lay("File operation failed")
        .with_metadata("file_path", "/etc/sensitive.conf")
        .with_signpost("Check file permissions and user privileges");

    println!("   IO Error: {yoshi_error}");
    println!("   Source Available: {:?}", yoshi_error.source().is_some());
}

fn test_business_logic() {
    let business_error = ShowcaseError::BusinessRuleViolation {
        rule: "Maximum transaction limit exceeded".to_string(),
        details: "Transaction amount $5000 exceeds daily limit of $2500".to_string(),
        recovery_action: Some("Split transaction or request limit increase".to_string()),
    };

    let yoshi_error = Yoshi::from(business_error)
        .lay("Payment processing failed")
        .with_metadata("transaction_amount", "5000.00")
        .with_metadata("daily_limit", "2500.00")
        .with_metadata("user_tier", "standard")
        .with_signpost("Contact support for limit increase or split the transaction");

    println!("   Business Error: {yoshi_error}");
    println!(
        "   Recovery Available: {:?}",
        yoshi_error.suggestion().is_some()
    );
}

fn run_framework_comparison() {
    let engine = EcosystemComparisonEngine::new();
    let report = engine.execute_comprehensive_ecosystem_comparison();

    println!("   🏆 Framework Comparison Results:");
    println!("   ─────────────────────────────────");

    // Calculate all framework scores
    let mut framework_scores: std::collections::HashMap<String, f64> =
        std::collections::HashMap::new();

    // Track category scores for determining winners
    let mut context_scores: std::collections::HashMap<String, f64> =
        std::collections::HashMap::new();
    let mut ergonomics_scores: std::collections::HashMap<String, f64> =
        std::collections::HashMap::new();
    let mut derive_scores: std::collections::HashMap<String, f64> =
        std::collections::HashMap::new();

    for framework in ["Yoshi", "thiserror", "anyhow", "eyre", "snafu"] {
        if let Some(results) = report.results.get(framework) {
            let avg_context = results
                .iter()
                .map(|r| f64::from(r.context_richness))
                .sum::<f64>()
                / results.len() as f64;
            let avg_ergonomics = results
                .iter()
                .map(|r| f64::from(r.ergonomics_score))
                .sum::<f64>()
                / results.len() as f64;
            let avg_derive = results
                .iter()
                .map(|r| f64::from(r.derive_capabilities))
                .sum::<f64>()
                / results.len() as f64;

            let overall = (avg_context + avg_ergonomics + avg_derive) / 3.0;
            framework_scores.insert(framework.to_string(), overall);

            context_scores.insert(framework.to_string(), avg_context);
            ergonomics_scores.insert(framework.to_string(), avg_ergonomics);
            derive_scores.insert(framework.to_string(), avg_derive);
        }
    }

    // Find category winners
    let context_winner = context_scores
        .iter()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap();
    let ergonomics_winner = ergonomics_scores
        .iter()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap();
    let derive_winner = derive_scores
        .iter()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap();

    println!("   📊 CATEGORY WINNERS:");
    println!(
        "      🏆 Context Richness: {} ({:.1}/100)",
        context_winner.0, context_winner.1
    );
    println!(
        "      🏆 Ergonomics: {} ({:.1}/100)",
        ergonomics_winner.0, ergonomics_winner.1
    );
    println!(
        "      🏆 Derive Capabilities: {} ({:.1}/100)",
        derive_winner.0, derive_winner.1
    );

    // Sort frameworks by overall score
    let mut sorted_frameworks: Vec<_> = framework_scores.iter().collect();
    sorted_frameworks.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    println!("\n   🥇 OVERALL RANKINGS:");
    let medals = ["🥇", "🥈", "🥉", "4️⃣", "5️⃣"];
    for (i, (framework, score)) in sorted_frameworks.iter().enumerate() {
        let medal = medals.get(i).unwrap_or(&"📊");
        println!("      {medal} {framework}: {score:.1}/100");
    }

    if let Some((winner, _)) = sorted_frameworks.first() {
        println!("\n   🎉 OVERALL WINNER: {winner} based on combined scores!");
    }
}
