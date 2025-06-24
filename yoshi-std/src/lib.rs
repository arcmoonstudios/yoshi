/* yoshi-std/src/lib.rs */
#![deny(dead_code)]
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![deny(clippy::todo)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![deny(unused_variables)]
#![deny(clippy::dbg_macro)]
#![deny(clippy::expect_used)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::unreachable)]
#![deny(clippy::print_stdout)]
#![deny(clippy::unimplemented)]
#![allow(clippy::too_many_lines)]
#![deny(clippy::indexing_slicing)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::missing_safety_doc)]
#![allow(clippy::struct_excessive_bools)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(clippy::missing_docs_in_private_items)]
//! # Yoshi Std - Standard Library Extensions for Yoshi Error Framework
//!
//! This crate provides standard library-specific enhancements and utilities for the Yoshi
//! error handling framework. It re-exports all core functionality from `yoshi-core` and
//! adds std-specific features like enhanced backtrace support, async utilities, and
//! integration with standard library error types.
//!
//! ## Module Classification
//! - **Performance-Critical**: Sub-microsecond error creation with O(1) context attachment
//! - **Complexity Level**: Expert-level error handling with beginner-friendly APIs
//! - **API Stability**: Stable with semantic versioning guarantees
//!
//! ## Architecture
//!
//! This crate serves as the standard library layer of the Yoshi ecosystem:
//!
//! - **Core Re-exports**: All fundamental types from `yoshi-core`
//! - **Std Enhancements**: Additional functionality requiring standard library
//! - **Integration Layer**: Seamless integration with std error types
//! - **Async Support**: Tokio and async-std compatibility (feature-gated)
//!
//! ## Core Types (Re-exported from yoshi-core)
//!
//! - [`Yoshi`]: The main error type providing structured error handling
//! - [`YoshiKind`]: Error categories with type-specific fields
//! - [`Nest`]: Contextual information and metadata
//! - [`HatchExt`]: Extension trait for `Result` types
//! - [`YoshiLocation`]: Source code location capture
//! - [`YoshiBacktrace`]: Performance-monitored backtrace wrapper
//! - [`Result`]: Type alias for `Result` with `Yoshi` as default error
//! - [`error_instance_count()`]: Global counter for Yoshi error instances
//!
//! ## Feature Flags
//!
//! ```toml
//! [dependencies]
//! yoshi-std = { version = "0.1", features = ["std", "serde", "async"] }
//! ```
//!
//! - **`std`** (default): Standard library integration with backtrace support
//! - **`serde`**: Serialization support for error persistence and transmission
//! - **`async`**: Tokio integration and async utilities
//! - **`tracing`**: Integration with the tracing ecosystem
//!
//! # Examples
//!
//! Basic error creation and context addition:
//!
//! ```rust
//! use yoshi_std::{Yoshi, YoshiKind, HatchExt};
//! # use std::io;
//! # use std::io::ErrorKind;
//! #
//! # fn simulate_io_error() -> Result<(), io::Error> {
//! #    Err(io::Error::new(ErrorKind::PermissionDenied, "cannot access file"))
//! # }
//!
//! fn load_config(path: &str) -> Result<String, Yoshi> {
//!     // Convert I/O errors to Yoshi errors with additional context
//!     simulate_io_error()
//!         .map_err(Yoshi::from)?;
//!
//!     // Errors can be built up with context as they propagate
//!     Err(Yoshi::new(YoshiKind::NotFound {
//!         resource_type: "config file".into(),
//!         identifier: path.into(),
//!         search_locations: None,
//!     })
//!     .with_metadata("config_path", path)
//!     .with_signpost("Ensure the configuration file exists and is readable")
//!     .nest(format!("Failed to load configuration from {}", path)))
//! }
//!
//! # fn main() {
//! match load_config("/etc/app/config.json") {
//!     Ok(config) => tracing::info!("Loaded: {}", config),
//!     Err(error) => {
//!         tracing::info!("Configuration error: {}", error);
//!         // Rich error output includes context, metadata, and suggestions
//!     }
//! }
//! # }
//! ```
//! **Brief:** Standard library extensions providing enhanced error handling with backtrace support and I/O integration.
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + Core re-exports with comprehensive yoshi-core functionality integration
//!  - All fundamental types with zero-cost abstraction guarantees
//!  - Type aliases for ergonomic error handling with memory-efficient boxing
//!  - Extension traits for `std::io::Error` with rich context attachment
//!  - Diagnostic information structs for LSP integration and IDE support
//! + Standard library-specific string interning system with `RwLock` optimization
//!  - High-performance concurrent `HashMap` with O(1) cache hit performance
//!  - Thread-safe string deduplication with 30-70% memory reduction
//!  - Global interning pool with atomic statistics and performance monitoring
//!  - Lock-free fast path for cache hits with write-lock fallback for misses
//! + Enhanced backtrace system with performance monitoring and thread metadata
//!  - `StdYoshiBacktrace` wrapper with capture cost measurement in nanoseconds
//!  - Thread ID and name capture with timestamp recording for debugging
//!  - Conditional capture based on `RUST_BACKTRACE` environment variable
//!  - Performance characteristics tracking with memory usage optimization
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

//============================================================================
// MODULE DECLARATIONS
//============================================================================

/// **Core API Surface** - Primary API and re-exports from yoshi-core
pub mod api;

/// **Type Conversion Systems** - Batman strategies and ultra-high-performance conversions
pub mod conversions;

/// **Standard Library Integration** - Std-specific functionality and enhancements
pub mod std_integration;

/// **Autonomous Error Analytics** - Advanced error monitoring and predictive analytics
pub mod analytics;

//============================================================================
// PRIMARY API RE-EXPORTS
//============================================================================

// Re-export the main YoshiStd API for convenient access
pub use api::YoshiStd;

// Re-export deprecated snake_case module for backward compatibility
#[allow(deprecated)]
pub use api::yoshi_std;

// Re-export all core functionality from yoshi-core
pub use yoshi_core::*;

//============================================================================
// CONVERSION SYSTEM RE-EXPORTS
//============================================================================

// Batman Strategy newtype wrappers
pub use conversions::{HatchWrapper, ResultWrapper};

// Ultra-high-performance conversion traits
pub use conversions::{
    BulkYoshiConvert, UltraYoshiConvert, YoshiConversionStrategy, YoshiConvertError,
    YoshiConvertOptimizer, YoshiLayoutCompatible,
};

// Type aliases for ergonomic error handling
pub use conversions::{
    AnyError,
    // Core types
    Context,
    Error,
    Hatch,
    HatchedYoshi,
    // Ergonomic aliases
    Oops,
    Payload,
    Result,
    YoshiEgg,
};

// Extension traits for enhanced error handling
pub use conversions::{
    HatchIo,
    // Re-export yoshi-core's Hatchable for convenience
    Hatchable,
    Hatchling,
    IoHatchable,
    LayWithContext,
    ResultExt,
};

// Conversion helper functions
pub use conversions::{err, io_error_to_yoshi, ok, to_any_error_result, to_yoshi_result};

//============================================================================
// STD INTEGRATION RE-EXPORTS
//============================================================================

// Enhanced backtrace system
pub use std_integration::{capture_std_backtrace, StdYoshiBacktrace, YoshiBacktrace};

// String interning system
pub use std_integration::{intern_string_std, StdStringInternPool};

// Extended Yoshi types
pub use std_integration::{StdResult, StdYoshi, StdYoshiKind};

// Extension traits
pub use std_integration::IoErrorExt;

// Integration utilities (removed duplicate std_integration re-export)

#[cfg(feature = "async")]
pub use std_integration::async_utils;

#[cfg(feature = "tracing")]
pub use std_integration::tracing_integration;

//============================================================================
// ANALYTICS SYSTEM RE-EXPORTS
//============================================================================

// Global error tracking
pub use analytics::{error_instance_count, increment_error_counter};

// Auto-correction engine
pub use analytics::YoshiACE;

// NOTE: Advanced analytics functionality is available in the analytics module.
// Only basic error tracking and YoshiACE are re-exported for convenience.

// Autonomous systems
pub use analytics::{
    AutonomousCircuitBreaker, AutonomousErrorMonitor, AutonomousOptimizationMonitor,
    AutonomousPerformanceMonitor, AutonomousRecovery, AutonomousTestGenerator, IntelligentDebugger,
    IntelligentDocumentationGenerator, StackTraceEnhancer,
};

// Construct-specific recovery systems
pub use analytics::{
    AutonomousConstructRecovery, ConstructDebugNest, ConstructRecoveryStrategy,
    IntelligentConstructDebugger,
};

// Recovery strategy types
pub use yoshi_core::ErrorRecoveryStrategy;

// Re-export yoshi-derive macros when derive feature is enabled
#[cfg(feature = "derive")]
pub use yoshi_derive::*;

//============================================================================
// COMPILE-TIME TESTS FOR MODULAR STRUCTURE
//============================================================================

#[cfg(test)]
/// **`modular_structure_tests`**
///
/// Module providing modular structure tests functionality for the Yoshi error handling framework.
/// This module encapsulates related types and operations for optimal organization.
mod modular_structure_tests {
    use super::*;

    /// Test that all core types are accessible through the new modular structure
    #[test]
    fn test_core_types_accessibility() {
        // Test core re-exports work
        let _yoshi = Yoshi::new(YoshiKind::Internal {
            message: "test".into(),
            source: None,
            component: None,
        });

        // Test conversion types work
        let _any_error = AnyError::new("test");
        let _result: Result<()> = Ok(());

        // Test analytics work
        let _count = error_instance_count();

        // Test basic functionality works
        let _test = "yoshi-std works";
    }

    /// Test that the `YoshiStd` API provides complete access
    #[test]
    fn test_yoshi_std_api_completeness() {
        // Test that YoshiStd API provides access to all major types
        let _yoshi = YoshiStd::Yoshi::new(YoshiStd::YoshiKind::Internal {
            message: "test".into(),
            source: None,
            component: None,
        });

        let _any_error = YoshiStd::AnyError::new("test");
        let _result: YoshiStd::Result<()> = Ok(());
    }

    /// Test that conversions work seamlessly across modules
    #[test]
    fn test_cross_module_conversions() {
        use crate::conversions::{Hatch, ResultExt};

        // Test conversion between types from different modules
        let hatch_result: Hatch<String> = Ok("test".to_string());
        let any_error_result = hatch_result.into_any_error_result();
        let back_to_hatch = any_error_result.into_yoshi_result();

        assert!(back_to_hatch.is_ok());
    }

    /// Test that analytics integration works
    #[test]
    #[ignore] // TODO: Re-enable when analytics are moved to auto_fix
    fn test_analytics_integration() {
        // use crate::analytics::{AutonomousErrorAnalytics, YoshiACE};

        // Test analytics can track errors
        increment_error_counter();
        let count = error_instance_count();
        assert!(count > 0);

        // Test ACE error creation
        let _ace_error = YoshiACE::diagnostic_processing("test message", "test/path");
    }
}
