# Yoshi Performance Benchmarks

## Performance Overview

Yoshi is designed to provide rich error information while maintaining reasonable performance. This document explains our benchmarking methodology and results.

## Key Metrics

| Framework | Error Creation | Memory Usage | Description |
|-----------|---------------|--------------|-------------|
| **Yoshi** | **1201 ns** | **208 bytes** | Rich context, metadata, and typed errors |
| thiserror | 22 ns | 24 bytes | Static compile-time errors |
| anyhow | 629 ns | 8 bytes | Dynamic runtime errors |
| eyre | 51 ns | 8 bytes | Error reporting with context |

> **Important Performance Context**: Yoshi's error creation is slower than alternatives because it captures rich metadata. However, for most applications, error creation is a rare event (typically <100/second), making the extra microsecond negligible compared to the benefits of rich error information.

## Benchmark Setup

These benchmarks were run with:

- CPU: AMD Ryzen 9 5950X @ 3.4GHz
- Rust: 1.87.0
- OS: Ubuntu 24.04 LTS
- Commit: `4e3a91f8` (2025-05-15)
- Criterion: v0.5.0

## How to Run Benchmarks

You can reproduce these benchmarks by running:

```bash
cargo bench -p yoshi-benches
```

## Detailed Results

### Error Creation

```md
yoshi           time:   [1.1954 µs 1.2014 µs 1.2085 µs]
thiserror       time:   [21.981 ns 22.024 ns 22.075 ns]
anyhow          time:   [629.32 ns 629.83 ns 630.42 ns]
eyre            time:   [50.938 ns 51.013 ns 51.096 ns]
```

### Context Addition

```md
yoshi           time:   [215.31 ns 215.78 ns 216.33 ns]
anyhow          time:   [290.20 ns 290.65 ns 291.19 ns]
eyre            time:   [73.265 ns 73.334 ns 73.418 ns]
```

### Error Formatting

```md
yoshi           time:   [1.5224 µs 1.5283 µs 1.5354 µs]
thiserror       time:   [389.83 ns 390.49 ns 391.34 ns]
anyhow          time:   [704.65 ns 706.23 ns 708.09 ns]
eyre            time:   [731.47 ns 733.10 ns 734.96 ns]
```

## Understanding the Tradeoffs

Yoshi is optimized for:

1. **Developer Experience**: Rich error information makes debugging easier
2. **Maintenance**: Structured errors allow for better error handling and reporting
3. **Production Troubleshooting**: Detailed errors help identify issues in production

The additional time spent in error creation (roughly 1µs) is negligible for most applications where errors are exceptional events. The memory usage is higher but still modest at 208 bytes per error.

For extremely performance-critical sections where you need to create errors in hot paths thousands of times per second, consider using `thiserror` directly.

## Optimizing Error Handling

If you're concerned about performance in a specific section:

```rust
use yoshi::*;

// In hot loops, you can defer error creation
fn hot_path_function() -> Result<(), YoshiKind> {
    for _ in 0..1000000 {
        // Use simple error kind if in hot path
        if something_failed() {
            return Err(YoshiKind::Validation);
        }
    }
    Ok(())
}

// Then convert to rich errors at the boundary
fn public_api_function() -> Result<()> {
    hot_path_function().map_err(|kind| {
        // Create rich error only once at the boundary
        yoshi!(kind, "Validation failed in hot path")
    })
}
```
