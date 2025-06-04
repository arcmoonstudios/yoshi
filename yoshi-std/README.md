# yoshi-std

![Yoshi Logo](../assets/YoshiLogo.png)

[![Crates.io](https://img.shields.io/crates/v/yoshi-std.svg)](https://crates.io/crates/yoshi-std)
[![Docs.rs](https://docs.rs/yoshi-std/badge.svg)](https://docs.rs/yoshi-std)
[![Rust Version](https://img.shields.io/badge/rust-1.87%2B-blue.svg)](https://www.rust-lang.org)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

Core error types for the Yoshi framework. Use this directly if you want minimal dependencies.

## What's this?

The foundation of Yoshi - provides `Yoshi`, `YoshiKind`, and `YoContext` types. Everything else is built on top of this.

## Installation

```toml
[dependencies]
yoshi-std = "0.1"

# For no_std environments

yoshi-std = { version = "0.1", default-features = false }
```

## Basic Usage

```rust
use yoshi_std::{Yoshi, YoshiKind};

// Create errors
let error = Yoshi::new(YoshiKind::NotFound, "User not found");

// Add context
let error = error
    .with_context("user_id", "12345")
    .with_context("operation", "lookup");

// Chain errors
let io_error = std::fs::read("missing.txt").unwrap_err();
let yoshi_error = Yoshi::from_error(YoshiKind::Io, io_error)
    .with_context("file", "missing.txt");
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

## Features

- `std` - Standard library support (default)
- `serde` - Serialization support
- `tracing` - Tracing integration
- `proptest` - Property testing utilities

## License

Licensed under either of Apache License, Version 2.0 or MIT License at your option.
