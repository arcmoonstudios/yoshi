# yoshi

![Yoshi Logo](https://github.com/arcmoonstudios/yoshi/raw/main/assets/YoshiLogo.png)

[![Crates.io](https://img.shields.io/crates/v/yoshi.svg)](https://crates.io/crates/yoshi)
[![Docs.rs](https://docs.rs/yoshi/badge.svg)](https://docs.rs/yoshi)
[![Rust Version](https://img.shields.io/badge/rust-1.87%2B-blue.svg)](https://www.rust-lang.org)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/arcmoonstudios/yoshi/blob/main/LICENSE)

The main entry point for the Yoshi error handling framework. This crate re-exports everything you need from the Yoshi ecosystem.

## Installation

```toml
[dependencies]
# Basic usage
yoshi = "0.1"

# With derive macros and serialization
yoshi = { version = "0.1", features = ["derive", "serde"] }

# Everything enabled
yoshi = { version = "0.1", features = ["full"] }
```

## Core Functionality

```rust
use yoshi::*;

// Create rich, structured errors
fn validate_input(value: &str) -> Result<()> {
    if value.is_empty() {
        return Err(yoshi!(
            YoshiKind::Validation,
            "Input cannot be empty",
            field: "value",
            suggestion: "Provide a non-empty string"
        ));
    }
    Ok(())
}

// Attach metadata for debugging
fn process_config(path: &str) -> Result<Config> {
    let config = std::fs::read_to_string(path)
        .map_err(|e| yoshi!(YoshiKind::Io, "Failed to read config", path: path, source: e))?
        .parse::<Config>()
        .map_err(|e| yoshi!(YoshiKind::Parse, "Invalid config format", source: e))?;

    // Conditionally add contextual metadata
    if config.is_development() {
        Yoshi::get_current()
            .meta("environment", "development")
            .meta("debug_mode", true);
    }

    Ok(config)
}
```

## Features Table

| Feature | Description |
|---------|-------------|
| `std` | Standard library support (default) |
| `derive` | Re-exports `yoshi-derive` macros |
| `serde` | Serialization support |
| `tracing` | Tracing integration |
| `full` | Enables all features |

## No-std Support

```rust
// In your crate root:
#![cfg_attr(not(feature="std"), no_std)]

use yoshi::prelude::*;

// Works in embedded environments too!
fn no_std_function() -> core::result::Result<(), YoshiKind> {
    if condition_failed() {
        return Err(YoshiKind::Validation);
    }
    Ok(())
}
```

## What This Crate Re-exports

| From | What |
|------|------|
| `yoshi-std` | `Yoshi`, `YoshiKind`, `YoContext`, `Result` |
| `yoshi-derive` | `YoshiError` derive macro (with `derive` feature) |

## Documentation

For more detailed documentation and examples:

- [Macro Guide](https://github.com/arcmoonstudios/yoshi/blob/main/docs/macro.md)
- [Performance Details](https://github.com/arcmoonstudios/yoshi/blob/main/docs/perf.md)
- [Full Examples](https://github.com/arcmoonstudios/yoshi/tree/main/examples/)

## License

Licensed under either of Apache License, Version 2.0 or MIT License at your option.
