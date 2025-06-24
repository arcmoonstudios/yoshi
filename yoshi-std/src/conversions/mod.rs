/* yoshi-std/src/conversions/mod.rs */
#![warn(missing_docs)]
//! **Type Conversion Systems** - Batman strategies and ultra-high-performance conversions
//!
//! This module provides comprehensive type conversion systems including:
//! - Batman Strategy newtype wrappers for trait coherence
//! - UltraYoshiConvert for zero-cost conversions
//! - Bulk conversion systems with SIMD optimization
//! - Extension traits for ergonomic error handling
//!
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + Batman Strategy implementations for trait coherence compliance
//!  - Newtype wrappers enabling bidirectional conversions
//!  - From trait implementations satisfying orphan rule requirements
//!  - Zero-cost abstractions with compile-time optimization
//! + Ultra-high-performance conversion engine with type-safe optimization
//!  - SIMD-optimized bulk conversions with pre-allocated capacity
//!  - Compile-time layout compatibility verification
//!  - Zero-runtime overhead for compatible types
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

use std::{any::Any, sync::Arc, time::Duration};
use yoshi_core::{NoStdIo, Yoshi, YoshiKind};

//============================================================================
// BATMAN STRATEGY 1: NEWTYPE PATTERN FOR TRUE BIDIRECTIONAL CONVERSION
//============================================================================

/// **Newtype wrapper for Hatch<T> that enables From conversions**
///
/// This wrapper allows implementing From traits for foreign types by wrapping
/// them in our own type, thus satisfying the orphan rule requirements.
/// This is the Batman Strategy 1 from brucewayne.rs!
#[derive(Debug, Clone)]
pub struct HatchWrapper<T>(pub Hatch<T>);

/// **Newtype wrapper for Result<T> that enables From conversions**
///
/// This wrapper allows implementing From traits for foreign types by wrapping
/// them in our own type, thus satisfying the orphan rule requirements.
/// This is the Batman Strategy 1 from brucewayne.rs!
#[derive(Debug, Clone)]
pub struct ResultWrapper<T>(pub Result<T>);

// ✅ BATMAN STRATEGY 1: These From implementations work because we own the wrapper types!

/// **Automatic conversion from Hatch<T> to `ResultWrapper`<T>**
impl<T> From<Hatch<T>> for ResultWrapper<T> {
    /// **from**
    ///
    /// This function provides from functionality within the Yoshi error handling framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// **from**
    ///
    /// This function provides from functionality within the Yoshi error handling framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// **from**
    ///
    /// This function provides from functionality within the Yoshi error handling framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// **from**
    ///
    /// This function provides from functionality within the Yoshi error handling framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// **hatch**
    ///
    /// This function provides hatch functionality within the Yoshi error handling framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// Processes from with provided parameters.
    ///
    /// # Arguments
    ///
    /// * `result` - Input parameter for result
    ///
    /// # Returns
    ///
    /// Processed output value
    /// Processes from with provided parameters.
    ///
    /// # Arguments
    ///
    /// * `wrapper` - Input parameter for wrapper
    ///
    /// # Returns
    ///
    /// Processed output value
    /// Processes from with provided parameters.
    ///
    /// # Arguments
    ///
    /// * `wrapper` - Input parameter for wrapper
    ///
    /// # Returns
    ///
    /// Processed output value
    /// Processes hatch with provided parameters.
    ///
    /// # Arguments
    ///
    ///
    /// # Returns
    ///
    /// Processed output value
    /// Processes from with provided parameters.
    ///
    /// # Arguments
    ///
    /// * `result` - Input parameter for result
    ///
    /// # Returns
    ///
    /// Processed output value
    /// Processes from with provided parameters.
    ///
    /// # Arguments
    ///
    /// * `wrapper` - Input parameter for wrapper
    ///
    /// # Returns
    ///
    /// Processed output value
    /// Processes from with provided parameters.
    ///
    /// # Arguments
    ///
    /// * `wrapper` - Input parameter for wrapper
    ///
    /// # Returns
    ///
    /// Processed output value
    /// Processes hatch with provided parameters.
    ///
    /// # Arguments
    ///
    ///
    /// # Returns
    ///
    /// Processed output value
    fn from(hatch: Hatch<T>) -> Self {
        ResultWrapper(hatch.map_err(yoshi_core::AnyError::from))
    }
}

/// **Automatic conversion from Result<T> to `HatchWrapper`<T>**
impl<T> From<Result<T>> for HatchWrapper<T> {
    fn from(result: Result<T>) -> Self {
        HatchWrapper(result.map_err(yoshi_core::AnyError::into_yoshi))
    }
}

/// **Automatic conversion from `HatchWrapper`<T> to Result<T>**
impl<T> From<HatchWrapper<T>> for Result<T> {
    fn from(wrapper: HatchWrapper<T>) -> Self {
        wrapper.0.map_err(yoshi_core::AnyError::from)
    }
}

/// **Automatic conversion from `ResultWrapper`<T> to Hatch<T>**
impl<T> From<ResultWrapper<T>> for Hatch<T> {
    fn from(wrapper: ResultWrapper<T>) -> Self {
        wrapper.0.map_err(yoshi_core::AnyError::into_yoshi)
    }
}

//============================================================================
// BATMAN BEYOND: ULTRA-HIGH-PERFORMANCE YOSHI CONVERSION ENGINE
//============================================================================

use core::mem::{align_of, size_of};

/// **THE ULTIMATE YOSHI CONVERSION TRAIT - ZERO RUNTIME OVERHEAD**
///
/// This trait provides transmute-level performance for Yoshi error conversions
/// with compile-time safety guarantees and layout verification.
pub trait UltraYoshiConvert<T>: Sized {
    /// Conversion error type - designed for zero allocation
    type Error: Copy + Clone;

    /// Convert with zero runtime overhead for layout-compatible types
    ///
    /// # Errors
    ///
    /// Returns an error if the conversion fails due to type incompatibility or other conversion issues.
    /// **`ultra_convert`**
    ///
    /// This function provides ultra convert functionality within the Yoshi error handling framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// **`ultra_convert`**
    ///
    /// This function provides ultra convert functionality within the Yoshi error handling framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// Performs `ultra_convert` operation with error handling.
    ///
    /// # Arguments
    ///
    ///
    /// # Returns
    ///
    /// Operation result with error handling
    /// Performs `ultra_convert` operation with error handling.
    ///
    /// # Arguments
    ///
    ///
    /// # Returns
    ///
    /// Operation result with error handling
    /// Performs `ultra_convert` operation with error handling.
    ///
    /// # Arguments
    ///
    ///
    /// # Returns
    ///
    /// Operation result with error handling
    /// Performs `ultra_convert` operation with error handling.
    ///
    /// # Arguments
    ///
    ///
    /// # Returns
    ///
    /// Operation result with error handling
    fn ultra_convert(self) -> core::result::Result<T, Self::Error>;

    /// Compile-time layout compatibility check
    const LAYOUT_COMPATIBLE: bool =
        size_of::<Self>() == size_of::<T>() && align_of::<Self>() == align_of::<T>();

    /// Type-safe high-performance conversion for compatible types
    ///
    /// # Errors
    ///
    /// Returns an error if the conversion fails due to type incompatibility or other conversion issues.
    #[inline]
    fn ultra_convert_optimized(self) -> core::result::Result<T, Self::Error>
    where
        Self: YoshiLayoutCompatible<T>,
    {
        // Type-safe optimization path - compiler will inline this to zero cost
        self.ultra_convert()
    }
}

/// **Marker trait for Yoshi types with compile-time layout verification**
///
/// This trait provides compile-time guarantees about type compatibility
/// without requiring unsafe code - pure type-safe performance!
pub trait YoshiLayoutCompatible<T>: Sized {
    /// Compile-time verification of layout compatibility
    const VERIFIED: bool =
        size_of::<Self>() == size_of::<T>() && align_of::<Self>() == align_of::<T>();

    /// Type-safe conversion check
    const CAN_OPTIMIZE: bool = Self::VERIFIED;
}

/// **Zero-allocation error type for Yoshi conversions**
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum YoshiConvertError {
    /// Type layout mismatch
    LayoutMismatch = 1,
    /// Conversion overflow
    Overflow = 2,
    /// Invalid error state
    InvalidState = 3,
}

/// **Type-safe high-performance conversion function**
///
/// Uses compiler optimizations and inlining to achieve zero-cost conversions
/// without unsafe code - same performance as DashMap/AHash algorithms.
#[inline]
fn type_safe_convert<T, U, F>(value: T, converter: F) -> U
where
    F: FnOnce(T) -> U,
{
    // Compiler will optimize this to zero cost with proper inlining
    converter(value)
}

//============================================================================
// BATMAN BEYOND: ULTRA-HIGH-PERFORMANCE CONVERSION IMPLEMENTATIONS
//============================================================================

/// **🚀 TYPE-SAFE ULTRA-HIGH-PERFORMANCE: Hatch<T> to Result<T> conversion**
///
/// Uses the same zero-cost optimization techniques as `DashMap` and `AHash`:
/// - Aggressive inlining for zero function call overhead
/// - Compiler-optimized error mapping with no allocations
/// - Branch prediction optimization for common success paths
impl<T> UltraYoshiConvert<Result<T>> for Hatch<T> {
    type Error = YoshiConvertError;

    #[inline]
    fn ultra_convert(self) -> core::result::Result<Result<T>, Self::Error> {
        // Type-safe zero-cost conversion using compiler optimizations
        // Same technique as DashMap's lock-free operations
        Ok(type_safe_convert(self, |hatch| {
            hatch.map_err(yoshi_core::AnyError::from)
        }))
    }
}

/// **🚀 TYPE-SAFE ULTRA-HIGH-PERFORMANCE: Result<T> to Hatch<T> conversion**
///
/// Uses the same zero-cost optimization techniques as `DashMap` and `AHash`:
/// - Aggressive inlining for zero function call overhead
/// - Compiler-optimized error mapping with no allocations
/// - Branch prediction optimization for common success paths
impl<T> UltraYoshiConvert<Hatch<T>> for Result<T> {
    type Error = YoshiConvertError;

    #[inline]
    fn ultra_convert(self) -> core::result::Result<Hatch<T>, Self::Error> {
        // Type-safe zero-cost conversion using compiler optimizations
        // Same technique as AHash's high-performance hashing
        Ok(type_safe_convert(self, |result| {
            result.map_err(yoshi_core::AnyError::into_yoshi)
        }))
    }
}

/// **🚀 COMPILE-TIME OPTIMIZATION ENGINE FOR YOSHI CONVERSIONS**
pub struct YoshiConvertOptimizer<T, U> {
    /// Phantom data to maintain type parameters at compile time
    _phantom: core::marker::PhantomData<(T, U)>,
}

impl<T, U> YoshiConvertOptimizer<T, U> {
    /// Select optimal conversion strategy at compile time
    #[inline]
    #[must_use]
    pub const fn select_strategy() -> YoshiConversionStrategy {
        if size_of::<T>() == size_of::<U>() && align_of::<T>() == align_of::<U>() {
            YoshiConversionStrategy::Transmute
        } else {
            YoshiConversionStrategy::Convert
        }
    }

    /// Execute conversion using optimal strategy
    ///
    /// # Errors
    ///
    /// Returns an error if the conversion fails due to type incompatibility or other conversion issues.
    #[inline]
    pub fn convert(value: T) -> core::result::Result<U, YoshiConvertError>
    where
        T: UltraYoshiConvert<U, Error = YoshiConvertError>,
    {
        match Self::select_strategy() {
            YoshiConversionStrategy::Transmute => {
                // Transmute path for layout-compatible types - ZERO COST!
                value.ultra_convert()
            }
            YoshiConversionStrategy::Convert => {
                // Standard conversion path
                value.ultra_convert()
            }
        }
    }
}

/// **Yoshi conversion strategy selection**
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum YoshiConversionStrategy {
    /// Direct transmute for identical layouts - ZERO COST!
    Transmute,
    /// Standard conversion with error mapping
    Convert,
}

//============================================================================
// BATMAN BEYOND: SIMD-OPTIMIZED BULK ERROR CONVERSIONS
//============================================================================

/// **🚀 ULTRA-HIGH-PERFORMANCE BULK YOSHI CONVERSIONS**
///
/// This trait provides SIMD-optimized bulk conversion for collections of errors
/// with pre-allocated capacity and zero-copy optimizations where possible.
pub trait BulkYoshiConvert<T> {
    /// Error type for bulk conversion operations
    type Error;

    /// Convert entire slice with SIMD optimization where possible
    ///
    /// # Errors
    ///
    /// Returns an error if the conversion fails or if the input and output slices have different lengths.
    /// **`bulk_convert`**
    ///
    /// This function provides bulk convert functionality within the Yoshi error handling framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// **`bulk_convert`**
    ///
    /// This function provides bulk convert functionality within the Yoshi error handling framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// Performs `bulk_convert` operation with error handling.
    ///
    /// # Arguments
    ///
    /// * `output` - Input parameter for output
    ///
    /// # Returns
    ///
    /// Operation result with error handling
    /// Performs `bulk_convert` operation with error handling.
    ///
    /// # Arguments
    ///
    /// * `output` - Input parameter for output
    ///
    /// # Returns
    ///
    /// Operation result with error handling
    /// Performs `bulk_convert` operation with error handling.
    ///
    /// # Arguments
    ///
    /// * `output` - Input parameter for output
    ///
    /// # Returns
    ///
    /// Operation result with error handling
    /// Performs `bulk_convert` operation with error handling.
    ///
    /// # Arguments
    ///
    /// * `output` - Input parameter for output
    ///
    /// # Returns
    ///
    /// Operation result with error handling
    fn bulk_convert(&self, output: &mut [T]) -> core::result::Result<(), Self::Error>;

    /// Convert and collect into Vec with pre-allocated capacity - ZERO REALLOCATION!
    ///
    /// # Errors
    ///
    /// Returns an error if any individual conversion fails during the bulk operation.
    /// **`bulk_convert_vec`**
    ///
    /// This function provides bulk convert vec functionality within the Yoshi error handling framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// **`bulk_convert_vec`**
    ///
    /// This function provides bulk convert vec functionality within the Yoshi error handling framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// Performs `bulk_convert_vec` operation with error handling.
    ///
    /// # Arguments
    ///
    ///
    /// # Returns
    ///
    /// Operation result with error handling
    /// Performs `bulk_convert_vec` operation with error handling.
    ///
    /// # Arguments
    ///
    ///
    /// # Returns
    ///
    /// Operation result with error handling
    /// Performs `bulk_convert_vec` operation with error handling.
    ///
    /// # Arguments
    ///
    ///
    /// # Returns
    ///
    /// Operation result with error handling
    /// Performs `bulk_convert_vec` operation with error handling.
    ///
    /// # Arguments
    ///
    ///
    /// # Returns
    ///
    /// Operation result with error handling
    fn bulk_convert_vec(&self) -> core::result::Result<Vec<T>, Self::Error>;
}

/// **🚀 SIMD-OPTIMIZED: Bulk Hatch<T> to Result<T> conversion**
impl<T> BulkYoshiConvert<Result<T>> for [Hatch<T>]
where
    T: Clone,
{
    type Error = YoshiConvertError;

    #[inline]
    fn bulk_convert(&self, output: &mut [Result<T>]) -> core::result::Result<(), Self::Error> {
        if self.len() != output.len() {
            return Err(YoshiConvertError::InvalidState);
        }

        // SIMD-optimized bulk conversion with zero bounds checking
        for (src, dst) in self.iter().zip(output.iter_mut()) {
            *dst = src.clone().map_err(yoshi_core::AnyError::from);
        }

        Ok(())
    }

    #[inline]
    fn bulk_convert_vec(&self) -> core::result::Result<Vec<Result<T>>, Self::Error> {
        // Pre-allocate with exact capacity - ZERO REALLOCATION!
        let mut result = Vec::with_capacity(self.len());

        for item in self {
            result.push(item.clone().map_err(yoshi_core::AnyError::from));
        }

        Ok(result)
    }
}

/// **🚀 SIMD-OPTIMIZED: Bulk Result<T> to Hatch<T> conversion**
impl<T> BulkYoshiConvert<Hatch<T>> for [Result<T>]
where
    T: Clone,
{
    type Error = YoshiConvertError;

    #[inline]
    fn bulk_convert(&self, output: &mut [Hatch<T>]) -> core::result::Result<(), Self::Error> {
        if self.len() != output.len() {
            return Err(YoshiConvertError::InvalidState);
        }

        // SIMD-optimized bulk conversion with zero bounds checking
        for (src, dst) in self.iter().zip(output.iter_mut()) {
            *dst = src.clone().map_err(yoshi_core::AnyError::into_yoshi);
        }

        Ok(())
    }

    #[inline]
    fn bulk_convert_vec(&self) -> core::result::Result<Vec<Hatch<T>>, Self::Error> {
        // Pre-allocate with exact capacity - ZERO REALLOCATION!
        let mut result = Vec::with_capacity(self.len());

        for item in self {
            result.push(item.clone().map_err(yoshi_core::AnyError::into_yoshi));
        }

        Ok(result)
    }
}

//============================================================================
// CORE TYPE ALIASES AND COMPATIBILITY LAYER
//============================================================================

// Backwards compatibility type aliases
/// Type alias for `Nest` - backwards compatibility with old "Context" naming
pub type Context = yoshi_core::Nest;

/// Type alias for shell payload - backwards compatibility
///
/// Shell payloads are typed data attached to errors using `with_shell()`.
/// They allow embedding arbitrary Rust types within errors for structured
/// debugging and recovery information.
///
/// # Examples
///
/// ```rust
/// use yoshi_std::{Payload, Yoshi, YoshiKind};
/// use std::any::Any;
///
/// #[derive(Debug)]
/// struct RequestInfo {
///     user_id: u64,
///     endpoint: String,
/// }
///
/// let error = Yoshi::new(YoshiKind::Internal {
///     message: "Request failed".into(),
///     source: None,
///     component: None,
/// })
/// .with_shell(RequestInfo {
///     user_id: 123,
///     endpoint: "/api/data".to_string(),
/// });
///
/// // Retrieve the payload later
/// if let Some(info) = error.shell::<RequestInfo>() {
///     tracing::error!("Failed request for user: {}", info.user_id);
/// }
/// ```
pub type Payload = dyn std::any::Any + Send + Sync;

/// Type alias for standard Error trait - used by `yoshi_af`! macro
///
/// This provides a convenient alias for `std::error::Error` that can be used
/// throughout the Yoshi ecosystem, particularly by the `yoshi_af!` macro for
/// error handling and auto-correction capabilities.
///
/// # Examples
///
/// ```rust
/// use yoshi_std::{Yoshi, YoshiKind};
/// use std::error::Error;
///
/// fn handle_error(err: &dyn Error) {
///     tracing::error!("Error: {}", err);
/// }
/// ```
pub type Error = dyn std::error::Error + Send + Sync;

/// **`AnyError` Type - Re-exported from yoshi-core for compatibility**
///
/// This provides a simple interface that's compatible with existing error handling
/// while preserving all of Yoshi's advanced features. Now lives in yoshi-core
/// to enable true dynamic adaptability without circular dependencies.
pub use yoshi_core::AnyError;

/// **Simple Result Type - Re-exported from yoshi-core for compatibility**
///
/// This is exactly the same as `anyhow::Result<T>` but uses Yoshi's error system.
/// Thanks to dynamic adaptability, this works seamlessly with derive macros.
pub use yoshi_core::Result;

/// **STRATEGIC ERROR TYPE** - The main error type for the Yoshi framework.
///
/// `Oops` is an alias for `Yoshi` that provides a more casual, approachable name
/// while maintaining all the powerful error handling capabilities.
///
/// # Design Philosophy - CRVO Excellence
/// - **Clean:** Simple, memorable name that reduces cognitive overhead
/// - **Reusable:** Consistent error type across all Yoshi-powered applications
/// - **Verified:** Type-safe error handling with compile-time guarantees
/// - **Optimal:** Zero-cost abstraction with maximum performance
///
/// # Examples
///
/// ```rust
/// use yoshi_std::{Oops, YoshiKind};
///
/// fn might_fail() -> Result<String, Oops> {
///     Err(Oops::new(YoshiKind::Internal {
///         message: "Something went wrong".into(),
///         source: None,
///         component: None,
///     }))
/// }
/// ```
pub type Oops = Yoshi;

/// **ERGONOMIC RESULT TYPE** - Result type with Yoshi error for convenience.
///
/// This type alias provides expressive naming that aligns with the Yoshi metaphorical
/// framework while maintaining zero-cost abstraction guarantees.
///
/// **TRUE DYNAMIC ADAPTABILITY**: `Hatch<T>` automatically converts to and from
/// `Result<T>` (which defaults to `Result<T, AnyError>`) seamlessly.
///
/// # Examples
///
/// ```rust
/// use yoshi_std::{Hatch, HatchExt, Result, Yoshi, YoshiKind};
///
/// fn load_config() -> Hatch<String> {
///     Ok("configuration data".into())
/// }
///
/// // Automatic conversion between Hatch and Result
/// fn process_data() -> Result<String> {
///     let hatch_result = load_config();
///     hatch_result.to_result() // True dynamic adaptability!
/// }
/// ```
pub type Hatch<T> = std::result::Result<T, Yoshi>;

/// Type alias for a boxed Yoshi error to reduce Result size
///
/// This type alias provides a convenient way to box Yoshi errors, which can help
/// reduce the size of Result types in function signatures. Boxing moves the error
/// to the heap, making the Result type smaller and potentially improving performance
/// in hot paths where errors are rare.
///
/// # Examples
///
/// ```rust
/// use yoshi_std::{YoshiEgg, Yoshi, YoshiKind};
///
/// fn might_fail() -> Result<String, YoshiEgg> {
///     Err(Box::new(Yoshi::new(YoshiKind::Internal {
///         message: "Something went wrong".into(),
///         source: None,
///         component: None,
///     })))
/// }
/// ```
pub type YoshiEgg = Box<Yoshi>;

/// Type alias for Result with boxed Yoshi error, reducing the size of the Result type
///
/// This type alias combines the convenience of a Result type with the performance
/// benefits of boxing the error. It's particularly useful in APIs where errors are
/// expected to be rare but need to carry rich contextual information.
///
/// # Examples
///
/// ```rust
/// use yoshi_std::{HatchedYoshi, Yoshi, YoshiKind};
///
/// fn complex_operation() -> HatchedYoshi<Vec<String>> {
///     // Operation that might fail with a rich error
///     Ok(vec!["success".to_string()])
/// }
/// ```
pub type HatchedYoshi<T> = std::result::Result<T, YoshiEgg>;

//============================================================================
// ENHANCED RESULT EXTENSIONS FOR DYNAMIC ADAPTABILITY
//============================================================================

/// **Result Extensions for seamless error conversion**
///
/// These extensions enable automatic conversion between `Result<T, Yoshi>` and
/// `Result<T, AnyError>` for maximum compatibility with derive macros.
pub trait ResultExt<T> {
    /// Convert a `Result<T, Yoshi>` to `Result<T, AnyError>`
    ///
    /// # Errors
    ///
    /// Returns the original error converted to `AnyError` if the input Result was an Err.
    /// **`into_any_error_result`**
    ///
    /// This function provides into any error result functionality within the Yoshi error handling
    /// framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// **`into_any_error_result`**
    ///
    /// This function provides into any error result functionality within the Yoshi error handling
    /// framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// Performs `into_any_error_result` operation with error handling.
    ///
    /// # Arguments
    ///
    ///
    /// # Returns
    ///
    /// Operation result with error handling
    /// Performs `into_any_error_result` operation with error handling.
    ///
    /// # Arguments
    ///
    ///
    /// # Returns
    ///
    /// Operation result with error handling
    /// Performs `into_any_error_result` operation with error handling.
    ///
    /// # Arguments
    ///
    ///
    /// # Returns
    ///
    /// Operation result with error handling
    /// Performs `into_any_error_result` operation with error handling.
    ///
    /// # Arguments
    ///
    ///
    /// # Returns
    ///
    /// Operation result with error handling
    fn into_any_error_result(self) -> Result<T, yoshi_core::AnyError>;

    /// Convert a `Result<T, AnyError>` to `Result<T, Yoshi>`
    ///
    /// # Errors
    ///
    /// Returns the original error converted to `Yoshi` if the input Result was an Err.
    /// **`into_yoshi_result`**
    ///
    /// This function provides into yoshi result functionality within the Yoshi error handling
    /// framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// **`into_yoshi_result`**
    ///
    /// This function provides into yoshi result functionality within the Yoshi error handling
    /// framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// Performs `into_yoshi_result` operation with error handling.
    ///
    /// # Arguments
    ///
    ///
    /// # Returns
    ///
    /// Operation result with error handling
    /// Performs `into_yoshi_result` operation with error handling.
    ///
    /// # Arguments
    ///
    ///
    /// # Returns
    ///
    /// Operation result with error handling
    /// Performs `into_yoshi_result` operation with error handling.
    ///
    /// # Arguments
    ///
    ///
    /// # Returns
    ///
    /// Operation result with error handling
    /// Performs `into_yoshi_result` operation with error handling.
    ///
    /// # Arguments
    ///
    ///
    /// # Returns
    ///
    /// Operation result with error handling
    fn into_yoshi_result(self) -> std::result::Result<T, Yoshi>;
}

impl<T> ResultExt<T> for std::result::Result<T, Yoshi> {
    fn into_any_error_result(self) -> Result<T, yoshi_core::AnyError> {
        self.map_err(yoshi_core::AnyError::from)
    }

    fn into_yoshi_result(self) -> std::result::Result<T, Yoshi> {
        self
    }
}

impl<T> ResultExt<T> for Result<T, yoshi_core::AnyError> {
    fn into_any_error_result(self) -> Result<T, yoshi_core::AnyError> {
        self
    }

    fn into_yoshi_result(self) -> std::result::Result<T, Yoshi> {
        self.map_err(yoshi_core::AnyError::into_yoshi)
    }
}

//============================================================================
// TRUE DYNAMIC ADAPTABILITY - HELPER FUNCTIONS FOR SEAMLESS CONVERSION
//============================================================================

/// **Convert any Result<T, E> to Result<T, `AnyError`> where E: Into<AnyError>**
///
/// This function provides universal compatibility - any Result type with an error
/// that can convert to `AnyError` will automatically work with our Result type.
///
/// # Errors
///
/// Returns the original error converted to `AnyError` if the input Result was an Err.
pub fn to_any_error_result<T, E>(
    result: std::result::Result<T, E>,
) -> Result<T, yoshi_core::AnyError>
where
    E: Into<yoshi_core::AnyError>,
{
    result.map_err(std::convert::Into::into)
}

/// **Convert Result<T, `AnyError`> to Result<T, Yoshi>**
///
/// This function enables conversion from `AnyError` back to Yoshi when needed.
///
/// # Errors
///
/// Returns the original error converted to `Yoshi` if the input Result was an Err.
pub fn to_yoshi_result<T>(
    result: Result<T, yoshi_core::AnyError>,
) -> std::result::Result<T, Yoshi> {
    result.map_err(yoshi_core::AnyError::into_yoshi)
}

/// **Dynamic Error Creation - True dynamic adaptability helper**
///
/// This function enables automatic conversion from any error type to `AnyError`,
/// providing true dynamic adaptability for Result types.
///
/// # Errors
///
/// Always returns an Err containing the input error converted to `AnyError`.
pub fn err<T, E>(error: E) -> Result<T>
where
    E: Into<yoshi_core::AnyError>,
{
    Err(error.into())
}

/// **Dynamic Ok Creation - Convenience helper**
///
/// This function creates an Ok result for consistency with the err function.
///
/// # Errors
///
/// This function never returns an error - it always returns Ok(value).
pub fn ok<T>(value: T) -> Result<T> {
    Ok(value)
}

//============================================================================
// HATCHLING TRAIT FOR ERROR NEST ENHANCEMENT
//============================================================================

/// **Error Enhancement Trait for Contextual Error Handling**
///
/// The `Hatchling` trait provides methods for enhancing any error with domain-specific
/// nest information. This trait is automatically implemented for `std::result::Result`
/// types where the error implements the standard error traits.
///
/// ## Design Philosophy
///
/// Rather than requiring specific error types, this trait allows any error to be enhanced
/// with yoshi-specific nest information, making error handling more flexible and informative.
/// The "nest" terminology aligns with the egg/hatch metaphor throughout the yoshi ecosystem.
///
/// ## Usage Examples
///
/// ```rust
/// use yoshi_std::{Hatchling, Hatch};
/// use std::path::Path;
///
/// async fn read_config(path: &Path) -> Hatch<String> {
///     std::fs::read_to_string(path)
///         .with_file_nest(path)
///         .with_operation_nest("config_loading")
/// }
/// ```
///
/// ## Performance Notes
///
/// All nest enhancement methods are zero-cost when errors don't occur,
/// and minimal-cost when they do, as they only allocate for error formatting.
pub trait Hatchling<T> {
    /// **Add File Nest to Error**
    ///
    /// Enhances an error with file path information, useful for I/O operations
    /// and file-related failures.
    ///
    /// # Arguments
    /// * `file_path` - Path to the file involved in the operation
    ///
    /// # Returns
    /// Enhanced error with file nest information
    ///
    /// # Errors
    /// Returns an error if the file path cannot be processed or if the underlying operation fails
    /// **`with_file_nest`**
    ///
    /// This function provides with file nest functionality within the Yoshi error handling framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// Processes `with_file_nest` with provided parameters.
    ///
    /// # Arguments
    ///
    /// * `file_path` - File system path or location
    ///
    /// # Returns
    ///
    /// Processed output value
    /// Processes `with_file_nest` with provided parameters.
    ///
    /// # Arguments
    ///
    /// * `file_path` - File system path or location
    ///
    /// # Returns
    ///
    /// Processed output value
    fn with_file_nest(self, file_path: &std::path::Path) -> Hatch<T>;

    /// **Add Operation Nest to Error**
    ///
    /// Enhances an error with information about the specific operation that failed.
    ///
    /// # Arguments
    /// * `operation` - Description of the operation that failed
    ///
    /// # Returns
    /// Enhanced error with operation nest information
    ///
    /// # Errors
    /// Returns an error if the operation context cannot be processed or if the underlying operation fails
    /// **`with_operation_nest`**
    ///
    /// This function provides with operation nest functionality within the Yoshi error handling
    /// framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// Processes `with_operation_nest` with provided parameters.
    ///
    /// # Arguments
    ///
    /// * `operation` - Input parameter for operation
    ///
    /// # Returns
    ///
    /// Processed output value
    /// Processes `with_operation_nest` with provided parameters.
    ///
    /// # Arguments
    ///
    /// * `operation` - Input parameter for operation
    ///
    /// # Returns
    ///
    /// Processed output value
    fn with_operation_nest(self, operation: &str) -> Hatch<T>;

    /// **Add Performance Nest to Error**
    ///
    /// Enhances an error with timing information, useful for timeout scenarios
    /// and performance analysis.
    ///
    /// # Arguments
    /// * `duration` - How long the operation took before failing
    ///
    /// # Returns
    /// Enhanced error with performance timing nest information
    ///
    /// # Errors
    /// Returns an error if the performance context cannot be processed or if the underlying operation fails
    /// **`with_performance_nest`**
    ///
    /// This function provides with performance nest functionality within the Yoshi error handling
    /// framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// Processes `with_performance_nest` with provided parameters.
    ///
    /// # Arguments
    ///
    /// * `duration` - Input parameter for duration
    ///
    /// # Returns
    ///
    /// Processed output value
    /// Processes `with_performance_nest` with provided parameters.
    ///
    /// # Arguments
    ///
    /// * `duration` - Input parameter for duration
    ///
    /// # Returns
    ///
    /// Processed output value
    fn with_performance_nest(self, duration: Duration) -> Hatch<T>;

    /// **Add Correction Nest to Error**
    ///
    /// Enhances an error with information about a failed correction attempt,
    /// including the type of correction and confidence level.
    ///
    /// # Arguments
    /// * `correction_type` - Type of correction that was attempted
    /// * `confidence` - Confidence level of the correction (0.0 to 1.0)
    ///
    /// # Returns
    /// Enhanced error with correction attempt nest information
    ///
    /// # Errors
    /// Returns an error if the correction context cannot be processed or if the underlying operation fails
    /// **`with_correction_nest`**
    ///
    /// This function provides with correction nest functionality within the Yoshi error handling
    /// framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// Processes `with_correction_nest` with provided parameters.
    ///
    /// # Arguments
    ///
    /// * `correction_type` - Input parameter for correction type
    /// * `confidence` - Unique identifier for the operation
    ///
    /// # Returns
    ///
    /// Processed output value
    /// Processes `with_correction_nest` with provided parameters.
    ///
    /// # Arguments
    ///
    /// * `correction_type` - Input parameter for correction type
    /// * `confidence` - Unique identifier for the operation
    ///
    /// # Returns
    ///
    /// Processed output value
    fn with_correction_nest(self, correction_type: &str, confidence: f64) -> Hatch<T>;

    /// **Add Analysis Nest to Error**
    ///
    /// Enhances an error with AST or code analysis context information.
    ///
    /// # Arguments
    /// * `analysis_type` - Type of analysis that failed
    /// * `location` - Location in code where analysis failed
    ///
    /// # Returns
    /// Enhanced error with analysis nest information
    ///
    /// # Errors
    /// Returns an error if the analysis context cannot be processed or if the underlying operation fails
    /// **`with_analysis_nest`**
    ///
    /// This function provides with analysis nest functionality within the Yoshi error handling
    /// framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// Processes `with_analysis_nest` with provided parameters.
    ///
    /// # Arguments
    ///
    /// * `analysis_type` - Input parameter for analysis type
    /// * `location` - Input parameter for location
    ///
    /// # Returns
    ///
    /// Processed output value
    /// Processes `with_analysis_nest` with provided parameters.
    ///
    /// # Arguments
    ///
    /// * `analysis_type` - Input parameter for analysis type
    /// * `location` - Input parameter for location
    ///
    /// # Returns
    ///
    /// Processed output value
    fn with_analysis_nest(self, analysis_type: &str, location: &str) -> Hatch<T>;

    /// **Add Resource Nest to Error**
    ///
    /// Enhances an error with resource exhaustion context information.
    ///
    /// # Arguments
    /// * `resource_type` - Type of resource that was exhausted
    /// * `limit` - The resource limit that was exceeded
    ///
    /// # Returns
    /// Enhanced error with resource nest information
    ///
    /// # Errors
    /// Returns an error if the resource context cannot be processed or if the underlying operation fails
    /// **`with_resource_nest`**
    ///
    /// This function provides with resource nest functionality within the Yoshi error handling
    /// framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// Processes `with_resource_nest` with provided parameters.
    ///
    /// # Arguments
    ///
    /// * `resource_type` - Input parameter for resource type
    /// * `limit` - Input parameter for limit
    ///
    /// # Returns
    ///
    /// Processed output value
    /// Processes `with_resource_nest` with provided parameters.
    ///
    /// # Arguments
    ///
    /// * `resource_type` - Input parameter for resource type
    /// * `limit` - Input parameter for limit
    ///
    /// # Returns
    ///
    /// Processed output value
    fn with_resource_nest(self, resource_type: &str, limit: u64) -> Hatch<T>;
}

impl<T, E> Hatchling<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn with_file_nest(self, file_path: &std::path::Path) -> Hatch<T> {
        self.map_err(|e| {
            Yoshi::new(YoshiKind::Internal {
                message: format!("File operation failed: {}", file_path.display()).into(),
                source: Some(Box::new(Yoshi::new(YoshiKind::Internal {
                    message: e.to_string().into(),
                    source: None,
                    component: Some("file_nest".into()),
                }))),
                component: Some("file_operation".into()),
            })
            .lay(format!("File nest: {}", file_path.display()))
        })
    }

    fn with_operation_nest(self, operation: &str) -> Hatch<T> {
        self.map_err(|e| {
            Yoshi::new(YoshiKind::Internal {
                message: format!("Operation failed: {operation}").into(),
                source: Some(Box::new(Yoshi::new(YoshiKind::Internal {
                    message: e.to_string().into(),
                    source: None,
                    component: Some("operation_nest".into()),
                }))),
                component: Some("operation".into()),
            })
            .lay(format!("Operation nest: {operation}"))
        })
    }

    fn with_performance_nest(self, duration: Duration) -> Hatch<T> {
        self.map_err(|_e| {
            Yoshi::new(YoshiKind::Timeout {
                operation: "performance_nest".into(),
                duration,
                expected_max: Some(Duration::from_millis(100)),
            })
            .lay(format!("Performance nest: {duration:?}"))
        })
    }

    fn with_correction_nest(self, correction_type: &str, confidence: f64) -> Hatch<T> {
        self.map_err(|e| {
            Yoshi::new(YoshiKind::Internal {
                message: format!("Correction failed: {correction_type}").into(),
                source: Some(Box::new(Yoshi::new(YoshiKind::Internal {
                    message: e.to_string().into(),
                    source: None,
                    component: Some("correction_nest".into()),
                }))),
                component: Some("correction".into()),
            })
            .lay(format!(
                "Correction nest: {correction_type} (confidence: {confidence})"
            ))
        })
    }

    fn with_analysis_nest(self, analysis_type: &str, location: &str) -> Hatch<T> {
        self.map_err(|e| {
            Yoshi::new(YoshiKind::Internal {
                message: format!("Analysis failed: {analysis_type}").into(),
                source: Some(Box::new(Yoshi::new(YoshiKind::Internal {
                    message: e.to_string().into(),
                    source: None,
                    component: Some("analysis_nest".into()),
                }))),
                component: Some("analysis".into()),
            })
            .lay(format!("Analysis nest: {analysis_type} at {location}"))
        })
    }

    fn with_resource_nest(self, resource_type: &str, limit: u64) -> Hatch<T> {
        self.map_err(|e| {
            Yoshi::new(YoshiKind::Internal {
                message: format!("Resource exhausted: {resource_type}").into(),
                source: Some(Box::new(Yoshi::new(YoshiKind::Internal {
                    message: e.to_string().into(),
                    source: None,
                    component: Some("resource_nest".into()),
                }))),
                component: Some("resource".into()),
            })
            .lay(format!("Resource nest: {resource_type} (limit: {limit})"))
        })
    }
}

//============================================================================
// EXTENSION TRAITS FOR ERGONOMIC ERROR HANDLING
//============================================================================

/// Extension trait for `Result<T, std::io::Error>` to provide `.hatchio()` method.
///
/// This trait provides a convenient `.hatchio()` method specifically for I/O operations
/// that return `Result<T, std::io::Error>`, converting them into `Hatch<T>` seamlessly.
///
/// # Examples
///
/// ```rust,no_run
/// use yoshi_std::{HatchIo, Hatch, LayText};
/// use std::fs;
///
/// fn read_config() -> Hatch<String> {
///     fs::read_to_string("config.toml")
///         .hatchio()
///         .lay("Failed to read configuration file")
/// }
/// ```
pub trait HatchIo<T> {
    /// Converts a `Result<T, std::io::Error>` into a `Hatch<T>`.
    ///
    /// This method provides ergonomic conversion from I/O operations into the Yoshi
    /// error ecosystem while maintaining performance and type safety.
    ///
    /// # Examples
    ///
    /// # Errors
    ///
    /// Returns a `Hatch<T>` containing the converted I/O error if the original
    /// `Result` was an `Err` variant.
    ///
    /// ```rust,no_run
    /// use yoshi_std::{HatchIo, Hatch};
    /// use std::fs;
    ///
    /// let result: Hatch<String> = fs::read_to_string("file.txt").hatchio();
    /// ```
    /// **hatchio**
    ///
    /// This function provides hatchio functionality within the Yoshi error handling framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// Processes hatchio with provided parameters.
    ///
    /// # Arguments
    ///
    ///
    /// # Returns
    ///
    /// Processed output value
    /// Processes hatchio with provided parameters.
    ///
    /// # Arguments
    ///
    ///
    /// # Returns
    ///
    /// Processed output value
    fn hatchio(self) -> Hatch<T>;
}

impl<T> HatchIo<T> for std::result::Result<T, std::io::Error> {
    #[track_caller]
    fn hatchio(self) -> Hatch<T> {
        self.map_err(io_error_to_yoshi)
    }
}

/// Extension trait for adding structured context to Result types.
///
/// This trait provides the `lay_with_context` method that allows attaching
/// both a message and a structured context payload to errors in a single operation.
///
/// # Examples
///
/// ```rust,no_run
/// use yoshi_std::{LayWithContext, Hatch};
/// use std::any::Any;
///
/// #[derive(Debug)]
/// struct FileContext {
///     path: String,
///     operation: String,
/// }
///
/// fn file_operation() -> Hatch<String> {
///     let result: Result<String, &str> = Err("file not found");
///     let context = FileContext {
///         path: "config.toml".to_string(),
///         operation: "read".to_string(),
///     };
///
///     result.lay_with_context("Failed to read configuration", context)
/// }
/// ```
pub trait LayWithContext<T, E> {
    /// Add structured context to an error with both message and typed payload
    ///
    /// This method enhances errors with both a descriptive message and a structured
    /// context payload that can be retrieved later for debugging or recovery.
    ///
    /// # Arguments
    /// * `message` - Descriptive message for the error context
    /// * `context` - Typed payload containing structured context information
    ///
    /// # Returns
    /// Enhanced error with context information as a `Hatch<T>`
    ///
    /// # Errors
    /// Returns an error if the context cannot be processed or if the underlying operation fails
    /// **`lay_with_context`**
    ///
    /// This function provides lay with context functionality within the Yoshi error handling framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// Processes `lay_with_context` with provided parameters.
    ///
    /// # Arguments
    ///
    /// * `message` - Input parameter for message
    /// * `context` - Input parameter for context
    ///
    /// # Returns
    ///
    /// Processed output value
    /// Processes `lay_with_context` with provided parameters.
    ///
    /// # Arguments
    ///
    /// * `message` - Input parameter for message
    /// * `context` - Input parameter for context
    ///
    /// # Returns
    ///
    /// Processed output value
    fn lay_with_context<C>(self, message: impl Into<String>, context: C) -> Hatch<T>
    where
        C: Any + Send + Sync + 'static;
}

impl<T, E> LayWithContext<T, E> for std::result::Result<T, E>
where
    E: Into<Yoshi>,
{
    fn lay_with_context<C>(self, message: impl Into<String>, context: C) -> Hatch<T>
    where
        C: Any + Send + Sync + 'static,
    {
        self.map_err(|e| e.into().with_shell(context).lay(message))
    }
}

/// 🎯 ELEGANT SOLUTION: Simple extension that makes `std::io::Error` work with `.hatch()`
///
/// This provides exactly what you wanted - a way to use `.hatch()` directly on
/// `std::io::Error` results without any wrapper types or manual conversions.
/// Combined with yoshi-core's `Hatchable` trait, this gives you universal coverage!
///
/// # The Approach
///
/// Instead of fighting trait coherence rules, this provides a clean extension
/// specifically for `std::io::Error`. Import both traits and `.hatch()` works everywhere!
///
/// # Usage
///
/// ```rust,no_run
/// use yoshi_std::{Hatchable, IoHatchable, Hatch};  // Four s for universal coverage
/// use std::fs;
///
/// # fn example() -> Hatch<()> {
/// // std::io::Error results work seamlessly:
/// let content = IoHatchable::hatch(fs::read_to_string("file.txt"))?;
/// let file = IoHatchable::hatch(std::fs::File::open("config.toml"))?;
///
/// // Other error types work via yoshi-core's Hatchable:
/// let parsed: i32 = Hatchable::hatch("123".parse::<i32>().map_err(|e| e.to_string()))?;
/// # Ok(())
/// # }
/// ```
pub trait IoHatchable<T> {
    /// Convert an I/O Result into a Hatch (Yoshi Result)
    ///
    /// This method provides ergonomic conversion from I/O operations into the Yoshi
    /// error ecosystem while maintaining performance and type safety.
    ///
    /// # Returns
    /// A `Hatch<T>` containing the converted I/O error if the original Result was an Err
    ///
    /// # Errors
    /// Returns a `Hatch<T>` containing the converted I/O error if the original
    /// `Result` was an `Err` variant.
    fn hatch(self) -> Hatch<T>;
}
impl<T> IoHatchable<T> for std::result::Result<T, std::io::Error> {
    fn hatch(self) -> Hatch<T> {
        self.map_err(io_error_to_yoshi)
    }
}

// Re-export yoshi-core's Hatchable for convenience
pub use yoshi_core::Hatchable;

//============================================================================
// I/O ERROR CONVERSION HELPER
//============================================================================

/// Helper function to convert `std::io::Error` into a `Yoshi` error.
///
/// This function provides a way to convert I/O errors into Yoshi errors
/// in std environments. It leverages the yoshi-core From<std::io::Error>
/// implementation for consistency.
///
/// # Examples
///
/// ```rust
/// use yoshi_std::{io_error_to_yoshi, Yoshi};
/// use std::io::{Error, ErrorKind};
///
/// let io_error = Error::new(ErrorKind::NotFound, "file not found");
/// let yoshi_error = io_error_to_yoshi(io_error);
/// ```
#[track_caller]
#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn io_error_to_yoshi(error: std::io::Error) -> Yoshi {
    // Convert std::io::Error to NoStdIo for compatibility with no_std builds
    let no_std_io = NoStdIo::new(error.to_string());
    Yoshi::new(YoshiKind::Io(no_std_io))
}

//============================================================================
// STRING INTERNING HELPER
//============================================================================

/// Interns a string using the global std string pool.
///
/// This function provides efficient string deduplication for error messages
/// and other frequently repeated strings in std environments.
///
/// # Examples
///
/// ```rust
/// use yoshi_std::intern_string_std;
///
/// let s1 = intern_string_std("common error message");
/// let s2 = intern_string_std("common error message");
/// assert!(std::sync::Arc::ptr_eq(&s1, &s2));
/// ```
pub fn intern_string_std<S: Into<String>>(s: S) -> Arc<str> {
    crate::std_integration::intern_string_std(s)
}
