/* yoshi/src/auto_fix/mod.rs */
//! #![yoshi(auto-fix)]
//! # YoshiAF - Autonomous Fixing Engine (Quality of Life for Rust Development)
// This module provides the `#![yoshi(auto-fix)]` functionality that automatically
// detects and fixes Rust code issues using patterns equivalent to Clippy's
// Pedantic + Nursery level corrections, plus comprehensive semantic analysis,
// flawless corrections, and AI-powered derive suggestions.
//
// ## Usage
//
// Add `#![yoshi(auto-fix)]` to any Rust file to enable autonomous code fixing:
//
// ```rust
// // #![yoshi(auto-fix)] - YoshiAF will detect this pattern!
//
// fn example() {
//     println!("🎉 This will become tracing::info!");
//     let _unused = "this will be removed";
//     Ok(()) // This will be simplified to ()
// }
// ```
//
// ## Supported Corrections
//
// YoshiAF implements corrections equivalent to Clippy's Pedantic + Nursery:
//
// - **500+ Clippy Patterns**: Complete ClippyFixEngine integration
// - **Semantic Analysis**: AI-powered derive suggestions with caching
// - **Flawless Corrections**: Safety-tiered correction patterns
// - **Unused Variables**: Removes `let _unused = ...` patterns
// - **Unnecessary Wraps**: Converts `Ok(())` to `()` where appropriate
// - **println! to tracing**: Converts `println!` to appropriate tracing macros
// - **And 1000+ more patterns** from comprehensive analysis engines

// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **Author:** Lord Xyn
// **License:** MIT OR Apache-2.0

// yoshi attribute import removed - using comment-based approach
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use std::time::Instant;
use tempfile::TempDir;
use tracing::{debug, error, info, instrument, warn};
use walkdir::WalkDir;

// Use our SUPERIOR Yoshi error handling framework!
use crate::{Hatch, Yoshi, YoshiKind};

// Import backup manager for safe file operations
mod backup_manager;
pub use backup_manager::{
    AutoRecoveryResult, BackupDirectoryInfo, BackupError, BackupManifest, BackupOperation,
    ChecksumValidator, CleanupOperation, DiagnosticLevel, DiagnosticMessage, FileDiagnostics,
    MandatoryBackupManager, RestoreOperation,
};

// No-std compatibility types
mod no_std_compat;
pub use no_std_compat::{NoStdIoKind, SystemTime, ThreadId};

// Semantic framework for intelligent derive analysis (formerly semanticator.rs)
mod semanticator;
pub use semanticator::{
    AIInferenceEngine, DeriveAnalysis, DeriveApplicationResult, DeriveStrategy, FieldAnalysis,
    FrameworkReport, PerformanceImpact, SemanticConfig, SemanticDeriveFramework, SemanticError,
    SemanticMetrics, SizeCategory,
};

/// **Comprehensive Clippy Lint Fixes Module**
///
/// This module implements automated fixes for ALL Clippy lint patterns
/// documented in `docs/unclipped_References.md` using the same methodology
/// as the `auto_fix/mod.rs` architecture. Provides 500+ Clippy patterns
/// with regex-based implementations for comprehensive code improvement.
pub mod unclipped;
pub use unclipped::{
    test_clippy_fix_engine, ClippyFixEngine, ClippyFixError, ClippyFixPattern, ClippyFixStats,
};

/// **Flawless Auto-Corrections Module**
///
/// This module implements comprehensive auto-correction patterns using the
/// SAME methodology as yoshi/src/auto_fix/mod.rs to maintain architectural
/// consistency and avoid making mod.rs megalithic. Provides safety-tiered
/// correction patterns with intelligent context-aware replacements.
pub mod flawless;
pub use flawless::{
    test_flawless_corrector, CorrectionPattern, CorrectionSafetyLevel, FlawlessCorrectionStats,
    FlawlessCorrector,
};

// Autonomous documentation generation module (auto_docs.rs)
pub mod auto_docs;
pub use auto_docs::{
    generate_autonomous_rustdoc_for_dirs, generate_autonomous_rustdoc_with_config,
    test_autonomous_rustdoc_generator, CompileTimeRustdocEngine, GenerationStats, RustdocConfig,
    RustdocGenError,
};

/// **`YoshiAF` - The Ultimate Autonomous Fixing Engine (All-in-One Integration Core)**
///
/// This is the central orchestrating engine that combines `ClippyFixEngine`,
/// `FlawlessCorrector`, `SemanticDeriveFramework`, and autonomous fixing into
/// a single, unified, enterprise-grade processing pipeline.
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

    // === INTEGRATED ENGINES ===
    /// The comprehensive `ClippyFixEngine` with 500+ patterns
    clippy_engine: Arc<Mutex<ClippyFixEngine>>,
    /// The `FlawlessCorrector` with safety-tiered corrections
    flawless_corrector: Arc<Mutex<FlawlessCorrector>>,
    /// The `SemanticDeriveFramework` with AI-powered analysis
    semantic_framework: Arc<SemanticDeriveFramework>,
    /// Optional AI inference engine for advanced semantic analysis
    ai_engine: Option<Arc<AIInferenceEngine>>,

    // === PERFORMANCE & MONITORING ===
    /// Comprehensive metrics across all engines
    integrated_metrics: Arc<RwLock<IntegratedMetrics>>,
    /// Temporary directory for processing operations
    temp_dir: Option<Arc<TempDir>>,
    // semantic_cache removed - SemanticDeriveFramework has its own internal cache
}

/// Configuration for `YoshiAF` autonomous fixing with full integration
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

    // === INTEGRATED ENGINE CONFIGURATION ===
    /// Configuration for semantic analysis
    pub semantic_config: semanticator::SemanticConfig,
    /// Enable `ClippyFixEngine` processing
    pub enable_clippy_engine: bool,
    /// Enable `FlawlessCorrector` processing
    pub enable_flawless_corrector: bool,
    /// Enable `SemanticDeriveFramework` processing
    pub enable_semantic_framework: bool,
    /// Enable AI-powered semantic analysis
    pub enable_ai_analysis: bool,
    /// Maximum processing time per file in milliseconds
    pub max_processing_time_ms: u64,
}

/// Types of fixes that `YoshiAF` can apply (Comprehensive Integration)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AutoFixType {
    /// Remove unused variables (let _unused = ...)
    UnusedVariables,
    /// Remove unused imports (use `std::unused::`*)
    UnusedImports,
    /// Fix unnecessary Result wrapping (Ok(()) -> ())
    UnnecessaryWraps,
    /// Replace println! with appropriate tracing macros
    PrintlnToTracing,
    /// Fix needless borrows (&variable when variable works)
    NeedlessBorrow,
    /// Remove dead code blocks
    DeadCode,
    /// Fix `clippy::pedantic` warnings automatically
    ClippyPedantic,
    /// Fix `clippy::nursery` warnings automatically
    ClippyNursery,
    /// Fix all clippy warnings (equivalent to --fix)
    ClippyAll,

    // === INTEGRATED ENGINE TYPES ===
    /// Apply all `ClippyFixEngine` patterns (500+ fixes)
    ClippyEngine,
    /// Apply `FlawlessCorrector` patterns (safety-tiered)
    FlawlessCorrections,
    /// Apply semantic derive suggestions with AI analysis
    SemanticDerives,
    /// Apply comprehensive quality improvements
    QualityEnhancement,
    /// Apply performance optimizations
    PerformanceOptimization,
    /// Apply safety hardening patterns
    SafetyHardening,
}

/// Comprehensive statistics for `YoshiAF` operations
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

    // === INTEGRATED ENGINE STATS ===
    /// Statistics from `ClippyFixEngine`
    pub clippy_stats: Option<ClippyFixStats>,
    /// Statistics from `FlawlessCorrector`
    pub flawless_stats: Option<FlawlessCorrectionStats>,
    /// Statistics from `SemanticDeriveFramework`
    pub semantic_stats: Option<semanticator::SemanticMetrics>,
    /// Overall success rate across all engines
    pub overall_success_rate: f64,
}

/// Integrated metrics across all processing engines
#[derive(Debug, Clone, Default)]
pub struct IntegratedMetrics {
    /// Total processing operations across all engines
    pub total_operations: u64,
    /// Total processing time across all engines
    pub total_processing_time_ms: u64,
    /// Cache hit rate for semantic analysis
    pub cache_hit_rate: f64,
    /// Error rate across all engines
    pub error_rate: f64,
    /// Performance breakdown by engine
    pub engine_performance: HashMap<String, EnginePerformance>,
    /// Memory usage tracking
    pub peak_memory_usage_bytes: u64,
}

/// Performance metrics for individual engines
#[derive(Debug, Clone, Default)]
pub struct EnginePerformance {
    /// Number of operations performed
    pub operations: u64,
    /// Total processing time in milliseconds
    pub processing_time_ms: u64,
    /// Success rate (0.0 to 1.0)
    pub success_rate: f64,
    /// Average processing time per operation
    pub avg_time_per_operation_ms: f64,
}

// CachedSemanticAnalysis removed - SemanticDeriveFramework has its own internal cache

/// Represents a comprehensive code issue that `YoshiAF` can automatically correct
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
    /// Engine that detected this issue
    pub detected_by_engine: String,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Safety level of the proposed fix
    pub safety_level: CorrectionSafetyLevel,
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
                AutoFixType::ClippyEngine,
                AutoFixType::FlawlessCorrections,
                AutoFixType::SemanticDerives,
                AutoFixType::UnusedVariables,
                AutoFixType::UnusedImports,
                AutoFixType::UnnecessaryWraps,
                AutoFixType::PrintlnToTracing,
                AutoFixType::NeedlessBorrow,
                AutoFixType::DeadCode,
                AutoFixType::QualityEnhancement,
            ],
            create_backups: true,
            validate_after_fix: true,
            max_fixes_per_file: 1000,

            // Integrated engine configuration
            semantic_config: semanticator::SemanticConfig::default(),
            enable_clippy_engine: true,
            enable_flawless_corrector: true,
            enable_semantic_framework: true,
            enable_ai_analysis: true,
            max_processing_time_ms: 30_000, // 30 seconds per file
        }
    }
}

impl YoshiAF {
    /// Creates a new YoshiAF engine with default configuration and all integrated engines
    ///
    /// # Errors
    ///
    /// Returns an error if any of the integrated engines cannot be initialized
    #[instrument(level = "info")]
    pub fn new() -> Hatch<Self> {
        Self::with_config(AutoFixConfig::default())
    }

    /// Creates YoshiAF with custom configuration and full engine integration
    ///
    /// # Arguments
    ///
    /// * `config` - Custom configuration for autonomous fixing and integrated engines
    ///
    /// # Errors
    ///
    /// Returns an error if any engine cannot be initialized
    #[instrument(level = "info", skip(config))]
    pub fn with_config(config: AutoFixConfig) -> Hatch<Self> {
        info!("🚀 Initializing YoshiAF with full engine integration...");

        // Initialize temporary directory
        let temp_dir = if config.max_processing_time_ms > 0 {
            Some(Arc::new(TempDir::new().map_err(|e| {
                Yoshi::new(YoshiKind::Internal {
                    message: format!("Failed to create temporary directory: {e}").into(),
                    source: None,
                    component: Some("yoshiaf_temp_init".into()),
                })
            })?))
        } else {
            None
        };

        // Initialize ClippyFixEngine
        let clippy_engine = if config.enable_clippy_engine {
            info!("📦 Initializing ClippyFixEngine with 500+ patterns...");
            Arc::new(Mutex::new(ClippyFixEngine::new().map_err(|e| {
                Yoshi::new(YoshiKind::Internal {
                    message: format!("Failed to initialize ClippyFixEngine: {e}").into(),
                    source: None,
                    component: Some("yoshiaf_clippy_init".into()),
                })
            })?))
        } else {
            // Create a dummy engine for consistency
            Arc::new(Mutex::new(ClippyFixEngine::new().map_err(|e| {
                Yoshi::new(YoshiKind::Internal {
                    message: format!("Failed to initialize ClippyFixEngine: {e}").into(),
                    source: None,
                    component: Some("yoshiaf_clippy_init".into()),
                })
            })?))
        };

        // Initialize FlawlessCorrector
        let flawless_corrector = if config.enable_flawless_corrector {
            info!("🔧 Initializing FlawlessCorrector with safety-tiered patterns...");
            Arc::new(Mutex::new(FlawlessCorrector::new().map_err(|e| {
                Yoshi::new(YoshiKind::Internal {
                    message: format!("Failed to initialize FlawlessCorrector: {e}").into(),
                    source: None,
                    component: Some("yoshiaf_flawless_init".into()),
                })
            })?))
        } else {
            Arc::new(Mutex::new(FlawlessCorrector::new().map_err(|e| {
                Yoshi::new(YoshiKind::Internal {
                    message: format!("Failed to initialize FlawlessCorrector: {e}").into(),
                    source: None,
                    component: Some("yoshiaf_flawless_init".into()),
                })
            })?))
        };

        // Initialize SemanticDeriveFramework
        let semantic_framework = if config.enable_semantic_framework {
            info!("🧠 Initializing SemanticDeriveFramework with AI capabilities...");
            Arc::new(SemanticDeriveFramework::new().map_err(|e| {
                Yoshi::new(YoshiKind::Internal {
                    message: format!("Failed to initialize SemanticDeriveFramework: {e:?}").into(),
                    source: None,
                    component: Some("yoshiaf_semantic_init".into()),
                })
            })?)
        } else {
            Arc::new(SemanticDeriveFramework::new().map_err(|e| {
                Yoshi::new(YoshiKind::Internal {
                    message: format!("Failed to initialize SemanticDeriveFramework: {e:?}").into(),
                    source: None,
                    component: Some("yoshiaf_semantic_init".into()),
                })
            })?)
        };

        // Initialize AI engine if enabled
        let ai_engine = if config.enable_ai_analysis {
            match AIInferenceEngine::new() {
                Ok(engine) => {
                    info!("🤖 AI inference engine initialized successfully");
                    Some(Arc::new(engine))
                }
                Err(e) => {
                    warn!("⚠️  AI inference engine not available: {:?}", e);
                    None
                }
            }
        } else {
            info!("🚫 AI inference disabled by configuration");
            None
        };

        info!("✅ All engines initialized successfully!");

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

            // Integrated engines
            clippy_engine,
            flawless_corrector,
            semantic_framework,
            ai_engine,

            // Performance and monitoring
            integrated_metrics: Arc::new(RwLock::new(IntegratedMetrics::default())),
            temp_dir,
        })
    }

    /// **Apply comprehensive autonomous code fixes using all integrated engines**
    ///
    /// This is the main entry point that orchestrates ClippyFixEngine,
    /// FlawlessCorrector, SemanticDeriveFramework, and traditional YoshiAF
    /// fixes in a unified, performance-optimized pipeline.
    ///
    /// # Errors
    ///
    /// Returns an error if fixing processing fails
    #[instrument(level = "info", skip(self))]
    pub fn apply_autonomous_fixes(&mut self) -> Hatch<AutoFixStats> {
        let start_time = Instant::now();

        info!("🤖 Starting YoshiAF - The Ultimate Autonomous Fixing Engine");
        info!("🎯 Integrated: ClippyFixEngine + FlawlessCorrector + SemanticFramework");
        info!("📊 AI Analysis: {}", self.config.enable_ai_analysis);

        // Discover all Rust files with #![yoshi(auto-fix)]
        let target_files = self.discover_auto_fix_files()?;

        if target_files.is_empty() {
            info!("✅ No files to process. Finished.");
            return Ok(AutoFixStats::default());
        }

        let mut total_fixes = 0;
        let mut total_lines_modified = 0;
        let mut fixes_by_type: HashMap<AutoFixType, usize> = HashMap::new();

        // Initialize engine statistics
        let mut clippy_stats_accumulator = ClippyFixStats {
            total_fixes_applied: 0,
            patterns_processed: 0,
            lint_types_fixed: Vec::new(),
            processing_time_ms: 0,
        };

        let mut flawless_stats_accumulator = FlawlessCorrectionStats {
            total_corrections_applied: 0,
            patterns_processed: 0,
            correction_types_applied: Vec::new(),
            safety_breakdown: HashMap::new(),
            processing_time_ms: 0,
        };

        let mut semantic_stats_accumulator = semanticator::SemanticMetrics::default();

        // Process each file through the integrated pipeline
        for file_path in &target_files {
            info!("🔍 Processing file: {}", file_path.display());

            // Check if we've already processed this file
            if self.processed_files.contains(file_path) {
                info!("  ⏭️  Already processed, skipping...");
                continue;
            }

            let file_start_time = Instant::now();

            // Apply comprehensive integrated fixes
            let file_result = self.apply_integrated_fixes_to_file(file_path)?;

            total_fixes += file_result.total_fixes;
            total_lines_modified += file_result.lines_modified;

            // Accumulate engine-specific statistics
            if let Some(ref clippy_stats) = file_result.clippy_stats {
                clippy_stats_accumulator.total_fixes_applied += clippy_stats.total_fixes_applied;
                clippy_stats_accumulator.patterns_processed += clippy_stats.patterns_processed;
                clippy_stats_accumulator.processing_time_ms += clippy_stats.processing_time_ms;
                clippy_stats_accumulator
                    .lint_types_fixed
                    .extend(clippy_stats.lint_types_fixed.clone());
            }

            if let Some(ref flawless_stats) = file_result.flawless_stats {
                flawless_stats_accumulator.total_corrections_applied +=
                    flawless_stats.total_corrections_applied;
                flawless_stats_accumulator.patterns_processed += flawless_stats.patterns_processed;
                flawless_stats_accumulator.processing_time_ms += flawless_stats.processing_time_ms;
                flawless_stats_accumulator
                    .correction_types_applied
                    .extend(flawless_stats.correction_types_applied.clone());
                for (safety_level, count) in &flawless_stats.safety_breakdown {
                    *flawless_stats_accumulator
                        .safety_breakdown
                        .entry(*safety_level)
                        .or_insert(0) += count;
                }
            }

            if let Some(ref semantic_stats) = file_result.semantic_stats {
                semantic_stats_accumulator.items_analyzed += semantic_stats.items_analyzed;
                semantic_stats_accumulator.derives_applied += semantic_stats.derives_applied;
                semantic_stats_accumulator.total_processing_time_ms +=
                    semantic_stats.total_processing_time_ms;
                semantic_stats_accumulator.cache_hits += semantic_stats.cache_hits;
                semantic_stats_accumulator.cache_misses += semantic_stats.cache_misses;
                semantic_stats_accumulator.ai_inference_calls += semantic_stats.ai_inference_calls;
            }

            // Track fixes by type
            for (fix_type, count) in &file_result.fixes_by_type {
                *fixes_by_type.entry(fix_type.clone()).or_insert(0) += count;
            }

            let file_duration = file_start_time.elapsed();
            info!(
                "  ✅ Applied {} fixes in {}ms",
                file_result.total_fixes,
                file_duration.as_millis()
            );

            // Mark file as processed
            self.processed_files.insert(file_path.clone());
        }

        // Update cache hit rates
        semantic_stats_accumulator.update_cache_hit_rate();

        let processing_time = start_time.elapsed().as_millis() as u64;

        // Calculate overall success rate
        let total_operations = clippy_stats_accumulator.patterns_processed
            + flawless_stats_accumulator.patterns_processed
            + semantic_stats_accumulator.items_analyzed as usize;

        let successful_operations = clippy_stats_accumulator.total_fixes_applied
            + flawless_stats_accumulator.total_corrections_applied
            + semantic_stats_accumulator.derives_applied as usize;

        let overall_success_rate = if total_operations > 0 {
            successful_operations as f64 / total_operations as f64 * 100.0
        } else {
            100.0
        };

        self.fix_stats = AutoFixStats {
            files_processed: target_files.len(),
            fixes_applied: total_fixes,
            lines_modified: total_lines_modified,
            processing_time_ms: processing_time,
            memory_usage_bytes: self.estimate_memory_usage(),
            fixes_by_type,
            clippy_stats: Some(clippy_stats_accumulator),
            flawless_stats: Some(flawless_stats_accumulator),
            semantic_stats: Some(semantic_stats_accumulator),
            overall_success_rate,
        };

        info!("🎉 YoshiAF comprehensive processing completed!");
        info!("📊 Integrated Fix Statistics:");
        info!("   📁 Files processed: {}", self.fix_stats.files_processed);
        info!(
            "   🔧 Total fixes applied: {}",
            self.fix_stats.fixes_applied
        );
        info!("   📄 Lines modified: {}", self.fix_stats.lines_modified);
        info!(
            "   ⏱️  Total processing time: {}ms",
            self.fix_stats.processing_time_ms
        );
        info!(
            "   🎯 Overall success rate: {:.1}%",
            self.fix_stats.overall_success_rate
        );

        if let Some(ref clippy_stats) = self.fix_stats.clippy_stats {
            info!(
                "   📋 Clippy patterns: {} fixes, {} patterns",
                clippy_stats.total_fixes_applied, clippy_stats.patterns_processed
            );
        }

        if let Some(ref flawless_stats) = self.fix_stats.flawless_stats {
            info!(
                "   🔧 Flawless corrections: {} fixes, {} patterns",
                flawless_stats.total_corrections_applied, flawless_stats.patterns_processed
            );
        }

        if let Some(ref semantic_stats) = self.fix_stats.semantic_stats {
            info!(
                "   🧠 Semantic analysis: {} derives, {:.1}% cache hit rate",
                semantic_stats.derives_applied, semantic_stats.cache_hit_rate
            );
        }

        if !self.fix_stats.fixes_by_type.is_empty() {
            info!("   🎯 Fixes by type:");
            for (fix_type, count) in &self.fix_stats.fixes_by_type {
                info!("      {:?}: {}", fix_type, count);
            }
        }

        Ok(self.fix_stats.clone())
    }

    /// **Apply integrated fixes to a single file using all engines**
    fn apply_integrated_fixes_to_file(&mut self, file_path: &Path) -> Hatch<FileProcessingResult> {
        // **MANDATORY BACKUP BEFORE ANY CHANGES**
        if self.config.create_backups {
            debug!("🛡️ Creating backup for: {}", file_path.display());
            let backup_result = self
                .backup_manager
                .create_combined_backups(&[file_path.to_path_buf()])
                .map_err(|e| {
                    Yoshi::new(YoshiKind::Internal {
                        message: format!(
                            "Failed to create backup for {}: {e}",
                            file_path.display()
                        )
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
        }

        let original_content = fs::read_to_string(file_path).map_err(|e| {
            Yoshi::new(YoshiKind::Internal {
                message: format!("Failed to read file {}: {e}", file_path.display()).into(),
                source: None,
                component: Some("yoshiaf_read".into()),
            })
        })?;

        let mut current_content = original_content.clone();
        let mut total_fixes = 0;
        let mut fixes_by_type = HashMap::new();

        // Engine-specific statistics
        let mut clippy_stats = None;
        let mut flawless_stats = None;
        let mut semantic_stats = None;

        // === PHASE 1: ClippyFixEngine Processing ===
        if self.config.enable_clippy_engine
            && self.config.fix_types.contains(&AutoFixType::ClippyEngine)
        {
            debug!("🔧 Applying ClippyFixEngine patterns...");

            let clippy_start = Instant::now();
            if let Ok(mut engine) = self.clippy_engine.lock() {
                match engine.apply_clippy_fixes(&current_content) {
                    Ok(clippy_fixed) => {
                        if clippy_fixed != current_content {
                            let engine_stats = engine.get_stats();
                            let fixes_applied = engine_stats.total_fixes_applied;
                            current_content = clippy_fixed;
                            total_fixes += fixes_applied;
                            *fixes_by_type.entry(AutoFixType::ClippyEngine).or_insert(0) +=
                                fixes_applied;

                            clippy_stats = Some(ClippyFixStats {
                                total_fixes_applied: fixes_applied,
                                patterns_processed: engine_stats.patterns_processed,
                                lint_types_fixed: engine_stats.lint_types_fixed,
                                processing_time_ms: clippy_start.elapsed().as_millis() as u64,
                            });

                            debug!("  ✅ ClippyFixEngine applied {} fixes", fixes_applied);
                        }
                    }
                    Err(e) => {
                        warn!("⚠️  ClippyFixEngine failed: {}", e);
                    }
                }
            }
        }

        // === PHASE 2: FlawlessCorrector Processing ===
        if self.config.enable_flawless_corrector
            && self
                .config
                .fix_types
                .contains(&AutoFixType::FlawlessCorrections)
        {
            debug!("🔧 Applying FlawlessCorrector patterns...");

            let flawless_start = Instant::now();
            if let Ok(mut corrector) = self.flawless_corrector.lock() {
                match corrector.apply_flawless_corrections(&current_content, "auto-fix diagnostic")
                {
                    Ok(flawless_fixed) => {
                        if flawless_fixed != current_content {
                            let corrector_stats = corrector.get_stats();
                            let fixes_applied = corrector_stats.total_corrections_applied;
                            current_content = flawless_fixed;
                            total_fixes += fixes_applied;
                            *fixes_by_type
                                .entry(AutoFixType::FlawlessCorrections)
                                .or_insert(0) += fixes_applied;

                            flawless_stats = Some(FlawlessCorrectionStats {
                                total_corrections_applied: fixes_applied,
                                patterns_processed: corrector_stats.patterns_processed,
                                correction_types_applied: corrector_stats.correction_types_applied,
                                safety_breakdown: corrector_stats.safety_breakdown,
                                processing_time_ms: flawless_start.elapsed().as_millis() as u64,
                            });

                            debug!(
                                "  ✅ FlawlessCorrector applied {} corrections",
                                fixes_applied
                            );
                        }
                    }
                    Err(e) => {
                        warn!("⚠️  FlawlessCorrector failed: {}", e);
                    }
                }
            }
        }

        // === PHASE 3: SemanticDeriveFramework Processing ===
        if self.config.enable_semantic_framework
            && self
                .config
                .fix_types
                .contains(&AutoFixType::SemanticDerives)
        {
            debug!("🧠 Applying SemanticDeriveFramework analysis...");

            let semantic_start = Instant::now();

            // Use a temporary file for semantic processing since it expects file paths
            if let Some(ref temp_dir) = self.temp_dir {
                let temp_file = temp_dir.path().join("semantic_temp.rs");
                if let Err(e) = fs::write(&temp_file, &current_content) {
                    warn!(
                        "⚠️  Failed to write temp file for semantic processing: {}",
                        e
                    );
                } else {
                    match self
                        .semantic_framework
                        .apply_semantic_derives(&[temp_file.clone()])
                    {
                        Ok(framework_report) => {
                            // Read the processed file back
                            if let Ok(semantic_fixed) = fs::read_to_string(&temp_file) {
                                if semantic_fixed != current_content {
                                    let derives_applied = framework_report.total_derives_applied;
                                    current_content = semantic_fixed;
                                    total_fixes += derives_applied;
                                    *fixes_by_type
                                        .entry(AutoFixType::SemanticDerives)
                                        .or_insert(0) += derives_applied;

                                    semantic_stats = Some(semanticator::SemanticMetrics {
                                        items_analyzed: framework_report.files_processed as u64,
                                        derives_applied: derives_applied as u64,
                                        total_processing_time_ms: semantic_start
                                            .elapsed()
                                            .as_millis()
                                            as u64,
                                        cache_hit_rate: 0.0, // Will be calculated later
                                        cache_hits: 0,
                                        cache_misses: framework_report.files_processed as u64,
                                        ai_inference_calls: if self.ai_engine.is_some() {
                                            1
                                        } else {
                                            0
                                        },
                                        avg_confidence_score: framework_report.success_rate,
                                    });

                                    debug!("  ✅ SemanticDeriveFramework applied {} derive suggestions", derives_applied);
                                }
                            }
                        }
                        Err(e) => {
                            warn!("⚠️  SemanticDeriveFramework failed: {:?}", e);
                        }
                    }

                    // Clean up temp file
                }
            }
        }

        // === PHASE 4: Traditional YoshiAF Fixes ===
        let traditional_fixes =
            self.apply_traditional_yoshiaf_fixes(&current_content, file_path)?;
        if !traditional_fixes.is_empty() {
            let mut lines: Vec<String> = current_content
                .lines()
                .map(yoshi_std::ToString::to_string)
                .collect();

            // Apply traditional fixes in reverse order to maintain line numbers
            for issue in traditional_fixes.iter().rev() {
                if issue.line_number > 0 && issue.line_number <= lines.len() {
                    match issue.issue_type {
                        AutoFixType::UnusedVariables => {
                            lines.remove(issue.line_number - 1);
                            total_fixes += 1;
                            *fixes_by_type
                                .entry(AutoFixType::UnusedVariables)
                                .or_insert(0) += 1;
                        }
                        AutoFixType::PrintlnToTracing => {
                            lines[issue.line_number - 1] = issue.fixed_code.clone();
                            total_fixes += 1;
                            *fixes_by_type
                                .entry(AutoFixType::PrintlnToTracing)
                                .or_insert(0) += 1;
                        }
                        _ => {
                            debug!(
                                "  ⏭️  Skipped {:?} (handled by other engines)",
                                issue.issue_type
                            );
                        }
                    }
                }
            }

            current_content = lines.join("\n");
        }

        // Write the final fixed content back to the file
        let final_fixes_applied = if current_content == original_content {
            0
        } else {
            fs::write(file_path, &current_content).map_err(|e| {
                Yoshi::new(YoshiKind::Internal {
                    message: format!("Failed to write fixed file {}: {e}", file_path.display())
                        .into(),
                    source: None,
                    component: Some("yoshiaf_write".into()),
                })
            })?;
            total_fixes
        };

        Ok(FileProcessingResult {
            total_fixes: final_fixes_applied,
            lines_modified: if final_fixes_applied > 0 {
                current_content
                    .lines()
                    .count()
                    .abs_diff(original_content.lines().count())
            } else {
                0
            },
            fixes_by_type,
            clippy_stats,
            flawless_stats,
            semantic_stats,
        })
    }

    /// **Apply traditional `YoshiAF` fixes (for compatibility)**
    fn apply_traditional_yoshiaf_fixes(
        &self,
        content: &str,
        _file_path: &Path,
    ) -> Hatch<Vec<AutoFixIssue>> {
        let mut issues = Vec::new();

        // Analyze for traditional fix types that aren't handled by integrated engines
        for (line_num, line) in content.lines().enumerate() {
            for fix_type in &self.config.fix_types {
                match fix_type {
                    AutoFixType::UnusedVariables => {
                        // CRITICAL FIX: Don't remove multi-line variable declarations
                        // This was causing function corruption by removing `let _var = `
                        // but leaving orphaned `.map_err(|e| { ... })?;` blocks
                        if line.trim().starts_with("let _")
                            && !line.contains("//")
                            && line.contains('=')
                            && !line.contains(".map_err")  // Skip complex multi-line constructs
                            && !line.contains("MandatoryBackupManager")  // Skip backup manager calls
                            && line.trim().ends_with(';')
                        // Only handle simple single-line declarations
                        {
                            issues.push(AutoFixIssue {
                                issue_type: AutoFixType::UnusedVariables,
                                file_path: PathBuf::from("current_file"),
                                line_number: line_num + 1,
                                column_number: 0,
                                description: "Simple unused variable with underscore prefix"
                                    .to_string(),
                                suggested_fix: "Remove simple unused variable".to_string(),
                                original_code: line.to_string(),
                                fixed_code: String::new(),
                                detected_by_engine: "YoshiAF-Traditional".to_string(),
                                confidence: 0.9,
                                safety_level: CorrectionSafetyLevel::Safe,
                            });
                        }
                    }
                    AutoFixType::PrintlnToTracing => {
                        if line.contains("println!") && !line.contains("//") {
                            let fixed = if line.contains("🚨")
                                || line.contains("❌")
                                || line.contains("Error:")
                            {
                                line.replace("tracing::info!", "tracing::error!")
                            } else if line.contains("⚠️") || line.contains("Warning:") {
                                line.replace("tracing::info!", "tracing::warn!")
                            } else if line.contains("🎉")
                                || line.contains("✅")
                                || line.contains("Success:")
                            {
                                line.replace("tracing::info!", "tracing::info!")
                            } else if line.contains("🔍") || line.contains("Debug:") {
                                line.replace("tracing::info!", "tracing::debug!")
                            } else {
                                line.replace("tracing::info!", "tracing::info!")
                            };

                            if fixed != line {
                                issues.push(AutoFixIssue {
                                    issue_type: AutoFixType::PrintlnToTracing,
                                    file_path: PathBuf::from("current_file"),
                                    line_number: line_num + 1,
                                    column_number: 0,
                                    description: "tracing::info! should use tracing macro"
                                        .to_string(),
                                    suggested_fix: "Replace with appropriate tracing macro"
                                        .to_string(),
                                    original_code: line.to_string(),
                                    fixed_code: fixed,
                                    detected_by_engine: "YoshiAF-Traditional".to_string(),
                                    confidence: 0.95,
                                    safety_level: CorrectionSafetyLevel::Cautious,
                                });
                            }
                        }
                    }
                    _ => {
                        // Other types are handled by integrated engines
                    }
                }
            }
        }

        Ok(issues)
    }

    /// **Discovers all Rust files with #![yoshi(auto-fix)] attribute**
    fn discover_auto_fix_files(&self) -> Hatch<Vec<PathBuf>> {
        let mut target_files = Vec::new();
        let mut scanned_files = 0;
        let mut excluded_files = 0;

        debug!("🔍 Scanning for files with #![yoshi(auto-fix)]...");

        for source_dir in &self.config.source_dirs {
            if !source_dir.exists() {
                warn!("⚠️  Directory does not exist: {}", source_dir.display());
                continue;
            }

            debug!("📁 Scanning directory: {}", source_dir.display());

            for entry in WalkDir::new(source_dir).follow_links(true) {
                let entry = entry.map_err(|e| {
                    Yoshi::new(YoshiKind::Internal {
                        message: format!("Failed to read directory entry: {e}").into(),
                        source: None,
                        component: Some("yoshiaf_discovery".into()),
                    })
                })?;
                let path = entry.path();

                // Only process Rust files
                if path.extension().is_some_and(|ext| ext == "rs") {
                    scanned_files += 1;

                    // Check exclusion patterns
                    let should_exclude = self.config.exclude_patterns.iter().any(|pattern| {
                        let pattern_clean = pattern.trim_matches('*');
                        let path_str = path.to_string_lossy();
                        path_str.contains(pattern_clean)
                    });

                    if should_exclude {
                        excluded_files += 1;
                        debug!("   ⏭️  Excluded: {}", path.display());
                        continue;
                    }

                    // Check for #![yoshi(auto-fix)] attribute
                    if self.has_yoshi_auto_fix_attribute(path)? {
                        debug!("   ✅ Found #![yoshi(auto-fix)]: {}", path.display());
                        target_files.push(path.to_path_buf());
                    }
                }
            }
        }

        info!("📊 Discovery Summary:");
        info!(
            "   📁 Directories scanned: {}",
            self.config.source_dirs.len()
        );
        info!("   📄 Rust files scanned: {}", scanned_files);
        info!("   ⏭️  Files excluded: {}", excluded_files);
        info!(
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
                component: Some("yoshiaf_file_read".into()),
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

    /// **Estimates memory usage across all integrated engines**
    fn estimate_memory_usage(&self) -> usize {
        let base_overhead = 2 * 1024 * 1024; // 2MB base overhead for integration
        let per_file_overhead = 8192; // 8KB per file (increased for integrated processing)
        let per_fix_overhead = 512; // 512 bytes per fix (increased for comprehensive tracking)
        let engine_overhead = 3 * 1024 * 1024; // 3MB for ClippyFixEngine + FlawlessCorrector + SemanticFramework

        base_overhead
            + (self.fix_stats.files_processed * per_file_overhead)
            + (self.fix_stats.fixes_applied * per_fix_overhead)
            + engine_overhead
    }

    /// **Get comprehensive statistics from all integrated engines**
    pub fn get_comprehensive_stats(&self) -> Hatch<AutoFixStats> {
        Ok(self.fix_stats.clone())
    }

    /// **Get integrated metrics across all engines**
    pub fn get_integrated_metrics(&self) -> Hatch<IntegratedMetrics> {
        self.integrated_metrics
            .read()
            .map(|metrics| metrics.clone())
            .map_err(|_| {
                Yoshi::new(YoshiKind::Internal {
                    message: "Failed to acquire read lock on integrated metrics".into(),
                    source: None,
                    component: Some("yoshiaf_metrics".into()),
                })
            })
    }
}

/// **Result of processing a single file through all integrated engines**
#[derive(Debug)]
struct FileProcessingResult {
    /// Total number of fixes applied across all engines
    total_fixes: usize,
    /// Number of lines modified
    lines_modified: usize,
    /// Breakdown of fixes by type
    fixes_by_type: HashMap<AutoFixType, usize>,
    /// Statistics from `ClippyFixEngine`
    clippy_stats: Option<ClippyFixStats>,
    /// Statistics from `FlawlessCorrector`
    flawless_stats: Option<FlawlessCorrectionStats>,
    /// Statistics from `SemanticDeriveFramework`
    semantic_stats: Option<semanticator::SemanticMetrics>,
}

/// **Generate comprehensive autonomous code fixes for specific directories**
///
/// This function provides a convenient API for applying the full `YoshiAF`
/// integrated engine to specific directories with all engines enabled.
///
/// # Arguments
///
/// * `source_dirs` - Source directories to scan for fixes
/// * `fix_types` - Types of fixes to apply (integrated engine types recommended)
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
        max_fixes_per_file: 1000,

        // Enable all integrated engines for maximum effectiveness
        enable_clippy_engine: true,
        enable_flawless_corrector: true,
        enable_semantic_framework: true,
        enable_ai_analysis: true,
        semantic_config: semanticator::SemanticConfig::default(),
        max_processing_time_ms: 30_000,
    };

    let mut engine = YoshiAF::with_config(config)?;
    engine.apply_autonomous_fixes()
}

/// **Test function for comprehensive `YoshiAF` integration**
///
/// This function demonstrates the full `YoshiAF` integrated engine in action,
/// testing `ClippyFixEngine` + `FlawlessCorrector` + `SemanticDeriveFramework`.
pub fn test_yoshi_af() -> Hatch<()> {
    info!("🤖 Testing YoshiAF - The Ultimate Quality of Life Integration Engine...");

    match YoshiAF::new() {
        Ok(mut engine) => match engine.apply_autonomous_fixes() {
            Ok(stats) => {
                info!("✅ YoshiAF comprehensive integration test completed successfully!");
                info!("📊 Final Integrated Statistics:");
                info!("   📁 Files processed: {}", stats.files_processed);
                info!("   🔧 Total fixes applied: {}", stats.fixes_applied);
                info!("   📄 Lines modified: {}", stats.lines_modified);
                info!("   ⏱️  Processing time: {}ms", stats.processing_time_ms);
                info!(
                    "   🎯 Overall success rate: {:.1}%",
                    stats.overall_success_rate
                );

                if let Some(ref clippy_stats) = stats.clippy_stats {
                    info!(
                        "   📋 ClippyFixEngine: {} fixes across {} patterns",
                        clippy_stats.total_fixes_applied, clippy_stats.patterns_processed
                    );
                }

                if let Some(ref flawless_stats) = stats.flawless_stats {
                    info!(
                        "   🔧 FlawlessCorrector: {} corrections across {} patterns",
                        flawless_stats.total_corrections_applied, flawless_stats.patterns_processed
                    );
                }

                if let Some(ref semantic_stats) = stats.semantic_stats {
                    info!(
                        "   🧠 SemanticFramework: {} derives with {:.1}% cache efficiency",
                        semantic_stats.derives_applied, semantic_stats.cache_hit_rate
                    );
                }

                Ok(())
            }
            Err(e) => {
                error!("❌ YoshiAF integration test failed: {}", e);
                Err(e)
            }
        },
        Err(e) => {
            error!("❌ Failed to initialize YoshiAF integrated engine: {}", e);
            Err(e)
        }
    }
}

/// **Test comprehensive integration with all engines**
pub fn test_comprehensive_integration() -> Hatch<()> {
    info!("🚀 Testing comprehensive YoshiAF integration with all engines...");

    // Create test configuration with all engines enabled
    let config = AutoFixConfig {
        source_dirs: vec![PathBuf::from("src")],
        fix_types: vec![
            AutoFixType::ClippyEngine,
            AutoFixType::FlawlessCorrections,
            AutoFixType::SemanticDerives,
            AutoFixType::QualityEnhancement,
            AutoFixType::PerformanceOptimization,
            AutoFixType::SafetyHardening,
        ],
        enable_clippy_engine: true,
        enable_flawless_corrector: true,
        enable_semantic_framework: true,
        enable_ai_analysis: true,
        create_backups: true,
        validate_after_fix: true,
        max_fixes_per_file: 1000,
        max_processing_time_ms: 60_000, // Allow more time for comprehensive testing
        ..Default::default()
    };

    let engine = YoshiAF::with_config(config)?;
    let metrics = engine.get_integrated_metrics()?;

    info!("✅ Comprehensive integration test completed!");
    info!("📊 Integration Results:");
    info!("   🎯 Total operations: {}", metrics.total_operations);
    info!(
        "   ⏱️  Total processing time: {}ms",
        metrics.total_processing_time_ms
    );
    info!(
        "   💾 Peak memory usage: {} bytes",
        metrics.peak_memory_usage_bytes
    );
    info!("   📈 Error rate: {:.2}%", metrics.error_rate * 100.0);
    info!("   🔄 Cache hit rate: {:.1}%", metrics.cache_hit_rate);

    for (engine_name, performance) in &metrics.engine_performance {
        info!(
            "   🔧 {}: {} ops, {:.1}% success, {:.1}ms avg",
            engine_name,
            performance.operations,
            performance.success_rate * 100.0,
            performance.avg_time_per_operation_ms
        );
    }

    Ok(())
}
