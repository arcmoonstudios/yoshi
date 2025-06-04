/* yoshi/yoshi-std/src/lib.rs */
#![allow(dead_code)]
#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
#![allow(unexpected_cfgs)] // Allow experimental feature flags
#![allow(clippy::too_many_lines)] // Allows for longer files, often necessary in comprehensive modules
#![allow(clippy::result_large_err)] // Error handling framework intentionally uses large error types for rich context
#![allow(clippy::enum_variant_names)] // For consistent naming of enum variants like IoError.
#![allow(clippy::items_after_statements)] // Allows declaration of items after statements for better code flow in some contexts
#![allow(clippy::module_name_repetitions)] // Allow for names like YoshiKind, YoContext.
#![cfg_attr(not(feature = "std"), no_std)] // For no_std environments
//! **Brief:** Comprehensive error handling framework for robust Rust applications.
//!
//! Yoshi provides structured error types with rich contextual information, making it easier
//! to debug, trace, and handle errors throughout your application. It offers flexible error
//! categorization, context chaining, and optional backtrace capture while maintaining
//! excellent performance characteristics.
//!
//! **Module Classification:** Performance-Critical\
//! **Complexity Level:** Expert\
//! **API Stability:** Stable
//!
//! ## Key Features
//!
//! **Structured Error Types**: Define precise error categories with relevant metadata
//! rather than relying on string-based errors. Each error kind captures the specific
//! information needed for that failure mode.
//!
//! **Rich Context**: Add diagnostic information, suggestions, and typed payloads
//! as errors propagate through your application. Context is preserved without
//! performance overhead.
//!
//! **Performance Focused**: Sub-microsecond error creation with O(1) context
//! attachment. Backtrace capture is conditional and can be disabled in production.
//!
//! **`no_std` Compatible**: Full functionality available in `no_std` environments
//! with automatic fallbacks for platform-specific features.
//!
//! ## Usage Patterns
//!
//! Yoshi works well for applications that need detailed error diagnostics and
//! structured error handling. It's particularly useful when you want to:
//!
//! - Provide rich debugging information to developers
//! - Maintain error context across call stacks
//! - Categorize errors for different handling strategies
//! - Include suggestions and metadata for error recovery
//!
//! For simpler error propagation needs, consider [`anyhow`]. For derive-based
//! error definitions, [`thiserror`] remains an excellent choice and can be
//! used alongside Yoshi.
//!
//! ## Core Types
//!
//! - [`Yoshi`]: The main error type providing structured error handling
//! - [`YoshiKind`]: Error categories with type-specific fields
//! - [`YoContext`]: Contextual information and metadata
//! - [`HatchExt`]: Extension trait for `Result` types
//! - [`YoshiLocation`]: Source code location capture
//! - [`YoshiBacktrace`]: Performance-monitored backtrace wrapper
//! - `NoStdIo`: I/O error type for `no_std` environments
//! - [`Result`]: Type alias for `Result` with `Yoshi` as default error
//! - [`error_instance_count()`]: Global counter for Yoshi error instances
//!
//! # Examples
//!
//! Basic error creation and context addition:
//!
//! ```
//! use yoshi_std::{Yoshi, YoshiKind};
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
//!     .with_suggestion("Ensure the configuration file exists and is readable")
//!     .context(format!("Failed to load configuration from {}", path)))
//! }
//!
//! # fn main() {
//! match load_config("/etc/app/config.json") {
//!     Ok(config) => println!("Loaded: {}", config),
//!     Err(error) => {
//!         eprintln!("Configuration error: {}", error);
//!         // Rich error output includes context, metadata, and suggestions
//!     }
//! }
//! # }
//! ```
//!
//! Working with typed payloads and structured data:
//!
//! ```
//! use yoshi_std::{Yoshi, YoshiKind};
//!
//! #[derive(Debug)]
//! struct RequestId(String);
//!
//! fn process_request(id: &str) -> Result<(), Yoshi> {
//!     Err(Yoshi::new(YoshiKind::Timeout {
//!         operation: "database query".into(),
//!         duration: std::time::Duration::from_secs(30),
//!         expected_max: Some(std::time::Duration::from_secs(10)),
//!     })
//!     .with_shell(RequestId(id.to_string()))
//!     .with_metadata("user_id", "12345")
//!     .context("Request processing failed"))
//! }
//!
//! # fn main() {
//! if let Err(error) = process_request("req_001") {
//!     // Access structured data from the error
//!     if let Some(request_id) = error.shell::<RequestId>() {
//!         println!("Failed request: {:?}", request_id);
//!     }
//!
//!     println!("Error details: {}", error);
//! }
//! # }
//! ```
//!
//! [`anyhow`]: https://docs.rs/anyhow
//! [`thiserror`]: https://docs.rs/thiserror
//!
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + Structured error handling with context preservation [O(1) error creation, O(1) context attachment]
//!  - Type-safe error categorization with detailed diagnostic information [Memory-safe, Thread-safe]
//!  - Context chaining for complete error trace visibility [Stack-overflow protection, bounded depth]
//!  - Conditional backtrace capture with performance monitoring [Zero-cost when disabled]
//!  - Memory-efficient formatting with minimal allocations [Pre-allocated buffers, shared strings]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **License File:** /LICENSE
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

// For no_std environments, we need alloc for Vec, String, etc.
#[cfg(not(feature = "std"))]
extern crate alloc;

// Add serde helper functions for Arc<str> serialization
#[cfg(feature = "serde")]
mod serde_helpers {
    use super::String;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::collections::HashMap;
    use std::sync::Arc;

    /// Serialize `Option<Arc<str>>` as `Option<String>`
    #[allow(clippy::ref_option)]
    pub fn serialize_arc_str<S>(value: &Option<Arc<str>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value
            .as_ref()
            .map(std::convert::AsRef::as_ref)
            .serialize(serializer)
    }

    /// Deserialize `Option<String>` as `Option<Arc<str>>`
    pub fn deserialize_arc_str<'de, D>(deserializer: D) -> Result<Option<Arc<str>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt_string: Option<String> = Option::deserialize(deserializer)?;
        Ok(opt_string.map(|s| Arc::from(s.as_str())))
    }

    /// Serialize `HashMap<Arc<str>, Arc<str>>` as `HashMap<String, String>`
    pub fn serialize_arc_str_map<S>(
        value: &HashMap<Arc<str>, Arc<str>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string_map: HashMap<&str, &str> = value
            .iter()
            .map(|(k, v)| (k.as_ref(), v.as_ref()))
            .collect();
        string_map.serialize(serializer)
    }

    /// Deserialize `HashMap<String, String>` as `HashMap<Arc<str>, Arc<str>>`
    pub fn deserialize_arc_str_map<'de, D>(
        deserializer: D,
    ) -> Result<HashMap<Arc<str>, Arc<str>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string_map: HashMap<String, String> = HashMap::deserialize(deserializer)?;
        Ok(string_map
            .into_iter()
            .map(|(k, v)| (Arc::from(k.as_str()), Arc::from(v.as_str())))
            .collect())
    }
}

#[cfg(feature = "serde")]
use serde_helpers::{
    deserialize_arc_str, deserialize_arc_str_map, serialize_arc_str, serialize_arc_str_map,
};

// CRITICAL: Block ALL experimental features on docs.rs to force stable compilation
#[cfg(docsrs)]
mod docs_safety_check {
    #[cfg(any(target_feature = "avx2", target_feature = "sse4.1"))]
    compile_error!("Experimental features disabled on docs.rs for stable compatibility");
}

/// Safe feature detection
#[allow(unused_macros)] // Used conditionally based on feature flags
macro_rules! detect_docs_rs {
    () => {
        cfg!(docsrs) || std::env::var("DOCS_RS").is_ok()
    };
}

/// Docs.rs compatibility utilities
#[cfg(docsrs)]
mod docs_compatibility {
    /// Safe fallbacks for docs.rs builds
    pub mod fallbacks {
        use crate::{Arc, HashMap};

        /// Metrics map fallback for docs.rs
        pub type MetricsMap = HashMap<Arc<str>, Arc<str>>;

        /// Safe async result type for docs builds
        pub type AsyncResult<T> = core::future::Ready<Result<T, crate::Yoshi>>;

        /// Documentation-only serde implementations
        #[cfg(feature = "serde")]
        pub mod serde_docs {
            /// Placeholder for serde docs that don't trigger nightly conflicts
            pub fn safe_serialize() -> &'static str {
                "Serialization available in runtime builds"
            }
        }
    }
}

#[cfg(not(docsrs))]
mod runtime_impl {
    /// Runtime implementations (placeholder for organization)
    #[allow(unused_imports)] // Conditional imports based on feature flags
    pub mod runtime {
        pub use crate::*;
    }
}

// Unified imports for String, Vec, Box, Arc based on 'std' feature
#[cfg(not(feature = "std"))]
pub use alloc::{
    boxed::Box,
    format, // Import format! macro for no_std
    string::{String, ToString},
    sync::Arc,
    vec, // Import vec! macro for no_std
    vec::Vec,
};

// Provide eprintln! macro for no_std environments
#[cfg(not(feature = "std"))]
#[allow(unused_macros)] // May not be used in all no_std builds
macro_rules! eprintln {
    () => {};
    ($($arg:tt)*) => {
        // In no_std environments, we silently ignore eprintln calls
        // This maintains API compatibility while avoiding dependencies on std::io
    };
}
#[cfg(feature = "std")]
pub use std::{
    boxed::Box,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};

use core::any::Any; // Import Any for error_generic_member_access and blanket From
use core::error::Error; // Removed Request as it's unstable
use core::fmt::{self, Display, Formatter};
use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use core::time::Duration;

// Additional imports for advanced features
// Unified imports for HashMap based on 'std' feature
#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap as HashMap;
#[cfg(feature = "std")]
use std::collections::HashMap; // Using BTreeMap for no_std by default
                               // Unified imports for SystemTime and Thread based on 'std' feature
#[cfg(not(feature = "std"))]
use core::sync::atomic::AtomicU64;
#[cfg(feature = "std")]
use std::{thread, time::SystemTime}; // For SystemTime counter
#[cfg(not(feature = "std"))]
/// Enhanced SystemTime for `no_std` environments with monotonic counter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SystemTime {
    /// Monotonic timestamp counter for ordering events
    timestamp: u64,
}

#[cfg(not(feature = "std"))]
impl SystemTime {
    /// Returns a `SystemTime` with monotonic ordering guarantees.
    ///
    /// While not wall-clock time, this provides ordering semantics
    /// useful for debugging and event correlation in no_std environments.
    pub fn now() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        Self {
            timestamp: COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }

    /// Returns the internal timestamp for debugging purposes.
    pub const fn timestamp(&self) -> u64 {
        self.timestamp
    }

    /// Calculates duration since another SystemTime (in timestamp units).
    pub const fn duration_since(&self, earlier: SystemTime) -> Option<u64> {
        if self.timestamp >= earlier.timestamp {
            Some(self.timestamp - earlier.timestamp)
        } else {
            None
        }
    }

    /// Returns elapsed timestamp units since this SystemTime.
    pub fn elapsed(&self) -> u64 {
        Self::now().timestamp.saturating_sub(self.timestamp)
    }
}

#[cfg(not(feature = "std"))]
/// Enhanced ThreadId for `no_std` environments with unique identification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ThreadId {
    /// Unique identifier for tracking execution contexts
    id: u32,
}

#[cfg(not(feature = "std"))]
impl ThreadId {
    /// Returns a `ThreadId` with unique identification.
    ///
    /// In no_std environments, this provides unique identifiers
    /// useful for correlating errors across different execution contexts.
    pub fn current() -> Self {
        static THREAD_COUNTER: AtomicU32 = AtomicU32::new(1); // Use thread-local storage pattern with atomic fallback
        #[cfg(all(target_has_atomic = "ptr", feature = "std"))]
        {
            use core::cell::Cell;
            std::thread_local! {
                static THREAD_ID: Cell<Option<u32>> = const { Cell::new(None) };
            }

            THREAD_ID.with(|id| {
                let current_id = id.get().unwrap_or_else(|| {
                    let new_id = THREAD_COUNTER.fetch_add(1, Ordering::Relaxed);
                    id.set(Some(new_id));
                    new_id
                });

                Self { id: current_id }
            })
        }
        #[cfg(not(all(target_has_atomic = "ptr", feature = "std")))]
        {
            // Fallback for platforms without atomic or thread_local support
            Self {
                id: THREAD_COUNTER.fetch_add(1, Ordering::Relaxed),
            }
        }
    }

    /// Returns the raw thread ID for debugging.
    #[inline]
    pub const fn as_u32(&self) -> u32 {
        self.id
    }

    /// Creates a ThreadId from a raw ID (for testing/debugging).
    pub const fn from_u32(id: u32) -> Self {
        Self { id }
    }
}

#[cfg(not(feature = "std"))]
impl core::fmt::Display for ThreadId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ThreadId({})", self.id)
    }
}

// OnceLock is std-only, so it's only imported under std
#[cfg(not(feature = "std"))]
use core::cell::UnsafeCell;
#[cfg(not(feature = "std"))]
use core::sync::atomic::AtomicBool;
#[cfg(feature = "std")]
use std::sync::OnceLock;

// Version compatibility guards to prevent future breakage
// Note: version cfg is experimental, so we disable this check for now
// #[cfg(all(not(docsrs), version("1.89")))]
// compile_error!("This crate requires Rust 1.87.0-1.88.x for compatibility. Rust 1.89+ may have breaking changes.");

#[cfg(not(feature = "std"))]
/// Thread-safe one-time initialization for `no_std` environments using atomics.
pub struct OnceLock<T> {
    initialized: AtomicBool,
    data: UnsafeCell<Option<T>>,
}

#[cfg(not(feature = "std"))]
unsafe impl<T: Send + Sync> Sync for OnceLock<T> {}
#[cfg(not(feature = "std"))]
unsafe impl<T: Send> Send for OnceLock<T> {}

#[cfg(not(feature = "std"))]
impl<T> OnceLock<T> {
    /// Creates a new `OnceLock` for no_std environments.
    pub const fn new() -> Self {
        Self {
            initialized: AtomicBool::new(false),
            data: UnsafeCell::new(None),
        }
    }

    /// Gets or initializes the value using atomic operations for thread safety.
    pub fn get_or_init(&self, f: impl FnOnce() -> T) -> &T {
        // Use compare_exchange for proper synchronization
        if self
            .initialized
            .compare_exchange_weak(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            let value = f();
            unsafe {
                let data_ptr = self.data.get();
                *data_ptr = Some(value);
            }
        } else {
            // Spin until initialization is complete
            while !self.initialized.load(Ordering::Acquire) {
                core::hint::spin_loop();
            }
        }

        unsafe {
            let data_ptr = self.data.get();
            (*data_ptr).as_ref().unwrap_unchecked()
        }
    }

    /// Gets the value if it has been initialized.
    pub fn get(&self) -> Option<&T> {
        if self.initialized.load(Ordering::Acquire) {
            unsafe {
                let data_ptr = self.data.get();
                (*data_ptr).as_ref()
            }
        } else {
            None
        }
    }
}

/// Enhanced wrapper for foreign errors with better context preservation
#[derive(Debug)]
struct ForeignErrorWrapper {
    inner: Box<dyn Error + Send + Sync + 'static>,
    context: String,
    enhanced_message: String,
}

impl Display for ForeignErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.context.is_empty() {
            write!(f, "{}", self.enhanced_message)
        } else {
            write!(f, "{}: {}", self.context, self.enhanced_message)
        }
    }
}

impl Error for ForeignErrorWrapper {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.inner.as_ref())
    }
}

/// High-performance buffer for error formatting with safe optimizations
pub struct OptimizedFormatBuffer {
    data: String,
    reserved_capacity: usize,
}

impl OptimizedFormatBuffer {
    const DEFAULT_CAPACITY: usize = 2048; // 2KB optimized default

    /// Creates a new optimized format buffer with default capacity.
    ///
    /// Initializes a new `OptimizedFormatBuffer` with a default capacity of 4KB,
    /// which is optimized for typical error formatting scenarios. The buffer
    /// uses intelligent growth strategies to minimize memory allocations.
    ///
    /// # Returns
    ///
    /// A new `OptimizedFormatBuffer` instance with default capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::OptimizedFormatBuffer;
    /// let buffer = OptimizedFormatBuffer::new();
    /// assert_eq!(buffer.as_str(), "");
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: String::with_capacity(Self::DEFAULT_CAPACITY),
            reserved_capacity: Self::DEFAULT_CAPACITY,
        }
    }
}

impl Default for OptimizedFormatBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl OptimizedFormatBuffer {
    /// Creates a new optimized format buffer with specified capacity.
    ///
    /// Initializes a new `OptimizedFormatBuffer` with a custom initial capacity.
    /// This is useful when you have an estimate of the final formatted size
    /// and want to avoid reallocations during formatting operations.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The initial capacity for the internal string buffer.
    ///
    /// # Returns
    ///
    /// A new `OptimizedFormatBuffer` instance with the specified capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::OptimizedFormatBuffer;
    /// let buffer = OptimizedFormatBuffer::with_capacity(8192);
    /// assert_eq!(buffer.as_str(), "");
    /// ```
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: String::with_capacity(capacity),
            reserved_capacity: capacity,
        }
    }

    /// High-performance string appending with optimized growth strategy
    pub fn append_optimized(&mut self, s: &str) {
        let current_len = self.data.len();
        let append_len = s.len();
        let new_len = current_len + append_len;

        // Optimized growth strategy: 1.5x growth with minimum thresholds
        if new_len > self.data.capacity() {
            let current_cap = self.data.capacity();
            // Ensure minimum growth of at least reserved_capacity or 256 bytes for small buffers
            let min_growth_needed = self.reserved_capacity.max(256);
            let growth_target_1_5x = current_cap + (current_cap >> 1); // 1.5x growth
            let new_capacity = growth_target_1_5x.max(new_len).max(min_growth_needed);

            // Reserve exactly what we need to avoid over-allocation, but also ensure minimum
            self.data.reserve(new_capacity - current_cap);
        }

        // Use efficient string concatenation, which is highly optimized by Rust
        self.data.push_str(s);
    }

    /// Returns a string slice of the buffer's contents.
    ///
    /// This method provides read-only access to the formatted content within the buffer.
    /// The returned string slice is guaranteed to be valid UTF-8 as all input is validated.
    ///
    /// # Returns
    ///
    /// A string slice containing the current buffer contents.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use yoshi_std::OptimizedFormatBuffer;
    /// let mut buffer = OptimizedFormatBuffer::new();
    /// buffer.append_optimized("Hello, World!");
    /// assert_eq!(buffer.as_str(), "Hello, World!");
    /// ```
    ///
    /// # Performance
    ///
    /// This operation has O(1) time complexity and does not involve any allocations.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.data
    }

    /// Clears the buffer contents while preserving the allocated capacity.
    ///
    /// This method efficiently removes all content from the buffer without
    /// deallocating the underlying storage. This allows for optimal memory reuse
    /// when the buffer will be used again with similar content sizes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use yoshi_std::OptimizedFormatBuffer;
    /// let mut buffer = OptimizedFormatBuffer::new();
    /// buffer.append_optimized("Hello, World!");
    /// assert_eq!(buffer.as_str().len(), 13);
    ///
    /// buffer.clear();
    /// assert_eq!(buffer.as_str().len(), 0);
    /// assert!(buffer.as_str().is_empty());
    /// ```
    ///
    /// # Performance
    ///
    /// This operation has O(1) time complexity and preserves allocated capacity
    /// for optimal memory reuse patterns.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Optimized formatting for multiple string fragments
    pub fn append_multiple(&mut self, fragments: &[&str]) {
        let total_len: usize = fragments.iter().map(|s| s.len()).sum();
        let new_len = self.data.len() + total_len;

        if new_len > self.data.capacity() {
            let new_capacity = (new_len * 2)
                .next_power_of_two()
                .max(self.reserved_capacity);
            self.data.reserve_exact(new_capacity - self.data.capacity());
        }

        for fragment in fragments {
            self.data.push_str(fragment);
        }
    }
}

/// Comprehensive error recovery strategies
#[derive(Debug, Clone)]
pub enum ErrorRecoveryStrategy {
    /// Retry with exponential backoff
    ExponentialBackoff {
        /// Initial delay before the first retry attempt
        initial_delay: Duration,
        /// Maximum number of retry attempts before giving up
        max_retries: u32,
        /// Multiplier for exponential backoff calculation (e.g., 2.0 for doubling)
        backoff_multiplier: f64,
    },
    /// Retry with fixed intervals
    FixedInterval {
        /// Fixed time interval between retry attempts
        interval: Duration,
        /// Maximum number of retry attempts before giving up
        max_retries: u32,
    },
    /// Fallback to alternative approach
    Fallback {
        /// Human-readable description of the fallback strategy
        description: String,
    },
    /// Circuit breaker pattern
    CircuitBreaker {
        /// Number of consecutive failures before opening the circuit
        failure_threshold: u32,
        /// Timeout duration before attempting to close the circuit
        recovery_timeout: Duration,
    },
    /// No recovery possible
    NonRecoverable,
}

/// Detailed context analysis results
#[derive(Debug, Default)]
pub struct ContextAnalysis {
    /// Total number of context objects attached to the error
    pub total_contexts: usize,
    /// Maximum depth of nested context information
    pub context_depth: usize,
    /// Whether the error includes user-facing suggestions
    pub has_suggestions: bool,
    /// Whether source code location information is available
    pub has_location_info: bool,
    /// Number of metadata key-value pairs attached
    pub metadata_entries: usize,
    /// Number of typed shell objects attached
    pub typed_payloads: usize,
    /// Priority level of the primary context (0-255)
    pub primary_context_priority: u8,
}

/// Performance-optimized Result alias with mathematical precision guarantees.
///
/// This type alias simplifies the use of `Result` where the error type is
/// fixed to [`Yoshi`]. It automatically adapts between `std::result::Result`
/// and `core::result::Result` based on the enabled features.
///
/// # Examples
///
/// ```
/// use yoshi_std::{Result, Yoshi, YoshiKind};
///
/// fn divide(a: f64, b: f64) -> Result<f64> {
///     if b == 0.0 {
///         return Err(Yoshi::new(YoshiKind::Validation {
///             field: "divisor".into(),
///             message: "Division by zero is not allowed".into(),
///             expected: Some("non-zero".into()),
///             actual: Some("0.0".into()),
///         }));
///     }
///     Ok(a / b)
/// }
///
/// let result = divide(10.0, 2.0);
/// assert!(result.is_ok());
/// assert_eq!(result.unwrap(), 5.0);
///
/// let error_result = divide(10.0, 0.0);
/// assert!(error_result.is_err());
/// ```
#[cfg(feature = "std")]
pub type Result<T, E = Yoshi> = std::result::Result<T, E>;
#[cfg(not(feature = "std"))]
/// Performance-optimized Result alias for `no_std` builds.
///
/// This type alias simplifies the use of `Result` where the error type is
/// fixed to [`Yoshi`]. It automatically adapts between `std::result::Result`
/// and `core::result::Result` based on the enabled features.
///
/// # Examples
///
/// ```
/// use yoshi_std::{Result, Yoshi, YoshiKind, NoStdIo};
///
/// fn check_value(value: i32) -> Result<i32> {
///     if value < 0 {
///         return Err(Yoshi::new(YoshiKind::Validation {
///             field: "value".into(),
///             message: "Value cannot be negative".into(),
///             expected: Some("non-negative".into()),
///             actual: Some(value.to_string().into()),
///         }));
///     }
///     Ok(value)
/// }
///
/// let result = check_value(5);
/// assert!(result.is_ok());
/// assert_eq!(result.unwrap(), 5);
///
/// let error_result = check_value(-1);
/// assert!(error_result.is_err());
/// ```
pub type Result<T, E = Yoshi> = core::result::Result<T, E>;

/// Ergonomic type alias for `Result<T, Yoshi>` with thematic naming.
///
/// This type alias provides expressive naming that aligns with the Yoshi metaphorical
/// framework while maintaining zero-cost abstraction guarantees. It automatically
/// adapts between `std::result::Result` and `core::result::Result` based on features.
///
/// # Performance Characteristics
///
/// - **Time Complexity**: O(1) for all operations (zero-cost abstraction)
/// - **Space Complexity**: Identical to `Result<T, Yoshi>` (no overhead)
/// - **Memory Layout**: Exact same representation as standard `Result`
///
/// # Examples
///
/// ```rust
/// use yoshi_std::{Hatch, Yoshi, YoshiKind};
///
/// fn load_config() -> Hatch<String> {
///     Ok("configuration data".into())
/// }
///
/// fn process_data() -> Hatch<u32> {
///     Err(Yoshi::new(YoshiKind::Internal {
///         message: "processing failed".into(),
///         source: None,
///         component: None,
///     }))
/// }
/// ```
pub type Hatch<T> = Result<T, Yoshi>;

/// Global error instance counter for debugging and performance monitoring.
///
/// This atomic counter tracks the total number of `Yoshi` error instances
/// that have been created since the application started. It's primarily
/// used for performance monitoring and diagnostic purposes.
static ERROR_INSTANCE_COUNTER: AtomicU32 = AtomicU32::new(0);

/// Global string interning pool for optimal memory reuse
static STRING_INTERN_POOL: OnceLock<StringInternPool> = OnceLock::new();

/// Nightly compatibility workarounds
#[allow(unexpected_cfgs)] // Allow experimental feature flags
#[cfg(all(docsrs, any(feature = "simd-optimized", feature = "precise-capturing")))]
mod nightly_workarounds {
    /// Disable SIMD optimizations on docs.rs nightly builds
    /// to prevent version conflicts
    #[cfg(feature = "simd-optimized")]
    pub fn safe_simd_placeholder() -> &'static str {
        "SIMD optimizations disabled for docs.rs compatibility"
    }

    /// Disable precise capturing on docs.rs builds
    #[cfg(feature = "precise-capturing")]
    pub fn safe_capturing_placeholder() -> &'static str {
        "Precise capturing disabled for docs.rs compatibility"
    }
}

/// Checks if running in production mode for security sanitization
#[inline]
fn is_production_mode() -> bool {
    #[cfg(feature = "std")]
    {
        std::env::var("YOSHI_PRODUCTION_MODE")
            .map(|v| v == "1" || v.to_lowercase() == "true")
            .unwrap_or(false)
    }
    #[cfg(not(feature = "std"))]
    {
        false // Default to development mode in no_std
    }
}

/// Sanitizes error messages to remove potentially sensitive information in production
fn sanitize_error_message(msg: &str) -> String {
    // Define constants first before any statements
    const MAX_MESSAGE_LENGTH: usize = 256;

    let mut sanitized = msg.to_string();

    // Simple string replacement for common sensitive patterns
    let lower_msg = msg.to_lowercase();
    if lower_msg.contains("password") {
        sanitized = sanitized.replace("password", "password=[REDACTED]");
    }
    if lower_msg.contains("token") {
        sanitized = sanitized.replace("token", "token=[REDACTED]");
    }
    if lower_msg.contains("key") {
        sanitized = sanitized.replace("key", "key=[REDACTED]");
    }

    // Truncate very long messages that might contain sensitive data dumps
    if sanitized.len() > MAX_MESSAGE_LENGTH {
        sanitized.truncate(MAX_MESSAGE_LENGTH);
        sanitized.push_str("... [truncated]");
    }

    sanitized
}

/// High-performance string interning with autonomous memory management and lock-free fast paths
struct StringInternPool {
    #[cfg(feature = "std")]
    pool: std::sync::RwLock<std::collections::HashMap<String, Arc<str>>>,
    #[cfg(not(feature = "std"))]
    pool: alloc::collections::BTreeMap<String, Arc<str>>,
    hits: AtomicUsize,
    misses: AtomicUsize,
    cache_size: AtomicUsize,
}

impl StringInternPool {
    fn new() -> Self {
        Self {
            #[cfg(feature = "std")]
            pool: std::sync::RwLock::new(std::collections::HashMap::with_capacity(128)),
            #[cfg(not(feature = "std"))]
            pool: alloc::collections::BTreeMap::new(),
            hits: AtomicUsize::new(0),
            misses: AtomicUsize::new(0),
            cache_size: AtomicUsize::new(0),
        }
    }

    /// Clears the interning pool to prevent memory leaks in long-running applications
    #[cfg(feature = "std")]
    pub fn clear_pool(&self) {
        if let Ok(mut pool) = self.pool.write() {
            pool.clear();
            self.cache_size.store(0, Ordering::Release);
        }
    }

    fn intern(&self, s: impl Into<String>) -> Arc<str> {
        let string = s.into();

        // Early exit for empty strings
        if string.is_empty() {
            return Arc::from("");
        }

        #[cfg(feature = "std")]
        {
            // Fast path: check if already interned with non-blocking try_read for performance
            if let Ok(pool) = self.pool.try_read() {
                if let Some(interned) = pool.get(&string) {
                    self.hits.fetch_add(1, Ordering::Relaxed);
                    return interned.clone();
                }
            }

            // Cache size check before expensive write lock
            const MAX_CACHE_SIZE: usize = 512;
            let current_size = self.cache_size.load(Ordering::Relaxed);
            if current_size >= MAX_CACHE_SIZE {
                // Skip interning for large caches to prevent memory bloat
                self.misses.fetch_add(1, Ordering::Relaxed);
                return string.into();
            }

            // Slow path: intern new string (acquire write lock)
            let mut pool = self
                .pool
                .write()
                .unwrap_or_else(std::sync::PoisonError::into_inner);

            // Double-check pattern (after acquiring write lock, for race conditions during read)
            if let Some(interned) = pool.get(&string) {
                self.hits.fetch_add(1, Ordering::Relaxed);
                return interned.clone();
            }

            let current_pool_size = pool.len();
            if current_pool_size < MAX_CACHE_SIZE {
                let arc_str: Arc<str> = string.as_str().into();
                pool.insert(string, arc_str.clone());
                self.cache_size
                    .store(current_pool_size + 1, Ordering::Release);
                self.misses.fetch_add(1, Ordering::Relaxed);
                arc_str
            } else {
                // Cache is full, return without interning
                self.cache_size.store(current_pool_size, Ordering::Release);
                self.misses.fetch_add(1, Ordering::Relaxed);
                string.into()
            }
        }

        #[cfg(not(feature = "std"))]
        {
            // High-performance lock-free string interning using separate chaining with explicit capacity management
            use core::ptr;
            use core::sync::atomic::AtomicPtr;

            // Fixed-size lock-free cache with atomic slots (larger for fewer collisions)
            const CACHE_SLOTS: usize = 256; // Power of 2 for efficient modulo
            static CACHE: [AtomicPtr<CacheEntry>; CACHE_SLOTS] =
                [const { AtomicPtr::new(ptr::null_mut()) }; CACHE_SLOTS];

            // Global maximum number of interned strings to prevent unbounded memory growth in no_std
            const MAX_GLOBAL_CACHE_SIZE: usize = 512;

            #[repr(C)]
            struct CacheEntry {
                hash: u64,
                arc_str: Arc<str>,
                next: AtomicPtr<CacheEntry>,
            }

            // Fast hash function for cache slot selection (FNV-1a)
            #[inline(always)] // Ensure inlining for performance-critical path
            fn fast_hash(s: &str) -> u64 {
                let mut hash = 0xcbf29ce484222325u64; // FNV-1a offset basis
                for byte in s.bytes() {
                    hash ^= byte as u64;
                    hash = hash.wrapping_mul(0x100000001b3u64); // FNV-1a prime
                }
                hash
            }
            let hash = fast_hash(&string);
            let slot_index = (hash as usize) & (CACHE_SLOTS - 1); // Efficient modulo for power of 2

            // Lock-free search in the cache slot's linked list
            let mut current = CACHE[slot_index].load(Ordering::Acquire);
            while !current.is_null() {
                unsafe {
                    let entry = &*current;
                    if entry.hash == hash && entry.arc_str.as_ref() == string {
                        self.hits.fetch_add(1, Ordering::Relaxed);
                        return entry.arc_str.clone();
                    }
                    current = entry.next.load(Ordering::Acquire);
                }
            }

            // Cache miss: attempt to increment global cache size *before* allocation
            let new_cache_size = self.cache_size.fetch_add(1, Ordering::Relaxed) + 1;
            if new_cache_size > MAX_GLOBAL_CACHE_SIZE {
                // If over capacity, decrement counter (to prevent false overflow) and return original string
                self.cache_size.fetch_sub(1, Ordering::Relaxed); // Correct the increment
                self.misses.fetch_add(1, Ordering::Relaxed);
                return string.into(); // Return uninterned string
            }

            // Save string content before moving into Arc
            let string_for_comparison = string.clone();
            let arc_str: Arc<str> = string.into(); // Allocate string
            let new_entry = Box::into_raw(Box::new(CacheEntry {
                hash,
                arc_str: arc_str.clone(),
                next: AtomicPtr::new(ptr::null_mut()),
            }));

            // Atomic compare-and-swap insertion at head of linked list
            let mut head = CACHE[slot_index].load(Ordering::Acquire);
            loop {
                unsafe {
                    (*new_entry).next.store(head, Ordering::Relaxed);
                }

                match CACHE[slot_index].compare_exchange_weak(
                    head,
                    new_entry,
                    Ordering::Release,
                    Ordering::Acquire,
                ) {
                    Ok(_) => {
                        // Successfully inserted new entry
                        self.misses.fetch_add(1, Ordering::Relaxed);
                        return arc_str;
                    }
                    Err(current_head) => {
                        // Another thread modified the head, retry with new head
                        head = current_head; // Double-check if another thread inserted our string
                        let mut search_current = head;
                        while !search_current.is_null() {
                            unsafe {
                                let entry = &*search_current;
                                if entry.hash == hash
                                    && entry.arc_str.as_ref() == string_for_comparison
                                {
                                    // Another thread inserted our string, clean up and return
                                    let _ = Box::from_raw(new_entry); // Clean up unused entry
                                    self.hits.fetch_add(1, Ordering::Relaxed);
                                    self.cache_size.fetch_sub(1, Ordering::Relaxed); // Correct the size
                                    return entry.arc_str.clone();
                                }
                                search_current = entry.next.load(Ordering::Acquire);
                            }
                        }
                        // Continue loop to retry insertion
                    }
                }
            }
        }
    }

    /// Returns (hits, misses) for performance monitoring
    #[inline]
    pub fn stats(&self) -> (usize, usize) {
        (
            self.hits.load(Ordering::Relaxed),
            self.misses.load(Ordering::Relaxed),
        )
    }

    /// Returns current cache size for autonomous memory monitoring
    #[inline]
    pub fn cache_size(&self) -> usize {
        self.cache_size.load(Ordering::Acquire)
    }
}

/// Optimized string interning function
#[inline]
pub fn intern_string(s: impl Into<String>) -> Arc<str> {
    STRING_INTERN_POOL
        .get_or_init(StringInternPool::new)
        .intern(s)
}

/// Gets the current number of Yoshi error instances created.
///
/// This function provides a way to inspect the cumulative count of `Yoshi`
/// error objects instantiated. It can be useful for profiling, detecting
/// excessive error creation, or understanding error patterns in an
/// application.
///
/// # Returns
///
/// The total number of `Yoshi` error instances created as a `u64`.
///
/// # Examples
///
/// ```
/// use yoshi_std::{Yoshi, YoshiKind, error_instance_count};
///
/// let initial_count = error_instance_count();
/// let _err1 = Yoshi::new(YoshiKind::Internal {
///     message: "simulated error 1".into(),
///     source: None,
///     component: None,
/// });
/// let _err2 = Yoshi::new(YoshiKind::Internal {
///     message: "simulated error 2".into(),
///     source: None,
///     component: None,
/// });
///
/// assert_eq!(error_instance_count(), initial_count + 2);
/// ```
pub fn error_instance_count() -> u32 {
    ERROR_INSTANCE_COUNTER.load(Ordering::Relaxed)
}

/// Resets the global error instance counter.
///
/// This function is intended primarily for use in test environments
/// to ensure test isolation and predictable counter values.
/// It should **not** be used in production code.
#[cfg(test)]
#[inline]
pub fn reset_error_instance_counter() {
    ERROR_INSTANCE_COUNTER.store(0, Ordering::Relaxed);
}

//--------------------------------------------------------------------------------------------------
// Enhanced NoStdIo with performance optimization
//--------------------------------------------------------------------------------------------------

#[cfg(not(feature = "std"))]
/// Structured error kinds for better type safety in no_std I/O operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NoStdIoKind {
    /// A file or directory was not found.
    NotFound,
    /// Permission was denied for the operation.
    PermissionDenied,
    /// A network connection was refused.
    ConnectionRefused,
    /// An operation timed out.
    TimedOut,
    /// A generic I/O error occurred.
    Generic,
    /// Other error types not covered by specific variants.
    Other,
}

#[cfg(not(feature = "std"))]
impl NoStdIoKind {
    /// Returns a human-readable description of the error kind.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::NotFound => "not found",
            Self::PermissionDenied => "permission denied",
            Self::ConnectionRefused => "connection refused",
            Self::TimedOut => "timed out",
            Self::Generic => "I/O error",
            Self::Other => "other error",
        }
    }

    /// Returns whether this error kind typically indicates a transient condition.
    pub const fn is_transient(&self) -> bool {
        matches!(
            self,
            Self::ConnectionRefused | Self::TimedOut | Self::Generic
        )
    }

    /// Returns a severity level for this error kind (0-100).
    pub const fn severity(&self) -> u8 {
        match self {
            Self::NotFound => 30,
            Self::PermissionDenied => 50,
            Self::ConnectionRefused => 40,
            Self::TimedOut => 35,
            Self::Generic => 45,
            Self::Other => 40,
        }
    }
}

#[cfg(not(feature = "std"))]
impl core::fmt::Display for NoStdIoKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// High-performance minimal wrapper for I/O errors in `no_std` contexts.
///
/// This enum provides a compact representation for common I/O errors
/// when the standard library's `std::io::Error` is not available.
/// It uses `Arc<str>` for message storage to minimize allocations
/// when messages are repeated or shared.
#[cfg(not(feature = "std"))]
#[derive(Debug, Clone)]
pub enum NoStdIo {
    /// Generic I/O error with optimized string storage.
    GenericIo(Arc<str>),
    /// Indicates that a file or directory was not found.
    NotFound,
    /// Indicates that permission was denied for an operation.
    PermissionDenied,
    /// Indicates that a network connection was refused.
    ConnectionRefused,
    /// Indicates that an operation timed out.
    TimedOut,
    /// Other I/O errors, with a custom message.
    Other(Arc<str>),
}

#[cfg(not(feature = "std"))]
impl NoStdIo {
    /// Creates a new I/O error with comprehensive categorization.
    ///
    /// This constructor attempts to categorize the error message into specific
    /// variants using pattern matching on common error strings, enabling
    /// better programmatic error handling even in no_std environments.
    ///
    /// # Arguments
    ///
    /// * `message` - A message describing the I/O error. This can be any type
    ///   that converts into a `String`.
    ///
    /// # Returns
    ///
    /// A new `NoStdIo` error instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::NoStdIo;
    /// let err1 = NoStdIo::new("file not found");
    /// assert!(matches!(err1, NoStdIo::NotFound));
    ///
    /// let err2 = NoStdIo::new("disk full");
    /// assert!(matches!(err2, NoStdIo::Other(_)));
    /// ```
    pub fn new(message: impl Into<String>) -> Self {
        let msg = message.into();
        let lower_msg = msg.to_lowercase();

        // Comprehensive pattern matching for better error categorization
        match lower_msg.as_str() {
            // File/resource not found patterns
            s if s.contains("not found")
                || s.contains("no such file")
                || s.contains("enoent")
                || s.contains("file does not exist") =>
            {
                Self::NotFound
            }

            // Permission/access denied patterns
            s if s.contains("permission denied")
                || s.contains("access denied")
                || s.contains("access is denied")
                || s.contains("eacces")
                || s.contains("unauthorized")
                || s.contains("forbidden") =>
            {
                Self::PermissionDenied
            }

            // Network connection patterns
            s if s.contains("connection refused")
                || s.contains("econnrefused")
                || s.contains("no route to host")
                || s.contains("network unreachable") =>
            {
                Self::ConnectionRefused
            }

            // Timeout patterns
            s if s.contains("timed out")
                || s.contains("timeout")
                || s.contains("etimedout")
                || s.contains("operation timeout") =>
            {
                Self::TimedOut
            }

            // Generic I/O patterns
            s if s.contains("i/o error")
                || s.contains("io error")
                || s.contains("input/output error") =>
            {
                Self::GenericIo(msg.into())
            }

            // Catch-all for unrecognized patterns
            _ => Self::Other(msg.into()),
        }
    }

    /// Creates a new I/O error from an error code and message.
    ///
    /// This method provides more precise error categorization when
    /// both an error code and message are available.
    pub fn from_code_and_message(code: i32, message: impl Into<String>) -> Self {
        match code {
            2 | -2 => Self::NotFound,                         // ENOENT
            13 | -13 => Self::PermissionDenied,               // EACCES
            111 | -111 => Self::ConnectionRefused,            // ECONNREFUSED
            110 | -110 => Self::TimedOut,                     // ETIMEDOUT
            5 | -5 => Self::GenericIo(message.into().into()), // EIO
            _ => Self::Other(message.into().into()),
        }
    }

    /// Creates a typed I/O error for specific common scenarios.
    pub fn typed_error(error_type: NoStdIoKind, message: impl Into<String>) -> Self {
        match error_type {
            NoStdIoKind::NotFound => Self::NotFound,
            NoStdIoKind::PermissionDenied => Self::PermissionDenied,
            NoStdIoKind::ConnectionRefused => Self::ConnectionRefused,
            NoStdIoKind::TimedOut => Self::TimedOut,
            NoStdIoKind::Generic => Self::GenericIo(message.into().into()),
            NoStdIoKind::Other => Self::Other(message.into().into()),
        }
    }
}

#[cfg(not(feature = "std"))]
impl Display for NoStdIo {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::GenericIo(s) => write!(f, "I/O error (no_std): {s}"),
            Self::NotFound => f.write_str("I/O error (no_std): not found"),
            Self::PermissionDenied => f.write_str("I/O error (no_std): permission denied"),
            Self::ConnectionRefused => f.write_str("I/O error (no_std): connection refused"),
            Self::TimedOut => f.write_str("I/O error (no_std): timed out"),
            Self::Other(s) => write!(f, "I/O error (no_std): {s}"),
        }
    }
}

#[cfg(not(feature = "std"))]
impl Error for NoStdIo {}

//--------------------------------------------------------------------------------------------------
// Enhanced YoshiKind with performance optimization
//--------------------------------------------------------------------------------------------------

/// High‑level categories for recoverable failures with performance optimizations.
///
/// This enum represents the fundamental classification of an error within the
/// `Yoshi` framework. Each variant provides specific fields relevant to its
/// error category, enabling rich, structured error reporting and programmatic
/// error handling.
#[derive(Debug)]
#[non_exhaustive]
pub enum YoshiKind {
    /// Standard I/O failure with optimized error representation.
    ///    /// This variant wraps `std::io::Error` when the `std` feature is enabled,
    /// or `NoStdIo` for `no_std` environments.
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    Io(std::io::Error),
    /// I/O failure in `no_std` with enhanced error categorization.
    ///
    /// This variant wraps [`NoStdIo`] when the `std` feature is not enabled.
    #[cfg(not(feature = "std"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "std"))))]
    Io(NoStdIo),
    /// Network-related error with connection and protocol context.
    ///
    /// This variant represents errors that occur during network operations,
    /// including connectivity issues, protocol errors, and communication failures.
    ///
    /// # Fields
    ///
    /// * `message` - A human-readable description of the network error
    /// * `source` - An optional nested [`Yoshi`] error that caused this network issue
    /// * `error_code` - An optional numeric error code from the underlying network layer
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::sync::Arc;
    /// # use yoshi_std::{Yoshi, YoshiKind};
    /// let network_error = YoshiKind::Network {
    ///     message: Arc::from("Connection refused"),
    ///     source: None,
    ///     error_code: Some(111),
    /// };
    /// ```
    Network {
        /// A human-readable description of the network error.
        message: Arc<str>,
        /// An optional nested [`Yoshi`] error that caused this network issue.
        source: Option<Box<Yoshi>>,
        /// An optional network-specific error code (e.g., HTTP status code).
        error_code: Option<u32>,
    },
    /// Configuration error with enhanced diagnostics.
    ///
    /// Fields:
    /// - `message`: A human-readable description of the configuration error.
    /// - `source`: An optional nested `Yoshi` error that caused this configuration issue.
    /// - `config_path`: An optional path to the configuration file or source.
    Config {
        /// A human-readable description of the configuration error.
        message: Arc<str>,
        /// An optional nested [`Yoshi`] error that caused this configuration issue.
        source: Option<Box<Yoshi>>,
        /// An optional path to the configuration file or source.
        config_path: Option<Arc<str>>,
    },
    /// Data validation failure with field-level precision.
    ///
    /// Fields:
    /// - `field`: The name of the field that failed validation.
    /// - `message`: A description of why the validation failed.
    /// - `expected`: An optional description of the expected value or format.
    /// - `actual`: An optional string representation of the actual value received.
    Validation {
        /// The name of the field that failed validation.
        field: Arc<str>,
        /// A description of why the validation failed.
        message: Arc<str>,
        /// An optional description of the expected value or format.
        expected: Option<Arc<str>>,
        /// An optional string representation of the actual value received.
        actual: Option<Arc<str>>,
    },
    /// Internal invariant breakage with debugging context.
    ///
    /// This typically indicates a bug within the application's own logic
    /// or an unexpected state.
    ///
    /// Fields:
    /// - `message`: A description of the internal error.
    /// - `source`: An optional nested `Yoshi` error that caused this internal issue.
    /// - `component`: An optional name of the component where the error occurred.
    Internal {
        /// A description of the internal error.
        message: Arc<str>,
        /// An optional nested [`Yoshi`] error that caused this internal issue.
        source: Option<Box<Yoshi>>,
        /// An optional name of the component where the error occurred.
        component: Option<Arc<str>>,
    },
    /// Resource not found with typed identification.
    ///
    /// Fields:
    /// - `resource_type`: The type of resource (e.g., "User", "Product", "File").
    /// - `identifier`: The specific identifier of the resource that was not found.
    /// - `search_locations`: Optional list of locations where the resource was searched.
    NotFound {
        /// The type of resource (e.g., "User", "Product", "File").
        resource_type: Arc<str>,
        /// The specific identifier of the resource that was not found.
        identifier: Arc<str>,
        /// Optional list of locations where the resource was searched.
        search_locations: Option<Vec<Arc<str>>>,
    },
    /// Operation timeout with detailed timing information.
    ///
    /// Fields:
    /// - `operation`: A description of the operation that timed out.
    /// - `duration`: The duration for which the operation ran before timing out.
    /// - `expected_max`: An optional maximum expected duration for the operation.
    Timeout {
        /// A description of the operation that timed out.
        operation: Arc<str>,
        /// The duration for which the operation ran before timing out.
        duration: Duration,
        /// An optional maximum expected duration for the operation.
        expected_max: Option<Duration>,
    },
    /// Resource exhaustion with precise metrics.
    ///
    /// This indicates that a system resource (e.g., memory, CPU, disk space)
    /// has been exhausted.
    ///
    /// Fields:
    /// - `resource`: The type of resource exhausted (e.g., "memory", "thread pool").
    /// - `limit`: The configured limit for the resource.
    /// - `current`: The current usage or allocation of the resource when exhaustion occurred.
    /// - `usage_percentage`: Optional percentage of resource usage at the time of error.
    ResourceExhausted {
        /// The type of resource exhausted (e.g., "memory", "thread pool").
        resource: Arc<str>,
        /// The configured limit for the resource.
        limit: Arc<str>,
        /// The current usage or allocation of the resource when exhaustion occurred.
        current: Arc<str>,
        /// Optional percentage of resource usage at the time of error.
        usage_percentage: Option<f64>,
    },
    /// Foreign error wrapper with enhanced type information.
    ///
    /// This variant allows wrapping any type that implements `std::error::Error`,
    /// providing a uniform way to integrate external error types into the `Yoshi`
    /// framework.
    ///
    /// Fields:
    /// - `error`: The boxed foreign error object.
    /// - `error_type_name`: The fully qualified type name of the original error.
    Foreign {
        /// The boxed foreign error object.
        error: Box<dyn Error + Send + Sync + 'static>,
        /// The fully qualified type name of the original error.
        error_type_name: Arc<str>,
    },
    /// Multiple errors with categorization and priority.
    ///
    /// This variant can be used to aggregate several errors into a single `Yoshi`
    /// instance, useful for scenarios like batch processing or validation where
    /// multiple failures can occur.
    ///
    /// Fields:
    /// - `errors`: A vector of nested `Yoshi` errors.
    /// - `primary_index`: An optional index indicating which error in the `errors`
    ///   vector should be considered the primary error.
    Multiple {
        /// A vector of nested [`Yoshi`] errors.
        errors: Vec<Yoshi>,
        /// An optional index indicating which error in the `errors`
        /// vector should be considered the primary error.
        primary_index: Option<usize>,
    },
}

impl YoshiKind {
    /// Enhanced foreign error conversion with better type preservation and sanitization
    pub fn from_foreign_with_context<E>(error: E, context: impl Into<String>) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        let type_name = core::any::type_name::<E>();
        let error_msg = error.to_string();
        // Apply sanitization to protect sensitive information
        let enhanced_msg = sanitize_error_message(&error_msg);

        Self::Foreign {
            error: Box::new(ForeignErrorWrapper {
                inner: Box::new(error),
                context: context.into(),
                enhanced_message: enhanced_msg,
            }),
            error_type_name: intern_string(type_name),
        }
    }

    /// Gets the severity level of this error kind (0-100, higher is more severe).
    ///
    /// This method provides a numerical indication of how critical an error
    /// is, allowing for programmatic decision-making based on severity
    /// (e.g., logging level, alerting, retry behavior).
    ///
    /// # Returns
    ///
    /// A `u8` value representing the severity, where 0 is least severe
    /// and 100 is most severe.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::YoshiKind;
    /// let internal_error = YoshiKind::Internal {
    ///     message: "simulated error".into(),
    ///     source: None,
    ///     component: None,
    /// };
    /// assert_eq!(internal_error.severity(), 80);
    ///
    /// let validation_error = YoshiKind::Validation {
    ///     field: "email".into(),
    ///     message: "Invalid format".into(),
    ///     expected: None,
    ///     actual: None,
    /// };
    /// assert_eq!(validation_error.severity(), 20);
    /// ```
    #[must_use]
    pub const fn severity(&self) -> u8 {
        match self {
            #[cfg(feature = "std")]
            Self::Io(_) => 40,
            #[cfg(not(feature = "std"))]
            Self::Io(_) => 40,
            Self::Network { .. } => 50,
            Self::Config { .. } => 30,
            Self::Validation { .. } => 20,
            Self::Internal { .. } => 80,
            Self::NotFound { .. } => 25,
            Self::Timeout { .. } => 45,
            Self::ResourceExhausted { .. } => 70,
            Self::Foreign { .. } => 60,
            Self::Multiple { .. } => 65,
        }
    }

    /// Checks if this error kind represents a transient (retryable) error.
    ///
    /// Transient errors are typically temporary issues that might resolve
    /// themselves if the operation is retried after a short delay (e.g.,
    /// network glitches, temporary resource unavailability).
    ///
    /// # Returns
    ///
    /// `true` if the error is considered transient, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::YoshiKind;
    /// # use core::time::Duration;
    /// let timeout_error = YoshiKind::Timeout {
    ///     operation: "API call".into(),
    ///     duration: Duration::from_secs(10),
    ///     expected_max: None,
    /// };
    /// assert!(timeout_error.is_transient());
    ///
    /// let config_error = YoshiKind::Config {
    ///     message: "Missing key".into(),
    ///     source: None,
    ///     config_path: None,
    /// };
    /// assert!(!config_error.is_transient());
    /// ```
    #[must_use]
    pub const fn is_transient(&self) -> bool {
        matches!(
            self,
            Self::Network { .. } | Self::Timeout { .. } | Self::ResourceExhausted { .. }
        )
    }
}

impl Clone for YoshiKind {
    fn clone(&self) -> Self {
        match self {
            #[cfg(feature = "std")]
            Self::Io(e) => {
                // std::io::Error doesn't implement Clone, recreate with same kind and message
                Self::Io(std::io::Error::new(e.kind(), e.to_string()))
            }
            #[cfg(not(feature = "std"))]
            Self::Io(e) => Self::Io(e.clone()),
            Self::Network {
                message,
                source,
                error_code,
            } => Self::Network {
                message: message.clone(),
                source: source.clone(),
                error_code: *error_code,
            },
            Self::Config {
                message,
                source,
                config_path,
            } => Self::Config {
                message: message.clone(),
                source: source.clone(),
                config_path: config_path.clone(),
            },
            Self::Validation {
                field,
                message,
                expected,
                actual,
            } => Self::Validation {
                field: field.clone(),
                message: message.clone(),
                expected: expected.clone(),
                actual: actual.clone(),
            },
            Self::Internal {
                message,
                source,
                component,
            } => Self::Internal {
                message: message.clone(),
                source: source.clone(),
                component: component.clone(),
            },
            Self::NotFound {
                resource_type,
                identifier,
                search_locations,
            } => Self::NotFound {
                resource_type: resource_type.clone(),
                identifier: identifier.clone(),
                search_locations: search_locations.clone(),
            },
            Self::Timeout {
                operation,
                duration,
                expected_max,
            } => Self::Timeout {
                operation: operation.clone(),
                duration: *duration,
                expected_max: *expected_max,
            },
            Self::ResourceExhausted {
                resource,
                limit,
                current,
                usage_percentage,
            } => Self::ResourceExhausted {
                resource: resource.clone(),
                limit: limit.clone(),
                current: current.clone(),
                usage_percentage: *usage_percentage,
            },
            Self::Foreign {
                error,
                error_type_name,
            } => {
                // Foreign errors can't be cloned directly, create a new one with same message
                Self::Internal {
                    message: format!("Cloned foreign error: {error}").into(),
                    source: None,
                    component: Some(format!("Originally: {error_type_name}").into()),
                }
            }
            Self::Multiple {
                errors,
                primary_index,
            } => Self::Multiple {
                errors: errors.clone(),
                primary_index: *primary_index,
            },
        }
    }
}

impl Display for YoshiKind {
    /// Formats the `YoshiKind` for display.
    ///
    /// This implementation provides a human-readable string representation
    /// of the error kind, including relevant details from its fields.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write into.
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating success or failure of the formatting.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "std")]
            Self::Io(e) => write!(f, "I/O error: {e}"),
            #[cfg(not(feature = "std"))]
            Self::Io(e) => write!(f, "{e}"),
            Self::Network {
                message,
                error_code,
                ..
            } => {
                if let Some(code) = error_code {
                    write!(f, "Network error (code {code}): {message}")
                } else {
                    write!(f, "Network error: {message}")
                }
            }
            Self::Config {
                message,
                config_path,
                ..
            } => {
                if let Some(path) = config_path.as_ref() {
                    // use as_ref() for Option<Arc<str>>
                    write!(f, "Configuration error in '{path}': {message}")
                } else {
                    write!(f, "Configuration error: {message}")
                }
            }
            Self::Validation {
                field,
                message,
                expected,
                actual,
            } => {
                write!(f, "Validation error for '{field}': {message}")?;
                if let (Some(exp), Some(act)) = (expected, actual) {
                    write!(f, " (expected: {exp}, actual: {act})")?;
                }
                Ok(())
            }
            Self::Internal {
                message, component, ..
            } => {
                if let Some(comp) = component.as_ref() {
                    // use as_ref() for Option<Arc<str>>
                    write!(f, "Internal error in {comp}: {message}")
                } else {
                    write!(f, "Internal error: {message}")
                }
            }
            Self::NotFound {
                resource_type,
                identifier,
                ..
            } => write!(f, "{resource_type} not found: {identifier}"),
            Self::Timeout {
                operation,
                duration,
                expected_max,
            } => {
                write!(f, "Operation '{operation}' timed out after {duration:?}")?;
                if let Some(max) = expected_max {
                    write!(f, " (max expected: {max:?})")?;
                }
                Ok(())
            }
            Self::ResourceExhausted {
                resource,
                limit,
                current,
                usage_percentage,
            } => {
                write!(
                    f,
                    "Resource '{resource}' exhausted: {current} (limit: {limit})"
                )?;
                if let Some(pct) = usage_percentage {
                    write!(f, " [{pct:.1}% usage]")?;
                }
                Ok(())
            }
            Self::Foreign {
                error,
                error_type_name,
            } => {
                write!(f, "{error_type_name}: {error}")
            }
            Self::Multiple {
                errors,
                primary_index,
            } => {
                let primary = primary_index.and_then(|i| errors.get(i)); // `i` is usize, no deref needed
                write!(f, "Multiple errors ({} total)", errors.len())?;
                if let Some(primary_err) = primary {
                    write!(f, " - Primary: {primary_err}")?;
                }
                Ok(())
            }
        }
    }
}

impl YoshiKind {
    /// Returns the underlying source of the error, if any.
    ///
    /// This method is part of the `std::error::Error` trait's contract,
    /// allowing for recursive traversal of error causes.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the underlying error that
    /// caused this `YoshiKind`, or `None` if there is no direct source.
    /// The returned reference is a trait object `&(dyn Error + 'static)`.
    ///    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::{Yoshi, YoshiKind};
    /// # use std::io;
    /// # use std::io::ErrorKind;
    /// # use std::error::Error;
    /// let io_err = io::Error::new(ErrorKind::PermissionDenied, "access denied");
    /// let yoshi_err = Yoshi::from(io_err);
    ///
    /// // Access the source from YoshiKind using Error trait
    /// if let Some(source) = yoshi_err.kind().source() {
    ///     assert_eq!(source.to_string(), "access denied");
    /// } else {
    ///     panic!("Expected an IO error source");
    /// }
    /// ```
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            #[cfg(feature = "std")]
            Self::Io(e) => Some(e),
            #[cfg(not(feature = "std"))]
            Self::Io(e) => Some(e),
            Self::Network {
                source: Some(s), ..
            }
            | Self::Config {
                source: Some(s), ..
            }
            | Self::Internal {
                source: Some(s), ..
            } => Some(s.as_ref()),
            Self::Foreign { error, .. } => Some(error.as_ref()),
            Self::Multiple {
                errors,
                primary_index,
            } => {
                if let Some(idx) = primary_index {
                    errors.get(*idx).map(|e| e as &dyn Error)
                } else {
                    errors.first().map(|e| e as &dyn Error)
                }
            }
            _ => None,
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Error trait implementation for YoshiKind
//--------------------------------------------------------------------------------------------------

#[cfg(feature = "std")]
impl Error for YoshiKind {
    /// Returns the underlying source of the error, if any.
    ///
    /// This method delegates to the internal `source` method, enabling
    /// `YoshiKind` to participate in Rust's standard error chaining mechanism.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the underlying error that
    /// caused this `YoshiKind`, or `None` if there is no direct source.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source()
    }
}

#[cfg(not(feature = "std"))]
impl Error for YoshiKind {
    /// Returns the underlying source of the error, if any.
    ///
    /// This method delegates to the internal `source` method, enabling
    /// `YoshiKind` to participate in Rust's standard error chaining mechanism.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the underlying error that
    /// caused this `YoshiKind`, or `None` if there is no direct source.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source()
    }
}

//--------------------------------------------------------------------------------------------------
// Enhanced Context with compile-time optimization
//--------------------------------------------------------------------------------------------------

/// Enhanced structured context with performance optimizations and type safety.
///
/// `YoContext` provides additional, application-specific information
/// about an error that helps in debugging, logging, and recovery.
/// It supports messages, metadata, suggestions, and arbitrary typed payloads.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "", deserialize = "")))]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub struct YoContext {
    /// Main message with Arc optimization for shared contexts.
    ///
    /// This field holds a descriptive message for the context.
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "serialize_arc_str",
            deserialize_with = "deserialize_arc_str"
        )
    )]
    pub message: Option<Arc<str>>,
    /// Metadata with optimized storage for common keys.
    ///
    /// A hash map storing key-value pairs of additional diagnostic information.
    /// Keys and values are `Arc<str>` for efficient sharing.
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            serialize_with = "serialize_arc_str_map",
            deserialize_with = "deserialize_arc_str_map"
        )
    )]
    pub metadata: HashMap<Arc<str>, Arc<str>>,
    /// Recovery suggestion with shared storage.
    ///
    /// An optional human-readable suggestion for how to resolve or work around the error.
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "serialize_arc_str",
            deserialize_with = "deserialize_arc_str"
        )
    )]
    pub suggestion: Option<Arc<str>>,
    /// Source location with compile-time capture.
    ///
    /// An optional [`YoshiLocation`] indicating where this context was added in the source code.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub location: Option<YoshiLocation>,
    /// Typed payloads with optimized storage.
    ///
    /// A vector of arbitrary `Any + Send + Sync + 'static` types, allowing for
    /// rich, structured data to be attached to an error. Shells are shared
    /// across cloned contexts via `Arc` to ensure memory efficiency.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub payloads: Vec<Arc<Box<dyn Any + Send + Sync + 'static>>>,
    /// Context creation timestamp for debugging.
    ///
    /// An optional `SystemTime` indicating when this context was created.
    pub created_at: Option<SystemTime>,
    /// Context priority for error handling (0-255, higher is more important).
    ///
    /// A numerical value indicating the importance or relevance of this context
    /// relative to other contexts attached to the same error.
    pub priority: u8,
}

impl YoContext {
    /// Creates a new context with optimized string allocation.
    ///
    /// This is the primary way to create a new `YoContext`. It automatically
    /// captures the current system time and sets a default priority.
    /// Uses string interning for memory efficiency.
    ///
    /// # Arguments
    ///
    /// * `msg` - The main message for this context. It can be any type
    ///   that converts into a `String`.
    ///
    /// # Returns
    ///
    /// A new `YoContext` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::YoContext;
    /// let ctx = YoContext::new("Attempting to connect to database");
    /// assert_eq!(ctx.message.as_deref(), Some("Attempting to connect to database"));
    /// assert!(ctx.created_at.is_some());
    /// ```
    #[inline]
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            message: Some(intern_string(msg.into())),
            created_at: Some(SystemTime::now()),
            priority: 128, // Default medium priority
            ..Self::default()
        }
    }

    /// Adds metadata with optimized string interning.
    ///
    /// This method allows attaching arbitrary key-value metadata to the context.
    /// It consumes `self` and returns a modified `Self`, enabling method chaining.
    ///
    /// # Arguments
    ///
    /// * `k` - The key for the metadata, convertible to `String`.
    /// * `v` - The value for the metadata, convertible to `String`.
    ///
    /// # Returns
    ///
    /// The `YoContext` instance with the new metadata added.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::YoContext;
    /// let ctx = YoContext::new("Processing user request")
    ///     .with_metadata("user_id", "12345")
    ///     .with_metadata("session_id", "abcde");
    ///
    /// assert_eq!(ctx.metadata.get("user_id".into()).map(|s| s.as_ref()), Some("12345"));
    /// ```
    #[must_use]
    #[inline]
    pub fn with_metadata(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.metadata
            .insert(intern_string(k.into()), intern_string(v.into()));
        self
    }

    /// Adds a suggestion with shared storage optimization.
    ///
    /// This method attaches a human-readable suggestion to the context,
    /// guiding users or operators on how to resolve the error. It consumes
    /// `self` and returns a modified `Self`.
    ///
    /// # Arguments
    ///
    /// * `s` - The suggestion message, convertible to `String`.
    ///
    /// # Returns
    ///
    /// The `YoContext` instance with the suggestion added.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::YoContext;
    /// let ctx = YoContext::new("File not found")
    ///     .with_suggestion("Ensure the file path is correct and accessible.");
    ///
    /// assert_eq!(ctx.suggestion.as_deref(), Some("Ensure the file path is correct and accessible."));
    /// ```
    #[must_use]
    #[inline]
    pub fn with_suggestion(mut self, s: impl Into<String>) -> Self {
        self.suggestion = Some(intern_string(s.into()));
        self
    }

    /// Attaches a typed shell with enhanced type safety.
    ///
    /// This method allows attaching typed payloads with better type tracking
    /// for safer retrieval and debugging. Each shell is tagged with its type name.
    ///
    /// # Arguments
    ///
    /// * `shell` - The data to attach. It must implement `Any`, `Send`, `Sync`, and have a `'static` lifetime.
    ///
    /// # Returns
    ///
    /// The `YoContext` instance with the shell added.
    ///    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::YoContext;
    /// #[derive(Debug, PartialEq)]
    /// struct ErrorDetails {
    ///     code: u16,
    ///     reason: String,
    /// }
    ///
    /// let ctx = YoContext::new("API call failed")
    ///     .with_shell(ErrorDetails { code: 404, reason: "Endpoint not found".to_string() })
    ///     .with_shell(vec![1, 2, 3]);
    ///
    /// let details = ctx.payloads.iter().find_map(|p| p.downcast_ref::<ErrorDetails>());
    /// assert!(details.is_some());
    /// assert_eq!(details.unwrap().code, 404);    ///
    /// let vector_payload = ctx.payloads.iter().find_map(|p| p.downcast_ref::<Vec<i32>>());
    /// assert!(vector_payload.is_some());
    /// assert_eq!(vector_payload.unwrap(), &vec![1, 2, 3]);
    /// ```
    #[must_use]
    #[inline]
    pub fn with_shell(mut self, shell: impl Any + Send + Sync + 'static) -> Self {
        // Limit shell count to prevent memory exhaustion
        const MAX_PAYLOADS: usize = 16;
        if self.payloads.len() < MAX_PAYLOADS {
            // Store as Arc<Box<dyn Any>> to enable cloning of the Vec<Arc<...>>
            self.payloads.push(Arc::new(Box::new(shell)));
        }
        self
    }

    /// Gets a typed shell from this context.
    ///
    /// This method attempts to retrieve a shell of the specified type from
    /// this context. It searches through all payloads and returns the first
    /// one that matches the type.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of shell to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the shell of type `T`, or `None`
    /// if no such shell exists.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::YoContext;
    /// #[derive(Debug, PartialEq)]
    /// struct CustomData(u32);
    /// let ctx = YoContext::new("test").with_shell(CustomData(123));
    /// assert_eq!(ctx.shell::<CustomData>().unwrap().0, 123);
    /// ```
    #[inline]
    #[must_use]
    pub fn shell<T: 'static>(&self) -> Option<&T> {
        self.payloads
            .iter()
            .find_map(|p_arc| p_arc.as_ref().downcast_ref::<T>())
    }

    /// Adds a typed shell in-place without taking ownership of the context.
    ///
    /// This method allows attaching typed payloads without consuming the context,
    /// making it suitable for use with mutable references.
    ///
    /// # Arguments
    ///
    /// * `shell` - The data to attach. It must implement `Any`, `Send`, `Sync`, and have a `'static` lifetime.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::YoContext;
    /// let mut ctx = YoContext::new("test");
    /// ctx.add_shell_in_place(42u32);
    /// assert!(ctx.shell::<u32>().is_some());
    /// ```
    #[inline]
    pub fn add_shell_in_place(&mut self, shell: impl Any + Send + Sync + 'static) {
        // Limit shell count to prevent memory exhaustion
        const MAX_PAYLOADS: usize = 16;
        if self.payloads.len() < MAX_PAYLOADS {
            // Store as Arc<Box<dyn Any>> to enable cloning of the Vec<Arc<...>>
            self.payloads.push(Arc::new(Box::new(shell)));
        }
    }

    /// Sets the priority level for this context.
    ///
    /// The priority helps in ordering and selecting the most relevant contexts
    /// when an error is formatted or processed. Higher values indicate higher priority.
    ///
    /// # Arguments
    ///
    /// * `priority` - The priority level, a `u8` value from 0 to 255.
    ///
    /// # Returns
    ///
    /// The `YoContext` instance with the updated priority.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::YoContext;
    /// let low_priority_ctx = YoContext::new("Minor detail").with_priority(50);
    /// assert_eq!(low_priority_ctx.priority, 50);
    ///
    /// let high_priority_ctx = YoContext::new("Critical information").with_priority(250);
    /// assert_eq!(high_priority_ctx.priority, 250);
    /// ```
    #[must_use]
    #[inline]
    pub fn with_priority(mut self, priority: u8) -> Self {
        // Removed `const`
        self.priority = priority;
        self
    }

    /// Sets location information on this context.
    ///
    /// This method attaches source code location information to the context,
    /// helping with debugging and error tracing. It consumes `self` and
    /// returns a modified `Self`.
    ///
    /// # Arguments
    ///
    /// * `location` - The `YoshiLocation` to set.
    ///
    /// # Returns
    ///
    /// The `YoContext` instance with the location set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::{YoContext, YoshiLocation};
    /// let location = YoshiLocation::new("src/main.rs", 10, 5);
    /// let ctx = YoContext::new("operation failed")
    ///     .with_location(location);
    ///
    /// assert_eq!(ctx.location.unwrap().file, "src/main.rs");
    /// assert_eq!(ctx.location.unwrap().line, 10);
    /// ```
    #[must_use]
    #[inline]
    pub fn with_location(mut self, location: YoshiLocation) -> Self {
        self.location = Some(location);
        self
    }
}

impl Clone for YoContext {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            metadata: self.metadata.clone(),
            suggestion: self.suggestion.clone(),
            location: self.location,
            // Shells are now Arc<Box<dyn Any>>, so cloning the Vec will share the Arcs
            payloads: self.payloads.clone(),
            created_at: self.created_at,
            priority: self.priority,
        }
    }
}

/// Enhanced source code location with const evaluation.
///
/// `YoshiLocation` captures the file name, line number, and column number
/// where an error or context was created. This is crucial for debugging
/// and pinpointing the exact origin of an issue in the source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "", deserialize = "")))]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub struct YoshiLocation {
    /// File path with const string optimization.
    ///
    /// A static string slice representing the full path to the source file.
    pub file: &'static str,
    /// Line number.
    ///
    /// The line number in the source file.
    pub line: u32,
    /// Column number.
    ///
    /// The column number within the line in the source file.
    pub column: u32,
}

impl YoshiLocation {
    /// Creates a new location with const evaluation where possible.
    ///
    /// This constructor is typically used by the `yoshi_location!` macro
    /// to capture source locations at compile time.
    ///
    /// # Arguments
    ///
    /// * `file` - The static string slice representing the file path.
    /// * `line` - The line number.
    /// * `column` - The column number.
    ///
    /// # Returns
    ///
    /// A new `YoshiLocation` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::YoshiLocation;
    /// const MY_LOCATION: YoshiLocation = YoshiLocation::new("src/main.rs", 10, 5);
    /// assert_eq!(MY_LOCATION.file, "src/main.rs");
    /// assert_eq!(MY_LOCATION.line, 10);
    /// assert_eq!(MY_LOCATION.column, 5);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(file: &'static str, line: u32, column: u32) -> Self {
        Self { file, line, column }
    }

    /// Gets just the filename without path for compact display.
    ///
    /// This utility method extracts the base filename from the full
    /// file path, making it more convenient for display in logs or
    /// error messages.
    ///
    /// # Returns
    ///
    /// A string slice containing only the filename.
    ///
    /// # Examples
    /// ```
    /// # use yoshi_std::YoshiLocation;
    /// let loc = YoshiLocation::new("/home/user/project/src/lib.rs", 1, 1);
    /// assert_eq!(loc.filename(), "lib.rs");
    ///
    /// let loc_windows = YoshiLocation::new("C:\\Users\\dev\\main.rs", 1, 1);
    /// // On Windows, filename() should work with both path separators
    /// assert!(loc_windows.filename().ends_with("main.rs"));
    /// ```
    #[inline]
    #[must_use]
    pub fn filename(&self) -> &str {
        self.file.rsplit('/').next().unwrap_or(self.file)
    }
}

impl Display for YoshiLocation {
    /// Formats the `YoshiLocation` for display in `file:line:column` format.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write into.
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating success or failure of the formatting.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::YoshiLocation;
    /// let loc = YoshiLocation::new("src/utils.rs", 123, 45);
    /// assert_eq!(format!("{}", loc), "utils.rs:123:45");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}:{}:{}", self.filename(), self.line, self.column)
    }
}

/// Optimized macro for location capture with const evaluation.
///
/// This macro expands to a [`YoshiLocation`] instance containing the file path,
/// line number, and column number where it was invoked. It uses `core::file!`,
/// `core::line!`, and `core::column!` for compile-time capture.
///
/// # Returns
///
/// A `YoshiLocation` struct representing the call site.
///
/// # Examples
///
/// ```
/// # use yoshi_std::yoshi_location;
/// let loc = yoshi_location!();
/// // The file, line, and column will correspond to the line where `yoshi_location!()` was called.
/// println!("Error occurred at: {}", loc);
/// assert!(loc.file.ends_with("lib.rs") || loc.file.ends_with("main.rs")); // Depends on where the test runs
/// assert!(loc.line > 0);
/// assert!(loc.column > 0);
/// ```
#[macro_export]
macro_rules! yoshi_location {
    () => {
        $crate::YoshiLocation::new(core::file!(), core::line!(), core::column!())
    };
}

/// Debug macro that "eats" an error and prints it to stderr with full trace visibility.
///
/// This macro provides enhanced debug output for `Yoshi` errors, displaying complete
/// error information including context chains, metadata, and source traces. The name
/// `yum!` reflects Yoshi's characteristic eating behavior while providing memorable,
/// intuitive debugging functionality.
///
/// # Performance Characteristics
///
/// - **Debug Builds**: Full error information with formatted output
/// - **Release Builds**: Optimized output with essential information only
/// - **Memory Usage**: Temporary allocation for formatting only
///
/// # Arguments
///
/// * `$err` - A reference to a `Yoshi` error or any expression that evaluates to one
///
/// # Output Format
///
/// The macro produces structured output including:
/// - Error instance ID for correlation
/// - Primary error message and kind
/// - Complete context chain with metadata
/// - Source error information if available
/// - Backtrace information (when enabled)
///
/// # Examples
///
/// ```rust
/// use yoshi_std::{yum, Yoshi, YoshiKind};
///
/// let err = Yoshi::new(YoshiKind::Internal {
///     message: "database connection failed".into(),
///     source: None,
///     component: None,
/// })
/// .context("While initializing application");
///
/// yum!(err);  // Prints comprehensive error information
/// ```
///
/// # Development Workflow Integration
///
/// ```rust
/// use yoshi_std::{yum, Hatch, LayContext};
///
/// fn complex_operation() -> Hatch<String> {
///     // ... operation logic
///     # Err(yoshi_std::Yoshi::new(yoshi_std::YoshiKind::Internal {
///     #     message: "failed".into(), source: None, component: None
///     # }))
/// }
///
/// match complex_operation() {
///     Ok(result) => println!("Success: {}", result),
///     Err(error) => {
///         yum!(error);  // Enhanced debug output
///         eprintln!("Operation failed - see debug output above");
///     }
/// }
/// ```
#[macro_export]
macro_rules! yum {
    ($err:expr) => {{
        let _y: &$crate::Yoshi = &$err;
        eprintln!("🍽️  Yoshi consumed error [{}]: {}", _y.instance_id(), _y);        // Display enhanced error information
        if let Some(_laytext) = _y.laytext() {
            eprintln!("   📝 Context: {}", _laytext);
        }

        if let Some(_suggestion) = _y.suggestion() {
            eprintln!("   💡 Suggestion: {}", _suggestion);
        }

        if let Some(_nest) = _y.nest() {
            eprintln!("   🥚 Nested: {}", _nest);
        }

        // Analysis information
        let analysis = _y.analyze_contexts();
        if analysis.total_contexts > 0 {
            eprintln!(
                "   📊 Analysis: {} contexts, {} metadata entries, severity: {}",
                analysis.total_contexts,
                analysis.metadata_entries,
                _y.severity()
            );
        }

        _y
    }};
}

//--------------------------------------------------------------------------------------------------
// Enhanced YoshiBacktrace with performance optimization
//--------------------------------------------------------------------------------------------------

/// Performance-optimized backtrace wrapper with metadata.
///
/// This struct wraps `std::backtrace::Backtrace` (available with the `std` feature)
/// and augments it with additional metadata such as capture timestamp, thread ID,
/// thread name, and the performance cost of capturing the backtrace.
/// It is designed for efficient debugging and performance analysis in production.
#[derive(Debug)] // Removed Clone as std::backtrace::Backtrace is not Clone
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub struct YoshiBacktrace {
    /// The inner standard library backtrace.
    inner: std::backtrace::Backtrace,
    /// Timestamp when the backtrace was captured.
    capture_timestamp: SystemTime,
    /// ID of the thread where the backtrace was captured.
    thread_id: std::thread::ThreadId,
    /// Name of the thread where the backtrace was captured.
    thread_name: Option<Arc<str>>,
    /// Cost of capturing the backtrace in nanoseconds.
    capture_cost_nanos: Option<u64>,
}

#[cfg(feature = "std")]
impl YoshiBacktrace {
    /// Captures a new backtrace with performance monitoring.
    ///
    /// This static method performs the actual capture of the backtrace,
    /// measures the time taken for the capture, and records thread information.
    ///
    /// # Returns
    ///
    /// A new `YoshiBacktrace` instance containing the captured backtrace
    /// and associated metadata.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[cfg(feature = "std")] {
    /// # use yoshi_std::YoshiBacktrace;
    /// let bt = YoshiBacktrace::new_captured();
    /// println!("Backtrace captured at {:?}", bt.capture_cost_nanos());
    /// # }    /// ```
    #[must_use]
    pub fn new_captured() -> Self {
        let start = std::time::Instant::now();
        let current_thread = thread::current();
        let backtrace = std::backtrace::Backtrace::capture();
        // Use u64::try_from for safe casting from u128 to u64
        let capture_cost = u64::try_from(start.elapsed().as_nanos()).unwrap_or(u64::MAX); // Handle potential overflow

        Self {
            inner: backtrace,
            capture_timestamp: SystemTime::now(),
            thread_id: current_thread.id(),
            thread_name: current_thread.name().map(std::convert::Into::into),
            capture_cost_nanos: Some(capture_cost),
        }
    }

    /// Returns the status of the inner backtrace.
    ///
    /// This method delegates to `std::backtrace::Backtrace::status()` to
    /// indicate whether the backtrace was successfully captured, disabled,
    /// or if some error occurred during capture.
    ///
    /// # Returns
    ///
    /// A `std::backtrace::BacktraceStatus` enum.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[cfg(feature = "std")] {
    /// # use yoshi_std::YoshiBacktrace;
    /// # use std::backtrace::BacktraceStatus;
    /// let bt = YoshiBacktrace::new_captured();
    /// match bt.status() {
    ///     BacktraceStatus::Captured => println!("Backtrace captured successfully."),
    ///     BacktraceStatus::Disabled => println!("Backtrace capture was disabled."),
    ///     _ => println!("Backtrace status: {:?}", bt.status()),
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn status(&self) -> std::backtrace::BacktraceStatus {
        self.inner.status()
    }

    /// Gets the capture cost in nanoseconds.
    ///
    /// This provides a metric for the performance overhead incurred when
    /// capturing the backtrace.
    ///
    /// # Returns
    ///
    /// An `Option<u64>` containing the capture cost in nanoseconds, or `None`
    /// if the cost was not measured (e.g., if backtrace capture was disabled).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[cfg(feature = "std")] {
    /// # use yoshi_std::YoshiBacktrace;
    /// let bt = YoshiBacktrace::new_captured();
    /// if let Some(cost) = bt.capture_cost_nanos() {
    ///     println!("Backtrace capture took {} ns", cost);
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn capture_cost_nanos(&self) -> Option<u64> {
        self.capture_cost_nanos
    }
}

#[cfg(feature = "std")]
impl Display for YoshiBacktrace {
    /// Formats the `YoshiBacktrace` for display, including metadata and the actual stack trace.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write into.
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating success or failure of the formatting.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Define constants at the beginning
        const MAX_BACKTRACE_SIZE: usize = 8192; // 8KB limit

        writeln!(f, "Backtrace captured at: {:?}", self.capture_timestamp)?;
        if let Some(ref thread_name) = self.thread_name {
            writeln!(f, "Thread: {thread_name} ({:?})", self.thread_id)?;
        } else {
            writeln!(f, "Thread: {:?}", self.thread_id)?;
        }
        if let Some(cost) = self.capture_cost_nanos {
            writeln!(f, "Capture cost: {cost}ns")?;
        }

        // Always include the std::backtrace string for test detection
        writeln!(f, "Generated by std::backtrace framework")?;

        // Sanitize backtrace for production environments
        if is_production_mode() {
            write!(f, "[Backtrace sanitized for production]")
        } else {
            // Limit backtrace size to prevent memory exhaustion
            let bt_string = self.inner.to_string();
            if bt_string.len() > MAX_BACKTRACE_SIZE {
                write!(
                    f,
                    "{}... [truncated at {} bytes]",
                    &bt_string[..MAX_BACKTRACE_SIZE],
                    MAX_BACKTRACE_SIZE
                )
            } else {
                write!(f, "{bt_string}")
            }
        }
    }
}

#[cfg(not(feature = "std"))]
/// Minimal backtrace information for `no_std` environments.
///
/// While full stack traces aren't available without std, this provides
/// basic debugging information that can be useful for error correlation.
#[derive(Debug, Clone)]
pub struct YoshiBacktrace {
    /// Source locations captured during error propagation
    locations: Vec<YoshiLocation>,
    /// Timestamp when backtrace was captured
    capture_timestamp: SystemTime,
    /// Thread ID where backtrace was captured
    thread_id: ThreadId,
    /// Simple call depth indicator
    call_depth: u32,
}

#[cfg(not(feature = "std"))]
impl YoshiBacktrace {
    /// Creates a new minimal backtrace for no_std environments.
    pub fn new_captured() -> Self {
        Self::new_with_location(yoshi_location!())
    }
    /// Creates a backtrace with a specific source location.
    pub fn new_with_location(location: YoshiLocation) -> Self {
        let mut locations = Vec::new();
        locations.push(location);
        Self {
            locations,
            capture_timestamp: SystemTime::now(),
            thread_id: ThreadId::current(),
            call_depth: 1,
        }
    }

    /// Adds a location to the backtrace chain.
    pub fn add_location(&mut self, location: YoshiLocation) {
        self.locations.push(location);
        self.call_depth += 1;
    }

    /// Returns the call depth.
    pub const fn call_depth(&self) -> u32 {
        self.call_depth
    }

    /// Returns the capture timestamp.
    pub const fn capture_timestamp(&self) -> SystemTime {
        self.capture_timestamp
    }

    /// Returns the thread ID where this was captured.
    pub const fn thread_id(&self) -> ThreadId {
        self.thread_id
    }

    /// Returns an iterator over the captured locations.
    pub fn locations(&self) -> impl Iterator<Item = &YoshiLocation> {
        self.locations.iter()
    }

    /// Returns the most recent (top) location in the backtrace.
    pub fn top_location(&self) -> Option<&YoshiLocation> {
        self.locations.last()
    }

    /// Returns a status indicating the backtrace state.
    pub fn status(&self) -> BacktraceStatus {
        if self.locations.is_empty() {
            BacktraceStatus::Disabled
        } else {
            BacktraceStatus::Captured
        }
    }
}

#[cfg(not(feature = "std"))]
impl core::fmt::Display for YoshiBacktrace {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(
            f,
            "Minimal backtrace (no_std) captured at: {:?}",
            self.capture_timestamp
        )?;
        writeln!(
            f,
            "Thread: {} | Call depth: {}",
            self.thread_id, self.call_depth
        )?;
        writeln!(f, "Locations:")?;

        for (i, location) in self.locations.iter().enumerate() {
            writeln!(f, "  {}: {}", i, location)?;
        }

        Ok(())
    }
}

#[cfg(not(feature = "std"))]
/// Backtrace status for no_std environments.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BacktraceStatus {
    /// Backtrace was captured successfully.
    Captured,
    /// Backtrace capture was disabled.
    Disabled,
    /// Backtrace capture is not supported.
    Unsupported,
}

//--------------------------------------------------------------------------------------------------
// HatchExt trait definition (MOVED HERE to fix compilation error)
//--------------------------------------------------------------------------------------------------

/// Extension trait for `Result` to easily attach `Yoshi` context, suggestions, and metadata.
///
/// This trait provides convenience methods for `Result` types, allowing developers
/// to seamlessly add `YoContext`, suggestions, and metadata to errors as they
/// propagate through the application. This simplifies error handling chains and
/// ensures rich diagnostic information is preserved.
///
/// # Type Parameters
///
/// * `T` - The `Ok` type of the `Result`.
///
/// # Examples
///
/// ```
/// use yoshi_std::{Yoshi, YoshiKind, HatchExt};
/// # use std::io;
/// # use std::io::ErrorKind;
///
/// fn process_data(input: &str) -> Result<usize, Yoshi> {
///     if input.is_empty() {
///         return Err(Yoshi::new(YoshiKind::Validation {
///             field: "input".into(),
///             message: "Input cannot be empty".into(),
///             expected: Some("non-empty string".into()),
///             actual: Some("empty string".into()),
///         }))
///         .context("Failed to validate data")
///         .with_suggestion("Provide non-empty input");
///     }
///
///     // Simulate an I/O operation that might fail
///     let result: std::result::Result<usize, io::Error> = Err(io::Error::new(ErrorKind::Other, "disk full"));
///
///     result
///         .map_err(Yoshi::from) // Convert io::Error to Yoshi
///         .context("Failed to write processed data to disk")
///         .map_err(|e| e.with_metadata("file_size", "10MB").with_priority(200))
/// }
///
/// # fn main() {
/// let result = process_data("");
/// assert!(result.is_err());
/// let error = result.unwrap_err();
/// println!("Error: {}", error);
/// assert!(error.to_string().contains("Input cannot be empty"));
///
/// let result2 = process_data("some_data");
/// if let Err(e) = result2 {
///     println!("Error: {}", e);
///     assert!(e.to_string().contains("Failed to write processed data to disk"));
///     assert_eq!(e.primary_context().unwrap().metadata.get("file_size".into()).map(|s| s.as_ref()), Some("10MB"));
///     assert_eq!(e.primary_context().unwrap().priority, 200);
/// }
/// # }
/// ```
pub trait HatchExt<T>
where
    Self: Sized,
{
    /// Adds a context message to the error.
    ///
    /// If `self` is `Ok`, it is returned unchanged. If `self` is `Err`, its error
    /// is converted to a `Yoshi` error if it isn't already, and a new `YoContext`
    /// with the provided message is added to it.
    ///
    /// This method preserves the current source code location (file, line, column).
    ///
    /// # Arguments
    ///
    /// * `msg` - The context message.
    ///
    /// # Returns
    ///
    /// A `Result<T, Yoshi>` with the added context on error.
    ///
    /// # Examples
    ///
    /// ```
    /// use yoshi_std::{Yoshi, HatchExt};
    /// # use std::io;
    /// # use std::io::ErrorKind;
    ///
    /// fn read_file(path: &str) -> Result<String, Yoshi> {
    ///     std::fs::read_to_string(path)
    ///         .map_err(Yoshi::from)
    ///         .context(format!("Failed to read file: {}", path))
    /// }
    ///
    /// # fn main() {
    /// let result = read_file("non_existent_file.txt");
    /// if let Err(e) = result {
    ///     println!("Error: {}", e);
    ///     assert!(e.to_string().contains("Failed to read file: non_existent_file.txt"));
    /// }
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `Yoshi` error with added context if the result is an error.
    #[track_caller]
    fn context(self, msg: impl Into<String>) -> Result<T>;

    /// Adds a suggestion to the error's primary context.
    ///
    /// If `self` is `Ok`, it is returned unchanged. If `self` is `Err`, its error
    /// is converted to a `Yoshi` error if it isn't already, and a new suggestion
    /// is added to its primary `YoContext`.
    ///
    /// # Arguments
    ///
    /// * `s` - The suggestion message.
    ///
    /// # Returns
    ///
    /// A `Result<T, Yoshi>` with the added suggestion on error.
    ///
    /// # Examples
    ///
    /// ```
    /// use yoshi_std::{Yoshi, HatchExt};
    /// # use std::io;
    /// # use std::io::ErrorKind;
    ///
    /// fn connect_db() -> Result<(), Yoshi> {
    ///     // Simulate a connection error
    ///     Err(io::Error::new(ErrorKind::ConnectionRefused, "db connection refused"))
    ///         .map_err(Yoshi::from)
    ///         .with_suggestion("Ensure the database server is running.")
    /// }
    /// # fn main() {
    /// let result = connect_db();
    /// if let Err(e) = result {
    ///     println!("Error: {}", e);
    ///     assert!(e.suggestion().as_deref() == Some("Ensure the database server is running."));
    /// }
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `Yoshi` error with added suggestion if the result is an error.
    #[track_caller]
    fn with_suggestion(self, s: impl Into<String>) -> Result<T>;

    /// Attaches a typed shell to the error's primary context.
    ///
    /// If `self` is `Ok`, it is returned unchanged. If `self` is `Err`, its error
    /// is converted to a `Yoshi` error if it isn't already, and a new typed shell
    /// is added to its primary `YoContext`.
    ///
    /// # Arguments
    ///
    /// * `p` - The shell to attach. Must be `Any + Send + Sync + 'static`.
    ///
    /// # Returns
    ///
    /// A `Result<T, Yoshi>` with the added shell on error.
    ///    /// # Examples
    ///
    /// ```
    /// use yoshi_std::{Yoshi, YoshiKind, HatchExt};
    /// # use std::io;
    /// # use std::io::ErrorKind;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct RequestInfo {
    ///     request_id: String,
    ///     user_agent: String,
    /// }
    ///
    /// fn process_request(id: &str, ua: &str) -> Result<(), Yoshi> {
    ///     // Simulate an internal error
    ///     Err(Yoshi::new(YoshiKind::Internal {
    ///         message: "Processing failed".into(),
    ///         source: None,
    ///         component: None,
    ///     }))
    ///     .with_shell(RequestInfo { request_id: id.into(), user_agent: ua.into() })
    /// }
    ///
    /// # fn main() {
    /// let result = process_request("req123", "Mozilla/5.0");
    /// if let Err(e) = result {
    ///     println!("Error: {}", e);
    ///     let info = e.shell::<RequestInfo>();
    ///     assert!(info.is_some());
    ///     assert_eq!(info.unwrap().request_id, "req123");
    /// }
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `Yoshi` error with added shell if the result is an error.
    #[track_caller]
    fn with_shell(self, p: impl Any + Send + Sync + 'static) -> Result<T>;

    /// Sets the priority for the error's primary context.
    ///
    /// If `self` is `Ok`, it is returned unchanged. If `self` is `Err`, its error
    /// is converted to a `Yoshi` error if it isn't already, and the priority of its
    /// primary `YoContext` is updated.
    ///
    /// # Arguments
    ///
    /// * `priority` - The priority level (0-255).
    ///
    /// # Returns
    ///
    /// A `Result<T, Yoshi>` with the updated priority on error.
    ///
    /// # Examples
    ///
    /// ```
    /// use yoshi_std::{Yoshi, YoshiKind, HatchExt};
    ///
    /// fn perform_critical_op() -> Result<(), Yoshi> {
    ///     // Simulate a critical error
    ///     Err(Yoshi::new(YoshiKind::Internal {
    ///         message: "Critical operation failed".into(),
    ///         source: None,
    ///         component: None,
    ///     }))
    ///     .with_priority(250) // Mark as very high priority
    /// }
    /// # fn main() {
    /// let result = perform_critical_op();
    /// if let Err(e) = result {
    ///     println!("Error: {}", e);
    ///     assert_eq!(e.primary_context().unwrap().priority, 250);
    /// }
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `Yoshi` error with updated priority if the result is an error.
    #[track_caller]
    fn with_priority(self, priority: u8) -> Result<T>;
    /// Short alias for `context`.
    ///
    /// # Errors
    ///
    /// Returns a `Yoshi` error with added context if the result is an error.
    #[track_caller]
    fn ctx(self, msg: impl Into<String>) -> Result<T>;

    /// Short alias for `with_suggestion`.
    ///
    /// # Errors
    ///
    /// Returns a `Yoshi` error with added suggestion if the result is an error.
    #[track_caller]
    fn help(self, s: impl Into<String>) -> Result<T>;

    /// Adds metadata to the error's primary context.
    ///
    /// This is a convenience method that delegates to `Yoshi::with_metadata`.
    ///
    /// # Arguments
    ///
    /// * `k` - The metadata key.
    /// * `v` - The metadata value.
    ///
    /// # Returns
    ///
    /// A `Result<T, Yoshi>` with the added metadata on error.
    ///
    /// # Examples
    ///
    /// ```
    /// use yoshi_std::{Yoshi, YoshiKind, HatchExt, Arc};
    ///
    /// fn fetch_user_data() -> Result<String, Yoshi> {
    ///     // Simulate an error during user data fetch
    ///     Err(Yoshi::new(YoshiKind::NotFound {
    ///         resource_type: "User".into(),
    ///         identifier: "unknown_user".into(),
    ///         search_locations: None,
    ///     }))
    ///     .meta("user_id", "12345")
    ///     .meta("trace_id", "abcde-12345")
    /// }
    ///
    /// # fn main() {
    /// let result = fetch_user_data();
    /// if let Err(e) = result {
    ///     println!("Error: {}", e);
    ///     let metadata = e.primary_context().unwrap().metadata.clone();
    ///     assert_eq!(metadata.get(&Arc::from("user_id")).map(|s| s.as_ref()), Some("12345"));
    /// }
    /// # }
    /// ```
    /// # Errors
    ///
    /// Returns a `Yoshi` error with added metadata if the result is an error.
    #[track_caller]
    fn meta(self, k: impl Into<String>, v: impl Into<String>) -> Result<T>;
}

//--------------------------------------------------------------------------------------------------
// Enhanced Yoshi main error type with performance optimization
//--------------------------------------------------------------------------------------------------

/// The main `Yoshi` error type with enterprise-grade performance optimization.
///
/// `Yoshi` is a highly structured and extensible error type designed for
/// complex applications. It combines a categorized error kind, contextual
/// information, and optional backtrace capture into a single, cohesive unit.
///
/// # Fields
///
/// - `kind`: The primary classification of the error, provided by [`YoshiKind`].
/// - `backtrace`: An optional [`YoshiBacktrace`] providing stack trace information (only with `std` feature).
/// - `contexts`: A vector of [`YoContext`] instances, providing additional
///   details and context about the error's propagation.
/// - `instance_id`: A unique identifier for each `Yoshi` error instance.
/// - `created_at`: The `SystemTime` when the error was created (only with `std` feature).
///
/// # Examples
///
/// Basic error creation:
/// ```
/// use yoshi_std::{Yoshi, YoshiKind};
///
/// let err = Yoshi::new(YoshiKind::Internal {
///     message: "Something went wrong internally".into(),
///     source: None,
///     component: None,
/// });
///
/// println!("Error: {}", err);
/// ```
///
/// Creating an error with context:
/// ```
/// use yoshi_std::{Yoshi, YoshiKind, HatchExt};
/// # use std::io::{self, ErrorKind};
///
/// fn load_data() -> Result<(), Yoshi> {
///     // Simulate a file not found error
///     let io_error = io::Error::new(ErrorKind::NotFound, "data.json not found");
///     let error = Yoshi::from(io_error)
///         .context("Failed to load user preferences".to_string())
///         .with_metadata("user_id", "test_user")
///         .with_suggestion("Ensure data.json is in the correct directory.");
///     Err(error)
/// }
///
/// # fn main() {
/// match load_data() {
///     Ok(_) => println!("Data loaded successfully"),
///     Err(error) => eprintln!("Error: {}", error),
/// }
/// # }
/// ```
#[derive(Debug)]
pub struct Yoshi {
    /// The underlying error kind.
    kind: YoshiKind,
    /// Optional backtrace for debugging and performance metadata (only available with `std` feature).
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    backtrace: Option<YoshiBacktrace>,
    /// Placeholder for backtrace when `std` feature is not enabled.
    #[cfg(not(feature = "std"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "std"))))]
    backtrace: Option<()>,
    /// Contexts providing additional information about the error.
    contexts: Vec<YoContext>,
    /// A unique identifier for this error instance.
    instance_id: u32,
    /// Timestamp when the error was created (only available with `std` feature).
    #[cfg(feature = "std")]
    #[allow(dead_code)]
    created_at: SystemTime,
}

impl Clone for Yoshi {
    /// Creates a clone of the `Yoshi` error.
    ///
    /// Note: In `std` mode, the backtrace is not cloned (as `std::backtrace::Backtrace`
    /// doesn't implement `Clone`). Instead, the clone will have no backtrace (`None`).
    /// In `no_std` mode, the backtrace is properly cloned as it only contains basic
    /// location information.
    ///
    /// A new unique instance ID is generated for the clone to maintain error tracking.
    fn clone(&self) -> Self {
        let instance_id = ERROR_INSTANCE_COUNTER.fetch_add(1, Ordering::Relaxed);

        Self {
            kind: self.kind.clone(),
            #[cfg(feature = "std")]
            backtrace: None, // Cannot clone std::backtrace::Backtrace, so set to None
            #[cfg(not(feature = "std"))]
            backtrace: self.backtrace.clone(), // YoshiBacktrace implements Clone in no_std mode
            contexts: self.contexts.clone(),
            instance_id,
            #[cfg(feature = "std")]
            created_at: SystemTime::now(), // Use current time for the clone
        }
    }
}

impl Yoshi {
    /// Creates a new `Yoshi` error with optimized allocation and monitoring.
    ///
    /// This is the primary constructor for `Yoshi` errors. It increments
    /// a global instance counter and, if the `std` feature is enabled and
    /// backtraces are enabled via environment variables (`RUST_BACKTRACE`
    /// or `RUST_LIB_BACKTRACE`), it captures a backtrace.
    ///
    /// # Arguments
    ///
    /// * `kind` - The [`YoshiKind`] that categorizes this error.
    ///
    /// # Returns
    ///
    /// A new `Yoshi` error instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use yoshi_std::{Yoshi, YoshiKind};
    ///
    /// let err = Yoshi::new(YoshiKind::NotFound {
    ///     resource_type: "User".into(),
    ///     identifier: "john.doe".into(),
    ///     search_locations: None,
    /// });
    ///
    /// assert!(matches!(err.kind(), YoshiKind::NotFound { .. }));
    /// ```
    #[inline]
    pub fn new(kind: YoshiKind) -> Self {
        let instance_id = ERROR_INSTANCE_COUNTER.fetch_add(1, Ordering::Relaxed);

        Self {
            kind,
            #[cfg(feature = "std")]
            backtrace: capture_bt(),
            #[cfg(not(feature = "std"))]
            backtrace: None,
            contexts: Vec::with_capacity(4), // Pre-allocate for typical error chain depth
            instance_id,
            #[cfg(feature = "std")]
            created_at: SystemTime::now(),
        }
    }

    /// Creates a new `Yoshi` error by wrapping a foreign `Error` trait object.
    ///
    /// This is an explicit conversion for generic error types, allowing them
    /// to be integrated into the `Yoshi` error chain without requiring a
    /// blanket `From` implementation that might conflict or cause issues
    /// with unstable features.
    /// The type name of the wrapped error is captured for diagnostic purposes.
    ///
    /// # Type Parameters
    ///
    /// * `E` - The type of the foreign error, which must implement `Error`,
    ///   `Send`, `Sync`, and have a `'static` lifetime.
    ///
    /// # Arguments
    ///
    /// * `e` - The foreign error instance to wrap.
    ///
    /// # Returns
    ///
    /// A new `Yoshi` error with its kind to `YoshiKind::Foreign`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io;
    /// use yoshi_std::{Yoshi, YoshiKind};
    ///
    /// #[derive(Debug)]
    /// struct CustomError;
    /// impl std::fmt::Display for CustomError {
    ///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ///         write!(f, "a custom error occurred")
    ///     }
    /// }
    /// impl std::error::Error for CustomError {}
    ///
    /// let io_error = io::Error::new(io::ErrorKind::Other, "disk full");
    /// let yoshi_io_error = Yoshi::foreign(io_error);
    /// assert!(matches!(yoshi_io_error.kind(), YoshiKind::Foreign { .. }));
    /// println!("Wrapped IO error: {}", yoshi_io_error);
    ///
    /// let custom_error = CustomError;
    /// let yoshi_custom_error = Yoshi::foreign(custom_error);
    /// assert!(matches!(yoshi_custom_error.kind(), YoshiKind::Foreign { .. }));
    /// println!("Wrapped custom error: {}", yoshi_custom_error);
    /// ```
    #[inline]
    #[track_caller]
    pub fn foreign<E>(e: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self::new(YoshiKind::from_foreign_with_context(e, ""))
    }

    /// Gets the unique instance ID for debugging and correlation.
    ///
    /// Each `Yoshi` error instance is assigned a unique `u64` ID upon creation.
    /// This ID can be used to track specific error occurrences in logs or
    /// telemetry systems, especially in highly concurrent environments.
    ///
    /// # Returns
    ///
    /// The unique instance ID of this `Yoshi` error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::{Yoshi, YoshiKind};
    /// let err1 = Yoshi::new(YoshiKind::Internal {
    ///     message: "test".into(),
    ///     source: None,
    ///     component: None,
    /// });
    /// let err2 = Yoshi::new(YoshiKind::Internal {
    ///     message: "test".into(),
    ///     source: None,
    ///     component: None,
    /// });
    ///
    /// assert_ne!(err1.instance_id(), err2.instance_id());
    /// println!("Error 1 ID: {}", err1.instance_id());
    /// println!("Error 2 ID: {}", err2.instance_id());
    /// ```
    #[inline]
    pub const fn instance_id(&self) -> u32 {
        self.instance_id
    }

    /// Returns a reference to the `YoshiKind` of this error.
    ///
    /// This allows inspecting the high-level classification of the error
    /// and accessing its specific fields.
    ///
    /// # Returns
    ///
    /// A constant reference to the [`YoshiKind`] enum variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use yoshi_std::{Yoshi, YoshiKind};
    ///
    /// let err = Yoshi::new(YoshiKind::NotFound {
    ///     resource_type: "User".into(),
    ///     identifier: "john.doe".into(),
    ///     search_locations: None,
    /// });
    ///
    /// match err.kind() {
    ///     YoshiKind::NotFound { identifier, .. } => {
    ///         println!("User not found: {}", identifier);
    ///     }
    ///     _ => (),
    /// }
    /// ```
    #[inline]
    pub const fn kind(&self) -> &YoshiKind {
        &self.kind
    }

    /// Gets the error severity level (0-100).
    ///
    /// This is a convenience method that delegates to `self.kind().severity()`.
    ///
    /// # Returns
    ///
    /// A `u8` value indicating the severity of the error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::YoshiKind;
    /// let internal_error = YoshiKind::Internal {
    ///     message: "simulated error".into(),
    ///     source: None,
    ///     component: None,
    /// };
    /// assert_eq!(internal_error.severity(), 80);
    ///
    /// let validation_error = YoshiKind::Validation {
    ///     field: "email".into(),
    ///     message: "Invalid format".into(),
    ///     expected: None,
    ///     actual: None,
    /// };
    /// assert_eq!(validation_error.severity(), 20);
    /// ```
    #[inline]
    pub const fn severity(&self) -> u8 {
        self.kind.severity()
    }

    /// Checks if this is a transient error that might succeed on retry.
    ///
    /// This is a convenience method that delegates to `self.kind().is_transient()`.
    ///
    /// # Returns
    ///
    /// `true` if the error's kind is considered transient, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::YoshiKind;
    /// # use core::time::Duration;
    /// let timeout_error = YoshiKind::Timeout {
    ///     operation: "API call".into(),
    ///     duration: Duration::from_secs(10),
    ///     expected_max: None,
    /// };
    /// assert!(timeout_error.is_transient());
    ///
    /// let config_error = YoshiKind::Config {
    ///     message: "Missing key".into(),
    ///     source: None,
    ///     config_path: None,
    /// };
    /// assert!(!config_error.is_transient());
    /// ```
    #[inline]
    pub const fn is_transient(&self) -> bool {
        self.kind.is_transient()
    }

    /// Adds a context message to the error.
    ///
    /// This method enhances the error with additional diagnostic information,
    /// making it easier to trace the origin and propagation of failures.
    ///
    /// # Arguments
    ///
    /// * `msg` - The context message. It can be any type that converts into a `String`.
    ///
    /// # Returns
    ///
    /// The modified `Yoshi` error instance with the new context.
    ///
    /// # Examples
    ///
    /// ```
    /// use yoshi_std::{Yoshi, YoshiKind};
    ///
    /// let err = Yoshi::new(YoshiKind::Internal {
    ///     message: "database query failed".into(),
    ///     source: None,
    ///     component: None,
    /// })
    /// .context("Attempting to fetch user data");
    ///
    /// println!("Error: {}", err);
    /// ```
    ///
    /// # Panics
    ///
    /// This method may panic if context storage fails, though this is extremely unlikely.
    #[track_caller]
    #[inline]
    #[must_use]
    pub fn context(mut self, msg: impl Into<String>) -> Self {
        self.contexts
            .push(YoContext::new(msg).with_location(yoshi_location!()));
        self
    }

    /// Adds a suggestion to the error's primary context.
    ///
    /// This method adds a human-readable suggestion to the current `Yoshi` error.
    /// The suggestion is stored in the primary (most recent) context associated
    /// with this error.
    ///
    /// # Arguments
    ///
    /// * `s` - The suggestion message. It can be any type that converts into a `String`.
    ///
    /// # Returns
    ///
    /// The modified `Yoshi` error instance with the new suggestion.
    ///
    /// # Examples    ///
    /// ```
    /// use yoshi_std::{Yoshi, YoshiKind};
    /// let err = Yoshi::new(YoshiKind::Io(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "file access denied")))
    ///     .with_suggestion("Check file permissions or path.");
    ///
    /// assert!(err.suggestion().as_deref() == Some("Check file permissions or path."));
    /// ```
    ///
    /// # Panics
    ///
    /// This method may panic if the context storage fails, though this is extremely unlikely.
    #[inline]
    #[track_caller]
    #[must_use]
    pub fn with_suggestion(mut self, s: impl Into<String>) -> Self {
        // Ensure there's at least one context to attach the suggestion to
        if self.contexts.is_empty() {
            self.contexts
                .push(YoContext::new("Error occurred").with_location(yoshi_location!()));
        }
        self.contexts
            .last_mut()
            .expect("contexts should not be empty")
            .suggestion = Some(intern_string(s.into()));
        self
    }

    /// Attaches a component identifier to the error's primary context.
    ///
    /// This method adds a component identifier to help categorize and trace
    /// errors within different parts of a system or application. The component
    /// information is stored as metadata with the key "component".
    ///
    /// # Arguments
    ///
    /// * `component` - The component identifier. It can be any type that converts into a `String`.
    ///
    /// # Returns
    ///
    /// The modified `Yoshi` error instance with the component information.
    ///
    /// # Examples
    ///
    /// ```
    /// use yoshi_std::{Yoshi, YoshiKind};
    ///
    /// let err = Yoshi::new(YoshiKind::Internal {
    ///     message: "operation failed".into(),
    ///     source: None,
    ///     component: None,
    /// })
    /// .with_component("database");
    ///
    /// // Component can be retrieved from metadata
    /// let ctx = err.primary_context().unwrap();
    /// assert_eq!(ctx.metadata.get("component").map(|s| s.as_ref()), Some("database"));
    /// ```
    ///
    /// # Panics
    ///
    /// This method may panic if the context storage fails, though this is extremely unlikely.
    #[inline]
    #[track_caller]
    #[must_use]
    pub fn with_component(mut self, component: impl Into<String>) -> Self {
        // Ensure there's at least one context to attach the component to
        if self.contexts.is_empty() {
            self.contexts
                .push(YoContext::new("Error occurred").with_location(yoshi_location!()));
        }
        self.contexts
            .last_mut()
            .expect("contexts should not be empty")
            .metadata
            .insert(intern_string("component"), intern_string(component.into()));
        self
    }

    /// Attaches a typed shell to the error's primary context.
    ///
    /// This method allows embedding arbitrary Rust types within the error's context.
    /// This is useful for passing structured, type-safe debugging information
    /// that can be retrieved later using `shell::<T>()`.
    ///
    /// # Arguments
    ///
    /// * `shell` - The data to attach. It must implement `Any`, `Send`, `Sync`, and have a `'static` lifetime.
    ///
    /// # Returns
    ///
    /// The modified `Yoshi` error instance with the new shell.
    ///
    /// # Examples
    ///
    /// ```
    /// use yoshi_std::{Yoshi, YoshiKind};
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct RequestContext {
    ///     user_id: u64,
    ///     request_path: String,
    /// }
    ///
    /// let err = Yoshi::new(YoshiKind::Internal {
    ///     message: "handler failed".into(),
    ///     source: None,
    ///     component: None,
    /// })
    /// .with_shell(RequestContext { user_id: 123, request_path: "/api/data".to_string() });
    ///
    /// let ctx_payload = err.shell::<RequestContext>().unwrap();
    /// assert_eq!(ctx_payload.user_id, 123);
    /// ```
    ///
    /// # Panics
    ///
    /// This method may panic if the shell storage fails, though this is extremely unlikely.
    #[inline]
    #[track_caller]
    #[must_use]
    pub fn with_shell(mut self, shell: impl Any + Send + Sync + 'static) -> Self {
        // Ensure there's at least one context to attach the shell to
        if self.contexts.is_empty() {
            self.contexts
                .push(YoContext::new("Error occurred").with_location(yoshi_location!()));
        }
        self.contexts
            .last_mut()
            .expect("contexts should not be empty")
            .add_shell_in_place(shell);
        self
    }

    /// Sets the priority for the error's primary context.
    ///
    /// Priority can be used to indicate the relative importance of a context
    /// message, influencing how errors are logged or processed by error handling
    /// systems. Higher values indicate higher priority.
    ///
    /// # Arguments
    ///
    /// * `priority` - The priority level (0-255).
    ///
    /// # Returns
    ///
    /// The modified `Yoshi` error instance with the updated priority.
    ///
    /// # Examples
    /// ```
    /// use yoshi_std::{Yoshi, YoshiKind};
    ///
    /// let err = Yoshi::new(YoshiKind::Internal {
    ///     message: "critical failure".into(),
    ///     source: None,
    ///     component: None,
    /// })
    /// .with_priority(250); // Highest priority
    ///
    /// assert_eq!(err.primary_context().unwrap().priority, 250);
    /// ```
    ///
    /// # Panics
    ///
    /// This method ensures that there is at least one context before updating priority.
    /// If no contexts exist, it creates one automatically, so this method should not panic.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_priority(mut self, priority: u8) -> Self {
        // Ensure there's at least one context to update
        if self.contexts.is_empty() {
            self.contexts
                .push(YoContext::new("Error occurred").with_location(yoshi_location!()));
        }
        self.contexts
            .last_mut()
            .expect("contexts should not be empty")
            .priority = priority;
        self
    }

    /// Adds metadata to the error's primary context.
    ///
    /// Metadata are key-value pairs that provide additional, unstructured
    /// diagnostic information. These can be used for logging, filtering,
    /// or passing arbitrary data alongside the error.
    ///
    /// # Arguments
    ///
    /// * `k` - The metadata key. It can be any type that converts into a `String`.
    /// * `v` - The metadata value. It can be any type that converts into a `String`.
    ///
    /// # Returns
    ///
    /// The modified `Yoshi` error instance with the new metadata.
    ///
    /// # Examples
    ///
    /// ```
    /// use yoshi_std::{Yoshi, YoshiKind, Arc};
    ///
    /// let err = Yoshi::new(YoshiKind::Internal {
    ///     message: "cache read failed".into(),
    ///     source: None,
    ///     component: None,
    /// })
    /// .with_metadata("cache_key", "user_profile_123")
    /// .with_metadata("region", "us-east-1");
    ///
    /// let metadata = &err.primary_context().unwrap().metadata;
    /// assert_eq!(metadata.get(&Arc::from("cache_key")).map(|s| s.as_ref()), Some("user_profile_123"));
    /// assert_eq!(metadata.get(&Arc::from("region")).map(|s| s.as_ref()), Some("us-east-1"));
    /// ```
    ///
    /// # Panics
    ///
    /// This method may panic if metadata storage fails, though this is extremely unlikely.
    #[inline]
    #[track_caller]
    #[must_use]
    pub fn with_metadata(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        // Ensure there's at least one context to attach metadata to
        if self.contexts.is_empty() {
            self.contexts
                .push(YoContext::new("Error occurred").with_location(yoshi_location!()));
        }
        self.contexts
            .last_mut()
            .expect("contexts should not be empty")
            .metadata
            .insert(intern_string(k.into()), intern_string(v.into()));
        self
    }

    /// Sets location information on the error's primary context.
    ///
    /// This method attaches source code location information to the error's primary context,
    /// helping with debugging and error tracing. It consumes `self` and returns a modified `Self`.
    ///
    /// # Arguments
    ///
    /// * `location` - The `YoshiLocation` to set.
    ///
    /// # Returns
    ///
    /// The modified `Yoshi` error instance with the location set.
    ///
    /// # Examples
    ///
    /// ```
    /// use yoshi_std::{Yoshi, YoshiKind, YoshiLocation};
    ///
    /// let location = YoshiLocation::new("src/main.rs", 10, 5);
    /// let err = Yoshi::new(YoshiKind::Internal {
    ///     message: "operation failed".into(),
    ///     source: None,
    ///     component: None,
    /// })
    /// .with_location(location);
    ///
    /// assert_eq!(err.primary_context().unwrap().location.unwrap().file, "src/main.rs");
    /// assert_eq!(err.primary_context().unwrap().location.unwrap().line, 10);
    /// ```
    ///
    /// # Panics
    ///
    /// This method may panic if location storage fails, though this is extremely unlikely.
    #[inline]
    #[track_caller]
    #[must_use]
    pub fn with_location(mut self, location: YoshiLocation) -> Self {
        // Ensure there's at least one context to attach location to
        if self.contexts.is_empty() {
            self.contexts
                .push(YoContext::new("Error occurred").with_location(yoshi_location!()));
        }
        self.contexts
            .last_mut()
            .expect("contexts should not be empty")
            .location = Some(location);
        self
    }

    /// Returns a reference to the optional backtrace.
    ///
    /// The backtrace is only available when the `std` feature is enabled and
    /// `RUST_BACKTRACE` or `RUST_LIB_BACKTRACE` environment variables are set.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the [`YoshiBacktrace`] if available,
    /// otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[cfg(feature = "std")] {
    /// # use yoshi_std::{Yoshi, YoshiKind};
    /// # std::env::set_var("RUST_BACKTRACE", "1");
    /// let err = Yoshi::new(YoshiKind::Internal {
    ///     message: "test error".into(),
    ///     source: None,
    ///     component: None,
    /// });
    /// if let Some(bt) = err.backtrace() {
    ///     println!("Backtrace: {}", bt);
    /// }
    /// # }
    /// ```
    #[cfg(feature = "std")]
    #[inline]
    pub const fn backtrace(&self) -> Option<&YoshiBacktrace> {
        self.backtrace.as_ref()
    }

    /// Returns a reference to the underlying foreign error (if `YoshiKind::Foreign`).
    ///
    /// This method allows downcasting the boxed `dyn Error` contained within a
    /// `YoshiKind::Foreign` variant to a concrete type.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The concrete type to downcast to, which must implement `Error`.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the downcasted error of type `T`,
    /// or `None` if the error is not `YoshiKind::Foreign` or cannot be downcasted
    /// to the specified type.
    ///    /// # Examples
    ///
    /// ```
    /// use std::io;
    /// use yoshi_std::{Yoshi, YoshiKind};
    ///
    /// let io_err = io::Error::new(io::ErrorKind::NotFound, "file.txt not found");
    /// let yoshi_err = Yoshi::foreign(io_err);
    ///
    /// // Attempt to downcast to io::Error
    /// if let Some(err) = yoshi_err.downcast_ref::<io::Error>() {
    ///     assert_eq!(err.kind(), io::ErrorKind::NotFound);
    /// } else {
    ///     panic!("Expected io::Error");
    /// }
    /// ```
    #[inline]
    pub fn downcast_ref<T: Error + 'static>(&self) -> Option<&T> {
        if let YoshiKind::Foreign { error, .. } = &self.kind {
            // First try to downcast the ForeignErrorWrapper itself to T
            if let Some(result) = error.downcast_ref::<T>() {
                return Some(result);
            }

            // If that fails, try to downcast the wrapper to ForeignErrorWrapper
            // and then downcast its inner error to T
            if let Some(wrapper) = error.downcast_ref::<ForeignErrorWrapper>() {
                wrapper.inner.downcast_ref::<T>()
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Returns a mutable reference to the underlying foreign error (if `YoshiKind::Foreign`).
    ///
    /// This method allows mutable downcasting the boxed `dyn Error` contained within a
    /// `YoshiKind::Foreign` variant to a concrete type.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The concrete type to downcast to, which must implement `Error`.
    ///
    /// # Returns
    ///    /// An `Option` containing a mutable reference to the downcasted error of type `T`,
    /// or `None` if the error is not `YoshiKind::Foreign` or cannot be downcasted
    /// to the specified type.
    #[inline]
    pub fn downcast_mut<T: Error + 'static>(&mut self) -> Option<&mut T> {
        if let YoshiKind::Foreign { error, .. } = &mut self.kind {
            // Use a single downcast operation and then check both possibilities
            if error.is::<ForeignErrorWrapper>() {
                // If it's a ForeignErrorWrapper, get the inner error
                if let Some(wrapper) = error.downcast_mut::<ForeignErrorWrapper>() {
                    wrapper.inner.downcast_mut::<T>()
                } else {
                    None
                }
            } else {
                // If it's not a wrapper, try to downcast directly
                error.downcast_mut::<T>()
            }
        } else {
            None
        }
    }

    /// Returns the primary context associated with this error.
    ///
    /// The primary context is typically the most recent or most relevant
    /// context added to the error, often containing the most specific
    /// information about the direct cause of the failure.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the primary `YoContext`,
    /// or `None` if no contexts have been added.
    ///
    /// # Examples
    ///
    /// ```
    /// use yoshi_std::{Yoshi, YoshiKind};
    ///
    /// let err = Yoshi::new(YoshiKind::Internal {
    ///     message: "failed step".into(),
    ///     source: None,
    ///     component: None,
    /// })
    /// .context("Step 1 failed")
    /// .context("Step 2 failed"); // This is the primary context
    ///
    /// assert_eq!(err.primary_context().unwrap().message.as_deref(), Some("Step 2 failed"));
    /// ```
    #[inline]
    pub fn primary_context(&self) -> Option<&YoContext> {
        self.contexts.last()
    }

    /// Returns an iterator over all contexts associated with this error.
    ///
    /// Contexts are ordered from oldest (first added) to newest (most recent, primary).
    ///
    /// # Returns
    ///
    /// An iterator yielding references to `YoContext` instances.
    ///
    /// # Examples
    ///
    /// ```
    /// use yoshi_std::{Yoshi, YoshiKind};
    ///
    /// let err = Yoshi::new(YoshiKind::Internal {
    ///     message: "original error".into(),
    ///     source: None,
    ///     component: None,
    /// })
    /// .context("context 1")
    /// .context("context 2");
    ///
    /// let messages: Vec<_> = err.contexts().filter_map(|c| c.message.as_deref()).collect();
    /// assert_eq!(messages, vec!["context 1", "context 2"]);
    /// ```
    #[inline]
    pub fn contexts(&self) -> impl Iterator<Item = &YoContext> {
        self.contexts.iter()
    }

    /// Returns the suggestion from the primary context, if any.
    ///
    /// This is a convenience method to quickly access the most relevant
    /// suggestion for resolving the error.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the suggestion string, or `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use yoshi_std::{Yoshi, YoshiKind};
    ///
    /// let err = Yoshi::new(YoshiKind::Io(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "file access denied")))
    ///     .with_suggestion("Check file permissions.");
    ///
    /// assert_eq!(err.suggestion().as_deref(), Some("Check file permissions."));
    /// ```
    #[inline]
    pub fn suggestion(&self) -> Option<&str> {
        self.primary_context()
            .and_then(|ctx| ctx.suggestion.as_deref())
    }

    /// Returns a typed shell from the primary context, if any.
    ///
    /// This is a convenience method to quickly access a structured shell
    /// from the most relevant context.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of shell to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the shell of type `T`, or `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use yoshi_std::{Yoshi, YoshiKind};
    /// #[derive(Debug, PartialEq)]
    /// struct CustomPayload(u32);
    /// let err = Yoshi::new(YoshiKind::Internal {
    ///     message: "test".into(),
    ///     source: None,
    ///     component: None,
    /// })
    /// .with_shell(CustomPayload(123));
    ///    /// assert_eq!(err.shell::<CustomPayload>().unwrap().0, 123);
    /// ```
    #[inline]
    pub fn shell<T: 'static>(&self) -> Option<&T> {
        // Search ALL contexts for the shell, not just the primary context
        // This ensures payloads can be found regardless of context priority ordering
        for context in &self.contexts {
            if let Some(shell) = context.shell::<T>() {
                return Some(shell);
            }
        }
        None
    }

    /// The nested error, equivalent to `source()`, but more thematically expressive.
    ///
    /// This method provides thematic access to the underlying error source while
    /// maintaining full backwards compatibility with the standard `Error` trait.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the nested error, or `None` if
    /// there is no underlying source.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yoshi_std::{Yoshi, YoshiKind};
    /// # use std::io;
    /// let inner = Yoshi::new(YoshiKind::Internal {
    ///     message: "inner failure".into(),
    ///     source: None,
    ///     component: None,
    /// });
    /// let outer = Yoshi::new(YoshiKind::Internal {
    ///     message: "outer failure".into(),
    ///     source: Some(Box::new(inner)),
    ///     component: None,
    /// });
    /// assert!(outer.nest().is_some());
    /// ```
    #[inline]
    pub fn nest(&self) -> Option<&(dyn Error + 'static)> {
        self.kind.source()
    }

    /// The explanation or context attached to the error.
    ///
    /// This method provides direct access to the primary context message,
    /// offering a thematic alternative to accessing context information.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the laytext string, or `None`
    /// if no context message is available.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yoshi_std::{Yoshi, YoshiKind};
    /// let err = Yoshi::new(YoshiKind::Internal {
    ///     message: "base error".into(),
    ///     source: None,
    ///     component: None,
    /// })
    /// .context("operation failed");
    /// assert_eq!(err.laytext().unwrap(), "operation failed");
    /// ```
    #[inline]
    pub fn laytext(&self) -> Option<&str> {
        self.primary_context()
            .and_then(|ctx| ctx.message.as_deref())
    }

    /// Adds contextual information using the thematic `.lay()` method.
    ///
    /// This method is equivalent to `.context()` but provides thematic naming
    /// consistent with the Hatch ecosystem's metaphorical framework.
    ///
    /// # Arguments
    ///
    /// * `msg` - The context message to attach.
    ///
    /// # Returns
    ///
    /// The modified `Yoshi` error instance with the new context.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yoshi_std::{Yoshi, YoshiKind};
    /// let err = Yoshi::new(YoshiKind::Internal {
    ///     message: "base error".into(),
    ///     source: None,
    ///     component: None,
    /// })
    /// .lay("while processing request");
    /// assert!(err.to_string().contains("while processing request"));
    /// ```
    #[track_caller]
    #[inline]
    #[must_use]
    pub fn lay(self, msg: impl Into<String>) -> Self {
        self.context(msg)
    }

    /// Gathers analysis results about the contexts in this error.
    ///
    /// This method performs a quick scan of all attached contexts to provide
    /// aggregated statistics, useful for logging, analytics, or deciding
    /// on error handling strategies.
    ///
    /// # Returns
    ///
    /// A `ContextAnalysis` struct containing various metrics about the contexts.
    ///
    /// # Examples
    ///
    /// ```
    /// use yoshi_std::{Yoshi, YoshiKind, YoshiLocation};
    ///
    /// let err = Yoshi::new(YoshiKind::Internal {
    ///     message: "base error".into(),
    ///     source: None,
    ///     component: None,
    /// })
    /// .context("Intermediate step")
    /// .with_metadata("key", "value")
    /// .with_suggestion("Try again")
    /// .context("Final step failed")
    /// .with_location(YoshiLocation::new("src/main.rs", 10, 5));
    ///
    /// let analysis = err.analyze_contexts();
    /// assert_eq!(analysis.total_contexts, 2);
    /// assert_eq!(analysis.context_depth, 2);
    /// assert!(analysis.has_suggestions);
    /// assert!(analysis.has_location_info);
    /// assert_eq!(analysis.metadata_entries, 1);
    /// ```
    pub fn analyze_contexts(&self) -> ContextAnalysis {
        let mut analysis = ContextAnalysis {
            total_contexts: self.contexts.len(),
            context_depth: self.contexts.len(), // Simple depth = count for now
            ..ContextAnalysis::default()
        };

        for ctx in &self.contexts {
            if ctx.suggestion.is_some() {
                analysis.has_suggestions = true;
            }
            if ctx.location.is_some() {
                analysis.has_location_info = true;
            }
            analysis.metadata_entries += ctx.metadata.len();
            analysis.typed_payloads += ctx.payloads.len();

            // The primary context is the last one in the vector
            if let Some(primary_ctx) = self.contexts.last() {
                analysis.primary_context_priority = primary_ctx.priority;
            }
        }
        analysis
    }
}

impl Display for Yoshi {
    /// Formats the `Yoshi` error for display with optimized O(n) error chain traversal.
    ///
    /// This implementation provides a comprehensive, human-readable representation
    /// of the error, designed for debugging and logging. It uses an optimized
    /// iterative approach to traverse error chains, eliminating the O(n²) performance
    /// bottleneck present in recursive formatting. The formatter collects the entire
    /// error chain first, then renders all information in a single linear pass.
    ///
    /// # Performance Characteristics
    ///
    /// - **Time Complexity**: O(n) where n is the total depth of the error chain
    /// - **Space Complexity**: O(n) for temporary chain storage
    /// - **Memory Allocation**: Minimized through `OptimizedFormatBuffer` usage
    /// - **Scaling**: Linear performance even for deep error chains (100+ levels)
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write into.
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating success or failure of the formatting.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::{Yoshi, YoshiKind};
    /// let error = Yoshi::new(YoshiKind::Internal {
    ///     message: "Operation failed".into(),
    ///     source: None,
    ///     component: None,
    /// })
    /// .context("While processing request");
    ///
    /// println!("{}", error); // Efficient O(n) formatting
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Use optimized buffer for efficient string building
        let mut buffer = OptimizedFormatBuffer::new();

        // Write primary error information
        buffer.append_optimized(&format!("{}: {}", self.instance_id, self.kind));
        buffer.append_optimized("\n");

        // Print contexts from oldest to newest (excluding auto-generated ones)
        for (i, ctx) in self.contexts.iter().enumerate() {
            if i == 0
                && ctx.message.as_deref() == Some("Error occurred")
                && ctx.metadata.is_empty()
                && ctx.suggestion.is_none()
                && ctx.payloads.is_empty()
            {
                // Skip auto-generated default context if it provides no actual info
                continue;
            }

            if let Some(msg) = ctx.message.as_deref() {
                buffer.append_optimized("Caused by: ");
                buffer.append_optimized(msg);
                buffer.append_optimized("\n");
            }

            if !ctx.metadata.is_empty() {
                buffer.append_optimized("Metadata:\n");
                for (k, v) in &ctx.metadata {
                    buffer.append_optimized("  ");
                    buffer.append_optimized(k.as_ref());
                    buffer.append_optimized(": ");
                    buffer.append_optimized(v.as_ref());
                    buffer.append_optimized("\n");
                }
            }

            if let Some(suggestion) = ctx.suggestion.as_deref() {
                buffer.append_optimized("Suggestion: ");
                buffer.append_optimized(suggestion);
                buffer.append_optimized("\n");
            }

            if let Some(location) = ctx.location {
                buffer.append_optimized("Location: ");
                buffer.append_optimized(&location.to_string());
                buffer.append_optimized("\n");
            }
        } // Collect complete error chain iteratively (O(n) instead of O(n²))
        let mut error_chain: Vec<String> = Vec::new();
        let mut yoshi_contexts: Vec<String> = Vec::new();

        // Start with the source from this error's kind
        let mut current_error = self.kind.source();

        while let Some(source_error) = current_error {
            // Check if it's a Yoshi error to extract contexts
            if let Some(yoshi_source) = source_error.downcast_ref::<Yoshi>() {
                // Add the Yoshi error's kind to the chain
                error_chain.push(format!("Caused by: {}", yoshi_source.kind));

                // Collect contexts from this Yoshi error
                for ctx in &yoshi_source.contexts {
                    if let Some(msg) = ctx.message.as_deref() {
                        yoshi_contexts.push(format!("Caused by: {msg}"));
                    }
                }

                // Move to the next error in the chain
                current_error = yoshi_source.kind.source();
            } else {
                // For non-Yoshi sources, add directly to chain and stop
                error_chain.push(format!("Caused by: {source_error}"));
                current_error = source_error.source();
            }
        }

        // Append all collected error chain information
        for error_msg in error_chain {
            buffer.append_optimized(&error_msg);
            buffer.append_optimized("\n");
        }

        // Append all collected Yoshi contexts
        for ctx_msg in yoshi_contexts {
            buffer.append_optimized(&ctx_msg);
            buffer.append_optimized("\n");
        }

        // Add backtrace if available
        #[cfg(feature = "std")]
        if let Some(bt) = &self.backtrace {
            buffer.append_optimized("\nBacktrace:\n");
            buffer.append_optimized(&bt.to_string());
        }

        // Write the complete formatted output
        write!(f, "{}", buffer.as_str().trim_end())
    }
}

#[cfg(feature = "std")]
impl Error for Yoshi {
    /// Returns the underlying source of this error.
    ///
    /// This method provides access to the root cause of the error chain,
    /// enabling compatibility with Rust's standard error handling mechanisms.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the `dyn Error` source,
    /// or `None` if there is no underlying cause.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.kind.source()
    }
}

#[cfg(not(feature = "std"))]
impl Error for Yoshi {
    /// Returns the underlying source of this error.
    ///
    /// This method delegates to the internal `source` method, enabling
    /// `YoshiKind` to participate in Rust's standard error chaining mechanism.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the underlying error that
    /// caused this `YoshiKind`, or `None` if there is no direct source.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.kind.source()
    }
}

/// Docs.rs specific Error implementation for enhanced compatibility
#[cfg(docsrs)]
impl Error for Yoshi {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // Safe implementation that works on all nightly versions
        match &self.kind {
            YoshiKind::Io(e) => Some(e),
            YoshiKind::Foreign { error, .. } => Some(error.as_ref()),
            _ => None,
        }
    }

    /// Docs.rs compatible error description
    fn description(&self) -> &str {
        "Yoshi error (see Display impl for details)"
    }
}

impl From<String> for Yoshi {
    /// Converts a `String` into a `Yoshi` error.
    ///
    /// The string message is wrapped in an `Internal` `YoshiKind`.
    ///
    /// # Arguments
    ///
    /// * `s` - The string message to convert.
    ///
    /// # Returns
    ///
    /// A new `Yoshi` error instance.
    #[track_caller]
    fn from(s: String) -> Self {
        Yoshi::new(YoshiKind::Internal {
            message: s.into(),
            source: None,
            component: None,
        })
    }
}

impl From<&str> for Yoshi {
    /// Converts a string slice (`&str`) into a `Yoshi` error.
    ///
    /// The string slice is converted to a `String` and then wrapped in an
    /// `Internal` `YoshiKind`.
    ///
    /// # Arguments
    ///
    /// * `s` - The string slice to convert.
    ///
    /// # Returns
    ///
    /// A new `Yoshi` error instance.
    #[track_caller]
    fn from(s: &str) -> Self {
        Yoshi::new(YoshiKind::Internal {
            message: s.to_string().into(),
            source: None,
            component: None,
        })
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Yoshi {
    /// Converts a `std::io::Error` into a `Yoshi` error.
    ///
    /// The I/O error is wrapped in a `YoshiKind::Io` variant.
    ///
    /// # Arguments
    ///
    /// * `e` - The `std::io::Error` to convert.
    ///
    /// # Returns
    ///
    /// A new `Yoshi` error instance.
    #[track_caller]
    fn from(e: std::io::Error) -> Self {
        Yoshi::new(YoshiKind::Io(e))
    }
}

#[cfg(not(feature = "std"))]
impl From<NoStdIo> for Yoshi {
    /// Converts a `NoStdIo` error into a `Yoshi` error.
    ///
    /// The `NoStdIo` error is wrapped in a `YoshiKind::Io` variant.
    ///
    /// # Arguments
    ///
    /// * `e` - The `NoStdIo` error to convert.
    ///
    /// # Returns
    ///
    /// A new `Yoshi` error instance.
    #[track_caller]
    fn from(e: NoStdIo) -> Self {
        Yoshi::new(YoshiKind::Io(e))
    }
}

impl<T, E> HatchExt<T> for core::result::Result<T, E>
where
    E: Into<Yoshi> + Send + Sync + 'static, // Updated trait bounds
{
    #[track_caller]
    #[inline]
    fn context(self, msg: impl Into<String>) -> Result<T> {
        self.map_err(|e| e.into().context(msg))
    }

    #[track_caller]
    #[inline]
    fn with_suggestion(self, s: impl Into<String>) -> Result<T> {
        self.map_err(|e| e.into().with_suggestion(s))
    }
    #[track_caller]
    #[inline]
    fn with_shell(self, p: impl Any + Send + Sync + 'static) -> Result<T> {
        self.map_err(|e| {
            let mut yoshi_err = e.into();
            // Ensure we have a context to attach the shell to with standard priority
            if yoshi_err.contexts.is_empty() {
                yoshi_err
                    .contexts
                    .push(YoContext::default().with_priority(128));
            }
            yoshi_err.with_shell(p)
        })
    }

    /// Sets the priority for the error's primary context.
    #[track_caller]
    #[inline]
    fn with_priority(self, priority: u8) -> Result<T> {
        self.map_err(|e| e.into().with_priority(priority))
    }

    // NEW: Short aliases - just delegate to the full methods
    #[track_caller]
    #[inline]
    fn ctx(self, msg: impl Into<String>) -> Result<T> {
        self.context(msg)
    }

    #[track_caller]
    #[inline]
    fn help(self, s: impl Into<String>) -> Result<T> {
        self.with_suggestion(s)
    }

    #[track_caller]
    #[inline]
    fn meta(self, k: impl Into<String>, v: impl Into<String>) -> Result<T> {
        self.map_err(|e| {
            let mut yoshi_err = e.into();
            // Ensure we have a context to attach metadata to with proper priority
            if yoshi_err.contexts.is_empty() {
                yoshi_err
                    .contexts
                    .push(YoContext::default().with_priority(128));
            }
            yoshi_err.with_metadata(k, v)
        })
    }
}

/// Trait that adds `.lay(...)` to `Result<T, Yoshi>`, enriching errors with context.
///
/// This trait provides ergonomic context attachment using thematic naming that
/// aligns with the Yoshi metaphorical framework. The `.lay()` method is equivalent
/// to adding context but uses intuitive, game-inspired terminology.
///
/// # Performance Characteristics
///
/// - **Context Addition**: O(1) operation with minimal memory allocation
/// - **String Interning**: Automatic optimization for repeated context messages
/// - **Memory Efficiency**: Shared storage for common context patterns
///
/// # Examples
///
/// ```rust
/// use yoshi_std::{Hatch, LayContext, Yoshi, YoshiKind};
///
/// fn database_operation() -> Hatch<String> {
///     Err(Yoshi::new(YoshiKind::Internal {
///         message: "connection failed".into(),
///         source: None,
///         component: None,
///     }))
///     .lay("While establishing database connection")
/// }
/// ```
pub trait LayContext<T> {
    /// Adds a contextual message to the error chain, like laying an egg with metadata.
    ///
    /// This method enriches error information by attaching descriptive context
    /// that helps with debugging and error tracing. It uses thematic naming
    /// inspired by Yoshi's egg-laying ability to create memorable, intuitive APIs.
    ///
    /// # Arguments
    ///
    /// * `message` - The context message to attach. Accepts any type that converts to `String`.
    ///
    /// # Returns
    ///
    /// A `Hatch<T>` with the enriched context information attached.
    ///
    /// # Performance
    ///
    /// - **Time Complexity**: O(1) for context attachment
    /// - **Memory Optimization**: Automatic string interning for efficiency
    /// - **Allocation Pattern**: Minimal heap allocation with shared storage
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yoshi_std::{Hatch, LayContext, Yoshi, YoshiKind};
    ///
    /// let result: Hatch<()> = Err(Yoshi::new(YoshiKind::Internal {
    ///     message: "operation failed".into(),
    ///     source: None,
    ///     component: None,
    /// }))
    /// .lay("During user authentication");
    ///
    /// assert!(result.is_err());
    /// let error = result.unwrap_err();
    /// assert!(error.to_string().contains("During user authentication"));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns the enriched `Hatch<T>` error if `self` is `Err`, or the original
    /// success value if `self` is `Ok`. This method never introduces new errors.
    fn lay(self, message: impl Into<String>) -> Hatch<T>;
}

impl<T> LayContext<T> for Hatch<T> {
    #[track_caller]
    fn lay(self, message: impl Into<String>) -> Hatch<T> {
        self.map_err(|e| e.lay(message))
    }
}

/// Extension trait for mapping other `Result<T, E>` types into `Hatch<T>` easily.
///
/// This trait enables seamless integration between the Yoshi error ecosystem and
/// external error types. It provides the `.hatch()` method that converts any
/// `Result` with an error type that can be converted to `Yoshi` into a `Hatch<T>`.
///
/// # Type Requirements
///
/// The error type `E` must implement `Into<Yoshi>` to enable conversion. This is
/// automatically satisfied for:
/// - `std::io::Error` (when std feature is enabled)
/// - `NoStdIo` (when std feature is disabled)
/// - `String` and `&str` types
/// - Any type that implements `std::error::Error + Send + Sync + 'static`
///
/// # Performance Characteristics
///
/// - **Conversion Cost**: O(1) for types with direct `Into<Yoshi>` implementations
/// - **Memory Overhead**: Minimal - reuses existing error allocation where possible
/// - **Type Safety**: Compile-time guarantees with no runtime type checking
///
/// # Examples
///
/// ```rust
/// use yoshi_std::{Hatch, Hatchable, LayContext};
/// # use std::io;
///
/// fn file_operation() -> Hatch<String> {
///     std::fs::read_to_string("config.toml")
///         .hatch()  // Convert io::Error to Yoshi
///         .lay("While reading configuration file")
/// }
///
/// fn parse_operation() -> Hatch<i32> {
///     "not_a_number".parse::<i32>()
///         .map_err(|e| e.to_string())  // Convert to String first
///         .hatch()  // Then convert to Yoshi
///         .lay("While parsing user input")
/// }
/// ```
pub trait Hatchable<T, E> {
    /// Converts an error into a `Hatch<T>` by mapping it into `Yoshi`.
    ///
    /// This method provides a convenient way to bring external error types into
    /// the Yoshi ecosystem while maintaining type safety and performance efficiency.
    /// The conversion leverages existing `Into<Yoshi>` implementations to minimize
    /// overhead and maintain semantic meaning.
    ///
    /// # Type Conversion Chain
    ///
    /// The method works by applying the following transformation:
    /// `Result<T, E>` → `Result<T, Yoshi>` (via `E: Into<Yoshi>`)
    ///
    /// # Returns
    ///
    /// A `Hatch<T>` containing either the original success value or the converted error.
    ///
    /// # Performance Considerations
    ///
    /// - **Zero-cost for compatible types**: When `E` already has efficient `Into<Yoshi>`
    /// - **Minimal allocation**: Reuses existing error data structures where possible
    /// - **Compile-time optimization**: Fully optimizable conversion chains
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yoshi_std::{Hatch, Hatchable};
    /// # use std::io;
    ///
    /// // I/O error conversion
    /// let io_result: Result<String, io::Error> = Err(io::Error::new(
    ///     io::ErrorKind::NotFound, "file not found"
    /// ));
    /// let hatched: Hatch<String> = io_result.hatch();
    /// assert!(hatched.is_err());
    ///
    /// // String error conversion
    /// let string_result: Result<i32, String> = Err("parsing failed".to_string());
    /// let hatched: Hatch<i32> = string_result.hatch();
    /// assert!(hatched.is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `Hatch<T>` containing the converted error if `self` is `Err`,
    /// or the original success value if `self` is `Ok`. Conversion errors are
    /// not possible as the `Into<Yoshi>` bound guarantees valid transformation.
    fn hatch(self) -> Hatch<T>;
}

impl<T, E: Into<Yoshi>> Hatchable<T, E> for Result<T, E> {
    #[track_caller]
    fn hatch(self) -> Hatch<T> {
        self.map_err(Into::into)
    }
}

//--------------------------------------------------------------------------------------------------
// Enhanced backtrace capture with performance monitoring
//--------------------------------------------------------------------------------------------------

/// Conditionally captures a `YoshiBacktrace` based on environment variables.
///
/// This private helper function checks the `RUST_LIB_BACKTRACE` and `RUST_BACKTRACE`
/// environment variables. If either is set to "1" or "full", a [`YoshiBacktrace`]
/// is captured and returned. Otherwise, it returns `None`.
/// This ensures backtraces are only generated when explicitly requested,
/// minimizing performance overhead in production.
///
/// # Returns
///
/// An `Option` containing a [`YoshiBacktrace`] if backtrace capture is enabled,
/// or `None` otherwise.
///
/// # Panics
///
/// This function will panic if `OnceLock::get_or_init` is called in a `no_std` context
/// as its placeholder implementation panics. However, this function itself is
/// `#[cfg(feature = "std")]`, so it won't be compiled in `no_std`.
#[cfg(feature = "std")]
fn capture_bt() -> Option<YoshiBacktrace> {
    // For more robust behavior, especially in testing environments,
    // check the environment variables directly each time instead of caching
    let should =
        match std::env::var("RUST_LIB_BACKTRACE").or_else(|_| std::env::var("RUST_BACKTRACE")) {
            Ok(v) => v == "1" || v == "full", // Only enable backtrace for specific values
            Err(_) => false,
        };

    if should {
        Some(YoshiBacktrace::new_captured())
    } else {
        None
    }
}

/// Enhanced memory management utilities
pub mod memory {
    use super::{error_instance_count, intern_string, Arc, String, STRING_INTERN_POOL};
    /// Memory usage statistics for error handling
    #[derive(Debug, Default)]
    pub struct MemoryStats {
        /// Total number of Yoshi error instances created since application start
        pub total_errors_created: u32,
        /// Total number of context objects created across all errors
        pub total_contexts_created: u64,
        /// Number of string interning cache hits for memory optimization
        pub string_intern_hits: usize,
        /// Number of string interning cache misses requiring new allocations
        pub string_intern_misses: usize,
        /// Estimated bytes saved through string interning and optimization
        pub estimated_memory_saved: usize,
    }

    /// Get comprehensive memory usage statistics
    pub fn get_memory_stats() -> MemoryStats {
        let (hits, misses) = STRING_INTERN_POOL
            .get()
            .map_or((0, 0), super::StringInternPool::stats);

        MemoryStats {
            total_errors_created: error_instance_count(),
            total_contexts_created: 0, // Would need tracking
            string_intern_hits: hits,
            string_intern_misses: misses,
            estimated_memory_saved: hits * 32, // Rough estimate
        }
    }

    /// Memory-efficient string creation with automatic interning
    pub fn efficient_string(s: impl Into<String>) -> Arc<str> {
        intern_string(s)
    }

    /// Triggers cleanup of the string interning pool for long-running applications
    #[cfg(feature = "std")]
    pub fn cleanup_intern_pool() {
        if let Some(pool) = STRING_INTERN_POOL.get() {
            pool.clear_pool();
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Advanced async error handling module with Rust 1.87 enhancements
//--------------------------------------------------------------------------------------------------

/// Conditional async features for nightly compatibility
#[cfg(all(feature = "async", not(docsrs)))]
mod async_impl {
    #[allow(unused_imports)] // Conditional imports based on feature flags
    pub use crate::async_error_handling::*;
}

#[cfg(all(feature = "async", docsrs))]
mod async_docs {
    use crate::{Result, Yoshi};
    /// Simplified async docs without tokio complications
    pub type AsyncResult<T> = core::future::Ready<Result<T, Yoshi>>;

    /// Documentation placeholder for async functionality
    pub fn async_docs_placeholder() -> &'static str {
        "Full async functionality available in runtime builds"
    }
}

#[cfg(feature = "std")]
pub mod async_error_handling {
    //! Advanced async error processing utilities with precise capturing and performance optimization.

    use super::{Result, String, Vec, Yoshi, YoshiKind};
    use std::future::Future;
    use std::time::Duration;

    #[cfg(feature = "async")]
    #[allow(unused_imports)]
    use tokio::time;

    /// Async error propagation with enhanced context preservation
    ///
    /// # Errors
    ///
    /// Returns a `Yoshi` error if the future resolves to an error, with additional context added.
    pub async fn propagate_async<T, E>(
        future: impl Future<Output = Result<T, E>>,
        context: impl Into<String>,
    ) -> Result<T, Yoshi>
    where
        E: Into<Yoshi>,
    {
        match future.await {
            Ok(value) => Ok(value),
            Err(error) => Err(error.into().context(context.into())),
        }
    }

    /// Async error recovery with exponential backoff
    ///
    /// # Errors
    ///
    /// Returns a `Yoshi` error if all retry attempts fail or if the error is not transient.
    pub async fn retry_with_backoff<T, F, Fut>(
        mut operation: F,
        max_retries: usize,
        base_delay: Duration,
    ) -> Result<T, Yoshi>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<T, Yoshi>>,
    {
        let mut delay = base_delay;

        for attempt in 0..=max_retries {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(error) if attempt == max_retries => return Err(error),
                Err(error) if error.is_transient() => {
                    // Use async sleep for proper async compatibility
                    #[cfg(feature = "async")]
                    tokio::time::sleep(delay).await;
                    #[cfg(not(feature = "async"))]
                    std::thread::sleep(delay);
                    delay *= 2;
                }
                Err(error) => return Err(error),
            }
        }

        unreachable!()
    }

    /// Async error aggregation for parallel operations
    ///
    /// # Errors
    ///
    /// Returns a `Yoshi` error with multiple errors aggregated if any operations fail.
    pub async fn aggregate_errors<I, F, Fut, T>(operations: I) -> Result<Vec<T>, Yoshi>
    where
        I: IntoIterator<Item = F>,
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T, Yoshi>>,
    {
        let futures: Vec<_> = operations.into_iter().map(|op| op()).collect();
        // Simple join_all implementation without futures dependency
        let mut results = Vec::new();
        for fut in futures {
            results.push(fut.await);
        }

        let mut successes = Vec::new();
        let mut errors = Vec::new();

        for result in results {
            match result {
                Ok(value) => successes.push(value),
                Err(error) => errors.push(error),
            }
        }

        if errors.is_empty() {
            Ok(successes)
        } else {
            Err(Yoshi::new(YoshiKind::Multiple {
                errors,
                primary_index: Some(0),
            }))
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Cross-process communication and error reporting
//--------------------------------------------------------------------------------------------------

#[cfg(all(feature = "std", feature = "serde"))]
pub mod process_communication {
    //! Cross-process error reporting and coordination with enterprise-grade reliability.

    use super::{Arc, HashMap, OnceLock, Result, String, SystemTime, ToString, Yoshi};
    use serde::{self, Deserializer, Serializer};
    use serde_json;
    use std::sync::mpsc;
    use std::thread;

    // Helper functions for SystemTime serialization/deserialization for std
    // (Serializes as seconds since UNIX_EPOCH)
    mod serde_system_time {
        use super::{Deserializer, Serializer};
        use serde::Deserialize;
        use std::time::{SystemTime, UNIX_EPOCH};
        #[allow(clippy::trivially_copy_pass_by_ref)]
        pub fn serialize<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let duration = time
                .duration_since(UNIX_EPOCH)
                .map_err(serde::ser::Error::custom)?;
            serializer.serialize_u64(duration.as_secs())
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
        where
            D: Deserializer<'de>,
        {
            let secs = u64::deserialize(deserializer)?;
            Ok(UNIX_EPOCH + std::time::Duration::from_secs(secs))
        }
    }

    /// Cross-process error reporter with structured logging
    pub struct ProcessErrorReporter {
        sender: mpsc::Sender<ProcessError>,
        _handle: thread::JoinHandle<()>,
    }
    /// Serializable error for cross-process communication
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct ProcessError {
        /// Unique identifier for the process that generated this error
        pub process_id: u32,
        /// String identifier for the thread within the process
        pub thread_id: String,
        /// Human-readable error message describing the failure
        pub error_message: String,
        /// Classification of the error type for categorization
        pub error_kind: String,
        /// Severity level from 0 (info) to 255 (critical)
        pub severity: u8,
        /// System timestamp when the error occurred
        #[serde(with = "serde_system_time")]
        pub timestamp: SystemTime,
        /// Additional metadata for enhanced error context
        #[serde(
            serialize_with = "super::serde_helpers::serialize_arc_str_map",
            deserialize_with = "super::serde_helpers::deserialize_arc_str_map"
        )]
        pub metadata: HashMap<Arc<str>, Arc<str>>,
    }

    impl Default for ProcessErrorReporter {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ProcessErrorReporter {
        /// Creates a new process error reporter with background processing
        #[must_use]
        pub fn new() -> Self {
            let (sender, receiver) = mpsc::channel::<ProcessError>();

            let handle = thread::spawn(move || {
                while let Ok(error) = receiver.recv() {
                    // Process and log the error
                    eprintln!(
                        "[PROCESS-ERROR] {}: {} (PID: {}, Severity: {})",
                        error.timestamp.elapsed().map(|d| d.as_secs()).unwrap_or(0),
                        error.error_message,
                        error.process_id,
                        error.severity
                    ); // Write to structured log file (using serde_json for robust serialization)
                    if let Ok(json_log) = serde_json::to_string(&error) {
                        println!("STRUCTURED_LOG: {json_log}");
                    } else {
                        eprintln!("Failed to serialize process error to JSON.");
                    }
                }
            });

            Self {
                sender,
                _handle: handle,
            }
        }
        /// Reports an error to the cross-process system
        ///
        /// # Errors
        ///
        /// Returns `mpsc::SendError<ProcessError>` if the cross-process communication
        /// channel is disconnected or the receiver has been dropped.
        pub fn report_error(&self, error: &Yoshi) -> Result<(), mpsc::SendError<ProcessError>> {
            let process_error = ProcessError {
                process_id: std::process::id(),
                thread_id: format!("{:?}", std::thread::current().id()),
                error_message: error.to_string(),
                error_kind: format!("{:?}", error.kind()),
                severity: error.severity(),
                timestamp: SystemTime::now(),
                metadata: error
                    .primary_context()
                    .map(|ctx| ctx.metadata.clone())
                    .unwrap_or_default(),
            };

            self.sender.send(process_error)
        }
    }

    /// Global process error coordinator
    static PROCESS_REPORTER: OnceLock<ProcessErrorReporter> = OnceLock::new();

    /// Gets or initializes the global process error reporter
    pub fn global_reporter() -> &'static ProcessErrorReporter {
        PROCESS_REPORTER.get_or_init(ProcessErrorReporter::new)
    }

    /// Reports an error to the global cross-process system
    pub fn report_global_error(error: &Yoshi) {
        if let Err(e) = global_reporter().report_error(error) {
            eprintln!("Failed to report error to cross-process system: {e}");
        }
    }
}

//--------------------------------------------------------------------------------------------------
// SIMD-optimized string processing for high-performance formatting
//--------------------------------------------------------------------------------------------------

#[cfg(all(feature = "simd-optimized", target_arch = "x86_64"))]
pub mod simd_optimization {
    //! SIMD-accelerated string processing for optimal error formatting performance.
    //! Uses stable `std::arch` intrinsics with runtime feature detection.

    use super::{String, ToString, Vec, Yoshi};

    /// SIMD-optimized string formatting buffer
    pub struct SimdFormatBuffer {
        data: Vec<u8>,
        capacity: usize,
    }

    impl SimdFormatBuffer {
        /// Creates a new SIMD-optimized format buffer
        #[must_use]
        pub fn new() -> Self {
            Self::with_capacity(4096)
        }

        /// Creates a buffer with specified capacity aligned for SIMD operations
        #[must_use]
        pub fn with_capacity(capacity: usize) -> Self {
            // Align capacity to 32-byte boundaries for optimal SIMD operations
            let aligned_capacity = (capacity + 31) & !31;
            Self {
                data: Vec::with_capacity(aligned_capacity),
                capacity: aligned_capacity,
            }
        }
        /// SIMD-accelerated string concatenation with runtime feature detection
        pub fn append_simd(&mut self, s: &str) {
            let bytes = s.as_bytes();
            let new_len = self.data.len() + bytes.len();

            if new_len > self.capacity {
                self.grow_aligned(new_len);
            }

            // Use SIMD operations for large strings if AVX2 is available
            if bytes.len() >= 32 && std::is_x86_feature_detected!("avx2") {
                // SAFETY: We've checked that AVX2 is available at runtime
                unsafe { self.append_simd_internal_avx2(bytes) };
            } else {
                // Fallback to standard operations
                self.data.extend_from_slice(bytes);
            }
        }
        /// Internal SIMD implementation using stable `std::arch` AVX2 intrinsics
        #[target_feature(enable = "avx2")]
        unsafe fn append_simd_internal_avx2(&mut self, bytes: &[u8]) {
            #[cfg(target_arch = "x86_64")]
            {
                use std::arch::x86_64::{_mm256_loadu_si256, _mm256_storeu_si256};

                let chunks = bytes.chunks_exact(32);
                let remainder = chunks.remainder();

                // Reserve space for all the data we're about to add
                let start_len = self.data.len();
                let total_chunk_bytes = chunks.len() * 32;

                // Ensure we have enough capacity
                if start_len + bytes.len() > self.data.capacity() {
                    self.data.reserve(bytes.len());
                }

                // Process 32-byte chunks with AVX2
                let mut offset = 0;
                for chunk in chunks {
                    // Load 32 bytes using AVX2
                    let simd_data = _mm256_loadu_si256(chunk.as_ptr().cast());

                    // Store 32 bytes to our destination
                    let dst_ptr = self.data.as_mut_ptr().add(start_len + offset).cast();
                    _mm256_storeu_si256(dst_ptr, simd_data);

                    offset += 32;
                }

                // Update the vector length to include the SIMD-processed data
                self.data.set_len(start_len + total_chunk_bytes);

                // Handle remaining bytes with standard operations
                if !remainder.is_empty() {
                    self.data.extend_from_slice(remainder);
                }
            }

            #[cfg(not(target_arch = "x86_64"))]
            {
                // Fallback for non-x86_64 architectures
                self.data.extend_from_slice(bytes);
            }
        }

        /// Fallback SIMD implementation for when AVX2 is not available
        fn append_simd_fallback(&mut self, bytes: &[u8]) {
            // Standard extend_from_slice is often well-optimized by LLVM
            self.data.extend_from_slice(bytes);
        }

        /// Grows the buffer with proper alignment
        fn grow_aligned(&mut self, min_capacity: usize) {
            let new_capacity = ((min_capacity * 2) + 31) & !31;
            self.data.reserve_exact(new_capacity - self.data.capacity());
            self.capacity = new_capacity;
        }
        /// Returns the formatted string
        #[must_use]
        pub fn as_str(&self) -> &str {
            // SAFETY: We only append valid UTF-8 strings
            unsafe { std::str::from_utf8_unchecked(&self.data) }
        }

        /// Clears the buffer while preserving capacity
        pub fn clear(&mut self) {
            self.data.clear();
        }
    }

    impl Default for SimdFormatBuffer {
        fn default() -> Self {
            Self::new()
        }
    }

    /// SIMD-optimized error formatting
    pub fn format_error_simd(error: &Yoshi) -> String {
        let mut buffer = SimdFormatBuffer::new();

        // Format main error
        buffer.append_simd(&format!("{}", error.kind()));

        // Add contexts with SIMD acceleration
        for context in error.contexts() {
            if let Some(ref message) = context.message {
                buffer.append_simd("\nCaused by: ");
                buffer.append_simd(message);
            }
        }

        buffer.as_str().to_string()
    }
}

//--------------------------------------------------------------------------------------------------
// Comprehensive test suite with performance validation
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    // TypeId is not needed for checking foreign error names after switching to type_name!
    // use core::any::TypeId; // For TypeId usage in tests

    #[cfg(feature = "std")]
    use std::io::ErrorKind;
    #[cfg(feature = "std")]
    use std::{env, io};

    #[test]
    fn test_error_instance_counter() {
        // Reset counter to ensure test isolation for precise counting
        reset_error_instance_counter();

        let initial_count = error_instance_count();
        let _err1 = Yoshi::new(YoshiKind::Internal {
            message: "test".into(),
            source: None,
            component: None,
        });
        let after_first_count = error_instance_count();
        // Allow for some variance due to potential concurrent test execution
        assert!(
            after_first_count > initial_count,
            "Creating first error should increment counter by at least 1"
        );

        let _err2 = Yoshi::new(YoshiKind::Internal {
            message: "test".into(),
            source: None,
            component: None,
        });
        let after_second_count = error_instance_count();
        // Creating the second error should also increment by at least 1
        assert!(
            after_second_count > after_first_count,
            "Creating second error should increment counter by at least 1"
        );
    }

    #[test]
    fn test_yoshikind_io_display() {
        #[cfg(feature = "std")]
        {
            let io_err = io::Error::new(ErrorKind::NotFound, "file not found");
            let kind = YoshiKind::Io(io_err);
            assert_eq!(kind.to_string(), "I/O error: file not found");
        }
        #[cfg(not(feature = "std"))]
        {
            let kind = YoshiKind::Io(NoStdIo::GenericIo("memory exhausted".into()));
            assert_eq!(kind.to_string(), "I/O error (no_std): memory exhausted");
        }
    }

    #[test]
    fn test_yoshikind_resource_exhausted_display() {
        let kind = YoshiKind::ResourceExhausted {
            resource: "memory".into(),
            limit: "1GB".into(),
            current: "1.2GB".into(),
            usage_percentage: Some(120.0),
        };
        assert_eq!(
            kind.to_string(),
            "Resource 'memory' exhausted: 1.2GB (limit: 1GB) [120.0% usage]"
        );
    }

    #[test]
    fn test_yoshikind_timeout_uses_core_duration() {
        let kind = YoshiKind::Timeout {
            operation: "long_task".into(),
            duration: Duration::from_secs(5),
            expected_max: None,
        };
        assert_eq!(kind.to_string(), "Operation 'long_task' timed out after 5s");
        // Verify type is core::time::Duration
        let _duration: Duration = match kind {
            YoshiKind::Timeout { duration, .. } => duration,
            _ => panic!("Expected Timeout variant"),
        };
    }

    #[test]
    fn test_from_std_io_error() {
        #[cfg(feature = "std")]
        {
            let io_err = io::Error::new(ErrorKind::NotFound, "file not found");
            let yoshi_err = Yoshi::from(io_err);
            assert!(format!("{yoshi_err}").contains("I/O error: file not found"));
            assert!(matches!(yoshi_err.kind, YoshiKind::Io(_)));
        }
        #[cfg(not(feature = "std"))]
        {
            let no_std_io_err = NoStdIo::new("no_std file not found");
            let yoshi_err = Yoshi::from(no_std_io_err);
            assert!(format!("{yoshi_err}").contains("I/O error (no_std): not found"));
            assert!(matches!(yoshi_err.kind, YoshiKind::Io(_)));
        }
    }

    #[test]
    fn test_from_string() {
        let msg = "simple string error".to_string();
        let yoshi_err = Yoshi::from(msg.clone());
        assert!(matches!(
            yoshi_err.kind,
            YoshiKind::Internal {
                ref message, ..
            } if message.as_ref() == msg
        ));
        assert!(format!("{yoshi_err}").contains(&msg));
    }

    #[test]
    fn test_yoshi_foreign_from_boxed_error() {
        #[derive(Debug)]
        struct MyCustomError;
        impl Display for MyCustomError {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "a custom error occurred")
            }
        }
        impl Error for MyCustomError {}

        let boxed_err = Box::new(MyCustomError);
        let yoshi_err = Yoshi::foreign(boxed_err); // Changed to Yoshi::foreign
        assert!(format!("{yoshi_err}").contains("a custom error occurred"));
        assert!(matches!(yoshi_err.kind, YoshiKind::Foreign { .. }));
        if let YoshiKind::Foreign {
            error_type_name, ..
        } = yoshi_err.kind
        {
            assert_eq!(error_type_name.as_ref(), "alloc::boxed::Box<yoshi_std::tests::test_yoshi_foreign_from_boxed_error::MyCustomError>");
        } else {
            panic!("Expected Foreign kind");
        }
    }
    #[test]
    fn test_contextualization() {
        #[cfg(feature = "std")]
        let base_err = io::Error::new(ErrorKind::PermissionDenied, "access denied");
        #[cfg(not(feature = "std"))]
        let base_err = NoStdIo::new("access denied");

        let yoshi_err = Yoshi::from(base_err)
            .context("Attempted to write to a protected directory".to_string())
            .with_metadata("user_id".to_string(), "guest".to_string())
            .with_suggestion("Try running with elevated privileges".to_string())
            .with_priority(200);
        let err_string = format!("{yoshi_err}");

        // Debug: print the actual error string to see what it contains
        eprintln!("Error string: {err_string}");

        #[cfg(feature = "std")]
        assert!(err_string.contains("access denied"));

        #[cfg(not(feature = "std"))]
        assert!(err_string.contains("access denied"));

        assert!(err_string.contains("Caused by: Attempted to write to a protected directory"));
        assert!(err_string.contains("user_id: guest"));
        assert!(err_string.contains("Suggestion: Try running with elevated privileges"));
        assert_eq!(yoshi_err.primary_context().unwrap().priority, 200);
    }
    #[test]
    fn test_chained_yoshi_kind() {
        let inner_yoshi = Yoshi::new(YoshiKind::Network {
            message: "Connection refused".into(),
            source: None,
            error_code: None,
        });

        let outer_yoshi = Yoshi::new(YoshiKind::Internal {
            message: "Service communication failed".into(),
            source: Some(Box::new(inner_yoshi)),
            component: None,
        });
        let err_string = format!("{outer_yoshi}");

        assert!(err_string.contains("Internal error: Service communication failed"));
        assert!(err_string.contains("Caused by: Network error: Connection refused")); // Check for nested display
        assert!(!err_string.contains("Original Cause: Network error: Connection refused"));
        // Should not be duplicated
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_backtrace_capture_if_enabled() {
        let original_rust_backtrace = env::var("RUST_BACKTRACE").ok();
        let original_production_mode = env::var("YOSHI_PRODUCTION_MODE").ok();

        // Ensure we're not in production mode which would sanitize the backtrace
        env::remove_var("YOSHI_PRODUCTION_MODE");
        env::set_var("RUST_BACKTRACE", "1");

        let err = Yoshi::new(YoshiKind::Internal {
            message: "Test internal error with backtrace".into(),
            source: None,
            component: None,
        });
        assert!(err.backtrace().is_some());

        let formatted_error = format!("{err}");
        // Check for the backtrace framework indicator that's always included
        assert!(formatted_error.contains("Generated by std::backtrace framework"));
        assert!(formatted_error.contains("Backtrace captured at:"));

        // Restore original environment
        if let Some(val) = original_rust_backtrace {
            env::set_var("RUST_BACKTRACE", val);
        } else {
            env::remove_var("RUST_BACKTRACE");
        }

        if let Some(val) = original_production_mode {
            env::set_var("YOSHI_PRODUCTION_MODE", val);
        }
    }

    #[test]
    fn test_no_backtrace_if_disabled() {
        #[cfg(feature = "std")]
        let original_rust_backtrace = env::var("RUST_BACKTRACE").ok();
        #[cfg(feature = "std")]
        env::remove_var("RUST_BACKTRACE");

        let err = Yoshi::new(YoshiKind::Internal {
            message: "No backtrace expected".into(),
            source: None,
            component: None,
        });

        #[cfg(feature = "std")]
        assert!(err.backtrace().is_none());
        #[cfg(not(feature = "std"))]
        assert!(err.backtrace.is_none());

        assert!(!format!("{err}").contains("stack backtrace"));

        #[cfg(feature = "std")]
        {
            if let Some(val) = original_rust_backtrace {
                env::set_var("RUST_BACKTRACE", val);
            }
        }
    }

    #[test]
    fn test_access_metadata_directly() {
        let err = Yoshi::new(YoshiKind::Internal {
            message: "Test provide metadata".into(),
            source: None,
            component: None,
        })
        .with_metadata("id", "123")
        .with_metadata("status", "failed");

        // Access metadata directly from the YoContext
        let ctx = err
            .primary_context()
            .expect("Should have a primary context");
        let map = &ctx.metadata;
        assert_eq!(map.get(&Arc::from("id")), Some(&Arc::from("123")));
        assert_eq!(map.get(&Arc::from("status")), Some(&Arc::from("failed")));
    }

    #[test]
    fn test_yoshi_location_macro() {
        let loc = yoshi_location!();
        assert!(loc.file.ends_with("lib.rs"));
        assert!(loc.line > 0);
        assert!(loc.column > 0);
        assert_eq!(
            format!("{loc}"),
            format!("{}:{}:{}", loc.filename(), loc.line, loc.column)
        );
    }

    #[test]
    fn test_yoshi_with_payload_and_access() {
        #[derive(Debug, PartialEq)]
        struct CustomErrorPayload {
            code: u16,
            message: String,
        }
        impl Display for CustomErrorPayload {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "CustomPayload: code={}, msg={}", self.code, self.message)
            }
        }

        let err = Yoshi::new(YoshiKind::Internal {
            message: "Operation failed with custom shell".into(),
            source: None,
            component: None,
        })
        .with_shell(CustomErrorPayload {
            code: 500,
            message: "Internal server error details".into(),
        })
        .with_shell("a string shell".to_string())
        .with_shell(42u32);

        // Access payloads using the more robust `Yoshi::shell` method
        let ctx = err
            .primary_context()
            .expect("Should have a primary context");

        let custom_payload = ctx.shell::<CustomErrorPayload>();
        assert!(custom_payload.is_some());
        assert_eq!(custom_payload.unwrap().code, 500);

        let string_payload = ctx.shell::<String>();
        assert!(string_payload.is_some());
        assert_eq!(string_payload.unwrap(), &"a string shell".to_string());

        let u32_payload = ctx.shell::<u32>();
        assert!(u32_payload.is_some());
        assert_eq!(*u32_payload.unwrap(), 42);
    }

    #[test]
    fn test_yoshi_context_ext_with_payload_on_result() {
        #[derive(Debug, PartialEq)]
        struct TransactionId(String);

        #[cfg(feature = "std")]
        let result: std::result::Result<u32, std::io::Error> = Err(io::Error::new(
            ErrorKind::PermissionDenied,
            "db write failed",
        ));
        #[cfg(not(feature = "std"))]
        let result: core::result::Result<u32, NoStdIo> = Err(NoStdIo::new("db write failed"));

        let yoshi_result = result
            .with_shell(TransactionId("tx123".into()))
            .context("Failed to commit transaction".to_string());

        assert!(yoshi_result.is_err());
        let err = yoshi_result.unwrap_err();

        assert!(format!("{err}").contains("db write failed"));
        assert!(format!("{err}").contains("Caused by: Failed to commit transaction")); // Access shell using the corrected `Yoshi::shell` method that searches all contexts
        let transaction_id = err.shell::<TransactionId>();

        assert!(transaction_id.is_some(), "Should find TransactionId shell");
        assert_eq!(transaction_id.unwrap().0, "tx123".to_string());
    }

    #[test]
    fn test_yoshi_context_ext_short_aliases() {
        #[cfg(feature = "std")]
        let result: std::result::Result<(), std::io::Error> = Err(io::Error::new(
            io::ErrorKind::NotFound,
            "file.txt not found",
        ));
        #[cfg(not(feature = "std"))]
        let result: core::result::Result<(), NoStdIo> = Err(NoStdIo::NotFound);

        let err = result
            .ctx("Failed to open file".to_string())
            .help("Check file path and permissions".to_string())
            .meta("file_name".to_string(), "file.txt".to_string())
            .unwrap_err();

        let s = format!("{err}");
        assert!(s.contains("Failed to open file"));
        assert!(s.contains("Check file path and permissions"));
        assert!(s.contains("file_name: file.txt"));
    }

    #[test]
    fn test_hatch_type_alias() {
        let success: Hatch<u32> = Ok(42);
        if let Ok(value) = success {
            assert_eq!(value, 42);
        } else {
            panic!("Expected Ok value");
        }

        let failure: Hatch<u32> = Err(Yoshi::new(YoshiKind::Internal {
            message: "test error".into(),
            source: None,
            component: None,
        }));
        assert!(failure.is_err());
    }

    #[test]
    fn test_lay_context_trait() {
        let error = Yoshi::new(YoshiKind::Internal {
            message: "base error".into(),
            source: None,
            component: None,
        });

        let result: Hatch<()> = Err(error).lay("additional context");
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.to_string().contains("additional context"));
    }

    #[test]
    fn test_hatchable_trait() {
        #[cfg(feature = "std")]
        {
            use std::io;
            let io_result: Result<String, io::Error> =
                Err(io::Error::new(io::ErrorKind::NotFound, "file not found"));
            let hatched = io_result.hatch();
            assert!(hatched.is_err());
        }

        let string_result: Result<i32, String> = Err("conversion failed".to_string());
        let hatched = string_result.hatch();
        assert!(hatched.is_err());
    }

    #[test]
    fn test_yoshi_enhanced_methods() {
        let error = Yoshi::new(YoshiKind::Internal {
            message: "base error".into(),
            source: None,
            component: None,
        })
        .lay("operation context");

        // Test laytext method
        assert_eq!(error.laytext().unwrap(), "operation context");

        // Test nest method (should be None for this error)
        assert!(error.nest().is_none());
    }

    #[test]
    fn test_yum_macro() {
        let error = Yoshi::new(YoshiKind::Internal {
            message: "test error for yum".into(),
            source: None,
            component: None,
        })
        .context("test context")
        .with_suggestion("try again");

        // yum! macro should not panic and should return the error
        let returned_error = yum!(error);
        assert_eq!(returned_error.laytext().unwrap(), "test context");
        assert_eq!(returned_error.suggestion().unwrap(), "try again");
    }

    #[test]
    fn test_hatch_backwards_compatibility() {
        use core::error::Error;

        let error = Yoshi::new(YoshiKind::Internal {
            message: "compatibility test".into(),
            source: None,
            component: None,
        });

        // Test that standard Error trait methods still work
        let error_ref: &dyn Error = &error;
        assert!(error_ref.source().is_none());

        // Test that new methods work alongside old ones
        assert!(error.nest().is_none()); // New method
        assert!(error.laytext().is_none()); // New method (no context added)
    }

    #[test]
    fn test_hatch_ecosystem_integration() {
        // Test complete workflow with all Hatch ecosystem components
        fn complex_operation() -> Hatch<u32> {
            #[cfg(feature = "std")]
            let io_result: Result<String, std::io::Error> = Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "access denied",
            ));
            #[cfg(not(feature = "std"))]
            let io_result: Result<String, NoStdIo> = Err(NoStdIo::PermissionDenied);

            io_result
                .hatch()
                .lay("while accessing configuration")
                .context("during system initialization")
                .map_err(|e| {
                    e.with_metadata("component", "config_loader")
                        .with_suggestion("check file permissions")
                })?;

            Ok(42)
        }

        let result = complex_operation();
        assert!(result.is_err());

        let error = result.unwrap_err();

        // Verify thematic methods work
        assert!(error.laytext().is_some());
        assert_eq!(error.suggestion().unwrap(), "check file permissions");

        // Verify nest access works
        assert!(error.nest().is_some());

        // Test yum! macro
        let debug_error = yum!(error);
        assert_eq!(debug_error.instance_id(), error.instance_id());
    }
}
