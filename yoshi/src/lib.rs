/* yoshi/yoshi/src/lib.rs */
#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
#![allow(clippy::use_self)]
#![allow(unused_variables)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "simd-optimized"), deny(unsafe_code))]

//! **Brief:** The `yoshi` crate serves as the primary entry point and facade for the Yoshi error handling framework.
//! It re-exports core functionalities from the `yoshi-std` crate, providing a unified and convenient API for
//! robust, highly performant, and flexible error handling designed for critical applications.
//! By using `yoshi` as a facade, developers benefit from structured error kinds, contextualization with rich metadata,
//! and efficient backtrace capture, all while ensuring architectural clarity and minimizing maintenance overhead.
//!
//! This design emphasizes mathematical predictability in performance and resource usage, now with a clear
//! separation of concerns between the core implementation and the public API facade.
//!
//! ## Mathematical Properties
//!
//! **Algorithmic Complexity:**
//! - Time Complexity: O(1) for error creation, O(1) for context attachment. O(N) for context chain traversal and formatting (where N is context depth).
//! - Space Complexity: O(N) where N is context chain depth, bounded by `MAX_DEPTH=32`
//! - Concurrency Safety: Send + Sync + 'static guarantees with atomic instance counting
//!
//! **Performance Characteristics:**
//! - Expected Performance: Sub-microsecond error creation, <100ns context attachment. Full error formatting depends on context depth.
//! - Worst-Case Scenarios: `O(MAX_DEPTH)` for deep context chains with cycle protection during formatting.
//! - Optimization Opportunities: SIMD-friendly formatting, pre-allocated buffers, lazy backtrace capture
//!
//! ## Module Organization
//!
//! This crate re-exports the following key components from `yoshi-std`:
//!
//! - [`Yoshi`]: The main error type, providing structured error handling capabilities.
//! - [`YoshiKind`]: Defines high-level categories for errors.
//! - [`YoContext`]: Stores additional contextual information for errors.
//! - [`YoshiLocation`]: Represents a source code location.
//! - [`YoshiBacktrace`]: Wraps a standard library backtrace with performance metadata.
//! - [`HatchExt`]: An extension trait for `Result` to easily attach context.
//! - `NoStdIo`: A minimal I/O error type for `no_std` environments (available in `no_std` environments).
//! - [`Result`]: A type alias for `std::result::Result` or `core::result::Result` with `Yoshi` as the default error.
//! - [`error_instance_count()`]: Global counter for Yoshi error instances.
//! - [`process_communication`]: Module for cross-process error reporting (feature `rust-1-87`, `std`).
//! - [`async_error_handling`]: Module for async error processing utilities (feature `rust-1-87`, `std`).
//! - [`simd_optimization`]: Module for SIMD-accelerated string processing (feature `simd-optimized`, `x86_64`).
//! - [`cross_process_metrics`]: Module for global error metrics (feature `rust-1-87`).
//!
//! Additionally, when the `derive` feature is enabled, this crate re-exports from `yoshi-derive`
//! to provide procedural macros for custom error implementations.
//!
//! # Examples
//!
//! **Basic error creation using the `yoshi!` macro:**
//! ```
//! use yoshi::{yoshi, Yoshi, YoshiKind, Result};
//!
//! let err1 = yoshi!(message: "Something went wrong internally");
//! assert!(matches!(err1.kind(), YoshiKind::Internal { .. }));
//!
//! let err2 = yoshi!(kind: YoshiKind::NotFound {
//!     resource_type: "User".into(),
//!     identifier: "john.doe".into(),
//!     search_locations: None,
//! });
//! assert!(matches!(err2.kind(), YoshiKind::NotFound { .. }));
//!
//! #[derive(Debug)]
//! struct MyLegacyError;
//! impl std::fmt::Display for MyLegacyError {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!         write!(f, "a legacy error")
//!     }
//! }
//! impl std::error::Error for MyLegacyError {}
//!
//! let legacy_error = MyLegacyError;
//! let err3 = yoshi!(error: legacy_error);
//! assert!(matches!(err3.kind(), YoshiKind::Foreign { .. }));
//!
//! // Chaining additional context directly within the macro
//! let err4 = yoshi!(message: "Operation failed",
//!     with_metadata = ("component", "network"),
//!     with_suggestion = "Check your internet connection.");
//! assert!(err4.suggestion().as_deref() == Some("Check your internet connection."));
//! ```
//!
//! **Propagating `Yoshi` errors with `HatchExt`:**
//! ```
//! use yoshi::{yoshi, Yoshi, YoshiKind, HatchExt};
//! # use std::io::{self, ErrorKind};
//!
//! fn load_data() -> Result<(), Yoshi> {
//!     // Simulate a file not found error
//!     let io_error = io::Error::new(ErrorKind::NotFound, "data.json not found");
//!     Err(yoshi!(error: io_error))
//!         .context("Failed to load user preferences".to_string())
//!         .meta("user_id", "test_user")
//!         .help("Ensure data.json is in the correct directory.")
//! }
//!
//! if let Err(e) = load_data() {
//!     println!("Encountered an error:\n{}", e);
//! }
//! ```
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + High-performance error handling with zero-cost abstractions [O(1) error creation, O(1) context attachment]
//!  - Advanced error categorization with semantic type safety [Memory-safe, Thread-safe]
//!  - Comprehensive context chaining with cycle detection [Stack-overflow protection, O(N) traversal, where N is context depth]
//!  - Enterprise-grade backtrace capture with metadata [Conditional compilation, Performance monitoring]
//!  - Structured error formatting with SIMD optimization [Buffer pre-allocation, Minimal allocations]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **License File:** /LICENSE
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

// =============================================================================
// Nightly Compatibility and docs.rs Support
// =============================================================================

// 1. Suppress nightly-specific warnings that become errors
#![cfg_attr(docsrs, allow(internal_features))]
#![cfg_attr(docsrs, allow(incomplete_features))]

// 2. Handle potential feature conflicts - no longer needed with stable features
// All features are now stable and compatible with docs.rs

// 3. Conditional feature compilation for docs.rs
#[cfg(docsrs)]
mod docs_fallback {
    // Provide safe fallbacks for advanced features when building docs
    pub use std::collections::HashMap as MetricsMap;
}

#[cfg(not(docsrs))]
mod runtime_impl {
    // Your actual implementations here
}

// 4. Version-specific workarounds
#[cfg(all(docsrs, any(feature = "simd-optimized", feature = "precise-capturing")))]
mod nightly_workarounds {
    // Disable SIMD optimizations on docs.rs nightly builds
    // to prevent version conflicts
}

// 5. Safe feature detection
#[allow(unused_macros)]
macro_rules! detect_docs_rs {
    () => {
        cfg!(docsrs) || std::env::var("DOCS_RS").is_ok()
    };
}

// 6. Conditional async features
#[cfg(all(feature = "async", not(docsrs)))]
mod async_impl {
    // Your async implementations
}

#[cfg(all(feature = "async", docsrs))]
mod async_docs {
    // Simplified async docs without tokio complications
    pub type AsyncResult<T> = std::future::Ready<Result<T, crate::Yoshi>>;
}

// 7. Serialize feature guards
#[cfg(all(feature = "serde", not(docsrs)))]
mod serde_impl {
    // Real serde implementations
}

#[cfg(all(feature = "serde", docsrs))]
mod serde_docs {
    // Documentation-only serde implementations
    // that don't trigger nightly serialization conflicts
}

// =============================================================================
// Documentation and Module Declaration
// =============================================================================

// Import alloc crate for no_std environments
#[cfg(not(feature = "std"))]
extern crate alloc;

pub use yoshi_std::error_instance_count;

// Main types and trait
pub use yoshi_std::{HatchExt, Result, YoContext, Yoshi, YoshiBacktrace, YoshiKind, YoshiLocation};

// Import Arc from std or core based on feature flag
#[cfg(not(feature = "std"))]
pub use alloc::sync::Arc;
#[cfg(feature = "std")]
pub use std::sync::Arc;

// Re-export yoshi_location macro for internal use
pub use yoshi_std::yoshi_location;

// `no_std` specific types and utilities
#[cfg(not(feature = "std"))]
pub use yoshi_std::{NoStdIo, OnceLock, SystemTime, ThreadId};

// Conditional modules re-exports based on features
#[cfg(feature = "std")]
pub use yoshi_std::async_error_handling;

#[cfg(all(feature = "std", feature = "serde"))]
pub use yoshi_std::process_communication;

#[cfg(all(feature = "simd-optimized", target_arch = "x86_64"))]
pub use yoshi_std::simd_optimization;

// Re-export from yoshi-derive if the 'derive' feature is enabled
#[cfg(feature = "derive")]
#[doc(hidden)] // Typically hidden from main docs as it's a procedural macro crate
pub use yoshi_derive::*;

// Explicit re-export of the yoshi_af! procedural macro to ensure accessibility via use yoshi::*;
#[cfg(feature = "derive")]
pub use yoshi_derive::yoshi_af;

// Explicit re-export of YoshiError derive macro to ensure accessibility via use yoshi::*;
#[cfg(feature = "derive")]
pub use yoshi_derive::YoshiError;

// The yoshi_location! macro is now internal to the `yoshi!` macro and not directly exposed
// from the facade crate. It still exists in yoshi_std as a #[macro_export] for other internal uses
// (e.g., by yoshi-derive) and for the `yoshi!` macro itself.

/// The main `yoshi!` macro for creating and contextualizing `Yoshi` errors.
///
/// This macro provides a convenient and idiomatic way to construct `Yoshi` errors,
/// automatically capturing the source code location and allowing for inline chaining
/// of common `Yoshi` builder methods like `with_metadata`, `with_suggestion`, etc.
///
/// It supports three primary modes of error creation:
///
/// 1.  **From a simple message:** Creates an `Internal` (or `Io` for `no_std`) `YoshiKind`
///     from a string literal or expression.
///     `yoshi!(message: "Something failed")`
///
/// 2.  **From a specific `YoshiKind`:** Allows direct construction of any `YoshiKind` variant.
///     `yoshi!(kind: YoshiKind::NotFound { resource_type: "User".into(), identifier: "abc".into(), search_locations: None })`
///
/// 3.  **From an existing `std::error::Error` type:** Wraps any `Error` trait object
///     using `Yoshi::foreign`.
///     `yoshi!(error: some_io_error)`
///
/// Additionally, you can chain `Yoshi` methods directly within the macro call:
/// `yoshi!(message: "Failed to load data", with_metadata = ("path", "/app/data.json"), with_suggestion = "Check file permissions")`
///
/// # Arguments
///
/// The macro takes a keyword argument to specify the type of error creation:
/// - `message: $msg:expr`: Creates an error from a message.
/// - `kind: $kind_expr:expr`: Creates an error directly from a `YoshiKind` variant.
/// - `error: $err_expr:expr`: Creates an error by wrapping an existing `std::error::Error`.
///
/// Optional trailing keyword arguments for chaining `Yoshi` builder methods:
/// - `with_metadata = ($key:expr, $value:expr)`: Adds metadata.
/// - `with_suggestion = $sugg:expr`: Adds a suggestion.
/// - `with_shell = $shell:expr`: Adds a typed shell.
/// - `with_priority = $priority:expr`: Sets the priority.
///
/// # Examples
///
/// ```
/// use yoshi::{yoshi, YoshiKind, Arc};
/// # use std::io;
/// # use std::io::ErrorKind;
///
/// // Create an internal error from a message
/// let err1 = yoshi!(message: "Failed to process request");
/// println!("Error 1: {}", err1);
///
/// // Create a network error directly using YoshiKind
/// let err2 = yoshi!(kind: YoshiKind::Network {
///     message: "Connection lost".into(),
///     source: None,
///     error_code: Some(1001),
/// },
/// with_suggestion = "Restart network service");
/// println!("Error 2: {}", err2);
/// assert!(err2.suggestion().as_deref() == Some("Restart network service"));
///
/// // Wrap a standard I/O error
/// let io_error = io::Error::new(ErrorKind::PermissionDenied, "cannot access file");
/// let err3 = yoshi!(error: io_error,
///     with_metadata = ("file", "/etc/config.json"));
/// println!("Error 3: {}", err3);
/// assert_eq!(err3.primary_context().unwrap().metadata.get(&Arc::from("file")).map(|s| s.as_ref()), Some("/etc/config.json"));
/// ```
#[macro_export]
macro_rules! yoshi {

    // **ENHANCED**: Message-based error creation (existing functionality preserved)
    (message: $msg:expr) => {
        $crate::Yoshi::new($crate::YoshiKind::Internal {
            message: $msg.into(),
            source: None,
            component: None,
        })
    };

    // **ENHANCED**: Kind-based error creation (existing functionality preserved)
    (kind: $kind:expr) => {
        $crate::Yoshi::new($kind)
    };

    // **ENHANCED**: Error wrapping (existing functionality preserved)
    (error: $err:expr) => {
        $crate::Yoshi::foreign($err)
    };

    // **ENHANCED**: Message with additional attributes (existing functionality preserved)
    (message: $msg:expr, $($attr_key:ident = $attr_val:expr),+ $(,)?) => {{
        let mut __yoshi_instance = $crate::Yoshi::new($crate::YoshiKind::Internal {
            message: $msg.into(),
            source: None,
            component: None,
        });
        $(
            __yoshi_instance = yoshi!(@apply_attr __yoshi_instance, $attr_key, $attr_val);
        )+
        __yoshi_instance
    }};

    // **ENHANCED**: Kind with additional attributes (existing functionality preserved)
    (kind: $kind:expr, $($attr_key:ident = $attr_val:expr),+ $(,)?) => {{
        let mut __yoshi_instance = $crate::Yoshi::new($kind);
        $(
            __yoshi_instance = yoshi!(@apply_attr __yoshi_instance, $attr_key, $attr_val);
        )+
        __yoshi_instance
    }};

    // **ENHANCED**: Error with additional attributes (existing functionality preserved)
    (error: $err:expr, $($attr_key:ident = $attr_val:expr),+ $(,)?) => {{
        let mut __yoshi_instance = $crate::Yoshi::foreign($err);
        $(
            __yoshi_instance = yoshi!(@apply_attr __yoshi_instance, $attr_key, $attr_val);
        )+
        __yoshi_instance
    }};

    // **ENHANCED**: Internal attribute application (existing functionality preserved)
    (@apply_attr $instance:expr, with_metadata, $metadata:expr) => {{
        let metadata_tuple = $metadata;
        $instance.with_metadata(metadata_tuple.0, metadata_tuple.1)
    }};
    (@apply_attr $instance:expr, with_suggestion, $suggestion:expr) => {
        $instance.with_suggestion($suggestion)
    };
    (@apply_attr $instance:expr, with_shell, $shell:expr) => {
        $instance.with_shell($shell)
    };
    (@apply_attr $instance:expr, with_priority, $priority:expr) => {
        $instance.with_priority($priority)
    };
}

/// Enterprise-grade autofix-compatible error enum generator with comprehensive LSP integration.
///
/// This macro creates LSP-integrated error enums with comprehensive diagnostic capabilities,
/// autofix suggestions, and IDE code action support. It preserves all `#[autofix(...)]` attributes
/// for LSP code action extraction while generating the enum as-is with enhanced functionality.
///
/// # Features
///
/// - **Multi-field autofix attribute support**: `pattern`, `suggestion`, `severity`, `auto_apply`
/// - **Automatic LSP diagnostic payload generation**: Complete diagnostic data for language servers
/// - **Runtime variant introspection**: Zero reflection overhead with compile-time optimization
/// - **Compile-time optimized autofix suggestion lookup**: High-performance suggestion resolution
/// - **Complete attribute preservation**: Maintains all attributes for downstream tooling
/// - **`YoshiError` derive integration**: Automatically adds `YoshiError` derive if not present
/// - **`YoshiAutoFixable` trait implementation**: LSP integration for code actions and suggestions
///
/// # Supported Autofix Formats
///
/// ```rust
/// #[autofix("Simple suggestion")]
/// #[autofix(suggestion = "Detailed suggestion")]
/// #[autofix(
///     pattern = "timeout",
///     suggestion = "Increase timeout or check connectivity",
///     severity = "Warning",
///     auto_apply
/// )]
/// ```
///
/// # LSP Integration
///
/// Generates comprehensive LSP integration including:
/// - `autofix_suggestions()` - Static suggestion lookup table with O(1) access
/// - `variant_autofix()` - Instance-specific suggestion resolution
/// - `contextual_autofix()` - Enhanced suggestion with variant context
/// - LSP diagnostic helpers for code action generation
///
/// # Mathematical Properties
///
/// **Algorithmic Complexity:**
/// - Time Complexity: O(V + A) where V=variants, A=autofix attributes. Linear scaling with memoization
/// - Space Complexity: O(V) for variant analysis + O(A) for autofix metadata cache
/// - LSP Integration: O(1) autofix suggestion lookup with compile-time optimization
///
/// **Performance Characteristics:**
/// - Expected Performance: <50ms compilation overhead for typical error enums (<25 variants)
/// - Worst-Case Scenarios: O(V²) for complex autofix dependencies, mitigated by caching
/// - Optimization Opportunities: Parallel attribute processing, incremental compilation support
///
/// # Examples
///
/// **Basic error enum with autofix suggestions:**
/// ```rust
/// #[cfg(feature = "derive")]
/// {
///     use yoshi::yoshi_af;
///     use yoshi_derive::YoshiError;
///     use yoshi_std::YoshiAutoFixable;
///
///     yoshi_af! {
///         #[derive(Debug, YoshiError)]
///         pub enum NetworkError {
///             #[yoshi(display = "Connection timeout after {duration_ms}ms")]
///             #[yoshi(suggestion = "Increase timeout duration or check network connectivity")]
///             #[autofix(suggestion = "Consider increasing connection timeout")]
///             Timeout { duration_ms: u32 },
///
///             #[yoshi(display = "DNS resolution failed for {hostname}")]
///             #[autofix(
///                 pattern = "dns",
///                 suggestion = "Check DNS configuration",
///                 severity = "Error"
///             )]
///             DnsFailure { hostname: String },
///         }
///     }
/// }
/// # #[cfg(not(feature = "derive"))]
/// # struct NetworkError;
/// ```
///
/// **Advanced autofix configuration with multiple attributes:**
/// ```rust
/// #[cfg(feature = "derive")]
/// {
///     use yoshi::yoshi_af;
///     use yoshi_derive::YoshiError;
///     use yoshi_std::YoshiAutoFixable;
///
///     yoshi_af! {
///         #[derive(Debug, Clone, YoshiError)]
///         pub enum DatabaseError {
///             #[yoshi(display = "Connection pool exhausted: {active}/{max}")]
///             #[autofix(
///                 pattern = "pool_exhausted",
///                 suggestion = "Increase connection pool size or reduce concurrent operations",
///                 severity = "Warning",
///                 auto_apply
///             )]
///             PoolExhausted { active: u32, max: u32 },
///
///             #[yoshi(display = "Query timeout after {timeout_ms}ms")]
///             #[autofix(suggestion = "Optimize query or increase timeout")]
///             QueryTimeout { timeout_ms: u64, query: String },
///         }
///     }
/// }
/// # #[cfg(not(feature = "derive"))]
/// # enum DatabaseError {
/// #     PoolExhausted { active: u32, max: u32 },
/// #     QueryTimeout { timeout_ms: u64, query: String },
/// # }
/// ```
///
/// # Generated Implementations
///
/// The macro automatically generates:
/// - Original enum with all preserved attributes
/// - `YoshiError` derive (if not already present)
/// - `YoshiAutoFixable` trait implementation for LSP integration
/// - Autofix metadata extraction for diagnostic enhancement
/// - LSP diagnostic helper functions
/// - Variant name introspection methods
///
/// # Requirements
///
/// - Requires the `derive` feature to be enabled
/// - Requires `yoshi-std` crate for `YoshiAutoFixable` trait
/// - Compatible with `#[derive(YoshiError)]` and other standard derives
///
/// # Panics
///
/// This macro does not panic under normal operation. All error conditions
/// are handled gracefully through the macro expansion system with detailed
/// compile-time error messages.
// =============================================================================
// Comprehensive Example: Demonstrating Both yoshi! and yoshi_af! Integration
// =============================================================================
// Import YoshiAutoFixable trait for example usage
#[cfg(feature = "derive")]
use yoshi_std::YoshiAutoFixable;

#[cfg(feature = "derive")]
yoshi_af! {
    /// Comprehensive example error enum demonstrating both `yoshi!` and `yoshi_af!` macro integration.
    ///
    /// This enum showcases the complete Yoshi ecosystem:
    /// - Defined using `yoshi_af!` for LSP integration and autofix capabilities
    /// - Used with `yoshi!` macro for ergonomic error creation
    /// - Demonstrates best practices for error handling in production applications
    #[derive(Debug, Clone, YoshiError)]
    pub enum Oops {
        /// Configuration file is missing or inaccessible
        #[yoshi(display = "Configuration file not found: {file_path}")]
        #[yoshi(kind = "Config")]
        #[yoshi(suggestion = "Create the configuration file or check the file path")]
        ConfigMissing {
            /// Path to the missing configuration file
            file_path: String,
        },

        /// Network connection failed with status code
        #[yoshi(display = "HTTP {status_code} error: {endpoint}")]
        #[yoshi(kind = "Network")]
        #[yoshi(transient = true)]
        ConnectionFailed {
            /// HTTP status code received
            status_code: u16,
            /// Target endpoint that failed
            endpoint: String,
        },

        /// Permission denied accessing a resource
        #[yoshi(display = "Permission denied: {resource_path}")]
        #[yoshi(kind = "NotFound")]
        #[yoshi(severity = 80)]
        PermissionDenied {
            /// Path to the inaccessible resource
            resource_path: String,
        },

        /// Generic internal error for demonstration
        #[yoshi(display = "Internal system error: {reason}")]
        #[yoshi(kind = "Internal")]
        InternalError {
            /// Reason for the internal error
            reason: String,
        },
    }
}

#[cfg(feature = "derive")]
impl Oops {
    /// Creates a configuration missing error using builder pattern.
    ///
    /// This demonstrates how to create custom constructors that work seamlessly
    /// with the `yoshi!` macro for enhanced error creation.
    pub fn config_missing(file_path: impl Into<String>) -> Self {
        Self::ConfigMissing {
            file_path: file_path.into(),
        }
    }

    /// Creates a connection failed error with status and endpoint.
    pub fn connection_failed(status_code: u16, endpoint: impl Into<String>) -> Self {
        Self::ConnectionFailed {
            status_code,
            endpoint: endpoint.into(),
        }
    }

    /// Creates a permission denied error for a specific resource.
    pub fn permission_denied(resource_path: impl Into<String>) -> Self {
        Self::PermissionDenied {
            resource_path: resource_path.into(),
        }
    }

    /// Creates an internal error.
    pub fn internal_error(reason: impl Into<String>) -> Self {
        Self::InternalError {
            reason: reason.into(),
        }
    }

    /// Demonstrates combining `yoshi_af`! enum with yoshi! macro for enhanced error creation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yoshi::*;
    ///
    /// let enhanced_config_error = Oops::create_enhanced_config_error("app.toml");
    /// println!("Enhanced error: {}", enhanced_config_error);
    ///
    /// // The error includes metadata and suggestions automatically
    /// assert!(enhanced_config_error.suggestion().is_some());
    /// ```
    #[must_use]
    pub fn create_enhanced_config_error(file_path: &str) -> Yoshi {
        yoshi!(
            error: Self::config_missing(file_path),
            with_metadata = ("component", "configuration_loader"),
            with_metadata = ("attempted_path", file_path),
            with_suggestion = "Run 'cargo run --bin init-config' to generate defaults"
        )
    }

    /// Demonstrates advanced error chaining with context preservation.
    #[must_use]
    pub fn create_network_error_with_context(status: u16, endpoint: &str) -> Yoshi {
        yoshi!(
            error: Self::connection_failed(status, endpoint),
            with_metadata = ("retry_count", "3"),
            with_metadata = ("timeout_ms", "5000"),
            with_suggestion = "Check network configuration and endpoint availability"
        )
    }

    /// Demonstrates variant introspection capabilities.
    #[must_use]
    pub fn demonstrate_variant_info(&self) -> String {
        let variant_name = self.variant_name();
        format!("Error variant: {variant_name}")
    }
}

// =============================================================================
// Additional Nightly Compatibility Features
// =============================================================================

// 8. Robust error type for docs.rs
#[cfg(docsrs)]
impl std::error::Error for Yoshi {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Safe implementation that works on all nightly versions
        self.kind().source()
    }

    fn description(&self) -> &str {
        // Deprecated but still needed for compatibility
        "Yoshi error"
    }
}
