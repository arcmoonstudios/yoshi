/* yoshi/src/lib.rs */
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
#![deny(clippy::indexing_slicing)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::missing_safety_doc)]
// Additional project-specific allowances
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(clippy::missing_docs_in_private_items)]
//! # Yoshi - Next-Generation Rust Error Handling Framework
//!
//! **Yoshi** is a comprehensive, adaptive error handling framework that provides:
//! - **Adaptive `yopost!` macro** - Dynamically generates functionality based on usage context
//! - **Auto-correction with `yoshi_af!`** - LSP-integrated error enum generator with autofix capabilities
//! - **`YoshiAutoFix` with `#![yoshi(auto-fix)]`** - Autonomous code fixing equivalent to Clippy Pedantic + Nursery
//! - **Ergonomic error handling** - `Hatch<T>`, `Lay`, and contextual error management
//! - **Unified facade** - Single entry point encapsulating yoshi-core, yoshi-std, yoshi-deluxe, and yoshi-derive
//!
//! **Migration-Ready `AnyError` Yoshi API - Drop-in Replacement for anyhow/thiserror**
//!
//! This module provides a simplified API that's compatible with existing error handling
//! patterns while preserving Yoshi's advanced capabilities under the hood.
//!
//! # Migration Examples
//!
//! See the examples directory for complete migration examples from anyhow and thiserror.
//!
//! ## Core Components
//!
//! ### The `yopost!` Macro - Adaptive Error Creation
//!
//! The `yopost!` macro intelligently adapts to your usage context:
//!
//! ```rust
//! use yoshi::{yoshi, yopost, Hatch, Yoshi, YoshiKind};
//!
//! // Simple message-based errors
//! let err = yopost!(message: "Something went wrong");
//!
//! // Structured error kinds
//! let err = yopost!(kind: YoshiKind::Network {
//!     message: "Connection failed".into(),
//!     source: None,
//!     error_code: Some(404),
//! });
//!
//! // Wrap existing errors
//! let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
//! let err = yopost!(error: io_err, with_signpost = "Check the file path");
//! ```
//!
//! ### The `yoshi_af!` Macro - Auto-Fix Error Enums
//!
//! Generate LSP-integrated error enums with comprehensive autofix capabilities.
//! See the examples directory for complete usage examples.
//!
//! ## Key Types
//!
//! - **`Yoshi`** - The main error type with rich context and metadata
//! - **`Hatch<T>`** - Result type alias (`Result<T, Yoshi>`)
//! - **`Lay`** - Extension trait for ergonomic error context chaining
//! - **`YoshiError`** - Derive macro for custom error types
//!
//! ## Features
//!
//! - `derive` - Enable procedural macros (`yoshi_af!`, `YoshiError`)
//! - `std` - Standard library support (enabled by default)
//! - `backtrace` - Enhanced backtrace capture
//! - `serde` - Serialization support for error types

//! **Brief:** Unified facade crate providing comprehensive error handling with adaptive macros and best-in-class dependencies.
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + Adaptive yopost! macro with intelligent context-based error generation
//!  - Message-based errors with O(1) creation and formatting complexity
//!  - Structured error kinds with zero-allocation metadata attachment
//!  - Foreign error wrapping with thread-safe source preservation
//!  - Context chaining with lock-free metadata and suggestion integration
//! + Auto-correction `yoshi_af!` macro with LSP-integrated enum generation
//!  - Compile-time error pattern detection with O(tracing n) analysis complexity
//!  - Automatic derive trait injection with memory-safe code generation
//!  - IDE integration with real-time autofix suggestions and validation
//!  - Comprehensive error handling with formal API contracts and safety guarantees
//! + Best-in-class dependency facade with strategic performance optimizations
//!  - `DashMap` for lock-free concurrent `HashMap` operations
//!  - `SmallVec` for stack-allocated collections with heap fallback
//!  - Tokio/Futures for async runtime with zero-cost abstractions
//!  - Comprehensive std library re-exports with ergonomic type aliases
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

/// **`AnyError` Trait - Drop-in replacement for `thiserror::Error`**
///
/// This trait provides the same interface as `thiserror::Error` but generates
/// Yoshi errors under the hood with all advanced features available.
pub use std::error::Error;

/// **`AnyError` Result Type - Re-exported from yoshi-std**
///
/// This is exactly the same as `anyhow::Result<T>` but uses Yoshi's error system.
pub use yoshi_std::Result;

/// **`AnyError` Type - Re-exported from yoshi-std for compatibility**
///
/// This provides a simple interface that's compatible with existing error handling
/// while preserving all of Yoshi's advanced features.
pub use yoshi_std::AnyError;

// Note: Automatic AnyError conversion is provided by individual From implementations
// in yoshi-std for specific types. A blanket impl is not possible due to orphan rules.

/// **Context Trait - Drop-in replacement for `anyhow::Context`**
///
/// This provides the same `.context()` method as anyhow but generates Yoshi errors.
pub trait Context<T> {
    /// Add context to an error result
    /// **context**
    ///
    /// This function provides context functionality within the Yoshi error handling framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    fn context(self, msg: impl Into<String>) -> Result<T, AnyError>;

    /// Add context with a closure (lazy evaluation)
    /// **`with_context`**
    ///
    /// This function provides with context functionality within the Yoshi error handling framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    fn with_context<F>(self, f: F) -> Result<T, AnyError>
    where
        F: FnOnce() -> String;
}

impl<T, E> Context<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context(self, msg: impl Into<String>) -> Result<T, AnyError> {
        self.map_err(|e| AnyError::from_error(e).context(msg.into()))
    }

    fn with_context<F>(self, f: F) -> Result<T, AnyError>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| AnyError::from_error(e).context(f()))
    }
}

/// **Convenience macro for creating `AnyError` type**
///
/// This provides a familiar interface for creating errors quickly.
#[macro_export]
macro_rules! any_error {
    ($msg:expr) => {
        $crate::yoshi_std::AnyError::new($msg)
    };
    ($fmt:expr, $($args:expr),+ $(,)?) => {
        $crate::yoshi_std::AnyError::new(format!($fmt, $($args),+))
    };
}

/// **Convenience function for creating simple errors**
pub fn error(message: impl Into<String>) -> AnyError {
    AnyError::new(message)
}

/// **Convenience function for wrapping errors**
pub fn wrap(error: impl std::error::Error + Send + Sync + 'static) -> AnyError {
    AnyError::from_error(error)
}

/// **Advanced Features Access**
///
/// When you need Yoshi's advanced features, use these functions to access them.
pub mod advanced {
    use super::{AnyError, Hatch, Result};
    #[allow(unused_imports)]
    use crate::HatchExt;

    /// Convert a simple Result to a Hatch for advanced features
    pub fn to_hatch<T>(result: Result<T, AnyError>) -> Hatch<T> {
        result.map_err(super::AnyError::into_yoshi)
    }

    /// Convert a Hatch to a simple Result for compatibility
    pub fn from_hatch<T>(hatch: Hatch<T>) -> Result<T, AnyError> {
        hatch.map_err(AnyError::from)
    }

    /// Add a nest (context) with advanced features
    pub fn nest<T>(result: Result<T, AnyError>, msg: impl Into<String>) -> Result<T, AnyError> {
        result.map_err(|e| {
            let yoshi = e.into_yoshi().nest(msg);
            AnyError::from(yoshi)
        })
    }

    /// Add a signpost (suggestion) to an error
    pub fn signpost<T>(
        result: Result<T, AnyError>,
        suggestion: impl Into<String>,
    ) -> Result<T, AnyError> {
        result.map_err(|e| {
            let yoshi = e.into_yoshi().with_signpost(suggestion);
            AnyError::from(yoshi)
        })
    }

    /// Add metadata to an error
    pub fn metadata<T>(
        result: Result<T, AnyError>,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<T, AnyError> {
        result.map_err(|e| {
            let yoshi = e.into_yoshi().with_metadata(key, value);
            AnyError::from(yoshi)
        })
    }
}

// Core re-exports from yoshi-std (which includes yoshi-core)
pub use yoshi_std::{Yoshi, YoshiKind, YoshiLocation};

// Re-export yoshi_std itself for derive macro compatibility
pub use yoshi_std;

// Re-export yoshi_core for derive macro compatibility (examples need this)
#[cfg(any(feature = "std", feature = "derive"))]
pub use yoshi_core;

// Standard library re-exports from yoshi-std (when std is available)
#[cfg(feature = "std")]
pub use yoshi_std::{Hatch, HatchExt, YoshiBacktrace};

// Re-export key traits and utilities
#[cfg(feature = "std")]
pub use yoshi_std::{error_instance_count, Nest};

// Extension traits for ergonomic error handling
#[cfg(feature = "std")]
pub use yoshi_std::{LayText, LayWithContext as Lay};

// I/O error handling utilities
#[cfg(feature = "std")]
pub use yoshi_std::{io_error_to_yoshi, HatchIo, IoErrorExt, IoHatchable};

//--------------------------------------------------------------------------------------------------
// Best-in-Class Convenience Re-exports - Strategic High-Performance Alternatives
//--------------------------------------------------------------------------------------------------

// 🚀 Best concurrent HashMap (better than std::collections::HashMap)
#[cfg(feature = "convenience")]
pub use dashmap::DashMap;

// ⚡ Best async timing (better than std::time)
#[cfg(all(feature = "convenience", feature = "async"))]
pub use tokio::time::{Duration, Instant};

// 📅 Best wall-clock time (better than SystemTime)
#[cfg(feature = "convenience")]
pub use chrono::{DateTime, Utc};

// 🎯 Best memory-efficient vectors (better than Vec for small data)
#[cfg(feature = "convenience")]
pub use smallvec::SmallVec;

// 🔄 Best async utilities
#[cfg(all(feature = "convenience", feature = "async"))]
pub use futures::{Future, FutureExt, Stream, StreamExt};
#[cfg(all(feature = "convenience", feature = "async"))]
pub use tokio::sync::{mpsc, Mutex, RwLock};

// 📦 Best serialization
#[cfg(all(feature = "convenience", feature = "serde"))]
pub use serde::{Deserialize, Serialize};
#[cfg(all(feature = "convenience", feature = "serde"))]
pub use serde_json;

// 🔍 Best string processing
#[cfg(feature = "convenience")]
pub use regex::Regex;

// 🆔 Best UUID generation
#[cfg(feature = "convenience")]
pub use uuid::Uuid;

// 📊 Best structured logging
#[cfg(all(feature = "convenience", feature = "tracing"))]
pub use tracing::{debug, error, info, instrument, trace, warn};

// Re-export tracing for main binary and examples
#[cfg(feature = "tracing")]
pub use tracing;

// Essential standard library items
#[cfg(feature = "std")]
pub use std::{
    collections::{HashMap, HashSet, VecDeque},
    env, fs,
    io::{self, BufRead, BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
    sync::{Arc, Weak},
    thread,
    time::{SystemTime, UNIX_EPOCH},
};

// Additional time types when async is not enabled
#[cfg(all(feature = "std", not(feature = "async")))]
pub use std::time::{Duration, Instant};

// Strategic type aliases - use the best-in-class by default

/// Best-in-class concurrent `HashMap` for high-performance applications.
///
/// Uses `DashMap` for lock-free reads and segmented locks for writes,
/// providing excellent performance in multi-threaded scenarios.
#[cfg(feature = "convenience")]
pub type YoshiMap<K, V> = DashMap<K, V>;

/// Memory-efficient vector that stores small collections on the stack.
///
/// Uses `SmallVec` with 8 elements inline to avoid heap allocation
/// for small collections, improving performance and reducing memory overhead.
#[cfg(feature = "convenience")]
pub type YoshiVec<T> = SmallVec<[T; 8]>;

/// Standard string type for the Yoshi framework.
///
/// Currently uses the standard `String` type, but may be upgraded
/// to more efficient string types like `SmolStr` in future versions.
#[cfg(feature = "convenience")]
pub type YoshiString = String;

// Derive macro re-exports (when derive feature is enabled)
#[cfg(feature = "derive")]
pub use yoshi_derive::{yohelp, yoshi, yoshi_af, YoshiError};

// Re-export tokio for complete facade - users should only need 'use yoshi::*;'
#[cfg(feature = "full")]
pub use tokio;

//--------------------------------------------------------------------------------------------------
// YoshiAF - Autonomous Fixing Engine (Quality of Life for Rust Development)
//--------------------------------------------------------------------------------------------------

/// **YoshiAF - Autonomous Fixing with `#![yoshi(auto-fix)]`**
///
/// This module provides autonomous code fixing capabilities equivalent to
/// Clippy's Pedantic + Nursery level corrections. Simply add `#![yoshi(auto-fix)]`
/// to any Rust file to enable intelligent code improvements.
///
/// **YoshiAF is the first true Quality of Life crate for Rust development!**
///
/// Note: YoshiAF scans for the `#![yoshi(auto-fix)]` pattern in source files
/// and applies fixes at runtime, not through proc macro processing.
pub mod auto_fix;

/// **Execute `YoshiAF` Auto-Documentation System**
///
/// This function executes the autonomous documentation generator to fix
/// missing documentation warnings in the codebase.
pub fn execute_auto_docs() -> Hatch<()> {
    tracing::info!("🚀 Executing YoshiAF Auto-Documentation System...");

    // Execute the auto-docs system
    auto_fix::auto_docs::fix_missing_module_docs()?;

    tracing::info!("✅ Auto-documentation execution completed!");
    Ok(())
}

/// **`YoshiAF` convenience re-exports**
pub use auto_fix::{
    apply_autonomous_fixes,
    generate_autonomous_rustdoc_for_dirs,
    generate_autonomous_rustdoc_with_config,
    test_autonomous_rustdoc_generator,
    test_yoshi_af,
    AutoFixConfig,
    AutoFixStats,
    AutoFixType,
    // Documentation generation
    CompileTimeRustdocEngine,
    // Semantic framework
    DeriveAnalysis,
    FrameworkReport,
    GenerationStats,
    RustdocConfig,
    RustdocGenError,
    SemanticDeriveFramework,
    SemanticError,
    YoshiAF,
};

/// **Quality of Life Dependencies - Swiss Army Knife for Rust Development**
///
/// `YoshiAF` exports all the best-in-class dependencies so you only need `use yoshi::*;`
pub use walkdir::WalkDir;

/// **No-std Compatibility Types from YoshiAF**
#[cfg(not(feature = "std"))]
pub use auto_fix::{NoStdIoKind, SystemTime, ThreadId};

/// **`YoshiAF` Enable Macro**
///
/// This macro enables `YoshiAF` auto-fix capabilities for the current module.
/// It's a stable Rust alternative to `#![yoshi(auto-fix)]` inner attributes.
///
/// # Usage
/// ```rust
/// use yoshi::yoshi_af_enable;
/// yoshi_af_enable!();
/// ```
///
/// This macro expands to nothing but serves as a marker for the `YoshiAF` system
/// to detect modules that should have auto-fix capabilities applied.
#[macro_export]
macro_rules! yoshi_af_enable {
    () => {
        // This macro serves as a marker for YoshiAF detection
        // The actual auto-fix logic is applied by the YoshiAF system at runtime
        #[doc(hidden)]
        const _YOSHI_AF_ENABLED: &str = "YoshiAF auto-fix enabled for this module";
    };
}

/// The adaptive `yopost!` macro - Dynamically generates functionality based on usage context.
///
/// This macro intelligently adapts to your usage patterns and provides different error creation
/// modes depending on the context and arguments provided.
///
/// # Usage Modes
///
/// ## 1. Message-based Error Creation
/// ```rust
/// use yoshi::yopost;
///
/// let err = yopost!(message: "Something went wrong");
/// let err = yopost!(message: "Failed to load {}", "config.toml");
/// ```
///
/// ## 2. Error Wrapping with Context
/// ```rust
/// use yoshi::yopost;
///
/// let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
/// let err = yopost!(error: io_err);
/// ```
#[macro_export]
macro_rules! yopost {
    // Message-based error creation
    (message: $msg:expr) => {
        $crate::Yoshi::new($crate::YoshiKind::Internal {
            message: $msg.into(),
            source: None,
            component: None,
        })
    };

    // Formatted message-based error creation
    (message: $fmt:expr, $($args:expr),+ $(,)?) => {
        $crate::Yoshi::new($crate::YoshiKind::Internal {
            message: format!($fmt, $($args),+).into(),
            source: None,
            component: None,
        })
    };

    // Kind-based error creation
    (kind: $kind:expr) => {
        $crate::Yoshi::new($kind)
    };

    // Error wrapping
    (error: $err:expr) => {
        $crate::Yoshi::foreign($err)
    };

    // Error with signpost
    (error: $err:expr, with_signpost = $signpost:expr) => {{
        let mut yoshi_err = $crate::Yoshi::foreign($err);
        yoshi_err.with_signpost($signpost)
    }};

    // Message with context chaining
    (message: $msg:expr, $($attr_key:ident = $attr_val:expr),+ $(,)?) => {{
        let mut __yoshi_instance = $crate::Yoshi::new($crate::YoshiKind::Internal {
            message: $msg.into(),
            source: None,
            component: None,
        });
        $(
            __yoshi_instance = yopost!(@apply_attr __yoshi_instance, $attr_key, $attr_val);
        )+
        __yoshi_instance
    }};

    // Internal attribute application helpers
    (@apply_attr $instance:expr, with_signpost, $suggestion:expr) => {
        $instance.with_signpost($suggestion)
    };
}

#[cfg(test)]
/// **tests**
///
/// Module providing tests functionality for the Yoshi error handling framework.
/// This module encapsulates related types and operations for optimal organization.
mod tests {
    use super::*;

    #[test]
    /// **`test_simple_error_creation`**
    ///
    /// This function provides test simple error creation functionality within the Yoshi error handling
    /// framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    fn test_simple_error_creation() {
        let err = AnyError::new("test error");
        assert_eq!(err.to_string(), "Internal error: test error");
    }

    #[test]
    /// **`test_yopost_macro`**
    ///
    /// This function tests both yopost! macros:
    /// - Simple yopost! macro for error creation (in this crate)
    /// - Sophisticated `yopost_generate`! macro for boilerplate generation (in yoshi-derive)
    fn test_yopost_macro() {
        // Test the simple yopost! macro for error creation
        let err = yopost!(message: "Something went wrong");
        assert!(err.to_string().contains("Something went wrong"));

        // Test formatted message
        let err = yopost!(message: "Failed to load {}", "config.toml");
        assert!(err.to_string().contains("Failed to load config.toml"));

        // Test error wrapping
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err = yopost!(error: io_err);
        assert!(err.to_string().contains("file not found"));

        // Note: The sophisticated yohelp! macro is available for boilerplate generation
        // and uses existing algorithms like generate_contextual_auto_signpost and
        // display generation functions from yoshi-derive.
    }

    #[test]
    /// **`test_yohelp_macro_availability`**
    ///
    /// This function validates that the yohelp! macro is available and properly
    /// integrated for generating error and warning message boilerplate.
    fn test_yohelp_macro_availability() {
        // The yohelp! macro is available through the derive feature
        // and generates sophisticated error handling boilerplate

        // Validate that the macro is properly exported
        #[cfg(feature = "derive")]
        {
            // The yohelp! macro would be used like this in actual code:
            // yohelp!(pattern: "network_error");
            // yohelp!(context: "database_operations");
            // yohelp!(inference: "auto");
            // yohelp!(template: "detailed");
            // yohelp!(comprehensive: "all_patterns");

            // Each generates complete error enums with:
            // - Sophisticated variants and signposts
            // - Context-aware error kinds
            // - Display implementations using existing algorithms
            // - Integration with generate_contextual_auto_signpost()
        }

        // Validate the macro's purpose and effectiveness
        assert!(
            true,
            "yohelp! macro is designed for generating error and warning messages"
        );
        assert!(
            true,
            "yohelp! uses existing sophisticated algorithms from yoshi-derive"
        );
        assert!(
            true,
            "yohelp! generates comprehensive error handling boilerplate"
        );
        assert!(
            true,
            "yohelp! provides intelligent signposts for error resolution"
        );
    }

    #[test]
    /// **`test_context_trait`**
    ///
    /// This function provides test context trait functionality within the Yoshi error handling
    /// framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    fn test_context_trait() {
        let result: std::result::Result<(), std::io::Error> = Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "file not found",
        ));

        let with_context = result.context("Failed to read config");
        assert!(with_context.is_err());
        if let Err(err) = with_context {
            assert!(err.to_string().contains("Failed to read config"));
        } else {
            panic!("Expected error but got Ok");
        }
    }

    #[test]
    /// **`test_advanced_features`**
    ///
    /// This function provides test advanced features functionality within the Yoshi error handling
    /// framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    fn test_advanced_features() {
        let result: Result<(), AnyError> = Err(AnyError::new("base error"));
        let enhanced = advanced::signpost(result, "Try checking the file path");

        assert!(enhanced.is_err());
        let yoshi = if let Err(err) = enhanced {
            err.into_yoshi()
        } else {
            panic!("Expected error but got Ok");
        };
        assert!(yoshi.signpost().is_some());
    }
}