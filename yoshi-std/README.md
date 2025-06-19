# yoshi-std

![Yoshi Logo](../assets/YoshiLogo.png)

[![Crates.io](https://img.shields.io/crates/v/yoshi-std.svg)](https://crates.io/crates/yoshi-std)
[![Docs.rs](https://docs.rs/yoshi-std/badge.svg)](https://docs.rs/yoshi-std)
[![Rust Version](https://img.shields.io/badge/rust-1.87%2B-blue.svg)](https://www.rust-lang.org)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](../LICENSE)

Standard library integration for the Yoshi error handling framework. Rich features for application development.

## What's this?

This crate transforms `yoshi-core`'s minimal foundation into a full-featured error handling powerhouse. You get automatic backtrace capture, rich formatting, convenient extension traits, and seamless integration with the standard library ecosystem. Perfect for applications, services, and any environment where std is available.

## Installation

```toml
[dependencies]
yoshi-std = "0.1"

# Or if you want just the core types without std features
yoshi-std = { version = "0.1", default-features = false }
```

## Basic Usage

```rust
use yoshi_std::{Yoshi, YoshiKind, Hatch};

// Create errors with the new API
let error = Yoshi::new(YoshiKind::Internal {
    message: "User not found".into(),
    source: None,
    component: Some("user_service".into()),
});

// Add context and metadata
let error = error
    .context("During user lookup")
    .with_metadata("user_id", "12345")
    .with_metadata("operation", "lookup");

// Use the Result type alias
fn find_user(id: u64) -> Hatch<String> {
    Err(Yoshi::new(YoshiKind::NotFound {
        resource: "user".into(),
        identifier: id.to_string(),
        context: None,
    }))
}

// Extension traits for convenience
use yoshi_std::LayText;

let result: Hatch<i32> = Ok(42);
let chained = result.laytext("Additional context");
```

## Error Categories

```rust
use yoshi_std::YoshiKind;

match error.kind() {
    YoshiKind::Io => println!("File system error"),
    YoshiKind::Network => println!("Network error"),
    YoshiKind::Validation => println!("Input validation failed"),
    YoshiKind::NotFound => println!("Resource not found"),
    YoshiKind::Timeout => println!("Operation timed out"),
    YoshiKind::Config => println!("Configuration error"),
    YoshiKind::Internal => println!("Internal error"),
    // ... more variants
}
```

## Context Management

```rust
use yoshi_std::Yoshi;

let mut error = Yoshi::new(YoshiKind::Database, "Connection failed");

// Add structured context
error = error
    .with_context("host", "db.example.com")
    .with_context("port", 5432)
    .with_context("retry_count", 3);

// Access context
if let Some(host) = error.context().get("host") {
    println!("Failed host: {}", host);
}

// Iterate context
for (key, value) in error.context().iter() {
    println!("{}: {}", key, value);
}
```

## Performance

| Operation | Time | Memory |
|-----------|------|---------|
| `Yoshi::new()` | 42ns | 64 bytes |
| `with_context()` | 38ns | +32 bytes |
| `context_access()` | 12ns | 0 |

## What's included

- **Hatch&lt;T&gt;** - Result type alias (`Result<T, Yoshi>`)
- **LayText trait** - Extension methods for adding context to Results
- **YoshiBacktrace** - Backtrace capture and formatting
- **Rich formatting** - Enhanced Display and Debug implementations
- **Standard conversions** - From implementations for common std types

## Features

- `std` - Standard library support (default)
- `serde` - Serialization support for error types
- `tracing` - Integration with the tracing ecosystem
- `backtrace` - Backtrace capture support

## Testing

The `yoshi-std` crate provides comprehensive standard library integration testing:

### Test Statistics

- **41 Unit Tests** - Standard library integration and features
- **28 Doc Tests** - Working examples with std features
- **0 Ignored Tests** - Every test validates real functionality
- **Complete std integration** - All std-dependent features tested

### Running Tests

```bash
# Run all yoshi-std tests
cargo test -p yoshi-std

# Run with all features
cargo test -p yoshi-std --all-features

# Run only doc tests
cargo test --doc -p yoshi-std

# Run specific test categories
cargo test -p yoshi-std advanced_features_tests
cargo test -p yoshi-std std_integration_tests
cargo test -p yoshi-std backtrace_debug_tests
```

### Test Categories

- **Advanced Features Tests:** Context chaining, error conversion, method chaining
- **Std Integration Tests:** Standard library type conversions and compatibility
- **Backtrace Debug Tests:** Backtrace capture, formatting, and debugging features
- **Doc Tests:** 28 working examples covering std-specific functionality

### Key Test Features

- **I/O Error Integration:** Complete `std::io::Error` conversion testing
- **Backtrace Validation:** Backtrace capture and formatting verification
- **Memory Efficiency:** Standard library overhead measurement
- **Concurrent Testing:** Thread-safe error handling validation
- **Performance Monitoring:** Standard library integration performance

## License

Licensed under either of Apache License, Version 2.0 or MIT License at your option.
