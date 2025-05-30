# Troubleshooting the Yoshi Error Framework

This document provides solutions to common issues and debugging tips when working with the Yoshi error handling framework. Yoshi is designed for robustness and performance, but like any complex library, understanding its nuances can streamline your development process.

## General Troubleshooting Steps

Before diving into specific issues, consider these general steps:

1.  **Verify Feature Flags**: Ensure you have enabled the necessary features in your `Cargo.toml`.
    *   `std`: For standard library features (I/O errors, backtraces, `std::collections::HashMap`, etc.).
    *   `derive`: To use the `#[derive(YoshiError)]` procedural macro.
    *   `serde`: For `serde::Serialize` and `serde::Deserialize` implementations on `YoshiContext`.
    *   `tracing`: For integration with the `tracing` crate.
    *   `unstable-metrics`: For advanced performance metrics collection (may require nightly Rust or specific target architectures).
    *   `simd-optimized`: For SIMD-accelerated string processing (requires `x86_64` target and appropriate Rust compilation flags).

2.  **Check Yoshi Version**: Make sure your `yoshi`, `yoshi-std`, and `yoshi-derive` crate versions are compatible and up-to-date.

3.  **Read Compiler Errors Carefully**: Rust's compiler diagnostics are usually very informative. Pay close attention to suggested fixes and `note:` lines.

4.  **Consult Documentation**: Refer to the crate's `docs.rs` documentation for detailed API usage.

## Common Compilation Errors and Solutions

### 1. `no method named ... found` or `trait not in scope`

**Problem:** You're trying to use a method like `.context()`, `.meta()`, `.help()`, or `.with_payload()` on a `Result` type, but the compiler complains the method doesn't exist.

**Reason:** These methods are provided by the `YoshiContextExt` trait, which needs to be in scope.

**Solution:** Import the `YoshiContextExt` trait:

```rust
use yoshi::YoshiContextExt; // For your facade crate usage
// or
use yoshi_std::YoshiContextExt; // For direct yoshi-std usage

2. mismatched types in YoshiKind fields

Problem: When constructing a YoshiKind variant (e.g., YoshiKind::Network), the compiler reports type mismatches for fields like message, source, error_code, etc.

Reason: YoshiKind fields typically expect Arc<str>, Option<Box<Yoshi>>, or specific primitive types (u32, Duration). You might be providing a String, &str, or an incompatible type.

Solution: Ensure correct types and use .into() where appropriate for Arc<str> conversion.

```rust
// Incorrect
// let err = Yoshi::new(YoshiKind::Network {
// message: "Connection failed", // &str needs .into()
// source: my_io_error, // std::io::Error needs to be converted to Yoshi and Boxed
// error_code: "HTTP_500", // String literal needs to be parsed or converted to u32
// });

// Correct
use yoshi_std::{Yoshi, YoshiKind};
use std::io;

let my_io_error = io::Error::new(io::ErrorKind::ConnectionRefused, "socket error");
let err = Yoshi::new(YoshiKind::Network {
message: "Connection failed".into(), // Convert &str to Arc<str>
source: Some(Box::new(Yoshi::from(my_io_error))), // Convert io::Error to Yoshi, then box it
error_code: Some(500), // Provide as u32
});
```

3. the trait bound ... is not satisfied for Send/Sync/'static

Problem: You're trying to use a custom error type as a source for YoshiKind::Foreign or attach a custom struct as a payload, but the compiler complains about missing Send, Sync, or 'static bounds.

Reason: For thread safety and long-term storage, Yoshi requires Error sources and Any payloads to be Send + Sync + 'static. Your custom type might not implicitly satisfy these or might contain non-Send/Sync fields (e.g., Rc, RefCell).

Solution:

For Custom Error Types as Sources: Ensure your custom error struct or enum derives Send and Sync (if applicable) and its fields are also Send + Sync + 'static.

For Payloads: Ensure the type you're passing to with_payload() is Send + Sync + 'static. If it's not, you might need to wrap it in Arc or Mutex if sharing across threads, or consider if it truly needs to be part of the error payload.

```rust
// Assume MyCustomData is NOT Send + Sync + 'static
// struct MyCustomData { non_send_field: std::rc::Rc<()> } // Example

// Incorrect usage if MyCustomData doesn't meet bounds:
// let error = Yoshi::new(YoshiKind::Internal { message: "Internal".into(), source: None, component: None })
// .with_payload(MyCustomData { non_send_field: std::rc::Rc::new(()) }); // Compile error

// If it's genuinely needed, consider its design. If it's just for debugging, Debug impl might be enough.
```

4. Macro-related errors (yoshi!, #[derive(YoshiError)])

Problem: macro-error: unexpected token in input, no rules expected ... or missing derive implementations.

Reason:

yoshi! macro: Incorrect argument format (e.g., passing with_metadata = "key", "value" instead of ("key", "value")), or attempting to chain methods not supported by the macro's apply_attr rule.

#[derive(YoshiError)]: Missing yoshi-derive feature in Cargo.toml, or syntax errors in the #[yoshi(...)] attributes (e.g., invalid kind string, missing fields in display format string).

Solution:

yoshi! macro:

Ensure attribute values are tuples for with_metadata: with_metadata = ("key", "value").

Ensure string arguments are &str or String: with_suggestion = "my suggestion" or my_string_var.to_string().

Review the macro's documentation for exact syntax.

#[derive(YoshiError)]:

Add yoshi-derive = { version = "...", optional = true } and derive = ["yoshi-derive"] to your crate's Cargo.toml dependencies, and enable the derive feature for yoshi.

Double-check attribute syntax (e.g., #[yoshi(display = "{field_name}")]).

Ensure all placeholders in display strings correspond to actual fields in the enum variant.

```toml

Cargo.toml

[dependencies]
yoshi = { version = "0.1", features = ["derive", "std"] } # Enable the derive feature
yoshi-std = { version = "0.1", optional = true } # Optional, if you use yoshi-std directly
yoshi-derive = { version = "0.1", optional = true } # Optional, if you use yoshi-derive directly

### 5. `type annotations needed` for `Arc<str>` in HashMap lookups

**Problem:** When retrieving values from `YoshiContext.metadata` (which is `HashMap<Arc<str>, Arc<str>>`), the compiler might ask for type annotations, especially if you try to `get()` with `&str`.

**Reason:** `HashMap::get` takes a `Q: ?Sized + Hash + Eq` where `K: Borrow<Q>`. While `Arc<str>` implements `Borrow<str>`, the compiler sometimes needs help inferring this.

**Solution:** Explicitly convert your lookup key to `Arc<str>` using `.into()` or use `&Arc::from("key")` for the lookup.

\```rust
use yoshi_std::YoshiContext;
use std::sync::Arc;

let mut ctx = YoshiContext::new("Test context");
ctx = ctx.with_metadata("user_id", "123");

// Corrected lookup
let user_id = ctx.metadata.get(&Arc::from("user_id")).map(|s| s.as_ref());
println!("User ID: {:?}", user_id);
\```

## Common Runtime Issues and Debugging Tips

### 1. Errors not displaying as expected

**Problem:** Your formatted error output (`println!("{}", err)`) doesn't show all the context, metadata, or structured details you added.

**Reason:**
*   You might be using `{:?}` (Debug format) instead of `{}` (Display format). While Debug shows internal structure, Display is controlled by `fmt::Display` implementation and is designed for human readability.
*   Context messages, metadata, and suggestions are tied to `YoshiContext` objects. If they are not correctly added to the `Yoshi` error, they won't appear.
*   Some `YoshiKind` variants (like `Foreign` or `Io`) might display their source directly, which can sometimes overshadow additional context unless formatted specifically.

**Solution:**
*   Always use `println!("{}", my_error)` for the user-friendly formatted output.
*   Ensure you are chaining `Yoshi::context()`, `with_metadata()`, `with_suggestion()`, etc., correctly. Remember these methods apply to the `Yoshi` instance itself, which manages its internal `contexts` vector. Calling `my_yoshi_context.with_metadata()` on a standalone `YoshiContext` won't add it to a `Yoshi` error unless that context is then added to the `Yoshi` error. The convenience methods on `Yoshi` (e.g., `error.with_metadata(...)`) ensure the additions are correctly applied to the error's primary context.
*   Inspect `println!("{:?}", my_error)` to see the internal `contexts` vector and confirm data is present.

### 2. Backtraces are missing or incomplete

**Problem:** Your `Yoshi` errors don't include backtraces, even though you expect them.

**Reason:**
*   Backtrace capture is typically controlled by the `RUST_BACKTRACE` or `RUST_LIB_BACKTRACE` environment variable (`Yoshi` uses `std::backtrace::Backtrace` which respects these).
*   The `std` feature for `yoshi-std` might not be enabled, which is required for `std::backtrace::Backtrace`.
*   In `no_std` environments, full backtraces are not available; `YoshiBacktrace` in `no_std` mode provides minimal location info.

**Solution:**
*   **Enable `std` feature**: Make sure your `Cargo.toml` has `yoshi = { version = "...", features = ["std"] }`.
*   **Set `RUST_BACKTRACE`**: Before running your program, set `RUST_BACKTRACE=1` or `RUST_BACKTRACE=full`.
    *   `RUST_BACKTRACE=1`: basic backtrace.
    *   `RUST_BACKTRACE=full`: more verbose backtrace (with more symbols).
    *   `RUST_LIB_BACKTRACE`: similar to `RUST_BACKTRACE` but specifically for library tracing.
*   **Production vs. Development**: Remember that Yoshi's `YoshiBacktrace` capture is designed to be zero-cost in production unless explicitly enabled via these environment variables, to avoid performance overhead.

### 3. Unexpected performance regressions or high memory usage

**Problem:** After integrating Yoshi, your application experiences slower error handling or increased memory footprint.

**Reason:**
*   **Excessive allocations**: While Yoshi uses string interning and `Arc<str>` to reduce allocations, frequent creation of unique long strings or deep context chains can still cause overhead.
*   **Backtrace capture**: If `RUST_BACKTRACE` is inadvertently enabled in a performance-critical benchmark or production environment, backtrace capture is a significant overhead.
*   **Debugging features**: Using `Debug` formatting in performance-sensitive logging can be slower than `Display` format.
*   **SIMD optimization**: If `simd-optimized` is enabled, ensure your build environment supports and uses AVX2 instructions (`RUSTFLAGS="-C target-cpu=native"` or similar).

**Solution:**
*   **Profile your application**: Use Rust's built-in `perf` tools or external profilers to pinpoint bottlenecks.
*   **Manage `RUST_BACKTRACE`**: Always ensure `RUST_BACKTRACE` is unset or `0` in performance-sensitive environments.
*   **Optimize strings**: For highly repetitive string data in contexts/kinds, ensure you're leveraging Yoshi's `intern_string()` utility directly where applicable, or relying on `Arc<str>` conversions.
*   **Limit context depth**: While Yoshi has cycle detection, extremely deep context chains are inherently more expensive to traverse and format. Design your error propagation to keep context chains concise.
*   **Disable unstable features**: If `unstable-metrics` or `simd-optimized` are not actively beneficial for your specific performance goals, consider disabling them to reduce potential overheads from their generated code paths.

### 4. `YoshiKind` mapping logic in `yoshi-derive` not as expected

**Problem:** Your derived error variants don't map to the `YoshiKind` you intend, or auto-inference behaves unexpectedly.

**Reason:** Yoshi's derive macro uses a set of auto-inference rules based on variant names and field types. If you don't explicitly specify `#[yoshi(kind = "...")]`, it will infer one. If your variant name is ambiguous (e.g., "GeneralError"), it might default to `Foreign` or `Internal`.

**Solution:**
*   **Explicit is best**: Always use `#[yoshi(kind = "MyDesiredKind")]` to explicitly define the `YoshiKind` for each variant. This overrides auto-inference.
*   **Review auto-inference rules**: Consult the `yoshi-derive` documentation for how inference works if you rely on it.
*   **Inspect generated code**: For complex derives, you can inspect the code generated by procedural macros. Use `cargo expand` or `cargo rustc -- -Zunpretty=expanded` to see the actual Rust code that the derive macro produces. This can reveal unexpected mappings or missing implementations.

## Integration-Specific Troubleshooting

### 1. `no_std` Environments

**Problem:** My `no_std` project isn't compiling with Yoshi.

**Reason:** The `std` feature is likely implicitly enabled. Yoshi's default features often include `std`.

**Solution:**
*   Explicitly disable default features for Yoshi in your `Cargo.toml`:
    \```toml
    # Cargo.toml
    [dependencies]
    yoshi = { version = "0.1", default-features = false }
    yoshi-std = { version = "0.1", default-features = false } # If using yoshi-std directly
    yoshi-derive = { version = "0.1", default-features = false } # If using yoshi-derive directly
    # Also ensure any other features you NEED (like 'derive') are re-enabled
    yoshi = { version = "0.1", default-features = false, features = ["derive"] }
    \```
*   In `no_std`, `std::io::Error` is replaced by `yoshi_std::NoStdIo`. Ensure your code adapts accordingly for `Io` kinds.
*   Yoshi provides `SystemTime` and `ThreadId` replacements for `no_std` environments. Their behavior is different (e.g., `SystemTime::now()` is monotonic, not wall-clock).

### 2. `serde` Integration

**Problem:** `Yoshi` errors cannot be serialized/deserialized, or some fields are missing.

**Reason:**
*   The `serde` feature is not enabled for `yoshi-std`.
*   `YoshiContext` contains fields that are intentionally skipped from serialization (`location`, `payloads`) because they are not easily serializable or are meant for runtime introspection only.

**Solution:**
*   Enable the `serde` feature in your `Cargo.toml`:
    \```toml
    # Cargo.toml
    [dependencies]
    yoshi = { version = "0.1", features = ["serde", "std"] }
    # Or specifically for yoshi-std if used directly:
    yoshi-std = { version = "0.1", features = ["serde"] }
    \```
*   Understand that `location` and `payloads` fields in `YoshiContext` are marked `#[serde(skip)]` by design. If you need to serialize custom payloads, you'll need to extract them manually and serialize them separately.

### 3. `tracing` Integration

**Problem:** `Yoshi`'s `make_event()` or `create_span()` methods are not available or not logging.

**Reason:**
*   The `tracing` feature is not enabled for `yoshi-std`.
*   A `tracing` subscriber is not initialized in your application's `main` function or test setup.

**Solution:**
*   Enable the `tracing` feature in `Cargo.toml`:
    \```toml
    # Cargo.toml
    [dependencies]
    yoshi = { version = "0.1", features = ["tracing", "std"] }
    # Or specifically for yoshi-std:
    yoshi-std = { version = "0.1", features = ["tracing"] }
    tracing = "0.1" # Also add tracing dependency
    tracing-subscriber = "0.3" # For example, to enable logging
    \```
*   Initialize a tracing subscriber at the start of your application:
    \```rust
    // In your main.rs or lib.rs
    #[cfg(feature = "tracing")]
    fn setup_tracing() {
        tracing_subscriber::fmt::init();
    }

    fn main() {
        #[cfg(feature = "tracing")]
        setup_tracing();

        // Your application logic
    }
    \```

## Debugging Yoshi Errors

Yoshi provides powerful introspection tools for debugging complex error scenarios:

1.  **`println!("{}", error)` vs. `println!("{:?}", error)`**:
    *   `{}` (Display): Provides a human-readable, formatted error message designed for end-users or logs. It prioritizes clarity and follows the `Display` trait implementation.
    *   `{:?}` (Debug): Prints the raw internal structure of the `Yoshi` struct, including all `YoshiKind` fields, the entire `contexts` vector, and `backtrace` (if captured). This is invaluable for developers debugging error propagation.

2.  **`Yoshi::instance_id()`**: Each `Yoshi` error gets a unique `u64` ID. This is extremely useful for correlating specific error instances across distributed logs or concurrent systems.

3.  **`Yoshi::kind()`**: Access the root `YoshiKind` enum to programmatically inspect the high-level error classification and its structured fields.

4.  **`Yoshi::contexts()` and `Yoshi::primary_context()`**:
    *   `error.contexts()`: Returns an iterator over all `YoshiContext` objects attached to the error, in the order they were added (or reverse for display order).
    *   `error.primary_context()`: Returns the `YoshiContext` with the highest `priority`, or the most recently added if priorities are equal. This is often the most relevant context for a given error.

5.  **`YoshiContext::metadata`**: Access the `HashMap<Arc<str>, Arc<str>>` to inspect all key-value metadata attached to a specific context.

6.  **`Yoshi::payload::<T>()` or `YoshiContext::payload::<T>()`**: Retrieve custom typed data attached as payloads. This is crucial for passing structured debugging information or recovery instructions.

7.  **`Yoshi::analyze_context()`**: Returns a `ContextAnalysis` struct with aggregated statistics about the error's contexts (total contexts, depth, metadata entries, etc.).

By combining these tools, you can effectively trace the origin, path, and detailed state of any error within your Yoshi-powered application.
IGNORE_WHEN_COPYING_START
content_copy
download
Use code with caution.
IGNORE_WHEN_COPYING_END