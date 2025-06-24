/* yoshi/src/auto_fix/no_std_compat.rs */
//! #![yoshi(auto-fix)]
//! # No-std Compatibility Types for YoshiAF
//!
//! This module provides enhanced types for `no_std` environments that maintain
//! compatibility with the YoshiAF autonomous fixing system.
//! ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
//! **Copyright:** (c) 2025 ArcMoon Studios
//! **Author:** Lord Xyn
//! **License:** MIT

use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// Enhanced SystemTime for `no_std` environments with monotonic counter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SystemTime {
    /// Monotonic timestamp counter for ordering events
    timestamp: u64,
}

impl SystemTime {
    /// Returns a `SystemTime` with monotonic ordering guarantees.
    ///
    /// While not wall-clock time, this provides ordering semantics
    /// useful for debugging and event correlation in no_std environments.
    pub fn now() -> Self {
        /// Static variable: COUNTER.
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

/// Enhanced ThreadId for `no_std` environments with unique identification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ThreadId {
    /// Unique identifier for tracking execution contexts
    id: u32,
}

impl ThreadId {
    /// Returns a `ThreadId` with unique identification.
    ///
    /// In no_std environments, this provides unique identifiers
    /// useful for correlating errors across different execution contexts.
    pub fn current() -> Self {
        /// Static variable: THREAD_COUNTER.
        static THREAD_COUNTER: AtomicU32 = AtomicU32::new(1);

        Self {
            id: THREAD_COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }

    /// Returns the raw thread ID for debugging.
    #[inline]
    pub const fn as_u32(&self) -> u32 {
        self.id
    }

    /// **from_u32**
    ///
    /// This function provides from u32 functionality within the Yoshi error handling framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    pub const fn from_u32(id: u32) -> Self {
        Self { id }
    }
}

impl core::fmt::Display for ThreadId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ThreadId({})", self.id)
    }
}

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

impl core::fmt::Display for NoStdIoKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}
