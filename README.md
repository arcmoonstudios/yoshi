# Yoshi Error Handling Framework

![Yoshi Logo](assets/YoshiLogo.png)

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.87%2B-orange.svg)](https://forge.rust-lang.org/releases.html)
[![Build Status](https://img.shields.io/badge/build-passing-green.svg)](https://github.com/arcmoonstudios/yoshi)

A structured error handling framework for Rust that actually tells you what went wrong.

## What's this?

Yoshi gives you rich, structured errors with context instead of generic "something broke" messages. Think `anyhow` but with categories, metadata, and suggestions for fixing things.

## Quick Start

```toml
[dependencies]
yoshi = "0.1"
```

```rust
use yoshi::{Yoshi, YoshiKind, Result};

fn load_config(path: &str) -> Result<String> {
    std::fs::read_to_string(path)
        .map_err(|e| Yoshi::new(YoshiKind::Io {
            message: "Failed to read config".into(),
            source: Some(Box::new(e)),
            path: Some(path.into()),
        }))
        .context(format!("Loading config from {}", path))
}

fn main() {
    match load_config("/etc/app/config.toml") {
        Ok(config) => println!("Config: {}", config),
        Err(err) => {
            eprintln!("Error: {}", err);
            eprintln!("Context: {:#}", err.context_chain());
        }
    }
}
```

## Why Yoshi?

**Structured errors**: Instead of `"error"`, get `IoError { path: "/etc/config", operation: "read" }`

**Rich context**: Errors carry metadata, suggestions, and full context chains

**Performance**: Sub-microsecond error creation, minimal allocations

**Derive macros**: Generate error types automatically

```rust
use yoshi_derive::YoshiError;

#[derive(Debug, YoshiError)]
pub enum MyError {
    #[yoshi(display = "User {user_id} not found")]
    #[yoshi(kind = "NotFound")]
    UserNotFound { user_id: u32 },

    #[yoshi(display = "Database timeout")]
    #[yoshi(kind = "Timeout")]
    #[yoshi(transient = true)]
    DatabaseTimeout,
}
```

## Features

- **Structured error categories** - Know exactly what type of error occurred
- **Context chaining** - Full error history as problems propagate
- **Metadata attachment** - Add debugging info to errors
- **Performance optimized** - <1μs error creation
- **no_std support** - Works in embedded environments
- **Derive macros** - Generate error types automatically

## Performance

| Framework | Error Creation | Memory Usage |
|-----------|---------------|--------------|
| **Yoshi** | **1201 ns** | **208 bytes** |
| thiserror | 22 ns | 24 bytes |
| anyhow | 629 ns | 8 bytes |
| eyre | 51 ns | 8 bytes |

*Yoshi trades some speed for much richer error information**

## Documentation

- [API Docs](https://docs.rs/yoshi)
- [Examples](examples/)
- [Migration Guide](docs/migration.md)

## License

Licensed under either of Apache License, Version 2.0 or MIT License at your option.

---

Made by [ArcMoon Studios](https://github.com/arcmoonstudios)
