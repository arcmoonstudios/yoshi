# Working with Error Context in Yoshi

One of Yoshi's most powerful features is the ability to attach rich contextual information to errors. This guide shows how to add and retrieve metadata from errors.

## Adding Metadata to Errors

### Using the `yoshi!` Macro

The most common way to add metadata is directly in the `yoshi!` macro:

```rust
use yoshi::*;

fn validate_user(user_id: u64, role: &str) -> Result<()> {
    // Add metadata directly in the macro
    if role == "guest" {
        return Err(yoshi!(
            YoshiKind::Permission,
            "Insufficient permissions",
            user_id: user_id,
            requested_role: role,
            required_role: "admin",
            suggestion: "Request elevated permissions"
        ));
    }

    Ok(())
}
```

### Using the `.meta()` Method

You can also add metadata after creating an error using the `.meta()` method:

```rust
use yoshi::*;

fn fetch_data(url: &str) -> Result<Data> {
    let response = make_request(url).map_err(|e| {
        // Create base error
        let error = yoshi!(
            YoshiKind::Network,
            "Failed to fetch data",
            url: url,
            source: e
        );

        // Add additional metadata
        error
            .meta("retry_count", 3)
            .meta("timeout_ms", 5000)
            .meta("protocol", if url.starts_with("https") { "https" } else { "http" })
    })?;

    // Process response...
    Ok(response.into())
}
```

### Dynamic Metadata Collection

For complex scenarios, you can build metadata collections dynamically:

```rust
use yoshi::*;
use std::collections::HashMap;

fn process_batch(items: &[Item]) -> Result<BatchResult> {
    let mut failures = HashMap::new();

    for (idx, item) in items.iter().enumerate() {
        if let Err(e) = process_item(item) {
            failures.insert(format!("item_{}", idx), e.to_string());
        }
    }

    if !failures.is_empty() {
        return Err(yoshi!(
            YoshiKind::Processing,
            "Batch processing partially failed",
            total_items: items.len(),
            failed_items: failures.len()
        ).meta("failures", failures));
    }

    Ok(BatchResult::new(items.len()))
}
```

## Retrieving Metadata from Errors

### Using the `.get_meta()` Method

```rust
use yoshi::*;

fn handle_error(err: &Yoshi) {
    // Get basic metadata with type conversion
    if let Some(user_id) = err.get_meta::<u64>("user_id") {
        println!("Error occurred for user: {}", user_id);
    }

    // Get optional string values
    if let Some(suggestion) = err.get_meta::<String>("suggestion") {
        println!("Suggestion: {}", suggestion);
    }

    // Complex types (if serialized with serde feature enabled)
    if let Some(failures) = err.get_meta::<HashMap<String, String>>("failures") {
        println!("Failed items:");
        for (item, reason) in failures {
            println!("- {}: {}", item, reason);
        }
    }
}
```

### Checking for Metadata Existence

```rust
use yoshi::*;

fn categorize_error(err: &Yoshi) -> ErrorCategory {
    if err.has_meta("user_id") {
        return ErrorCategory::UserRelated;
    } else if err.has_meta("url") {
        return ErrorCategory::NetworkRelated;
    } else if err.has_meta("file") || err.has_meta("path") {
        return ErrorCategory::FileSystemRelated;
    }

    ErrorCategory::Other
}
```

### Accessing Error Context Chain

```rust
use yoshi::*;

fn log_error(err: &Yoshi) {
    // Get full chain of error contexts
    let context_chain = err.context_chain();

    println!("Error: {}", err);
    println!("Context chain:");

    for (idx, ctx) in context_chain.iter().enumerate() {
        println!("  {}. {}", idx + 1, ctx);

        // Print metadata for each context level
        for (key, value) in ctx.metadata() {
            println!("     - {}: {}", key, value);
        }
    }
}
```

## Best Practices for Error Context

1. **Include Relevant IDs**: Always add identifiers like user IDs, request IDs, or document IDs
2. **Include Input Values**: Add the values that caused the error (but be careful with sensitive data)
3. **Add Suggestions**: When possible, include suggestions on how to fix the error
4. **Be Consistent**: Use consistent names for common metadata (e.g., always use `user_id` not sometimes `userId`)
5. **Context, Not State**: Use metadata for debugging context, not for passing program state
