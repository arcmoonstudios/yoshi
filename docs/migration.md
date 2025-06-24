# Migrating Your Rust Error Handling to Yoshi

This guide provides a comprehensive roadmap for migrating existing Rust projects from popular error handling crates like `anyhow`, `eyre`, `thiserror`, and `snafu` to the Yoshi error framework. Yoshi is designed for performance-critical applications, offering structured, flexible, and highly performant error handling with rich contextualization.

## Why Migrate to Yoshi?

Yoshi offers a unique blend of features that often surpass the capabilities of other error handling solutions, particularly for enterprise-grade applications:

* **Structured Errors**: Define precise error categories (`YoshiKind`) with relevant fields, moving beyond opaque strings or trait objects.
* **Rich Contextualization**: Attach multiple layers of diagnostic information, arbitrary typed payloads, user-facing suggestions, and metadata as errors propagate.
* **Mathematical Performance**: Sub-microsecond error creation and O(1) context attachment with intelligent memory optimizations like string interning.
* **`no_std` Compatibility**: Full functionality available in `no_std` environments, ensuring broad applicability across embedded and high-performance contexts.
* **Unified API**: A consistent API for both simple `Result` propagation (`HatchExt`) and complex error construction (`yopost!` macro or direct `Yoshi::new`).
* **Extensible by Design**: Supports custom error definitions via `yoshi-derive` macros, seamlessly integrating into the `Yoshi` ecosystem.
* **Advanced Features**: Built-in support for error priority, recovery strategies, detailed context analysis, and performance monitoring.

## Core Yoshi Concepts vs. Other Frameworks

Before diving into specific migration paths, let's understand how Yoshi's core components map to concepts you might be familiar with:

| Other Framework Concept                                    | Yoshi Equivalent(s)                                                                    | Description                                                                                                                                                                                                                                                                                                                                                     |
| :--------------------------------------------------------- | :------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `std::error::Error` (trait object)                         | `Yoshi` (main error type)                                                              | `Yoshi` is the central error type, encapsulating structured `YoshiKind`, context chain, and optional `YoshiBacktrace`. It implements `std::error::Error`.                                                                                                                                                                                              |
| Custom error enums (`thiserror`, `snafu`)                  | `yoshi_af!` macro for auto-correcting enums                                                 | Define custom, structured error types with comprehensive auto-correction capabilities. The `yoshi_af!` macro generates LSP integration, conversion traits, and enhanced error handling.                                                                                                                                                                             |
| Dynamic error types (`anyhow::Error`, `eyre::Report`)      | `yopost!(error: error)` or `yopost!(message: "text")`                                       | Wrap any `std::error::Error` or create message-based errors. Yoshi automatically handles conversion and provides structured error kinds.                                                                                                                                                                                                                |
| Context chaining (`.context()`, `.wrap_err()`)             | `.lay("context")`, `Yoshi::context()`, `Yoshi::with_metadata()` | Yoshi's `.lay()` method provides ergonomic context chaining. Additional methods like `with_metadata()` and `with_signpost()` add rich diagnostic information.                                                                                                                                                                  |
| Backtraces (`RUST_BACKTRACE`, `anyhow::Error::backtrace()`) | `YoshiBacktrace` (conditional capture), `Yoshi::backtrace()`                           | Yoshi captures backtraces when enabled, with enhanced location tracking and thread information. Available in yoshi-std layer.                                                                                                                                                                                                  |
| Error kinds/variants (enum variants, `snafu` selectors)    | `YoshiKind` (structured enum)                                                                     | Yoshi provides a rich, structured `YoshiKind` enum with detailed fields for each variant (e.g., `Network { message, source, error_code }`, `Validation { field, message, expected, actual }`).                                                                                                                                                                          |
| Error introspection (`.downcast_ref()`, `ErrorCompat`)     | `Yoshi::kind()`, `Yoshi::contexts()`, `yum!` macro                          | Yoshi offers dedicated accessor methods and the `yum!` macro for comprehensive error analysis and debugging.                                                                                                                                                                                                                                                  |
| Result type alias (`anyhow::Result`, `snafu::Result`)      | `Hatch<T>` (recommended)                                                                     | A convenient type alias for `Result<T, Yoshi>`. The name "Hatch" reflects the Yoshi theme and provides clear semantic meaning.                                                                                                                                                                                                                                     |
| Error creation macros (`anyhow!`, `eyre!`, `snafu!`)       | `yopost!` macro, `yoshi_af!` macro, `yum!` macro                                                                         | Multiple powerful macros: `yopost!` for adaptive error creation, `yoshi_af!` for auto-correcting enums, and `yum!` for enhanced debugging output.                                                                                                                                                                                   |

## Migration Paths by Crate

### 1. Migrating from `thiserror`

`thiserror` is excellent for defining custom error enums with `Display` and `Error` trait implementations. Yoshi's `yoshi-derive` crate offers a superset of `thiserror`'s capabilities, allowing for a straightforward transition.

**Key Changes:**

* Replace `#[derive(Error)]` with `#[derive(YoshiError)]`.
* Replace `#[error(...)]` with `#[yoshi(display = "...")]`.
* Replace `#[source]` with `#[yoshi(source)]`.
* Replace `#[from]` with `#[yoshi(from)]` (for single-field tuple variants).
* Add `#[yoshi(kind = "YoshiKindVariant")]` to explicitly map your error variants to Yoshi's predefined categories.
* Consider adding `#[yoshi(error_code = ...)]`, `#[yoshi(severity = ...)]`, `#[yoshi(transient = true)]`, `#[yoshi(context = "...")`, `#[yoshi(suggestion = "...")` for enhanced diagnostics.
* Your derived error type `MyError` will now automatically implement `From<MyError> for Yoshi`, allowing seamless conversion.

**Example: `thiserror` to `yoshi-derive`**

```rust
// Before (thiserror)
#[derive(thiserror::Error, Debug)]
pub enum OldAppError {
    #[error("Failed to load config: {source}")]
    ConfigLoad {
        #[from]
        source: std::io::Error,
        path: String,
    },
    #[error("User {user_id} not found")]
    UserNotFound { user_id: u32 },
}

// After (yoshi-derive)
use yoshi_derive::YoshiError;
use yoshi_std::{Yoshi, YoshiKind, HatchExt}; // Needed for type and extension methods

#[derive(YoshiError, Debug)]
#[yoshi(error_code_prefix = "APP")] // Optional: Add a global error code prefix
pub enum NewAppError {
    #[yoshi(display = "Failed to load config: {source_err}")] // Changed to source_err for clarity
    #[yoshi(kind = "Config")] // Map to YoshiKind::Config
    #[yoshi(error_code = 101)] // Optional: Specific error code
    ConfigLoad {
        #[yoshi(source)] // Mark as source
        source_err: std::io::Error,
        #[yoshi(context = "config_path")] // Add path to context metadata
        path: String,
    },
    #[yoshi(display = "User not found: {user_id}")]
    #[yoshi(kind = "NotFound")] // Map to YoshiKind::NotFound
    #[yoshi(severity = 50)] // Optional: Set severity
    UserNotFound {
        #[yoshi(context = "user_identifier")] // Add user_id to context metadata
        user_id: u32,
        #[yoshi(suggestion = "Check user ID in database.")] // Add suggestion
        #[yoshi(shell)] // Add UserInfo as a typed shell
        user_info: UserInfo, // Assume UserInfo is a struct
    },
}

#[derive(Debug, Clone)]
pub struct UserInfo { pub name: String } // Example struct for shell

// Usage comparison
fn old_function() -> Result<(), OldAppError> {
    // ... logic ...
    Err(OldAppError::ConfigLoad {
        source: std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"),
        path: "/app/config.json".to_string(),
    })
}

fn new_function() -> yoshi_std::Result<(), Yoshi> { // Use yoshi_std::Result
    // ... logic ...
    Err(NewAppError::ConfigLoad {
        source_err: std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"),
        path: "/app/config.json".to_string(),
    }.into()) // Automatically converts to Yoshi
    .context("During configuration initialization".to_string())
}

// Example usage of UserNotFound with shell
fn new_function_user_not_found() -> yoshi_std::Result<(), Yoshi> {
    Err(NewAppError::UserNotFound {
        user_id: 42,
        user_info: UserInfo { name: "Arthur Dent".to_string() },
    }.into())
    .context("Failed to retrieve user profile".to_string())
}

#### ✅ Example: Macro-Based Error Creation

```rust
// Demonstrating how to use the yopost! macro with derived errors
fn new_function_user_not_found_with_macro() -> Yoshi {
    yopost!(kind: NewAppError::UserNotFound {
        user_id: 42,
        user_info: UserInfo { name: "Arthur Dent".to_string() },
    }.into()) // Still needs .into() here as yopost! macro expects YoshiKind or Error
}
```

#### ✅ Example: Direct API-Based Error Creation

```rust
// Demonstrating direct API usage of derived error
fn new_function_user_not_found_with_api() -> Yoshi {
    Yoshi::new(NewAppError::UserNotFound {
        user_id: 42,
        user_info: UserInfo { name: "Arthur Dent".to_string() },
    }.into()) // Direct conversion
}
```

> **Note:** `thiserror` is still excellent for lightweight projects. Migration to Yoshi is most beneficial when structured diagnostics, telemetry, or enterprise-grade observability are priorities.

```text

### 2. Migrating from `anyhow`

`anyhow` is favored for its simplicity in dynamic error handling and contextualization. Yoshi offers structured alternatives while maintaining ease of use.

**Key Changes:**

* Replace `anyhow::Result<T>` with `yoshi::Result<T>`.
* Replace `anyhow::Error` with `Yoshi`.
* Replace `anyhow::anyhow!` with `yopost!`.
* Replace `.context()` (`anyhow`) with `Yoshi::context()` or `HatchExt::context()`.
* For wrapping any `std::error::Error`, use `Yoshi::foreign(error)`.
* Backtraces are controlled by `RUST_BACKTRACE` environment variable in Yoshi, similar to `anyhow`'s default.

**Example: `anyhow` to Yoshi**

```rust
// Before (anyhow)
use anyhow::{Context, Result};
use std::io::{self, ErrorKind};

fn parse_file(path: &str) -> Result<String> {
    std::fs::read_to_string(path)
        .context(format!("Failed to read file: {}", path))
}

fn process_data(data: &str) -> Result<String> {
    if data.is_empty() {
        anyhow::bail!("Data cannot be empty");
    }
    Ok(format!("Processed: {}", data))
}

fn run_old_app() -> Result<String> {
    let data = parse_file("non_existent.txt")?;
    process_data(&data)
}

// After (Yoshi)
use yoshi::{yoshi, Result as YoshiResult, Yoshi, YoshiKind, HatchExt};
use std::io::{self, ErrorKind}; // For compatibility with original error types

fn parse_file_yoshi(path: &str) -> YoshiResult<String> {
    std::fs::read_to_string(path)
        .map_err(Yoshi::from) // Convert std::io::Error to Yoshi
        .context(format!("Failed to read file: {}", path).to_string()) // Add context using HatchExt
        .meta("file_path", path.to_string()) // Add metadata
}

fn process_data_yoshi(data: &str) -> YoshiResult<String> {
    if data.is_empty() {
        // Use Yoshi Kind for a structured error
        return Err(Yoshi::new(YoshiKind::Validation {
            field: "data".into(),
            message: "Data cannot be empty".into(),
            expected: Some("non-empty string".into()),
            actual: Some("empty string".into()),
        }));
    }
    Ok(format!("Processed: {}", data))
}

fn run_new_app() -> YoshiResult<String> {
    let data = parse_file_yoshi("non_existent.txt")?;
    process_data_yoshi(&data)
}

// Demonstrating the yopost! macro for error creation
fn run_new_app_with_macro() -> YoshiResult<String> {
    let data = parse_file_yoshi("non_existent.txt")
        .map_err(|e| yopost!(error: e, with_signpost = "Ensure file exists"))?; // Wrap and add suggestion

    // Using Yoshi macro for structured validation error
    if data.is_empty() {
        return Err(yopost!(kind: YoshiKind::Validation {
            field: "data".into(),
            message: "Data cannot be empty".into(),
            expected: Some("non-empty string".into()),
            actual: Some("empty string".into()),
        }));
    }
    Ok(format!("Processed: {}", data))
}

#### ✅ Example: Direct API-Based Error Creation

```rust
// Demonstrating direct API usage for structured error
fn run_new_app_with_api() -> YoshiResult<String> {
    let data_res: YoshiResult<String> = std::fs::read_to_string("non_existent.txt")
        .map_err(Yoshi::from) // Convert std::io::Error to Yoshi
        .context("Failed to read file for processing".to_string()); // Add context

    let data = data_res?;

    if data.is_empty() {
        return Err(Yoshi::new(YoshiKind::Validation {
            field: "data".into(),
            message: "Data cannot be empty".into(),
            expected: Some("non-empty string".into()),
            actual: Some("empty string".into()),
        }));
    }
    Ok(format!("Processed: {}", data))
}
```

> **Note:** `anyhow` is still excellent for lightweight projects. Migration to Yoshi is most beneficial when structured diagnostics, telemetry, or enterprise-grade observability are priorities.

```text

#### ✅ Example: anyhow Macro-Based Error Creation

```rust
// Demonstrating how to use the yopost! macro with anyhow errors
fn run_new_app_anyhow_macro() -> YoshiResult<String> {
    let data = std::fs::read_to_string("non_existent.txt")
        .map_err(Yoshi::from)?
        .context("Failed to read file for processing".to_string());

    // Using yopost! macro to add context and suggestions
    yopost!(error: data, with_signpost = "Check if the file exists and is accessible")
}
```

### 3. Migrating from `eyre`

`eyre` (similar to `anyhow`) offers powerful reporting with `eyre::Report`. Migration involves replacing `Report` with `Yoshi` and adapting context and macro usage.

**Key Changes:**

* Replace `eyre::Report` with `Yoshi`.
* Replace `eyre::Result<T>` with `yoshi::Result<T>`.
* Replace `eyre::eyre!` with `yopost!`.
* Replace `.wrap_err()` or `.context()` (`eyre`) with `Yoshi::context()` or `HatchExt::context()`.
* `eyre`'s `attach()` for structured data can be replaced by Yoshi's `with_metadata()` or `with_shell()`.

**Example: `eyre` to Yoshi**

```rust
// Before (eyre)
use eyre::{eyre, Result};
use std::io::{self, ErrorKind};

fn fetch_user_old(id: u32) -> Result<String> {
    if id == 0 {
        return Err(eyre!("Invalid user ID: {}", id).suggestion("Provide a positive ID"));
    }
    // Simulate a network error
    Err(io::Error::new(ErrorKind::ConnectionReset, "Network connection reset").into())
}

fn load_profile_old(user_id: u32) -> Result<String> {
    fetch_user_old(user_id)
        .wrap_err_with(|| format!("Failed to load profile for user {}", user_id))
}

// After (Yoshi)
use yoshi::{yoshi, Result as YoshiResult, Yoshi, YoshiKind, HatchExt};
use std::io::{self, ErrorKind}; // For compatibility with original error types

fn fetch_user_yoshi(id: u32) -> YoshiResult<String> {
    if id == 0 {
        // Use yopost! macro for structured validation error with suggestion
        return Err(yopost!(kind: YoshiKind::Validation {
            field: "user_id".into(),
            message: format!("Invalid user ID: {}", id).into(),
            expected: Some("positive integer".into()),
            actual: Some(id.to_string().into()),
        },
        with_signpost = "Provide a positive ID.")
        );
    }
    // Simulate a network error, converting to Yoshi
    Err(io::Error::new(ErrorKind::ConnectionReset, "Network connection reset").into())
}

fn load_profile_yoshi(user_id: u32) -> YoshiResult<String> {
    fetch_user_yoshi(user_id)
        .context(format!("Failed to load profile for user {}", user_id).to_string()) // Add context
        .meta("user_id", user_id.to_string()) // Add metadata for user_id
}

// Demonstrating yopost! macro for context
fn load_profile_yoshi_with_macro(user_id: u32) -> YoshiResult<String> {
    fetch_user_yoshi(user_id)
        .map_err(|e| yopost!(error: e, with_metadata = ("user_id", user_id.to_string()))) // Wrap and add metadata
        .context(format!("Failed to load profile for user {}", user_id).to_string())
}

// Demonstrating direct API usage with derived errors for direct error creation
fn load_profile_yoshi_with_api(user_id: u32) -> YoshiResult<String> {
    fetch_user_yoshi(user_id)
        .map_err(|e| e.with_metadata("user_id", user_id.to_string())) // Directly use with_metadata on Yoshi
        .context(format!("Failed to load profile for user {}", user_id).to_string())
}
```

### 4. Migrating from `snafu`

`snafu` focuses on explicit context selection and structured error definitions, making it conceptually closer to Yoshi. The migration primarily involves translating `snafu` error definitions to `yoshi-derive` and adapting context selectors.

**Key Changes:**

* Replace `#[derive(Snafu)]` with `#[derive(YoshiError)]`.
* Translate `Snafu` struct fields/attributes to `#[yoshi(display = "")]`, `#[yoshi(source)]`, `#[yoshi(context = "")]`, `#[yoshi(shell)]`.
* Explicitly map variants to `YoshiKind` using `#[yoshi(kind = "...")]`.
* `Snafu`'s context selectors (e.g., `MySnafuError { msg: "...", other_field }.fail()`) become `Yoshi`'s constructor followed by `context()`, `with_metadata()`, etc.
* Replace `snafu::Result<T, E>` with `yoshi::Result<T, E>`.

**Example: `snafu` to Yoshi**

```rust
// Before (snafu)
use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum OldSnafuError {
    #[snafu(display("Unable to read config from {path}: {source}"))]
    ReadConfig {
        path: String,
        source: std::io::Error,
    },
    #[snafu(display("User {user_id} not authorized for {action}"))]
    Unauthorized {
        user_id: u32,
        action: String,
    },
}

type OldResult<T, E = OldSnafuError> = std::result::Result<T, E>;

fn get_config_old(path: &str) -> OldResult<String> {
    std::fs::read_to_string(path)
        .context(ReadConfig { path: path.to_string() })
}

fn check_auth_old(user_id: u32, action: &str) -> OldResult<()> {
    if user_id == 0 {
        Unauthorized { user_id, action: action.to_string() }.fail()
    } else {
        Ok(())
    }
}

// After (Yoshi)
use yoshi_derive::YoshiError;
use yoshi::{yoshi, Result as YoshiResult, Yoshi, YoshiKind, HatchExt};
use std::io; // For std::io::Error

#[derive(YoshiError, Debug)]
#[yoshi(error_code_prefix = "APP")]
pub enum NewYoshiError {
    #[yoshi(display = "Unable to read config from {config_path}: {source_err}")]
    #[yoshi(kind = "Config")]
    #[yoshi(error_code = 201)]
    ReadConfig {
        #[yoshi(context = "file_path")] // Adds to Nest metadata
        config_path: String,
        #[yoshi(source)] // Marks as source
        source_err: io::Error,
    },
    #[yoshi(display = "User {user_id} not authorized for {action}")]
    #[yoshi(kind = "Validation")] // Map to Validation for authorization failures
    #[yoshi(error_code = 202)]
    #[yoshi(severity = 70)]
    Unauthorized {
        #[yoshi(context = "user_identifier")] // Adds to Nest metadata
        user_id: u32,
        #[yoshi(context = "action_performed")]
        action: String,
    },
}

// Use YoshiResult for your application's Result type
type NewResult<T, E = Yoshi> = YoshiResult<T, E>;

fn get_config_new(path: &str) -> NewResult<String> {
    std::fs::read_to_string(path)
        .map_err(|e| NewYoshiError::ReadConfig {
            config_path: path.to_string(),
            source_err: e,
        }.into()) // Convert derived error to Yoshi
        .context(format!("Failed to retrieve config at {}", path).to_string()) // Add context
}

fn check_auth_new(user_id: u32, action: &str) -> NewResult<()> {
    if user_id == 0 {
        // Direct conversion of derived error
        Err(NewYoshiError::Unauthorized {
            user_id,
            action: action.to_string(),
        }.into())
        .context(format!("Authorization check failed for action: {}", action).to_string())
    } else {
        Ok(())
    }
}

// Demonstrating the yopost! macro with derived errors for direct error creation
fn check_auth_new_with_macro(user_id: u32, action: &str) -> YoshiResult<()> {
    if user_id == 0 {
        Err(yopost!(kind: NewYoshiError::Unauthorized {
            user_id: user_id,
            action: action.to_string(),
        }.into())) // Use .into() to convert derived error to Yoshi
        .context(format!("Authorization check failed for action: {}", action).to_string())
    } else {
        Ok(())
    }
}

// Demonstrating direct API usage with derived errors for direct error creation
fn check_auth_new_with_api(user_id: u32, action: &str) -> YoshiResult<()> {
    if user_id == 0 {
        Err(Yoshi::new(NewYoshiError::Unauthorized {
            user_id: user_id,
            action: action.to_string(),
        }.into())) // Direct conversion using From trait
        .context(format!("Authorization check failed for action: {}", action).to_string())
    } else {
        Ok(())
    }
}
```

#### ✅ Example: Explicit API-Based Error Creation

```rust
// Using Yoshi's API directly for error creation
use yoshi::{Yoshi, YoshiKind, HatchExt};

fn check_auth_api(user_id: u32, action: &str) -> YoshiResult<()> {
    if user_id == 0 {
        // Manually constructing a Yoshi error
        let error = Yoshi::new(YoshiKind::Validation {
            field: "user_id".into(),
            message: "User ID cannot be zero".into(),
            expected: Some("non-zero integer".into()),
            actual: Some("zero".into()),
        })
        .with_signpost("Provide a valid user ID.".to_string())
        .with_metadata("action", action.to_string());

        return Err(error.context("Authorization failed".to_string()));
    }

    // Simulate successful authorization
    Ok(())
}
```

### 🧭 When to Use Macro vs API?

```markdown
| Use Case                        | Recommended Syntax  | Example |
|---------------------------------|---------------------|---------|
| Quick one-liner                 | `yopost!(...)`       | `yopost!(message: "Quick error")` |
| Multi-field struct conversion   | `Yoshi::new(...)`   | `Yoshi::new(MyError::ConfigLoad { ... }.into())` |
| Custom derived enum conversion | `.into()` then `.context(...)` | `MyError::UserNotFound { ... }.into()` |
| Result context addition | `HatchExt::context()` | `result.context("Operation failed")` |
```

## Leveraging Advanced Yoshi Features During Migration

As you migrate, consider integrating Yoshi's advanced capabilities:

* **Typed Shells**: Instead of just strings, attach any `Any + Send + Sync + 'static` type to a `Nest` using `with_shell()`. This is invaluable for structured debugging or passing recovery-related data.

```rust
    #[derive(Debug, Clone, PartialEq)]
    struct TransactionData { id: String, amount: f64 }

    let err = yopost!(message: "Transaction failed",
        with_shell = TransactionData { id: "tx_123".to_string(), amount: 100.0 })
        .with_priority(255); // Mark as critical

    // Later, retrieve the shell:
    if let Some(data) = err.shell::<TransactionData>() {
        println!("Transaction ID from shell: {}", data.id);
    }
```

* **Error Recovery Strategies**: Define `enum`s for recovery strategies and attach them as payloads.

```rust
    use std::time::Duration; // Required for Duration

    #[derive(Debug, Clone, PartialEq)]
    enum RecoveryHint {
        Retry(Duration),
        ManualIntervention(String),
    }

    let err = yopost!(kind: YoshiKind::Network {
        message: "External service down".into(),
        source: None, error_code: Some(503),
    },
    with_shell = RecoveryHint::Retry(Duration::from_secs(5)))
    .with_signpost("Attempt retry after a short delay.".to_string()); // .to_string() for Into<String>

    if let Some(hint) = err.shell::<RecoveryHint>() {
        match hint {
            RecoveryHint::Retry(d) => println!("Auto-retry after {:?}", d),
            _ => println!("Unhandled recovery hint: {:?}", hint),
        }
    }
```

* **Customizable Location Capture**: `YoshiLocation` is `Copy` and very lightweight, making it cheap to store. The `yoshi_location!` macro (internal to `yopost!`) automatically captures `file!`, `line!`, `column!`.

* **Performance Monitoring**: Yoshi provides global counters for error instances and other metrics through the memory module, valuable for profiling in long-running applications.

## Conclusion

Migrating to Yoshi can significantly enhance the debuggability, observability, and programmatic handling of errors in your Rust applications. While it requires adapting to Yoshi's structured approach, the benefits in terms of clarity, performance, and advanced features make it a worthwhile investment for robust and maintainable systems. Start by migrating your custom error definitions with `yoshi-derive`, and then progressively adapt your error creation and propagation logic to leverage the `yopost!` macro and `HatchExt` for a seamless transition.

> **Note:** `snafu` is still excellent for lightweight projects. Migration to Yoshi is most beneficial when structured diagnostics, telemetry, or enterprise-grade observability are priorities.
