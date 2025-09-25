/* yoshi-deluxe/src/errors.rs */
//! **Brief:** Comprehensive error handling using yoshi framework for yoshi-deluxe.
//!
//! This module provides robust error handling that integrates seamlessly with the yoshi
//! error framework, offering structured error types with rich context, auto-fix suggestions,
//! and comprehensive diagnostic information for all auto-correction system operations.

use std::{collections::HashMap, error::Error as StdError, path::PathBuf, time::Duration};
use yoshi_derive::YoshiError;
use yoshi_std::{Hatch, HatchExt, Hatchable, Result as YoshiResult, Yoshi, YoshiKind};

//--------------------------------------------------------------------------------------------------
// Core Error Types with Yoshi Integration
//--------------------------------------------------------------------------------------------------

/// Comprehensive auto-correction system errors with yoshi integration
#[derive(Debug, YoshiError)]
#[yoshi(
    default_severity = 128,
    namespace = "yoshi_deluxe",
    generate_helpers = true,
    auto_inference = true
)]
pub enum AutoCorrectionError {
    /// Diagnostic processing failed with context
    #[yoshi(
        display = "Failed to process compiler diagnostic: {message}",
        kind = "Validation",
        severity = 160,
        suggestion = "Verify cargo check output format and project structure",
        doc_url = "https://docs.rs/yoshi-deluxe/latest/yoshi_deluxe/errors/"
    )]
    DiagnosticProcessing {
        /// Error details
        message: String,
        /// Source diagnostic data if available
        #[yoshi(context = "diagnostic_data")]
        diagnostic_data: Option<String>,
        /// Project path context
        #[yoshi(context = "project_path")]
        project_path: String,
        /// Cargo command that failed
        #[yoshi(context = "cargo_command")]
        cargo_command: Option<String>,
    },

    /// AST analysis failed with precise location information
    #[yoshi(
        display = "AST analysis failed at {file_path}:{line}:{column}: {reason}",
        kind = "Internal",
        severity = 180,
        suggestion = "Check source file syntax and ensure valid Rust code",
        category = "ast_analysis"
    )]
    AstAnalysis {
        /// Reason for analysis failure
        reason: String,
        /// Source file path
        file_path: String,
        /// Line number (1-indexed)
        line: usize,
        /// Column number (1-indexed)
        column: usize,
        /// Byte offset if available
        #[yoshi(context = "byte_offset")]
        byte_offset: Option<usize>,
        /// Source error if chained
        #[yoshi(context = "source_error")]
        source_error: Option<String>,
        /// AST node type that failed
        #[yoshi(context = "node_type")]
        node_type: Option<String>,
    },

    /// Documentation scraping encountered issues
    #[yoshi(
        display = "Documentation scraping failed for {crate_name}::{type_name}: {error_type}",
        kind = "Network",
        severity = 120,
        transient = true,
        suggestion = "Check network connectivity and verify crate exists on docs.rs",
        category = "documentation"
    )]
    DocumentationScraping {
        /// Target crate name
        crate_name: String,
        /// Target type name
        type_name: String,
        /// Type of error encountered
        error_type: String,
        /// HTTP status code if available
        #[yoshi(context = "http_status")]
        http_status: Option<u16>,
        /// Underlying network error
        #[yoshi(context = "network_error")]
        network_error: Option<String>,
        /// Attempted URL
        #[yoshi(context = "attempted_url")]
        attempted_url: Option<String>,
        /// Retry attempt number
        #[yoshi(context = "retry_attempt")]
        retry_attempt: Option<usize>,
    },

    /// Code generation failed with correction context
    #[yoshi(
        display = "Code generation failed for {correction_type}: {details}",
        kind = "Internal",
        severity = 200,
        suggestion = "Review correction logic and ensure valid Rust syntax generation",
        category = "code_generation"
    )]
    CodeGeneration {
        /// Type of correction being attempted
        correction_type: String,
        /// Specific failure details
        details: String,
        /// Original problematic code
        #[yoshi(context = "original_code")]
        original_code: String,
        /// Generation context metadata
        #[yoshi(context = "generation_context")]
        generation_context: HashMap<String, String>,
        /// Confidence score when generation failed
        #[yoshi(context = "confidence_score")]
        confidence_score: Option<f64>,
        /// Validation errors if any
        #[yoshi(context = "validation_errors")]
        validation_errors: Option<Vec<String>>,
    },

    /// File operations failed with comprehensive context
    #[yoshi(
        display = "File operation failed: {operation} on {file_path}",
        kind = "Io",
        severity = 140,
        suggestion = "Check file permissions, disk space, and file existence",
        category = "file_operations"
    )]
    FileOperation {
        /// Type of file operation
        operation: String,
        /// Target file path
        file_path: String,
        /// File size if relevant
        #[yoshi(context = "file_size")]
        file_size: Option<u64>,
        /// Underlying IO error
        #[yoshi(source)]
        io_error: std::io::Error,
        /// Expected file permissions
        #[yoshi(context = "expected_permissions")]
        expected_permissions: Option<String>,
        /// Actual file permissions
        #[yoshi(context = "actual_permissions")]
        actual_permissions: Option<String>,
    },

    /// Configuration issues with system setup
    #[yoshi(
        display = "Configuration error: {parameter} = {value}",
        kind = "Config",
        severity = 100,
        suggestion = "Review system configuration and ensure valid parameter values",
        category = "configuration"
    )]
    Configuration {
        /// Configuration parameter name
        parameter: String,
        /// Invalid value
        value: String,
        /// Expected value format
        #[yoshi(context = "expected_format")]
        expected_format: Option<String>,
        /// Configuration source
        #[yoshi(context = "config_source")]
        config_source: Option<String>,
        /// Validation rule that failed
        #[yoshi(context = "validation_rule")]
        validation_rule: Option<String>,
    },

    /// Resource exhaustion errors
    #[yoshi(
        display = "Resource exhausted: {resource_type} (limit: {limit}, requested: {requested})",
        kind = "ResourceExhausted",
        severity = 220,
        transient = true,
        suggestion = "Reduce resource usage or increase system limits",
        category = "resource_management"
    )]
    ResourceExhausted {
        /// Type of resource
        resource_type: String,
        /// Resource limit
        limit: u64,
        /// Requested amount
        requested: u64,
        /// Current usage
        #[yoshi(context = "current_usage")]
        current_usage: Option<u64>,
        /// Resource pool identifier
        #[yoshi(context = "resource_pool")]
        resource_pool: Option<String>,
    },

    /// Cache operation failures
    #[yoshi(
        display = "Cache operation failed: {operation} for key '{cache_key}'",
        kind = "Internal",
        severity = 110,
        transient = true,
        suggestion = "Check cache configuration and available memory",
        category = "caching"
    )]
    CacheOperation {
        /// Cache operation type
        operation: String,
        /// Cache key involved
        cache_key: String,
        /// Cache type (docs, ast, etc.)
        #[yoshi(context = "cache_type")]
        cache_type: String,
        /// Cache size at time of failure
        #[yoshi(context = "cache_size")]
        cache_size: Option<usize>,
        /// Error reason
        reason: String,
    },

    /// Parsing and syntax errors
    #[yoshi(
        display = "Parsing failed for {content_type}: {error_message}",
        kind = "Validation",
        severity = 150,
        suggestion = "Verify input format and syntax",
        category = "parsing"
    )]
    ParsingFailure {
        /// Type of content being parsed
        content_type: String,
        /// Parsing error message
        error_message: String,
        /// Input content snippet
        #[yoshi(context = "content_snippet")]
        content_snippet: Option<String>,
        /// Expected format
        #[yoshi(context = "expected_format")]
        expected_format: Option<String>,
        /// Parser used
        #[yoshi(context = "parser")]
        parser: Option<String>,
    },

    /// Timeout errors for long-running operations
    #[yoshi(
        display = "Operation timed out: {operation} after {duration:?}",
        kind = "Timeout",
        severity = 130,
        transient = true,
        suggestion = "Increase timeout limit or optimize operation performance",
        category = "performance"
    )]
    OperationTimeout {
        /// Operation that timed out
        operation: String,
        /// Actual duration before timeout
        duration: Duration,
        /// Expected maximum duration
        #[yoshi(context = "max_duration")]
        max_duration: Option<Duration>,
        /// Operation context
        #[yoshi(context = "operation_context")]
        operation_context: Option<String>,
    },

    /// Version compatibility issues
    #[yoshi(
        display = "Version compatibility error: {component} requires {required_version}, found {actual_version}",
        kind = "Validation",
        severity = 170,
        suggestion = "Update dependencies to compatible versions",
        category = "compatibility"
    )]
    VersionCompatibility {
        /// Component with version issue
        component: String,
        /// Required version
        required_version: String,
        /// Actual version found
        actual_version: String,
        /// Compatibility rule
        #[yoshi(context = "compatibility_rule")]
        compatibility_rule: Option<String>,
    },
}

//--------------------------------------------------------------------------------------------------
// Convenient Result Type Aliases
//--------------------------------------------------------------------------------------------------

/// Convenient Result type alias using yoshi integration
pub type Result<T> = YoshiResult<T>;

/// Hatch type alias for yoshi-deluxe operations
pub type DeluxeHatch<T> = Hatch<T>;

//--------------------------------------------------------------------------------------------------
// Error Enhancement Traits and Extensions
//--------------------------------------------------------------------------------------------------

/// Extension trait for enhancing errors with yoshi-deluxe specific context
pub trait YoshiDeluxeExt<T> {
    /// Add file context to an error
    fn with_file_context(self, file_path: &std::path::Path) -> Result<T>;

    /// Add operation context to an error
    fn with_operation_context(self, operation: &str) -> Result<T>;

    /// Add performance context to an error
    fn with_performance_context(self, duration: Duration) -> Result<T>;

    /// Add correction context to an error
    fn with_correction_context(self, correction_type: &str, confidence: f64) -> Result<T>;
}

impl<T, E> YoshiDeluxeExt<T> for std::result::Result<T, E>
where
    E: StdError + Send + Sync + Into<Yoshi> + 'static,
{
    fn with_file_context(self, file_path: &std::path::Path) -> Result<T> {
        self.hatch()
            .meta("file_path", file_path.display().to_string())
            .meta(
                "file_name",
                file_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown"),
            )
    }

    fn with_operation_context(self, operation: &str) -> Result<T> {
        self.hatch().meta("operation", operation).meta(
            "timestamp",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
                .to_string(),
        )
    }

    fn with_performance_context(self, duration: Duration) -> Result<T> {
        self.hatch()
            .meta("duration_ms", duration.as_millis().to_string())
            .meta(
                "performance_category",
                if duration.as_millis() > 1000 {
                    "slow"
                } else if duration.as_millis() > 100 {
                    "medium"
                } else {
                    "fast"
                },
            )
    }

    fn with_correction_context(self, correction_type: &str, confidence: f64) -> Result<T> {
        self.hatch()
            .meta("correction_type", correction_type)
            .meta("confidence_score", confidence.to_string())
            .meta(
                "confidence_level",
                if confidence > 0.9 {
                    "high"
                } else if confidence > 0.7 {
                    "medium"
                } else {
                    "low"
                },
            )
    }
}

//--------------------------------------------------------------------------------------------------
// Error Factories for Common Patterns
//--------------------------------------------------------------------------------------------------

/// Factory functions for creating common error types with proper context
pub mod factory {
    use super::*;

    /// Create a diagnostic processing error with context
    pub fn diagnostic_processing_error(
        message: impl Into<String>,
        project_path: impl Into<PathBuf>,
    ) -> Yoshi {
        AutoCorrectionError::DiagnosticProcessing {
            message: message.into(),
            diagnostic_data: None,
            project_path: project_path.into().display().to_string(),
            cargo_command: None,
        }
        .into()
    }

    /// Create an AST analysis error with location
    pub fn ast_analysis_error(
        reason: impl Into<String>,
        file_path: impl Into<PathBuf>,
        line: usize,
        column: usize,
        source_error: syn::Error,
    ) -> Yoshi {
        AutoCorrectionError::AstAnalysis {
            reason: reason.into(),
            file_path: file_path.into().display().to_string(),
            line,
            column,
            byte_offset: None,
            source_error: Some(source_error.to_string()),
            node_type: None,
        }
        .into()
    }

    /// Create a documentation scraping error
    pub fn docs_scraping_error(
        crate_name: impl Into<String>,
        type_name: impl Into<String>,
        error_type: impl Into<String>,
        network_error: impl Into<String>,
    ) -> Yoshi {
        AutoCorrectionError::DocumentationScraping {
            crate_name: crate_name.into(),
            type_name: type_name.into(),
            error_type: error_type.into(),
            http_status: None,
            network_error: Some(network_error.into()),
            attempted_url: None,
            retry_attempt: None,
        }
        .into()
    }

    /// Create a code generation error
    pub fn code_generation_error(
        correction_type: impl Into<String>,
        details: impl Into<String>,
        original_code: impl Into<String>,
    ) -> Yoshi {
        AutoCorrectionError::CodeGeneration {
            correction_type: correction_type.into(),
            details: details.into(),
            original_code: original_code.into(),
            generation_context: HashMap::new(),
            confidence_score: None,
            validation_errors: None,
        }
        .into()
    }

    /// Create a file operation error
    pub fn file_operation_error(
        operation: impl Into<String>,
        file_path: impl Into<PathBuf>,
        io_error: std::io::Error,
    ) -> Yoshi {
        AutoCorrectionError::FileOperation {
            operation: operation.into(),
            file_path: file_path.into().display().to_string(),
            file_size: None,
            io_error,
            expected_permissions: None,
            actual_permissions: None,
        }
        .into()
    }

    /// Create a configuration error
    pub fn configuration_error(parameter: impl Into<String>, value: impl Into<String>) -> Yoshi {
        AutoCorrectionError::Configuration {
            parameter: parameter.into(),
            value: value.into(),
            expected_format: None,
            config_source: None,
            validation_rule: None,
        }
        .into()
    }

    /// Create a resource exhausted error
    pub fn resource_exhausted_error(
        resource_type: impl Into<String>,
        limit: u64,
        requested: u64,
    ) -> Yoshi {
        AutoCorrectionError::ResourceExhausted {
            resource_type: resource_type.into(),
            limit,
            requested,
            current_usage: None,
            resource_pool: None,
        }
        .into()
    }

    /// Create a timeout error
    pub fn timeout_error(operation: impl Into<String>, duration: Duration) -> Yoshi {
        AutoCorrectionError::OperationTimeout {
            operation: operation.into(),
            duration,
            max_duration: None,
            operation_context: None,
        }
        .into()
    }
}

//--------------------------------------------------------------------------------------------------
// Error Context Builders
//--------------------------------------------------------------------------------------------------

/// Builder for adding rich context to errors
#[derive(Debug, Default)]
pub struct ErrorContextBuilder {
    metadata: HashMap<String, String>,
    suggestions: Vec<String>,
    context_layers: Vec<String>,
}

impl ErrorContextBuilder {
    /// Create a new error context builder
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add metadata key-value pair
    #[must_use]
    pub fn meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Add a suggestion
    #[must_use]
    pub fn suggest(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }

    /// Add a context layer
    #[must_use]
    pub fn context(mut self, context: impl Into<String>) -> Self {
        self.context_layers.push(context.into());
        self
    }

    /// Apply the built context to a yoshi error
    pub fn apply_to(self, mut error: Yoshi) -> Yoshi {
        // Add metadata
        for (key, value) in self.metadata {
            error = error.with_metadata(&key, &value);
        }

        // Add suggestions
        for suggestion in self.suggestions {
            error = error.with_suggestion(&suggestion);
        }

        // Add context layers
        for context in self.context_layers {
            error = error.context(&context);
        }

        error
    }
}

//--------------------------------------------------------------------------------------------------
// Error Analysis and Reporting
//--------------------------------------------------------------------------------------------------

/// Analyze error patterns and provide insights
pub struct ErrorAnalyzer;

impl ErrorAnalyzer {
    /// Analyze an error and provide categorization
    #[must_use]
    pub fn analyze_error(error: &Yoshi) -> ErrorAnalysis {
        ErrorAnalysis {
            category: Self::categorize_error(error),
            severity_level: Self::assess_severity(error),
            recovery_suggestions: Self::generate_recovery_suggestions(error),
            is_transient: error.is_transient(),
            error_pattern: Self::identify_pattern(error),
        }
    }

    fn categorize_error(error: &Yoshi) -> String {
        let error_str = error.to_string().to_lowercase();

        if error_str.contains("network") || error_str.contains("http") {
            "Network"
        } else if error_str.contains("file") || error_str.contains("io") {
            "File System"
        } else if error_str.contains("parse") || error_str.contains("syntax") {
            "Parsing"
        } else if error_str.contains("config") {
            "Configuration"
        } else if error_str.contains("timeout") {
            "Performance"
        } else {
            "General"
        }
        .to_string()
    }

    fn assess_severity(error: &Yoshi) -> SeverityLevel {
        let severity = error.severity();

        if severity >= 200 {
            SeverityLevel::Critical
        } else if severity >= 150 {
            SeverityLevel::High
        } else if severity >= 100 {
            SeverityLevel::Medium
        } else {
            SeverityLevel::Low
        }
    }

    fn generate_recovery_suggestions(error: &Yoshi) -> Vec<String> {
        let mut suggestions = Vec::new();
        let error_str = error.to_string().to_lowercase();

        if error_str.contains("network") {
            suggestions.push("Check network connectivity".to_string());
            suggestions.push("Verify proxy settings".to_string());
            suggestions.push("Try again with retries".to_string());
        }

        if error_str.contains("file") {
            suggestions.push("Check file permissions".to_string());
            suggestions.push("Verify file exists".to_string());
            suggestions.push("Check available disk space".to_string());
        }

        if error_str.contains("timeout") {
            suggestions.push("Increase timeout duration".to_string());
            suggestions.push("Optimize operation performance".to_string());
            suggestions.push("Break operation into smaller chunks".to_string());
        }

        if suggestions.is_empty() {
            suggestions.push("Review error details and context".to_string());
            suggestions.push("Check system logs for more information".to_string());
        }

        suggestions
    }

    fn identify_pattern(error: &Yoshi) -> String {
        let error_str = error.to_string();

        if error_str.contains("E0599") {
            "Method Not Found"
        } else if error_str.contains("E0308") {
            "Type Mismatch"
        } else if error_str.contains("E0425") {
            "Unresolved Name"
        } else if error_str.contains("Connection refused") {
            "Network Connection Issue"
        } else if error_str.contains("Permission denied") {
            "File Permission Issue"
        } else {
            "Unknown Pattern"
        }
        .to_string()
    }
}

/// Error analysis result
#[derive(Debug, Clone)]
pub struct ErrorAnalysis {
    /// Error category
    pub category: String,
    /// Severity assessment
    pub severity_level: SeverityLevel,
    /// Recovery suggestions
    pub recovery_suggestions: Vec<String>,
    /// Whether error is transient
    pub is_transient: bool,
    /// Identified error pattern
    pub error_pattern: String,
}

/// Severity level enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeverityLevel {
    /// Low severity
    Low,
    /// Medium severity
    Medium,
    /// High severity
    High,
    /// Critical severity
    Critical,
}

//--------------------------------------------------------------------------------------------------
// Error Recovery Strategies
//--------------------------------------------------------------------------------------------------

/// Error recovery strategy implementations
pub mod recovery {
    use super::*;

    /// Attempt automatic error recovery
    pub async fn attempt_recovery<T, F, Fut>(operation: F, max_retries: usize) -> Result<T>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        let mut last_error = None;

        for attempt in 0..=max_retries {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(error) => {
                    if attempt == max_retries {
                        last_error = Some(error);
                        break;
                    }

                    // Wait before retry (exponential backoff)
                    let delay = Duration::from_millis(100 * 2_u64.pow(attempt as u32));
                    tokio::time::sleep(delay).await;
                }
            }
        }

        Err(last_error.unwrap_or_else(|| {
            factory::configuration_error("recovery_operation", "unknown_failure")
        }))
    }

    /// Retry operation with specific error patterns
    pub async fn retry_on_pattern<T, F, Fut>(
        operation: F,
        max_retries: usize,
        retry_patterns: &[&str],
    ) -> Result<T>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        for attempt in 0..=max_retries {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(error) => {
                    let error_str = error.to_string().to_lowercase();
                    let should_retry = retry_patterns
                        .iter()
                        .any(|pattern| error_str.contains(&pattern.to_lowercase()));

                    if !should_retry || attempt == max_retries {
                        return Err(error);
                    }

                    let delay = Duration::from_millis(200 * (attempt + 1) as u64);
                    tokio::time::sleep(delay).await;
                }
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error =
            factory::diagnostic_processing_error("Test diagnostic error", "/tmp/test-project");

        assert!(error.to_string().contains("Test diagnostic error"));
        assert_eq!(error.severity(), 160);
    }

    #[test]
    fn test_error_context_builder() {
        let error = factory::configuration_error("test_param", "invalid_value");

        let enhanced_error = ErrorContextBuilder::new()
            .meta("component", "test_component")
            .meta("version", "1.0.0")
            .suggest("Update configuration file")
            .context("During system initialization")
            .apply_to(error);

        assert!(enhanced_error.to_string().contains("test_param"));
    }

    #[test]
    fn test_error_analysis() {
        let error = factory::timeout_error("database_query", Duration::from_secs(30));

        let analysis = ErrorAnalyzer::analyze_error(&error);
        assert_eq!(analysis.category, "Performance");
        assert!(analysis.recovery_suggestions.len() > 0);
    }

    #[test]
    fn test_yoshi_deluxe_ext() {
        let result: std::result::Result<(), std::io::Error> = Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found",
        ));

        let enhanced = result.with_file_context(std::path::Path::new("/tmp/test.rs"));
        assert!(enhanced.is_err());
    }

    #[tokio::test]
    async fn test_recovery_retry() {
        let mut attempt_count = 0;

        let result = recovery::attempt_recovery(
            || {
                attempt_count += 1;
                async move {
                    if attempt_count < 3 {
                        Err(factory::timeout_error(
                            "test_op",
                            Duration::from_millis(100),
                        ))
                    } else {
                        Ok(42)
                    }
                }
            },
            5,
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempt_count, 3);
    }
}
