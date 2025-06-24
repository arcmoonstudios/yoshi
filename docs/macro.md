# Yoshi Macro Guide

This guide showcases the powerful macros that make Yoshi the most advanced error handling framework for Rust.

## Installation

First, make sure you have Yoshi installed with the necessary features:

```bash
cargo add yoshi --features full
```

## The `yopost!` Macro

The adaptive `yopost!` macro intelligently creates structured errors based on usage context.

### Message-Based Error Creation

```rust
use yoshi::*;

fn validate_email(email: &str) -> Hatch<()> {
    if email.is_empty() {
        return Err(yopost!(message: "Email cannot be empty"));
    }

    if !email.contains('@') {
        return Err(yopost!(message: "Invalid email format: missing @"));
    }

    Ok(())
}
```

### Structured Error Kind Creation

```rust
use yoshi::*;

fn validate_user_data(email: &str) -> Hatch<()> {
    if email.is_empty() {
        return Err(yopost!(kind: YoshiKind::Validation {
            field: "email".into(),
            message: "Email cannot be empty".into(),
            expected: Some("user@domain.com".into()),
            actual: Some(email.into()),
        }));
    }

    Ok(())
}
```

### Error Wrapping with Context

```rust
use yoshi::*;

fn read_config_file(path: &str) -> Hatch<String> {
    // Wrap existing errors
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let wrapped = yopost!(error: io_err);

    // With additional context
    let enhanced = yopost!(
        error: io_err,
        with_signpost = "Check the file path and permissions"
    );

    Ok("config content".to_string())
}
```

### Advanced Context Chaining

```rust
use yoshi::*;

fn complex_operation() -> Hatch<String> {
    let error = yopost!(
        message: "Database connection failed",
        with_metadata = ("host", "localhost:5432"),
        with_metadata = ("timeout", "30s"),
        with_signpost = "Check database server status",
        with_priority = 90
    );

    Err(error)
}
```

## The `yoshi_af!` Macro - Auto-Fix Error Enums

The `yoshi_af!` macro provides compile-time auto-correction and LSP integration:

```rust
use yoshi::*;

// Auto-correcting error enum with comprehensive capabilities
yoshi_af! {
    #[derive(Debug)]
    pub enum ApiError {
        #[yoshi(display = "User {user_id} not found")]
        #[yoshi(suggestion = "Verify the user ID and check user permissions")]
        UserNotFound {
            user_id: u64,
            search_locations: Vec<String>,
        },

        #[yoshi(display = "Database error: {message}")]
        #[yoshi(kind = "Database")]
        DatabaseError {
            message: String,
            #[yoshi(source)]
            cause: Option<Box<dyn std::error::Error>>,
        },
    }
}

// The macro automatically generates:
// - Conversion traits (From, Into)
// - Error trait implementations
// - YoshiError derive functionality
// - LSP integration for auto-fixes
```

## The `yum!` Macro - Enhanced Error Debugging

The `yum!` macro provides comprehensive error analysis and debugging output:

```rust
use yoshi::*;

fn process_request() -> Hatch<String> {
    let error = yopost!(kind: YoshiKind::Network {
        message: "Connection failed".into(),
        source: None,
        error_code: Some(503),
    })
    .with_metadata("host", "api.example.com")
    .with_signpost("Check network connectivity");

    Err(error)
}

fn main() {
    match process_request() {
        Ok(result) => println!("Success: {}", result),
        Err(error) => {
            yum!(error);  // Prints comprehensive error information:
                         // - Error instance ID for correlation
                         // - Primary error message and kind
                         // - Complete context chain with metadata
                         // - Source error information if available
                         // - Backtrace information (when enabled)
        }
    }
}
```

## Context Extension with `.lay()` Method

The `.lay()` method provides ergonomic context chaining:

```rust
use yoshi::*;

fn lookup_user(id: u64, database: &str) -> Hatch<User> {
    let user = db.find_user(id)
        .lay("Failed to query user database")?
        .ok_or_else(|| yopost!(kind: YoshiKind::NotFound {
            resource_type: "User".into(),
            identifier: format!("user_id_{}", id).into(),
            search_locations: Some(vec![database.into()]),
        }))?;

    Ok(user)
}
```

## Real-World Example

```rust
use yoshi::*;
use std::time::Duration;

async fn fetch_api_data(url: &str) -> Hatch<ApiResponse> {
    let client = reqwest::Client::new();

    let response = client.get(url)
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .lay("HTTP request failed")?;

    if !response.status().is_success() {
        return Err(yopost!(kind: YoshiKind::Network {
            message: format!("API returned error status: {}", response.status()).into(),
            source: None,
            error_code: Some(response.status().as_u16() as u32),
        })
        .with_metadata("url", url)
        .with_signpost("Check API endpoint and authentication"));
    }

    let api_response = response.json().await
        .lay("Failed to parse JSON response")?;

    Ok(api_response)
}
```

## Macro Variants Cheat Sheet

```rust
use yoshi::*;

// Message-based error creation
yopost!(message: "Something went wrong")
yopost!(message: "Invalid value: {}", value)

// Structured error kind creation
yopost!(kind: YoshiKind::Network {
    message: "Connection failed".into(),
    source: None,
    error_code: Some(503),
})

// Error wrapping
yopost!(error: io_error)
yopost!(error: io_error, with_signpost = "Check file permissions")

// Advanced context chaining
yopost!(
    message: "Database connection failed",
    with_metadata = ("host", "localhost:5432"),
    with_metadata = ("timeout", "30s"),
    with_signpost = "Check database server status"
)

// Auto-correcting error enums
yoshi_af! {
    #[derive(Debug)]
    pub enum MyError {
        #[yoshi(display = "User {id} not found")]
        UserNotFound { id: u64 },
    }
}

// Enhanced error debugging
yum!(error);  // Comprehensive error analysis

// Context extension
result.lay("Additional context")?
```

## Key Benefits

- **🚀 Performance** - Zero-cost abstractions with compile-time optimizations
- **🔧 Auto-Correction** - LSP-integrated auto-fixes with `yoshi_af!` macro
- **📊 Rich Context** - Structured error kinds with comprehensive metadata
- **🎯 Type Safety** - Strong typing with ergonomic error handling
- **🛠️ Developer Experience** - Enhanced debugging with `yum!` macro

The Yoshi macros provide the most advanced error handling capabilities in the Rust ecosystem, combining performance, safety, and developer experience.
