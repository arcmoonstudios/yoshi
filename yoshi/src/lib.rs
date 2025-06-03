/* yoshi/yoshi/src/lib.rs */
#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
#![allow(clippy::use_self)]
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

#[cfg(all(feature = "unstable-metrics", target_arch = "x86_64"))]
pub use yoshi_std::simd_optimization;

#[cfg(feature = "unstable-metrics")]
pub use yoshi_std::cross_process_metrics;

// Re-export from yoshi-derive if the 'derive' feature is enabled
#[cfg(feature = "derive")]
#[doc(hidden)] // Typically hidden from main docs as it's a procedural macro crate
pub use yoshi_derive::*;

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
    // Message-based error creation
    (message: $msg:expr) => {
        $crate::Yoshi::new($crate::YoshiKind::Internal {
            message: $msg.into(),
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

    // Message with additional attributes
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

    // Kind with additional attributes
    (kind: $kind:expr, $($attr_key:ident = $attr_val:expr),+ $(,)?) => {{
        let mut __yoshi_instance = $crate::Yoshi::new($kind);
        $(
            __yoshi_instance = yoshi!(@apply_attr __yoshi_instance, $attr_key, $attr_val);
        )+
        __yoshi_instance
    }};

    // Error with additional attributes
    (error: $err:expr, $($attr_key:ident = $attr_val:expr),+ $(,)?) => {{
        let mut __yoshi_instance = $crate::Yoshi::foreign($err);
        $(
            __yoshi_instance = yoshi!(@apply_attr __yoshi_instance, $attr_key, $attr_val);
        )+
        __yoshi_instance
    }};
      // Internal attribute application
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
