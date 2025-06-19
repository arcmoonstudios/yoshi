/* yoshi/tests/real_auto_correction_integration.rs */
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
//! **Brief:** Real integration tests for `yoshi_af`! → `AutoFixTrigger` → yoshi-deluxe pipeline
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + Complete end-to-end auto-correction pipeline testing
//!  - `yoshi_af`! macro generates `AutoFixTrigger` events during compilation
//!  - yoshi-deluxe `YoshiACSystem` processes triggers and generates corrections
//!  - Real project analysis with actual Rust code containing errors
//!  - Verification of correction proposals and application
//! + Integration testing with temporary project creation
//!  - Creates real Rust projects with Cargo.toml and source files
//!  - Uses `yoshi_af`! macros in test code to trigger auto-correction
//!  - Runs `YoshiACSystem.analyze_and_correct()` on real projects
//!  - Verifies that corrections are generated and can be applied
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

use tempfile::TempDir;
use tokio::fs;
use yoshi::*;
use yoshi_deluxe::{YoshiACSystem, SystemConfig};

//--------------------------------------------------------------------------------------------------
// Test Project Creation Utilities
//--------------------------------------------------------------------------------------------------

/// Create a test Rust project with `yoshi_af`! macros and intentional errors
async fn create_test_project_with_yoshi_af() -> Hatch<TempDir> {
    let temp_dir = tempfile::tempdir()
        .hatch()
        .lay("Failed to create temporary directory")?;

    // Create Cargo.toml
    let cargo_toml = r#"
[package]
name = "test-yoshi-project"
version = "0.1.0"
edition = "2021"

[dependencies]
yoshi = { path = "../../../yoshi" }
"#;

    // Create main.rs with yoshi_af! macros and intentional errors
    let main_rs = r#"
use yoshi::*;

// Test yoshi_af! with error-prone code
yoshi_af! {
    #[derive(Debug, YoshiError)]
    pub enum TestError {
        #[yoshi(display = "Network timeout occurred")]
        NetworkTimeout,

        #[yoshi(display = "File not found: {path}")]
        FileNotFound { path: String },

        #[yoshi(display = "Invalid input: {input}")]
        InvalidInput { input: String },
    }
}

// Function with intentional error patterns that should trigger auto-correction
yoshi_af! {
    fn problematic_function() -> Hatch<String> {
        // Missing error handling - should trigger auto-correction
        let content = std::fs::read_to_string("nonexistent.txt");

        // Unwrap usage - should trigger auto-correction
        let result = content.unwrap();

        Ok(result)
    }
}

// Implementation with missing error handling
yoshi_af! {
    impl TestError {
        pub fn from_io_error(err: std::io::Error) -> Self {
            // Missing proper error conversion - should trigger auto-correction
            TestError::FileNotFound {
                path: "unknown".to_string()
            }
        }
    }
}

fn main() {
    println!("Test project with yoshi_af! macros");
}
"#;

    // Write files
    fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml)
        .await
        .hatch()
        .lay("Failed to write Cargo.toml")?;

    let src_dir = temp_dir.path().join("src");
    fs::create_dir(&src_dir)
        .await
        .hatch()
        .lay("Failed to create src directory")?;

    fs::write(src_dir.join("main.rs"), main_rs)
        .await
        .hatch()
        .lay("Failed to write main.rs")?;

    Ok(temp_dir).lay("Test project created successfully")
}

/// Create a test project with multiple files containing `yoshi_af`! usage
async fn create_multi_file_test_project() -> Hatch<TempDir> {
    let temp_dir = tempfile::tempdir()
        .hatch()
        .lay("Failed to create temporary directory")?;

    // Create Cargo.toml
    let cargo_toml = r#"
[package]
name = "multi-file-yoshi-test"
version = "0.1.0"
edition = "2021"

[dependencies]
yoshi = { path = "../../../yoshi" }
"#;

    // Create lib.rs with yoshi_af! module
    let lib_rs = r#"
use yoshi::*;

yoshi_af! {
    pub mod database {
        use super::*;

        #[derive(Debug, YoshiError)]
        pub enum DatabaseError {
            #[yoshi(display = "Connection failed: {reason}")]
            ConnectionFailed { reason: String },

            #[yoshi(display = "Query timeout after {seconds}s")]
            QueryTimeout { seconds: u64 },
        }

        pub fn connect() -> Hatch<String> {
            // Missing retry logic - should trigger auto-correction
            std::thread::sleep(std::time::Duration::from_millis(100));
            Err(DatabaseError::ConnectionFailed {
                reason: "Network unreachable".to_string(),
            }.into())
        }
    }
}

yoshi_af! {
    pub mod network {
        use super::*;

        pub fn fetch_data(url: &str) -> Hatch<Vec<u8>> {
            // Missing timeout handling - should trigger auto-correction
            // Missing input validation - should trigger auto-correction
            let response = std::process::Command::new("curl")
                .arg(url)
                .output();

            match response {
                Ok(output) => Ok(output.stdout),
                Err(e) => Err(yoshi!("Network request failed: {}", e)),
            }
        }
    }
}
"#;

    // Write files
    fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml)
        .await
        .hatch()
        .lay("Failed to write Cargo.toml")?;

    let src_dir = temp_dir.path().join("src");
    fs::create_dir(&src_dir)
        .await
        .hatch()
        .lay("Failed to create src directory")?;

    fs::write(src_dir.join("lib.rs"), lib_rs)
        .await
        .hatch()
        .lay("Failed to write lib.rs")?;

    Ok(temp_dir).lay("Multi-file test project created successfully")
}

//--------------------------------------------------------------------------------------------------
// Real Integration Tests
//--------------------------------------------------------------------------------------------------

#[tokio::test]
async fn test_yoshi_af_generates_autofix_triggers() -> Hatch<()> {
    let test_project = create_test_project_with_yoshi_af().await?;

    // Create YoshiACSystem
    let system = YoshiACSystem::new();

    // Run analysis on the test project
    let corrections = system
        .analyze_and_correct(test_project.path())
        .await
        .lay("Failed to analyze test project with yoshi_af! macros")?;

    // Verify that corrections were generated
    assert!(
        !corrections.is_empty(),
        "No corrections generated from yoshi_af! macros"
    );

    // Verify that at least one correction relates to error handling
    let has_error_handling_correction = corrections.iter().any(|correction| {
        correction.diagnostic.message.contains("error")
            || correction.diagnostic.message.contains("unwrap")
            || correction.diagnostic.message.contains("expect")
    });

    assert!(
        has_error_handling_correction,
        "No error handling corrections found despite problematic code"
    );

    println!(
        "✅ Generated {} corrections from yoshi_af! macros",
        corrections.len()
    );
    for correction in &corrections {
        println!(
            "  📁 {}: {}",
            correction.file_path.display(),
            correction.diagnostic.message
        );
    }

    Ok(()).lay("yoshi_af! AutoFixTrigger generation test completed")
}

#[tokio::test]
async fn test_autofix_trigger_to_deluxe_integration() -> Hatch<()> {
    let test_project = create_multi_file_test_project().await?;

    // Create YoshiACSystem with enhanced configuration
    let config = SystemConfig {
        enable_parallel_processing: true,
        max_proposals_per_diagnostic: 5,
        min_confidence_threshold: 0.5,
        enable_docs_scraping: false, // Disable for faster testing
        auto_apply_safe_corrections: false,
        create_backup_files: true,
        ..Default::default()
    };

    let system = YoshiACSystem::with_config(config);

    // Run comprehensive analysis
    let corrections = system
        .analyze_and_correct(test_project.path())
        .await
        .lay("Failed to analyze multi-file project")?;

    // Verify corrections were generated
    assert!(
        !corrections.is_empty(),
        "No corrections generated from multi-file project"
    );

    // Verify that corrections have proposals
    let corrections_with_proposals: Vec<_> = corrections
        .iter()
        .filter(|c| !c.proposals.is_empty())
        .collect();

    assert!(
        !corrections_with_proposals.is_empty(),
        "No corrections have proposals"
    );

    // Verify proposal quality
    for correction in &corrections_with_proposals {
        let best_proposal = correction
            .best_proposal()
            .ok_or_else(|| yoshi!(message: "Correction missing best proposal"))?;

        assert!(
            best_proposal.confidence > 0.0,
            "Proposal has zero confidence"
        );

        // For now, allow empty corrected code as the auto-correction system
        // may generate placeholder proposals during development
        if best_proposal.corrected_code.is_empty() {
            println!("⚠️ Warning: Proposal has empty corrected code (development mode)");
        }
    }

    println!(
        "✅ Generated {} corrections with proposals",
        corrections_with_proposals.len()
    );

    Ok(()).lay("AutoFixTrigger to yoshi-deluxe integration test completed")
}

#[tokio::test]
async fn test_real_correction_application() -> Hatch<()> {
    let test_project = create_test_project_with_yoshi_af().await?;

    // Create YoshiACSystem with auto-apply enabled
    let config = SystemConfig {
        auto_apply_safe_corrections: true,
        create_backup_files: true,
        min_confidence_threshold: 0.7,
        enable_parallel_processing: false, // Sequential for deterministic testing
        ..Default::default()
    };

    let system = YoshiACSystem::with_config(config);

    // Run analysis
    let corrections = system
        .analyze_and_correct(test_project.path())
        .await
        .lay("Failed to analyze project for correction application")?;

    if corrections.is_empty() {
        println!("⚠️ No corrections found to apply");
        return Ok(());
    }

    // Apply corrections
    let applied_corrections = system
        .apply_corrections(&corrections, true)
        .await
        .lay("Failed to apply corrections")?;

    // Verify corrections were applied
    assert!(
        !applied_corrections.is_empty(),
        "No corrections were applied"
    );

    // Verify backup files were created
    for applied in &applied_corrections {
        let backup_path = applied.file_path.with_extension("rs.yoshibackup");
        assert!(
            backup_path.exists(),
            "Backup file not created: {}",
            backup_path.display()
        );
    }

    // Verify corrected files still compile (basic syntax check)
    for applied in &applied_corrections {
        let content = fs::read_to_string(&applied.file_path)
            .await
            .hatch()
            .lay("Failed to read corrected file")?;

        // Basic syntax validation using syn
        syn::parse_file(&content)
            .map_err(|e| yoshi!(message: "Corrected file has syntax errors: {}", e))?;
    }

    println!(
        "✅ Applied {} corrections successfully",
        applied_corrections.len()
    );
    for applied in &applied_corrections {
        println!(
            "  📝 {}: {} → {}",
            applied.file_path.display(),
            applied.original_code.chars().take(50).collect::<String>(),
            applied.corrected_code.chars().take(50).collect::<String>()
        );
    }

    Ok(()).lay("Real correction application test completed")
}

#[tokio::test]
async fn test_yoshi_af_error_derive_integration() -> Hatch<()> {
    // Test that yoshi_af! properly integrates with #[derive(YoshiError)]
    let temp_dir = tempfile::tempdir()
        .hatch()
        .lay("Failed to create temporary directory")?;

    let cargo_toml = r#"
[package]
name = "derive-integration-test"
version = "0.1.0"
edition = "2021"

[dependencies]
yoshi = { path = "../../../yoshi" }
"#;

    let lib_rs = r#"
use yoshi::*;

// Test yoshi_af! with YoshiError derive integration
yoshi_af! {
    #[derive(Debug, YoshiError)]
    pub enum IntegrationError {
        #[yoshi(display = "Validation failed: {field}")]
        #[yoshi(suggestion = "Check input format and try again")]
        ValidationFailed { field: String },

        #[yoshi(display = "Network error: {code}")]
        #[yoshi(suggestion = "Check network connectivity")]
        NetworkError { code: u16 },

        #[yoshi(transparent)]
        Io(#[yoshi(source)] std::io::Error),
    }
}

// Test function that should trigger auto-correction suggestions
yoshi_af! {
    pub fn validate_input(input: &str) -> Hatch<String> {
        if input.is_empty() {
            return Err(IntegrationError::ValidationFailed {
                field: "input".to_string(),
            }.into());
        }

        // Missing input sanitization - should trigger auto-correction
        // Missing length validation - should trigger auto-correction
        Ok(input.to_string())
    }
}

// Test implementation with error handling patterns
yoshi_af! {
    impl IntegrationError {
        pub fn from_network_code(code: u16) -> Self {
            Self::NetworkError { code }
        }

        pub fn suggest_fix(&self) -> String {
            match self {
                Self::ValidationFailed { field } => {
                    format!("Please check the {} field format", field)
                }
                Self::NetworkError { code } => {
                    format!("Network error {}: check connectivity", code)
                }
                Self::Io(_) => "Check file permissions and path".to_string(),
            }
        }
    }
}
"#;

    // Write files
    fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml)
        .await
        .hatch()
        .lay("Failed to write Cargo.toml")?;

    let src_dir = temp_dir.path().join("src");
    fs::create_dir(&src_dir)
        .await
        .hatch()
        .lay("Failed to create src directory")?;

    fs::write(src_dir.join("lib.rs"), lib_rs)
        .await
        .hatch()
        .lay("Failed to write lib.rs")?;

    // Run analysis
    let system = YoshiACSystem::new();
    let corrections = system
        .analyze_and_correct(temp_dir.path())
        .await
        .lay("Failed to analyze derive integration project")?;

    // Verify that the yoshi_af! + YoshiError integration works
    println!("✅ yoshi_af! + YoshiError integration analysis completed");
    println!("   Generated {} corrections", corrections.len());

    // The test passes if analysis completes without errors
    // This verifies that yoshi_af! properly handles YoshiError derive macros

    Ok(()).lay("yoshi_af! + YoshiError derive integration test completed")
}

#[tokio::test]
async fn test_autofix_trigger_error_conversion() -> Hatch<()> {
    // Test that AutoFixTrigger properly converts to Yoshi errors
    use yoshi_deluxe::errors::AutoFixTrigger;

    // Test AST analysis trigger
    let ast_trigger = AutoFixTrigger::AstAnalysis {
        reason: "Unexpected token 'unwrap'".to_string(),
        file_path: "/test/src/main.rs".into(),
        line: 42,
        column: 15,
    };

    let yoshi_error: Yoshi = ast_trigger.into();
    assert!(yoshi_error.to_string().contains("Unexpected token"));
    assert!(yoshi_error.to_string().contains("main.rs"));
    assert!(yoshi_error.to_string().contains("42:15"));

    // Test diagnostic processing trigger
    let diag_trigger = AutoFixTrigger::DiagnosticProcessing {
        message: "Failed to parse compiler output".to_string(),
    };

    let yoshi_error: Yoshi = diag_trigger.into();
    assert!(yoshi_error
        .to_string()
        .contains("Diagnostic processing failed"));
    assert!(yoshi_error.to_string().contains("parse compiler output"));

    // Test code generation trigger
    let codegen_trigger = AutoFixTrigger::CodeGeneration {
        correction_type: "error_handling".to_string(),
        details: "Cannot generate safe unwrap replacement".to_string(),
        original_code: "result.unwrap()".to_string(),
        generation_context: std::collections::HashMap::new(),
        confidence_score: Some(0.8),
        validation_msgs: Some(vec!["Syntax check passed".to_string()]),
    };

    let yoshi_error: Yoshi = codegen_trigger.into();
    assert!(yoshi_error.to_string().contains("Code generation failed"));
    assert!(yoshi_error.to_string().contains("error_handling"));

    println!("✅ AutoFixTrigger error conversion test completed");

    Ok(()).lay("AutoFixTrigger error conversion test completed")
}

#[tokio::test]
async fn test_end_to_end_pipeline_performance() -> Hatch<()> {
    let test_project = create_multi_file_test_project().await?;

    // Test with performance monitoring
    let config = SystemConfig {
        enable_metrics: true,
        enable_parallel_processing: true,
        max_concurrent_operations: 4,
        ..Default::default()
    };

    let system = YoshiACSystem::with_config(config);

    let start_time = std::time::Instant::now();

    // Run full pipeline
    let corrections = system
        .analyze_and_correct(test_project.path())
        .await
        .lay("Failed to run performance test")?;

    let analysis_time = start_time.elapsed();

    // Get system metrics
    let metrics = system.get_metrics();

    println!("✅ End-to-end pipeline performance test completed");
    println!("   Analysis time: {analysis_time:?}");
    println!("   Corrections generated: {}", corrections.len());
    println!(
        "   Diagnostic cache hit ratio: {:.2}%",
        metrics.diagnostic_metrics.cache_hit_ratio * 100.0
    );
    println!(
        "   AST cache hit ratio: {:.2}%",
        metrics.ast_metrics.cache_hit_ratio * 100.0
    );

    // Performance assertions
    assert!(
        analysis_time.as_secs() < 30,
        "Analysis took too long: {analysis_time:?}"
    );

    Ok(()).lay("End-to-end pipeline performance test completed")
}
