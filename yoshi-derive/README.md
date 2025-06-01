# Yoshi Derive - Advanced Error Handling Macros

[![Crates.io](https://img.shields.io/crates/v/yoshi-derive.svg)](https://crates.io/crates/yoshi-derive)
[![Documentation](https://docs.rs/yoshi-derive/badge.svg)](https://docs.rs/yoshi-derive)
[![License](https://img.shields.io/badge/license-BSL--1.1-blue.svg)](../LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.87%2B-orange.svg)](https://forge.rust-lang.org/releases.html)

**Enterprise-grade procedural macros for the Yoshi error handling framework with mathematical precision and performance optimization.**

---

## 🎯 **Overview**

`yoshi-derive` provides sophisticated derive macros and attribute processors that generate optimized error handling code with compile-time validation, performance hints, and intelligent error mapping strategies. Built on Rust 1.87's enhanced macro system with precise capturing in traits and stabilized intrinsics for optimal code generation.

### **Key Features**

- **🔥 Advanced AST Analysis** - O(n) complexity with intelligent memoization
- **⚡ Compile-time Validation** - Zero runtime cost with enhanced error reporting
- **🚀 Performance-optimized Code Generation** - SIMD-friendly patterns and cache optimization
- **🛡️ Type-safe Error Mapping** - Precise capturing and phantom type validation
- **🧠 Smart Contextual Analysis** - Dependency graph resolution for optimal error chains
- **📚 Enterprise-grade Documentation** - Comprehensive rustdoc coverage

### **Mathematical Properties**

- **Time Complexity**: O(V + A + F) where V=variants, A=attributes, F=fields
- **Space Complexity**: O(V) for variant analysis + O(A) for attribute cache
- **Code Generation**: O(1) amortized per variant through template-based expansion
- **Expected Performance**: <100ms compilation overhead for typical error enums (<50 variants)

---

## 🚀 **Quick Start**

Add to your `Cargo.toml`:

```toml
[dependencies]
yoshi-derive = "0.1.0"
yoshi = "0.1.0"
```

### **Basic Usage**

```rust
use yoshi_derive::YoshiError;
use std::path::PathBuf;

#[derive(Debug, YoshiError)]
pub enum MyAppError {
    #[yoshi(display = "Failed to parse config: {source}")]
    ParseError {
        #[yoshi(source)]
        source: std::io::Error,
        #[yoshi(context = "config_file")]
        path: String,
    },

    #[yoshi(display = "User not found: {user_id}")]
    #[yoshi(kind = "NotFound")]
    #[yoshi(severity = 60)]
    UserNotFound {
        user_id: u32,
        #[yoshi(context = "database_lookup")]
        #[yoshi(suggestion = "Check user ID in database")]
        attempted_query: String,
    },
}
```

---

## 📋 **Comprehensive Attribute Reference**

### **Container Attributes** (`#[yoshi(...)]` on enums)

| Attribute | Type | Description | Example |
|-----------|------|-------------|---------|
| `error_code_prefix` | `String` | Global prefix for error codes | `#[yoshi(error_code_prefix = "HTTP")]` |
| `default_severity` | `u8` | Default severity level (0-255) | `#[yoshi(default_severity = 75)]` |
| `performance_monitoring` | `bool` | Enable performance tracking | `#[yoshi(performance_monitoring = true)]` |
| `tracing_integration` | `bool` | Enable tracing support | `#[yoshi(tracing_integration = true)]` |

### **Variant Attributes** (`#[yoshi(...)]` on enum variants)

| Attribute | Type | Description | Example |
|-----------|------|-------------|---------|
| `display` | `String` | Custom display format string | `#[yoshi(display = "Error: {message}")]` |
| `kind` | `String` | Map to YoshiKind variant | `#[yoshi(kind = "Network")]` |
| `error_code` | `u32` | Unique error code | `#[yoshi(error_code = 1001)]` |
| `severity` | `u8` | Severity level (0-255) | `#[yoshi(severity = 80)]` |
| `transient` | `bool` | Mark as retryable error | `#[yoshi(transient = true)]` |
| `context` | `String` | Default context message | `#[yoshi(context = "Operation failed")]` |
| `suggestion` | `String` | Recovery suggestion | `#[yoshi(suggestion = "Check network")]` |

### **Field Attributes** (`#[yoshi(...)]` on struct fields)

| Attribute | Type | Description | Example |
|-----------|------|-------------|---------|
| `source` | Flag | Mark as error source | `#[yoshi(source)]` |
| `context` | `String` | Add to context metadata | `#[yoshi(context = "file_path")]` |
| `shell` | Flag | Add as typed shell | `#[yoshi(shell)]` |
| `skip` | Flag | Skip in Display formatting | `#[yoshi(skip)]` |
| `suggestion` | `String` | Field-level suggestion | `#[yoshi(suggestion = "Check file")]` |

---

## 🏗️ **Advanced Usage Examples**

### **Complete Error Enum with All Features**

```rust
use yoshi_derive::YoshiError;
use std::error::Error;
use yoshi_std::{Yoshi, YoshiKind};

#[derive(Debug, YoshiError)]
#[yoshi(error_code_prefix = "APP")]
#[yoshi(default_severity = 75)]
#[yoshi(performance_monitoring = true)]
#[yoshi(tracing_integration = true)]
pub enum AdvancedError {
    #[yoshi(error_code = 1001)]
    #[yoshi(display = "Critical system failure: {message}")]
    #[yoshi(kind = "Internal")]
    #[yoshi(severity = 255)]
    #[yoshi(context = "System critical error occurred")]
    #[yoshi(suggestion = "Contact system administrator immediately")]
    SystemFailure {
        message: String,
        #[yoshi(source)]
        cause: Box<dyn Error + Send + Sync + 'static>,
        #[yoshi(shell)]
        system_state: SystemState,
        #[yoshi(context = "timestamp")]
        occurred_at: String,
    },

    #[yoshi(error_code = 2001)]
    #[yoshi(display = "Database connection timeout")]
    #[yoshi(kind = "Timeout")]
    #[yoshi(severity = 120)]
    #[yoshi(transient = true)]
    DatabaseTimeout {
        #[yoshi(context = "connection_string")]
        host: String,
        port: u16,
        #[yoshi(shell)]
        connection_info: DatabaseInfo,
        #[yoshi(suggestion = "Check database connectivity")]
        attempted_duration: std::time::Duration,
    },

    #[yoshi(error_code = 3001)]
    #[yoshi(display = "Validation failed for field '{field}': {message}")]
    #[yoshi(kind = "Validation")]
    #[yoshi(severity = 40)]
    ValidationError {
        field: String,
        message: String,
        #[yoshi(context = "validation_rule")]
        rule: String,
        #[yoshi(shell)]
        submitted_value: serde_json::Value,
    },
}

#[derive(Debug)]
struct SystemState {
    memory_usage: f64,
    cpu_usage: f64,
    active_connections: u32,
}

#[derive(Debug)]
struct DatabaseInfo {
    host: String,
    port: u16,
    database_name: String,
    connection_pool_size: u32,
}
```

### **Generated Code Overview**

The derive macro automatically generates:

```rust
impl std::fmt::Display for AdvancedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SystemFailure { message, .. } => {
                write!(f, "Critical system failure: {}", message)
            }
            Self::DatabaseTimeout { .. } => {
                write!(f, "Database connection timeout")
            }
            Self::ValidationError { field, message, .. } => {
                write!(f, "Validation failed for field '{}': {}", field, message)
            }
        }
    }
}

impl std::error::Error for AdvancedError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::SystemFailure { cause, .. } => Some(cause.as_ref()),
            _ => None,
        }
    }
}

impl From<AdvancedError> for yoshi_std::Yoshi {
    fn from(error: AdvancedError) -> Self {
        match error {
            AdvancedError::SystemFailure { message, cause, system_state, occurred_at } => {
                let mut yoshi_err = Yoshi::new(YoshiKind::Internal {
                    message: message.into(),
                    source: Some(Box::new(Yoshi::from(*cause))),
                    component: Some("system".into()),
                });
                yoshi_err = yoshi_err.with_shell(system_state);
                yoshi_err = yoshi_err.with_metadata("timestamp", occurred_at);
                yoshi_err = yoshi_err.with_suggestion("Contact system administrator immediately");
                yoshi_err
            }
            // ... other variants
        }
    }
}
```

---

## 🧠 **Intelligent Auto-Inference System**

The derive macro includes sophisticated pattern recognition for automatic attribute inference:

### **Kind Inference by Variant Name**

- Names containing `timeout`, `expired` → `kind = "Timeout"`
- Names containing `network`, `connection`, `http` → `kind = "Network"`
- Names containing `not_found`, `missing` → `kind = "NotFound"`
- Names containing `internal`, `bug`, `panic` → `kind = "Internal"`
- Names containing `resource`, `limit`, `quota` → `kind = "ResourceExhausted"`

### **Field Type Analysis**

- `std::io::Error` → `source = true`
- `Box<dyn std::error::Error>` → `source = true`
- `reqwest::Error` → `source = true`
- Field names containing `path`, `file` → `context = "file_path"`
- Field names containing `url`, `uri` → `context = "endpoint"`
- Field names containing `user`, `id` → `context = "identifier"`

### **Display Format Inference**

- Single field variants get `display = "{variant_name}: {field}"`
- Multi-field variants get contextual formatting based on field names

---

## 🎨 **YoshiKind Mapping System**

The derive macro intelligently maps error variants to appropriate `YoshiKind` categories:

| Variant Kind | Generated YoshiKind | Typical Use Case |
|--------------|-------------------|------------------|
| `"Io"` | `YoshiKind::Io` | File system, I/O operations |
| `"Network"` | `YoshiKind::Network` | HTTP, TCP, networking errors |
| `"Config"` | `YoshiKind::Config` | Configuration parsing, validation |
| `"Validation"` | `YoshiKind::Validation` | Input validation, format checking |
| `"Internal"` | `YoshiKind::Internal` | Logic errors, invariant violations |
| `"NotFound"` | `YoshiKind::NotFound` | Resource lookup failures |
| `"Timeout"` | `YoshiKind::Timeout` | Operation timeouts |
| `"ResourceExhausted"` | `YoshiKind::ResourceExhausted` | Memory, connection limits |

---

## ⚡ **Performance Characteristics**

### **Compilation Performance**

- **Typical Enums** (<50 variants): <100ms overhead
- **Large Enums** (50-200 variants): <500ms overhead
- **Worst Case** (>200 variants): May approach 1-2s but with optimizations

### **Runtime Performance**

- **Error Creation**: O(1) - Zero runtime cost from macros
- **Display Formatting**: Depends on generated format strings
- **Context Attachment**: O(1) per context item
- **Shell Access**: O(1) hash map lookup

### **Memory Efficiency**

- Generated code uses static string literals where possible
- Minimal heap allocations during error creation
- Efficient struct layouts for variant data

---

## 🔍 **Compile-time Validation**

The derive macro performs extensive validation to catch errors early:

### **Structural Validation**

- ✅ Only one `#[yoshi(source)]` field per variant
- ✅ Valid identifier patterns for context keys
- ✅ Severity levels within valid ranges (0-255)
- ✅ Error codes are unique within enum
- ✅ Display format placeholders match field names

### **Performance Analysis**

- 🚀 Warnings for overly complex display formats
- 🚀 Hints for optimal field ordering
- 🚀 Suggestions for shell vs context usage
- 🚀 Detection of redundant attributes

### **Security Considerations**

- 🔒 Input sanitization for all user-provided strings
- 🔒 Validation of format string patterns
- 🔒 Prevention of code injection through attributes

---

## 🧪 **Testing and Quality Assurance**

### **Macro Testing Examples**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use yoshi_std::HatchExt;

    #[test]
    fn test_generated_display() {
        let error = MyAppError::UserNotFound {
            user_id: 12345,
            attempted_query: "SELECT * FROM users WHERE id = 12345".to_string(),
        };

        let display_output = format!("{}", error);
        assert!(display_output.contains("User not found: 12345"));
    }

    #[test]
    fn test_yoshi_conversion() {
        let error = MyAppError::ParseError {
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "config.json"),
            path: "/etc/app/config.json".to_string(),
        };

        let yoshi_error: yoshi_std::Yoshi = error.into();
        assert_eq!(yoshi_error.severity(), 75); // Default severity

        // Check that context metadata was attached
        let context = yoshi_error.primary_context().unwrap();
        assert!(context.metadata.contains_key(&std::sync::Arc::from("config_file")));
    }

    #[test]
    fn test_error_codes() {
        let system_error = AdvancedError::SystemFailure {
            message: "Critical failure".to_string(),
            cause: Box::new(std::io::Error::new(std::io::ErrorKind::Other, "system error")),
            system_state: SystemState {
                memory_usage: 95.5,
                cpu_usage: 88.2,
                active_connections: 1024,
            },
            occurred_at: "2025-01-20T10:30:00Z".to_string(),
        };

        assert_eq!(system_error.error_code(), Some(1001));
        assert_eq!(system_error.severity(), Some(255));
    }
}
```

---

## 📊 **Integration with Yoshi Ecosystem**

### **With yoshi-std**

```rust
use yoshi_derive::YoshiError;
use yoshi_std::{Result, HatchExt};

#[derive(Debug, YoshiError)]
pub enum ServiceError {
    #[yoshi(kind = "Network")]
    HttpError { status: u16 },
}

fn make_request() -> Result<String> {
    Err(ServiceError::HttpError { status: 404 })
        .context("Failed to fetch user data".to_string())
        .with_suggestion("Check the API endpoint")
}
```

### **With Tracing Integration**

```rust
#[derive(Debug, YoshiError)]
#[yoshi(tracing_integration = true)]
pub enum TracedError {
    #[yoshi(kind = "Internal")]
    ProcessingFailed { reason: String },
}

// Automatically generates tracing events
let error = TracedError::ProcessingFailed {
    reason: "Invalid state".to_string(),
};
let yoshi_error: yoshi_std::Yoshi = error.into();
yoshi_error.make_event(tracing::Level::ERROR);
```

---

## 🛠️ **Development and Contributing**

### **Building from Source**

```bash
git clone https://github.com/arcmoonstudios/yoshi.git
cd yoshi/yoshi-derive
cargo build --release
```

### **Running Tests**

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# Doc tests
cargo test --doc

# Benchmark tests
cargo bench
```

### **Performance Profiling**

```bash
# Profile compilation performance
cargo build --timings

# Run comprehensive benchmarks
cargo bench

# Generate detailed performance reports
cargo bench --features=full-benchmarks

# Profile macro expansion
cargo expand --bin your_binary > expanded.rs
```

---

## 📊 **Performance Benchmarking and Analysis**

### **Benchmark Results Summary**

**Latest Performance Analysis:** January 27, 2025
**Environment:** Windows x64, Rust 1.84.0
**Reference:** [Complete Benchmark Report](../BenchmarkResults.md)

#### **✅ Excellent Performance Areas**

| Metric | Performance | Status |
|--------|-------------|--------|
| **Basic Error Creation** | 49-162ns | ✅ **Excellent** |
| **Cross-Crate Integration** | 1.4-22µs | ✅ **Excellent** |
| **Memory Efficiency** | ~8µs/100 errors | ✅ **Excellent** |
| **Simple Error Formatting** | 347ns-1.2µs | ✅ **Excellent** |

#### **⚠️ Performance Concerns**

| Issue | Current Performance | Target | Action Required |
|-------|-------------------|---------|-----------------|
| **Error Chain Formatting** | 9.7ms (10-chain) | <100µs | 🔴 **Critical Fix Needed** |
| **Display Formatting** | +4% regression | 0% regression | 🟡 **Investigation Required** |
| **Database Scenarios** | +8% slower | ≤5% tolerance | 🟡 **Optimization Needed** |

### **Performance Targets vs Actual**

```text
✅ Error Creation:     Target: <1µs     | Actual: 49-162ns    | 🎯 EXCEEDED
✅ Memory Usage:       Target: <1KB     | Actual: 128-384B    | 🎯 EXCEEDED
✅ Integration:        Target: <50µs    | Actual: 1.4-22µs    | 🎯 EXCEEDED
❌ Chain Formatting:   Target: <100µs   | Actual: 9.7ms       | 🚨 97x OVER
✅ Simple Formatting:  Target: <10µs    | Actual: 347ns-1.2µs | 🎯 EXCEEDED
```

### **Macro Compilation Performance Details**

The yoshi-derive macro is designed for **zero-runtime overhead** with compile-time generation:

- **Macro Expansion**: < 5ms for typical error enums
- **Code Generation**: O(n) scaling with number of variants
- **Compilation Impact**: Minimal - comparable to hand-written implementations
- **Binary Size**: Near-zero overhead - only generates necessary code

### **Runtime Performance Analysis**

#### **Error Creation (Sub-microsecond)**

```rust
// Benchmark: 49-162ns per operation
let error = MyError::NetworkTimeout {
    message: "Connection failed".to_string(),
    endpoint: "https://api.example.com".to_string(),
    timeout_duration: Duration::from_secs(30),
}; // ~162ns
```

#### **Memory Allocation Patterns**

```text
Basic Error:     128 bytes  (1 allocation)
With Context:    256 bytes  (2 allocations)
With Shell:    384 bytes  (3 allocations)
Error Chain:     128n bytes (n+1 allocations)
```

#### **Cross-Crate Integration**

```rust
// Benchmark: 1.4-22µs depending on complexity
let yoshi_error: yoshi_std::Yoshi = my_error.into(); // ~5.2µs
```

### **Critical Performance Issue: Error Chain Formatting**

**Current Behavior:** O(n²) scaling for error chain formatting

```text
1 chain:    13.2µs   (acceptable)
2 chains:   89.4µs   (acceptable)
5 chains:   1.2ms    (concerning)
10 chains:  9.7ms    (unacceptable)
```

**Root Cause:** Recursive string allocation and inefficient chain traversal

**Proposed Solution:**

```rust
// Replace recursive approach with iterative O(n) algorithm
pub fn format_error_chain_optimized(error: &dyn Error) -> String {
    let chain_depth = calculate_chain_depth(error);
    let mut buffer = String::with_capacity(estimate_buffer_size(chain_depth));

    let mut current = Some(error);
    while let Some(err) = current {
        buffer.push_str(&err.to_string());
        current = err.source();
        if current.is_some() {
            buffer.push_str(" -> ");
        }
    }
    buffer
}
```

### **Performance Monitoring**

The derive macro can optionally generate performance monitoring code:

```rust
#[derive(Debug, YoshiError)]
#[yoshi(performance_monitoring = true)]
pub enum MonitoredError {
    #[yoshi(kind = "Network")]
    NetworkError { message: String },
}

// Automatically tracks creation time, memory usage, and frequency
let error = MonitoredError::NetworkError {
    message: "Connection failed".to_string(),
};
error.track_creation(); // Records performance metrics
```

### **Optimization Recommendations**

#### **Immediate Actions**

1. **Fix error chain formatting scaling** - Replace O(n²) with O(n) algorithm
2. **Investigate display formatting regression** - Profile recent changes
3. **Optimize database scenario performance** - Reduce 8% overhead

#### **Future Optimizations**

1. **Implement lazy formatting** - Defer expensive string operations
2. **Add performance budgets** - CI-based performance regression testing
3. **Optimize memory allocation** - Use `SmallVec` and object pooling
4. **SIMD string processing** - Leverage CPU-specific optimizations

---

## 📚 **Additional Resources**

- **[Main Yoshi Documentation](../README.md)** - Complete framework overview
- **[API Documentation](https://docs.rs/yoshi-derive)** - Detailed API reference
- **[Examples Repository](../examples/)** - Real-world usage examples
- **[Performance Guide](../docs/PERFORMANCE.md)** - Optimization strategies
- **[Migration Guide](../docs/MIGRATION.md)** - Upgrading from other error crates

---

## 🏢 **Enterprise Information**

**Powered by ArcMoon Studios** - Where precision meets innovation.

- **Business Inquiries**: [LordXyn@proton.me](mailto:LordXyn@proton.me)
- **Technical Support**: [GitHub Issues](https://github.com/arcmoonstudios/yoshi/issues)
- **License**: Business Source License 1.1 (BSL-1.1)
- **Commercial License**: Available for production use

### **License Terms**

- ✅ **Non-production use**: Free for development, testing, and evaluation
- ✅ **Open source projects**: Free under BSL-1.1 terms
- 💼 **Commercial/Production use**: Requires paid license
- 📅 **Change Date**: 2025-05-25 (converts to GPL v3)

---

## 🚀 **Roadmap**

- **v0.2.0**: Enhanced macro diagnostics and IDE integration
- **v0.3.0**: Custom derive macro for error context types
- **v0.4.0**: Integration with cargo expand for debugging
- **v1.0.0**: Stable API with full Rust 1.87+ feature support

---

**GitHub**: [ArcMoon Studios](https://github.com/arcmoonstudios) | **Copyright**: (c) 2025 ArcMoon Studios | **Author**: Lord Xyn

*Mathematical precision. Enterprise excellence. Zero compromises.*
