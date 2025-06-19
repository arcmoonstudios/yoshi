# yoshi-derive

![Yoshi Logo](../assets/YoshiLogo.png)

[![Crates.io](https://img.shields.io/crates/v/yoshi-derive.svg)](https://crates.io/crates/yoshi-derive)
[![Docs.rs](https://docs.rs/yoshi-derive/badge.svg)](https://docs.rs/yoshi-derive)
[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](../LICENSE)

Procedural macros for the Yoshi error handling framework. Intelligent code generation that writes the boilerplate for you.

## What's this?

This crate provides `#[derive(YoshiError)]` and the powerful `yoshi_af!` macro that automatically generate error types optimized for Yoshi. The macros are smart enough to infer sensible defaults from your code structure while giving you complete control when you need it. Less typing, more functionality.

## Installation

```toml
[dependencies]
yoshi-derive = "0.1"
yoshi = "0.1"  # Or yoshi-std if you prefer minimal dependencies
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

The macro analyzes your code structure and automatically infers appropriate attributes:

- **Variant names** - `timeout` becomes `transient`, `not_found` becomes `kind = "NotFound"`
- **Field types** - `std::io::Error` automatically becomes a `source` field
- **Common patterns** - Standard error patterns get sensible defaults without configuration

The inference engine handles the common cases so you can focus on the unique aspects of your errors.

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

## What gets generated

The macro tries to generate useful implementations:

- `std::fmt::Display` with format string support
- `std::error::Error` with proper source chaining
- `From<YourError>` for `yoshi::Yoshi` conversion
- Optional helper methods for variant checking (if you want them)

## Testing

The `yoshi-derive` crate has the most extensive integration test suite:

### Test Statistics

- **126 Integration Tests** - Comprehensive macro functionality testing
- **0 Doc Tests** - Procedural macros tested through integration tests
- **0 Ignored Tests** - Every test validates real macro functionality
- **Complete macro coverage** - All derive and procedural macro features tested

### Running Tests

```bash
# Run all yoshi-derive tests
cargo test -p yoshi-derive

# Run specific test categories
cargo test -p yoshi-derive integration_tests
cargo test -p yoshi-derive auto_correction_tests
cargo test -p yoshi-derive compilation_tests
cargo test -p yoshi-derive error_handling_tests
```

### Test Categories

- **Integration Tests (30 tests):** Complete derive macro functionality
- **Auto Correction Tests (15 tests):** `yoshi_af!` macro auto-correction features
- **Compilation Tests (16 tests):** Macro compilation and edge cases
- **Error Handling Tests (11 tests):** Error scenarios and validation
- **Autofix Trigger Integration (10 tests):** Integration with yoshi-deluxe
- **Simple Auto Correction Tests (19 tests):** Basic auto-correction patterns
- **Diagnostic Tests (9 tests):** Macro diagnostic and error reporting
- **Test Runner (13 tests):** Comprehensive macro expansion testing

### Key Test Features

- **Macro Expansion Validation:** Ensures correct code generation
- **Auto-Correction Integration:** Tests `yoshi_af!` macro auto-correction
- **Error Pattern Detection:** Validates error pattern recognition
- **LSP Integration Testing:** Tests IDE integration capabilities
- **Performance Testing:** Macro expansion performance validation
- **Edge Case Coverage:** Complex scenarios and unusual type combinations

### Why No Doc Tests?

Procedural macro crates typically don't have doc tests because:

- **Integration tests are more appropriate** for testing macro functionality
- **Macro expansion testing** requires complex setup better suited to integration tests
- **Real-world usage scenarios** are better validated through comprehensive integration tests
- **The 126 integration tests** provide far more thorough coverage than doc tests could

## License

Licensed under either of Apache License, Version 2.0 or MIT License at your option.
