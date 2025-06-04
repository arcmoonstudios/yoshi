# yoshi

![Yoshi Logo](../assets/YoshiLogo.png)

[![Crates.io](https://img.shields.io/crates/v/yoshi.svg)](https://crates.io/crates/yoshi)
[![Docs.rs](https://docs.rs/yoshi/badge.svg)](https://docs.rs/yoshi)
[![Rust Version](https://img.shields.io/badge/rust-1.87%2B-blue.svg)](https://www.rust-lang.org)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](../LICENSE)

The main entry point for the Yoshi error handling framework. This is what you want to add to your `Cargo.toml`.

## What's this?

This crate re-exports everything from the Yoshi framework in one convenient package. Instead of importing `yoshi-std`, `yoshi-derive`, etc. separately, just use `yoshi`.

## Installation

```toml
[dependencies]
yoshi = "0.1"

# With derive macros and serialization
yoshi = { version = "0.1", features = ["derive", "serde"] }

# Everything enabled
yoshi = { version = "0.1", features = ["full"] }
```

## Usage

```rust
# Using the `yoshi!` Macro

The `yoshi!` macro provides a quick way to create structured errors with context.

## Basic Usage

```rust
use yoshi::*;

fn validate_email(email: &str) -> Result<(), Yoshi> {
    if email.is_empty() {
        return Err(yoshi!(
            YoshiKind::Validation,
            "Email cannot be empty",
            field: "email",
            value: email,
            suggestion: "Provide a valid email address"
        ));
    }

    if !email.contains('@') {
        return Err(yoshi!(
            YoshiKind::Validation,
            "Invalid email format: missing @",
            field: "email",
            value: email,
            expected: "user@domain.com"
        ));
    }

    Ok(())
}
```

## Quick Error Creation

```rust
use yoshi::*;

// Simple error
let error = yoshi!(YoshiKind::NotFound, "User not found");

// With context
let error = yoshi!(
    YoshiKind::Database,
    "Connection failed",
    host: "localhost",
    port: 5432,
    timeout: "30s"
);

// With multiple context and suggestions
let error = yoshi!(
    YoshiKind::Config,
    "Invalid configuration",
    file: "/etc/app.conf",
    line: 42,
    suggestion: "Check the configuration syntax",
    suggestion: "Ensure all required fields are present"
);
```

## Bail-style Usage

```rust
use yoshi::*;

fn process_file(path: &str) -> Result<String> {
    let metadata = std::fs::metadata(path).map_err(|e| yoshi!(
        YoshiKind::Io,
        "Failed to read file metadata",
        path: path,
        source: e
    ))?;

    if metadata.len() > 1_000_000 {
        bail!(
            YoshiKind::Validation,
            "File too large",
            path: path,
            size: metadata.len(),
            max_size: 1_000_000,
            suggestion: "Use a smaller file or increase the size limit"
        );
    }

    std::fs::read_to_string(path).map_err(|e| yoshi!(
        YoshiKind::Io,
        "Failed to read file contents",
        path: path,
        source: e
    ))
}
```

## Format String Support

```rust
use yoshi::*;

fn lookup_user(id: u64, database: &str) -> Result<User, Yoshi> {
    // Format strings work just like println!
    let user = db.find_user(id).ok_or_else(|| yoshi!(
        YoshiKind::NotFound,
        "User {} not found in database '{}'", id, database,
        user_id: id,
        database: database,
        table: "users"
    ))?;

    Ok(user)
}
```

## Real-World Example

```rust
use yoshi::*;

async fn fetch_api_data(url: &str) -> Result<ApiResponse> {
    let client = reqwest::Client::new();

    let response = client.get(url)
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| yoshi!(
            YoshiKind::Network,
            "HTTP request failed",
            url: url,
            source: e,
            timeout: "30s",
            suggestion: "Check network connectivity"
        ))?;

    if !response.status().is_success() {
        bail!(
            YoshiKind::Network,
            "API returned error status: {}", response.status(),
            url: url,
            status_code: response.status().as_u16(),
            suggestion: "Check API endpoint and authentication"
        );
    }

    response.json().await.map_err(|e| yoshi!(
        YoshiKind::Parse,
        "Failed to parse JSON response",
        url: url,
        source: e,
        content_type: response.headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown")
    ))
}
```

## Macro Variants

```rust
use yoshi::*;

// Basic error creation
yoshi!(YoshiKind::Internal, "Something went wrong")

// With format string
yoshi!(YoshiKind::Validation, "Invalid value: {}", value)

// With context
yoshi!(YoshiKind::Network, "Connection failed", host: "example.com", port: 80)

// Bail out of function (equivalent to return Err(...))
bail!(YoshiKind::NotFound, "Resource not found", id: 123)

// Ensure condition (equivalent to if !condition { bail!(...) })
ensure!(user.is_active(), YoshiKind::Validation, "User is inactive", user_id: user.id)
```

The `yoshi!` macro makes error creation concise while maintaining Yoshi's structured approach with rich context and metadata.

## What gets re-exported

| From | What |
|------|------|
| `yoshi-std` | `Yoshi`, `YoshiKind`, `YoContext`, `Result` |
| `yoshi-derive` | `YoshiError` derive macro (with `derive` feature) |

## Features

| Feature | Description |
|---------|-------------|
| `std` | Standard library support (default) |
| `derive` | Re-exports `yoshi-derive` macros |
| `serde` | Serialization support |
| `tracing` | Tracing integration |
| `full` | Enables all features |

## Examples

Check out the [examples](../examples/) directory for real-world usage patterns.

## License

Licensed under either of Apache License, Version 2.0 or MIT License at your option.
