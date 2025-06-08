# Yoshi Error Handling Framework Overview

The Yoshi Error Framework provides a structured approach to error handling in Rust that combines the ergonomics of `anyhow` with the type safety of `thiserror` while adding rich context and metadata.

## Core Concepts

### Structured Errors

Rather than generic string messages, Yoshi provides structured error types:

```rust
// Instead of this:
Err("failed to connect to database")

// Yoshi gives you this:
Err(yoshi!(
    YoshiKind::Database,
    "Failed to connect to database",
    host: "db.example.com",
    port: 5432,
    timeout: "30s"
))
```

### Error Kinds

Yoshi categorizes errors using `YoshiKind`, making error handling and reporting more consistent:

```rust
pub enum YoshiKind {
    Validation,  // Input validation errors
    NotFound,    // Resource not found
    Permission,  // Permission denied
    Auth,        // Authentication/authorization failures
    Timeout,     // Operation timed out
    Config,      // Configuration errors
    Database,    // Database-related errors
    Network,     // Network-related errors
    Io,          // IO-related errors
    Parse,       // Parsing errors
    Internal,    // Internal/unexpected errors
    // ...and more
}
```

### Context Chaining

As errors propagate up the call stack, Yoshi maintains the full context chain:

```rust
fn get_user(id: u64) -> Result<User> {
    db.query("SELECT * FROM users WHERE id = ?", [id])
        .context("Querying user database")
        .map_err(|e| yoshi!(
            YoshiKind::Database,
            "Failed to retrieve user",
            user_id: id,
            source: e
        ))?
        .ok_or_else(|| yoshi!(
            YoshiKind::NotFound,
            "User not found",
            user_id: id
        ))
}

// When you call get_user and it fails, you get the full context chain:
// Error: Failed to retrieve user
// Caused by: Querying user database
// Caused by: Connection refused
```

### Metadata Attachment

Yoshi errors can carry rich metadata to help with debugging:

```rust
fn validate_config(config: &Config) -> Result<()> {
    let mut error = None;

    // Check required fields
    for field in REQUIRED_FIELDS {
        if !config.has_field(field) {
            error = Some(yoshi!(
                YoshiKind::Validation,
                "Missing required field",
                field: field,
                config_file: config.path()
            ));
            break;
        }
    }

    if let Some(err) = error {
        // Add additional metadata
        return Err(err.meta("config_version", config.version())
                     .meta("valid_fields", config.fields().join(", ")));
    }

    Ok(())
}
```

### Comprehensive Error Types

Use the derive macro to create rich error types easily:

```rust
use yoshi_derive::YoshiError;

#[derive(Debug, YoshiError)]
pub enum ApiError {
    #[yoshi(display = "User {user_id} not found")]
    #[yoshi(kind = "NotFound")]
    UserNotFound {
        user_id: u64,
        #[yoshi(skip)]
        _private: ()
    },

    #[yoshi(display = "Database error: {message}")]
    #[yoshi(kind = "Database")]
    DatabaseError {
        message: String,
        #[yoshi(source)]
        cause: Option<sqlx::Error>
    },

    #[yoshi(display = "Request timed out after {timeout_secs} seconds")]
    #[yoshi(kind = "Timeout")]
    #[yoshi(transient = true)]  // Marks error as possibly transient
    RequestTimeout {
        timeout_secs: u64
    },
}
```

## No-Std Support

Yoshi works in embedded environments too:

```rust
// In your crate root:
#![cfg_attr(not(feature="std"), no_std)]

// Then use Yoshi's no_std compatible features
use yoshi::prelude::*;

fn embedded_function() -> Result<(), YoshiKind> {
    // Works without the standard library!
    if something_failed() {
        return Err(YoshiKind::Validation);
    }

    Ok(())
}
```

## Integration with Ecosystems

### Tracing Integration

Yoshi integrates with the `tracing` ecosystem:

```rust
use yoshi::{Yoshi, YoshiKind};
use tracing::instrument;

#[instrument]
fn process_request(req: Request) -> Result<Response, Yoshi> {
    // Yoshi error fields are automatically captured in spans
    let user = get_user(req.user_id)?;
    // ...
}
```

### Serde Integration

Errors can be serialized and deserialized with serde:

```rust
use yoshi::*;
use serde_json;

fn handle_error(err: &Yoshi) {
    // Serialize error to JSON
    let error_json = serde_json::to_string(err).unwrap();
    println!("Error JSON: {}", error_json);

    // Can be deserialized back into a Yoshi error
    let deserialized: Yoshi = serde_json::from_str(&error_json).unwrap();
}
```

## API Documentation

For full API documentation, visit:

- [Yoshi API Docs](https://docs.rs/yoshi)
- [Yoshi Std API Docs](https://docs.rs/yoshi-std)
- [Yoshi Derive API Docs](https://docs.rs/yoshi-derive)
