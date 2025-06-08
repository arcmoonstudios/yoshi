# Yoshi Error Handling Framework

![Yoshi Logo](assets/YoshiLogo.png)

[![Crates.io](https://img.shields.io/crates/v/yoshi.svg)](https://crates.io/crates/yoshi)
[![Docs.rs](https://docs.rs/yoshi/badge.svg)](https://docs.rs/yoshi)
[![Rust Version](https://img.shields.io/badge/rust-1.87%2B-blue.svg)](https://www.rust-lang.org)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

A structured error handling framework for Rust that tells you what went wrong, where, and how to fix it.

## What is Yoshi?

Yoshi provides rich, structured errors with context and metadata instead of generic "something broke" messages. It combines the ergonomics of `anyhow` with the type safety of `thiserror`, while adding powerful features like error categorization, suggestions, and metadata.

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

- **Powerful Macros** - Create rich errors with one line using `yoshi!`, `bail!`, and `ensure!`
- **Structured Categories** - Categorize errors with `YoshiKind` for consistent handling
- **Rich Context** - Capture and chain context as errors bubble up
- **Metadata & Suggestions** - Attach debugging data and provide fix suggestions
- **Derive Support** - Generate error types and conversions with `#[derive(YoshiError)]`
- **No-std Compatible** - Works in embedded environments

## Concise Error Creation

```rust
// Use the expressive yoshi! macro
let error = yoshi!(
    YoshiKind::Database,
    "Failed to connect to database",
    host: "db.example.com",
    port: 5432,
    retry_count: 3,
    suggestion: "Check database credentials and firewall settings"
);

// Or derive your own error types
use yoshi_derive::YoshiError;

#[derive(Debug, YoshiError)]
pub enum ApiError {
    #[yoshi(kind = "NotFound")]
    #[yoshi(display = "User {user_id} not found")]
    UserNotFound { user_id: u64 },

    #[yoshi(kind = "Timeout")]
    RequestTimeout { seconds: u64 },
}
```

## Documentation & Examples

- [Introduction & Concepts](https://github.com/arcmoonstudios/yoshi/blob/main/docs/overview.md)
- [Macro Guide](https://github.com/arcmoonstudios/yoshi/blob/main/docs/macro.md)
- [Error Context & Metadata](https://github.com/arcmoonstudios/yoshi/blob/main/docs/context.md)
- [Performance Details](https://github.com/arcmoonstudios/yoshi/blob/main/docs/perf.md)
- [Migration Guide](https://github.com/arcmoonstudios/yoshi/blob/main/docs/migration.md)
- [API Docs](https://docs.rs/yoshi)
- [Examples](https://github.com/arcmoonstudios/yoshi/tree/main/examples/)

## License

Licensed under either of Apache License, Version 2.0 or MIT License at your option.

---

Made by [ArcMoon Studios](https://github.com/arcmoonstudios)
