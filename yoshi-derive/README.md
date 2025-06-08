# yoshi-derive

[![Crates.io](https://img.shields.io/crates/v/yoshi-derive.svg)](https://crates.io/crates/yoshi-derive)
[![Docs.rs](https://docs.rs/yoshi-derive/badge.svg)](https://docs.rs/yoshi-derive)
[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](../LICENSE)

A procedural macro for deriving error types that integrate with the Yoshi error handling framework.

## Overview

This crate provides `#[derive(YoshiError)]` to automatically generate `Display`, `Error`, and Yoshi conversion implementations for your error enums. It includes auto-inference capabilities to reduce boilerplate and enhance developer productivity.

## Installation

```toml
[dependencies]
yoshi-derive = "0.1"
yoshi-std = "0.1"
```

## Basic Usage

```rust
use yoshi_derive::YoshiError;

#[derive(Debug, YoshiError)]
pub enum AppError {
    #[yoshi(display = "User {user_id} not found")]
    UserNotFound { user_id: u32 },

    #[yoshi(display = "IO operation failed: {source}")]
    IoError {
        #[yoshi(source)]
        source: std::io::Error,
    },
}
```

## Available Attributes

### Container-level (`#[yoshi(...)]` on enums)

| Attribute | Description |
|-----------|-------------|
| `default_severity` | Default severity level (0-255) |
| `default_kind` | Default error kind |
| `auto_inference` | Enable automatic attribute inference |
| `generate_helpers` | Generate helper methods |

### Variant-level (`#[yoshi(...)]` on variants)

| Attribute | Description |
|-----------|-------------|
| `display` | Custom display format string |
| `kind` | Error classification |
| `severity` | Severity level (0-255) |
| `suggestion` | User-facing suggestion |
| `transient` | Mark as retryable |
| `from` | Generate From implementation |
| `code` | Unique error code |

### Field-level (`#[yoshi(...)]` on fields)

| Attribute | Description |
|-----------|-------------|
| `source` | Mark as error source |
| `context` | Include in metadata |
| `skip` | Skip in display |
| `sensitive` | Redact in output |

## Example with Inference

```rust
use yoshi_derive::YoshiError;

#[derive(Debug, YoshiError)]
pub enum NetworkError {
    // Automatically infers: kind = "Timeout", transient = true
    ConnectionTimeout,

    // Automatically detects std::io::Error as source
    IoError(std::io::Error),

    // Custom attributes override inference
    #[yoshi(severity = 200, suggestion = "Check API key")]
    AuthenticationFailed { key: String },
}
```

## Auto-inference Features

When enabled, the macro attempts to infer appropriate attributes based on:

- **Variant names**: `timeout` → `transient`, `not_found` → `kind = "NotFound"`
- **Field types**: `std::io::Error` → `source` field
- **Context patterns**: Common error patterns get reasonable defaults

## LSP Integration

The `yoshi_af!` macro provides enhanced IDE support:

```rust
use yoshi_derive::yoshi_af;

yoshi_af! {
    pub enum MyError {
        #[autofix(suggestion = "Check network connectivity")]
        NetworkTimeout,
    }
}
```

## Generated Implementations

The macro generates:

- `std::fmt::Display` with format string support
- `std::error::Error` with proper source chaining
- `From<YourError>` for `yoshi_std::Yoshi` conversion
- Optional helper methods for variant checking

## License

Licensed under either of Apache License, Version 2.0 or MIT License at your option.
