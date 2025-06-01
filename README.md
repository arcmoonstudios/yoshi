# Yoshi Error Handling Framework

[![License: BSL-1.1](https://img.shields.io/badge/License-BSL_1.1-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.87+-orange.svg)](https://www.rust-lang.org/)
[![Build Status](https://img.shields.io/badge/build-passing-green.svg)](https://github.com/arcmoonstudios/yoshi)

> **Enterprise-grade structured error handling for Rust applications with mathematical precision and performance optimization.**

Yoshi is a comprehensive error-handling framework designed for critical Rust applications that require detailed error diagnostics, structured error categorization, and high-performance error propagation. Built by [ArcMoon Studios](https://github.com/arcmoonstudios), Yoshi provides zero-cost abstractions for error handling while maintaining rich contextual information.

## 🚀 Key Features

### **🏗️ Structured Error Types**

- **Categorical Error System**: Define precise error categories with relevant metadata instead of string-based errors
- **Type-Safe Design**: Memory-safe and thread-safe error handling with compile-time guarantees
- **Rich Diagnostics**: Capture detailed diagnostic information for each error type

### **⚡ Performance Excellence**

- **Sub-microsecond Error Creation**: O(1) error creation with O(1) context attachment
- **Zero-Cost Abstractions**: Conditional backtrace capture that can be disabled in production
- **Memory Efficient**: Pre-allocated buffers and shared strings for minimal allocations
- **Stack-Overflow Protection**: Bounded context depth with intelligent overflow handling

### **🔗 Context Preservation**

- **Context Chaining**: Maintain complete error trace visibility as errors propagate
- **Metadata Attachment**: Add typed payloads, suggestions, and diagnostic information
- **Source Location Tracking**: Automatic capture of source code locations for debugging

### **🌐 Platform Compatibility**

- **`no_std` Support**: Full functionality available in embedded and `no_std` environments
- **Cross-Platform**: Automatic fallbacks for platform-specific features
- **WebAssembly Ready**: Compatible with WASM targets for web deployment

## 📦 Crate Structure

```text
yoshi/
├── yoshi/           # 🎭 Main facade crate - Primary API entry point
├── yoshi-std/       # 🛠️ Core engine - Standard library implementation
├── yoshi-derive/    # 🔧 Procedural macros - Code generation utilities
└── yoshi-benches/   # 📊 Comprehensive benchmarks - Performance validation
```

### **Crate Descriptions**

- **`yoshi`**: The main facade providing a unified API for all Yoshi functionality
- **`yoshi-std`**: Core error handling implementation with standard library features
- **`yoshi-derive`**: Procedural macros for automatic error type generation
- **`yoshi-benches`**: Performance benchmarks and comparative analysis tools

## 🎯 Quick Start

### Installation

Add Yoshi to your `Cargo.toml`:

```toml
[dependencies]
yoshi = "0.1.0"

# For no_std environments
yoshi = { version = "0.1.0", default-features = false }

# With optional features
yoshi = { version = "0.1.0", features = ["serde", "tracing"] }
```

### Basic Usage

```rust
use yoshi::{Yoshi, YoshiKind, HatchExt, Result};

fn load_config(path: &str) -> Result<String> {
    // Create structured errors with rich context
    std::fs::read_to_string(path)
        .map_err(|e| Yoshi::new(YoshiKind::Io {
            message: "Failed to read configuration file".into(),
            source: Some(Box::new(e)),
            path: Some(path.into()),
        }))
        .context(format!("Loading configuration from {}", path))?
        .parse()
        .map_err(|e| Yoshi::new(YoshiKind::Parse {
            message: "Invalid configuration format".into(),
            source: Some(Box::new(e)),
            expected_format: Some("TOML".into()),
        }))
        .context("Parsing configuration file")
}

fn main() {
    match load_config("/etc/app/config.toml") {
        Ok(config) => println!("Config loaded: {}", config),
        Err(err) => {
            // Rich error information with full context chain
            eprintln!("Error: {}", err);
            eprintln!("Context: {:#}", err.context_chain());

            // Access structured error data
            if let YoshiKind::Io { path, .. } = err.kind() {
                eprintln!("Failed file path: {:?}", path);
            }
        }
    }
}
```

### Advanced Error Creation

```rust
use yoshi::{Yoshi, YoshiKind};

// Create errors with rich metadata and suggestions
fn validate_user_input(input: &str) -> Result<(), Yoshi> {
    if input.is_empty() {
        return Err(Yoshi::new(YoshiKind::Validation {
            field: "user_input".into(),
            message: "Input cannot be empty".into(),
            expected: Some("non-empty string".into()),
            actual: Some("empty string".into()),
        })
        .with_metadata("input_length", "0")
        .with_metadata("validation_rule", "non_empty")
        .with_suggestion("Provide a non-empty input value")
        .with_suggestion("Check input validation logic"));
    }

    if input.len() > 1000 {
        return Err(Yoshi::new(YoshiKind::Validation {
            field: "user_input".into(),
            message: "Input exceeds maximum length".into(),
            expected: Some("≤ 1000 characters".into()),
            actual: Some(format!("{} characters", input.len()).into()),
        })
        .with_metadata("input_length", input.len().to_string())
        .with_metadata("max_length", "1000")
        .with_suggestion("Reduce input length to 1000 characters or less"));
    }

    Ok(())
}
```

## 🏗️ Architecture

### **Error Categories**

Yoshi provides structured error categories through `YoshiKind`:

```rust
pub enum YoshiKind {
    Io { message: String, source: Option<Box<dyn Error>>, path: Option<String> },
    Network { message: String, source: Option<Box<dyn Error>>, error_code: Option<u32> },
    Validation { field: String, message: String, expected: Option<String>, actual: Option<String> },
    NotFound { resource_type: String, identifier: String, search_locations: Option<Vec<String>> },
    Timeout { operation: String, duration: Duration, expected_max: Option<Duration> },
    Config { message: String, source: Option<Box<dyn Error>>, config_path: Option<String> },
    Parse { message: String, source: Option<Box<dyn Error>>, expected_format: Option<String> },
    ResourceExhausted { resource: String, limit: String, current: String, usage_percentage: Option<f64> },
    // ... and more
}
```

### **Context System**

Errors can accumulate rich contextual information:

```rust
let error = base_error
    .context("High-level operation context")
    .with_metadata("operation_id", "op_12345")
    .with_metadata("user_id", "user_67890")
    .with_suggestion("Try the operation again")
    .with_suggestion("Check system resources")
    .with_shell(additional_debug_data);
```

## 📊 Performance

Yoshi is designed for high-performance applications with the following characteristics:

| Operation | Performance | Memory Usage |
|-----------|------------|--------------|
| Error Creation | < 1μs | 64-128 bytes |
| Context Addition | O(1) | 32-64 bytes |
| Error Formatting | < 10μs | Minimal allocation |
| Backtrace Capture | Optional | Conditional |

### **Benchmarking**

Run comprehensive benchmarks to validate performance:

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark category
cargo bench --bench error_creation
cargo bench --bench error_formatting
cargo bench --bench error_context

# Generate performance reports
cargo bench -- --output-format html
```

## 🔧 Configuration

### **Feature Flags**

```toml
[dependencies.yoshi]
version = "0.1.0"
features = [
    "std",           # Standard library support (default)
    "serde",         # Serialization support
    "tracing",       # Tracing integration
    "backtrace",     # Backtrace capture
    "miette",        # Miette integration
]
```

### **Environment Variables**

- `YOSHI_BACKTRACE`: Enable/disable backtrace capture (`0`/`1`)
- `YOSHI_MAX_CONTEXT_DEPTH`: Maximum context chain depth (default: 50)
- `YOSHI_PERFORMANCE_MONITOR`: Enable performance monitoring (`0`/`1`)

## 🧪 Testing

Yoshi includes comprehensive testing and validation:

```bash
# Run all tests
cargo test

# Run tests with all features
cargo test --all-features

# Run tests in no_std mode
cargo test --no-default-features

# Run integration tests
cargo test --test integration_tests

# Run doc tests
cargo test --doc
```

## 📚 Documentation

- **[API Documentation](docs/Yoshi.md)**: Complete API reference and examples
- **[Performance Guide](docs/performance.md)**: Optimization strategies and benchmarks
- **[Migration Guide](docs/migration.md)**: Migrating from other error handling crates
- **[Best Practices](docs/best-practices.md)**: Recommended patterns and anti-patterns

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### **Development Setup**

```bash
# Clone the repository
git clone https://github.com/arcmoonstudios/yoshi.git
cd yoshi

# Install Rust toolchain
rustup install stable
rustup default stable

# Run tests
cargo test --all-features

# Run benchmarks
cargo bench

# Check code quality
cargo clippy -- -D warnings
cargo fmt --check
```

### **Code Standards**

- Follow ArcMoon Studios coding standards
- Maintain ≥95% test coverage
- Include comprehensive documentation
- Ensure all benchmarks pass performance thresholds

## 📄 License

This project is licensed under the [Business Source License 1.1](LICENSE).

### **License Summary**

- ✅ **Non-commercial use**: Free for development, testing, and non-production use
- ✅ **Open source**: Full source code available for review and contribution
- ❌ **Commercial/Production use**: Requires a commercial license from ArcMoon Studios
- 🕐 **Change License**: Automatically becomes GPL v3 after 48 months

For commercial licensing, contact: [LordXyn@proton.me](mailto:LordXyn@proton.me)

## 🏢 About ArcMoon Studios

ArcMoon Studios specializes in enterprise-grade Rust development tools and frameworks. We focus on mathematical precision, performance optimization, and production-ready solutions for critical applications.

- **GitHub**: [ArcMoon Studios](https://github.com/arcmoonstudios)
- **Contact**: [LordXyn@proton.me](mailto:LordXyn@proton.me)
- **Website**: [Coming Soon]

## 📈 Roadmap

### **Version 0.2.0**

- [ ] Async error handling support
- [ ] WebAssembly optimization
- [ ] Advanced error recovery patterns
- [ ] Performance dashboard

### **Version 0.3.0**

- [ ] Distributed tracing integration
- [ ] Error analytics and reporting
- [ ] Plugin architecture
- [ ] IDE integration tools

## 🙏 Acknowledgments

- Built with inspiration from `anyhow`, `thiserror`, and `eyre`
- Performance optimizations inspired by enterprise error handling needs
- Community feedback and contributions from the Rust ecosystem

---

**Yoshi** - Where mathematical precision meets enterprise-grade error handling.

Made with 💜 by ArcMoon Studios
