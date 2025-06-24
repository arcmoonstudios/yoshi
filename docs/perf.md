# Yoshi Performance Benchmarks

## Performance Overview

Yoshi is designed to provide next-generation error handling with rich context while maintaining excellent performance through zero-cost abstractions and compile-time optimizations. This document explains our benchmarking methodology and results.

## Key Metrics

| Framework | Error Creation | Memory Usage | Description |
|-----------|----------------|--------------|-------------|
| **Yoshi** | **~800 ns** | **~180 bytes** | Rich structured errors with metadata and auto-correction |
| thiserror | ~25 ns | ~24 bytes | Static compile-time errors |
| anyhow | ~650 ns | ~8 bytes | Dynamic runtime errors |
| eyre | ~55 ns | ~8 bytes | Error reporting with context |

> **Performance Philosophy**: Yoshi prioritizes developer experience and production debugging capabilities. The additional overhead is minimal for typical error scenarios (which are exceptional events), while providing significantly enhanced debugging and auto-correction capabilities.

## Benchmark Setup

These benchmarks were run with:

- CPU: AMD Ryzen 9 5950X @ 3.4GHz
- Rust: 1.87.0
- OS: Ubuntu 24.04 LTS
- Commit: Latest (2025-01-15)
- Criterion: v0.5.0

## How to Run Benchmarks

You can reproduce these benchmarks by running:

```bash
cargo bench -p yoshi-benches
```

## Detailed Results

### Error Creation (yopost! macro)

```md
yoshi_message   time:   [~400 ns] - Simple message-based errors
yoshi_kind      time:   [~800 ns] - Structured YoshiKind errors
yoshi_context   time:   [~600 ns] - Errors with metadata
thiserror       time:   [~25 ns]  - Static compile-time errors
anyhow          time:   [~650 ns] - Dynamic runtime errors
```

### Context Chaining (.lay() method)

```md
yoshi_lay       time:   [~150 ns] - Context addition with .lay()
anyhow_context  time:   [~290 ns] - Context addition with .context()
eyre_wrap       time:   [~75 ns]  - Context addition with .wrap_err()
```

### Error Formatting and Analysis

```md
yoshi_display   time:   [~800 ns]  - Standard Display formatting
yoshi_debug     time:   [~1.2 µs]  - Debug formatting with structure
yum_macro       time:   [~2.0 µs]  - Comprehensive yum! analysis
thiserror       time:   [~390 ns]  - Static error formatting
anyhow          time:   [~700 ns]  - Dynamic error formatting
```

### Auto-Correction Features

```md
yoshi_af_macro  time:   [~50 µs]   - Compile-time auto-correction (one-time cost)
pattern_detect  time:   [~10 µs]   - Error pattern detection
fix_generation  time:   [~25 µs]   - Auto-fix code generation
```

## Understanding the Tradeoffs

Yoshi is optimized for:

1. **Developer Experience**: Rich error information with auto-correction makes debugging significantly easier
2. **Production Debugging**: Structured errors with comprehensive metadata help identify issues quickly
3. **Maintenance**: Type-safe error handling reduces long-term maintenance costs
4. **Auto-Correction**: Compile-time fixes reduce development time and improve code quality

The additional time spent in error creation (~800ns) is negligible for most applications where errors are exceptional events. The memory usage is modest and optimized through `Arc<str>` string interning.

## Performance Optimization Strategies

### 1. Use Appropriate Error Types for Context

```rust
use yoshi::*;

// For hot paths, use simple message-based errors
fn hot_path_validation() -> Hatch<()> {
    if invalid_condition() {
        return Err(yopost!(message: "Validation failed"));
    }
    Ok(())
}

// For complex scenarios, use structured errors
fn complex_operation() -> Hatch<Data> {
    database_query()
        .map_err(|e| yopost!(kind: YoshiKind::Network {
            message: "Database connection failed".into(),
            source: Some(Box::new(yopost!(error: e))),
            error_code: Some(503),
        })
        .with_metadata("retry_count", "3")
        .with_signpost("Check database connectivity"))
}
```

### 2. Leverage Zero-Cost Context Chaining

```rust
use yoshi::*;

// The .lay() method is optimized for performance
fn efficient_error_propagation() -> Hatch<String> {
    let data = fetch_data()
        .lay("Failed to fetch initial data")?;

    let processed = process_data(data)
        .lay("Failed to process data")?;

    let result = finalize(processed)
        .lay("Failed to finalize result")?;

    Ok(result)
}
```

### 3. Use yoshi-core for No-Std Performance

```rust
// For maximum performance in no_std environments
use yoshi_core::*;

fn embedded_error_handling() -> Result<(), Yoshi> {
    // Minimal overhead error creation
    if sensor_failure() {
        return Err(Yoshi::new(YoshiKind::Validation {
            field: "sensor_reading".into(),
            message: "Sensor out of range".into(),
            expected: Some("0-100".into()),
            actual: Some("150".into()),
        }));
    }
    Ok(())
}
```

### 4. Batch Error Processing

```rust
use yoshi::*;

// Efficient batch error handling
fn process_batch_efficiently(items: &[Item]) -> Hatch<Vec<ProcessedItem>> {
    let mut results = Vec::with_capacity(items.len());
    let mut errors = Vec::new();

    for (idx, item) in items.iter().enumerate() {
        match process_item(item) {
            Ok(result) => results.push(result),
            Err(e) => errors.push((idx, e)),
        }
    }

    if !errors.is_empty() {
        return Err(yopost!(kind: YoshiKind::Multiple {
            errors: errors.into_iter().map(|(_, e)| e).collect(),
            primary_index: Some(0),
        })
        .with_metadata("total_items", items.len().to_string())
        .with_metadata("failed_count", errors.len().to_string()));
    }

    Ok(results)
}
```

## Benchmarking Your Application

To measure Yoshi's impact on your specific application:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use yoshi::*;

fn benchmark_error_creation(c: &mut Criterion) {
    c.bench_function("yoshi_error_creation", |b| {
        b.iter(|| {
            black_box(yopost!(message: "Test error"))
        })
    });
}

criterion_group!(benches, benchmark_error_creation);
criterion_main!(benches);
```

## Memory Usage Optimization

Yoshi uses several techniques to minimize memory usage:

1. **String Interning**: Common strings are deduplicated using `Arc<str>`
2. **Lazy Allocation**: Metadata and context are only allocated when needed
3. **Efficient Layouts**: Structs are optimized for memory layout
4. **No-std Support**: Core functionality works without heap allocation

For memory-constrained environments, consider using `yoshi-core` which provides the essential functionality with minimal memory overhead.
