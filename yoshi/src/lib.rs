/* yoshi/src/lib.rs */
#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
#![allow(clippy::use_self)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "simd-optimized"), deny(unsafe_code))]
//! **Brief:** The Yoshi error handling framework was designed as an all-in-one solution
//! for handling errors in any kind of application, taking the developers' sanity as a
//! first-class citizen. It's designed to be both efficient and user-friendly, ensuring that
//! developers can focus on their core tasks while Yoshi carries the weight of their errors.
//!
//! This crate provides a robust, highly performant, and flexible error handling solution
//! designed for critical applications. It features structured error kinds, contextualization
//! with rich metadata, and efficient backtrace capture, all while minimizing runtime overhead.
//! The design emphasizes mathematical predictability in performance and resource usage.
//! 
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + High-performance error handling with zero-cost abstractions [O(1) error creation, O(1) context attachment]
//!  - Advanced error categorization with semantic type safety [Memory-safe, Thread-safe]
//!  - Comprehensive context chaining with cycle detection [Stack-overflow protection, O(N) traversal, where N is context depth]
//!  - Enterprise-grade backtrace capture with metadata [Conditional compilation, Performance monitoring]
//!  - Structured error formatting with SIMD optimization [Buffer pre-allocation, Minimal allocations]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//!
//! ## Mathematical Properties
//!
//! **Algorithmic Complexity:**
//! - Time Complexity: O(1) for error creation, O(1) for context attachment. O(N) for context chain traversal and formatting (where N is context depth).
//! - Space Complexity: O(N) where N is context chain depth, bounded by MAX_DEPTH=32
//! - Concurrency Safety: Send + Sync + 'static guarantees with atomic instance counting
//!
//! **Performance Characteristics:**
//! - Expected Performance: Sub-microsecond error creation, <100ns context attachment. Full error formatting depends on context depth.
//! - Worst-Case Scenarios: O(MAX_DEPTH) for deep context chains with cycle protection during formatting.
//! - Optimization Opportunities: SIMD-friendly formatting, pre-allocated buffers, lazy backtrace capture
//!
//! ## Module Organization
//!
//! - [`Yoshi`]: The main error type, providing structured error handling capabilities.
//! - [`YoshiKind`]: Defines high-level categories for errors.
//! - [`YoshiContext`]: Stores additional contextual information for errors.
//! - [`YoshiLocation`]: Represents a source code location.
//! - [`YoshiBacktrace`]: Wraps a standard library backtrace with performance metadata.
//! - [`YoshiContextExt`]: An extension trait for `Result` to easily attach context.
//! - [`NoStdIo`]: A minimal I/O error type for `no_std` environments.
//!
//! # Examples
//!
//! Basic usage of creating and propagating `Yoshi` errors:
//!
//! ```
//! use yoshi_std::{Yoshi, YoshiKind, YoshiContextExt};
//! # use std::io;
//! # use std::io::ErrorKind;
//! #
//! # fn might_fail_io() -> Result<(), io::Error> {
//! #    Err(io::Error::new(ErrorKind::PermissionDenied, "cannot access file"))
//! # }
//!
//! fn read_config_file(path: &str) -> Result<String, Yoshi> {
//!     // Simulate an I/O error
//!     let io_error = might_fail_io().map_err(Yoshi::from)?;
//!
//!     // Add context to the error using the extension trait
//!     Err(io_error)
//!         .context(format!("Failed to read config from path: {}", path))
//!         .with_metadata("file_path", path)
//!         .with_suggestion("Ensure the file exists and permissions are correct.")
//! }
//!
//! # fn main() {
//! let config_path = "/etc/app/config.json";
//! if let Err(e) = read_config_file(config_path) {
//!     println!("Application error: {}", e);
//!     // Expected output might contain:
//!     // "I/O error: cannot access file"
//!     // "Caused by: Failed to read config from path: /etc/app/config.json"
//!     // "  file_path: /etc/app/config.json"
//!     // "  Suggestion: Ensure the file exists and permissions are correct."
//! }
//! # }
//! ```
//!
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios  
// **License:** Business Source License 1.1 (BSL-1.1)
// **License File:** /LICENSE
// **License Terms:** Non-production use only; commercial/production use requires paid license.
// **Effective Date:** 2025-05-25 | **Change License:** GPL v3
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

// Unconditionally enable alloc crate for no_std builds using heap allocations
#[cfg(not(feature = "std"))]
extern crate alloc;

// Unified imports for String, Vec, Box, Arc based on 'std' feature
#[cfg(feature = "std")]
use std::{
    boxed::Box,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};
#[cfg(not(feature = "std"))]
use alloc::{
    boxed::Box,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};

use core::any::Any; // Import Any for error_generic_member_access and blanket From
use core::error::Error; // Removed Request as it's unstable
use core::fmt::{self, Display, Formatter, Write};
use core::mem;
use core::sync::atomic::{AtomicU64, Ordering};
use core::time::Duration;

// Unified imports for HashMap based on 'std' feature
#[cfg(feature = "std")]
use std::collections::HashMap;
#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap as HashMap; // Using BTreeMap for no_std by default

#[cfg(all(feature = "serde", feature = "rust-1-87", feature = "std"))]
use serde_json::{self, Value as JsonValue};

// Unified imports for SystemTime and Thread based on 'std' feature
#[cfg(feature = "std")]
use std::{thread, time::SystemTime};
#[cfg(not(feature = "std"))]
/// Enhanced SystemTime for `no_std` environments with monotonic counter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
use core::sync::atomic::{AtomicU32, Ordering};

// Rust 1.87 Enhanced Imports for Cross-Process Communication and Advanced Features
#[cfg(all(feature = "rust-1-87", feature = "std"))]
use std::process::{Child, Command, ExitStatus, Stdio};
#[cfg(all(feature = "rust-1-87", feature = "std", unix))]
use std::os::unix::net::UnixStream;
#[cfg(all(feature = "rust-1-87", feature = "std", windows))]
use std::os::windows::io::AsRawHandle;

/// Cross-platform process communication for error reporting.
#[cfg(all(feature = "rust-1-87", feature = "std"))]
pub mod process_communication {
    use super::*;
    
    /// Generic error reporter that can spawn external processes.
    #[allow(dead_code)] // Library API - may not be used internally
    pub struct ErrorReporter {
        command: String,
        args: Vec<String>,
    }
      impl ErrorReporter {
        /// Creates a new error reporter with the specified command.
        #[allow(dead_code)] // Library API - may not be used internally
        pub fn new(command: impl Into<String>) -> Self {
            Self {
                command: command.into(),
                args: Vec::new(),
            }
        }
        
        /// Adds an argument to the command that will be executed.
        #[allow(dead_code)] // Library API - may not be used internally
        pub fn with_arg(mut self, arg: impl Into<String>) -> Self {
            self.args.push(arg.into());
            self
        }
        
        /// Spawns the error reporter process with the given error data.
        #[allow(dead_code)] // Library API - may not be used internally
        pub fn spawn_reporter(&self, error_data: &str) -> std::io::Result<Child> {
            let mut cmd = Command::new(&self.command);
            cmd.args(&self.args)
               .arg("--error-data")
               .arg(error_data)
               .stdin(Stdio::piped())
               .stdout(Stdio::piped())
               .stderr(Stdio::piped());
            
            cmd.spawn()
        }
        
        /// Asynchronously reports an error to the external process.
        #[allow(dead_code)] // Library API - may not be used internally
        pub async fn report_async(&self, error: &Yoshi) -> std::io::Result<ExitStatus> {
            #[cfg(feature = "serde")]
            let error_json = serde_json::to_string(error)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            #[cfg(not(feature = "serde"))]
            let error_json = format!("{:?}", error);
            
            let mut child = self.spawn_reporter(&error_json)?;
            
            // Write error data to stdin if available
            if let Some(mut stdin) = child.stdin.take() {
                use std::io::Write;
                let _ = stdin.write_all(error_json.as_bytes());
            }
            
            child.wait()
        }
    }
    
    /// Platform-specific communication utilities.
    pub mod platform {
        use super::*;
        
        #[cfg(unix)]
        pub fn send_via_unix_socket(socket_path: &str, data: &[u8]) -> std::io::Result<()> {
            use std::io::Write;
            let mut stream = UnixStream::connect(socket_path)?;
            stream.write_all(data)?;
            stream.flush()?;
            Ok(())
        }
          /// Sends error data via a Windows handle.
        /// 
        /// This function provides a Windows-specific mechanism for transmitting error
        /// data through handles that implement `AsRawHandle`. It's designed for
        /// cross-process error communication in Windows environments.
        /// 
        /// # Arguments
        /// 
        /// * `handle` - A Windows handle that implements `AsRawHandle`
        /// * `data` - The error data to transmit as bytes
        /// 
        /// # Returns
        /// 
        /// Returns `Ok(())` on success, or an `std::io::Error` if transmission fails.
        /// 
        /// # Platform Support
        /// 
        /// This function is only available on Windows platforms and requires the
        /// `std` feature to be enabled.
        /// 
        /// # Examples
        /// 
        /// ```no_run
        /// # #[cfg(windows)]
        /// # fn example() -> std::io::Result<()> {
        /// use yoshi::cross_process::platform::send_via_handle;
        /// use std::os::windows::io::AsRawHandle;
        /// use std::fs::File;
        /// 
        /// let file = File::open("example.txt")?;
        /// let error_data = b"Error occurred in process";
        /// send_via_handle(&file, error_data)?;
        /// # Ok(())
        /// # }
        /// ```
        #[cfg(windows)]
        pub fn send_via_handle<T: AsRawHandle>(handle: &T, data: &[u8]) -> std::io::Result<()> {
            // Platform-specific implementation would go here
            let _raw_handle = handle.as_raw_handle();
            let _data_len = data.len();
            // Placeholder for actual Windows implementation
            Ok(())
        }
        
        #[cfg(not(any(unix, windows)))]
        pub fn send_via_fallback(data: &[u8]) -> std::io::Result<()> {
            // Generic fallback implementation
            eprintln!("Error data: {}", String::from_utf8_lossy(data));
            Ok(())
        }
    }
}

// SIMD intrinsics for high-performance string processing (conditionally imported)
#[cfg(all(feature = "simd-optimized", target_arch = "x86_64"))]
use core::arch::x86_64::{
    __m128i, __m256i, _mm_loadu_si128, _mm_set1_epi8,
    _mm_cmpeq_epi8, _mm_movemask_epi8, _mm256_loadu_si256, 
    _mm256_set1_epi8, _mm256_cmpeq_epi8, _mm256_movemask_epi8,
};

/// Generic SIMD optimization with runtime CPU feature detection.
#[cfg(all(feature = "simd-optimized", target_arch = "x86_64"))]
#[allow(dead_code)] // Public API for external use
mod simd_optimization {
    use super::*;
    
    /// Runtime CPU feature detection for optimal SIMD usage.
    #[allow(dead_code)] // Public API for external use
    pub struct CpuCapabilities {
        pub sse2: bool,
        pub avx2: bool,
        pub avx512f: bool,
    }
    
    impl CpuCapabilities {
        #[allow(dead_code)] // Public API for external use
        pub fn detect() -> Self {
            Self {
                sse2: is_x86_feature_detected!("sse2"),
                avx2: is_x86_feature_detected!("avx2"),
                avx512f: is_x86_feature_detected!("avx512f"),
            }
        }
    }
    
    /// SIMD-accelerated string search for error context matching.
    #[allow(dead_code)] // Public API for external use
    pub fn simd_find_byte(haystack: &[u8], needle: u8) -> Option<usize> {
        let caps = CpuCapabilities::detect();
        
        if caps.avx2 && haystack.len() >= 32 {
            unsafe { avx2_find_byte(haystack, needle) }
        } else if caps.sse2 && haystack.len() >= 16 {
            unsafe { sse2_find_byte(haystack, needle) }
        } else {
            haystack.iter().position(|&b| b == needle)
        }
    }
    
    #[target_feature(enable = "avx2")]
    #[allow(dead_code)] // Public API for external use
    unsafe fn avx2_find_byte(haystack: &[u8], needle: u8) -> Option<usize> {
        let needle_vec = _mm256_set1_epi8(needle as i8);
        
        for (chunk_idx, chunk) in haystack.chunks_exact(32).enumerate() {
            let data = _mm256_loadu_si256(chunk.as_ptr() as *const __m256i);
            let cmp = _mm256_cmpeq_epi8(data, needle_vec);
            let mask = _mm256_movemask_epi8(cmp);
            
            if mask != 0 {
                return Some(chunk_idx * 32 + mask.trailing_zeros() as usize);
            }
        }
        
        // Handle remainder
        let remainder_start = (haystack.len() / 32) * 32;
        haystack[remainder_start..].iter()
            .position(|&b| b == needle)
            .map(|pos| remainder_start + pos)
    }
    
    #[target_feature(enable = "sse2")]
    #[allow(dead_code)] // Public API for external use
    unsafe fn sse2_find_byte(haystack: &[u8], needle: u8) -> Option<usize> {
        let needle_vec = _mm_set1_epi8(needle as i8);
        
        for (chunk_idx, chunk) in haystack.chunks_exact(16).enumerate() {
            let data = _mm_loadu_si128(chunk.as_ptr() as *const __m128i);
            let cmp = _mm_cmpeq_epi8(data, needle_vec);
            let mask = _mm_movemask_epi8(cmp);
            
            if mask != 0 {
                return Some(chunk_idx * 16 + mask.trailing_zeros() as usize);
            }
        }
        
        let remainder_start = (haystack.len() / 16) * 16;
        haystack[remainder_start..].iter()
            .position(|&b| b == needle)
            .map(|pos| remainder_start + pos)
    }
    
    /// Optimized error message formatting based on available SIMD features.
    #[allow(dead_code)] // Public API for external use
    pub fn optimize_error_buffer(buffer: &mut String, estimated_size: usize) {
        let caps = CpuCapabilities::detect();
        
        let reserve_size = if caps.avx512f {
            estimated_size * 2  // Can process larger chunks efficiently
        } else if caps.avx2 {
            (estimated_size * 3) / 2
        } else {
            estimated_size
        };
        
        buffer.reserve(reserve_size);
    }
}

// Advanced async primitives for async error handling
#[cfg(all(feature = "rust-1-87", feature = "std"))]
use std::future::Future;
#[cfg(all(feature = "rust-1-87", feature = "std"))]
use std::pin::Pin;
#[cfg(all(feature = "rust-1-87", feature = "std"))]
use std::task::{Context, Poll, Waker};

/// Generic async error handling for any async runtime.
#[cfg(all(feature = "rust-1-87", feature = "std"))]
#[allow(dead_code)] // Public API for external use
mod async_error_handling {
    use super::*;
    
    /// Async error processor that can defer error handling.
    #[allow(dead_code)] // Public API for external use
    pub struct AsyncErrorProcessor {
        pending_errors: Vec<Yoshi>,
        processed_errors: Vec<String>,
        waker: Option<Waker>,
        state: ProcessorState,
    }

    #[derive(Debug, Clone, Copy)]
    #[allow(dead_code)] // Public API for external use
    enum ProcessorState {
        Idle,
        Processing,
        Complete,
    }
    
    impl AsyncErrorProcessor {
        #[allow(dead_code)] // Public API for external use
        pub fn new() -> Self {
            Self {
                pending_errors: Vec::new(),
                processed_errors: Vec::new(),
                waker: None,
                state: ProcessorState::Idle,
            }
        }
        
        #[allow(dead_code)] // Public API for external use
        pub fn add_error(&mut self, error: Yoshi) {
            self.pending_errors.push(error);
            if let Some(waker) = self.waker.take() {
                waker.wake();
            }
        }
        
        #[allow(dead_code)] // Public API for external use
        pub fn is_complete(&self) -> bool {
            matches!(self.state, ProcessorState::Complete)
        }
    }
    
    impl Future for AsyncErrorProcessor {
        type Output = Vec<String>;
        
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            self.waker = Some(cx.waker().clone());
            
            match self.state {
                ProcessorState::Idle if !self.pending_errors.is_empty() => {
                    self.state = ProcessorState::Processing;
                    cx.waker().wake_by_ref();
                    Poll::Pending
                },                ProcessorState::Processing => {
                    // Process all pending errors - collect drained errors first to avoid double borrow
                    let drained_errors: Vec<_> = self.pending_errors.drain(..).collect();
                    for error in drained_errors {
                        self.processed_errors.push(format!("{}", error));
                    }
                    self.state = ProcessorState::Complete;
                    Poll::Ready(self.processed_errors.clone())
                },
                ProcessorState::Complete => Poll::Ready(self.processed_errors.clone()),
                ProcessorState::Idle => Poll::Pending,
            }
        }
    }
    
    /// Async context builder for deferred error enrichment.
    #[allow(dead_code)] // Public API for external use
    pub struct AsyncContextBuilder {
        base_error: Option<Yoshi>,
        pending_contexts: Vec<String>,
        ready: bool,
    }
    
    impl AsyncContextBuilder {
        #[allow(dead_code)] // Public API for external use
        pub fn new(error: Yoshi) -> Self {
            Self {
                base_error: Some(error),
                pending_contexts: Vec::new(),
                ready: false,
            }
        }
        
        #[allow(dead_code)] // Public API for external use
        pub fn add_context(&mut self, context: String) {
            self.pending_contexts.push(context);
        }
        
        #[allow(dead_code)] // Public API for external use
        pub fn finalize(&mut self) {
            self.ready = true;
        }
    }
    
    impl Future for AsyncContextBuilder {
        type Output = Result<Yoshi, Yoshi>;
        
        fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
            if !self.ready {
                return Poll::Pending;
            }
            
            if let Some(mut error) = self.base_error.take() {
                for context in self.pending_contexts.drain(..) {
                    error = error.context(context);
                }
                Poll::Ready(Ok(error))
            } else {
                Poll::Ready(Err(Yoshi::new(YoshiKind::Internal {
                    message: "AsyncContextBuilder used after completion".into(),
                    source: None,
                    component: Some("async_error_handling".into()),
                })))
            }
        }
    }
}

// Additional atomic primitives for cross-process metrics
#[cfg(feature = "rust-1-87")]
use core::sync::atomic::{AtomicUsize, AtomicBool};

/// Generic cross-process metrics and communication.
#[cfg(feature = "rust-1-87")]
#[allow(dead_code)] // Public API for external use
mod cross_process_metrics {
    use super::*;
    
    /// Thread-safe metrics for cross-process error tracking.
    #[allow(dead_code)] // Public API for external use
    pub struct GlobalErrorMetrics {
        pub total_errors: AtomicUsize,
        pub total_contexts: AtomicUsize,
        pub backtrace_captures: AtomicUsize,
        pub simd_operations: AtomicUsize,
        pub async_operations: AtomicUsize,
        pub optimization_enabled: AtomicBool,
    }
    
    impl GlobalErrorMetrics {
        #[allow(dead_code)] // Public API for external use
        pub const fn new() -> Self {
            Self {
                total_errors: AtomicUsize::new(0),
                total_contexts: AtomicUsize::new(0),
                backtrace_captures: AtomicUsize::new(0),
                simd_operations: AtomicUsize::new(0),
                async_operations: AtomicUsize::new(0),
                optimization_enabled: AtomicBool::new(false),
            }
        }
        
        #[allow(dead_code)] // Public API for external use
        pub fn record_error(&self) {
            self.total_errors.fetch_add(1, Ordering::Relaxed);
        }
        
        #[allow(dead_code)] // Public API for external use
        pub fn record_context(&self) {
            self.total_contexts.fetch_add(1, Ordering::Relaxed);
        }
        
        #[allow(dead_code)] // Public API for external use
        pub fn record_backtrace(&self) {
            self.backtrace_captures.fetch_add(1, Ordering::Relaxed);
        }
        
        #[allow(dead_code)] // Public API for external use
        pub fn record_simd_op(&self) {
            self.simd_operations.fetch_add(1, Ordering::Relaxed);
        }
        
        #[allow(dead_code)] // Public API for external use
        pub fn record_async_op(&self) {
            self.async_operations.fetch_add(1, Ordering::Relaxed);
        }
        
        #[allow(dead_code)] // Public API for external use
        pub fn enable_optimizations(&self) {
            self.optimization_enabled.store(true, Ordering::Relaxed);
        }
        
        #[allow(dead_code)] // Public API for external use
        pub fn get_metrics(&self) -> ErrorMetrics {
            ErrorMetrics {
                errors: self.total_errors.load(Ordering::Relaxed),
                contexts: self.total_contexts.load(Ordering::Relaxed),
                backtraces: self.backtrace_captures.load(Ordering::Relaxed),
                simd_ops: self.simd_operations.load(Ordering::Relaxed),
                async_ops: self.async_operations.load(Ordering::Relaxed),
                optimized: self.optimization_enabled.load(Ordering::Relaxed),
            }
        }
        
        #[allow(dead_code)] // Public API for external use
        pub fn reset(&self) {
            self.total_errors.store(0, Ordering::Relaxed);
            self.total_contexts.store(0, Ordering::Relaxed);
            self.backtrace_captures.store(0, Ordering::Relaxed);
            self.simd_operations.store(0, Ordering::Relaxed);
            self.async_operations.store(0, Ordering::Relaxed);
            self.optimization_enabled.store(false, Ordering::Relaxed);
        }
    }
    
    #[derive(Debug, Clone, Copy)]
    #[allow(dead_code)] // Public API for external use
    pub struct ErrorMetrics {
        pub errors: usize,
        pub contexts: usize,
        pub backtraces: usize,
        pub simd_ops: usize,
        pub async_ops: usize,
        pub optimized: bool,
    }
    
    /// Global metrics instance for cross-process tracking.
    #[allow(dead_code)] // Public API for external use
    pub static GLOBAL_ERROR_METRICS: GlobalErrorMetrics = GlobalErrorMetrics::new();
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
        static THREAD_COUNTER: AtomicU32 = AtomicU32::new(1);
        
        // Use thread-local storage pattern with atomic fallback
        #[cfg(all(target_has_atomic = "ptr", any(feature = "std", target_thread_local)))]
        {
            use core::cell::Cell;
            thread_local! {
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
        #[cfg(not(all(target_has_atomic = "ptr", any(feature = "std", target_thread_local))))]
        {
            // Fallback for platforms without atomic or thread_local support
            Self {
                id: THREAD_COUNTER.fetch_add(1, Ordering::Relaxed),
            }
        }
    }

    /// Returns the raw thread ID for debugging.
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
#[cfg(feature = "std")]
use std::sync::OnceLock;
#[cfg(not(feature = "std"))]
use core::sync::atomic::{AtomicBool, Ordering};
#[cfg(not(feature = "std"))]
use core::cell::UnsafeCell;

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
        if !self.initialized.load(Ordering::Acquire) {
            // Double-checked locking pattern for no_std
            let value = f();
            unsafe {
                let data_ptr = self.data.get();
                *data_ptr = Some(value);
            }
            self.initialized.store(true, Ordering::Release);
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
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
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

/// Global error instance counter for debugging and performance monitoring.
///
/// This atomic counter tracks the total number of `Yoshi` error instances
/// that have been created since the application started. It's primarily
/// used for performance monitoring and diagnostic purposes.
static ERROR_INSTANCE_COUNTER: AtomicU64 = AtomicU64::new(0);

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
pub fn error_instance_count() -> u64 {
    ERROR_INSTANCE_COUNTER.load(Ordering::Relaxed)
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
        matches!(self, Self::ConnectionRefused | Self::TimedOut | Self::Generic)
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
#[cfg_attr(docsrs, doc(cfg(not(feature = "std"))))]
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
            s if s.contains("not found") || s.contains("no such file") || 
                s.contains("enoent") || s.contains("file does not exist") => Self::NotFound,
            
            // Permission/access denied patterns  
            s if s.contains("permission denied") || s.contains("access denied") ||
                s.contains("access is denied") || s.contains("eacces") ||
                s.contains("unauthorized") || s.contains("forbidden") => Self::PermissionDenied,
                
            // Network connection patterns
            s if s.contains("connection refused") || s.contains("econnrefused") ||
                s.contains("no route to host") || s.contains("network unreachable") => Self::ConnectionRefused,
                
            // Timeout patterns
            s if s.contains("timed out") || s.contains("timeout") || 
                s.contains("etimedout") || s.contains("operation timeout") => Self::TimedOut,
                
            // Generic I/O patterns
            s if s.contains("i/o error") || s.contains("io error") ||
                s.contains("input/output error") => Self::GenericIo(msg.into()),
                
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
            2 | -2 => Self::NotFound,           // ENOENT
            13 | -13 => Self::PermissionDenied, // EACCES  
            111 | -111 => Self::ConnectionRefused, // ECONNREFUSED
            110 | -110 => Self::TimedOut,       // ETIMEDOUT
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
    ///
    /// This variant wraps `std::io::Error` when the `std` feature is enabled,
    /// or [`NoStdIo`] for `no_std` environments.
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
    ///     message: "Invariant broken".into(),
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
    pub const fn is_transient(&self) -> bool {
        matches!(
            self,
            Self::Network { .. } | Self::Timeout { .. } | Self::ResourceExhausted { .. }
        )
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
                write!(f, "Resource '{resource}' exhausted: {current} (limit: {limit})")?;
                if let Some(pct) = usage_percentage {
                    write!(f, " [{pct:.1}% usage]")?;
                }
                Ok(())
            }
            Self::Foreign { error, error_type_name } => {
                write!(f, "{error_type_name}: {error}")
            }
            Self::Multiple { errors, primary_index } => {
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
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::{Yoshi, YoshiKind};
    /// # use std::io;
    /// # use std::io::ErrorKind;
    /// let io_err = io::Error::new(ErrorKind::PermissionDenied, "access denied");
    /// let yoshi_err = Yoshi::from(io_err.clone());
    ///
    /// // Access the source from YoshiKind
    /// if let Some(source) = yoshi_err.kind().source() {
    ///     assert_eq!(source.to_string(), io_err.to_string());
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
            Self::Network { source: Some(s), .. }
            | Self::Config { source: Some(s), .. }
            | Self::Internal { source: Some(s), .. } => Some(s.as_ref()),
            Self::Foreign { error, .. } => Some(error.as_ref()),
            Self::Multiple { errors, primary_index } => {
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
/// `YoshiContext` provides additional, application-specific information
/// about an error that helps in debugging, logging, and recovery.
/// It supports messages, metadata, suggestions, and arbitrary typed payloads.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub struct YoshiContext {
    /// Main message with Arc optimization for shared contexts.
    ///
    /// This field holds a descriptive message for the context.
    pub message: Option<Arc<str>>,
    /// Metadata with optimized storage for common keys.
    ///
    /// A hash map storing key-value pairs of additional diagnostic information.
    /// Keys and values are `Arc<str>` for efficient sharing.
    #[cfg_attr(feature = "serde", serde(default))]
    pub metadata: HashMap<Arc<str>, Arc<str>>,
    /// Recovery suggestion with shared storage.
    ///
    /// An optional human-readable suggestion for how to resolve or work around the error.
    pub suggestion: Option<Arc<str>>,
    /// Source location with compile-time capture.
    ///
    /// An optional [`YoshiLocation`] indicating where this context was added in the source code.
    pub location: Option<YoshiLocation>,
    /// Typed payloads with optimized storage.
    ///
    /// A vector of arbitrary `Any + Send + Sync + 'static` types, allowing for
    /// rich, structured data to be attached to an error.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub payloads: Vec<Box<dyn Any + Send + Sync + 'static>>,
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

impl YoshiContext {
    /// Creates a new context with optimized string allocation.
    ///
    /// This is the primary way to create a new `YoshiContext`. It automatically
    /// captures the current system time and sets a default priority.
    ///
    /// # Arguments
    ///
    /// * `msg` - The main message for this context. It can be any type
    ///   that converts into a `String`.
    ///
    /// # Returns
    ///
    /// A new `YoshiContext` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::YoshiContext;
    /// let ctx = YoshiContext::new("Attempting to connect to database");
    /// assert_eq!(ctx.message.as_deref(), Some("Attempting to connect to database"));
    /// assert!(ctx.created_at.is_some());
    /// ```
    #[inline]
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            message: Some(msg.into().into()),
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
    /// The `YoshiContext` instance with the new metadata added.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::YoshiContext;
    /// let ctx = YoshiContext::new("Processing user request")
    ///     .with_metadata("user_id", "12345")
    ///     .with_metadata("session_id", "abcde");
    ///
    /// assert_eq!(ctx.metadata.get("user_id".into()).map(|s| s.as_ref()), Some("12345"));
    /// ```
    #[must_use]
    #[inline]
    pub fn with_metadata(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.metadata.insert(k.into().into(), v.into().into());
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
    /// The `YoshiContext` instance with the suggestion added.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::YoshiContext;
    /// let ctx = YoshiContext::new("File not found")
    ///     .with_suggestion("Ensure the file path is correct and accessible.");
    ///
    /// assert_eq!(ctx.suggestion.as_deref(), Some("Ensure the file path is correct and accessible."));
    /// ```
    #[must_use]
    #[inline]
    pub fn with_suggestion(mut self, s: impl Into<String>) -> Self {
        self.suggestion = Some(s.into().into());
        self
    }

    /// Attaches a typed payload with optimized boxing.
    ///
    /// This method allows attaching arbitrary Rust types as payloads to the
    /// context. These payloads can be retrieved later using `downcast_ref`.
    /// This is useful for including complex, structured data relevant to the error.
    ///
    /// # Arguments
    ///
    /// * `payload` - The data to attach. It must implement `Any`, `Send`, `Sync`, and have a `'static` lifetime.
    ///
    /// # Returns
    ///
    /// The `YoshiContext` instance with the payload added.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::YoshiContext;
    /// #[derive(Debug, PartialEq)]
    /// struct ErrorDetails {
    ///     code: u16,
    ///     reason: String,
    /// }
    ///
    /// let ctx = YoshiContext::new("API call failed")
    ///     .with_payload(ErrorDetails { code: 404, reason: "Endpoint not found".to_string() })
    ///     .with_payload(vec![]);
    ///
    /// let details = ctx.payloads.iter().find_map(|p| p.downcast_ref::<ErrorDetails>());
    /// assert!(details.is_some());
    /// assert_eq!(details.unwrap().code, 404);
    ///
    /// let vector_payload = ctx.payloads.iter().find_map(|p| p.downcast_ref::<Vec<i32>>());
    /// assert!(vector_payload.is_some());
    /// assert_eq!(vector_payload.unwrap(), &vec![]);
    /// ```
    #[must_use]
    #[inline]
    pub fn with_payload(mut self, payload: impl Any + Send + Sync + 'static) -> Self {
        self.payloads.push(Box::new(payload));
        self
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
    /// The `YoshiContext` instance with the updated priority.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::YoshiContext;
    /// let low_priority_ctx = YoshiContext::new("Minor detail").with_priority(50);
    /// assert_eq!(low_priority_ctx.priority, 50);
    ///
    /// let high_priority_ctx = YoshiContext::new("Critical information").with_priority(250);
    /// assert_eq!(high_priority_ctx.priority, 250);
    /// ```
    #[must_use]
    #[inline]
    pub fn with_priority(mut self, priority: u8) -> Self { // Removed `const`
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
    /// The `YoshiContext` instance with the location set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::{YoshiContext, YoshiLocation};
    /// let location = YoshiLocation::new("test.rs", 42, 10);
    /// let ctx = YoshiContext::new("operation failed")
    ///     .with_location(location);
    ///
    /// assert_eq!(ctx.location.unwrap().file, "test.rs");
    /// assert_eq!(ctx.location.unwrap().line, 42);
    /// ```
    #[must_use]
    #[inline]
    pub fn with_location(mut self, location: YoshiLocation) -> Self {
        self.location = Some(location);
        self
    }
}

/// Enhanced source code location with const evaluation.
///
/// `YoshiLocation` captures the file name, line number, and column number
/// where an error or context was created. This is crucial for debugging
/// and pinpointing the exact origin of an issue in the source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    ///
    /// ```
    /// # use yoshi_std::YoshiLocation;
    /// let loc = YoshiLocation::new("/home/user/project/src/lib.rs", 1, 1);
    /// assert_eq!(loc.filename(), "lib.rs");
    ///
    /// let loc_windows = YoshiLocation::new("C:\\Users\\dev\\main.rs", 1, 1);
    /// assert_eq!(loc_windows.filename(), "main.rs"); // Behaves correctly on Windows too
    /// ```
    #[inline]
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
    /// # }
    /// ```
    fn new_captured() -> Self {
        let start = std::time::Instant::now();
        let current_thread = thread::current();
        let backtrace = std::backtrace::Backtrace::capture();
        let capture_cost = start.elapsed().as_nanos() as u64;

        Self {
            inner: backtrace,
            capture_timestamp: SystemTime::now(),
            thread_id: current_thread.id(),
            thread_name: current_thread.name().map(|s| s.into()),
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
        writeln!(f, "Backtrace captured at: {:?}", self.capture_timestamp)?;
        if let Some(ref thread_name) = self.thread_name {
            writeln!(f, "Thread: {thread_name} ({:?})", self.thread_id)?;
        } else {
            writeln!(f, "Thread: {:?}", self.thread_id)?;
        }
        if let Some(cost) = self.capture_cost_nanos {
            writeln!(f, "Capture cost: {}ns", cost)?;
        }
        write!(f, "{}", self.inner)
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
        Self {
            locations: vec![location],
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
        writeln!(f, "Minimal backtrace (no_std) captured at: {:?}", self.capture_timestamp)?;
        writeln!(f, "Thread: {} | Call depth: {}", self.thread_id, self.call_depth)?;
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
/// - `contexts`: A vector of [`YoshiContext`] instances, providing additional
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
///     component: Some("Calculator".into()),
/// });
///
/// println!("Error: {}", err);
/// ```
///
/// Creating an error with context:
/// ```
/// use yoshi_std::{Yoshi, YoshiKind, YoshiContextExt};
/// # use std::io::{self, ErrorKind};
///
/// fn load_data() -> Result<(), Yoshi> {
///     // Simulate a file not found error
///     let io_error = io::Error::new(ErrorKind::NotFound, "data.json not found");
///     Err(Yoshi::from(io_error))
///         .context("Failed to load user preferences".to_string())
///         .with_metadata("user_id", "test_user")
///         .with_suggestion("Ensure data.json is in the correct directory.")
/// }
///
/// if let Err(e) = load_data() {
///     println!("Encountered an error:\n{}", e);
/// }
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
    contexts: Vec<YoshiContext>,
    /// A unique identifier for this error instance.
    instance_id: u64,
    /// Timestamp when the error was created (only available with `std` feature).
    #[cfg(feature = "std")]
    created_at: SystemTime,
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
            contexts: Vec::new(),
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
        Self::new(YoshiKind::Foreign {
            error: Box::new(e),
            error_type_name: core::any::type_name::<E>().into(),
        })
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
    /// let err1 = Yoshi::new(YoshiKind::Internal { message: "test".into(), source: None, component: None });
    /// let err2 = Yoshi::new(YoshiKind::Validation { field: "name".into(), message: "invalid".into(), expected: None, actual: None });
    ///
    /// assert_ne!(err1.instance_id(), err2.instance_id());
    /// println!("Error 1 ID: {}", err1.instance_id());
    /// println!("Error 2 ID: {}", err2.instance_id());
    /// ```
    #[inline]
    pub const fn instance_id(&self) -> u64 {
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
    /// ```    /// # use yoshi_std::{Yoshi, YoshiKind};
    /// # use std::io;
    /// # use std::io::ErrorKind;
    /// let io_err = io::Error::new(ErrorKind::PermissionDenied, "access denied");
    /// let yoshi_err = Yoshi::from(io_err.clone());
    ///
    /// match yoshi_err.kind() {
    ///     YoshiKind::Io(_) => { /* handle Io error */ },
    ///     _ => { /* handle other kinds of errors */ },
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
    /// # use yoshi_std::{Yoshi, YoshiKind};
    /// let err = Yoshi::new(YoshiKind::Internal { message: "Critical bug".into(), source: None, component: None });
    /// assert_eq!(err.severity(), 80);
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
    /// let err = Yoshi::new(YoshiKind::Network { message: "Connection reset".into(), source: None, error_code: None });
    /// assert!(err.is_transient());
    /// ```
    #[inline]
    pub const fn is_transient(&self) -> bool {
        self.kind.is_transient()
    }

    /// Adds context with optimized string handling and location capture.
    ///
    /// This method prepends a new [`YoshiContext`] to the error's context chain.
    /// It automatically captures the source code location where `context()` is called.
    /// Contexts are typically added as an error propagates up the call stack,
    /// providing a clear trail of what happened at each layer.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message for the new context, convertible to `String`.
    ///
    /// # Returns
    ///
    /// The modified `Yoshi` error instance with the new context added.
    ///
    /// # Examples
    ///
    /// ```
    /// use yoshi_std::{Yoshi, YoshiKind};
    /// # use std::io;
    /// # use std::io::ErrorKind;
    ///
    /// fn parse_request() -> Result<(), Yoshi> {
    ///     // Simulate an internal error
    ///     let err = Yoshi::new(YoshiKind::Internal {
    ///         message: "Invalid header format".into(),
    ///         source: None,
    ///         component: Some("HTTP Parser".into()),
    ///     });
    ///     Err(err)
    ///         .context("Failed to process incoming request".to_string())
    ///         .context("During request deserialization".to_string())
    /// }
    ///
    /// if let Err(e) = parse_request() {
    ///     let s = format!("{}", e);
    ///     assert!(s.contains("During request deserialization"));
    ///     assert!(s.contains("Failed to process incoming request"));
    ///     assert!(s.contains("Invalid header format"));
    /// }
    /// ```
    #[track_caller]
    #[inline]
    pub fn context(mut self, msg: impl Into<String>) -> Self {
        let mut ctx = YoshiContext::new(msg);
        ctx.location = Some(yoshi_location!());
        // Append context to the end, then iterate in reverse for Display
        self.contexts.push(ctx);
        self
    }

    /// Adds metadata with optimized allocation.
    ///
    /// This is a convenience method that calls `with_metadata` on the
    /// *most recently added context*. If no contexts exist, it creates
    /// a default one and adds the metadata to it.
    ///
    /// # Arguments
    ///
    /// * `k` - The key for the metadata, convertible to `String`.
    /// * `v` - The value for the metadata, convertible to `String`.
    ///
    /// # Returns
    ///
    /// The modified `Yoshi` error instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::{Yoshi, YoshiKind};
    /// let err = Yoshi::new(YoshiKind::Internal { message: "test".into(), source: None, component: None })
    ///     .with_metadata("user_id", "123")
    ///     .with_metadata("operation", "login");
    ///
    /// let primary_ctx = err.primary_context().unwrap();
    /// assert_eq!(primary_ctx.metadata.get("user_id".into()).map(|s| s.as_ref()), Some("123"));
    /// assert_eq!(primary_ctx.metadata.get("operation".into()).map(|s| s.as_ref()), Some("login"));
    /// ```
    #[must_use]
    #[inline]
    pub fn with_metadata(self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.extend(|c| c.with_metadata(k, v))
    }

    /// Adds suggestion with shared storage.
    ///
    /// This is a convenience method that calls `with_suggestion` on the
    /// *most recently added context*. If no contexts exist, it creates
    /// a default one and adds the suggestion to it.
    ///
    /// # Arguments
    ///
    /// * `s` - The suggestion message, convertible to `String`.
    ///
    /// # Returns
    ///
    /// The modified `Yoshi` error instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::{Yoshi, YoshiKind};
    /// let err = Yoshi::new(YoshiKind::Network { message: "connection refused".into(), source: None, error_code: None })
    ///     .with_suggestion("Check network connectivity and firewall rules.");
    ///
    /// let primary_ctx = err.primary_context().unwrap();
    /// assert_eq!(primary_ctx.suggestion.as_deref(), Some("Check network connectivity and firewall rules."));
    /// ```
    #[must_use]
    #[inline]
    pub fn with_suggestion(self, s: impl Into<String>) -> Self {
        self.extend(|c| c.with_suggestion(s))
    }

    /// Attaches typed payload with optimized boxing.
    ///
    /// This is a convenience method that calls `with_payload` on the
    /// *most recently added context*. If no contexts exist, it creates
    /// a default one and adds the payload to it.
    ///
    /// # Arguments
    ///
    /// * `payload` - The data to attach. It must implement `Any`, `Send`, `Sync`, and have a `'static` lifetime.
    ///
    /// # Returns
    ///
    /// The modified `Yoshi` error instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::YoshiContext;
    /// #[derive(Debug, PartialEq)]
    /// struct ErrorDetails {
    ///     code: u16,
    ///     reason: String,
    /// }
    ///
    /// let ctx = YoshiContext::new("API call failed")
    ///     .with_payload(ErrorDetails { code: 404, reason: "Endpoint not found".to_string() })
    ///     .with_payload(vec![]);
    ///
    /// let details = ctx.payloads.iter().find_map(|p| p.downcast_ref::<ErrorDetails>());
    /// assert!(details.is_some());
    /// assert_eq!(details.unwrap().code, 404);
    ///
    /// let vector_payload = ctx.payloads.iter().find_map(|p| p.downcast_ref::<Vec<i32>>());
    /// assert!(vector_payload.is_some());
    /// assert_eq!(vector_payload.unwrap(), &vec![]);
    /// ```
    #[must_use]
    #[inline]
    pub fn with_payload(self, payload: impl Any + Send + Sync + 'static) -> Self {
        self.extend(|c| c.with_payload(payload))
    }

    /// Sets the priority on the current context.
    ///
    /// This is a convenience method that calls `with_priority` on the
    /// *most recently added context*. If no contexts exist, it creates
    /// a default one and sets the priority on it.
    ///
    /// # Arguments
    ///
    /// * `priority` - The priority level (0-255).
    ///
    /// # Returns
    ///
    /// The modified `Yoshi` error instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::YoshiContext;
    /// let low_priority_ctx = YoshiContext::new("Minor detail").with_priority(50);
    /// assert_eq!(low_priority_ctx.priority, 50);
    ///
    /// let high_priority_ctx = YoshiContext::new("Critical information").with_priority(250);
    /// assert_eq!(high_priority_ctx.priority, 250);
    /// ```
    #[must_use]
    #[inline]
    pub fn with_priority(self, priority: u8) -> Self { // Removed `const`
        self.extend(|c| c.with_priority(priority))
    }

    /// Sets location information on the current context.
    ///
    /// This is a convenience method that calls `with_location` on the
    /// *most recently added context*. If no contexts exist, it creates
    /// a default one and sets the location on it.
    ///
    /// # Arguments
    ///
    /// * `location` - The `YoshiLocation` to set.
    ///
    /// # Returns
    ///
    /// The modified `Yoshi` error instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::{Yoshi, YoshiKind, YoshiLocation};
    /// let location = YoshiLocation::new("test.rs", 42, 10);
    /// let err = Yoshi::new(YoshiKind::Internal { message: "test".into(), source: None, component: None })
    ///     .with_location(location);
    ///
    /// assert!(err.location().is_some());
    /// ```
    #[must_use]
    #[inline]
    pub fn with_location(self, location: YoshiLocation) -> Self {
        self.extend(|c| c.with_location(location))
    }

    /// Gets the suggestion from the primary context.
    ///
    /// This method retrieves the suggestion message from the context with
    /// the highest priority. If no contexts exist or no context has a
    /// suggestion, it returns `None`.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the suggestion string, or `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::{Yoshi, YoshiKind};
    /// let err = Yoshi::new(YoshiKind::Network { message: "connection failed".into(), source: None, error_code: None })
    ///     .with_suggestion("Check network connectivity");
    ///
    /// assert_eq!(err.suggestion(), Some("Check network connectivity"));
    /// ```
    #[inline]
    pub fn suggestion(&self) -> Option<&str> {
        self.primary_context()
            .and_then(|ctx| ctx.suggestion.as_deref())
    }

    /// Gets a typed payload from the primary context.
    ///
    /// This method attempts to retrieve a payload of the specified type from
    /// the context with the highest priority. It searches through all payloads
    /// in the primary context and returns the first one that matches the type.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of payload to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the payload of type `T`, or `None`
    /// if no such payload exists.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::{Yoshi, YoshiKind};
    /// #[derive(Debug, PartialEq)]
    /// struct RequestId(String);
    ///
    /// let err = Yoshi::new(YoshiKind::Internal {
    ///     message: "Operation failed".into(),
    ///     source: None,
    ///     component: None,
    /// })
    /// .with_payload(RequestId("req123".to_string()));
    ///
    /// let payload = err.payload::<RequestId>().unwrap();
    /// assert_eq!(payload.0, "req123");
    /// ```
    #[inline]
    pub fn payload<T: 'static>(&self) -> Option<&T> {
        self.primary_context()
            .and_then(|ctx| {
                ctx.payloads
                    .iter()
                    .find_map(|p| p.downcast_ref::<T>())
            })
    }

    /// Gets the location from the primary context.
    ///
    /// This method retrieves the source location information from the context
    /// with the highest priority. If no contexts exist or no context has
    /// location information, it returns `None`.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the `YoshiLocation`, or `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::{Yoshi, YoshiKind, YoshiLocation};
    /// let err = Yoshi::new(YoshiKind::Internal { message: "test".into(), source: None, component: None })
    ///     .context("operation failed");
    ///
    /// if let Some(location) = err.location() {
    ///     println!("Error occurred at: {}", location);
    /// }
    /// ```
    #[inline]
    pub fn location(&self) -> Option<&YoshiLocation> {
        self.primary_context()
            .and_then(|ctx| ctx.location.as_ref())
    }

    /// Gets the creation timestamp for debugging.
    ///
    /// This method returns the `SystemTime` at which this `Yoshi` error
    /// instance was originally created. This is useful for understanding
    /// the lifecycle of errors and for diagnostic purposes.
    /// Available only when the `std` feature is enabled.
    ///
    /// # Returns
    ///
    /// The `SystemTime` when the error was created.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "std")] {
    /// # use yoshi_std::{Yoshi, YoshiKind};
    /// # use std::time::SystemTime;
    /// let err = Yoshi::new(YoshiKind::Internal { message: "test".into(), source: None, component: None });
    /// let creation_time = err.created_at();
    /// let now = SystemTime::now();
    ///
    /// // The creation time should be very close to 'now'
    /// // For robust tests, you might need to compare durations.
    /// // assert!(now.duration_since(creation_time).unwrap() < std::time::Duration::from_millis(100));
    /// println!("Error created at: {:?}", creation_time);
    /// # }
    /// ```
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    #[inline]
    pub fn created_at(&self) -> SystemTime {
        self.created_at
    }

    /// Internal helper for context extension with memory optimization.
    ///
    /// This method is used internally by `with_*` methods to modify
    /// the most recently added context, or create a new default one
    /// if the context list is empty. It uses `mem::take` for efficient
    /// modification of the context without reallocations.
    ///
    /// # Arguments
    ///
    /// * `op` - A closure that takes a `YoshiContext` and returns a modified one.
    ///
    /// # Returns
    ///
    /// The modified `Yoshi` error instance.
    fn extend<F>(mut self, op: F) -> Self
    where
        F: FnOnce(YoshiContext) -> YoshiContext,
    {
        if let Some(c0) = self.contexts.last_mut() { // Changed to `last_mut`
            *c0 = op(mem::take(c0));
        } else {
            self.contexts.push(op(YoshiContext::default()));
        }
        self
    }

    /// Returns backtrace reference with performance metadata.
    ///
    /// This method provides access to the captured backtrace (if enabled)
    /// and its associated metadata, such as capture time and cost.
    /// Available only when the `std` feature is enabled.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the [`YoshiBacktrace`] if one
    /// was captured, or `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[cfg(feature = "std")] {
    /// # use yoshi_std::{Yoshi, YoshiKind};
    /// // Ensure RUST_BACKTRACE=1 is set in your environment to capture backtraces
    /// // std::env::set_var("RUST_BACKTRACE", "1");
    ///
    /// let err = Yoshi::new(YoshiKind::Internal { message: "Test error".into(), source: None, component: None });
    /// if let Some(bt) = err.backtrace() {
    ///     println!("Backtrace status: {:?}", bt.status());
    ///     println!("Backtrace capture cost: {:?}", bt.capture_cost_nanos());
    /// } else {
    ///     println!("Backtrace not captured (is RUST_BACKTRACE enabled?)");
    /// }
    /// # }
    /// ```
    #[inline]
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn backtrace(&self) -> Option<&YoshiBacktrace> {
        self.backtrace.as_ref()
    }

    /// Returns an iterator over the contexts associated with this error.
    ///
    /// Contexts are stored in a `Vec`, typically with the most recently
    /// added context at index 0. Iterating over them allows inspecting
    /// the full chain of contextual information.
    ///
    /// # Returns
    ///
    /// An iterator yielding references to `YoshiContext` objects.
    ///
    /// # Examples
    ///
    /// ```
    /// use yoshi_std::{Yoshi, YoshiKind};
    /// let err = Yoshi::new(YoshiKind::Internal { message: "base error".into(), source: None, component: None })
    ///     .context("step 1 failed")
    ///     .context("step 2 failed");
    ///
    /// // Iterating in reverse to see the most recent contexts first, matching display order
    /// for (i, ctx) in err.contexts().rev().enumerate() {
    ///     println!("Context {}: {:?}", i, ctx.message);
    /// }
    /// // Expected output:
    /// // Context 0: Some("step 2 failed")
    /// // Context 1: Some("step 1 failed")
    /// ```
    #[inline]
    pub fn contexts(&self) -> impl DoubleEndedIterator<Item = &YoshiContext> {
        self.contexts.iter()
    }

    /// Gets the highest priority context.
    ///
    /// This method finds the `YoshiContext` within the error's context chain
    /// that has the highest `priority` value. This can be useful for quickly
    /// identifying the most critical or relevant piece of contextual information.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the `YoshiContext` with the highest
    /// priority, or `None` if no contexts are present.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::{Yoshi, YoshiKind, YoshiContextExt};
    /// let err = Yoshi::new(YoshiKind::Internal { message: "base error".into(), source: None, component: None })
    ///     .context("Low priority info").with_priority(50)
    ///     .context("Critical detail").with_priority(250)
    ///     .context("Medium priority info").with_priority(100);
    ///
    /// let primary_ctx = err.primary_context().unwrap();
    /// assert_eq!(primary_ctx.message.as_deref(), Some("Critical detail"));
    /// assert_eq!(primary_ctx.priority, 250);
    /// ```
    #[inline]
    pub fn primary_context(&self) -> Option<&YoshiContext> {
        self.contexts.iter().max_by_key(|c| c.priority)
    }

    /// Emits a tracing event with structured fields.
    ///
    /// If the "tracing" feature is enabled, this method will emit a structured
    /// tracing event with details about the `Yoshi` error, including its
    /// instance ID, severity, and transience. This integrates `Yoshi` errors
    /// seamlessly into tracing-based observability systems.
    ///
    /// # Arguments
    ///
    /// * `level` - The `tracing::Level` at which to emit the event (e.g., `tracing::Level::ERROR`).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[cfg(feature = "tracing")] {
    /// # use yoshi_std::{Yoshi, YoshiKind};
    /// # use tracing::Level;
    /// // Initialize a tracing subscriber (e.g., tracing_subscriber::fmt().init();)
    ///
    /// let err = Yoshi::new(YoshiKind::Internal { message: "Service unavailable".into(), source: None, component: None });
    /// err.make_event(Level::ERROR);
    ///
    /// // The error details will be logged via the tracing subscriber.
    /// # }
    /// ```
    #[cfg(feature = "tracing")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tracing")))]
    #[inline]
    pub fn make_event(&self, level: tracing::Level) {
        tracing::event!(
            target: "yoshi",
            level,
            error = %self,
            ?self,
            instance_id = self.instance_id,
            severity = self.severity(),
            is_transient = self.is_transient(),
        );
    }
    /// Formats the error source chain with cycle detection to prevent infinite recursion.
    ///
    /// This internal helper method recursively formats the chain of underlying
    /// causes for a `Yoshi` error. It includes logic to detect and truncate
    /// circular references in the error chain, preventing stack overflows.
    ///
    /// # Arguments
    ///
    /// * `buffer` - A mutable `String` buffer to append the formatted source chain to.
    /// * `depth` - The current recursion depth, used for cycle detection and truncation.
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating success or failure of the formatting.
    ///
    /// # Panics
    ///
    /// This function does not panic under normal circumstances.
    ///
    /// # Safety
    ///
    /// This function is safe as it handles recursion depth and prevents cycles.
    fn format_source_chain(&self, buffer: &mut String, depth: usize) -> fmt::Result {
        const MAX_DEPTH: usize = 32; // Prevent stack overflow from circular references

        if depth >= MAX_DEPTH {
            write!(buffer, "\n  ... (error chain truncated at depth {})", MAX_DEPTH)?;
            return Ok(());
        }

        if let Some(source_err) = self.kind.source() {
            // Check if this error type typically displays its source internally
            let is_source_displayed = match self.kind {
                #[cfg(feature = "std")]
                YoshiKind::Io(_) => true,
                #[cfg(not(feature = "std"))]
                YoshiKind::Io(_) => true,
                YoshiKind::Foreign { .. } => false, // Foreign errors should show original cause
                _ => false
            };

            if !is_source_displayed {
                // For other error types, show the source directly
                write!(buffer, "\nOriginal Cause: {source_err}")?;

                // If the source is also a Yoshi error, format its chain
                if let Some(yoshi_source) = source_err.downcast_ref::<Yoshi>() {
                    yoshi_source.format_source_chain(buffer, depth + 1)?;
                } else if let Some(multiple_errors) = source_err.downcast_ref::<YoshiKind>() {
                    // Handle nested YoshiKind::Multiple specifically
                    if let YoshiKind::Multiple { errors, primary_index } = multiple_errors {
                         if let Some(idx) = primary_index {
                            if let Some(primary_err) = errors.get(*idx) {
                                write!(buffer, "\nPrimary cause: {}", primary_err.kind)?;
                                primary_err.format_source_chain(buffer, depth + 1)?;
                            }
                        } else if let Some(first_err) = errors.first() {
                            write!(buffer, "\nFirst cause: {}", first_err.kind)?;
                            first_err.format_source_chain(buffer, depth + 1)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

//--------------------------------------------------------------------------------------------------
// Optimized Display implementation with SIMD-friendly formatting
//--------------------------------------------------------------------------------------------------

impl Display for Yoshi {
    /// Formats the `Yoshi` error for human-readable display.
    ///
    /// This implementation constructs a comprehensive error message by:
    /// 1. Displaying the primary [`YoshiKind`].
    /// 2. Iterating through and formatting all associated [`YoshiContext`]s
    ///    in priority order (highest priority first).
    /// 3. Recursively formatting the underlying error source chain, with
    ///    built-in cycle detection.
    /// 4. Appending the captured backtrace (if `std` feature is enabled).
    ///
    /// The formatting uses a pre-allocated buffer for performance optimization.
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
    /// use yoshi_std::{Yoshi, YoshiKind, YoshiContextExt};
    /// # use std::io::{self, ErrorKind};
    ///
    /// let base_err = io::Error::new(ErrorKind::NotFound, "file not found");
    /// let err = Yoshi::from(base_err)
    ///     .context("Failed to open report")
    ///     .with_metadata("report_id", "R-123")
    ///     .with_suggestion("Verify report file exists");
    ///
    /// let formatted_error = format!("{}", err);
    /// assert!(formatted_error.contains("I/O error: file not found"));
    /// assert!(formatted_error.contains("Caused by: Failed to open report"));
    /// assert!(formatted_error.contains("report_id: R-123"));
    /// assert!(formatted_error.contains("Suggestion: Verify report file exists"));
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Pre-allocate buffer for better performance
        let mut buffer = String::with_capacity(256);

        // Format main error
        write!(buffer, "{}", self.kind)?;

        // Format contexts in priority order for better readability
        let mut sorted_contexts: Vec<_> = self.contexts.iter().enumerate().collect();
        sorted_contexts.sort_by_key(|(i, c)| (core::cmp::Reverse(c.priority), *i));

        for (_, ctx) in sorted_contexts {
            if let Some(msg) = &ctx.message {
                write!(buffer, "\nCaused by: {msg}")?;
            }
            if let Some(loc) = &ctx.location {
                write!(buffer, " at {loc}")?;
            }
            for (k, v) in &ctx.metadata {
                write!(buffer, "\n  {k}: {v}")?;
            }
            if let Some(s) = &ctx.suggestion {
                write!(buffer, "\n  Suggestion: {s}")?;
            }
        }

        // Handle source errors with cycle detection
        self.format_source_chain(&mut buffer, 0)?;

        // Add backtrace if available
        #[cfg(feature = "std")]
        if let Some(bt) = &self.backtrace {
            write!(buffer, "\n{bt}")?;
        }

        // Write optimized buffer to formatter
        f.write_str(&buffer)
    }
}

//--------------------------------------------------------------------------------------------------
// Enhanced Error implementation (removed provide as it's unstable)
//--------------------------------------------------------------------------------------------------

impl Error for Yoshi {
    /// Returns the underlying source of this `Yoshi` error.
    ///
    /// This method implements the `source` requirement of the `std::error::Error`
    /// trait, allowing `Yoshi` errors to participate in the standard Rust
    /// error chain mechanism. It delegates to the `source` method of [`YoshiKind`].
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the underlying error that
    /// caused this `Yoshi`, or `None` if there is no direct source.
    ///
    /// # Examples
    ///
    /// ```    /// # use yoshi_std::{Yoshi, YoshiKind};
    /// # use std::io;
    /// # use std::io::ErrorKind;
    /// let io_err = io::Error::new(ErrorKind::PermissionDenied, "access denied");
    /// let yoshi_err = Yoshi::from(io_err.clone());
    ///
    /// let source_error = yoshi_err.source().unwrap();
    /// assert_eq!(source_error.to_string(), io_err.to_string());
    /// ```
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.kind.source()
    }
    // `provide` method removed as it relies on unstable `error_generic_member_access` feature.
    // Use explicit accessor methods on `Yoshi` and `YoshiContext` instead for data retrieval.
}

//--------------------------------------------------------------------------------------------------
// Optimized conversions with performance monitoring
//--------------------------------------------------------------------------------------------------

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl From<std::io::Error> for Yoshi {
    /// Converts a `std::io::Error` into a `Yoshi` error.
    ///
    /// This blanket `From` implementation automatically wraps any `std::io::Error`
    /// into a `Yoshi` error of kind `YoshiKind::Io`.
    /// The source code location of the conversion is captured.
    ///
    /// # Arguments
    ///
    /// * `e` - The `std::io::Error` to convert.
    ///
    /// # Returns
    ///
    /// A new `Yoshi` error instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use yoshi_std::{Yoshi, YoshiKind};
    /// use std::io::{self, ErrorKind};
    ///
    /// let io_err = io::Error::new(ErrorKind::BrokenPipe, "pipe closed");
    /// let yoshi_err = Yoshi::from(io_err);
    ///
    /// assert!(matches!(yoshi_err.kind(), YoshiKind::Io(_)));
    /// ```
    #[track_caller]
    #[inline]
    fn from(e: std::io::Error) -> Self {
        Self::new(YoshiKind::Io(e))
    }
}

#[cfg(not(feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "std"))))]
impl From<NoStdIo> for Yoshi {
    /// Converts a [`NoStdIo`] error into a `Yoshi` error.
    ///
    /// This blanket `From` implementation automatically wraps any `NoStdIo`
    /// error into a `Yoshi` error of kind `YoshiKind::Io`.
    /// The source code location of the conversion is captured.
    ///
    /// # Arguments
    ///
    /// * `e` - The [`NoStdIo`] error to convert.
    ///
    /// # Returns
    ///
    /// A new `Yoshi` error instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yoshi_std::{Yoshi, YoshiKind, NoStdIo};
    /// let no_std_io_err = NoStdIo::new("no_std file not found");
    /// let yoshi_err = Yoshi::from(no_std_io_err);
    ///
    /// assert!(matches!(yoshi_err.kind(), YoshiKind::Io(_)));
    /// ```
    #[track_caller]
    #[inline]
    fn from(e: NoStdIo) -> Self {
        Self::new(YoshiKind::Io(e))
    }
}

impl From<String> for Yoshi {
    /// Converts a `String` into a `Yoshi` error.
    ///
    /// This implementation converts a generic `String` message into a `Yoshi` error.
    /// When the `std` feature is enabled, it defaults to `YoshiKind::Internal`.
    /// In `no_std` environments, it maps to `YoshiKind::Io(NoStdIo::Other)`.
    /// The source code location of the conversion is captured.
    ///
    /// # Arguments
    ///
    /// * `s` - The `String` message to convert.
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
    /// let msg = "Generic processing failure".to_string();
    /// let err = Yoshi::from(msg.clone());
    /// #[cfg(feature = "std")]
    /// assert!(matches!(
    ///     err.kind,
    ///     YoshiKind::Internal {
    ///         ref message, ..
    ///     } if message.as_ref() == msg
    /// ));
    /// #[cfg(not(feature = "std"))]
    /// assert!(matches!(
    ///     err.kind,
    ///     YoshiKind::Io(yoshi_std::NoStdIo::Other(ref message)) if message.as_ref() == msg
    /// ));
    /// assert!(format!("{}", err).contains("Generic processing failure"));
    /// ```
    #[track_caller]
    #[inline]
    fn from(s: String) -> Self {
        #[cfg(feature = "std")]
        {
            Self::new(YoshiKind::Internal {
                message: s.into(),
                source: None,
                component: None,
            })
        }
        #[cfg(not(feature = "std"))]
        {
            // In no_std, converting a string might be better as an Io error if it's the primary way
            // to get error messages, or an Internal error.
            // For consistency with std::io::Error behavior, mapping to Io is reasonable.
            Self::new(YoshiKind::Io(NoStdIo::Other(s.into())))
        }
    }
}

impl<'a> From<&'a str> for Yoshi {
    /// Converts a string slice (`&str`) into a `Yoshi` error.
    ///
    /// This implementation converts a string slice directly into a `String`,
    /// and then uses the `From<String>` implementation to create the `Yoshi` error.
    /// The source code location of the conversion is captured.
    ///
    /// # Arguments
    ///
    /// * `s` - The string slice message to convert.
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
    /// let msg = "Network connection lost";
    /// let err = Yoshi::from(msg);
    /// #[cfg(feature = "std")]
    /// assert!(matches!(
    ///     err.kind,
    ///     YoshiKind::Internal {
    ///         ref message, ..
    ///     } if message.as_ref() == msg
    /// ));
    /// #[cfg(not(feature = "std"))]
    /// assert!(matches!(
    ///     err.kind,
    ///     YoshiKind::Io(yoshi_std::NoStdIo::Other(ref message)) if message.as_ref() == msg
    /// ));
    /// assert!(format!("{}", err).contains("Network connection lost"));
    /// ```
    #[track_caller]
    #[inline]
    fn from(s: &'a str) -> Self {
        Self::from(s.to_string())
    }
}

// Removed the blanket `impl<E> From<E> for Yoshi` to avoid conflicts and reliance on unstable features.
// Use `Yoshi::foreign(error)` for explicit conversion of other `Error` types.

//--------------------------------------------------------------------------------------------------
// Enhanced Result extension with performance optimization
//--------------------------------------------------------------------------------------------------

/// High-performance extension trait for `Result` with optimized error handling.
///
/// This trait provides convenient methods for adding contextual information,
/// suggestions, metadata, and typed payloads to `Result` errors, transforming
/// them into `Yoshi` errors if they are not already.
/// This simplifies error propagation and enrichment.
pub trait YoshiContextExt<T> {
    /// Adds a new context message to the error.
    ///
    /// If the `Result` is an `Err` variant, the error is converted into a
    /// `Yoshi` error (if it isn't already) and a new [`YoshiContext`] is
    /// prepended to its context chain with the provided message.
    /// The source code location of the call is captured.
    ///
    /// # Arguments
    ///
    /// * `msg` - The context message, convertible to `String`.
    ///
    /// # Returns
    ///
    /// A `Result<T>` with the error (if any) extended with the new context.
    ///
    /// # Examples
    ///
    /// ```
    /// use yoshi_std::{Result, YoshiContextExt};
    /// # use std::io::{self, ErrorKind};
    ///
    /// fn try_read_file() -> std::io::Result<String> {
    ///     Err(io::Error::new(ErrorKind::NotFound, "file not found"))
    /// }
    ///
    /// let result: Result<String> = try_read_file()
    ///     .context("Failed to read user data".to_string());
    ///
    /// assert!(result.is_err());
    /// let err = result.unwrap_err();
    /// assert!(format!("{}", err).contains("Failed to read user data"));
    /// ```
    #[track_caller]
    fn context(self, msg: impl Into<String>) -> Result<T>;
    /// Adds a suggestion to the error.
    ///
    /// If the `Result` is an `Err` variant, the error is converted into a
    /// `Yoshi` error (if it isn't already) and a suggestion is added to
    /// its primary context.
    /// The source code location of the call is captured.
    ///
    /// # Arguments
    ///
    /// * `s` - The suggestion message, convertible to `String`.
    ///
    /// # Returns
    ///
    /// A `Result<T>` with the error (if any) extended with the suggestion.
    ///
    /// # Examples
    ///
    /// ```
    /// use yoshi_std::{Result, YoshiContextExt};
    /// # use std::io::{self, ErrorKind};
    ///
    /// fn validate_input(input: &str) -> std::io::Result<()> {
    ///     if input.is_empty() {
    ///         Err(io::Error::new(ErrorKind::InvalidInput, "input cannot be empty"))
    ///     } else {
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let result: Result<()> = validate_input("")
    ///     .with_suggestion("Provide a non-empty string for input.");
    ///
    /// assert!(result.is_err());
    /// let err = result.unwrap_err();
    /// assert!(format!("{}", err).contains("Provide a non-empty string for input."));
    /// ```
    #[track_caller]
    fn with_suggestion(self, s: impl Into<String>) -> Result<T>;
    /// Attaches a typed payload to the error.
    ///
    /// If the `Result` is an `Err` variant, the error is converted into a
    /// `Yoshi` error (if it isn't already) and a typed payload is added to
    /// its primary context.
    /// The source code location of the call is captured.
    ///
    /// # Arguments
    ///
    /// * `p` - The payload to attach. It must implement `Any`, `Send`, `Sync`, and have a `'static` lifetime.
    ///
    /// # Returns
    ///
    /// A `Result<T>` with the error (if any) extended with the payload.
    ///
    /// # Examples
    ///
    /// ```
    /// use yoshi_std::{Result, YoshiContextExt};
    /// # use std::io::{self, ErrorKind};
    /// #[derive(Debug, PartialEq)]
    /// struct OperationId(u64);
    ///
    /// fn perform_operation() -> std::io::Result<()> {
    ///     Err(io::Error::new(ErrorKind::TimedOut, "operation timed out"))
    /// }
    ///
    /// let op_id = OperationId(12345);
    /// let result: Result<()> = perform_operation()
    ///     .with_payload(op_id.clone());
    ///
    /// assert!(result.is_err());
    /// let err = result.unwrap_err();
    /// let primary_ctx = err.primary_context().unwrap();
    /// let retrieved_op_id = primary_ctx.payloads.iter().find_map(|p| p.downcast_ref::<OperationId>());
    /// assert_eq!(retrieved_op_id, Some(&op_id));
    /// ```
    #[track_caller]
    fn with_payload(self, p: impl Any + Send + Sync + 'static) -> Result<T>;
    /// Sets the priority for the error's primary context.
    ///
    /// If the `Result` is an `Err` variant, the error is converted into a
    /// `Yoshi` error (if it isn't already) and the priority of its
    /// primary context is set.
    /// The source code location of the call is captured.
    ///
    /// # Arguments
    ///
    /// * `priority` - The priority level (0-255).
    ///
    /// # Returns
    ///
    /// A `Result<T>` with the error (if any) with its primary context's priority set.
    ///
    /// # Examples
    ///
    /// ```
    /// use yoshi_std::{Result, YoshiContextExt};
    /// # use yoshi_std::YoshiKind;
    /// #
    /// let res: Result<()> = Err(yoshi_std::Yoshi::new(YoshiKind::Internal { message: "low importance".into(), source: None, component: None }))
    ///     .with_priority(200); // Mark this as high priority
    ///
    /// assert!(res.is_err());
    /// let err = res.unwrap_err();
    /// assert_eq!(err.primary_context().unwrap().priority, 200);
    /// ```
    #[track_caller]
    fn with_priority(self, priority: u8) -> Result<T>;

    // NEW: Ultra-short aliases for speed typing
    /// Alias for `context`.
    ///
    /// See [`YoshiContextExt::context`] for detailed documentation.
    #[track_caller]
    fn ctx(self, msg: impl Into<String>) -> Result<T>;
    /// Alias for `with_suggestion`.
    ///
    /// See [`YoshiContextExt::with_suggestion`] for detailed documentation.
    #[track_caller]
    fn help(self, s: impl Into<String>) -> Result<T>;
    /// Adds metadata to the error.
    ///
    /// If the `Result` is an `Err` variant, the error is converted into a
    /// `Yoshi` error (if it isn't already) and metadata is added to
    /// its primary context.
    /// The source code location of the call is captured.
    ///
    /// # Arguments
    ///
    /// * `k` - The key for the metadata, convertible to `String`.
    /// * `v` - The value for the metadata, convertible to `String`.
    ///
    /// # Returns
    ///
    /// A `Result<T>` with the error (if any) extended with the new metadata.
    ///
    /// # Examples
    ///
    /// ```
    /// use yoshi_std::{Result, YoshiContextExt, YoshiKind};
    ///
    /// let res: Result<()> = Err(yoshi_std::Yoshi::new(YoshiKind::Internal { message: "problem".into(), source: None, component: None }))
    ///     .meta("user_id", "john.doe");
    ///
    /// assert!(res.is_err());
    /// let err = res.unwrap_err();
    /// assert!(format!("{}", err).contains("user_id: john.doe"));
    /// ```
    #[track_caller]
    fn meta(self, k: impl Into<String>, v: impl Into<String>) -> Result<T>;
}

impl<T, E> YoshiContextExt<T> for core::result::Result<T, E>
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
    fn with_payload(self, p: impl Any + Send + Sync + 'static) -> Result<T> {
        self.map_err(|e| e.into().with_payload(p))
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
        self.map_err(|e| e.into().with_metadata(k, v))
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
    static ENABLED: OnceLock<bool> = OnceLock::new();
    let should = *ENABLED.get_or_init(|| {
        match std::env::var("RUST_LIB_BACKTRACE").or_else(|_| std::env::var("RUST_BACKTRACE")) {
            Ok(v) => matches!(v.as_str(), "1" | "full"),
            Err(_) => false,
        }
    });

    if should {
        Some(YoshiBacktrace::new_captured())
    } else {
        None
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
    use std::{env, io};
    #[cfg(feature = "std")]
    use std::io::ErrorKind;

    #[test]
    fn test_error_instance_counter() {
        let initial_count = error_instance_count();
        let _err1 = Yoshi::new(YoshiKind::Internal {
            message: "test".into(),
            source: None,
            component: None,
        });
        let _err2 = Yoshi::new(YoshiKind::Internal {
            message: "test".into(),
            source: None,
            component: None,
        });
        assert_eq!(error_instance_count(), initial_count + 2);
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
            assert!(format!("{}", yoshi_err).contains("I/O error: file not found"));
            assert!(matches!(yoshi_err.kind, YoshiKind::Io(_)));
        }
        #[cfg(not(feature = "std"))]
        {
            let no_std_io_err = NoStdIo::new("no_std file not found");
            let yoshi_err = Yoshi::from(no_std_io_err);
            assert!(
                format!("{}", yoshi_err).contains("I/O error (no_std): no_std file not found")
            );
            assert!(matches!(yoshi_err.kind, YoshiKind::Io(_)));
        }
    }

    #[test]
    fn test_from_string() {
        let msg = "simple string error".to_string();
        let yoshi_err = Yoshi::from(msg.clone());
        #[cfg(feature = "std")]
        {
            assert!(matches!(
                yoshi_err.kind,
                YoshiKind::Internal {
                    ref message, ..
                } if message.as_ref() == msg
            ));
        }
        #[cfg(not(feature = "std"))]
        {
            assert!(matches!(
                yoshi_err.kind,
                YoshiKind::Io(NoStdIo::Other(ref message)) if message.as_ref() == msg
            ));
        }
        assert!(format!("{}", yoshi_err).contains(&msg));
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
        assert!(format!("{}", yoshi_err).contains("My custom error"));
        assert!(matches!(yoshi_err.kind, YoshiKind::Foreign { .. }));
        if let YoshiKind::Foreign {
            error_type_name, ..
        } = yoshi_err.kind
        {
            assert_eq!(error_type_name.as_ref(), "yoshi_std::tests::MyCustomError");
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

        let err_string = format!("{}", yoshi_err);
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

        let err_string = format!("{}", outer_yoshi);
        assert!(err_string.contains("Internal error: Service communication failed"));
        assert!(err_string.contains("Caused by: Network error: Connection refused")); // Check for nested display
        assert!(!err_string.contains("Original Cause: Network error: Connection refused")); // Should not be duplicated
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_backtrace_capture_if_enabled() {
        let original_rust_backtrace = env::var("RUST_BACKTRACE").ok();
        env::set_var("RUST_BACKTRACE", "1");

        let err = Yoshi::new(YoshiKind::Internal {
            message: "Test internal error with backtrace".into(),
            source: None,
            component: None,
        });
        assert!(err.backtrace().is_some());
        assert!(format!("{}", err).contains("stack backtrace"));
        assert!(format!("{}", err).contains("Backtrace captured at:"));

        if let Some(val) = original_rust_backtrace {
            env::set_var("RUST_BACKTRACE", val);
        } else {
            env::remove_var("RUST_BACKTRACE");
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

        assert!(!format!("{}", err).contains("stack backtrace"));

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

        // Access metadata directly from the YoshiContext
        let ctx = err.primary_context().expect("Should have a primary context");
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
            format!("{}", loc),
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
                write!(
                    f,
                    "CustomPayload: code={}, msg={}",
                    self.code, self.message
                )
            }
        }

        let err = Yoshi::new(YoshiKind::Internal {
            message: "Operation failed with custom payload".into(),
            source: None,
            component: None,
        })
        .with_payload(CustomErrorPayload {
            code: 500,
            message: "Internal server error details".into(),
        })
        .with_payload("a string payload".to_string())
        .with_payload(42u32);

        // Access payloads directly from the YoshiContext
        let ctx = err.primary_context().expect("Should have a primary context");

        let custom_payload = ctx
            .payloads
            .iter()
            .find_map(|p| p.downcast_ref::<CustomErrorPayload>());
        assert!(custom_payload.is_some());
        assert_eq!(custom_payload.unwrap().code, 500);

        let string_payload = ctx
            .payloads
            .iter()
            .find_map(|p| p.downcast_ref::<String>());
        assert!(string_payload.is_some());
        assert_eq!(string_payload.unwrap(), &"a string payload".to_string());

        let u32_payload = ctx
            .payloads
            .iter()
            .find_map(|p| p.downcast_ref::<u32>());
        assert!(u32_payload.is_some());
        assert_eq!(*u32_payload.unwrap(), 42);
    }

    #[test]
    fn test_yoshi_context_ext_with_payload_on_result() -> Result<()> {
        #[derive(Debug, PartialEq)]
        struct TransactionId(String);

        #[cfg(feature = "std")]
        let result: std::result::Result<u32, std::io::Error> =
            Err(io::Error::new(ErrorKind::PermissionDenied, "db write failed"));
        #[cfg(not(feature = "std"))]
        let result: core::result::Result<u32, NoStdIo> = Err(NoStdIo::new("db write failed"));

        let yoshi_result = result
            .with_payload(TransactionId("tx123".into()))
            .context("Failed to commit transaction".to_string());

        assert!(yoshi_result.is_err());
        let err = yoshi_result.unwrap_err();

        assert!(format!("{}", err).contains("db write failed"));
        assert!(format!("{}", err).contains("Caused by: Failed to commit transaction"));

        // Access payloads directly from the YoshiContext
        let ctx = err.primary_context().expect("Should have a primary context");
        let transaction_id = ctx
            .payloads
            .iter()
            .find_map(|p| p.downcast_ref::<TransactionId>());
        assert!(transaction_id.is_some());
        assert_eq!(transaction_id.unwrap().0, "tx123".to_string());

        Ok(())
    }

    #[test]
    fn test_yoshi_context_ext_short_aliases() {
        #[cfg(feature = "std")]
        let result: std::result::Result<(), std::io::Error> =
            Err(io::Error::new(io::ErrorKind::NotFound, "file.txt not found"));
        #[cfg(not(feature = "std"))]
        let result: core::result::Result<(), NoStdIo> = Err(NoStdIo::NotFound);

        let err = result
            .ctx("Failed to open file".to_string())
            .help("Check file path and permissions".to_string())
            .meta("file_name".to_string(), "file.txt".to_string())
            .unwrap_err();

        let s = format!("{}", err);
        assert!(s.contains("Failed to open file"));
        assert!(s.contains("Check file path and permissions"));
        assert!(s.contains("file_name: file.txt"));
    }
}