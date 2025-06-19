#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::doc_markdown)]
/* yoshi-benches\benches\error_context.rs */
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
//! **Brief:** Performance benchmarks for Yoshi error context operations with metadata and suggestion analysis.
//!
//! **Module Classification:** Performance-Critical
//! **Complexity Level:** Expert
//! **API Stability:** Stable
//!
//! ## Mathematical Properties
//!
//! **Algorithmic Complexity:**
//! - Time Complexity: O(1) for context addition, O(n) for context chain traversal
//! - Space Complexity: O(k) for k contexts + O(m) for m metadata entries per context
//! - Concurrency Safety: Thread-safe context manipulation with lock-free operations
//!
//! **Performance Characteristics:**
//! - Expected Performance: < 100ns for context addition with metadata
//! - Worst-Case Scenarios: < 500ns for complex context chains with suggestions
//! - Optimization Opportunities: Context pooling and metadata interning
//!
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Comprehensive Error Context Benchmarks with Metadata Analysis]
//!  - [Context Addition: O(1) with optimized context chain management]
//!  - [Metadata Attachment: O(1) per entry with efficient key-value storage]
//!  - [Suggestion Integration: O(1) with intelligent suggestion ranking]
//!  - [Context Traversal: O(n) for chain depth n with lazy evaluation]
//!  - [Shell Attachment: O(1) with type-erased payload optimization]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;
use yoshi::{warn, Duration, Yoshi, YoshiKind};

/// Sample data structure for context shell benchmarks
#[derive(Debug, Clone)]
#[allow(dead_code)] // Fields are used for shell attachment benchmarks
struct RequestContext {
    request_id: String,
    user_id: u64,
    endpoint: String,
    timestamp: u64,
}

impl RequestContext {
    fn new(request_id: &str, user_id: u64, endpoint: &str) -> Self {
        Self {
            request_id: request_id.to_string(),
            user_id,
            endpoint: endpoint.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_else(|_| std::time::Duration::from_secs(0))
                .as_secs(),
        }
    }
}

/// Benchmarks basic context addition operations
fn bench_context_addition(c: &mut Criterion) {
    let mut group = c.benchmark_group("context_addition");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(10000);

    let base_error = Yoshi::new(YoshiKind::Internal {
        message: "Database connection failed".into(),
        source: None,
        component: Some("database".into()),
    });

    group.bench_function("single_context", |b| {
        b.iter(|| {
            let error = black_box(&base_error)
                .clone()
                .context("Failed during user authentication");
            black_box(error);
        });
    });

    group.bench_function("context_with_string", |b| {
        b.iter(|| {
            let error = black_box(&base_error)
                .clone()
                .context(format!("Failed for user {}", black_box(12345)));
            black_box(error);
        });
    });

    group.finish();
}

/// Benchmarks metadata attachment operations
#[allow(clippy::cast_sign_loss)] // `metadata_count` is always positive from the array
fn bench_metadata_attachment(c: &mut Criterion) {
    let mut group = c.benchmark_group("metadata_attachment");
    group.measurement_time(Duration::from_secs(10));

    for metadata_count in &[1, 5, 10, 20] {
        group.throughput(Throughput::Elements(*metadata_count as u64));

        group.bench_with_input(
            BenchmarkId::new("metadata_entries", metadata_count),
            metadata_count,
            |b, &metadata_count| {
                b.iter(|| {
                    let mut error = Yoshi::new(YoshiKind::Network {
                        message: "API request failed".into(),
                        source: None,
                        error_code: Some(500),
                    })
                    .context("HTTP request processing");

                    for i in 0..metadata_count {
                        error = error
                            .with_metadata(format!("key_{i}"), format!("value_{i}"))
                            .with_metadata("timestamp", "2025-06-02T12:00:00Z")
                            .with_metadata("request_id", format!("req_{i}"));
                    }

                    black_box(error);
                });
            },
        );
    }

    group.finish();
}

/// Benchmarks suggestion attachment operations
#[allow(clippy::cast_sign_loss)] // `suggestion_count` is always positive from the array
fn bench_suggestion_attachment(c: &mut Criterion) {
    let mut group = c.benchmark_group("suggestion_attachment");
    group.measurement_time(Duration::from_secs(10));

    for suggestion_count in &[1, 3, 5, 10] {
        group.throughput(Throughput::Elements(*suggestion_count as u64));

        group.bench_with_input(
            BenchmarkId::new("suggestion_entries", suggestion_count),
            suggestion_count,
            |b, &suggestion_count| {
                b.iter(|| {
                    let mut error = Yoshi::new(YoshiKind::Validation {
                        field: "email".into(),
                        message: "Invalid email format".into(),
                        expected: Some("user@domain.com".into()),
                        actual: Some("invalid-email".into()),
                    })
                    .context("User input validation");

                    for i in 0..suggestion_count {
                        error =
                            error.with_signpost(format!("Try suggestion {i}: Check email format"));
                    }

                    black_box(error);
                });
            },
        );
    }

    group.finish();
}

/// Benchmarks shell attachment operations
fn bench_shell_attachment(c: &mut Criterion) {
    let mut group = c.benchmark_group("shell_attachment");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("simple_shell", |b| {
        b.iter(|| {
            let context = RequestContext::new("req_123", 12345, "/api/users");
            let error = Yoshi::new(YoshiKind::Timeout {
                operation: "User data retrieval".into(),
                duration: Duration::from_secs(30),
                expected_max: Some(Duration::from_secs(10)),
            })
            .context("Request processing timeout")
            .with_shell(black_box(context));

            black_box(error);
        });
    });

    group.bench_function("multiple_shells", |b| {
        b.iter(|| {
            let context1 = RequestContext::new("req_123", 12345, "/api/users");
            let context2 = vec![
                "rule1".to_string(),
                "rule2".to_string(),
                "rule3".to_string(),
            ];

            let error = Yoshi::new(YoshiKind::Internal {
                message: "Multiple context failure".into(),
                source: None,
                component: Some("multi_context".into()),
            })
            .context("First context layer")
            .with_shell(black_box(context1))
            .context("Second context layer")
            .with_shell(black_box(context2));

            black_box(error);
        });
    });

    group.finish();
}

/// Benchmarks context chain operations
#[allow(clippy::cast_sign_loss)] // `chain_depth` is always positive from the array
fn bench_context_chain_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("context_chain_operations");
    group.measurement_time(Duration::from_secs(10));

    for chain_depth in &[1, 3, 5, 10, 20] {
        group.throughput(Throughput::Elements(*chain_depth as u64));

        group.bench_with_input(
            BenchmarkId::new("chain_depth", chain_depth),
            chain_depth,
            |b, &chain_depth| {
                b.iter(|| {
                    let mut error = Yoshi::new(YoshiKind::Internal {
                        message: "Root error".into(),
                        source: None,
                        component: None,
                    });

                    for i in 0..chain_depth {
                        error = error
                            .context(format!("Context layer {i}"))
                            .with_metadata("layer", i.to_string())
                            .with_metadata("operation", format!("op_{i}"))
                            .with_signpost(format!("Try approach {i}"));
                    }

                    black_box(error);
                });
            },
        );
    }

    group.finish();
}

// Workaround for criterion_group! missing_docs warning
#[allow(missing_docs)]
mod criterion_benchmarks {
    use super::{
        bench_context_addition, bench_context_chain_operations, bench_metadata_attachment,
        bench_shell_attachment, bench_suggestion_attachment, criterion_group,
    };

    criterion_group!(
        context_benches,
        bench_context_addition,
        bench_metadata_attachment,
        bench_suggestion_attachment,
        bench_shell_attachment,
        bench_context_chain_operations
    );
}

pub use criterion_benchmarks::context_benches;

criterion_main!(context_benches);
