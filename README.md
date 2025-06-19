# Yoshi Error Handling Framework

![Yoshi Logo](assets/YoshiLogo.png)

[![Crates.io](https://img.shields.io/crates/v/yoshi.svg)](https://crates.io/crates/yoshi)
[![Docs.rs](https://docs.rs/yoshi/badge.svg)](https://docs.rs/yoshi)
[![Rust Version](https://img.shields.io/badge/rust-1.87%2B-blue.svg)](https://www.rust-lang.org)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

A structured error handling framework for Rust that tells you what went wrong, where it happened, and how to fix it.

## What is Yoshi?

Yoshi transforms cryptic error messages into actionable insights. Instead of generic "something broke" messages, you get structured errors with context, suggestions, and metadata. We've combined the ergonomics of `anyhow` with the type safety of `thiserror`, then added intelligent error categorization, auto-correction suggestions, and rich debugging context.

## Quick Start

```toml
[dependencies]
yoshi = "0.1"
```

```rust
use yoshi::*;

fn load_config(path: &str) -> Result<String> {
    std::fs::read_to_string(path).map_err(|e| yoshi!(
        YoshiKind::Io,
        "Failed to read config file",
        path: path,
        source: e,
        suggestion: "Check file permissions and path"
    ))
}

fn main() -> Result<()> {
    match load_config("/etc/app/config.toml") {
        Ok(config) => println!("Config: {}", config),
        Err(err) => {
            // Rich, formatted error output
            eprintln!("Error: {}", err);
            // With full context chain
            eprintln!("Context: {:#}", err);
            return Err(err);
        }
    }

    Ok(())
}
```

## Key Features

- **Expressive Macros** - Create rich errors with `yoshi!`, `bail!`, and `ensure!`
- **Smart Categories** - Organize errors with `YoshiKind` for consistent handling patterns
- **Context Chaining** - Build detailed error stories as they propagate through your code
- **Auto-Correction** - Get intelligent suggestions for fixing common issues
- **Derive Support** - Generate error types automatically with `#[derive(YoshiError)]`
- **Modular Architecture** - Use what you need, from no-std embedded to full-stack applications

## Architecture

### Framework Architecture Overview

![Yoshi Framework FlowMap](assets/yoshiFlowMap.svg)

*Interactive architecture diagram showing the complete Yoshi framework structure, dependencies, and API relationships.*

Yoshi is built with a modular architecture that lets you use what you need:

- **[yoshi-core](yoshi-core/)** - No-std foundation with essential error types
- **[yoshi-std](yoshi-std/)** - Standard library integration and convenience features
- **[yoshi-derive](yoshi-derive/)** - Procedural macros for generating error types
- **[yoshi-deluxe](yoshi-deluxe/)** - Advanced auto-correction and IDE integration
- **[yoshi](yoshi/)** - Unified facade that brings everything together

## Simple Error Creation

```rust
// Use the yoshi! macro for quick errors
let error = yoshi!(
    YoshiKind::Database,
    "Failed to connect to database",
    host: "db.example.com",
    port: 5432,
    retry_count: 3,
    suggestion: "Check database credentials and firewall settings"
);

// Or derive your own error types if you prefer
use yoshi::*;

#[derive(Debug, YoshiError)]
pub enum ApiError {
    #[yoshi(display = "User {user_id} not found")]
    UserNotFound { user_id: u64 },

    #[yoshi(display = "Request timed out after {seconds}s")]
    RequestTimeout { seconds: u64 },
}
```

## Testing

The Yoshi framework has comprehensive test coverage across all crates:

### Overall Test Statistics

- **528+ Total Tests** across all crates
- **136 Doc Tests** with working examples
- **392+ Unit & Integration Tests**
- **0 Ignored Tests** - every test validates real functionality
- **100% Test Pass Rate** with `cargo test --all --all-features`

### Test Coverage by Crate

- **yoshi-core:** 86 unit tests + 96 doc tests (no-std foundation)
- **yoshi-std:** 41 unit tests + 28 doc tests (std integration)
- **yoshi-derive:** 126 integration tests (macro functionality)
- **yoshi-deluxe:** 81 unit tests + 2 doc tests (auto-correction system)
- **yoshi:** 54 integration tests + 8 doc tests (facade crate)
- **yoshi-benches:** 28 unit tests + 2 doc tests (performance benchmarks)

### Running Tests

```bash
# Run all tests (recommended)
cargo test --all --all-features

# Run tests for specific crate
cargo test -p yoshi-core
cargo test -p yoshi-std
cargo test -p yoshi-derive

# Run only doc tests
cargo test --doc --all-features

# Run benchmarks
cargo bench
```

## Documentation & Examples

- [Introduction & Concepts](https://github.com/arcmoonstudios/yoshi/blob/main/docs/overview.md)
- [Macro Guide](https://github.com/arcmoonstudios/yoshi/blob/main/docs/macro.md)
- [Error Context & Metadata](https://github.com/arcmoonstudios/yoshi/blob/main/docs/context.md)
- [Performance Details](https://github.com/arcmoonstudios/yoshi/blob/main/docs/perf.md)
- [Migration Guide](https://github.com/arcmoonstudios/yoshi/blob/main/docs/migration.md)
- [API Docs](https://docs.rs/yoshi)
- [Examples](https://github.com/arcmoonstudios/yoshi/tree/main/yoshi/examples/)

## License

Licensed under either of Apache License, Version 2.0 or MIT License at your option.

---

Made by [ArcMoon Studios](https://github.com/arcmoonstudios)
