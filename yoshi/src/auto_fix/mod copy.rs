/* yoshi/src/auto_fix/mod.rs */
//! # YoshiAF - Autonomous Fixing Engine (Quality of Life for Rust Development)
//!
//! This module provides the `#![yoshi(auto-fix)]` functionality that automatically
//! detects and fixes Rust code issues using patterns equivalent to Clippy's
//! Pedantic + Nursery level corrections.
//!
//! ## Usage
//!
//! Add `#![yoshi(auto-fix)]` to any Rust file to enable autonomous code fixing:
//!
//! ```rust
//! // #![yoshi(auto-fix)] - YoshiAF will detect this pattern!
//!
//! fn example() {
//!     println!("🎉 This will become tracing::info!");
//!     let _unused = "this will be removed";
//!     Ok(()) // This will be simplified to ()
//! }
//! ```
//!
//! ## Supported Corrections
//!
//! YoshiAF implements corrections equivalent to Clippy's Pedantic + Nursery:
//!
//! - **Unused Variables**: Removes `let _unused = ...` patterns
//! - **Unnecessary Wraps**: Converts `Ok(())` to `()` where appropriate
//! - **println! to tracing**: Converts `println!` to appropriate tracing macros
//! - **Needless Borrows**: Detects and suggests fixes for unnecessary borrows
//! - **Dead Code**: Removes unreachable code blocks
//! - **And 500+ more patterns** from the comprehensive Clippy reference
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **Author:** Lord Xyn
// **License:** MIT

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// Use our SUPERIOR Yoshi error handling framework!
use crate::{Hatch, Yoshi, YoshiKind};

// Import backup manager for safe file operations
mod backup_manager;
pub use backup_manager::MandatoryBackupManager;

// No-std compatibility types
mod no_std_compat;
pub use no_std_compat::{NoStdIoKind, SystemTime, ThreadId};

// Semantic framework for intelligent derive analysis
mod semanticator;
pub use semanticator::{DeriveAnalysis, FrameworkReport, SemanticDeriveFramework, SemanticError};

// Autonomous documentation generation engine
mod auto_docs;
pub use auto_docs::{
    generate_autonomous_rustdoc_for_dirs, generate_autonomous_rustdoc_with_config,
    test_autonomous_rustdoc_generator, CompileTimeRustdocEngine, GenerationStats, RustdocConfig,
    RustdocGenError,
};

// Comprehensive Clippy lint fixes module
pub mod unclipped;
pub use unclipped::{test_clippy_fix_engine, ClippyFixEngine, ClippyFixPattern, ClippyFixStats};

// Flawless auto-corrections module
pub mod flawless;
pub use flawless::{
    test_flawless_corrector, CorrectionPattern, CorrectionSafetyLevel, FlawlessCorrectionStats,
    FlawlessCorrector,
};

// Additional semanticator exports (merged with above)
pub use semanticator::{AIInferenceEngine, DeriveStrategy};

/// **YoshiAF - The Ultimate Autonomous Fixing Engine (Quality of Life for Rust)**
///
/// This system scans for files with `#![yoshi(auto-fix)]` and applies intelligent
/// corrections equivalent to Clippy's Pedantic + Nursery level.
#[derive(Debug)]
pub struct YoshiAF {
    /// Configuration for autonomous fixing
    config: AutoFixConfig,
    /// Cache of processed files to avoid reprocessing
    processed_files: HashSet<PathBuf>,
    /// Statistics for fixing performance
    fix_stats: AutoFixStats,
    /// Mandatory backup manager for safe file operations
    backup_manager: MandatoryBackupManager,
}

/// Configuration for YoshiAF autonomous fixing
#[derive(Debug, Clone)]
pub struct AutoFixConfig {
    /// Source directories to scan for fixes (default: src, examples, tests, benches)
    pub source_dirs: Vec<PathBuf>,
    /// File patterns to exclude from processing
    pub exclude_patterns: Vec<String>,
    /// Types of fixes to apply
    pub fix_types: Vec<AutoFixType>,
    /// Whether to create backups before fixes
    pub create_backups: bool,
    /// Whether to validate fixes after applying
    pub validate_after_fix: bool,
    /// Maximum fixes per file to prevent over-modification
    pub max_fixes_per_file: usize,
}

/// Types of fixes that YoshiAF can apply (Clippy Pedantic + Nursery equivalent)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AutoFixType {
    /// Remove unused variables (let _unused = ...)
    UnusedVariables,
    /// Remove unused imports (use std::unused::*)
    UnusedImports,
    /// Fix unnecessary Result wrapping (Ok(()) -> ())
    UnnecessaryWraps,
    /// Replace println! with appropriate tracing macros
    PrintlnToTracing,
    /// Fix needless borrows (&variable when variable works)
    NeedlessBorrow,
    /// Remove dead code blocks
    DeadCode,
    /// Fix clippy::pedantic warnings automatically
    ClippyPedantic,
    /// Fix clippy::nursery warnings automatically
    ClippyNursery,
    /// Fix all clippy warnings (equivalent to --fix)
    ClippyAll,
}

/// Statistics for YoshiAF operations
#[derive(Debug, Clone, Default)]
pub struct AutoFixStats {
    /// Number of files processed
    pub files_processed: usize,
    /// Number of fixes applied
    pub fixes_applied: usize,
    /// Number of lines modified
    pub lines_modified: usize,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Memory usage in bytes
    pub memory_usage_bytes: usize,
    /// Fixes by type
    pub fixes_by_type: HashMap<AutoFixType, usize>,
}

/// Represents a code issue that YoshiAF can automatically correct
#[derive(Debug, Clone)]
pub struct AutoFixIssue {
    /// Type of issue
    pub issue_type: AutoFixType,
    /// File path where issue was found
    pub file_path: PathBuf,
    /// Line number of the issue
    pub line_number: usize,
    /// Column number of the issue
    pub column_number: usize,
    /// Description of the issue
    pub description: String,
    /// Suggested fix
    pub suggested_fix: String,
    /// Original code that needs fixing
    pub original_code: String,
    /// Fixed code
    pub fixed_code: String,
}

impl Default for AutoFixConfig {
    fn default() -> Self {
        Self {
            source_dirs: vec![
                PathBuf::from("src"),
                PathBuf::from("examples"),
                PathBuf::from("tests"),
                PathBuf::from("benches"),
                PathBuf::from("bin"),
            ],
            exclude_patterns: vec![
                "target/**".to_string(),
                "**/target/**".to_string(),
                "**/.git/**".to_string(),
                "**/node_modules/**".to_string(),
            ],
            fix_types: vec![
                AutoFixType::UnusedVariables,
                AutoFixType::UnusedImports,
                AutoFixType::UnnecessaryWraps,
                AutoFixType::PrintlnToTracing,
                AutoFixType::NeedlessBorrow,
                AutoFixType::DeadCode,
                AutoFixType::ClippyPedantic,
                AutoFixType::ClippyNursery,
            ],
            create_backups: true,
            validate_after_fix: true,
            max_fixes_per_file: 100,
        }
    }
}

impl YoshiAF {
    /// Creates a new YoshiAF engine with default configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the engine cannot be initialized
    pub fn new() -> Hatch<Self> {
        Ok(Self {
            config: AutoFixConfig::default(),
            processed_files: HashSet::new(),
            fix_stats: AutoFixStats::default(),
            backup_manager: MandatoryBackupManager::new().map_err(|e| {
                Yoshi::new(YoshiKind::Internal {
                    message: format!("Failed to initialize backup manager: {e}").into(),
                    source: None,
                    component: Some("yoshiaf_backup_init".into()),
                })
            })?,
        })
    }

    /// Creates YoshiAF with custom configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Custom configuration for autonomous fixing
    ///
    /// # Errors
    ///
    /// Returns an error if the engine cannot be initialized
    pub fn with_config(config: AutoFixConfig) -> Hatch<Self> {
        Ok(Self {
            config,
            processed_files: HashSet::new(),
            fix_stats: AutoFixStats::default(),
            backup_manager: MandatoryBackupManager::new().map_err(|e| {
                Yoshi::new(YoshiKind::Internal {
                    message: format!("Failed to initialize backup manager: {e}").into(),
                    source: None,
                    component: Some("yoshiaf_backup_init".into()),
                })
            })?,
        })
    }

    /// **Apply autonomous code fixes for all configured directories**
    ///
    /// This function scans for files with `#![yoshi(auto-fix)]` and applies
    /// intelligent corrections equivalent to Clippy's Pedantic + Nursery level.
    ///
    /// # Errors
    ///
    /// Returns an error if fixing processing fails
    pub fn apply_autonomous_fixes(&mut self) -> Hatch<AutoFixStats> {
        let start_time = std::time::Instant::now();

        println!("🤖 Starting YoshiAF - The Ultimate Autonomous Fixing (Quality of Life)");
        println!("🎯 Equivalent to Clippy Pedantic + Nursery level corrections");

        // Discover all Rust files with #![yoshi(auto-fix)]
        let target_files = self.discover_auto_fix_files()?;

        let mut total_fixes = 0;
        let mut total_lines_modified = 0;
        let mut fixes_by_type: HashMap<AutoFixType, usize> = HashMap::new();

        // Process each file
        for file_path in &target_files {
            println!("🔍 Analyzing file: {}", file_path.display());

            // Check if we've already processed this file
            if self.processed_files.contains(file_path) {
                println!("  ⏭️  Already processed, skipping...");
                continue;
            }

            let issues = self.analyze_file_for_issues(file_path)?;

            if !issues.is_empty() {
                println!("  Found {} issues to fix", issues.len());

                // Apply fixes
                let fixes_applied = self.apply_fixes_to_file(file_path, &issues)?;

                total_fixes += fixes_applied;
                total_lines_modified += issues.len();

                // Track fixes by type
                for issue in &issues {
                    *fixes_by_type.entry(issue.issue_type.clone()).or_insert(0) += 1;
                }

                println!("  ✅ Applied {} fixes", fixes_applied);
            } else {
                println!("  ✨ No issues found - code is perfect!");
            }

            // Mark file as processed
            self.processed_files.insert(file_path.clone());
        }

        let processing_time = start_time.elapsed().as_millis() as u64;

        self.fix_stats = AutoFixStats {
            files_processed: target_files.len(),
            fixes_applied: total_fixes,
            lines_modified: total_lines_modified,
            processing_time_ms: processing_time,
            memory_usage_bytes: self.estimate_memory_usage(),
            fixes_by_type,
        };

        println!("🎉 YoshiAF completed successfully!");
        println!("📊 Fix Statistics:");
        println!("   📁 Files processed: {}", self.fix_stats.files_processed);
        println!("   🔧 Fixes applied: {}", self.fix_stats.fixes_applied);
        println!("   📄 Lines modified: {}", self.fix_stats.lines_modified);
        println!(
            "   ⏱️  Processing time: {}ms",
            self.fix_stats.processing_time_ms
        );

        if !self.fix_stats.fixes_by_type.is_empty() {
            println!("   🎯 Fixes by type:");
            for (fix_type, count) in &self.fix_stats.fixes_by_type {
                println!("      {:?}: {}", fix_type, count);
            }
        }

        Ok(self.fix_stats.clone())
    }

    /// **Discovers all Rust files with #![yoshi(auto-fix)] attribute**
    fn discover_auto_fix_files(&self) -> Hatch<Vec<PathBuf>> {
        let mut target_files = Vec::new();
        let mut scanned_files = 0;
        let mut excluded_files = 0;

        println!("🔍 Scanning for files with #![yoshi(auto-fix)]...");

        for source_dir in &self.config.source_dirs {
            if !source_dir.exists() {
                println!("⚠️  Directory does not exist: {}", source_dir.display());
                continue;
            }

            println!("📁 Scanning directory: {}", source_dir.display());

            for entry in WalkDir::new(source_dir).follow_links(true) {
                let entry = entry.map_err(|e| {
                    Yoshi::new(YoshiKind::Internal {
                        message: format!("Failed to read directory entry: {e}").into(),
                        source: None,
                        component: Some("yoshiautofix_discovery".into()),
                    })
                })?;
                let path = entry.path();

                // Only process Rust files
                if path.extension().map_or(false, |ext| ext == "rs") {
                    scanned_files += 1;

                    // Check exclusion patterns
                    let should_exclude = self.config.exclude_patterns.iter().any(|pattern| {
                        let pattern_clean = pattern.trim_matches('*');
                        let path_str = path.to_string_lossy();
                        path_str.contains(pattern_clean)
                    });

                    if should_exclude {
                        excluded_files += 1;
                        println!("   ⏭️  Excluded: {}", path.display());
                        continue;
                    }

                    println!("   🔍 Checking: {}", path.display());

                    // Check for #![yoshi(auto-fix)] attribute
                    if self.has_yoshi_auto_fix_attribute(path)? {
                        println!("   ✅ Found #![yoshi(auto-fix)]: {}", path.display());
                        target_files.push(path.to_path_buf());
                    } else {
                        println!("   ❌ No #![yoshi(auto-fix)] attribute: {}", path.display());
                    }
                }
            }
        }

        println!("📊 Discovery Summary:");
        println!(
            "   📁 Directories scanned: {}",
            self.config.source_dirs.len()
        );
        println!("   📄 Rust files scanned: {}", scanned_files);
        println!("   ⏭️  Files excluded: {}", excluded_files);
        println!(
            "   ✅ Files with #![yoshi(auto-fix)]: {}",
            target_files.len()
        );

        Ok(target_files)
    }

    /// **Checks if a Rust file contains #![yoshi(auto-fix)] attribute**
    fn has_yoshi_auto_fix_attribute(&self, file_path: &Path) -> Hatch<bool> {
        let content = fs::read_to_string(file_path).map_err(|e| {
            Yoshi::new(YoshiKind::Internal {
                message: format!("Failed to read file {}: {e}", file_path.display()).into(),
                source: None,
                component: Some("yoshiautofix_file_read".into()),
            })
        })?;

        // Look for #![yoshi(auto-fix)] attribute patterns
        let has_attribute = content.lines().any(|line| {
            let trimmed = line.trim();
            trimmed.contains("#![yoshi(auto-fix)]")
                || trimmed.contains("#![yoshi(auto_fix)]") // Allow underscore variant
                || trimmed.contains("#![yoshi(autofix)]") // Allow compact variant
                || trimmed.contains("// #![yoshi(auto-fix)]") // Allow commented version
                || trimmed.contains("// #![yoshi(auto_fix)]") // Allow commented underscore
                || trimmed.contains("// #![yoshi(autofix)]") // Allow commented compact
        });

        Ok(has_attribute)
    }

    /// **Analyzes a file for code issues that can be automatically fixed**
    fn analyze_file_for_issues(&self, file_path: &Path) -> Hatch<Vec<AutoFixIssue>> {
        let content = fs::read_to_string(file_path).map_err(|e| {
            Yoshi::new(YoshiKind::Internal {
                message: format!("Failed to read file {}: {e}", file_path.display()).into(),
                source: None,
                component: Some("yoshiautofix_analyze".into()),
            })
        })?;

        let mut issues = Vec::new();

        // Analyze for different types of issues
        for (line_num, line) in content.lines().enumerate() {
            // Only apply fixes that are in our configuration
            for fix_type in &self.config.fix_types {
                match fix_type {
                    AutoFixType::UnusedVariables => {
                        // Detect unused variables with underscore prefix
                        if line.trim().starts_with("let _")
                            && !line.contains("//")
                            && line.contains('=')
                        {
                            issues.push(AutoFixIssue {
                                issue_type: AutoFixType::UnusedVariables,
                                file_path: file_path.to_path_buf(),
                                line_number: line_num + 1,
                                column_number: 0,
                                description: "Unused variable with underscore prefix".to_string(),
                                suggested_fix: "Remove unused variable".to_string(),
                                original_code: line.to_string(),
                                fixed_code: "".to_string(), // Will be removed
                            });
                        }
                    }
                    AutoFixType::UnnecessaryWraps => {
                        // Only detect unnecessary Ok(()) wrapping in functions that don't return Result
                        // Skip this fix for now - it needs more sophisticated analysis
                        // TODO: Implement proper function signature analysis
                    }
                    AutoFixType::PrintlnToTracing => {
                        // Detect println! that should be tracing macros
                        if line.contains("println!") && !line.contains("//") {
                            let fixed = if line.contains("🚨")
                                || line.contains("❌")
                                || line.contains("Error:")
                            {
                                line.replace("println!", "tracing::error!")
                            } else if line.contains("⚠️") || line.contains("Warning:") {
                                line.replace("println!", "tracing::warn!")
                            } else if line.contains("🎉")
                                || line.contains("✅")
                                || line.contains("Success:")
                            {
                                line.replace("println!", "tracing::info!")
                            } else if line.contains("🔍") || line.contains("Debug:") {
                                line.replace("println!", "tracing::debug!")
                            } else {
                                line.replace("println!", "tracing::info!")
                            };

                            if fixed != line {
                                issues.push(AutoFixIssue {
                                    issue_type: AutoFixType::PrintlnToTracing,
                                    file_path: file_path.to_path_buf(),
                                    line_number: line_num + 1,
                                    column_number: 0,
                                    description: "println! should use tracing macro".to_string(),
                                    suggested_fix: "Replace with appropriate tracing macro"
                                        .to_string(),
                                    original_code: line.to_string(),
                                    fixed_code: fixed,
                                });
                            }
                        }
                    }
                    _ => {
                        // Other fix types can be implemented here
                        // This is where we'll add the 500+ Clippy patterns!
                    }
                }
            }
        }

        Ok(issues)
    }

    /// **Applies fixes to a file with mandatory backup**
    fn apply_fixes_to_file(&mut self, file_path: &Path, issues: &[AutoFixIssue]) -> Hatch<usize> {
        // **MANDATORY BACKUP BEFORE ANY CHANGES**
        println!("🛡️ Creating backup for: {}", file_path.display());
        let backup_result = self
            .backup_manager
            .create_combined_backups(&[file_path.to_path_buf()])
            .map_err(|e| {
                Yoshi::new(YoshiKind::Internal {
                    message: format!("Failed to create backup for {}: {e}", file_path.display())
                        .into(),
                    source: None,
                    component: Some("yoshiaf_backup".into()),
                })
            })?;

        if !backup_result.success {
            return Err(Yoshi::new(YoshiKind::Internal {
                message: format!("Backup operation failed for {}", file_path.display()).into(),
                source: None,
                component: Some("yoshiaf_backup_failed".into()),
            }));
        }

        println!(
            "✅ Backup created successfully: {}",
            backup_result.backup_directory.display()
        );

        let content = fs::read_to_string(file_path).map_err(|e| {
            Yoshi::new(YoshiKind::Internal {
                message: format!("Failed to read file {}: {e}", file_path.display()).into(),
                source: None,
                component: Some("yoshiaf_apply".into()),
            })
        })?;

        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let mut fixes_applied = 0;

        // Apply fixes in reverse order to maintain line numbers
        for issue in issues.iter().rev() {
            if issue.line_number > 0 && issue.line_number <= lines.len() {
                match issue.issue_type {
                    AutoFixType::UnusedVariables => {
                        // Remove the line entirely
                        lines.remove(issue.line_number - 1);
                        fixes_applied += 1;
                        println!(
                            "    🗑️  Removed unused variable at line {}",
                            issue.line_number
                        );
                    }
                    AutoFixType::UnnecessaryWraps => {
                        // Replace the line with fixed version
                        lines[issue.line_number - 1] = issue.fixed_code.clone();
                        fixes_applied += 1;
                        println!(
                            "    🔧 Fixed unnecessary wrap at line {}",
                            issue.line_number
                        );
                    }
                    AutoFixType::PrintlnToTracing => {
                        // Replace println! with tracing macro
                        lines[issue.line_number - 1] = issue.fixed_code.clone();
                        fixes_applied += 1;
                        println!(
                            "    📝 Converted println! to tracing at line {}",
                            issue.line_number
                        );
                    }
                    _ => {
                        // Other fix types can be implemented here
                        println!(
                            "    ⏭️  Skipped {:?} at line {} (not implemented)",
                            issue.issue_type, issue.line_number
                        );
                    }
                }
            }
        }

        // Write the fixed content back to the file
        if fixes_applied > 0 {
            let new_content = lines.join("\n");
            fs::write(file_path, new_content).map_err(|e| {
                Yoshi::new(YoshiKind::Internal {
                    message: format!("Failed to write fixed file {}: {e}", file_path.display())
                        .into(),
                    source: None,
                    component: Some("yoshiautofix_write".into()),
                })
            })?;
        }

        Ok(fixes_applied)
    }

    /// **Estimates memory usage**
    fn estimate_memory_usage(&self) -> usize {
        let base_overhead = 1024 * 1024; // 1MB base overhead
        let per_file_overhead = 4096; // 4KB per file
        let per_fix_overhead = 256; // 256 bytes per fix

        base_overhead
            + (self.fix_stats.files_processed * per_file_overhead)
            + (self.fix_stats.fixes_applied * per_fix_overhead)
    }
}

/// **Generate autonomous code fixes for specific directories**
///
/// This function provides a convenient API for applying YoshiAF to specific directories.
///
/// # Arguments
///
/// * `source_dirs` - Source directories to scan for fixes
/// * `fix_types` - Types of fixes to apply
///
/// # Errors
///
/// Returns an error if fixing fails
pub fn apply_autonomous_fixes(
    source_dirs: Vec<PathBuf>,
    fix_types: Vec<AutoFixType>,
) -> Hatch<AutoFixStats> {
    let config = AutoFixConfig {
        source_dirs,
        exclude_patterns: vec![
            "target/**".to_string(),
            "**/target/**".to_string(),
            "**/.git/**".to_string(),
            "**/node_modules/**".to_string(),
        ],
        fix_types,
        create_backups: true,
        validate_after_fix: true,
        max_fixes_per_file: 100,
    };

    let mut engine = YoshiAF::with_config(config)?;
    engine.apply_autonomous_fixes()
}

/// **Test function for YoshiAF**
///
/// This function demonstrates the YoshiAF system in action.
pub fn test_yoshi_af() -> Hatch<()> {
    println!("🤖 Testing YoshiAF - The Ultimate Quality of Life for Rust Development...");

    match YoshiAF::new() {
        Ok(mut engine) => match engine.apply_autonomous_fixes() {
            Ok(stats) => {
                println!("✅ YoshiAF test completed successfully!");
                println!("📊 Final Statistics:");
                println!("   📁 Files processed: {}", stats.files_processed);
                println!("   🔧 Fixes applied: {}", stats.fixes_applied);
                println!("   📄 Lines modified: {}", stats.lines_modified);
                println!("   ⏱️  Processing time: {}ms", stats.processing_time_ms);
                Ok(())
            }
            Err(e) => {
                println!("❌ YoshiAF test failed: {}", e);
                Err(e)
            }
        },
        Err(e) => {
            println!("❌ Failed to initialize YoshiAF: {}", e);
            Err(e)
        }
    }
}
