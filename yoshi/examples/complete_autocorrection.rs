/* examples/complete_autocorrection.rs */
//! Complete Auto-Correction System Example
//!
//! This example demonstrates the full auto-correction capabilities of the Yoshi framework.
//! It shows how to use `yoshi_af!`, `YoshiError`, and the `YoshiACSystem` together.

use std::time::Duration;
use yoshi::*;

// 1. Define custom error types with YoshiError derive
/// Application-specific error types with rich metadata
#[derive(Debug, YoshiError)]
#[allow(dead_code, unused_variables, unused)] // Example error types - fields used by derive macro
pub enum AppError {
    /// Network connection failure
    #[yoshi(display = "Network connection failed: {_reason}")]
    #[yoshi(suggestion = "Check network connectivity and retry")]
    NetworkFailure {
        /// The reason for the network failure
        _reason: String,
    },

    /// Configuration error
    #[yoshi(display = "Invalid configuration: {_field}")]
    #[yoshi(suggestion = "Check configuration file syntax")]
    ConfigError {
        /// The configuration field that caused the error
        _field: String,
    },

    /// I/O error wrapper
    #[yoshi(transparent)]
    Io(std::io::Error),
}

// Helper function to demonstrate error type usage
fn demonstrate_error_types() -> std::result::Result<(), AppError> {
    // This function uses the error types to avoid unused field warnings
    let network_error = AppError::NetworkFailure {
        _reason: "Connection timeout".to_string(),
    };
    let config_error = AppError::ConfigError {
        _field: "database_url".to_string(),
    };

    // Access the fields to demonstrate usage and avoid unused warnings
    if let AppError::NetworkFailure { _reason } = &network_error {
        println!("   Network error reason: {_reason}");
    }

    if let AppError::ConfigError { _field } = &config_error {
        println!("   Config error field: {_field}");
    }

    Ok(())
}

// 2. Use yoshi_af! to enable auto-correction on your code
yoshi_af! {
    /// Performs a risky network operation with error patterns for auto-correction detection
    pub fn risky_network_operation(url: Option<String>) -> Hatch<String> {
        // The auto-correction system will detect these patterns:
        let endpoint = url.ok_or_else(|| yoshi!(message: "URL is required"))?; // ✅ Fixed: proper error handling

        // ⚠️ panic pattern - will suggest proper error handling
assert!(!endpoint.is_empty(), "Empty URL not allowed");

        // Simulate network call that might fail
        let response = std::fs::read_to_string("response.json")
            .map_err(|e| yoshi!(error: e, with_signpost = "Check if response.json exists"))?;

        if response.len() > 1000 {
            todo!("Implement chunked response handling"); // ⚠️ todo pattern - will suggest implementation
        }

        Ok(response)
    }

    /// Processes data with intentional error patterns for auto-correction detection
    pub fn data_processing_with_errors(data: &mut [u8]) -> Hatch<usize> {
        if data.is_empty() {
            unreachable!("Empty data should be filtered upstream"); // ⚠️ unreachable pattern
        }

        // Simulate some processing that might have error patterns
        if data.len() > 1000 {
            todo!("Implement chunked processing for large data"); // ⚠️ todo pattern
        }

        data[0] = 42; // Safe data modification
        Ok(data.len())
    }

    /// Async operation with error patterns for auto-correction detection
    pub async fn async_operation_with_errors(input: Option<String>) -> Hatch<String> {
        let value = input.expect("Input must be provided"); // ⚠️ expect pattern

        // Simulate async work
        tokio::time::sleep(Duration::from_millis(100)).await;

        if value.contains("error") {
            todo!("Implement error case handling"); // ⚠️ todo in async context
        }

        Ok(value.to_uppercase())
    }
}

// 3. Comprehensive auto-correction analysis and application
#[tokio::main]
async fn main() -> Hatch<()> {
    println!("🍄 Yoshi Auto-Correction System Example 🍄\n");

    // Demonstrate error types to show their usage
    println!("📋 Demonstrating error type definitions...");
    if let Err(e) = demonstrate_error_types() {
        println!("   Error type demo failed: {e}");
    } else {
        println!("   ✅ Error types are properly defined and usable");
    }
    println!();

    // Configure the auto-correction system
    let config = SystemConfig {
        auto_apply_safe_corrections: false,  // Review before applying
        min_confidence_threshold: 0.8,       // High confidence only
        min_safety_level: SafetyLevel::Safe, // Safe corrections only
        create_backup_files: true,           // Always create backups
        max_concurrent_operations: 4,        // Parallel processing
        enable_docs_scraping: true,          // Enhanced suggestions
        ..Default::default()
    };

    let system = YoshiACSystem::with_config(config);

    // Analyze the current project for error patterns
    println!("🔍 Analyzing project for error patterns...");
    let corrections = system
        .analyze_and_correct(Path::new("."))
        .await
        .lay("Failed to analyze project")?;

    if corrections.is_empty() {
        println!("✅ No error patterns found - your code is already well-structured!");
        return Ok(());
    }

    println!("📊 Found {} potential corrections:\n", corrections.len());

    // Display detailed analysis results
    for (i, correction) in corrections.iter().enumerate() {
        println!("{}. 📁 File: {}", i + 1, correction.file_path.display());
        println!("   ⚠️  Issue: {}", correction.diagnostic.message);
        println!(
            "   📍 Location: Line {}",
            correction
                .diagnostic
                .primary_span()
                .map_or_else(|| "unknown".to_string(), |s| s.line_start.to_string())
        );

        // Show correction proposals
        for (j, proposal) in correction.proposals.iter().enumerate() {
            println!("   🔧 Fix {}: {}", j + 1, proposal.strategy_description());
            println!("      Confidence: {:.1}%", proposal.confidence * 100.0);
            println!("      Safety: {}", proposal.safety_level);

            if !proposal.corrected_code.is_empty() {
                println!("      Suggested code:");
                for line in proposal.corrected_code.lines().take(3) {
                    println!("        {line}");
                }
                if proposal.corrected_code.lines().count() > 3 {
                    println!("        ...");
                }
            }
        }
        println!();
    }

    // Demonstrate safe auto-application
    println!("🛡️ Applying safe corrections automatically...");
    let applied = system
        .apply_corrections(&corrections, false) // false = only safe corrections
        .await
        .lay("Failed to apply corrections")?;

    if applied.is_empty() {
        println!("ℹ️  No safe corrections were applied automatically.");
        println!("   Review the suggestions above and apply manually if desired.");
    } else {
        println!("✅ Applied {} safe corrections:", applied.len());
        for correction in &applied {
            println!(
                "   - {}: Applied correction",
                correction.file_path.display()
            );
        }
    }

    // Show system metrics
    let metrics = system.get_metrics();
    println!("\n📈 System Performance Metrics:");
    println!(
        "   Diagnostic cache hit ratio: {:.1}%",
        metrics.diagnostic_metrics.cache_hit_ratio * 100.0
    );
    println!(
        "   AST cache hit ratio: {:.1}%",
        metrics.ast_metrics.cache_hit_ratio * 100.0
    );
    println!(
        "   Total corrections generated: {}",
        metrics.generation_metrics.corrections_generated
    );

    println!("\n🎉 Auto-correction analysis complete!");
    println!("💡 Tip: Use 'yoshi_af!' around your functions to enable real-time error detection");
    println!("💡 Tip: Use '#[derive(YoshiError)]' for custom error types with rich metadata");

    Ok(())
}

// 4. Example of testing auto-correction functionality
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio::fs;

    #[tokio::test]
    async fn test_auto_correction_detection() -> Hatch<()> {
        // Create a test project with error patterns
        let temp_dir = tempdir()
            .map_err(|e| yoshi!(error: e, with_signpost = "Check filesystem permissions"))
            .lay("Failed to create temp dir")?;

        let test_code = r#"
use yoshi::*;

yoshi_af! {
    pub fn test_function(opt: Option<String>) -> String {
        opt.expect("Operation should succeed at line 252") // This should be detected
    }
}
"#;

        // Write test files
        fs::write(
            temp_dir.path().join("Cargo.toml"),
            r#"
[package]
name = "test-project"
version = "0.1.0"
edition = "2021"

[dependencies]
yoshi = { path = "../yoshi" }
"#,
        )
        .await
        .map_err(|e| yoshi!(error: e))
        .lay("Failed to write Cargo.toml")?;

        let src_dir = temp_dir.path().join("src");
        fs::create_dir(&src_dir)
            .await
            .map_err(|e| yoshi!(error: e))
            .lay("Failed to create src dir")?;
        fs::write(src_dir.join("lib.rs"), test_code)
            .await
            .map_err(|e| yoshi!(error: e))
            .lay("Failed to write test code")?;

        // Run auto-correction analysis
        let system = YoshiACSystem::new();
        let corrections = system
            .analyze_and_correct(temp_dir.path())
            .await
            .lay("Auto-correction analysis failed")?;

        // Verify corrections were found
        assert!(
            !corrections.is_empty(),
            "Expected auto-corrections to be found"
        );

        println!(
            "✅ Auto-correction test passed - found {} corrections",
            corrections.len()
        );
        Ok(())
    }
}
