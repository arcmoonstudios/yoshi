/* yoshi-std/src/api/mod.rs */
#![warn(missing_docs)]
//! **Core API Module** - Primary API surface and re-exports for yoshi-std
//!
//! This module provides the main `YoshiStd` API and all core re-exports
//! from yoshi-core, serving as the primary entry point for users.
//!
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + Core YoshiStd API with comprehensive yoshi-core integration
//!  - All fundamental types with zero-cost abstraction guarantees
//!  - Type aliases for ergonomic error handling with memory-efficient boxing
//!  - Primary API surface for external consumption
//! + Backwards compatibility layers and deprecated module support
//!  - snake_case module compatibility for migration support
//!  - Version-aware API surface management
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

/// Async utilities for error handling (requires 'async' feature)
#[cfg(feature = "async")]
pub mod async_utils;

/// Tracing integration utilities (requires 'tracing' feature)
#[cfg(feature = "tracing")]
pub mod tracing_integration;

//============================================================================
// YOSHI STD API - COMPLETE STD ERROR HANDLING ECOSYSTEM
//============================================================================

/// **`YoshiStd` API - Complete Standard Library Error Handling Ecosystem**
///
/// This module provides a single, comprehensive API for all standard library error handling needs.
/// Import this module to get access to all core types, std enhancements, analytics, and functionality
/// without needing to know about internal dependencies.
///
/// # Examples
///
/// ```rust
/// use yoshi_std::YoshiStd;
///
/// // All functionality available through YoshiStd
/// let error = YoshiStd::Yoshi::new(YoshiStd::YoshiKind::Network {
///     message: "Connection failed".into(),
///     source: None,
///     error_code: Some(500),
/// });
///
/// // Advanced analytics
/// let analytics = YoshiStd::AutonomousErrorAnalytics::record_error_occurrence(
///     "NetworkError",
///     "ConnectionFailed",
///     YoshiStd::SystemTime::now()
/// );
/// ```
#[allow(non_snake_case)]
pub mod YoshiStd {

    // Re-export all core functionality from yoshi-core
    pub use yoshi_core::*;

    // Re-export all std-specific functionality
    pub use crate::{
        analytics::{error_instance_count, increment_error_counter, YoshiACE},
        // AnyError from conversions module
        conversions::AnyError,
        // String interning functions
        conversions::{intern_string_std, io_error_to_yoshi},
        conversions::{
            BulkYoshiConvert,
            Error,
            Hatch,
            HatchIo,
            // Conversion types
            HatchWrapper,
            // Error enhancement and auto-correction engine
            Hatchable,
            HatchedYoshi,
            Hatchling,
            IoHatchable,
            Oops,
            Payload,
            // Type aliases
            Result,
            ResultWrapper,
            UltraYoshiConvert,
            YoshiConversionStrategy,
            YoshiConvertError,
            YoshiConvertOptimizer,
            YoshiEgg,
            YoshiLayoutCompatible,
        },
        // Extension traits from conversions module
        conversions::{LayWithContext, ResultExt},
        std_integration::{
            // Extension traits
            IoErrorExt,
        },
        // Core std types
        std_integration::{StdStringInternPool, StdYoshiBacktrace, YoshiBacktrace},
        // NOTE: Utils functionality moved to main yoshi crate
    };

    // NOTE: Optimization and performance functionality has been moved to the main `yoshi` crate.
    // Use `yoshi::*` to access YoshiAF autonomous fixing capabilities.

    // Re-export essential std dependencies used in our API
    pub use std::{
        backtrace::Backtrace,
        borrow::{Borrow, BorrowMut, Cow, ToOwned},
        // Core std types
        collections::{BTreeMap, BTreeSet, HashMap, HashSet},
        // Conversion traits
        convert::{AsMut, AsRef, From, Into, TryFrom, TryInto},

        env,
        // Error handling
        error::Error as StdError,
        ffi::{CStr, CString, OsStr, OsString},

        // Formatting and display
        fmt::{self, Debug, Display, Formatter, Write as FmtWrite},

        fs::{self, File, OpenOptions},
        io::{self, BufRead, BufReader, BufWriter, Read, Write},
        // Iteration
        iter::{FromIterator, IntoIterator, Iterator},

        // Markers
        marker::{PhantomData, Send, Sync},

        // Memory and ownership
        mem,
        ops::{Deref, DerefMut},

        option::Option::{self, None, Some},

        panic,

        path::{Path, PathBuf},
        // Primitives
        primitive::*,
        process,

        ptr,
        // Results and options (note: Result is already re-exported from super)
        result::Result as StdResult,
        str,
        // Strings
        string::{String, ToString},
        sync::{mpsc, Arc, Mutex, OnceLock, RwLock},
        thread::{self, JoinHandle, ThreadId},
        time::{Duration, Instant, SystemTime},
    };

    // Re-export async support when available
    #[cfg(feature = "tokio")]
    pub use tokio::{
        self, fs as tokio_fs,
        io::{AsyncBufRead, AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
        net::{TcpListener, TcpStream, UdpSocket},
        runtime::{Handle, Runtime},
        sync::{Mutex as TokioMutex, Notify, RwLock as TokioRwLock, Semaphore},
        task::{self, JoinHandle as TokioJoinHandle},
        time::{interval, sleep, timeout, Interval},
    };

    // Note: async-std support is planned but not yet implemented
    // The async-std feature is a placeholder for future async-std integration

    // Serde support when available
    #[cfg(feature = "serde")]
    pub use serde::{
        self,
        de::{DeserializeOwned, MapAccess, SeqAccess, Visitor},
        ser::{SerializeMap, SerializeSeq, SerializeStruct},
        Deserialize, Deserializer, Serialize, Serializer,
    };

    // JSON support when available
    #[cfg(feature = "serde_json")]
    pub use serde_json::{self, from_slice, from_str, to_string, to_vec, Map, Number, Value};

    // Tracing support when available
    #[cfg(feature = "tracing")]
    pub use tracing::{
        self, debug, error,
        field::{Field, Visit},
        info, instrument,
        subscriber::{Interest, Subscriber},
        trace, warn, Event, Instrument, Level, Span,
    };
}

/// **DEPRECATED: Use `YoshiStd` instead**
///
/// This module is deprecated in favor of the `PascalCase` `YoshiStd` module
/// which follows the same architectural pattern as `YoshiCore`.
#[deprecated(
    since = "0.1.0",
    note = "Use `YoshiStd` instead for consistency with `YoshiCore`"
)]
pub mod yoshi_std {
    pub use super::YoshiStd::*;
}

//============================================================================
// CORE RE-EXPORTS FROM YOSHI-CORE
//============================================================================

// Re-export all core functionality from yoshi-core
pub use yoshi_core::*;
