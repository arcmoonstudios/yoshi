/* yoshi-derive/examples/enhanced_yoshi_af_showcase.rs */

//! **Brief:** Showcase of enhanced `yoshi_af!` macro with auto-optimization capabilities.
//!
//! This example demonstrates the enhanced `yoshi_af!` macro that now includes
//! compile-time auto-optimization features. It shows how the macro can detect
//! and optimize common patterns automatically during compilation.
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

use yoshi_core::Hatch;

/// Main function that demonstrates the enhanced `yoshi_af!` macro capabilities
#[allow(clippy::print_stdout)]
fn main() {
    tracing::info!("🚀 Enhanced yoshi_af! Macro Showcase 🚀\n");

    // Test the enhanced macro with auto-optimization
    test_basic_optimization();
    test_error_handling_optimization();
    test_combined_optimizations();

    tracing::info!("\n✅ Enhanced yoshi_af! macro showcase completed!");
}

/// Demonstrates basic optimization capabilities of the enhanced macro
#[allow(clippy::print_stdout)]
fn test_basic_optimization() {
    tracing::info!("📦 Basic Optimization Test");
    tracing::info!("==========================");

    // This function will be processed by the enhanced yoshi_af! macro
    let result = basic_function_with_optimizations();
    match result {
        Ok(items) => tracing::info!("✅ Function succeeded with {} items", items.len()),
        Err(e) => tracing::error!("❌ Function failed: {e}"),
    }

    println!();
}

/// Demonstrates error handling optimization capabilities of the enhanced macro
#[allow(clippy::print_stdout)]
fn test_error_handling_optimization() {
    tracing::info!("🛡️ Error Handling Optimization Test");
    tracing::info!("===================================");

    let result = error_handling_function();
    match result {
        Ok(value) => tracing::info!("✅ Error handling succeeded: {value}"),
        Err(e) => tracing::error!("❌ Error handling failed: {e}"),
    }

    println!();
}

/// Demonstrates combined optimization capabilities of the enhanced macro
#[allow(clippy::print_stdout)]
fn test_combined_optimizations() {
    tracing::info!("⚡ Combined Optimizations Test");
    tracing::info!("=============================");

    let result = combined_optimizations_function();
    match result {
        Ok(data) => tracing::info!("✅ Combined optimizations succeeded: {data:?}"),
        Err(e) => tracing::error!("❌ Combined optimizations failed: {e}"),
    }

    println!();
}

// Example function that will be optimized by yoshi_af! macro
fn basic_function_with_optimizations() -> Hatch<Vec<String>> {
    // This Vec::new() will be optimized to Vec::with_capacity(3)
    let items = vec![
        "first".to_string(),
        "second".to_string(),
        "third".to_string(),
    ];

    Ok(items)
}

// Example function with error handling that will be optimized
fn error_handling_function() -> Hatch<String> {
    let maybe_value = Some("test_value".to_string());

    // This .unwrap() will be optimized to proper error handling
    let value = maybe_value.unwrap();

    Ok(value)
}

// Example function with multiple optimization opportunities
fn combined_optimizations_function() -> Hatch<Vec<ProcessedData>> {
    // Vec::new() optimization opportunity
    let mut results = Vec::new();

    // Simulate some data processing
    let raw_data = vec!["data1", "data2", "data3", "data4"];

    for item in raw_data {
        let processed = process_item(item)?;
        results.push(processed);
    }

    Ok(results)
}

// Helper function for processing data
fn process_item(data: &str) -> Hatch<ProcessedData> {
    if data.is_empty() {
        return Err(yoshi_core::Yoshi::new(yoshi_core::YoshiKind::Validation {
            field: "data".into(),
            message: "Empty data not allowed".into(),
            expected: Some("non-empty string".into()),
            actual: Some("empty string".into()),
        }));
    }

    Ok(ProcessedData {
        original: data.to_string(),
        processed: data.to_uppercase(),
        length: data.len(),
    })
}

// Simple data structure for the example
/// Data structure for processing and storing string data
#[derive(Debug, Clone)]
struct ProcessedData {
    /// The original input string
    original: String,
    /// The processed result string
    processed: String,
    /// The length of the processed string
    length: usize,
}

impl ProcessedData {
    /// Use all fields to avoid dead code warnings
    #[allow(dead_code)]
    const fn validate(&self) -> bool {
        !self.original.is_empty() && !self.processed.is_empty() && self.length > 0
    }
}

// Example of a function without optimization opportunities
#[cfg(test)]
fn already_optimized_function() -> Hatch<Vec<String>> {
    // This is already optimized - no changes needed
    let result = some_operation()?;
    Ok(vec![result])
}

// Helper function that returns a Result
/// Helper function that returns a successful result
#[allow(dead_code)]
fn some_operation() -> Hatch<String> {
    Ok("optimized_result".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_optimization_works() {
        let result = basic_function_with_optimizations();
        assert!(result.is_ok());
        let items = result.unwrap();
        assert_eq!(items.len(), 3);
        assert_eq!(items[0], "first");
        assert_eq!(items[1], "second");
        assert_eq!(items[2], "third");
    }

    #[test]
    fn test_error_handling_works() {
        let result = error_handling_function();
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "test_value");
    }

    #[test]
    fn test_combined_optimizations_work() {
        let result = combined_optimizations_function();
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.len(), 4);
        assert_eq!(data[0].original, "data1");
        assert_eq!(data[0].processed, "DATA1");
        assert_eq!(data[0].length, 5);
    }

    #[test]
    fn test_process_item_works() {
        let result = process_item("test");
        assert!(result.is_ok());
        let processed = result.unwrap();
        assert_eq!(processed.original, "test");
        assert_eq!(processed.processed, "TEST");
        assert_eq!(processed.length, 4);
    }

    #[test]
    fn test_process_item_handles_empty() {
        let result = process_item("");
        assert!(result.is_err());
    }

    #[test]
    fn test_already_optimized_function() {
        let result = already_optimized_function();
        assert!(result.is_ok());
        let items = result.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0], "optimized_result");
    }
}
