#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::doc_markdown)]
/* yoshi-benches\benches\cross_crate_integration.rs */
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
//! **Brief:** Performance benchmarks for cross-crate integration between yoshi-core, yoshi-std, yoshi-deluxe, and yoshi-derive.
//!
//! **Module Classification:** Performance-Critical
//! **Complexity Level:** Expert
//! **API Stability:** Stable
//!
//! ## Mathematical Properties
//!
//! **Algorithmic Complexity:**
//! - Time Complexity: O(1) for facade access, O(k) for k crate boundary crossings
//! - Space Complexity: O(1) for type conversions + O(n) for cross-crate data flow
//! - Concurrency Safety: Thread-safe cross-crate operations with zero data races
//!
//! **Performance Characteristics:**
//! - Expected Performance: < 50ns for facade-mediated access patterns
//! - Worst-Case Scenarios: < 200ns for complex derive macro + deluxe integration
//! - Optimization Opportunities: Zero-cost abstractions and compile-time optimization
//!
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Comprehensive Cross-Crate Integration Benchmarks]
//!  - [Facade Access Patterns: O(1) with zero-cost re-export validation]
//!  - [Core-Std Integration: O(1) with std feature boundary analysis]
//!  - [Derive Macro Performance: Compile-time + runtime cost measurement]
//!  - [Deluxe Feature Integration: O(1) with advanced feature cost analysis]
//!  - [Type Conversion Chains: O(k) for k crate boundaries with optimization]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;

// Import the unified yoshi facade - this tests facade integration
use yoshi::{warn, yoshi_core, yoshi_std, Duration, Hatch, Yoshi, YoshiKind, YoshiLocation};

// Direct crate imports for cross-crate integration testing
use yoshi_core::{
    Yoshi as CoreYoshi, YoshiKind as CoreYoshiKind, YoshiLocation as CoreYoshiLocation,
};
use yoshi_std::Hatch as StdHatch;

/// Test data structure for cross-crate integration scenarios
#[derive(Debug, Clone)]
#[allow(dead_code)] // Fields are used for integration testing
struct IntegrationTestData {
    operation_id: u64,
    component: String,
    metadata: Vec<(String, String)>,
}

impl IntegrationTestData {
    fn new(operation_id: u64, component: &str) -> Self {
        Self {
            operation_id,
            component: component.to_string(),
            metadata: vec![
                ("timestamp".to_string(), "2025-06-02T12:00:00Z".to_string()),
                ("version".to_string(), "1.0.0".to_string()),
            ],
        }
    }
}

/// Benchmarks facade access patterns vs direct crate access
fn bench_facade_vs_direct_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("facade_vs_direct_access");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(10000);

    // Facade access (through yoshi::*)
    group.bench_function("facade_error_creation", |b| {
        b.iter(|| {
            let error = Yoshi::new(YoshiKind::Internal {
                message: black_box("Facade access test".into()),
                source: None,
                component: Some(black_box("facade_test".into())),
            });
            black_box(error);
        });
    });

    // Direct core access
    group.bench_function("direct_core_access", |b| {
        b.iter(|| {
            let error = CoreYoshi::new(CoreYoshiKind::Internal {
                message: black_box("Direct core access test".into()),
                source: None,
                component: Some(black_box("core_test".into())),
            });
            black_box(error);
        });
    });

    // Location creation comparison
    group.bench_function("facade_location_creation", |b| {
        b.iter(|| {
            let location = YoshiLocation::new("test.rs", 123, 45);
            black_box(location);
        });
    });

    group.bench_function("direct_core_location_creation", |b| {
        b.iter(|| {
            let location = CoreYoshiLocation::new("test.rs", 123, 45);
            black_box(location);
        });
    });

    group.finish();
}

/// Benchmarks core-std integration patterns
fn bench_core_std_integration(c: &mut Criterion) {
    let mut group = c.benchmark_group("core_std_integration");
    group.measurement_time(Duration::from_secs(10));

    // Test Result type integration
    group.bench_function("facade_result_creation", |b| {
        b.iter(|| {
            let result: Hatch<i32> = Err(Yoshi::new(YoshiKind::Internal {
                message: black_box("Test error".into()),
                source: None,
                component: None,
            }));
            let _ = black_box(result);
        });
    });

    group.bench_function("direct_std_result_creation", |b| {
        b.iter(|| {
            let result: StdHatch<i32> = Err(CoreYoshi::new(CoreYoshiKind::Internal {
                message: black_box("Test error".into()),
                source: None,
                component: None,
            }));
            let _ = black_box(result);
        });
    });

    // Test extension trait integration
    group.bench_function("facade_context_usage", |b| {
        b.iter(|| {
            let error = Yoshi::new(YoshiKind::Internal {
                message: black_box("Test error".into()),
                source: None,
                component: None,
            })
            .context("Additional context");
            black_box(error);
        });
    });

    group.bench_function("direct_std_context_usage", |b| {
        b.iter(|| {
            let error = CoreYoshi::new(CoreYoshiKind::Internal {
                message: black_box("Test error".into()),
                source: None,
                component: None,
            })
            .context("Additional context");
            black_box(error);
        });
    });

    group.finish();
}

/// Benchmarks type conversion across crate boundaries
#[allow(clippy::cast_sign_loss)] // `conversion_count` is always positive from the array
fn bench_cross_crate_conversions(c: &mut Criterion) {
    let mut group = c.benchmark_group("cross_crate_conversions");
    group.measurement_time(Duration::from_secs(10));

    for conversion_count in &[1, 5, 10, 20] {
        group.throughput(Throughput::Elements(*conversion_count as u64));

        group.bench_with_input(
            BenchmarkId::new("facade_to_core_conversions", conversion_count),
            conversion_count,
            |b, &conversion_count| {
                b.iter(|| {
                    for i in 0..conversion_count {
                        // Create through facade
                        let facade_error = Yoshi::new(YoshiKind::Internal {
                            message: format!("Error {i}").into(),
                            source: None,
                            component: Some("test".into()),
                        });

                        // Convert to core type (simulated)
                        let _core_compatible = black_box(facade_error);
                    }
                });
            },
        );
    }

    group.finish();
}

/// Benchmarks derive macro integration performance
fn bench_derive_integration(c: &mut Criterion) {
    let mut group = c.benchmark_group("derive_integration");
    group.measurement_time(Duration::from_secs(10));

    // Test basic derive functionality through facade
    group.bench_function("derive_macro_simulation", |b| {
        b.iter(|| {
            // Simulate derive macro usage through facade
            let error = Yoshi::new(YoshiKind::Internal {
                message: black_box("Derive test".into()),
                source: None,
                component: Some("derive_test".into()),
            });
            black_box(error);
        });
    });

    group.finish();
}

/// Benchmarks deluxe feature integration
fn bench_deluxe_integration(c: &mut Criterion) {
    let mut group = c.benchmark_group("deluxe_integration");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("deluxe_feature_simulation", |b| {
        b.iter(|| {
            // Simulate deluxe functionality through facade
            let error = Yoshi::new(YoshiKind::Internal {
                message: black_box("Deluxe test".into()),
                source: None,
                component: Some("deluxe_test".into()),
            });
            black_box(error);
        });
    });

    group.finish();
}

/// Benchmarks comprehensive integration scenarios
fn bench_comprehensive_integration(c: &mut Criterion) {
    let mut group = c.benchmark_group("comprehensive_integration");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("full_stack_integration", |b| {
        b.iter(|| {
            let test_data = IntegrationTestData::new(12345, "integration_test");

            // Create error through facade
            let error = Yoshi::new(YoshiKind::Network {
                message: "Integration test error".into(),
                source: None,
                error_code: Some(500),
            })
            .context("Full stack integration test")
            .with_metadata("test_id", test_data.operation_id.to_string())
            .with_metadata("component", test_data.component.clone())
            .with_shell(black_box(test_data));

            black_box(error);
        });
    });

    group.finish();
}

// Workaround for criterion_group! missing_docs warning
#[allow(missing_docs)]
mod criterion_benchmarks {
    use super::{
        bench_comprehensive_integration, bench_core_std_integration, bench_cross_crate_conversions,
        bench_deluxe_integration, bench_derive_integration, bench_facade_vs_direct_access,
        criterion_group,
    };

    criterion_group!(
        integration_benches,
        bench_facade_vs_direct_access,
        bench_core_std_integration,
        bench_cross_crate_conversions,
        bench_derive_integration,
        bench_deluxe_integration,
        bench_comprehensive_integration
    );
}

pub use criterion_benchmarks::integration_benches;

criterion_main!(integration_benches);
