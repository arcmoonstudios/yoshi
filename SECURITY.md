# Security Considerations for the Yoshi Error Framework

The Yoshi error framework is designed with security and robustness in mind, particularly for high-stakes, performance-critical applications. This document outlines key security considerations, built-in safeguards, and best practices for using Yoshi securely in your projects.

## 1. Information Disclosure (Sensitive Data in Error Messages)

**Risk:** Error messages often contain details useful for debugging. However, in production environments, these details could inadvertently expose sensitive information (e.g., API keys, database credentials, user personal data, internal system paths, or exact software versions that reveal vulnerabilities).

**Yoshi's Safeguards:**

* **Runtime Sanitization (`is_production_mode()`):** Yoshi includes an internal `sanitize_error_message` function. This function performs basic string replacement (e.g., "[REDACTED]" for "password", "token", "key") and truncates very long messages (over 256 bytes) to prevent accidental data dumps.
  * This sanitization is activated if the `std` feature is enabled AND the `YOSHI_PRODUCTION_MODE` environment variable is set to `"1"` or `"true"`.
* **Explicit Control over `YoshiKind::Foreign`:** When wrapping external errors (`Yoshi::foreign()`), Yoshi captures the `type_name` of the original error. While this is useful for debugging, it's generally safe as type names are rarely sensitive. However, the original error's `Display` output *is* processed by `sanitize_error_message`.
* **Backtrace Redaction**: In `std` environments, `YoshiBacktrace` output is also automatically redacted (`[Backtrace sanitized for production]`) when `YOSHI_PRODUCTION_MODE` is active, preventing stack traces from revealing sensitive paths or internal logic.

**Best Practices:**

* **Always use `YOSHI_PRODUCTION_MODE` in Production:** Set this environment variable to `1` or `true` in your production deployments.
* **Avoid Sensitive Data in Error Messages**: Design your error messages (`message`, `suggestion`, `YoshiContext` messages) to be as generic as possible. If sensitive details are needed for debugging, attach them as **typed payloads** rather than directly in human-readable strings.
* **Leverage Typed Payloads for Internal Debugging**: Typed payloads are `#[serde(skip)]` by default. This means they are not serialized to JSON or other formats unless you explicitly extract and serialize them. This makes them safer for storing sensitive debug data that should only be accessed programmatically within the application or a secure debugger.
* **Custom Sanitization Hooks**: If Yoshi's default sanitization is insufficient, you can implement custom formatting logic or middleware that inspects and sanitizes error fields before logging or displaying.
* **Logging Levels**: Integrate Yoshi with `tracing` and adjust logging levels based on environment. `DEBUG` or `TRACE` level might include more verbose error details only in development.

## 2. Denial of Service (DoS) via Recursive Error Chains

**Risk:** An maliciously crafted or accidentally recursive error chain could lead to infinite recursion during formatting (`fmt::Display`) or traversal, causing stack overflows or excessive resource consumption (DoS).

**Yoshi's Safeguards:**

* **Bounded Recursion (`MAX_DEPTH`):** Yoshi's internal `format_source_chain_optimized` function explicitly limits the recursion depth (default `MAX_DEPTH = 32`). If the chain exceeds this, it truncates the output with a message like "... (error chain truncated at depth X for security)". This prevents stack overflows and excessive CPU cycles during formatting.
* **Internal Cycle Detection (Implicit):** While not a explicit cycle detection algorithm, the bounded recursion inherently mitigates infinite loops caused by circular references in error sources.

**Best Practices:**

* **Avoid Manual Circular References**: When constructing complex `Yoshi` errors manually (especially `YoshiKind::Internal` or `YoshiKind::Foreign` with `source`), ensure you do not create circular dependencies where an error's source directly or indirectly points back to itself.

## 3. Data Integrity and Type Safety

**Risk:** Mismatched data types or incorrect interpretation of error context could lead to logical errors or crashes.

**Yoshi's Safeguards:**

* **Structured `YoshiKind`:** By providing a structured `YoshiKind` enum with distinct fields for different error categories, Yoshi enforces type safety at the definition level. This reduces ambiguity compared to unstructured string errors.
* **Strongly-Typed Payloads:** `YoshiContext::payload::<T>()` relies on Rust's `Any` trait and `downcast_ref()`, which are type-safe. You can only retrieve a payload if its type matches the one you expect, preventing misinterpretations.
* **`Arc<str>` for Shared Strings:** Using `Arc<str>` for string fields in `YoshiKind` and `YoshiContext` ensures immutable, shared ownership, preventing accidental modification of error messages after creation.
* **Procedural Macro Validation (`yoshi-derive`):** The `yoshi-derive` macro performs compile-time validation of attribute usage and field mappings, catching many common errors before runtime. This includes checking placeholder usage in `display` formats and uniqueness of error codes.

**Best Practices:**

* **Define Custom Error Enums with `yoshi-derive`**: For application-specific errors, use `#[derive(YoshiError)]` to leverage compile-time checks and structured error definitions.
* **Use `YoshiKind` Appropriately**: Select the `YoshiKind` variant that best semantically describes your error. This aids in programmatic handling and clarity.
* **Validate Retrieved Payloads**: Always check the `Option` returned by `payload::<T>()` and handle `None` cases.

## 4. Resource Exhaustion (Memory/CPU)

**Risk:** Excessive error creation, deep context chains, or large payloads could consume too much memory or CPU, impacting application stability.

**Yoshi's Safeguards:**

* **String Interning**: Yoshi employs a global string interning pool (`StringInternPool`) for `Arc<str>` values in `YoshiKind` and `YoshiContext`. This dramatically reduces memory allocations for frequently repeated strings (e.g., common error messages, metadata keys).
* **`Arc` for Costly Clones:** `Yoshi` and `YoshiContext` use `Arc` for fields like `message`, `metadata` values, and `payloads`. Cloning a `Yoshi` error or `YoshiContext` creates shallow copies of these `Arc`s, significantly reducing memory and CPU overhead compared to deep cloning.
* **Conditional Backtrace Capture**: Backtraces are only captured when enabled by environment variables, preventing performance overhead in production by default.
* **`OptimizedFormatBuffer`**: The internal formatting logic uses a pre-allocated and intelligently growing `OptimizedFormatBuffer` to minimize reallocations during error display.
* **Memory Usage Statistics (`memory::get_memory_stats`):** If `unstable-metrics` is enabled, you can query global statistics on string interning hits/misses and estimated memory savings, helping to identify and address allocation hotspots.

**Best Practices:**

* **Monitor Error Rates**: Use `yoshi_std::error_instance_count()` and potentially `cross_process_metrics` (if `unstable-metrics` is enabled) to monitor the rate of error creation in your application. High rates might indicate underlying issues (e.g., retry loops, misconfigured external services).
* **Limit Context Depth (Implicitly)**: While Yoshi protects against recursion, avoid excessively deep manual context chains if not strictly necessary, as they still increase overall memory footprint per error instance.
* **Consider Clearing Intern Pool (Long-running applications)**: For extremely long-running services where unique strings might accumulate, consider occasionally calling `yoshi_std::memory::cleanup_intern_pool()` if `std` feature is enabled. This can help prevent memory leaks from unused interned strings, though it might introduce a brief performance hiccup.

## 5. Third-Party Dependency Vulnerabilities

**Risk:** While Yoshi aims to be secure, it relies on several Rust crates (e.g., `once_cell`, `serde`, `tracing`, `rayon` if used). Vulnerabilities in these dependencies could impact your application.

**Yoshi's Approach:**

* **Minimal Dependencies**: Yoshi strives for a minimal dependency footprint, especially for its `no_std` core.
* **Audited Dependencies**: Core dependencies are widely used and regularly audited by the Rust community.

**Best Practices:**

* **Regular Dependency Audits**: Use tools like `cargo audit` to regularly check for known vulnerabilities in your project's dependency tree, including Yoshi's transitive dependencies.
* **Keep Dependencies Updated**: Regularly update Yoshi and its dependencies to benefit from security patches and performance improvements.

## Conclusion

The Yoshi error framework provides a robust and secure foundation for error handling in Rust applications. By understanding its built-in safeguards and following best practices for information disclosure, resource management, and type safety, you can leverage Yoshi to build highly resilient and secure systems. Always prioritize careful design of error messages and rigorous testing in production-like environments.
