/* yoshi-benches\benches\error_formatting.rs */
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
//! **Brief:** Performance benchmarks for Yoshi error formatting operations with display optimization.
//!
//! **Module Classification:** Performance-Critical
//! **Complexity Level:** Expert
//! **API Stability:** Stable
//!
//! ## Mathematical Properties
//!
//! **Algorithmic Complexity:**
//! - Time Complexity: O(n) for n characters in formatted output
//! - Space Complexity: O(n) for output buffer + O(k) for k contexts
//! - Concurrency Safety: Lock-free formatting with thread-safe string interning
//!
//! **Performance Characteristics:**
//! - Expected Performance: < 500ns for basic error formatting
//! - Worst-Case Scenarios: < 2μs for complex errors with full context chains
//! - Optimization Opportunities: Buffer pooling and pre-allocated formatters
//!
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Comprehensive Error Formatting Benchmarks with Buffer Analysis]
//!  - [Basic Display Formatting: O(n) with optimized string building]
//!  - [Debug Formatting: O(n+k) with context and metadata inclusion]
//!  - [JSON Serialization: O(n) with structured output optimization]
//!  - [Custom Formatter: Variable complexity with user-defined templates]
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
use std::hint::black_box;
use std::time::Duration;
use yoshi_std::{Yoshi, YoshiKind};

/// Benchmarks basic error formatting operations
fn bench_basic_formatting(c: &mut Criterion) {
    let mut group = c.benchmark_group("basic_formatting");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(5000);

    let error = Yoshi::new(YoshiKind::Internal {
        message: "Critical system failure in authentication module".into(),
        source: None,
        component: Some("auth_service".into()),
    });

    group.bench_function("display_format", |b| {
        b.iter(|| {
            let formatted = format!("{}", black_box(&error));
            black_box(formatted);
        });
    });

    group.bench_function("debug_format", |b| {
        b.iter(|| {
            let formatted = format!("{:?}", black_box(&error));
            black_box(formatted);
        });
    });

    group.bench_function("to_string", |b| {
        b.iter(|| {
            let formatted = black_box(&error).to_string();
            black_box(formatted);
        });
    });

    group.finish();
}

/// Benchmarks formatting with varying context counts
#[allow(clippy::cast_sign_loss)] // `context_count` is always positive from the array
fn bench_formatting_with_contexts(c: &mut Criterion) {
    let mut group = c.benchmark_group("formatting_with_contexts");
    group.measurement_time(Duration::from_secs(10));

    for context_count in &[1, 3, 5, 10, 20] {
        group.throughput(Throughput::Elements(*context_count as u64));

        // Corrected YoshiKind::Network fields
        let mut error = Yoshi::new(YoshiKind::Network {
            message: "HTTP GET request to user service failed".into(), // Message is required
            source: None,
            error_code: Some(500), // Map status_code to error_code
        });

        // Add original network details as metadata to the initial context
        error = error
            .with_metadata("operation", "HTTP GET request to user service") // Use &str for metadata
            .with_metadata("endpoint", "https://api.example.com/users/12345") // Use &str for metadata
            .with_metadata("retry_after_secs", "30"); // Use &str for metadata

        for i in 0..*context_count {
            // Use Yoshi::context() to add new contexts, then chain metadata/suggestion
            error = error
                .context(format!("Context layer {i}").to_string()) // Direct format argument
                .with_metadata("layer", i.to_string())
                .with_metadata("timestamp", "2025-05-30T12:00:00Z")
                .with_metadata("request_id", format!("req_{i}").to_string()) // Direct format argument
                .with_suggestion(format!("Try approach {i} for resolution").to_string());
            // Direct format argument
        }

        group.bench_with_input(
            BenchmarkId::new("display_with_contexts", context_count),
            &error,
            |b, error| {
                b.iter(|| {
                    let formatted = format!("{}", black_box(error));
                    black_box(formatted);
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("debug_with_contexts", context_count),
            &error,
            |b, error| {
                b.iter(|| {
                    let formatted = format!("{:?}", black_box(error));
                    black_box(formatted);
                });
            },
        );
    }

    group.finish();
}

/// Benchmarks formatting with different error chain depths
#[allow(clippy::cast_sign_loss)] // `chain_depth` is always positive from the array
fn bench_error_chain_formatting(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_chain_formatting");
    group.measurement_time(Duration::from_secs(10));

    for chain_depth in &[1, 3, 5, 10] {
        group.throughput(Throughput::Elements(*chain_depth as u64));

        // Start with the innermost error, and build the chain outwards.
        // `current_outermost_error` will hold the top-level error in the chain.
        let mut current_outermost_error: Yoshi = Yoshi::new(YoshiKind::Internal {
            message: "Deepest error in chain".into(),
            source: None,
            component: Some("deepest_component".into()),
        });

        for i in 1..=*chain_depth {
            // Loop from 1 up to chain_depth to build layers
            // The previous 'current_outermost_error' becomes the source of the new one
            current_outermost_error = Yoshi::new(YoshiKind::Internal {
                message: format!("Wrapping error at level {i}").into(), // Direct format argument
                source: Some(Box::new(current_outermost_error)),        // Correctly set the source
                component: Some(format!("component_level_{i}").into()), // Direct format argument
            })
            // Now apply context and metadata to this new error using public methods
            .context(format!("Context for level {i}").to_string()) // Direct format argument
            .with_metadata("level_idx", i.to_string())
            .with_metadata("source_chain_info", "nested Yoshi error"); // Use &str for metadata
        }
        group.bench_with_input(
            BenchmarkId::new("format_error_chain", chain_depth),
            &current_outermost_error,
            |b, error| {
                b.iter(|| {
                    let formatted = format!("{}", black_box(error));
                    black_box(formatted);
                });
            },
        );
    }

    group.finish();
}

/// Benchmarks concurrent formatting operations
fn bench_concurrent_formatting(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_formatting");
    group.measurement_time(Duration::from_secs(15));

    let error = Yoshi::new(YoshiKind::Timeout {
        operation: "Database query with complex joins".into(),
        duration: Duration::from_secs(45),
        expected_max: Some(Duration::from_secs(30)),
    })
    // Add context, then chain metadata and suggestion
    .context("Query execution timeout in production database".to_string())
    .with_metadata("database", "primary") // Use &str for metadata
    .with_metadata("query_complexity", "high") // Use &str for metadata
    .with_metadata("table_count", "7") // Use &str for metadata
    .with_suggestion("Consider query optimization or database scaling"); // Use &str for suggestion

    group.bench_function("concurrent_display_formatting", |b| {
        b.iter(|| {
            use rayon::prelude::*;

            let results: Vec<String> = (0..100)
                .into_par_iter()
                .map(|_| format!("{}", black_box(&error)))
                .collect();

            black_box(results);
        });
    });

    group.bench_function("concurrent_debug_formatting", |b| {
        b.iter(|| {
            use rayon::prelude::*;

            let results: Vec<String> = (0..100)
                .into_par_iter()
                .map(|_| format!("{:?}", black_box(&error)))
                .collect();

            black_box(results);
        });
    });

    group.finish();
}

/// Benchmarks memory allocation during formatting
#[allow(clippy::cast_sign_loss)] // `message_size` is always positive from the array
fn bench_formatting_memory_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("formatting_memory_allocation");
    group.measurement_time(Duration::from_secs(10));

    // Create errors of varying sizes to test allocation patterns
    for message_size in &[10, 100, 1000, 10000] {
        let large_message = "X".repeat(*message_size);
        let error = Yoshi::new(YoshiKind::Internal {
            message: large_message.into(),
            source: None,
            component: Some("memory_test".into()),
        });

        group.throughput(Throughput::Bytes(*message_size as u64));

        group.bench_with_input(
            BenchmarkId::new("format_large_message", message_size),
            &error,
            |b, error| {
                b.iter(|| {
                    let formatted = format!("{}", black_box(error));
                    black_box(formatted);
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    formatting_benches,
    bench_basic_formatting,
    bench_formatting_with_contexts,
    bench_error_chain_formatting,
    bench_concurrent_formatting,
    bench_formatting_memory_allocation
);

criterion_main!(formatting_benches);
