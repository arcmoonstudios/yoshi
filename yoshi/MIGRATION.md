# 🚀 **Yoshi Migration Guide - From thiserror/anyhow to Yoshi**

This guide shows you how to migrate from existing error handling frameworks to Yoshi with **zero breaking changes** and **immediate benefits**.

## **🎯 Quick Start - Drop-in Replacement**

### **From `anyhow` (5 seconds)**

```rust
// Before
use anyhow::{Result, Context};

fn read_config() -> Result<String> {
    let content = std::fs::read_to_string("config.toml")
        .context("Failed to read config file")?;
    Ok(content)
}

// After - Just change the import!
use yoshi::simple::{Result, Context};

fn read_config() -> Result<String> {
    let content = std::fs::read_to_string("config.toml")
        .context("Failed to read config file")?;
    Ok(content)
}
```

### **From `thiserror` (10 seconds)**

```rust
// Before
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {message}")]
    Parse { message: String },
}

// After - Just change the import!
use yoshi::simple::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {message}")]
    Parse { message: String },
}
```

## **🔥 Gradual Enhancement - Unlock Yoshi's Power**

Once you've migrated, you can gradually add Yoshi's advanced features:

### **Level 1: Add Context (Nests)**

```rust
use yoshi::simple::{Result, advanced};

fn process_file(path: &str) -> Result<String> {
    let content = std::fs::read_to_string(path)
        .context("Reading file")?;

    // Add Yoshi's advanced context
    let processed = advanced::nest(
        process_content(&content),
        format!("Processing file: {}", path)
    )?;

    Ok(processed)
}
```

### **Level 2: Add Suggestions (Signposts)**

```rust
use yoshi::simple::{Result, advanced};

fn connect_database(url: &str) -> Result<Connection> {
    let conn = Database::connect(url)
        .context("Database connection failed")?;

    // Add helpful suggestions
    advanced::signpost(
        validate_connection(&conn),
        "Try checking your database credentials and network connectivity"
    )?;

    Ok(conn)
}
```

### **Level 3: Add Metadata**

```rust
use yoshi::simple::{Result, advanced};

fn api_request(endpoint: &str) -> Result<Response> {
    let response = http_client.get(endpoint)
        .context("API request failed")?;

    // Add structured metadata
    let result = advanced::metadata(
        parse_response(response),
        "endpoint", endpoint
    )?;

    advanced::metadata(result, "timestamp", chrono::Utc::now().to_rfc3339())
}
```

### **Level 4: Full Yoshi Power**

```rust
use yoshi::{Hatch, yoshi, simple::advanced};

fn advanced_processing(data: &[u8]) -> Hatch<ProcessedData> {
    // Convert from simple API to full Yoshi
    let simple_result = parse_data(data);
    let mut hatch_result = advanced::to_hatch(simple_result)?;

    // Use full Yoshi features
    hatch_result = hatch_result
        .nest("Advanced data processing")
        .with_signpost("Ensure data is valid UTF-8")
        .with_metadata("data_size", data.len().to_string())
        .with_priority(150);

    // Advanced error creation
    if data.is_empty() {
        return Err(yopost!(
            message: "Empty data provided",
            with_signpost = "Provide non-empty data for processing",
            with_metadata = ("operation", "data_validation")
        ));
    }

    Ok(ProcessedData::new(data))
}
```

## **📊 Comparison Table**

| Feature | anyhow | thiserror | Yoshi Simple | Yoshi Full |
|---------|--------|-----------|--------------|------------|
| **Drop-in compatibility** | ✅ | ✅ | ✅ | ✅ |
| **Zero learning curve** | ✅ | ✅ | ✅ | ❌ |
| **Rich context chains** | ⚠️ Basic | ❌ | ✅ | ✅ |
| **Actionable suggestions** | ❌ | ❌ | ✅ | ✅ |
| **Structured metadata** | ❌ | ❌ | ✅ | ✅ |
| **Auto-correction hints** | ❌ | ❌ | ⚠️ Basic | ✅ |
| **Performance optimization** | ❌ | ❌ | ⚠️ Basic | ✅ |
| **IDE integration** | ❌ | ❌ | ❌ | ✅ |

## **🛠️ Migration Strategies**

### **Strategy 1: Instant Migration (Recommended)**

1. Replace imports: `anyhow` → `yoshi::simple`
2. Test that everything compiles
3. Gradually add advanced features where needed

### **Strategy 2: Hybrid Approach**

1. Keep existing error types
2. Add Yoshi for new code
3. Gradually convert critical paths

### **Strategy 3: Full Migration**

1. Convert all error types to Yoshi
2. Add comprehensive context and suggestions
3. Integrate with IDE and auto-correction

## **🎁 Immediate Benefits**

Even with just the simple API, you get:

- **Better error messages** with automatic context chaining
- **Structured error data** for logging and monitoring
- **Future-proof** - can add advanced features anytime
- **Zero performance overhead** - same speed as anyhow
- **Better debugging** with rich error information

## **🚨 Common Pitfalls**

### **Don't do this:**

```rust
// ❌ Mixing APIs unnecessarily
use yoshi::{Yoshi, YoshiKind, Hatch}; // Too complex for simple cases

fn simple_function() -> Hatch<String> {
    let err = Yoshi::new(YoshiKind::Internal { /* complex setup */ });
    // This is overkill for simple errors
}
```

### **Do this instead:**

```rust
// ✅ Start simple, enhance gradually
use yoshi::simple::{Result, error};

fn simple_function() -> Result<String> {
    if something_wrong {
        return Err(error("Something went wrong"));
    }
    Ok("success".to_string())
}
```

## **📚 Next Steps**

1. **Start with simple API** - Get immediate benefits
2. **Add context gradually** - Enhance error messages
3. **Explore advanced features** - When you need more power
4. **Integrate with tooling** - IDE support, auto-correction

The beauty of Yoshi is that you can **start simple** and **grow into complexity** as needed, without ever hitting a wall or needing to rewrite your error handling.
