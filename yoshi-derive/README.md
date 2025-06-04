# yoshi-derive

![Yoshi Logo](../assets/YoshiLogo.png)

[![Crates.io](https://img.shields.io/crates/v/yoshi-derive.svg)](https://crates.io/crates/yoshi-derive)
[![Docs.rs](https://docs.rs/yoshi-derive/badge.svg)](https://docs.rs/yoshi-derive)
[![Rust Version](https://img.shields.io/badge/rust-1.87%2B-blue.svg)](https://www.rust-lang.org)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](../LICENSE)

Derive macros for automatically generating Yoshi error types. Because writing error boilerplate is boring.

## What's this?

Generates `std::error::Error` implementations, `Display`, and conversion to `Yoshi` types automatically.

## Installation

```toml
[dependencies]
yoshi-derive = "0.1"
yoshi = "0.1"
```

## Basic Usage

```rust
use yoshi_derive::YoshiError;

#[derive(Debug, YoshiError)]
pub enum MyError {
    #[yoshi(display = "User {user_id} not found")]
    #[yoshi(kind = "NotFound")]
    UserNotFound { user_id: u32 },

    #[yoshi(display = "Failed to parse config: {source}")]
    ParseError {
        #[yoshi(source)]
        source: std::io::Error,
        #[yoshi(context = "config_file")]
        path: String,
    },
}
```

## Attributes

### Container Attributes (`#[yoshi(...)]` on enums)

| Attribute | Description | Example |
|-----------|-------------|---------|
| `error_code_prefix` | Prefix for error codes | `#[yoshi(error_code_prefix = "HTTP")]` |
| `default_severity` | Default severity (0-255) | `#[yoshi(default_severity = 75)]` |

### Variant Attributes (`#[yoshi(...)]` on enum variants)

| Attribute | Description | Example |
|-----------|-------------|---------|
| `display` | Custom display format | `#[yoshi(display = "Error: {message}")]` |
| `kind` | Map to YoshiKind | `#[yoshi(kind = "Network")]` |
| `error_code` | Unique error code | `#[yoshi(error_code = 1001)]` |
| `severity` | Severity level | `#[yoshi(severity = 80)]` |
| `transient` | Mark as retryable | `#[yoshi(transient = true)]` |
| `suggestion` | Recovery suggestion | `#[yoshi(suggestion = "Check network")]` |

### Field Attributes (`#[yoshi(...)]` on struct fields)

| Attribute | Description | Example |
|-----------|-------------|---------|
| `source` | Mark as error source | `#[yoshi(source)]` |
| `context` | Add to context metadata | `#[yoshi(context = "file_path")]` |
| `shell` | Add as typed shell | `#[yoshi(shell)]` |
| `skip` | Skip in Display | `#[yoshi(skip)]` |

## Advanced Example

```rust
use yoshi_derive::YoshiError;

#[derive(Debug, YoshiError)]
#[yoshi(error_code_prefix = "DB")]
#[yoshi(default_severity = 75)]
pub enum DatabaseError {
    #[yoshi(error_code = 1001)]
    #[yoshi(display = "Connection to {host}:{port} failed")]
    #[yoshi(kind = "Network")]
    #[yoshi(severity = 120)]
    #[yoshi(transient = true)]
    ConnectionFailed {
        host: String,
        port: u16,
        #[yoshi(source)]
        cause: std::io::Error,
        #[yoshi(context = "connection_timeout")]
        timeout: std::time::Duration,
    },

    #[yoshi(error_code = 2001)]
    #[yoshi(display = "Query failed: {query}")]
    #[yoshi(kind = "Internal")]
    QueryFailed {
        query: String,
        #[yoshi(shell)]
        execution_stats: QueryStats,
    },
}

#[derive(Debug)]
struct QueryStats {
    duration_ms: u64,
    rows_affected: usize,
}
```

## Generated Code

The derive macro automatically creates:

- `std::fmt::Display` implementation
- `std::error::Error` implementation
- `From<YourError> for yoshi_std::Yoshi` conversion
- Error code and severity methods

## Smart Inference

The macro automatically infers attributes based on naming:

- `timeout`, `expired` → `kind = "Timeout"`
- `network`, `connection` → `kind = "Network"`
- `not_found`, `missing` → `kind = "NotFound"`
- `std::io::Error` fields → `source = true`

## Performance

- **Compilation**: <100ms for typical enums (<50 variants)
- **Runtime**: Zero overhead - generates efficient code
- **Memory**: Uses static strings where possible

## License

Licensed under either of Apache License, Version 2.0 or MIT License at your option.
