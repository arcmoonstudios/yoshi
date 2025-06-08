/* yoshi-deluxe/src/constants.rs */
//! **Brief:** Performance constants and optimized regex patterns for yoshi-deluxe.
//!
//! This module contains all performance-critical constants, thresholds, and pre-compiled
//! regex patterns used throughout the auto-correction system. All values are optimized
//! for production workloads with comprehensive benchmarking validation.

use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{
    header::{HeaderMap, HeaderValue, USER_AGENT},
    Client,
};
use std::sync::Arc;
use std::{collections::HashMap, time::Duration};
use tokio::sync::RwLock;

//--------------------------------------------------------------------------------------------------
// Core Performance Constants
//--------------------------------------------------------------------------------------------------

/// Maximum concurrent HTTP requests for docs.rs scraping
pub const MAX_CONCURRENT_REQUESTS: usize = 8;

/// Cache expiration time for documentation data (1 hour)
pub const DOCS_CACHE_EXPIRY: Duration = Duration::from_secs(3600);

/// Request timeout for docs.rs API calls
pub const HTTP_TIMEOUT: Duration = Duration::from_secs(20);

/// Maximum file size for AST processing (5MB)
pub const MAX_FILE_SIZE: usize = 5 * 1024 * 1024;

/// Regex compilation cache size
pub const REGEX_CACHE_SIZE: usize = 32;

/// Byte offset tolerance for AST mapping
pub const BYTE_OFFSET_TOLERANCE: usize = 5;

/// Maximum cache entries before LRU eviction
pub const MAX_CACHE_ENTRIES: usize = 1000;

/// Default similarity threshold for method suggestions
pub const DEFAULT_SIMILARITY_THRESHOLD: f64 = 0.6;

/// Maximum diagnostic processing batch size
pub const MAX_DIAGNOSTIC_BATCH_SIZE: usize = 100;

/// AST node analysis timeout (seconds)
pub const AST_ANALYSIS_TIMEOUT: Duration = Duration::from_secs(30);

/// Documentation scraping retry count
pub const DOCS_SCRAPING_RETRY_COUNT: usize = 3;

/// Code generation maximum iterations
pub const CODEGEN_MAX_ITERATIONS: usize = 5;

/// System health check interval
pub const HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(60);

//--------------------------------------------------------------------------------------------------
// Advanced Regex Compilation Cache with Performance Optimization
//--------------------------------------------------------------------------------------------------

lazy_static! {
    /// Global high-performance regex cache with O(1) lookup
    pub static ref REGEX_PATTERNS: HashMap<&'static str, Regex> = {
        let mut patterns = HashMap::with_capacity(REGEX_CACHE_SIZE);

        // Compiler error pattern matching with optimized expressions
        patterns.insert("method_not_found",
            Regex::new(r"no method named `(\w+)` found for (?:struct|type|enum) `([^`]+)`").unwrap());
        patterns.insert("type_mismatch",
            Regex::new(r"(?s)expected `([^`]+)`, found `([^`]+)`").unwrap());
        patterns.insert("missing_trait",
            Regex::new(r"the trait `([^`]+)` is not implemented for `([^`]+)`").unwrap());
        patterns.insert("unused_import",
            Regex::new(r"unused import: `([^`]+)`").unwrap());
        patterns.insert("missing_lifetime",
            Regex::new(r"missing lifetime specifier").unwrap());
        patterns.insert("missing_field",
            Regex::new(r"missing field `(\w+)` in initializer of `([^`]+)`").unwrap());
        patterns.insert("unknown_field",
            Regex::new(r"no field `(\w+)` on type `([^`]+)`").unwrap());
        patterns.insert("borrowing_error",
            Regex::new(r"(?:cannot borrow|borrow checker)").unwrap());
        patterns.insert("lifetime_error",
            Regex::new(r"(?:lifetime|borrowed value)").unwrap());
        patterns.insert("variable_not_found",
            Regex::new(r"cannot find (?:value|variable) `(\w+)` in this scope").unwrap());
        patterns.insert("function_not_found",
            Regex::new(r"cannot find function `(\w+)` in this scope").unwrap());
        patterns.insert("type_not_found",
            Regex::new(r"cannot find type `(\w+)` in this scope").unwrap());
        patterns.insert("module_not_found",
            Regex::new(r"unresolved import `([^`]+)`").unwrap());
        patterns.insert("macro_not_found",
            Regex::new(r"cannot find macro `(\w+)` in this scope").unwrap());

        // API structure patterns for robust parsing
        patterns.insert("api_method_structured",
            Regex::new(r#""name":\s*"(\w+)",\s*"signature":\s*"([^"]+)""#).unwrap());
        patterns.insert("api_trait_impl",
            Regex::new(r#""trait":\s*"(\w+)",\s*"for":\s*"(\w+)""#).unwrap());

        // Documentation parsing patterns
        patterns.insert("method_signature",
            Regex::new(r"fn\s+(\w+)\s*\((.*?)\)(?:\s*->\s*([^{;]+))?").unwrap());
        patterns.insert("struct_definition",
            Regex::new(r"struct\s+(\w+)(?:<([^>]+)>)?\s*\{").unwrap());
        patterns.insert("enum_definition",
            Regex::new(r"enum\s+(\w+)(?:<([^>]+)>)?\s*\{").unwrap());
        patterns.insert("trait_definition",
            Regex::new(r"trait\s+(\w+)(?:<([^>]+)>)?").unwrap());

        // Code quality patterns
        patterns.insert("todo_comment",
            Regex::new(r"(?i)(?://\s*)?todo[!:]?\s*(.*)").unwrap());
        patterns.insert("fixme_comment",
            Regex::new(r"(?i)(?://\s*)?fixme[!:]?\s*(.*)").unwrap());
        patterns.insert("panic_macro",
            Regex::new(r"panic!\s*\(\s*([^)]*)\s*\)").unwrap());
        patterns.insert("unwrap_call",
            Regex::new(r"\.unwrap\(\)").unwrap());
        patterns.insert("expect_call",
            Regex::new(r"\.expect\s*\(\s*([^)]*)\s*\)").unwrap());

        patterns
    };

    /// Production-optimized HTTP client with connection pooling
    pub static ref HTTP_CLIENT: Client = {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(
            "yoshi-deluxe/1.0.0 (https://github.com/arcmoonstudios/yoshi-deluxe)"));

        Client::builder()
            .timeout(HTTP_TIMEOUT)
            .default_headers(headers)
            .pool_max_idle_per_host(3)
            .pool_idle_timeout(Duration::from_secs(60))
            .danger_accept_invalid_certs(false)
            .build()
            .expect("Failed to create HTTP client")
    };
}

//--------------------------------------------------------------------------------------------------
// Cache Instances and Global State
//--------------------------------------------------------------------------------------------------

use crate::types::CachedDocsData;

lazy_static! {
    /// Intelligent caching system for documentation data
    pub static ref DOCS_CACHE: Arc<RwLock<HashMap<String, CachedDocsData>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

//--------------------------------------------------------------------------------------------------
// Error Code Mappings and Classifications
//--------------------------------------------------------------------------------------------------

/// Rust compiler error code to correction strategy mapping
pub const ERROR_CODE_STRATEGIES: &[(&str, &str)] = &[
    ("E0599", "method_not_found"),
    ("E0308", "type_mismatch"),
    ("E0425", "unresolved_name"),
    ("E0432", "unresolved_import"),
    ("E0433", "failed_to_resolve"),
    ("E0560", "struct_field_missing"),
    ("E0559", "struct_field_unknown"),
    ("E0277", "trait_not_implemented"),
    ("E0596", "cannot_borrow_mutably"),
    ("E0597", "borrowed_value_does_not_live_long_enough"),
    ("E0515", "cannot_return_value_referencing_local"),
    ("E0502", "cannot_borrow_as_mutable"),
    ("E0501", "cannot_borrow_as_immutable"),
    ("E0382", "use_of_moved_value"),
    ("E0384", "cannot_assign_to_immutable"),
    ("E0716", "temporary_value_dropped"),
];

/// Error severity classifications
pub const ERROR_SEVERITY_MAP: &[(&str, u8)] = &[
    // Critical errors (200+)
    ("E0308", 255), // Type mismatch
    ("E0382", 240), // Use of moved value
    ("E0425", 220), // Cannot find value
    ("E0432", 210), // Unresolved import
    ("E0596", 200), // Cannot borrow mutably
    // High severity errors (150-199)
    ("E0599", 180), // No method named
    ("E0277", 170), // Trait not implemented
    ("E0560", 160), // Missing struct field
    ("E0559", 150), // Unknown struct field
    // Medium severity errors (100-149)
    ("E0597", 140), // Borrowed value lifetime
    ("E0515", 130), // Cannot return reference
    ("E0502", 120), // Cannot borrow as mutable
    ("E0501", 110), // Cannot borrow as immutable
    ("E0384", 100), // Cannot assign to immutable
    // Low severity errors (<100)
    ("E0716", 80), // Temporary value dropped
];

//--------------------------------------------------------------------------------------------------
// Correction Confidence Thresholds
//--------------------------------------------------------------------------------------------------

/// Confidence thresholds for different types of corrections
pub const CONFIDENCE_THRESHOLDS: &[(&str, f64)] = &[
    ("method_rename", 0.85),
    ("type_conversion", 0.90),
    ("import_addition", 0.95),
    ("trait_import", 0.88),
    ("field_correction", 0.80),
    ("borrowing_fix", 0.75),
    ("lifetime_annotation", 0.70),
    ("generic_suggestion", 0.60),
];

/// Safety level thresholds for auto-application
pub const SAFETY_THRESHOLDS: &[(&str, f64)] = &[
    ("safe_auto_apply", 0.95),
    ("review_recommended", 0.80),
    ("manual_review_required", 0.60),
];

//--------------------------------------------------------------------------------------------------
// Documentation Source Configurations
//--------------------------------------------------------------------------------------------------

/// Documentation source URLs with fallback priorities
pub const DOCS_SOURCES: &[(&str, &str, u8)] = &[
    // (source_name, base_url, priority)
    ("docs_rs", "https://docs.rs", 100),
    ("github_docs", "https://docs.github.io", 80),
    ("rustdoc_local", "file://./target/doc", 60),
    ("crates_io", "https://crates.io", 40),
];

/// HTML selectors for documentation parsing
pub const DOCS_SELECTORS: &[(&str, &str)] = &[
    (
        "method",
        ".method, .impl-items .method, [data-method], .item-decl",
    ),
    ("method_name", ".method-name, .item-name, code"),
    ("signature", ".signature, pre"),
    ("docblock", ".docblock"),
    (
        "impl_items",
        ".impl-items, .trait-implementations, [data-impl]",
    ),
    (
        "examples",
        ".example-wrap pre, .docblock pre, pre.playground, code.rust",
    ),
    ("struct_fields", ".fields, .struct-fields"),
    ("enum_variants", ".variants, .enum-variants"),
    ("trait_methods", ".trait-methods, .required-methods"),
];

//--------------------------------------------------------------------------------------------------
// Performance Tuning Parameters
//--------------------------------------------------------------------------------------------------

/// Cache warming parameters
pub const CACHE_WARMING: &[(&str, usize)] = &[
    ("common_types", 50),
    ("std_methods", 100),
    ("frequent_errors", 25),
];

/// Parallel processing limits
pub const PARALLEL_LIMITS: &[(&str, usize)] = &[
    ("max_ast_workers", 4),
    ("max_docs_workers", 8),
    ("max_codegen_workers", 6),
    ("max_diagnostic_workers", 12),
];

/// Memory management thresholds
pub const MEMORY_THRESHOLDS: &[(&str, usize)] = &[
    ("cache_cleanup_trigger", 1024 * 1024 * 500), // 500MB
    ("max_string_intern_size", 1024),             // 1KB
    ("gc_trigger_interval", 300),                 // 5 minutes
];

//--------------------------------------------------------------------------------------------------
// Feature Flag Defaults
//--------------------------------------------------------------------------------------------------

/// Default feature configurations
pub const DEFAULT_FEATURES: &[(&str, bool)] = &[
    ("enable_docs_scraping", true),
    ("enable_parallel_processing", true),
    ("enable_caching", true),
    ("enable_metrics", true),
    ("enable_auto_fixes", true),
    ("enable_backup_creation", true),
    ("enable_health_monitoring", true),
    ("enable_performance_profiling", false),
    ("enable_debug_logging", false),
    ("enable_network_retries", true),
];

//--------------------------------------------------------------------------------------------------
// Utility Functions for Constants Access
//--------------------------------------------------------------------------------------------------

/// Get error severity for a specific error code
#[must_use]
pub fn get_error_severity(error_code: &str) -> u8 {
    ERROR_SEVERITY_MAP
        .iter()
        .find(|(code, _)| *code == error_code)
        .map(|(_, severity)| *severity)
        .unwrap_or(128) // Default severity
}

/// Get correction strategy for error code
#[must_use]
pub fn get_correction_strategy(error_code: &str) -> Option<&'static str> {
    ERROR_CODE_STRATEGIES
        .iter()
        .find(|(code, _)| *code == error_code)
        .map(|(_, strategy)| *strategy)
}

/// Get confidence threshold for correction type
#[must_use]
pub fn get_confidence_threshold(correction_type: &str) -> f64 {
    CONFIDENCE_THRESHOLDS
        .iter()
        .find(|(ctype, _)| *ctype == correction_type)
        .map(|(_, threshold)| *threshold)
        .unwrap_or(DEFAULT_SIMILARITY_THRESHOLD)
}

/// Check if feature is enabled by default
#[must_use]
pub fn is_feature_enabled_by_default(feature_name: &str) -> bool {
    DEFAULT_FEATURES
        .iter()
        .find(|(name, _)| *name == feature_name)
        .map(|(_, enabled)| *enabled)
        .unwrap_or(false)
}

/// Get parallel processing limit for component
#[must_use]
pub fn get_parallel_limit(component: &str) -> usize {
    PARALLEL_LIMITS
        .iter()
        .find(|(comp, _)| *comp == component)
        .map(|(_, limit)| *limit)
        .unwrap_or(1)
}

/// Get memory threshold for operation
#[must_use]
pub fn get_memory_threshold(operation: &str) -> usize {
    MEMORY_THRESHOLDS
        .iter()
        .find(|(op, _)| *op == operation)
        .map(|(_, threshold)| *threshold)
        .unwrap_or(1024 * 1024) // 1MB default
}

//--------------------------------------------------------------------------------------------------
// Validation and Health Checks
//--------------------------------------------------------------------------------------------------

/// Validate that all regex patterns compile correctly
pub fn validate_regex_patterns() -> crate::Result<()> {
    use crate::errors::AutoCorrectionError;

    for (name, regex) in REGEX_PATTERNS.iter() {
        if regex.as_str().is_empty() {
            return Err(AutoCorrectionError::Configuration {
                parameter: format!("regex_pattern_{name}"),
                value: "empty".to_string(),
                expected_format: Some("valid regex expression".to_string()),
                config_source: None,
                validation_rule: None,
            }
            .into());
        }
    }

    Ok(())
}

/// Perform constants health check
pub fn health_check_constants() -> crate::Result<ConstantsHealthReport> {
    let mut warnings = Vec::new();
    let mut errors = Vec::new();

    // Check timeout values
    if HTTP_TIMEOUT < Duration::from_secs(5) {
        warnings.push("HTTP_TIMEOUT is very low, may cause network failures".to_string());
    }

    // Check cache sizes
    if MAX_CACHE_ENTRIES < 100 {
        warnings.push("MAX_CACHE_ENTRIES is low, may impact performance".to_string());
    }

    // Check file size limits
    if MAX_FILE_SIZE > 50 * 1024 * 1024 {
        warnings.push("MAX_FILE_SIZE is very high, may cause memory issues".to_string());
    }

    // Validate regex patterns
    if let Err(e) = validate_regex_patterns() {
        errors.push(format!("Regex validation failed: {e}"));
    }

    // Calculate performance_optimal before moving the vectors
    let performance_optimal = errors.is_empty() && warnings.len() < 3;

    Ok(ConstantsHealthReport {
        total_constants: REGEX_PATTERNS.len()
            + ERROR_CODE_STRATEGIES.len()
            + DEFAULT_FEATURES.len(),
        warnings,
        errors,
        performance_optimal,
    })
}

/// Constants health report
#[derive(Debug, Clone)]
pub struct ConstantsHealthReport {
    /// Total number of constants defined
    pub total_constants: usize,
    /// Warning messages
    pub warnings: Vec<String>,
    /// Error messages
    pub errors: Vec<String>,
    /// Whether configuration is performance optimal
    pub performance_optimal: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_patterns_compile() {
        assert!(validate_regex_patterns().is_ok());
    }

    #[test]
    fn test_error_severity_lookup() {
        assert_eq!(get_error_severity("E0308"), 255);
        assert_eq!(get_error_severity("E0599"), 180);
        assert_eq!(get_error_severity("UNKNOWN"), 128);
    }

    #[test]
    fn test_correction_strategy_lookup() {
        assert_eq!(get_correction_strategy("E0599"), Some("method_not_found"));
        assert_eq!(get_correction_strategy("E0308"), Some("type_mismatch"));
        assert_eq!(get_correction_strategy("UNKNOWN"), None);
    }

    #[test]
    fn test_confidence_thresholds() {
        assert_eq!(get_confidence_threshold("import_addition"), 0.95);
        assert_eq!(
            get_confidence_threshold("unknown"),
            DEFAULT_SIMILARITY_THRESHOLD
        );
    }

    #[test]
    fn test_feature_flags() {
        assert!(is_feature_enabled_by_default("enable_docs_scraping"));
        assert!(!is_feature_enabled_by_default("enable_debug_logging"));
        assert!(!is_feature_enabled_by_default("unknown_feature"));
    }

    #[test]
    fn test_constants_health_check() {
        let report = health_check_constants().unwrap();
        assert!(report.total_constants > 0);
        println!("Constants health: {:?}", report);
    }
}
