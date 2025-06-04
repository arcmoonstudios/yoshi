# yoshi-benches

![Yoshi Logo](../assets/YoshiLogo.png)

[![Crates.io](https://img.shields.io/crates/v/yoshi.svg)](https://crates.io/crates/yoshi)
[![Docs.rs](https://docs.rs/yoshi/badge.svg)](https://docs.rs/yoshi)
[![Rust Version](https://img.shields.io/badge/rust-1.87%2B-blue.svg)](https://www.rust-lang.org)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](../LICENSE)

Performance benchmarks for the Yoshi error handling framework. Numbers don't lie.

## What's this?

Comprehensive benchmarking suite comparing Yoshi against `thiserror`, `anyhow`, `eyre`, and `snafu`. Validates that Yoshi's extra features don't kill performance.

## Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench error_creation
cargo bench --bench error_context
cargo bench --bench error_contest

# Generate HTML reports
cargo bench -- --output-format html
```

## Benchmark Categories

| Benchmark | What it measures |
|-----------|------------------|
| `error_creation` | How fast errors are created |
| `error_context` | Context attachment performance |
| `error_conversion` | Type conversion overhead |
| `error_formatting` | Display/Debug formatting speed |
| `error_contest` | Head-to-head vs other frameworks |

## Current Results

| Framework | Error Creation | Memory Usage |
|-----------|---------------|--------------|
| **Yoshi** | **1201 ns** | **208 bytes** |
| thiserror | 22 ns | 24 bytes |
| anyhow | 629 ns | 8 bytes |
| eyre | 51 ns | 8 bytes |

*Yoshi is slower but provides much richer error information**

## Performance Targets

- Error creation: <1μs
- Context addition: <50ns
- Memory usage: <256 bytes per error
- Zero allocations for basic errors

## Environment Variables

```bash
# Longer measurement time for stable results
export CRITERION_MEASUREMENT_TIME="10"

# More samples
export CRITERION_SAMPLE_SIZE="1000"

# Verbose output
export CRITERION_VERBOSE="true"
```

## CI Integration

```yaml
- name: Run Benchmarks
  run: |
    cd yoshi-benches
    cargo bench --verbose
    cargo bench -- --output-format json > results.json

- name: Check for regressions
  run: |
    cargo bench -- --baseline main --threshold 0.05
```

## Adding Benchmarks

1. Create new file in `benches/`
2. Use `criterion` for statistical analysis
3. Document expected complexity (O(1), O(n), etc.)
4. Add performance targets
5. Test multiple scenarios

## License

Licensed under either of Apache License, Version 2.0 or MIT License at your option.
