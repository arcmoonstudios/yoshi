# Working with Error Context in Yoshi

One of Yoshi's most powerful features is the ability to attach rich contextual information to errors. This guide shows how to add and retrieve metadata from errors.

## Adding Metadata to Errors

### Using the `yopost!` Macro

The most common way to add metadata is using the `yopost!` macro with structured error kinds:

```rust
use yoshi::*;

fn validate_user(user_id: u64, role: &str) -> Hatch<()> {
    // Add metadata using structured YoshiKind
    if role == "guest" {
        return Err(yopost!(kind: YoshiKind::Validation {
            field: "role".into(),
            message: "Insufficient permissions".into(),
            expected: Some("admin".into()),
            actual: Some(role.into()),
        })
        .with_metadata("user_id", user_id.to_string())
        .with_signpost("Request elevated permissions"));
    }

    Ok(())
}
```

### Using the `.with_metadata()` Method

You can also add metadata after creating an error using the `.with_metadata()` method:

```rust
use yoshi::*;

fn fetch_data(url: &str) -> Hatch<Data> {
    let response = make_request(url).map_err(|e| {
        // Create base error and add metadata
        yopost!(kind: YoshiKind::Network {
            message: "Failed to fetch data".into(),
            source: Some(Box::new(yopost!(error: e))),
            error_code: None,
        })
        .with_metadata("url", url)
        .with_metadata("retry_count", "3")
        .with_metadata("timeout_ms", "5000")
        .with_metadata("protocol", if url.starts_with("https") { "https" } else { "http" })
        .with_signpost("Check network connectivity and URL validity")
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

fn process_batch(items: &[Item]) -> Hatch<BatchResult> {
    let mut failures = Vec::new();
    let mut error_details = Vec::new();

    for (idx, item) in items.iter().enumerate() {
        if let Err(e) = process_item(item) {
            failures.push(format!("item_{}", idx));
            error_details.push(e.to_string());
        }
    }

    if !failures.is_empty() {
        let mut error = yopost!(kind: YoshiKind::Multiple {
            errors: error_details.into_iter().map(|msg| {
                yopost!(message: msg)
            }).collect(),
            primary_index: Some(0),
        })
        .with_metadata("total_items", items.len().to_string())
        .with_metadata("failed_items", failures.len().to_string())
        .with_metadata("failed_indices", failures.join(","));

        return Err(error);
    }

    Ok(BatchResult::new(items.len()))
}
```

## Retrieving Metadata from Errors

### Using the `yum!` Macro for Comprehensive Analysis

The easiest way to see all error information is using the `yum!` macro:

```rust
use yoshi::*;

fn handle_error(err: Yoshi) {
    // Comprehensive error analysis with all metadata
    yum!(err);
}
```

### Manual Metadata Access

For programmatic access to metadata, you can inspect the error structure:

```rust
use yoshi::*;

fn handle_error(err: &Yoshi) {
    // Access the error kind for structured information
    match err.kind() {
        YoshiKind::Validation { field, message, expected, actual } => {
            println!("Validation error in field '{}': {}", field, message);
            if let Some(exp) = expected {
                println!("Expected: {}", exp);
            }
            if let Some(act) = actual {
                println!("Actual: {}", act);
            }
        },
        YoshiKind::Network { message, error_code, .. } => {
            println!("Network error: {}", message);
            if let Some(code) = error_code {
                println!("Error code: {}", code);
            }
        },
        _ => {
            println!("Other error: {}", err);
        }
    }
}
```

### Error Categorization by Kind

```rust
use yoshi::*;

fn categorize_error(err: &Yoshi) -> ErrorCategory {
    match err.kind() {
        YoshiKind::Validation { .. } => ErrorCategory::UserInput,
        YoshiKind::Network { .. } => ErrorCategory::NetworkRelated,
        YoshiKind::Io(_) => ErrorCategory::FileSystemRelated,
        YoshiKind::Security { .. } => ErrorCategory::SecurityRelated,
        YoshiKind::Timeout { .. } => ErrorCategory::PerformanceRelated,
        _ => ErrorCategory::Other,
    }
}

enum ErrorCategory {
    UserInput,
    NetworkRelated,
    FileSystemRelated,
    SecurityRelated,
    PerformanceRelated,
    Other,
}
```

### Context Chaining with `.lay()`

```rust
use yoshi::*;

fn process_user_request(user_id: u64) -> Hatch<String> {
    // Chain context as errors propagate
    let user_data = fetch_user_data(user_id)
        .lay("Failed to fetch user data")?;

    let processed_data = process_data(&user_data)
        .lay("Failed to process user data")?;

    let result = finalize_processing(&processed_data)
        .lay("Failed to finalize processing")?;

    Ok(result)
}

fn fetch_user_data(user_id: u64) -> Hatch<UserData> {
    // This might fail with a network error
    database_query(user_id)
        .lay("Database query failed")
}
```

## Best Practices for Error Context

1. **Use Structured Error Kinds**: Prefer structured `YoshiKind` variants over generic messages for better error categorization.

2. **Include Relevant IDs**: Always add identifiers like user IDs, request IDs, or document IDs using `.with_metadata()`.

3. **Add Suggestions**: When possible, include suggestions on how to fix the error using `.with_signpost()`.

4. **Chain Context with `.lay()`**: Use the `.lay()` method to add context as errors propagate up the call stack.

5. **Use `yum!` for Debugging**: Use the `yum!` macro during development to see comprehensive error information.

6. **Be Consistent**: Use consistent names for common metadata (e.g., always use `user_id` not sometimes `userId`).

7. **Security Considerations**: Be careful not to include sensitive data in error metadata that might be logged.

8. **Performance**: Remember that error creation should be fast - avoid expensive operations in error construction.

## Example: Complete Error Handling Pattern

```rust
use yoshi::*;

fn complete_example() -> Hatch<String> {
    let user_id = 12345;

    // Structured error with comprehensive context
    let result = risky_operation(user_id)
        .lay("Failed during user operation")?;

    Ok(result)
}

fn risky_operation(user_id: u64) -> Hatch<String> {
    // Simulate a validation error with rich context
    Err(yopost!(kind: YoshiKind::Validation {
        field: "user_permissions".into(),
        message: "User lacks required permissions".into(),
        expected: Some("admin".into()),
        actual: Some("guest".into()),
    })
    .with_metadata("user_id", user_id.to_string())
    .with_metadata("operation", "sensitive_data_access")
    .with_signpost("Contact administrator to upgrade permissions"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_handling() {
        match complete_example() {
            Ok(_) => panic!("Expected error"),
            Err(error) => {
                // Use yum! for comprehensive error analysis in tests
                yum!(error);

                // Verify error structure
                assert!(matches!(error.kind(), YoshiKind::Validation { .. }));
            }
        }
    }
}
```
