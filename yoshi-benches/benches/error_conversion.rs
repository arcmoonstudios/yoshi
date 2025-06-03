/* yoshi-benches\benches\error_conversion.rs */
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
//! **Brief:** Performance benchmarks for Yoshi error conversion operations with type safety analysis.
//!
//! **Module Classification:** Performance-Critical
//! **Complexity Level:** Expert
//! **API Stability:** Stable
//!
//! ## Mathematical Properties
//!
//! **Algorithmic Complexity:**
//! - Time Complexity: O(1) for direct conversions, O(k) for chained conversions
//! - Space Complexity: O(1) base allocation + O(n) for error chain depth n
//! - Concurrency Safety: Thread-safe conversions with zero data races
//!
//! **Performance Characteristics:**
//! - Expected Performance: < 20ns for direct type conversions
//! - Worst-Case Scenarios: < 100ns for complex error chain mappings
//! - Optimization Opportunities: Zero-copy conversions and trait specialization
//!
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Comprehensive Error Conversion Benchmarks with Type Analysis]
//!  - [Direct Type Conversions: O(1) with zero-allocation where possible]
//!  - [Error Chain Mapping: O(n) for chain depth n with source preservation]
//!  - [Foreign Error Integration: O(1) with type name capture and metadata]
//!  - [Result Type Conversions: O(1) with optimized success/failure paths]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **License Terms:** Full open source freedom; dual licensing allows choice between MIT and Apache 2.0.
// **Effective Date:** 2025-06-02 | **Open Source Release|2025-06-02 | **Open Source Release
// **License File:** /LICENSE
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn
// **Last Validation:** 2025-06-02

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::error::Error; // Import the Error trait
use std::hint::black_box; // Use std::hint::black_box
use std::time::Duration;
use yoshi_std::{Result, Yoshi, YoshiKind};

/// Custom error types for conversion benchmarks - Pure Rust implementations
#[derive(Debug)]
#[allow(clippy::enum_variant_names)] // Allow for specific naming convention
enum CustomError {
    /// Database connection error
    Database {
        /// Error message
        message: String,
    },
    /// Authentication error (used in comprehensive benchmarks)
    #[allow(dead_code)]
    Auth {
        /// User ID that failed authentication
        user_id: u64,
    },
    /// Validation error (used in comprehensive benchmarks)
    #[allow(dead_code)]
    Validation,
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::Database { message } => {
                write!(f, "Database connection failed: {message}")
            }
            CustomError::Auth { user_id } => {
                write!(f, "Authentication failed for user: {user_id}")
            }
            CustomError::Validation => {
                write!(f, "Validation failed")
            }
        }
    }
}

impl std::error::Error for CustomError {}

#[derive(Debug)]
struct ComplexError {
    code: u32,
    message: String,
    details: Vec<String>,
}

impl std::fmt::Display for ComplexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error {}: {} (details: {})",
            self.code,
            self.message,
            self.details.len()
        )
    }
}

impl std::error::Error for ComplexError {}

/// Benchmarks direct type conversions to Yoshi
fn bench_direct_conversions(c: &mut Criterion) {
    let mut group = c.benchmark_group("direct_conversions");
    group.measurement_time(Duration::from_secs(8));
    group.sample_size(10000);

    // String to Yoshi conversion
    group.bench_function("string_to_yoshi", |b| {
        b.iter(|| {
            let error: Yoshi = black_box("Database connection timeout".to_string()).into();
            black_box(error);
        });
    });

    // &str to Yoshi conversion
    group.bench_function("str_to_yoshi", |b| {
        b.iter(|| {
            let error: Yoshi = black_box("Invalid user credentials").into();
            black_box(error);
        });
    });

    // std::io::Error to Yoshi conversion
    group.bench_function("io_error_to_yoshi", |b| {
        b.iter(|| {
            let io_error =
                std::io::Error::new(std::io::ErrorKind::NotFound, black_box("File not found"));
            let error: Yoshi = black_box(io_error).into();
            black_box(error);
        });
    });

    // Custom error to Yoshi conversion using Yoshi::foreign
    group.bench_function("custom_error_to_yoshi", |b| {
        b.iter(|| {
            let custom_error = CustomError::Database {
                message: black_box("Connection pool exhausted".to_string()),
            };
            let error: Yoshi = Yoshi::foreign(black_box(custom_error));
            black_box(error);
        });
    });

    group.finish();
}

/// Benchmarks Result type conversions
fn bench_result_conversions(c: &mut Criterion) {
    let mut group = c.benchmark_group("result_conversions");
    group.measurement_time(Duration::from_secs(8));

    // Success case conversion
    group.bench_function("ok_result_conversion", |b| {
        b.iter(|| {
            let std_result: std::result::Result<i32, String> = Ok(black_box(42));
            let yoshi_result: Result<i32> = std_result.map_err(Yoshi::from);
            let _ = black_box(yoshi_result);
        });
    });

    // Error case conversion
    group.bench_function("err_result_conversion", |b| {
        b.iter(|| {
            let std_result: std::result::Result<i32, String> =
                Err(black_box("Operation failed".to_string()));
            let yoshi_result: Result<i32> = std_result.map_err(Yoshi::from);
            let _ = black_box(yoshi_result);
        });
    });

    // Chain multiple conversions
    group.bench_function("chained_result_conversion", |b| {
        b.iter(|| {
            let result: Result<i32> = black_box("Initial error")
                .parse::<i32>()
                // Fix: Use Yoshi::foreign for ParseIntError
                .map_err(Yoshi::foreign)
                .and_then(|n| {
                    if n > 0 {
                        Ok(n * 2)
                    } else {
                        Err(Yoshi::from("Number must be positive"))
                    }
                });
            let _ = black_box(result);
        });
    });

    group.finish();
}

/// Benchmarks foreign error integration using pure Yoshi capabilities
fn bench_foreign_error_integration(c: &mut Criterion) {
    let mut group = c.benchmark_group("foreign_error_integration");
    group.measurement_time(Duration::from_secs(8));

    // Simple foreign error using std::fmt::Error via Yoshi::foreign
    group.bench_function("simple_foreign_error", |b| {
        b.iter(|| {
            let error = Yoshi::foreign(black_box(std::fmt::Error));
            black_box(error);
        });
    });

    // Complex foreign error conversion via Yoshi::foreign()
    group.bench_function("complex_foreign_error", |b| {
        b.iter(|| {
            let complex_error = ComplexError {
                code: black_box(500),
                message: black_box("Internal server error".to_string()),
                details: black_box(vec![
                    "Database unavailable".to_string(),
                    "Circuit breaker open".to_string(),
                ]),
            };
            let error = Yoshi::foreign(black_box(complex_error));
            black_box(error);
        });
    });

    group.finish();
}

/// Benchmarks error chain operations using Yoshi's native context capabilities
#[allow(clippy::cast_sign_loss)] // `chain_depth` is always positive from the array
fn bench_error_chain_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_chain_operations");
    group.measurement_time(Duration::from_secs(10));

    for chain_depth in &[1, 3, 5, 10] {
        group.throughput(Throughput::Elements(*chain_depth as u64));

        group.bench_with_input(
            BenchmarkId::new("chain_depth", chain_depth),
            chain_depth,
            |b, &chain_depth| {
                b.iter(|| {
                    // Start with a base Yoshi error
                    let mut current_error = Yoshi::new(YoshiKind::Internal {
                        message: black_box("Root cause error".into()),
                        source: None,    // Required field
                        component: None, // Required field
                    });

                    // Build error chain using Yoshi's context system
                    for i in 1..chain_depth {
                        current_error = current_error.context(
                            black_box(format!("Layer {i} context")), // `Yoshi::context` takes String, direct format arg
                        );
                    }

                    black_box(current_error);
                });
            },
        );
    }

    group.finish();
}

/// Benchmarks multiple error aggregation using Yoshi's Multiple variant
#[allow(clippy::cast_sign_loss)] // `error_count` is always positive from the array
fn bench_multiple_error_aggregation(c: &mut Criterion) {
    let mut group = c.benchmark_group("multiple_error_aggregation");
    group.measurement_time(Duration::from_secs(10));

    for error_count in &[2, 5, 10, 20] {
        group.throughput(Throughput::Elements(*error_count as u64));

        group.bench_with_input(
            BenchmarkId::new("error_count", error_count),
            error_count,
            |b, &error_count| {
                b.iter(|| {
                    let mut errors = Vec::with_capacity(error_count);

                    for i in 0..error_count {
                        let error = Yoshi::new(YoshiKind::Validation {
                            field: black_box(format!("field_{i}").into()), // Direct format arg
                            message: black_box(format!("Validation error {i}").into()), // Direct format arg
                            expected: None,
                            actual: None,
                        });
                        errors.push(error);
                    }

                    let aggregated = Yoshi::new(YoshiKind::Multiple {
                        errors: black_box(errors),
                        primary_index: Some(0),
                    });

                    black_box(aggregated);
                });
            },
        );
    }

    group.finish();
}

/// Benchmarks type-safe downcasting operations
fn bench_error_downcasting(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_downcasting");
    group.measurement_time(Duration::from_secs(8));

    // Successful downcast
    group.bench_function("successful_downcast", |b| {
        let io_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Access denied");
        let yoshi_error = Yoshi::from(io_error);

        b.iter(|| {
            // Simulate accessing the underlying source (std::error::Error trait is in scope)
            let source_result = yoshi_error.source();
            black_box(source_result);
        });
    });

    // Failed downcast
    group.bench_function("failed_downcast", |b| {
        let yoshi_error = Yoshi::from("String error");

        b.iter(|| {
            // Try to access source from a string-based error (should be None)
            let source_result = yoshi_error.source();
            black_box(source_result);
        });
    });

    group.finish();
}

/// Benchmarks error context preservation during conversion
fn bench_context_preservation(c: &mut Criterion) {
    let mut group = c.benchmark_group("context_preservation");
    group.measurement_time(Duration::from_secs(8));

    group.bench_function("context_preserved_conversion", |b| {
        b.iter(|| {
            let io_error = std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                black_box("Connection timeout"),
            );
            let yoshi_error = Yoshi::from(io_error)
                .context("During database connection".to_string()) // Use .context(String)
                .with_metadata("operation", "SELECT * FROM users") // Use &str for metadata
                .with_metadata("timeout_ms", "5000"); // Use &str for metadata

            black_box(yoshi_error);
        });
    });

    group.finish();
}

criterion_group!(
    error_conversion_benches,
    bench_direct_conversions,
    bench_result_conversions,
    bench_foreign_error_integration,
    bench_error_chain_operations,
    bench_multiple_error_aggregation,
    bench_error_downcasting,
    bench_context_preservation
);

criterion_main!(error_conversion_benches);
