/* benches/error_creation.rs */
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
//! **Brief:** Performance benchmarks for Yoshi error creation operations with mathematical precision.
//!
//! **Module Classification:** Performance-Critical
//! **Complexity Level:** Expert
//! **API Stability:** Stable
//!
//! ## Mathematical Properties
//!
//! **Algorithmic Complexity:**
//! - Time Complexity: O(1) for basic error creation, O(k) for k context attachments
//! - Space Complexity: O(1) base allocation + O(k) for k attached contexts
//! - Concurrency Safety: Send + Sync guaranteed across all error types
//!
//! **Performance Characteristics:**
//! - Expected Performance: < 50ns for basic error creation
//! - Worst-Case Scenarios: < 200ns with full context and backtrace
//! - Optimization Opportunities: String interning and Arc optimization
//!
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Comprehensive Error Creation Benchmarks with Mathematical Analysis]
//!  - [Basic Error Creation: O(1) time complexity with allocation analysis]
//!  - [Context Attachment: O(k) time complexity for k contexts with memory pooling]
//!  - [Payload Attachment: O(1) amortized with type erasure overhead analysis]
//!  - [Backtrace Capture: Variable complexity with performance cost measurement]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** Business Source License 1.1 (BSL-1.1)
// **License Terms:** Non-production use only; commercial/production use requires paid license.
// **Effective Date:** 2025-05-25 | **Change License:** GPL v3
// **License File:** /LICENSE
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn
// **Last Validation:** 2025-05-30

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::time::Duration;
use yoshi_std::{Yoshi, YoshiKind, YoshiLocation}; // Removed YoshiContext, YoshiContextExt

/// Benchmarks basic error creation operations
fn bench_basic_error_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("basic_error_creation");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(10000);

    // Internal error creation
    group.bench_function("internal_error", |b| {
        b.iter(|| {
            let error = Yoshi::new(YoshiKind::Internal {
                message: std::hint::black_box("Critical system failure".into()),
                source: None,
                component: Some(std::hint::black_box("database".into())),
            });
            std::hint::black_box(error)
        })
    });

    // Validation error creation
    group.bench_function("validation_error", |b| {
        b.iter(|| {
            let error = Yoshi::new(YoshiKind::Validation {
                field: std::hint::black_box("email".into()),
                message: std::hint::black_box("Invalid email format".into()),
                expected: Some(std::hint::black_box("user@domain.com".into())),
                actual: Some(std::hint::black_box("invalid-email".into())),
            });
            std::hint::black_box(error)
        })
    }); // Network error creation
    group.bench_function("network_error", |b| {
        b.iter(|| {
            let error = Yoshi::new(YoshiKind::Network {
                message: std::hint::black_box("HTTP GET failed".into()),
                source: Some(Box::new(Yoshi::new(YoshiKind::Internal {
                    message: std::hint::black_box("Underlying I/O error".into()),
                    source: None,
                    component: None,
                }))), // Correctly initialize source
                error_code: Some(std::hint::black_box("5001").parse().unwrap_or(0)), // Parse string to u32
            });
            std::hint::black_box(error)
        })
    });

    // Timeout error creation
    group.bench_function("timeout_error", |b| {
        b.iter(|| {
            let error = Yoshi::new(YoshiKind::Timeout {
                operation: std::hint::black_box("database query".into()),
                duration: Duration::from_secs(30),
                expected_max: Some(Duration::from_secs(10)),
            });
            std::hint::black_box(error)
        })
    });

    group.finish();
}

/// Benchmarks error creation with context attachments
fn bench_error_with_context(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_with_context");
    group.measurement_time(Duration::from_secs(10));

    for context_count in [1, 3, 5, 10].iter() {
        group.throughput(Throughput::Elements(*context_count as u64));

        group.bench_with_input(
            BenchmarkId::new("contexts", context_count),
            context_count,
            |b, &context_count| {
                b.iter(|| {
                    let mut error = Yoshi::new(YoshiKind::Internal {
                        message: std::hint::black_box("Base error".into()),
                        source: None,
                        component: None,
                    });

                    for i in 0..context_count {
                        // For Yoshi, context is added as a String.
                        // YoshiContextExt provides the .context() method.
                        error = error
                            .context(format!("Context {}", i)) // Use .context() which takes Into<String>
                            .with_metadata("index", i.to_string())
                            .with_metadata("timestamp", "2025-05-30T00:00:00Z")
                            .with_suggestion(format!("Try approach {}", i));
                    }

                    std::hint::black_box(error)
                })
            },
        );
    }

    group.finish();
}

/// Benchmarks error creation with typed payloads
fn bench_error_with_payloads(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_with_payloads");
    group.measurement_time(Duration::from_secs(10));

    #[derive(Debug, Clone)]
    struct CustomPayload {
        _data: Vec<u32>,   // Marked as unused
        _metadata: String, // Marked as unused
    }

    for payload_size in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*payload_size as u64));

        group.bench_with_input(
            BenchmarkId::new("payload_size", payload_size),
            payload_size,
            |b, &payload_size| {
                b.iter(|| {
                    let payload = CustomPayload {
                        _data: (0..payload_size).collect(),
                        _metadata: format!("Metadata with {} elements", payload_size),
                    };

                    let error = Yoshi::new(YoshiKind::Internal {
                        message: std::hint::black_box("Error with payload".into()),
                        source: None,
                        component: None,
                    })
                    .with_payload(std::hint::black_box(payload));

                    std::hint::black_box(error)
                })
            },
        );
    }

    group.finish();
}

/// Benchmarks error creation with location capture
fn bench_error_with_location(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_with_location");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("with_location", |b| {
        b.iter(|| {
            let location = YoshiLocation::new("src/benchmarks.rs", 123, 45);
            let error = Yoshi::new(YoshiKind::Internal {
                message: std::hint::black_box("Error with location".into()),
                source: None,
                component: None,
            })
            .with_location(location);

            std::hint::black_box(error)
        })
    });

    group.bench_function("with_macro_location", |b| {
        b.iter(|| {
            let error = Yoshi::new(YoshiKind::Internal {
                message: std::hint::black_box("Error with macro location".into()),
                source: None,
                component: None,
            })
            .with_location(yoshi_std::yoshi_location!());

            std::hint::black_box(error)
        })
    });

    group.finish();
}

/// Benchmarks backtrace capture performance
fn bench_error_with_backtrace(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_with_backtrace");
    group.measurement_time(Duration::from_secs(15)); // Longer measurement for backtrace overhead

    group.bench_function("without_backtrace", |b| {
        b.iter(|| {
            let error = Yoshi::new(YoshiKind::Internal {
                message: std::hint::black_box("Error without backtrace".into()),
                source: None,
                component: None,
            });
            std::hint::black_box(error)
        })
    });

    group.bench_function("with_backtrace", |b| {
        b.iter(|| {
            // Force backtrace capture by setting environment variable effect
            std::env::set_var("RUST_BACKTRACE", "1");
            let error = Yoshi::new(YoshiKind::Internal {
                message: std::hint::black_box("Error with backtrace".into()),
                source: None,
                component: None,
            });
            std::hint::black_box(error)
        })
    });

    group.finish();
}

/// Benchmarks error creation from standard library types
fn bench_error_from_std_types(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_from_std_types");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("from_string", |b| {
        b.iter(|| {
            let error = Yoshi::from(std::hint::black_box("Error from string".to_string()));
            std::hint::black_box(error)
        })
    });

    group.bench_function("from_str", |b| {
        b.iter(|| {
            let error = Yoshi::from(std::hint::black_box("Error from str"));
            std::hint::black_box(error)
        })
    });

    group.bench_function("from_io_error", |b| {
        b.iter(|| {
            let io_error = std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                std::hint::black_box("Permission denied"),
            );
            let error = Yoshi::from(std::hint::black_box(io_error));
            std::hint::black_box(error)
        })
    });

    group.finish();
}

/// Comprehensive benchmark suite configuration
fn configure_benchmark_suite() {
    std::env::set_var("CRITERION_BENCH", "1");
}

criterion_group!(
    name = benches;
    config = {
        let config = Criterion::default() // Removed mut
            .significance_level(0.01)
            .confidence_level(0.95)
            .warm_up_time(Duration::from_millis(500))
            .measurement_time(Duration::from_secs(5));

        configure_benchmark_suite();
        config
    };
    targets =
        bench_basic_error_creation,
        bench_error_with_context,
        bench_error_with_payloads,
        bench_error_with_location,
        bench_error_with_backtrace, // Assuming it's always included or std is always a feature
        bench_error_from_std_types
);

criterion_main!(benches); // Changed to 'benches'
