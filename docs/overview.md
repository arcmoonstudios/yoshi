# Yoshi Error Handling Framework Overview

The Yoshi Error Framework provides a next-generation structured approach to error handling in Rust that combines the ergonomics of `anyhow` with the type safety of `thiserror` while adding rich context, metadata, and auto-correction capabilities.

## Core Concepts

### Structured Errors

Rather than generic string messages, Yoshi provides rich, structured error types with comprehensive context:

```rust
use yoshi::*;

// Instead of this:
Err("failed to connect to database")

// Yoshi gives you this:
Err(yoshi!(message: "Failed to connect to database"))

// Or with structured error kinds:
Err(yoshi!(kind: YoshiKind::Network {
    message: "Database connection failed".into(),
    source: None,
    error_code: Some(503),
}))
```

### Error Kinds

Yoshi categorizes errors using structured `YoshiKind` variants with rich field information:

```rust
pub enum YoshiKind {
    // I/O operations with no_std compatibility
    Io(NoStdIo),

    // Network errors with connection context
    Network {
        message: Arc<str>,
        source: Option<Box<Yoshi>>,
        error_code: Option<u32>,
    },

    // Configuration errors with path information
    Config {
        message: Arc<str>,
        source: Option<Box<Yoshi>>,
        config_path: Option<Arc<str>>,
    },

    // Validation errors with field-level precision
    Validation {
        field: Arc<str>,
        message: Arc<str>,
        expected: Option<Arc<str>>,
        actual: Option<Arc<str>>,
    },

    // Internal errors with component tracking
    Internal {
        message: Arc<str>,
        source: Option<Box<Yoshi>>,
        component: Option<Arc<str>>,
    },

    // Resource not found with typed identification
    NotFound {
        resource_type: Arc<str>,
        identifier: Arc<str>,
        search_locations: Option<Vec<Arc<str>>>,
    },

    // Timeout with detailed timing information
    Timeout {
        operation: Arc<str>,
        duration: Duration,
        expected_max: Option<Duration>,
    },

    // Resource exhaustion with precise metrics
    ResourceExhausted {
        resource: Arc<str>,
        limit: Arc<str>,
        current: Arc<str>,
        usage_percentage: Option<f64>,
    },

    // Security errors with threat classification
    Security {
        message: Arc<str>,
        source: Option<Box<Yoshi>>,
        security_level: Arc<str>,
    },

    // Foreign error wrapper with type information
    Foreign {
        error: Box<dyn Error + Send + Sync + 'static>,
        error_type_name: Arc<str>,
    },

    // Multiple errors with categorization
    Multiple {
        errors: Vec<Yoshi>,
        primary_index: Option<usize>,
    },
}
```

### Context Chaining

As errors propagate up the call stack, Yoshi maintains the full context chain with rich metadata:

```rust
use yoshi::*;

fn get_user(id: u64) -> Hatch<User> {
    let user_data = db.query("SELECT * FROM users WHERE id = ?", [id])
        .lay("Querying user database")?;

    if user_data.is_empty() {
        return Err(yoshi!(kind: YoshiKind::NotFound {
            resource_type: "User".into(),
            identifier: format!("user_id_{}", id).into(),
            search_locations: Some(vec!["users_table".into(), "user_cache".into()]),
        }));
    }

    parse_user(user_data)
        .lay("Failed to parse user data")
}

// When you call get_user and it fails, you get the full context chain:
// Error: User not found (user_id_12345)
// Context: Failed to parse user data
// Context: Querying user database
// Searched in: users_table, user_cache
```

### Metadata Attachment

Yoshi errors can carry rich metadata and suggestions to help with debugging:

```rust
use yoshi::*;

fn validate_config(config: &Config) -> Hatch<()> {
    // Check required fields
    for field in REQUIRED_FIELDS {
        if !config.has_field(field) {
            return Err(yoshi!(kind: YoshiKind::Config {
                message: format!("Missing required field '{}'", field).into(),
                source: None,
                config_path: Some(config.path().into()),
            })
            .with_metadata("config_version", config.version())
            .with_metadata("valid_fields", config.fields().join(", "))
            .with_signpost("Check the configuration file format and ensure all required fields are present"));
        }
    }

    Ok(())
}

// Enhanced error output with yum! macro
fn handle_config_error(result: Hatch<()>) {
    if let Err(error) = result {
        yum!(error);  // Prints comprehensive error information with metadata
    }
}
```

### Auto-Correction with yoshi_af! Macro

Use the powerful `yoshi_af!` macro for compile-time auto-correction and LSP integration:

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
            #[yoshi(context = "connection_info")]
            connection_string: String,
        },

        #[yoshi(display = "Request timed out after {timeout_secs} seconds")]
        #[yoshi(kind = "Timeout")]
        #[yoshi(transient = true)]  // Marks error as possibly transient
        RequestTimeout {
            timeout_secs: u64,
            operation: String,
        },
    }
}

// The macro automatically generates:
// - Conversion traits (From, Into)
// - Error trait implementations
// - YoshiError derive functionality
// - LSP integration for auto-fixes
// - Comprehensive Display implementations
```

## No-Std Support

Yoshi provides comprehensive no_std support with yoshi-core as the foundation:

```rust
// In your embedded crate root:
#![no_std]

// Use yoshi-core for no_std environments
use yoshi_core::*;

fn embedded_function() -> Result<(), Yoshi> {
    // Works without the standard library!
    if something_failed() {
        return Err(Yoshi::new(YoshiKind::Validation {
            field: "sensor_reading".into(),
            message: "Value out of range".into(),
            expected: Some("0-100".into()),
            actual: Some("150".into()),
        }));
    }

    Ok(())
}

// For std environments, use the full yoshi crate
#[cfg(feature = "std")]
use yoshi::*;

#[cfg(feature = "std")]
fn std_function() -> Hatch<String> {
    // Full feature set including backtrace, std::io integration, etc.
    yoshi!(message: "This works with std features")
        .with_metadata("environment", "std")
        .into()
}
```

## Integration with Ecosystems

### Enhanced Error Debugging with yum

Yoshi provides the `yum!` macro for comprehensive error analysis:

```rust
use yoshi::*;

fn process_request(req: Request) -> Hatch<Response> {
    let user = get_user(req.user_id)
        .lay("Failed to retrieve user for request processing")?;

    // Process user...
    Ok(response)
}

fn main() {
    match process_request(request) {
        Ok(response) => println!("Success: {:?}", response),
        Err(error) => {
            yum!(error);  // Comprehensive error output with:
                         // - Error instance ID
                         // - Complete context chain
                         // - Metadata and suggestions
                         // - Source error information
                         // - Backtrace (when available)
        }
    }
}
```

### Serde Integration

Errors can be serialized and deserialized with serde (when feature enabled):

```rust
use yoshi::*;
use serde_json;

#[cfg(feature = "serde")]
fn handle_error_serialization(err: &Yoshi) {
    // Serialize error to JSON
    let error_json = serde_json::to_string(err).unwrap();
    println!("Error JSON: {}", error_json);

    // Can be deserialized back into a Yoshi error
    let deserialized: Yoshi = serde_json::from_str(&error_json).unwrap();
}
```

### Result Type Alias

Yoshi provides `Hatch<T>` as a convenient Result type alias:

```rust
use yoshi::*;

// Instead of Result<T, Yoshi>
fn operation() -> Hatch<String> {
    // Your operation logic
    Ok("success".to_string())
}

// Equivalent to Result<T, Box<dyn std::error::Error>>
type Hatch<T> = Result<T, Yoshi>;
```

## Crate Architecture

The Yoshi framework is organized into specialized crates:

### Core Crates

- **`yoshi-core`** - No-std foundation with essential error types and algorithms
- **`yoshi-std`** - Standard library convenience layer with enhanced features
- **`yoshi-derive`** - Procedural macros for auto-correction and code generation
- **`yoshi-deluxe`** - Advanced pattern detection and auto-correction engine
- **`yoshi`** - Facade crate that re-exports everything for easy use

### Usage Recommendations

```rust
// For no_std environments
use yoshi_core::*;

// For std environments with basic features
use yoshi_std::*;

// For full feature set (recommended)
use yoshi::*;
```

## API Documentation

For complete API documentation, visit:

- [Yoshi API Docs](https://docs.rs/yoshi) - Main facade crate
- [Yoshi Core API Docs](https://docs.rs/yoshi-core) - No-std foundation
- [Yoshi Std API Docs](https://docs.rs/yoshi-std) - Standard library features
- [Yoshi Derive API Docs](https://docs.rs/yoshi-derive) - Procedural macros
- [Yoshi Deluxe API Docs](https://docs.rs/yoshi-deluxe) - Auto-correction engine

## Key Features Summary

- **🚀 Performance** - Zero-cost abstractions with compile-time optimizations
- **🔧 Auto-Correction** - LSP-integrated auto-fixes with `yoshi_af!` macro
- **📊 Rich Context** - Structured error kinds with comprehensive metadata
- **🎯 Type Safety** - Strong typing with ergonomic error handling
- **🔄 No-std Support** - Works in embedded environments via yoshi-core
- **🛠️ Developer Experience** - Enhanced debugging with `yum!` macro
- **⚡ Ecosystem Integration** - Seamless integration with existing Rust error handling
