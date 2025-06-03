# yoshi-benches

![Yoshi Logo](../assets/YoshiLogo.png)

[![Crates.io](https://img.shields.io/crates/v/yoshi.svg)](https://crates.io/crates/yoshi)
[![Docs.rs](https://docs.rs/yoshi/badge.svg)](https://docs.rs/yoshi)
[![Rust Version](https://img.shields.io/badge/rust-1.87%2B-blue.svg)](https://www.rust-lang.org)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](../LICENSE)
[![CI](https://github.com/arcmoonstudios/yoshi/workflows/CI/badge.svg)](https://github.com/arcmoonstudios/yoshi/actions)
[![Coverage](https://codecov.io/gh/arcmoonstudios/yoshi/branch/main/graph/badge.svg)](https://codecov.io/gh/arcmoonstudios/yoshi)

**A comprehensive performance benchmarking suite for the Yoshi enterprise error handling framework, offering mathematical precision and empirical validation against industry-standard alternatives.**

## ArcMoon Studios Enterprise

> 🌙 ArcMoon Studios - Where precision meets innovation in **software reliability** and **error management** 🌙
>
> *Enterprise-grade Rust solutions for mission-critical applications*

---

## Overview

The `yoshi-benches` crate provides a meticulously crafted benchmarking suite, adhering to the **ArcMoon Studios Enterprise Development Framework** standards. Its primary purpose is to deliver mathematical precision in performance measurement and empirical validation of Yoshi's error handling framework's capabilities. This suite rigorously tests various aspects of error handling, including creation, context attachment, conversion, and formatting, providing a robust foundation for performance analysis and regression detection. It also includes comprehensive comparative analyses against leading Rust error handling libraries like `thiserror`, `anyhow`, and `eyre`, ensuring Yoshi's competitive advantage in high-performance, mission-critical environments.

### Key Areas of Benchmarking

+ **`error_creation`**: O(1) error instantiation with zero-cost abstractions.
+ **`error_context`**: O(1) context attachment with intelligent memory layout.
+ **`error_conversion`**: O(1) cross-framework conversion cost analysis.
+ **`error_formatting`**: O(n) display formatting with optimized string operations.
+ **`cross_crate_integration`**: O(1) inter-crate error boundary performance.
+ **`error_contest`**: Comprehensive comparative analysis against `thiserror`, `anyhow`, and `eyre`.

### Key Benchmarking Objectives

+ **Sub-microsecond Error Creation**: Validate O(1) error creation performance.
+ **Memory Efficiency**: Measure allocation patterns and optimize memory usage.
+ **Context Attachment**: Benchmark O(1) context addition operations.
+ **Cross-Framework Comparison**: Empirical validation against `thiserror`, `anyhow`, and `eyre`.
+ **Real-World Scenarios**: Performance testing under realistic application conditions.

## Architecture Classification

### Tool Performance Tier: High-Fidelity Measurement (Level 3 of 3)

+ **Time Complexity**: Benchmarks are designed to measure O(1) for core error creation and context operations, and O(L) for error chaining where L=chain depth. The suite itself aims for efficient execution of these measurements.
+ **Memory Overhead**: Benchmarks are designed to detect and report memory overhead of the measured operations. The suite itself strives for minimal impact on memory during measurement.
+ **Thread Safety**: Benchmarking suite supports thread-safe and parallel execution of tests, ensuring reliable measurement in concurrent environments.
+ **Allocation Strategy**: The suite focuses on measuring allocations induced by the target (Yoshi) library, utilizing Criterion.rs's capabilities to track precise memory usage.

### Target Audience

+ **Performance Engineers** - Validating and optimizing the performance characteristics of error handling.
+ **Systems Engineers** - Ensuring the error handling framework meets stringent performance requirements for critical infrastructure.
+ **Library Authors** - Benchmarking their own libraries against established standards and identifying performance bottlenecks.
+ **DevOps & CI/CD Specialists** - Integrating performance regression checks into automated pipelines.
+ **Enterprise Rust Teams** - Seeking empirical evidence for technology stack decisions and ensuring long-term performance stability.

## Installation

This crate is part of the Yoshi monorepo and primarily used for development and CI/CD. To run its benchmarks:

```toml
# No direct dependency needed in your application Cargo.toml.
# This crate is for development/benchmarking purposes of the Yoshi framework.
```

### Minimum Rust Version

`yoshi-benches` requires **Rust 1.70.0** or later due to advanced const generic usage and MSRV policy alignment with the Yoshi framework.

## Quick Start

### Running Benchmarks

```bash
# Navigate to the root of the yoshi monorepo
cd yoshi/yoshi-benches

# Run all benchmarks
cargo bench

# Run specific benchmark category
cargo bench --bench error_creation
cargo bench --bench error_context
cargo bench --bench error_contest

# Run benchmarks with detailed output
cargo bench -- --verbose

# Generate HTML reports
cargo bench -- --output-format html
```

### Performance Validation

```bash
# Run performance regression tests
cargo bench -- --test

# Compare against a named baseline (e.g., 'main')
cargo bench -- --baseline main

# Save current results as a new baseline (e.g., 'current')
cargo bench -- --save-baseline current
```

## Features

The `yoshi-benches` crate leverages Criterion.rs for robust benchmarking and provides specific capabilities for performance analysis. While it doesn't expose `Cargo.toml` features in the traditional sense for end-users, it offers distinct benchmark categories and configuration options.

### Benchmark Categories

| Benchmark | Description | Focus Area |
|-----------|-------------|------------|
| `error_creation` | Error instantiation performance | Creation overhead, memory allocation |
| `error_context` | Context attachment operations | Context chaining, metadata addition |
| `error_conversion` | Type conversion performance | Cross-framework compatibility |
| `error_formatting` | Display and debug formatting | String generation, template rendering |
| `cross_crate_integration` | Multi-crate error propagation | Integration overhead, cross-boundary costs |
| `error_contest` | Head-to-head performance comparison | Yoshi vs thiserror vs anyhow vs eyre |

### Core Features (Always Enabled)

+ **Statistical Analysis**: Utilizes Criterion.rs for robust statistical analysis of benchmark results, including confidence intervals and outlier detection.
+ **Regression Detection**: Built-in support for comparing current benchmark runs against historical baselines to detect performance regressions.
+ **Detailed Reporting**: Generates comprehensive HTML reports with interactive graphs and data visualizations.
+ **Memory Profiling**: Capabilities to measure and report memory allocations for analyzed operations.

### Benchmark Configuration

The `yoshi-benches` suite can be configured using environment variables supported by Criterion.rs, and potentially via a custom `benches.toml` for specific parameters.

```bash
# Set benchmark duration for more stable results (default is 5s)
export CRITERION_MEASUREMENT_TIME="10"

# Set the number of samples for each benchmark (default is 100)
export CRITERION_SAMPLE_SIZE="1000"

# Enable detailed output during benchmark execution
export CRITERION_VERBOSE="true"

# Example: Custom benchmark parameters (not directly used by yoshi-benches by default)
# You can define a `benches.toml` for custom thresholds or targets.
# [benchmark.error_creation]
# sample_size = 1000
# measurement_time = 5
# warm_up_time = 1
```

## Performance Characteristics

`yoshi-benches` is dedicated to rigorously measuring the performance of the Yoshi framework and its comparisons. The following provides insight into the *targets* and *measurement capabilities* of the suite.

### Benchmark Targets (Illustrative)

These are the performance targets that the `yoshi-benches` suite is designed to validate for the Yoshi framework's core operations:

|        Operations         | Latency (ns) | Throughput (ops/sec) | Memory (bytes) |
|---------------------------|--------------|----------------------|----------------|
| Error Creation (simple)   |     <100     |     >10,000,000      |     <128       |
| Error Creation (context)  |     <500     |     >2,000,000       |     <256       |
| Error Traversal (1-lvl)   |     <15      |     >50,000,000      |      0         |
| Context Addition (O(1))   |     <50      |     >20,000,000      |     <64        |
| Cross-framework conversion|     <100     |     >10,000,000      |     Zero-copy  |

*Note: Benchmarks are run on various hardware and configurations, results are illustrative and depend heavily on environment.*

### Complexity Analysis

+ **Time Complexity (Measured Operations)**: The benchmarks are designed to validate O(1) for core `Yoshi` operations like creation and context attachment. O(L) for operations involving error chains of length L.
+ **Space Complexity (Measured Operations)**: Benchmarks validate O(1) constant memory overhead for basic `Yoshi` instances, and O(N) linear space for N context key-value pairs.
+ **Concurrency (Benchmarking Suite)**: The suite itself is designed for thread-safe and parallel execution of benchmarks, ensuring accurate measurements under various load conditions.

### Performance Tuning

The `yoshi-benches` suite is a tool for performance tuning. Users can:

+ **Analyze Benchmark Results**: Review detailed HTML reports to identify performance bottlenecks and regressions.
+ **Profile Memory Usage**: Utilize built-in memory profiling capabilities to understand allocation patterns.
+ **Detect Regressions**: Integrate into CI/CD pipelines to automatically detect performance degradations against baselines.

```bash
# View detailed benchmark results in terminal
cargo bench 2>&1 | tee "benchmark_results.txt"

# Access HTML reports for detailed analysis
# Open target/criterion/report/index.html in your browser

# Profile memory usage during benchmarks (if specific features enabled in yoshi-std)
# cargo bench --features "yoshi-std/memory-profiling"

# Generate memory reports
# cargo bench -- --memory-report # (Requires specific Criterion.rs features/setup)
```

## Documentation

### Primary Resources

+ **[API Documentation][api-docs]** - API reference for the `yoshi-benches` suite itself.
+ **[Criterion.rs Documentation](https://docs.rs/criterion)** - Reference for the underlying benchmarking framework.

### Additional Resources

+ **[Performance Guide][performance-guide]** - Overview of Yoshi's performance optimization strategies.
+ **[Benchmark Analysis][benchmark-analysis]** - Guide on interpreting benchmark results and conducting performance analysis.
+ **[Examples][examples]** - Real-world usage patterns and best practices.
+ **[Contributing Guide][contributing]** - Details on adding new benchmarks.

## Examples

### Running Specific Benchmarks

```bash
# Run only the error creation benchmarks
cargo bench --bench error_creation

# Run the comparative 'error_contest' benchmark against other frameworks
cargo bench --bench error_contest
```

### Advanced Reporting & Analysis

```bash
# Generate HTML reports for all benchmarks
cargo bench

# Export benchmark data to CSV for external analysis
cargo bench -- --output-format csv > benchmark_data.csv

# Integrate with continuous benchmarking for regression detection
# Example: Using 'main' branch as baseline
cargo bench -- --baseline main --threshold 0.05 # Fail if regression > 5%
```

### Memory Profiling & Leak Detection

```bash
# (Requires specific compile-time features for the measured library, e.g., yoshi-std)
# cargo bench --features "yoshi-std/leak-detection"

# Analyze memory patterns (tool-specific, e.g., Valgrind on Linux)
# valgrind --tool=massif --pages-as-heap=yes cargo bench --bench error_creation
```

## Testing

`yoshi-benches` primarily serves as a testing tool itself for performance.

### Performance Regression Tests

The suite includes mechanisms to assert performance targets and detect regressions:

```bash
# Run built-in performance validation tests (if defined in benches)
cargo test --release --features "performance-tests"

# Validate against pre-defined performance targets (e.g., in a JSON file)
cargo bench -- --test --target-file "performance_targets.json"
```

### Continuous Integration Integration

Integrating `yoshi-benches` into CI/CD pipelines is crucial for maintaining performance standards.

```yaml
# Example GitHub Actions workflow snippet
- name: Run Performance Benchmarks
  run: |
    cd yoshi/yoshi-benches
    cargo bench --verbose
    # Output results to a JSON file for processing by external tools
    cargo bench -- --output-format json > benchmark_results.json

- name: Performance Regression Check
  run: |
    cd yoshi/yoshi-benches
    # Fail the build if performance degrades by more than 5% compared to 'main'
    cargo bench -- --baseline main --threshold 0.05
```

## Troubleshooting

### Common Issues

1.**Slow Benchmark Execution**

```powershell
    # Reduce sample size for faster execution
    cargo bench -- --sample-size 100
```

2.**Memory Usage Spikes**

```powershell
    # Enable memory monitoring
    cargo bench --features "memory-monitoring"
```

3.**Inconsistent Results**

```powershell
    # Increase warm-up time
    cargo bench -- --warm-up-time 5
```

### Performance Analysis Tools

```powershell
# Profile benchmark execution
cargo bench --features "profiling"

# Generate flame graphs
cargo bench -- --profile --output-format svg
```

## Contributing

We welcome contributions to `yoshi-benches`! Please read our [Contributing Guide][contributing] for details on:

+ Development environment setup
+ Code style and formatting requirements
+ Testing and benchmarking standards
+ Pull request process
+ Issue reporting guidelines

### Adding New Benchmarks

1. Create a new benchmark file in `benches/`
2. Add benchmark configuration to `Cargo.toml`
3. Follow ArcMoon Studios benchmarking standards
4. Include mathematical complexity analysis
5. Add performance targets and validation

### Benchmark Standards

+ **Criterion.rs**: All benchmarks must use the `criterion` framework for statistical analysis and robust measurement.
+ **Mathematical Complexity**: Each benchmark should explicitly document the expected algorithmic time and space complexity of the measured operation.
+ **Performance Targets**: Where applicable, define clear performance targets and thresholds for measured operations.
+ **Scenario Coverage**: Test across multiple scenarios, input sizes, and configurations to ensure comprehensive coverage.
+ **Memory Usage Profiling**: Benchmarks should be designed to allow for memory allocation tracking and profiling.

## License

This project is licensed under either of

+ Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
+ MIT License ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### License Summary for `yoshi-benches`

+ ✅ **Full Commercial Freedom**: Complete freedom for commercial use, production deployment, and enterprise integration
+ ✅ **Performance Analysis**: Unrestricted use for validating performance, benchmarking, and identifying regressions
+ ✅ **Production Use**: Full production deployment rights under dual MIT/Apache 2.0 licensing
+ 🌟 **Open Source**: Complete open source freedom with standard Rust ecosystem licensing
+ 💼 **Enterprise Ready**: No licensing restrictions for enterprise deployment and integration

See the [LICENSE](../LICENSE) file for complete terms and conditions.

---

<!-- Link References -->
[api-docs]: https://docs.rs/yoshi-benches
[performance-guide]: ../docs/performance.md
[benchmark-analysis]: ../docs/benchmark-analysis.md
[examples]: ../examples/
[contributing]: ../CONTRIBUTING.md

<!-- Footer -->
🌙 ArcMoon Studios - Where precision meets innovation in error handling technology 🌙

*Building the future of reliable software, one error at a time.*
