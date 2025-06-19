//! **Error Handling Framework for Yoshi-Deluxe**
//!
//! This module provides comprehensive error handling that leverages the foundational
//! yoshi-std and yoshi-core infrastructure for structured, contextual error management.
//! It implements the `YoshiACE` (Yoshi Auto-Correction Engine) error types and provides
//! convenient re-exports and trait implementations for seamless error handling throughout
//! the auto-correction system.
//!
//! ## Key Features
//!
//! - **Structured Error Types**: Domain-specific error variants for different failure modes
//! - **Contextual Error Enhancement**: `Hatchling` trait for adding context to any error
//! - **Foundational Integration**: Full compatibility with yoshi-std error framework
//! - **Convenient Re-exports**: Single import point for all error handling needs
//! - **Zero-Cost Abstractions**: Compile-time optimized error handling patterns
//!
//! ## Error Categories
//!
//! The `YoshiACE` enum provides specialized error variants for:
//! - **Diagnostic Processing**: Errors during compiler diagnostic analysis
//! - **AST Analysis**: Failures in syntax tree parsing and manipulation
//! - **Documentation Scraping**: Network and parsing errors during docs retrieval
//! - **Code Generation**: Issues during correction proposal generation
//! - **File Operations**: I/O errors with enhanced context
//! - **Configuration**: Invalid system configuration parameters
//! - **Resource Management**: Memory, file size, and concurrency limits
//! - **Operation Timeouts**: Time-based operation failures
//!
//! ## Usage Examples
//!
//! ```rust
//! use yoshi_deluxe::err::{YoshiACE, Hatch, Hatchling};
//! use std::path::Path;
//!
//! // Using YoshiACE for domain-specific errors
//! fn validate_config(max_size: usize) -> Hatch<()> {
//!     if max_size == 0 {
//!         return Err(YoshiACE::Configuration {
//!             _parameter: "max_size".to_string(),
//!             _value: "0".to_string(),
//!         }.into());
//!     }
//!     Ok(())
//! }
//!
//! // Using Hatchling trait for context enhancement
//! async fn read_config_file(path: &Path) -> Hatch<String> {
//!     tokio::fs::read_to_string(path)
//!         .await
//!         .with_file_context(path)
//!         .lay("Reading configuration file")
//! }
//! ```
//!
//! ## Performance Characteristics
//!
//! - **Zero-Cost**: Error handling compiles to optimal machine code
//! - **Memory Efficient**: Structured error data with minimal overhead
//! - **Context Preservation**: Full error chain with source location tracking
//! - **Thread Safe**: All error types implement `Send + Sync`

pub use std::error::Error;
pub use std::io::Error as IoError;
pub use std::time::Duration;
pub use yoshi_core::YoshiKind;
pub use yoshi_derive::YoshiError;
pub use yoshi_std::{Hatch, LayText, Yoshi};

//--------------------------------------------------------------------------------------------------
// Type Aliases
//--------------------------------------------------------------------------------------------------

/// Convenient Result type alias using yoshi integration (deprecated - use Hatch directly)
#[deprecated(note = "Use Hatch<T> directly instead of Result<T>")]
pub type Result<T> = Hatch<T>;

//--------------------------------------------------------------------------------------------------
// Error Types using YoshiError derive
//--------------------------------------------------------------------------------------------------

/// **Yoshi Auto-Correction Engine Error Types**
///
/// Comprehensive error enumeration for the yoshi-deluxe auto-correction system.
/// Each variant represents a specific failure mode with detailed context information
/// to enable precise error diagnosis and recovery strategies.
///
/// This enum uses the `YoshiError` derive macro from `yoshi-derive` to automatically
/// implement error handling traits and integration with the yoshi error framework.
#[derive(Debug, YoshiError)]
pub enum YoshiACE {
    /// **Diagnostic Processing Failure**
    ///
    /// Occurs when the system fails to process compiler diagnostics from cargo check/clippy.
    DiagnosticProcessing {
        /// Human-readable error description
        _message: String,
        /// Path to the project being analyzed
        _project_path: std::path::PathBuf,
    },

    /// **AST Analysis Failure**
    ///
    /// Represents failures during Abstract Syntax Tree parsing, analysis, or manipulation.
    AstAnalysis {
        /// Detailed reason for the analysis failure
        _reason: String,
        /// Path to the file that failed analysis
        _file_path: std::path::PathBuf,
        /// Line number where the error occurred
        _line: usize,
        /// Column number where the error occurred
        _column: usize,
        /// Optional byte offset in the file
        _byte_offset: Option<usize>,
        /// Underlying syn parsing error
        _source_error: syn::Error,
    },

    /// **Documentation Scraping Failure**
    ///
    /// Occurs when the system fails to retrieve or parse documentation from external sources.
    DocumentationScraping {
        /// Name of the crate being scraped
        _crate_name: String,
        /// Specific type or item being looked up
        _type_name: String,
        /// Underlying network or HTTP error
        _network_error: reqwest::Error,
    },

    /// **Code Generation Failure**
    ///
    /// Represents failures during the generation of correction proposals.
    CodeGeneration {
        /// Detailed description of the generation failure
        _details: String,
        /// Type of correction being attempted
        _correction_type: String,
        /// Original code that was being corrected
        _original_code: String,
    },

    /// **File Operation Failure**
    ///
    /// Wraps I/O errors with additional context about the specific file operation.
    FileOperation {
        /// Description of the operation that failed
        _operation: String,
        /// Path to the file involved in the operation
        _file_path: std::path::PathBuf,
        /// Underlying I/O error
        _io_error: IoError,
    },

    /// **Configuration Error**
    ///
    /// Indicates invalid system configuration parameters or settings.
    Configuration {
        /// Name of the configuration parameter
        _parameter: String,
        /// Invalid value that was provided
        _value: String,
    },

    /// **Resource Exhaustion**
    ///
    /// Occurs when system resources are exhausted or limits are exceeded.
    ResourceExhausted {
        /// Type of resource that was exhausted
        _resource_type: String,
        /// Maximum allowed limit
        _limit: u64,
        /// Amount that was requested
        _requested: u64,
    },

    /// **Operation Timeout**
    ///
    /// Represents timeouts during long-running operations.
    OperationTimeout {
        /// Description of the operation that timed out
        _operation: String,
        /// Duration that the operation took before timing out
        _duration: Duration,
    },
}

//--------------------------------------------------------------------------------------------------
// Error Enhancement Traits and Extensions
//--------------------------------------------------------------------------------------------------

/// **Error Enhancement Trait for Contextual Error Handling**
///
/// The `Hatchling` trait provides methods for enhancing any error with domain-specific
/// context information. This trait is automatically implemented for `std::result::Result`
/// types where the error implements the standard error traits.
///
/// ## Design Philosophy
///
/// Rather than requiring specific error types, this trait allows any error to be enhanced
/// with yoshi-deluxe specific context, making error handling more flexible and informative.
///
/// ## Usage Examples
///
/// ```rust
/// use yoshi_deluxe::err::Hatchling;
/// use std::path::Path;
///
/// async fn read_config(path: &Path) -> yoshi_deluxe::Hatch<String> {
///     tokio::fs::read_to_string(path)
///         .await
///         .with_file_context(path)
///         .with_operation_context("config_loading")
/// }
/// ```
///
/// ## Performance Notes
///
/// All context enhancement methods are zero-cost when errors don't occur,
/// and minimal-cost when they do, as they only allocate for error formatting.
pub trait Hatchling<T> {
    /// **Add File Context to Error**
    ///
    /// Enhances an error with file path information, useful for I/O operations
    /// and file-related failures.
    ///
    /// # Arguments
    /// * `file_path` - Path to the file involved in the operation
    ///
    /// # Returns
    /// Enhanced error with file context information
    fn with_file_context(self, file_path: &std::path::Path) -> Hatch<T>;

    /// **Add Operation Context to Error**
    ///
    /// Enhances an error with information about the specific operation that failed.
    ///
    /// # Arguments
    /// * `operation` - Description of the operation that failed
    ///
    /// # Returns
    /// Enhanced error with operation context information
    fn with_operation_context(self, operation: &str) -> Hatch<T>;

    /// **Add Performance Context to Error**
    ///
    /// Enhances an error with timing information, useful for timeout scenarios
    /// and performance analysis.
    ///
    /// # Arguments
    /// * `duration` - How long the operation took before failing
    ///
    /// # Returns
    /// Enhanced error with performance timing information
    fn with_performance_context(self, duration: Duration) -> Hatch<T>;

    /// **Add Correction Context to Error**
    ///
    /// Enhances an error with information about a failed correction attempt,
    /// including the type of correction and confidence level.
    ///
    /// # Arguments
    /// * `correction_type` - Type of correction that was attempted
    /// * `confidence` - Confidence level of the correction (0.0 to 1.0)
    ///
    /// # Returns
    /// Enhanced error with correction attempt information
    fn with_correction_context(self, correction_type: &str, confidence: f64) -> Hatch<T>;
}

impl<T, E> Hatchling<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn with_file_context(self, file_path: &std::path::Path) -> Hatch<T> {
        self.map_err(|e| {
            Yoshi::new(YoshiKind::Internal {
                message: format!("File operation failed: {}", file_path.display()).into(),
                source: Some(Box::new(Yoshi::new(YoshiKind::Internal {
                    message: e.to_string().into(),
                    source: None,
                    component: Some("file_context".into()),
                }))),
                component: Some("file_operation".into()),
            })
            .lay(format!("File context: {}", file_path.display()))
        })
    }

    fn with_operation_context(self, operation: &str) -> Hatch<T> {
        self.map_err(|e| {
            Yoshi::new(YoshiKind::Internal {
                message: format!("Operation failed: {}", operation).into(),
                source: Some(Box::new(Yoshi::new(YoshiKind::Internal {
                    message: e.to_string().into(),
                    source: None,
                    component: Some("operation_context".into()),
                }))),
                component: Some("operation".into()),
            })
            .lay(format!("Operation context: {}", operation))
        })
    }

    fn with_performance_context(self, duration: Duration) -> Hatch<T> {
        self.map_err(|_e| {
            Yoshi::new(YoshiKind::Timeout {
                operation: "performance_context".into(),
                duration,
                expected_max: Some(Duration::from_millis(100)),
            })
            .lay(format!("Performance issue: {:?}", duration))
        })
    }

    fn with_correction_context(self, correction_type: &str, confidence: f64) -> Hatch<T> {
        self.map_err(|e| {
            Yoshi::new(YoshiKind::Internal {
                message: format!("Correction failed: {}", correction_type).into(),
                source: Some(Box::new(Yoshi::new(YoshiKind::Internal {
                    message: e.to_string().into(),
                    source: None,
                    component: Some("correction_context".into()),
                }))),
                component: Some("correction".into()),
            })
            .lay(format!(
                "Correction context: {} (confidence: {})",
                correction_type, confidence
            ))
        })
    }
}
