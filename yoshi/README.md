# yoshi

![Yoshi Logo](../assets/YoshiLogo.png)

[![Crates.io](https://img.shields.io/crates/v/yoshi.svg)](https://crates.io/crates/yoshi)
[![Docs.rs](https://docs.rs/yoshi/badge.svg)](https://docs.rs/yoshi)
[![Rust Version](https://img.shields.io/badge/rust-1.87%2B-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-BSL--1.1-blue)](../LICENSE)
[![CI](https://github.com/arcmoonstudios/yoshi/workflows/CI/badge.svg)](https://github.com/arcmoonstudios/yoshi/actions)
[![Coverage](https://codecov.io/gh/arcmoonstudios/yoshi/branch/main/graph/badge.svg)](https://codecov.io/gh/arcmoonstudios/yoshi)

**A unified facade crate for the Yoshi enterprise error handling framework, offering zero-cost abstractions and compile-time optimizations for robust and scalable error management.**

## ArcMoon Studios Enterprise

> 🌙 ArcMoon Studios - Where precision meets innovation in **software reliability** and **error management** 🌙
>
> *Enterprise-grade Rust solutions for mission-critical applications*

---

## Overview

The `yoshi` crate serves as the primary entry point for the comprehensive Yoshi error handling framework, providing a unified and ergonomic interface that re-exports functionality from its modular underlying implementation crates (`yoshi-std`, `yoshi-derive`, `yoshi-benches`). This facade design ensures easy adoption and consistent API usage while maintaining granular control over dependencies and optimal performance characteristics. Yoshi empowers developers to define, categorize, and manage errors with high fidelity, offering contextual information, robust tracing integration, and derive macros for streamlined code generation. Its design prioritizes minimal runtime overhead, making it suitable for high-performance, mission-critical applications where reliable error propagation and actionable diagnostics are paramount.

### Key Features

+ **Unified API**: A single crate import (`yoshi`) provides seamless access to the entire Yoshi framework, simplifying dependency management and enhancing developer productivity.
+ **Zero-Cost Abstractions**: Leveraging Rust's powerful type system, `yoshi` offers compile-time validated error constructs that incur no runtime performance overhead beyond the underlying operations (O(1) for error creation and context attachment).
+ **Memory Safety Guarantees**: Built entirely in safe Rust, `yoshi` inherently provides memory safety guarantees, minimizing the risk of common error-related vulnerabilities. Formal verification status: Pending.
+ **Ecosystem Compatibility**: Designed for seamless integration with the broader Rust ecosystem, including `tokio` for async operations, `serde` for serialization, and `tracing` for structured logging, promoting zero-cost abstraction.
+ **Ergonomic API Design**: Intuitive builder patterns and fluent APIs enable highly readable and maintainable error definitions, with compile-time checks catching potential misconfigurations early.
+ **Telemetry Integration**: First-class integration with `tracing` and `opentelemetry` for comprehensive observability, allowing detailed error context to be automatically captured and propagated to monitoring systems with minimal (typically <1%) runtime overhead.
+ **Scalability & Production Readiness**: Architected for high concurrency and low latency, `yoshi` provides building blocks for scalable error handling in distributed systems, with battle-tested patterns for resilient production deployments.
+ **Extensible Error Definitions**: Leveraging procedural macros, `yoshi` allows developers to easily define custom error types that automatically conform to the framework's interfaces, supporting plugin-based error extensions and customization.

### Architecture Classification

#### Performance Tier: High-Performance (Level 3 of 3)

+ **Time Complexity**: O(1) for core operations (error creation, kind assignment, context attachment). O(L) for traversing an error chain of length L.
+ **Memory Overhead**: Constant with less than 0.1% baseline impact for core error types. Linear (O(N)) only when explicitly storing N context fields.
+ **Thread Safety**: Lock-free concurrency model for core types; `Arc` for shared error contexts where necessary, ensuring thread-safe operations with minimal contention.
+ **Allocation Strategy**: Minimal allocations. Primary error types are stack-allocated when possible, with heap allocations reserved for dynamic string data and boxed traits, reducing memory fragmentation.

### Target Audience

+ **Systems Engineers** - Building high-performance infrastructure components requiring robust error propagation.
+ **Backend Engineers** - Developing scalable web services and APIs where precise error handling and diagnostics are critical.
+ **Embedded Developers** - Creating resource-constrained applications benefiting from `yoshi`'s minimal overhead and `no-std` compatibility (via `yoshi-std`).
+ **DevOps & Observability Engineers** - Implementing monitoring and telemetry solutions that leverage structured error data.
+ **Enterprise Rust Teams** - Delivering mission-critical business applications demanding consistent, reliable, and auditable error management.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
yoshi = "0.1.0"

# Recommended: Enable performance optimizations and telemetry
yoshi = { version = "0.1.0", features = ["derive", "serde", "tracing"] }

# Enterprise: Full feature set for comprehensive production deployments
yoshi = { version = "0.1.0", features = ["full"] }
```

### Minimum Rust Version

`yoshi` requires **Rust 1.70.0** or later due to advanced const generic usage and MSRV policy alignment with the Yoshi framework.

## Quick Start

### Basic Usage

```rust
use yoshi::{Yoshi, YoshiKind}; // Re-exported from yoshi-std

fn main() -> Result<(), Yoshi> {
    // Create errors using the unified API
    let error = Yoshi::new(YoshiKind::NotFound, "Resource not found");

    // With context
    let error_with_context = Yoshi::new(YoshiKind::Validation, "Invalid input")
        .with_context("field", "email")
        .with_context("value", "invalid@example.com");

    // Use in Result types
    if some_condition_failed() {
        return Err(Yoshi::new(YoshiKind::Internal, "Operation failed"));
    }

    println!("Operation successful!");
    Ok(())
}

fn some_condition_failed() -> bool {
    true // Simulate failure
}
```

### Using Derive Macros

```rust
use yoshi::{Yoshi, YoshiDerive, YoshiKind, ErrorContext}; // Re-exported from yoshi-std & yoshi-derive
use std::time::Duration;

#[yoshi::yoshi_derive] // Attribute macro from yoshi-derive
#[derive(Debug)] // Required for printing
enum AppError {
    #[yoshi(kind = "Validation", display = "Invalid input for field '{field}'")]
    InvalidInput { field: String },

    #[yoshi(kind = "Network", display = "Connection to {host}:{port} failed")]
    ConnectionFailed { host: String, port: u16 },

    #[yoshi(kind = "Internal", display = "Database error: {source}")]
    DatabaseError {
        #[source] // Marks the source error
        source: sqlx::Error,
        #[yoshi(context = "query_id")] // Adds context to the Yoshi error
        query: String,
    },
}

#[tokio::main]
async fn enterprise_operation() -> Result<(), AppError> {
    // Generated errors automatically implement Into<Yoshi>
    let error: Yoshi = AppError::InvalidInput {
        field: "email".to_string()
    }.into();

    tracing::error!("Generated error: {:?}", error);

    // Simulating a database operation
    let db_result: Result<(), sqlx::Error> = Err(sqlx::Error::PoolClosed); // Example SQLx error

    if let Err(sql_err) = db_result {
        return Err(AppError::DatabaseError {
            source: sql_err,
            query: "SELECT * FROM users WHERE id = 1;".to_string(),
        });
    }

    Ok(())
}
```

## Re-exported Crates

The `yoshi` facade provides unified access to:

+ **`yoshi-std`** - Core error types (`Yoshi`, `YoshiKind`, `YoContext`)
+ **`yoshi-derive`** - Procedural macros (`YoshiDerive`, attribute macros)
+ **`yoshi-benches`** - Performance benchmarking suite (optional, feature-gated)

## Features

Configure `yoshi` with optional features:

```toml
[dependencies]
yoshi = { version = "0.1.0", features = ["derive", "serde", "tracing"] }
```

### Available Features

| Feature | Description | Dependencies | Impact |
|---------|-------------|--------------|--------|
| `std` | Standard library support | None | Core functionality, larger binary |
| `derive` | Re-exports procedural macros from `yoshi-derive` | `yoshi-derive` | +Compile time, ergonomic error definitions |
| `serde` | Enables `serde` for serialization/deserialization of `Yoshi` errors | `serde` | +10% binary size (approx), enables JSON/binary support |
| `tracing` | Integrates with the `tracing` ecosystem for structured logging and span association | `tracing`, `tracing-log` | +5% runtime overhead for enabled traces, enhanced observability |
| `full` | Enables all non-mutually exclusive features | All above | Complete functionality, maximal binary size |

### Core Features (Always Enabled)

+ **Zero-cost abstractions** with compile-time optimization for error structure.
+ **Memory safety guarantees** through Rust's type system, eliminating common error-related vulnerabilities.
+ **Thread-safe operations** for error creation and context handling.
+ **Comprehensive error handling** with contextual information, error kind, and source chains.
+ **Performance monitoring hooks** for observability integration (when `tracing` is enabled).
+ **Modular architecture** supporting granular feature control via sub-crates.

### Feature Combinations

#### Recommended Profiles

```toml
# High-performance server applications with full observability
yoshi = { version = "0.1.0", features = ["derive", "serde", "tracing"] }

# Embedded systems (requires `yoshi-std` directly for `no-std` if not using `yoshi` facade)
# yoshi-std = { version = "0.1.0", default-features = false } # Directly use yoshi-std

# Enterprise production deployment (all features for comprehensive capabilities)
yoshi = { version = "0.1.0", features = ["full"] }
```

## Performance Characteristics

`yoshi` itself is a facade; its performance characteristics directly reflect those of its underlying highly optimized `yoshi-std` core.

### Benchmark Results (from `yoshi-benches` - indicative for core operations)

```text
        Operations         | Latency (ns) | Throughput (ops/sec) | Memory (bytes)
---------------------------|--------------|----------------------|----------------
Error Creation (simple)    |     15-20    |      50,000,000      |    ~40-80
Error Creation (context)   |    50-100    |      15,000,000      |    ~120-200
Error Traversal (1-lvl)    |     10-15    |      60,000,000      |      0
Error Serialization (JSON) |   150-300    |       3,000,000      |    ~500
```

*Note: Benchmarks are illustrative and depend heavily on hardware, data size, and specific features enabled.*

### Complexity Analysis

+ **Time Complexity**: O(1) for core operations like `Yoshi::new()` and `with_context()`. O(L) for traversing an error chain of length L.
+ **Space Complexity**: O(1) constant memory overhead for basic `Yoshi` instances. O(N) linear space for N context key-value pairs stored within an error.
+ **Concurrency**: Lock-free approach for core `Yoshi` structure. `Arc<ErrorContext>` used for shared, immutable contexts, ensuring thread-safe read access with minimal contention.
+ **Memory Access**: Designed with cache locality in mind. Core error types are small and often stack-allocated, minimizing cache misses.

### Performance Tuning

While `yoshi` itself doesn't expose direct performance tuning knobs (as it's a facade), its underlying `yoshi-std` is highly optimized. Key strategies for performance include:

```rust
// General Rust performance best practices applicable to Yoshi:
// 1. Minimize dynamic allocations: Prefer stack-allocated data where possible.
// 2. Leverage smart pointers (`Arc`, `Box`) judiciously for shared or trait objects.
// 3. Ensure appropriate data structures for context (e.g., smallvec for few contexts).
// 4. Use 'release' profiles for production builds:
//    cargo build --release
// 5. Consider LTO and codegen units for final optimization:
//    [profile.release]
//    lto = "fat"
//    codegen-units = 1
```

## Documentation

### Primary Resources

+ **[API Documentation][api-docs]** - Complete API reference for the `yoshi` facade.
+ **[Main Framework Documentation][main-docs]** - Overview of the entire Yoshi framework and its design principles.
+ **[Core Types (`yoshi-std`)][std-docs]** - In-depth documentation for fundamental error types and their operations.
+ **[Derive Macros (`yoshi-derive`)][derive-docs]** - Details on procedural macros for automatic error definition.
+ **[Performance Benchmarks (`yoshi-benches`)][bench-docs]** - Information on the benchmarking suite and performance analysis.
+ **[Examples][examples]** - Real-world usage patterns and best practices.

### Additional Resources

+ **[Migration Guide][migration]** - Upgrading from previous versions or other error libraries.
+ **[Troubleshooting][troubleshooting]** - Common issues and solutions.
+ **[Contributing Guide][contributing]** - Development and contribution workflow.
+ **[Security Policy][security]** - Vulnerability reporting and security practices.

## Examples

### Production Use Cases

```rust
// High-throughput web service with tracing and structured errors
use yoshi::{Yoshi, YoshiKind, YoshiDerive};
use tracing::{error, instrument}; // Using tracing for observability

#[yoshi::yoshi_derive]
#[derive(Debug)]
enum WebServiceError {
    #[yoshi(kind = "NotFound", display = "Resource '{path}' not found")]
    ResourceNotFound { path: String },
    #[yoshi(kind = "Internal", display = "Service temporarily unavailable")]
    ServiceUnavailable,
}

#[instrument(skip_all, fields(request_id = %request_id))]
async fn handle_request(request_id: &str, path: &str) -> Result<String, WebServiceError> {
    if path == "/non_existent" {
        error!("Attempted to access non-existent path: {}", path);
        return Err(WebServiceError::ResourceNotFound { path: path.to_string() });
    }
    // Simulate some logic that might fail
    if rand::random::<f32>() < 0.1 {
        error!("Simulated service unavailability for request {}", request_id);
        return Err(WebServiceError::ServiceUnavailable);
    }
    Ok(format!("Content for {}", path))
}

#[tokio::main]
async fn main_web_server_example() {
    tracing_subscriber::fmt::init(); // Initialize tracing subscriber

    let request_id = "abc-123";
    match handle_request(request_id, "/non_existent").await {
        Ok(data) => println!("Success: {}", data),
        Err(e) => {
            // Yoshi errors implement std::error::Error and Display
            eprintln!("Request failed: {}", e);
            // Access underlying Yoshi properties
            eprintln!("  Kind: {:?}", e.kind());
            if let Some(ctx) = e.context() {
                eprintln!("  Context: {:?}", ctx);
            }
        }
    }
}
```

Find comprehensive examples in the [`examples/`][examples] directory covering:

+ Integration with popular web frameworks (Axum, Actix, Warp)
+ Async processing pipelines with backpressure and error propagation
+ Embedded systems with resource constraints (via `yoshi-std` directly)
+ Enterprise monitoring and observability with `tracing` and `opentelemetry`
+ Custom error handling strategies and recovery flows
+ Usage of derive macros for complex error structures

## Error Handling

`yoshi` provides a robust and comprehensive error handling system with rich contextual information:

```rust
use yoshi::{Yoshi, YoshiKind, ErrorContext};

fn process_data(input: &str) -> Result<String, Yoshi> {
    if input.is_empty() {
        return Err(Yoshi::new(YoshiKind::Validation, "Input cannot be empty")
            .with_context("input_type", "string"));
    }
    if input.len() > 100 {
        return Err(Yoshi::new(YoshiKind::InvalidInput, "Input too long")
            .with_context("max_length", 100)
            .with_context("actual_length", input.len()));
    }
    // Simulate an internal error
    if input == "fail" {
        let internal_error = std::io::Error::new(std::io::ErrorKind::Other, "disk full");
        return Err(Yoshi::new(YoshiKind::Internal, "Failed to write data")
            .with_source(internal_error)
            .with_context("operation", "file_write"));
    }

    Ok(format!("Processed: {}", input.to_uppercase()))
}

fn main() {
    match process_data("") {
        Ok(result) => println!("Success: {}", result),
        Err(e) => {
            eprintln!("Error processing data: {}", e);
            eprintln!("  Kind: {:?}", e.kind());
            if let Some(ctx) = e.context() {
                eprintln!("  Context: {:?}", ctx);
            }
            if let Some(source) = e.source() {
                eprintln!("  Source Error: {}", source);
            }
        }
    }
}
```

## Testing

Run the comprehensive test suite:

```bash
# Unit tests for the yoshi facade crate
cargo test

# Integration tests with all features enabled (recommended for CI)
cargo test --all-features

# Performance benchmarks (requires --features benches and nightly Rust)
# cargo +nightly bench --features benches

# Fuzzing (requires cargo-fuzz, targeting yoshi-std)
# cd yoshi-std && cargo fuzz run fuzz_target_1

# Memory safety validation (requires valgrind and release build)
# cargo test --release && valgrind target/release/deps/yoshi_std-*
```

## Contributing

We welcome contributions to `yoshi`! Please read our [Contributing Guide][contributing] for details on:

+ Development environment setup
+ Code style and formatting requirements
+ Testing and benchmarking standards
+ Pull request process
+ Issue reporting guidelines

### Development Setup

```bash
# Clone the repository
git clone https://github.com/arcmoonstudios/yoshi.git
cd yoshi/yoshi # Navigate to the yoshi facade crate directory

# Install development dependencies
rustup component add clippy rustfmt
cargo install cargo-audit cargo-outdated

# Run development checks
cargo check --all-features
cargo clippy --all-features -- -D warnings
cargo fmt --check
cargo audit
```

### Code Quality Standards

+ **Test Coverage**: Minimum 90% line coverage required for core logic.
+ **Performance**: No regressions in benchmark suite for core `yoshi-std` operations.
+ **Documentation**: All public APIs must have comprehensive rustdoc with examples.
+ **Safety**: No `unsafe` code without explicit justification, thorough review, and accompanying safety comments.

## Changelog

See [CHANGELOG.md][changelog] for detailed release notes and migration information.

## Security

Security is paramount for enterprise applications. See our [Security Policy][security] for:

+ Vulnerability reporting process
+ Security best practices
+ Supported versions and update policy
+ Security audit results

## License

This project is licensed under the **Business Source License 1.1 (BSL-1.1)**.

### License Summary

+ **✅ Development and Testing**: Free for non-production use, research, and evaluation.
+ **⚠️ Production Use**: Requires a commercial license for production deployments.
+ **📅 License Change**: Automatically converts to GPL v3 on **2025-05-25**.
+ **💼 Commercial Licensing**: Contact [LordXyn@proton.me](mailto:LordXyn@proton.me) for enterprise licensing.

### Key Permissions

+ ✅ Private use and modification
+ ✅ Distribution with license preservation
+ ✅ Patent use (with limitations)
+ ❌ Commercial production use without license

See the [LICENSE][license] file for complete terms and conditions.

---

<!-- Link References -->
[api-docs]: https://docs.rs/yoshi
[main-docs]: ../README.md
[std-docs]: ../yoshi-std/README.md
[derive-docs]: ../yoshi-derive/README.md
[bench-docs]: ../yoshi-benches/README.md
[examples]: ../examples/
[migration]: ../docs/migration.md
[troubleshooting]: ../docs/troubleshooting.md
[contributing]: ../CONTRIBUTING.md
[security]: ../SECURITY.md
[changelog]: ../CHANGELOG.md
[license]: ../LICENSE

<!-- Footer -->
🌙 ArcMoon Studios - Where precision meets innovation in error handling technology 🌙

*Building the future of reliable software, one error at a time.*
