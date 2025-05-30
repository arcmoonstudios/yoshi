/* benches/cross_crate_integration.rs */
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
//! **Brief:** Cross-crate integration benchmarks for the complete Yoshi error handling ecosystem.
//!
//! **Module Classification:** Performance-Critical
//! **Complexity Level:** Expert
//! **API Stability:** Stable
//!
//! ## Mathematical Properties
//!
//! **Algorithmic Complexity:**
//! - Time Complexity: O(1) for facade operations, O(k) for k-level error propagation
//! - Space Complexity: O(1) base + O(k) for propagation chains with optimal memory layout
//! - Concurrency Safety: Full Send + Sync guarantees across crate boundaries
//!
//! **Performance Characteristics:**
//! - Expected Performance: < 100ns for complete cross-crate error lifecycle
//! - Worst-Case Scenarios: < 500ns with full derive macro expansion and formatting
//! - Optimization Opportunities: Zero-cost abstractions and compile-time optimization
//!
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Cross-Crate Integration Benchmarks with Mathematical Precision]
//!  - [Facade to Implementation: O(1) delegation with zero-cost abstraction verification]
//!  - [Derive Macro Performance: Compile-time vs runtime cost analysis]
//!  - [Error Propagation Chains: O(k) scaling across crate boundaries]
//!  - [Complete Ecosystem Integration: End-to-end performance measurement]
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
use serde_json::json;
use std::hint::black_box;
use std::time::Duration;

// Import from all workspace crates to test integration
// FacadeYoshi and Yoshi are type aliases to the same struct yoshi_std::Yoshi
use yoshi::{Yoshi as FacadeYoshi, YoshiKind as FacadeYoshiKind}; // Facade crate
use yoshi_derive::YoshiError;
use yoshi_std::{Yoshi, YoshiKind}; // Core implementation // Derive macros

// Test error type using derive macro
#[derive(Debug, YoshiError)]
enum TestError {
    // Adjusted kind to "Validation" as per yoshi-derive's mapping
    #[yoshi(kind = "Validation", display = "Invalid user input: {_input}")]
    InvalidInput { _input: String },

    // Adjusted kind to "Network" as per yoshi-derive's mapping for database errors
    #[yoshi(kind = "Network", display = "Database operation failed: {_operation}")]
    DatabaseError { _operation: String },

    // Adjusted kind to "Network" as per yoshi-derive's mapping
    #[yoshi(kind = "Network", display = "Network request failed: {url}")]
    NetworkError { url: String },
}

/// Benchmarks facade crate performance and delegation
fn bench_facade_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("facade_operations");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(10000);

    // Direct facade error creation
    group.bench_function("facade_error_creation", |b| {
        b.iter(|| {
            let error = FacadeYoshi::new(FacadeYoshiKind::Internal {
                message: black_box("Facade error".into()),
                source: None,    // Required field
                component: None, // Required field
            });
            black_box(error);
        })
    });

    // Facade vs core implementation comparison
    group.bench_function("core_error_creation", |b| {
        b.iter(|| {
            let error = Yoshi::new(YoshiKind::Internal {
                message: black_box("Core error".into()),
                source: None,    // Required field
                component: None, // Required field
            });
            black_box(error);
        })
    });

    // Error conversion between facade and core
    // Note: FacadeYoshi and Yoshi are type aliases, so conversion is an identity operation.
    group.bench_function("facade_core_conversion", |b| {
        b.iter(|| {
            let facade_error = FacadeYoshi::new(FacadeYoshiKind::Internal {
                message: black_box("Conversion test".into()),
                source: None,    // Required field
                component: None, // Required field
            });

            // Convert to core and back (simulating cross-crate usage)
            // These are identity conversions due to type aliases.
            let core_error: Yoshi = black_box(facade_error);
            let back_to_facade: FacadeYoshi = black_box(core_error);
            black_box(back_to_facade);
        })
    });

    group.finish();
}

/// Benchmarks derive macro generated code performance
fn bench_derive_macro_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("derive_macro_performance");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(10000);

    // Derive macro error creation
    group.bench_function("derive_invalid_input", |b| {
        b.iter(|| {
            let error = TestError::InvalidInput {
                _input: black_box("test input".to_string()),
            };
            black_box(error);
        })
    });

    group.bench_function("derive_database_error", |b| {
        b.iter(|| {
            let error = TestError::DatabaseError {
                _operation: black_box("SELECT * FROM users".to_string()),
            };
            black_box(error);
        })
    });

    group.bench_function("derive_network_error", |b| {
        b.iter(|| {
            let error = TestError::NetworkError {
                url: black_box("https://api.example.com/users".to_string()),
            };
            black_box(error);
        })
    });

    // Conversion from derive macro errors to Yoshi
    group.bench_function("derive_to_yoshi_conversion", |b| {
        b.iter(|| {
            let test_error = TestError::InvalidInput {
                _input: black_box("conversion test".to_string()),
            };
            // The `YoshiError` derive macro implements `From<TestError> for Yoshi`
            let yoshi_error: Yoshi = black_box(test_error.into());
            black_box(yoshi_error);
        })
    });

    group.finish();
}

/// Benchmarks error propagation across crate boundaries
fn bench_cross_crate_propagation(c: &mut Criterion) {
    let mut group = c.benchmark_group("cross_crate_propagation");
    group.measurement_time(Duration::from_secs(10));

    // Test different propagation chain lengths
    for chain_length in [1, 3, 5, 10].iter() {
        group.throughput(Throughput::Elements(*chain_length as u64));

        group.bench_with_input(
            BenchmarkId::new("propagation_chain", chain_length),
            chain_length,
            |b, &chain_length| {
                b.iter(|| {
                    // Start with a derive macro error, converted to core Yoshi
                    let mut current_error: Yoshi = TestError::InvalidInput {
                        _input: black_box("initial error".to_string()),
                    }
                    .into();

                    // Propagate through multiple layers, simulating cross-crate boundaries
                    for i in 0..chain_length {
                        // Use Yoshi::context to add a new context message
                        current_error = current_error
                            .context(black_box(format!("layer_context_{}", i)))
                            // Subsequent `with_metadata` calls apply to the newly added context
                            .with_metadata(
                                black_box("operation".to_string()),
                                black_box(format!("layer_{}", i)),
                            )
                            .with_metadata(
                                black_box("component".to_string()),
                                black_box(format!("crate_{}", i)),
                            );

                        // Simulate conversion to facade and back
                        // No actual runtime conversion cost here as FacadeYoshi is Yoshi
                        let facade_error: FacadeYoshi = black_box(current_error.clone()); // Clone to allow current_error to be reused for next iteration
                        current_error = black_box(facade_error); // Assign back, still type alias
                    }
                    black_box(current_error);
                })
            },
        );
    }

    group.finish();
}

/// Benchmarks complete ecosystem integration scenarios
fn bench_ecosystem_integration(c: &mut Criterion) {
    let mut group = c.benchmark_group("ecosystem_integration");
    group.measurement_time(Duration::from_secs(15));

    // Complete error lifecycle: creation -> context -> conversion -> formatting
    group.bench_function("complete_error_lifecycle", |b| {
        b.iter(|| {
            // 1. Create error with derive macro
            let derive_error = TestError::DatabaseError {
                _operation: black_box("complex_query".to_string()),
            };

            // 2. Convert to core Yoshi
            let mut yoshi_error: Yoshi = black_box(derive_error.into());

            // 3. Add context and chain metadata/payloads to the *newly added* context
            yoshi_error = yoshi_error
                .context(black_box("Processing complex query".to_string()))
                .with_metadata(
                    black_box("service".to_string()),
                    black_box("user_service".to_string()),
                )
                .with_metadata(
                    black_box("subsystem".to_string()),
                    black_box("authentication".to_string()),
                )
                .with_payload(black_box(
                    json!({ // payload applies to the context just added
                        "user_id": 12345,
                        "timestamp": "2025-05-30T10:00:00Z"
                    }),
                ));

            // 4. Convert to facade (identity operation due to type alias)
            let facade_error: FacadeYoshi = black_box(yoshi_error.clone());

            // 5. Format for display
            let formatted = black_box(format!("{}", facade_error));
            let debug_formatted = black_box(format!("{:?}", facade_error));

            black_box((formatted, debug_formatted));
        })
    });

    // Concurrent ecosystem operations
    group.bench_function("concurrent_ecosystem_operations", |b| {
        use rayon::prelude::*;

        b.iter(|| {
            let results: Vec<_> = (0..100)
                .into_par_iter()
                .map(|i| {
                    // Create different error types in parallel
                    let error = match i % 3 {
                        0 => TestError::InvalidInput {
                            _input: format!("input_{}", i),
                        }
                        .into(),
                        1 => TestError::DatabaseError {
                            _operation: format!("operation_{}", i),
                        }
                        .into(),
                        _ => TestError::NetworkError {
                            url: format!("https://api.example.com/endpoint/{}", i),
                        }
                        .into(),
                    };

                    let yoshi_error: Yoshi = black_box(error);
                    let facade_error: FacadeYoshi = black_box(yoshi_error);
                    black_box(facade_error)
                })
                .collect();

            black_box(results);
        })
    });

    group.finish();
}

/// Benchmarks memory efficiency across crate boundaries
fn bench_cross_crate_memory_efficiency(c: &mut Criterion) {
    let mut group = c.benchmark_group("cross_crate_memory_efficiency");
    group.measurement_time(Duration::from_secs(10));

    // Memory allocation patterns for cross-crate operations
    group.bench_function("zero_copy_conversions", |b| {
        b.iter(|| {
            let yoshi_error = Yoshi::new(YoshiKind::Internal {
                message: black_box("Zero copy test".into()),
                source: None,    // Required field
                component: None, // Required field
            });

            // Test zero-copy conversion semantics (identity moves)
            let facade_error: FacadeYoshi = black_box(yoshi_error);
            let back_to_yoshi: Yoshi = black_box(facade_error);
            black_box(back_to_yoshi);
        })
    });

    // Large error structures across boundaries
    group.bench_function("large_error_cross_boundary", |b| {
        b.iter(|| {
            let large_payload = json!({
                "data": vec![0u8; 2048], // 2KB payload
                "metadata": (0..200).collect::<Vec<i32>>(),
                "timestamps": (0..50).map(|i| format!("2025-05-30T{}:00:00Z", i % 24)).collect::<Vec<_>>()
            });
            
            let mut yoshi_error = Yoshi::new(YoshiKind::Internal {
                message: black_box("Large error test".into()),
                source: None, // Required field
                component: None, // Required field
            });
            
            // Add a context message first to attach payload
            yoshi_error = yoshi_error.context(black_box("Payload context".to_string()))
                .with_payload(black_box(large_payload));
            
            // Cross-boundary transfer (identity moves)
            let facade_error: FacadeYoshi = black_box(yoshi_error);
            let back_to_yoshi: Yoshi = black_box(facade_error);
            black_box(back_to_yoshi);
        })
    });

    group.finish();
}

/// Benchmarks API compatibility and stability
fn bench_api_compatibility(c: &mut Criterion) {
    let mut group = c.benchmark_group("api_compatibility");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(10000);

    // Ensure API methods have consistent performance across crates
    group.bench_function("consistent_kind_access", |b| {
        let yoshi_error = Yoshi::new(YoshiKind::Internal {
            message: black_box("API test".into()),
            source: None,    // Required field
            component: None, // Required field
        });

        let facade_error: FacadeYoshi = yoshi_error; // Identity move

        b.iter(|| {
            let kind = black_box(facade_error.kind());
            black_box(kind);
        })
    });

    group.bench_function("consistent_context_access", |b| {
        let mut yoshi_error = Yoshi::new(YoshiKind::Internal {
            message: black_box("API test".into()),
            source: None,    // Required field
            component: None, // Required field
        });

        // Correctly add context and metadata
        yoshi_error = yoshi_error
            .context("test_operation_context".to_string())
            .with_metadata(
                black_box("operation".to_string()),
                black_box("test_operation".to_string()),
            );

        let facade_error: FacadeYoshi = yoshi_error; // Identity move

        b.iter(|| {
            let contexts: Vec<_> = black_box(facade_error.contexts().collect());
            black_box(contexts);
        })
    });

    group.finish();
}

criterion_group!(
    integration_benches,
    bench_facade_operations,
    bench_derive_macro_performance,
    bench_cross_crate_propagation,
    bench_ecosystem_integration,
    bench_cross_crate_memory_efficiency,
    bench_api_compatibility
);

criterion_main!(integration_benches);
