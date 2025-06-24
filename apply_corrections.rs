/* apply_corrections.rs - Use existing error correction strategies */
//! **Apply Yoshi Error Correction Strategies**
//!
//! This script uses the existing error correction framework to systematically
//! fix warnings in yoshi-std using the implemented strategies.

use std::path::PathBuf;
use yoshi_std::utils::error_corrector::YoshiDeriveErrorCorrector;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Starting systematic error correction using Yoshi strategies...");

    // Initialize the error corrector
    let mut corrector = YoshiDeriveErrorCorrector::new()?;

    // Files with println! warnings that need tracing conversion
    let files_with_println = vec![
        PathBuf::from("yoshi-std/src/utils/rustdoc_gen.rs"),
        PathBuf::from("yoshi-std/src/utils/mod.rs"),
    ];

    println!("📋 Applying println! to tracing corrections...");
    for file_path in &files_with_println {
        if file_path.exists() {
            match corrector.apply_println_to_tracing_corrections(file_path) {
                Ok(report) => {
                    println!(
                        "✅ Applied {} corrections to {}",
                        report.total_corrections,
                        file_path.display()
                    );
                }
                Err(e) => {
                    println!(
                        "❌ Failed to correct {}: {}",
                        file_path.display(),
                        e
                    );
                }
            }
        } else {
            println!("⚠️ File not found: {}", file_path.display());
        }
    }

    // Apply general error corrections to all yoshi-std files
    let yoshi_std_files = vec![
        PathBuf::from("yoshi-std/src/lib.rs"),
        PathBuf::from("yoshi-std/src/api/mod.rs"),
        PathBuf::from("yoshi-std/src/api/async_utils.rs"),
        PathBuf::from("yoshi-std/src/api/tracing_integration.rs"),
        PathBuf::from("yoshi-std/src/analytics/mod.rs"),
        PathBuf::from("yoshi-std/src/analytics/semantic_framework.rs"),
        PathBuf::from("yoshi-std/src/conversions/mod.rs"),
        PathBuf::from("yoshi-std/src/std_integration/mod.rs"),
        PathBuf::from("yoshi-std/src/utils/mod.rs"),
        PathBuf::from("yoshi-std/src/utils/backup_manager.rs"),
        PathBuf::from("yoshi-std/src/utils/error_corrector.rs"),
        PathBuf::from("yoshi-std/src/utils/rustdoc_gen.rs"),
    ];

    println!("🔧 Applying general error corrections...");
    match corrector.apply_error_corrections(&yoshi_std_files) {
        Ok(report) => {
            println!("✅ Error correction completed successfully!");
            println!("   📁 Files corrected: {}", report.corrected_files.len());
            println!("   🔧 Total corrections: {}", report.total_corrections);
            if !report.warnings.is_empty() {
                println!("   ⚠️ Warnings: {}", report.warnings.len());
                for warning in &report.warnings {
                    println!("      - {}", warning);
                }
            }
            if report.rollback_triggered {
                println!("   🚨 Rollback was triggered due to validation failures");
            }
        }
        Err(e) => {
            println!("❌ Error correction failed: {}", e);
        }
    }

    println!("🎉 Systematic error correction completed!");
    println!("💡 Run `cargo check` and `cargo clippy` to verify the corrections.");

    Ok(())
}
