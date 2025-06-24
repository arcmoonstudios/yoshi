# Troubleshooting the Yoshi Error Framework

This document provides solutions to common issues and debugging tips when working with the Yoshi error handling framework.

## General Troubleshooting Steps

Before diving into specific issues, consider these general steps:

1. **Verify Feature Flags**: Ensure you have enabled the necessary features in your `Cargo.toml`.
   * `std`: For standard library features (backtraces, std::io integration, etc.).
   * `derive`: To use the `yoshi_af!` macro and procedural macros.
   * `serde`: For `serde::Serialize` and `serde::Deserialize` implementations.
   * `full`: For complete feature set (recommended for most users).

2. **Check Yoshi Version**: Make sure your `yoshi`, `yoshi-core`, `yoshi-std`, `yoshi-derive`, and `yoshi-deluxe` crate versions are compatible and up-to-date.

3. **Read Compiler Errors Carefully**: Rust's compiler diagnostics are usually very informative. Pay close attention to suggested fixes and `note:` lines.

4. **Consult Documentation**: Refer to the crate's `docs.rs` documentation for detailed API usage.

5. **Use the `yum!` macro**: For comprehensive error analysis and debugging output.

## Common Compilation Errors and Solutions

### 1. `no method named ... found` or `trait not in scope`

**Problem:** You're trying to use a method like `.lay()` on a `Result` type, but the compiler complains the method doesn't exist.

**Reason:** The `.lay()` method is provided by importing the yoshi prelude.

**Solution:** Import the yoshi prelude:

```rust
use yoshi::*; // Recommended - imports everything needed

// The .lay() method is now available:
fn example() -> Hatch<String> {
    some_operation()
        .lay("Additional context")?;
    Ok("success".to_string())
}
```

### 2. Mismatched types in YoshiKind fields

**Problem:** When constructing a YoshiKind variant (e.g., YoshiKind::Network), the compiler reports type mismatches for fields like message, source, error_code, etc.

**Reason:** YoshiKind fields typically expect `Arc<str>`, `Option<Box<Yoshi>>`, or specific primitive types (u32, Duration). You might be providing a String, &str, or an incompatible type.

**Solution:** Ensure correct types and use .into() where appropriate for `Arc<str>` conversion.

```rust
use yoshi::*;

// Correct usage with proper type conversions
let err = yopost!(kind: YoshiKind::Network {
    message: "Connection failed".into(), // Convert &str to Arc<str>
    source: None, // Or Some(Box::new(other_yoshi_error))
    error_code: Some(500), // Provide as u32
});

// Alternative using the yopost! macro for simpler syntax
let err = yopost!(message: "Connection failed")
    .with_metadata("error_code", "500")
    .with_signpost("Check network connectivity");
```

### 3. Send/Sync/'static trait bound errors

**Problem:** You're trying to use a custom error type as a source or attach a custom struct, but the compiler complains about missing Send, Sync, or 'static bounds.

**Reason:** For thread safety and long-term storage, Yoshi requires Error sources and payloads to be Send + Sync + 'static.

**Solution:** Ensure your custom types satisfy these bounds:

```rust
use yoshi::*;

// Ensure your custom types are Send + Sync + 'static
#[derive(Debug, Clone)]
struct MyCustomData {
    value: String, // String is Send + Sync + 'static
}

// This will work
let error = yopost!(message: "Custom error")
    .with_metadata("custom_data", "some value");
```

### 4. Macro-related errors (yopost!, yoshi_af!)

**Problem:** `macro-error: unexpected token in input`, `no rules expected ...` or missing derive implementations.

**Reason:**

* **yopost! macro**: Incorrect argument format or attempting to chain methods not supported by the macro.
* **yoshi_af! macro**: Missing yoshi-derive feature in Cargo.toml, or syntax errors in the `#[yoshi(...)]` attributes.

**Solution:**

**yopost! macro:**

* Ensure attribute values are tuples for with_metadata: `with_metadata = ("key", "value")`.
* Ensure string arguments are &str or String: `with_signpost = "my suggestion"`.
* Use correct syntax patterns:

```rust
use yoshi::*;

// Correct yopost! macro usage
let error = yopost!(message: "Something went wrong");
let error = yopost!(kind: YoshiKind::Network {
    message: "Connection failed".into(),
    source: None,
    error_code: Some(503),
});
let error = yopost!(
    message: "Database error",
    with_metadata = ("host", "localhost"),
    with_signpost = "Check database connection"
);
```

**yoshi_af! macro:**

* Enable the derive feature in your Cargo.toml:

```toml
[dependencies]
yoshi = { version = "0.1", features = ["derive", "std"] }
```

* Use correct yoshi_af! syntax:

```rust
use yoshi::*;

yoshi_af! {
    #[derive(Debug)]
    pub enum MyError {
        #[yoshi(display = "User {user_id} not found")]
        UserNotFound { user_id: u64 },
    }
}
```

### 5. Using the `yum!` macro for debugging

**Problem:** You want to get comprehensive error information for debugging purposes.

**Solution:** Use the `yum!` macro for enhanced error output:

```rust
use yoshi::*;

fn example_function() -> Hatch<String> {
    // Some operation that might fail
    Err(yopost!(kind: YoshiKind::Network {
        message: "Connection timeout".into(),
        source: None,
        error_code: Some(408),
    })
    .with_metadata("host", "api.example.com")
    .with_signpost("Check network connectivity"))
}

fn main() {
    match example_function() {
        Ok(result) => println!("Success: {}", result),
        Err(error) => {
            yum!(error);  // Comprehensive error analysis output
        }
    }
}
```

## Common Runtime Issues and Debugging Tips

### 1. Errors not displaying as expected

**Problem:** Your formatted error output doesn't show all the context, metadata, or structured details you added.

**Reason:**

* You might be using `{:?}` (Debug format) instead of `{}` (Display format).
* Context messages and metadata might not be properly attached to the error.

**Solution:**

* Always use `println!("{}", my_error)` for user-friendly formatted output.
* Use the `yum!` macro for comprehensive error analysis:

```rust
use yoshi::*;

fn main() {
    let error = yopost!(message: "Something went wrong")
        .with_metadata("component", "database")
        .with_signpost("Check database connection");

    // Comprehensive error output
    yum!(error);

    // Or simple display
    println!("{}", error);
}
```

### 2. Backtraces are missing or incomplete

**Problem:** Your `Yoshi` errors don't include backtraces, even though you expect them.

**Reason:**

* Backtrace capture is controlled by the `RUST_BACKTRACE` environment variable.
* The `std` feature might not be enabled, which is required for backtrace support.
* In `no_std` environments, full backtraces are not available.

**Solution:**

* **Enable `std` feature**: Make sure your `Cargo.toml` has `yoshi = { version = "...", features = ["std"] }`.
* **Set `RUST_BACKTRACE`**: Before running your program, set `RUST_BACKTRACE=1` or `RUST_BACKTRACE=full`.
  * `RUST_BACKTRACE=1`: basic backtrace.
  * `RUST_BACKTRACE=full`: more verbose backtrace with symbols.
* **Production vs. Development**: Backtrace capture is designed to be zero-cost in production unless explicitly enabled.

### 3. Performance issues

**Problem:** After integrating Yoshi, your application experiences slower error handling or increased memory footprint.

**Reason:**

* Excessive allocations from frequent creation of unique long strings.
* Backtrace capture enabled in performance-critical environments.
* Using `Debug` formatting in performance-sensitive logging.

**Solution:**

* **Profile your application**: Use Rust's built-in profiling tools to pinpoint bottlenecks.
* **Manage `RUST_BACKTRACE`**: Ensure `RUST_BACKTRACE` is unset or `0` in performance-sensitive environments.
* **Optimize strings**: Leverage Yoshi's `Arc<str>` usage for string deduplication.
* **Limit context depth**: Keep context chains concise for better performance.

## Integration-Specific Troubleshooting

### 1. `no_std` Environments

**Problem:** My `no_std` project isn't compiling with Yoshi.

**Reason:** The `std` feature is likely implicitly enabled.

**Solution:**

* Explicitly disable default features for Yoshi in your `Cargo.toml`:

```toml
[dependencies]
yoshi-core = { version = "0.1", default-features = false }
# Use yoshi-core for no_std environments
```

* In `no_std`, use `yoshi-core` which provides the essential error handling without std dependencies.

### 2. `serde` Integration

**Problem:** `Yoshi` errors cannot be serialized/deserialized.

**Reason:** The `serde` feature is not enabled.

**Solution:**

* Enable the `serde` feature in your `Cargo.toml`:

```toml
[dependencies]
yoshi = { version = "0.1", features = ["serde", "std"] }
```

## Debugging Yoshi Errors

Yoshi provides powerful introspection tools for debugging complex error scenarios:

1. **`println!("{}", error)` vs. `println!("{:?}", error)`**:
   * `{}` (Display): Provides a human-readable, formatted error message.
   * `{:?}` (Debug): Prints the raw internal structure of the `Yoshi` struct.

2. **`yum!` macro**: Provides comprehensive error analysis with metadata, suggestions, and context.

3. **`Yoshi::kind()`**: Access the root `YoshiKind` enum to inspect the error classification.

4. **Error metadata**: Access attached metadata for debugging context.

By combining these tools, you can effectively trace the origin and detailed state of any error within your Yoshi-powered application.
