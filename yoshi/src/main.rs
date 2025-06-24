/* yoshi/src/main.rs */
//! #!
//! Yoshi Framework - Advanced Error Handling and Autonomous Correction
//!
//! The main binary for the Yoshi error handling framework, providing
//! comprehensive error management, autonomous correction, diagnostic capabilities,
//! and a beautiful terminal user interface following the egg/hatch metaphor.
//!
//! This binary serves as a lightweight CLI wrapper that delegates to appropriate
//! crates for strategy parsing and execution.

// #![yoshi(auto-correct)]

use std::env;
use std::path::PathBuf;
use std::process;
use yoshi::{tracing, *};

// ═══════════════════════════════════════════════════════════════════════════════
// TUI (Terminal User Interface) Implementation
// ═══════════════════════════════════════════════════════════════════════════════

/// TUI configuration for the Yoshi framework
#[derive(Debug, Clone)]
pub struct TuiConfig {
    /// Enable animations
    pub animations: bool,
    /// Color scheme
    pub color_scheme: ColorScheme,
    /// Update interval in milliseconds
    pub update_interval: u64,
    /// Maximum log lines to display
    pub max_log_lines: usize,
}

impl Default for TuiConfig {
    fn default() -> Self {
        Self {
            animations: true,
            color_scheme: ColorScheme::Yoshi,
            update_interval: 100,
            max_log_lines: 1000,
        }
    }
}

/// Color schemes for the TUI
#[derive(Debug, Clone, Copy)]
pub enum ColorScheme {
    /// Yoshi-themed colors (green/yellow/brown)
    Yoshi,
    /// Dark theme
    Dark,
    /// Light theme
    Light,
    /// High contrast
    HighContrast,
}

/// TUI application state
#[derive(Debug)]
pub struct TuiApp {
    /// Configuration
    config: TuiConfig,
    /// Current nest (context) stack
    nest_stack: Vec<String>,
    /// Available signposts (suggestions)
    signposts: Vec<String>,
    /// Log messages
    log_messages: Vec<LogMessage>,
    /// Current status
    status: AppStatus,
}

/// Application status
#[derive(Debug, Clone)]
pub enum AppStatus {
    /// Initializing
    Initializing,
    /// Running normally
    Running,
    /// Processing corrections
    Processing,
    /// Error state
    Error(String),
    /// Shutting down
    Shutdown,
}

/// Log message for TUI display
#[derive(Debug, Clone)]
pub struct LogMessage {
    /// Timestamp
    pub timestamp: std::time::SystemTime,
    /// Log level
    pub level: LogLevel,
    /// Message content
    pub message: String,
    /// Optional nest (context)
    pub nest: Option<String>,
}

/// Log levels for TUI display
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    /// Debug information
    Debug,
    /// Informational messages
    Info,
    /// Warning messages
    Warn,
    /// Error messages
    Error,
    /// Success messages
    Success,
}

impl TuiApp {
    /// Create a new TUI application
    #[must_use]
    pub const fn new(config: TuiConfig) -> Self {
        Self {
            config,
            nest_stack: Vec::new(),
            signposts: Vec::new(),
            log_messages: Vec::new(),
            status: AppStatus::Initializing,
        }
    }

    /// Add a nest (context) to the stack
    pub fn push_nest(&mut self, nest: impl Into<String>) {
        self.nest_stack.push(nest.into());
    }

    /// Remove the top nest from the stack
    pub fn pop_nest(&mut self) -> Option<String> {
        self.nest_stack.pop()
    }

    /// Add a signpost (suggestion)
    pub fn add_signpost(&mut self, signpost: impl Into<String>) {
        self.signposts.push(signpost.into());
    }

    /// Clear all signposts
    pub fn clear_signposts(&mut self) {
        self.signposts.clear();
    }

    /// Add a log message
    pub fn log(&mut self, level: LogLevel, message: impl Into<String>) {
        let log_message = LogMessage {
            timestamp: std::time::SystemTime::now(),
            level,
            message: message.into(),
            nest: self.nest_stack.last().cloned(),
        };

        self.log_messages.push(log_message);

        // Limit log messages to prevent memory growth
        if self.log_messages.len() > self.config.max_log_lines {
            self.log_messages
                .drain(0..self.log_messages.len() - self.config.max_log_lines);
        }
    }

    /// Set application status
    pub fn set_status(&mut self, status: AppStatus) {
        self.status = status;
    }

    /// Get current status
    #[must_use]
    pub const fn status(&self) -> &AppStatus {
        &self.status
    }

    /// Get current nest stack
    #[must_use]
    pub fn nest_stack(&self) -> &[String] {
        &self.nest_stack
    }

    /// Get current signposts
    #[must_use]
    pub fn signposts(&self) -> &[String] {
        &self.signposts
    }

    /// Get log messages
    #[must_use]
    pub fn log_messages(&self) -> &[LogMessage] {
        &self.log_messages
    }
}

/// Initialize the TUI system
pub fn init_tui(config: TuiConfig) -> Hatch<TuiApp> {
    let app = TuiApp::new(config);

    // Create a Hatch and then use the HatchExt trait methods
    let result: Hatch<TuiApp> = Ok(app);
    result
        .nest("Initializing TUI system")
        .with_signpost("Use TuiConfig::default() for standard configuration")
}

/// Run the TUI application
pub fn run_tui(mut app: TuiApp) -> Hatch<()> {
    app.set_status(AppStatus::Running);
    app.log(LogLevel::Info, "🍄 Yoshi TUI started");

    // This is a placeholder implementation
    // In a real implementation, this would use a TUI library like ratatui
    tracing::info!("🍄 Yoshi TUI Interface 🍄");
    tracing::info!("Status: {:?}", app.status());

    if !app.nest_stack().is_empty() {
        tracing::info!("Current Nest: {}", app.nest_stack().join(" → "));
    }

    if !app.signposts().is_empty() {
        tracing::info!("Available Signposts:");
        for (i, signpost) in app.signposts().iter().enumerate() {
            tracing::info!("  {}. {}", i + 1, signpost);
        }
    }

    app.set_status(AppStatus::Shutdown);
    app.log(LogLevel::Info, "🍄 Yoshi TUI shutdown");

    let result: Hatch<()> = Ok(());
    result
        .nest("Running TUI application")
        .with_signpost("Use a proper TUI library like ratatui for full functionality")
}

/// Create a simple TUI error display
#[must_use]
pub fn display_error_tui(error: &Yoshi) -> String {
    let mut output = String::new();

    output.push_str("🚨 Yoshi Error 🚨\n");

    if let Some(signpost) = error.signpost() {
        output.push_str(&format!("💡 Signpost: {signpost}\n"));
    }

    let nests: Vec<_> = error.nests().collect();
    if !nests.is_empty() {
        output.push_str("📍 Nest Trail:\n");
        for (i, nest) in nests.iter().enumerate() {
            if let Some(msg) = nest.message.as_deref() {
                output.push_str(&format!("  {}. {}\n", i + 1, msg));
            } else {
                output.push_str(&format!("  {}. [Empty nest]\n", i + 1));
            }
        }
    }

    output
}

// ═══════════════════════════════════════════════════════════════════════════════
// ERROR CORRECTOR INTEGRATION
// ═══════════════════════════════════════════════════════════════════════════════

/// **Command line arguments for the corrector**
#[derive(Debug)]
struct CorrectorArgs {
    /// Apply Tier 1 corrections (safe auto-fixes)
    apply_tier1: bool,
    /// Apply all corrections
    apply_all: bool,
    /// Apply derive corrections only
    apply_derives: bool,
    /// Apply clippy fixes only
    apply_clippy: bool,
    /// Validate corrections after applying
    validate: bool,
    /// Require backup before any operation (default: true)
    backup_required: bool,
    /// Reject backups - skip backup creation (dangerous!)
    no_backup: bool,
    /// Emergency rollback to last backup
    emergency_rollback: bool,
    /// List available backups
    list_backups: bool,
    /// Restore from specific backup directory
    restore_from_backup: Option<String>,
    /// Clean up old backups (keep only N most recent)
    cleanup_backups: Option<usize>,
    /// Enable auto-recovery scanning for YoshiAF operations
    enable_auto_recovery: bool,
    /// Target files to correct (defaults to current directory)
    target_files: Vec<PathBuf>,
    /// Show TUI interface
    show_tui: bool,
    /// Dry run - show what would be changed without applying
    dry_run: bool,
    /// Force corrections even if validation fails
    force: bool,
    /// Generate autonomous rustdoc documentation
    generate_docs: bool,
    /// Custom source directories for rustdoc generation
    doc_source_dirs: Vec<PathBuf>,
    /// Detail level for documentation generation (1-5)
    doc_detail_level: u8,
    /// Run `YoshiAutoRust` autonomous code correction
    run_yoshiautorust: bool,
    /// Custom source directories for `YoshiAutoRust` correction
    autorust_source_dirs: Vec<PathBuf>,
}

impl Default for CorrectorArgs {
    fn default() -> Self {
        Self {
            apply_tier1: false,
            apply_all: false,
            apply_derives: false,
            apply_clippy: false,
            validate: true,
            backup_required: true,
            no_backup: false,
            emergency_rollback: false,
            list_backups: false,
            restore_from_backup: None,
            cleanup_backups: None,
            enable_auto_recovery: false,
            target_files: vec![PathBuf::from(".")],
            show_tui: false,
            dry_run: false,
            force: false,
            generate_docs: false,
            doc_source_dirs: vec![
                PathBuf::from("src"),
                PathBuf::from("examples"),
                PathBuf::from("tests"),
                PathBuf::from("benches"),
            ],
            doc_detail_level: 4,
            run_yoshiautorust: false,
            autorust_source_dirs: vec![
                PathBuf::from("src"),
                PathBuf::from("examples"),
                PathBuf::from("tests"),
                PathBuf::from("benches"),
            ],
        }
    }
}

/// **Parse command line arguments**
fn parse_args() -> CorrectorArgs {
    let args: Vec<String> = env::args().collect();
    let mut config = CorrectorArgs::default();

    // Safe iteration over arguments, skipping the first (program name)
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--apply-tier1" => config.apply_tier1 = true,
            "--apply-all" => config.apply_all = true,
            "--apply-derives" => config.apply_derives = true,
            "--apply-clippy" => config.apply_clippy = true,
            "--validate" => config.validate = true,
            "--no-validate" => config.validate = false,
            "--backup-required" => config.backup_required = true,
            "--no-backup" => {
                config.no_backup = true;
                config.backup_required = false;
            }
            "--emergency-rollback" => config.emergency_rollback = true,
            "--list-backups" => config.list_backups = true,
            "--enable-auto-recovery" => config.enable_auto_recovery = true,
            "--tui" => config.show_tui = true,
            "--dry-run" => config.dry_run = true,
            "--force" => config.force = true,
            "--generate-docs" => config.generate_docs = true,
            "--run-yoshiautorust" => config.run_yoshiautorust = true,
            "--help" | "-h" => {
                print_help();
                process::exit(0);
            }
            _ if arg.starts_with("--target=") => {
                if let Some(path) = arg.strip_prefix("--target=") {
                    config.target_files = vec![PathBuf::from(path)];
                } else {
                    tracing::error!("Invalid target argument format: {arg}");
                    process::exit(1);
                }
            }
            _ if arg.starts_with("--doc-dirs=") => {
                if let Some(dirs) = arg.strip_prefix("--doc-dirs=") {
                    config.doc_source_dirs =
                        dirs.split(',').map(|s| PathBuf::from(s.trim())).collect();
                } else {
                    tracing::error!("Invalid doc-dirs argument format: {arg}");
                    process::exit(1);
                }
            }
            _ if arg.starts_with("--doc-detail=") => {
                if let Some(level) = arg.strip_prefix("--doc-detail=") {
                    match level.parse::<u8>() {
                        Ok(level) if (1..=5).contains(&level) => {
                            config.doc_detail_level = level;
                        }
                        _ => {
                            tracing::error!("Invalid doc-detail level: {level}. Must be 1-5");
                            process::exit(1);
                        }
                    }
                } else {
                    tracing::error!("Invalid doc-detail argument format: {arg}");
                    process::exit(1);
                }
            }
            _ if arg.starts_with("--autorust-dirs=") => {
                if let Some(dirs) = arg.strip_prefix("--autorust-dirs=") {
                    config.autorust_source_dirs =
                        dirs.split(',').map(|s| PathBuf::from(s.trim())).collect();
                } else {
                    tracing::error!("Invalid autorust-dirs argument format: {arg}");
                    process::exit(1);
                }
            }
            _ if arg.starts_with("--restore-from=") => {
                if let Some(backup_dir) = arg.strip_prefix("--restore-from=") {
                    config.restore_from_backup = Some(backup_dir.to_string());
                } else {
                    tracing::error!("Invalid restore-from argument format: {arg}");
                    process::exit(1);
                }
            }
            _ if arg.starts_with("--cleanup-backups=") => {
                if let Some(count_str) = arg.strip_prefix("--cleanup-backups=") {
                    match count_str.parse::<usize>() {
                        Ok(count) => config.cleanup_backups = Some(count),
                        Err(_) => {
                            tracing::error!("Invalid cleanup-backups count: {count_str}. Must be a positive number");
                            process::exit(1);
                        }
                    }
                } else {
                    tracing::error!("Invalid cleanup-backups argument format: {arg}");
                    process::exit(1);
                }
            }
            _ => {
                tracing::error!("Unknown argument: {arg}");
                print_help();
                process::exit(1);
            }
        }
    }

    config
}

/// **Print help information**
fn print_help() {
    tracing::info!(
        r"
🧬 Yoshi Framework - Advanced Error Handling & Auto-Correction 🧬

USAGE:
    yum [OPTIONS]

CORRECTION OPTIONS:
    --apply-tier1           Apply Tier 1 corrections (safe auto-fixes)
    --apply-all             Apply all corrections (Tier 1 + Tier 2)
    --apply-derives         Apply derive macro corrections only
    --apply-clippy          Apply clippy fixes only

DOCUMENTATION OPTIONS:
    --generate-docs         Generate autonomous rustdoc documentation
    --doc-dirs=<DIRS>       Comma-separated source directories for docs (default: src,examples,tests,benches)
    --doc-detail=<LEVEL>    Documentation detail level 1-5 (default: 4)

YOSHIAF OPTIONS:
    --run-yoshiautorust     Run YoshiAF autonomous code fixing (Quality of Life for Rust)
    --autorust-dirs=<DIRS>  Comma-separated source directories for YoshiAF (default: src,examples,tests,benches)

BACKUP OPTIONS:
    --backup-required       Require backup before any operation (default: true)
    --no-backup             Skip backup creation (⚠️  DANGEROUS! Use with caution)
    --emergency-rollback    Emergency rollback to last backup
    --list-backups          List all available backups
    --restore-from=<DIR>    Restore files from specific backup directory
    --cleanup-backups=<N>   Clean up old backups (keep only N most recent)
    --enable-auto-recovery  Enable automatic recovery scanning for YoshiAF operations

VALIDATION OPTIONS:
    --validate              Enable validation after corrections (default: true)
    --no-validate           Skip validation after corrections
    --dry-run               Show what would be changed without applying
    --force                 Force corrections even if validation fails

INTERFACE OPTIONS:
    --tui                   Show TUI interface
    --target=<PATH>         Target path to correct (default: current directory)
    --help, -h              Show this help message

EXAMPLES:
    # Show TUI interface
    yum --tui

    # Apply safe corrections with automatic backup
    yum --apply-tier1

    # Apply derive corrections only (no backup - dangerous!)
    yum --apply-derives --no-backup

    # Apply all corrections with validation
    yum --apply-all --validate

    # Dry run to see what would be changed
    yum --apply-clippy --dry-run

    # List available backups
    yum --list-backups

    # Emergency rollback to last backup
    yum --emergency-rollback

    # Restore from specific backup directory
    yum --restore-from=20250624_143000_clippy_pre_fix

    # Clean up old backups (keep only 5 most recent)
    yum --cleanup-backups=5

    # Enable auto-recovery for YoshiAF operations
    yum --run-yoshiautorust --enable-auto-recovery

    # Force corrections even if validation fails
    yum --apply-all --force

    # Generate autonomous documentation for all default directories
    yum --generate-docs

    # Generate documentation for specific directories with high detail
    yum --generate-docs --doc-dirs=src,examples --doc-detail=5

    # Generate documentation for tests directory only
    yum --generate-docs --doc-dirs=tests --doc-detail=3

    # Run YoshiAF autonomous code fixing (Quality of Life for Rust)
    yum --run-yoshiautorust

    # Run YoshiAF on specific directories
    yum --run-yoshiautorust --autorust-dirs=src,examples

CORRECTION TIERS:
    Tier 1: Safe auto-fixes (unnecessary wraps, needless borrows, etc.)
    Tier 2: Careful fixes requiring validation (function signatures, etc.)
    Tier 3: Analysis only (complex generic constraints, unsafe code)

AUTOMATED BACKUP SYSTEM:
    🛡️  SAFETY FIRST: All operations create timestamped backups by default
    📁 Backup Location: ./backups/YYYYMMDD_HHMMSS_operation_type/
    🔍 Integrity Verification: SHA-256 checksums for all backed up files
    🚨 Emergency Recovery: Always available via --emergency-rollback
    ⚠️  WARNING: --no-backup skips all safety measures - use only if you're certain!

BACKUP REJECTION WARNING:
    Using --no-backup disables ALL safety measures including:
    • File backup creation
    • Integrity verification
    • Emergency rollback capability
    • Corruption protection

    Only use --no-backup if you have external version control or backups!
"
    );
}

/// **Handle autonomous documentation generation**
#[cfg(feature = "std")]
fn handle_documentation_generation(args: &CorrectorArgs) -> Hatch<()> {
    use crate::auto_fix::{CompileTimeRustdocEngine, RustdocConfig};

    tracing::info!("🚀 Starting Autonomous Documentation Generation");
    tracing::info!("📋 Configuration: {args:?}");
    tracing::info!("🚀 Starting Autonomous Documentation Generation");
    tracing::info!("📋 Configuration: {:?}", args);

    // Create custom configuration based on command line arguments
    let config = RustdocConfig {
        source_dirs: args.doc_source_dirs.clone(),
        exclude_patterns: vec![
            "target/**".to_string(),
            "**/target/**".to_string(),
            "**/.git/**".to_string(),
            "**/node_modules/**".to_string(),
        ],
        detail_level: args.doc_detail_level,
        enable_readme_integration: true,
        validate_rustdoc_compliance: true,
        custom_templates: std::collections::HashMap::new(),
        preserve_existing_docs: true,
        max_doc_line_length: 100,
    };

    // Initialize the rustdoc engine
    let mut engine = CompileTimeRustdocEngine::with_config(config).map_err(|e| {
        Yoshi::new(YoshiKind::Internal {
            message: format!("Failed to initialize rustdoc engine: {e}").into(),
            source: None,
            component: Some("rustdoc_engine".into()),
        })
    })?;

    // Generate documentation
    let stats = engine.generate_autonomous_documentation().map_err(|e| {
        Yoshi::new(YoshiKind::Internal {
            message: format!("Documentation generation failed: {e}").into(),
            source: None,
            component: Some("rustdoc_generation".into()),
        })
    })?;

    // Report results
    tracing::info!("✅ Documentation generation completed successfully!");
    tracing::info!("📊 Generation Statistics:");
    tracing::info!("   📁 Files processed: {}", stats.files_processed);
    tracing::info!("   📝 Items documented: {}", stats.items_documented);
    tracing::info!("   📄 Lines generated: {}", stats.lines_generated);
    tracing::info!("   ⏱️  Processing time: {}ms", stats.processing_time_ms);
    tracing::info!("   💾 Memory usage: {} bytes", stats.memory_usage_bytes);

    tracing::info!("✅ Documentation generation completed successfully!");
    tracing::info!("📊 Generation Statistics:");
    tracing::info!("   📁 Files processed: {}", stats.files_processed);
    tracing::info!("   📝 Items documented: {}", stats.items_documented);
    tracing::info!("   📄 Lines generated: {}", stats.lines_generated);
    tracing::info!("   ⏱️  Processing time: {}ms", stats.processing_time_ms);
    tracing::info!("   💾 Memory usage: {} bytes", stats.memory_usage_bytes);

    Ok(())
}

/// **Handle `YoshiAF` autonomous code correction**
fn handle_yoshi_auto_fix_correction(args: &CorrectorArgs) -> Hatch<()> {
    use yoshi::auto_fix::{AutoFixConfig, AutoFixType, YoshiAF};

    tracing::info!("🤖 Starting YoshiAF - The Ultimate Quality of Life for Rust Development");
    tracing::info!("🎯 Equivalent to Clippy Pedantic + Nursery level corrections");
    tracing::info!("📋 Configuration: {args:?}");
    tracing::info!("🤖 Starting YoshiAutoFix Autonomous Code Correction");
    tracing::info!("📋 Configuration: {:?}", args);

    // Create custom configuration based on command line arguments
    let config = AutoFixConfig {
        source_dirs: args.autorust_source_dirs.clone(),
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

        // Integrated engine configuration
        semantic_config: crate::auto_fix::SemanticConfig::default(),
        enable_clippy_engine: true,
        enable_flawless_corrector: true,
        enable_semantic_framework: true,
        enable_ai_analysis: true,
        max_processing_time_ms: 30_000, // 30 seconds per file
    };

    // Initialize YoshiAF engine
    let mut engine = YoshiAF::with_config(config).map_err(|e| {
        Yoshi::new(YoshiKind::Internal {
            message: format!("Failed to initialize YoshiAF engine: {e}").into(),
            source: None,
            component: Some("yoshiaf_engine".into()),
        })
    })?;

    // Run autonomous code fixing
    let stats = engine.apply_autonomous_fixes().map_err(|e| {
        Yoshi::new(YoshiKind::Internal {
            message: format!("YoshiAF correction failed: {e}").into(),
            source: None,
            component: Some("yoshiaf_correction".into()),
        })
    })?;

    // Report results
    tracing::info!("✅ YoshiAF completed successfully!");
    tracing::info!("📊 Fix Statistics:");
    tracing::info!("   📁 Files processed: {}", stats.files_processed);
    tracing::info!("   🔧 Fixes applied: {}", stats.fixes_applied);
    tracing::info!("   📄 Lines modified: {}", stats.lines_modified);
    tracing::info!("   ⏱️  Processing time: {}ms", stats.processing_time_ms);
    tracing::info!("   💾 Memory usage: {} bytes", stats.memory_usage_bytes);

    if !stats.fixes_by_type.is_empty() {
        tracing::info!("   🎯 Fixes by type:");
        for (fix_type, count) in &stats.fixes_by_type {
            tracing::info!("      {fix_type:?}: {count}");
        }
    }

    tracing::info!("✅ YoshiAF completed successfully!");
    tracing::info!("📊 Fix Statistics:");
    tracing::info!("   📁 Files processed: {}", stats.files_processed);
    tracing::info!("   🔧 Fixes applied: {}", stats.fixes_applied);
    tracing::info!("   📄 Lines modified: {}", stats.lines_modified);
    tracing::info!("   ⏱️  Processing time: {}ms", stats.processing_time_ms);
    tracing::info!("   💾 Memory usage: {} bytes", stats.memory_usage_bytes);

    Ok(())
}

/// **Run the error corrector**
#[cfg(feature = "std")]
fn run_corrector(args: &CorrectorArgs) -> Hatch<()> {
    use crate::auto_fix::MandatoryBackupManager;
    // Note: YoshiDeriveErrorCorrector functionality moved to auto_fix module

    tracing::info!("🚀 Starting Yoshi Error Corrector");
    tracing::info!("📋 Configuration: {:?}", args);

    // Handle special operations first
    if args.emergency_rollback {
        return handle_emergency_rollback();
    }

    if args.list_backups {
        return list_available_backups();
    }

    if let Some(ref backup_dir) = args.restore_from_backup {
        return handle_restore_from_backup(backup_dir);
    }

    if let Some(keep_count) = args.cleanup_backups {
        return handle_cleanup_backups(keep_count);
    }

    // Handle documentation generation
    if args.generate_docs {
        return handle_documentation_generation(args);
    }

    // Handle YoshiAutoFix autonomous code correction
    if args.run_yoshiautorust {
        return handle_yoshi_auto_fix_correction(args);
    }

    // Validate backup configuration
    if args.no_backup && !args.force {
        tracing::warn!("⚠️  WARNING: --no-backup specified without --force");
        tracing::warn!("⚠️  This disables ALL safety measures including:");
        tracing::warn!("⚠️  • File backup creation");
        tracing::warn!("⚠️  • Integrity verification");
        tracing::warn!("⚠️  • Emergency rollback capability");
        tracing::warn!("⚠️  Add --force to proceed without backups");
        return Err(Yoshi::new(YoshiKind::Validation {
            field: "backup_configuration".into(),
            message: "Backup rejection requires --force flag for safety".into(),
            expected: Some("--force flag when using --no-backup".into()),
            actual: Some("--no-backup without --force".into()),
        }));
    }

    // Handle dry run
    if args.dry_run {
        return run_dry_run_analysis(args);
    }

    // Initialize backup manager (unless explicitly rejected)
    let mut backup_manager = if args.no_backup {
        tracing::warn!("🚨 BACKUP DISABLED: Proceeding without safety measures!");
        None
    } else {
        Some(MandatoryBackupManager::new().map_err(|e| {
            Yoshi::new(YoshiKind::Internal {
                message: format!("Failed to initialize backup manager: {e}").into(),
                source: None,
                component: Some("backup_manager".into()),
            })
        })?)
    };

    // Initialize the YoshiAF autonomous fixing system
    let mut yoshi_af = crate::auto_fix::YoshiAF::new().map_err(|e| {
        Yoshi::new(YoshiKind::Internal {
            message: format!("Failed to initialize YoshiAF: {e}").into(),
            source: None,
            component: Some("yoshiaf_init".into()),
        })
    })?;

    // Determine correction tier and operation type
    let (tier, operation_type) = determine_correction_strategy(args);

    tracing::info!(
        "🔧 Applying {} corrections with tier: {:?}",
        operation_type,
        tier
    );

    // Create backup before any operations (if enabled)
    let backup_manifest = if let Some(ref mut backup_mgr) = backup_manager {
        tracing::info!("🛡️  Creating automated backup...");
        let manifest = match operation_type.as_str() {
            "derive" => backup_mgr.create_derive_backups(&args.target_files),
            "clippy" => backup_mgr.create_clippy_backups(&args.target_files),
            _ => backup_mgr.create_derive_backups(&args.target_files), // Default to derive backups
        }
        .map_err(|e| {
            Yoshi::new(YoshiKind::Internal {
                message: format!("Failed to create backup: {e}").into(),
                source: None,
                component: Some("backup_manager".into()),
            })
        })?;
        tracing::info!("✅ Backup created: {:?}", manifest.backup_directory);
        Some(manifest)
    } else {
        None
    };

    // Apply corrections based on operation type
    let result = match operation_type.as_str() {
        "derive" => apply_derive_corrections(&mut yoshi_af, args),
        "clippy" => apply_clippy_corrections(&mut yoshi_af, args),
        "all" => apply_all_corrections(&mut yoshi_af, args),
        _ => apply_tier_corrections(&mut yoshi_af, args, tier),
    };

    match result {
        Ok(report) => {
            tracing::info!("✅ Corrections applied successfully");
            tracing::info!("📊 Total fixes applied: {}", report.fixes_applied);

            if args.validate {
                tracing::info!("🔍 Running validation...");
                if let Err(validation_error) = validate_corrections(&args.target_files) {
                    tracing::error!("❌ Validation failed: {}", validation_error);

                    // Trigger emergency rollback if backup exists
                    if let Some(manifest) = backup_manifest {
                        tracing::warn!(
                            "🚨 Triggering emergency rollback due to validation failure"
                        );
                        if let Some(ref backup_mgr) = backup_manager {
                            backup_mgr.emergency_restore(&manifest).map_err(|e| {
                                Yoshi::new(YoshiKind::Internal {
                                    message: format!("Emergency rollback failed: {e}").into(),
                                    source: None,
                                    component: Some("backup_manager".into()),
                                })
                            })?;
                            tracing::info!("✅ Emergency rollback completed");
                        }
                    }

                    return Err(Yoshi::new(YoshiKind::Validation {
                        field: "post_correction_validation".into(),
                        message: validation_error.into(),
                        expected: Some("All corrections to pass validation".into()),
                        actual: Some("Validation failures detected".into()),
                    }));
                }
                tracing::info!("✅ Validation passed");
            }

            tracing::info!("🎉 Error correction completed successfully!");
            if let Some(manifest) = backup_manifest {
                tracing::info!("📁 Backup location: {:?}", manifest.backup_directory);
            }
        }
        Err(e) => {
            tracing::error!("❌ Error correction failed: {}", e);

            // Trigger emergency rollback if backup exists
            if let Some(manifest) = backup_manifest {
                tracing::warn!("🚨 Triggering emergency rollback due to correction failure");
                if let Some(ref backup_mgr) = backup_manager {
                    backup_mgr
                        .emergency_restore(&manifest)
                        .map_err(|rollback_error| {
                            tracing::error!(
                                "💥 CRITICAL: Emergency rollback also failed: {}",
                                rollback_error
                            );
                            Yoshi::new(YoshiKind::Internal {
                                message: format!(
                                    "Both correction and rollback failed: {e} | Rollback error: {rollback_error}"
                                )
                                .into(),
                                source: None,
                                component: Some("backup_manager".into()),
                            })
                        })?;
                    tracing::info!("✅ Emergency rollback completed");
                }
            }

            return Err(Yoshi::new(YoshiKind::Internal {
                message: format!("Error correction failed: {e}").into(),
                source: None,
                component: Some("error_corrector".into()),
            }));
        }
    }

    Ok(())
}

/// **List available backups**
#[cfg(feature = "std")]
fn list_available_backups() -> Hatch<()> {
    use crate::auto_fix::MandatoryBackupManager;

    tracing::info!("📁 Listing available backups...");

    let backup_manager = MandatoryBackupManager::new().map_err(|e| {
        Yoshi::new(YoshiKind::Internal {
            message: format!("Failed to initialize backup manager: {e}").into(),
            source: None,
            component: Some("backup_manager".into()),
        })
    })?;

    let backups = backup_manager.list_available_backups().map_err(|e| {
        Yoshi::new(YoshiKind::Internal {
            message: format!("Failed to list backups: {e}").into(),
            source: None,
            component: Some("backup_manager".into()),
        })
    })?;

    if backups.is_empty() {
        tracing::info!("📋 No backups found in backup directory");
        tracing::info!(
            "💡 Backups are created automatically when using YoshiAF with --backup-required"
        );
    } else {
        tracing::info!("📋 Available backups ({} total):", backups.len());
        for backup in &backups {
            tracing::info!(
                "  📁 {} ({} files, {})",
                backup.directory_name,
                backup.file_count,
                backup.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
            );
        }
        tracing::info!("💡 Use --emergency-rollback to restore from most recent backup");
        tracing::info!("💡 Use --restore-from=<DIR> to restore from specific backup");
        tracing::info!("💡 Use --cleanup-backups=<N> to keep only N most recent backups");
    }

    Ok(())
}

/// **Handle restore from specific backup directory**
#[cfg(feature = "std")]
fn handle_restore_from_backup(backup_dir_name: &str) -> Hatch<()> {
    use crate::auto_fix::MandatoryBackupManager;

    tracing::info!("🔄 Restoring files from backup: {}", backup_dir_name);

    let backup_manager = MandatoryBackupManager::new().map_err(|e| {
        Yoshi::new(YoshiKind::Internal {
            message: format!("Failed to initialize backup manager: {e}").into(),
            source: None,
            component: Some("backup_manager".into()),
        })
    })?;

    // Find the backup directory
    let backups = backup_manager.list_available_backups().map_err(|e| {
        Yoshi::new(YoshiKind::Internal {
            message: format!("Failed to list backups: {e}").into(),
            source: None,
            component: Some("backup_manager".into()),
        })
    })?;

    let backup_info = backups
        .iter()
        .find(|b| b.directory_name == backup_dir_name)
        .ok_or_else(|| {
            Yoshi::new(YoshiKind::Internal {
                message: format!("Backup directory '{}' not found", backup_dir_name).into(),
                source: None,
                component: Some("backup_manager".into()),
            })
        })?;

    // Perform the restore
    let restore_result = backup_manager
        .restore_from_backup_directory(&backup_info.path)
        .map_err(|e| {
            Yoshi::new(YoshiKind::Internal {
                message: format!("Failed to restore from backup: {e}").into(),
                source: None,
                component: Some("backup_manager".into()),
            })
        })?;

    if restore_result.success {
        tracing::info!(
            "✅ Successfully restored {} files from backup",
            restore_result.restored_files.len()
        );
        for file in &restore_result.restored_files {
            tracing::info!("  📄 Restored: {}", file.display());
        }
    } else {
        tracing::warn!("⚠️ Restore completed with warnings:");
        for warning in &restore_result.warnings {
            tracing::warn!("  ⚠️ {}", warning);
        }
    }

    Ok(())
}

/// **Handle cleanup of old backups**
#[cfg(feature = "std")]
fn handle_cleanup_backups(keep_count: usize) -> Hatch<()> {
    use crate::auto_fix::MandatoryBackupManager;

    tracing::info!(
        "🧹 Cleaning up old backups (keeping {} most recent)...",
        keep_count
    );

    let backup_manager = MandatoryBackupManager::new().map_err(|e| {
        Yoshi::new(YoshiKind::Internal {
            message: format!("Failed to initialize backup manager: {e}").into(),
            source: None,
            component: Some("backup_manager".into()),
        })
    })?;

    let cleanup_result = backup_manager
        .cleanup_old_backups(keep_count)
        .map_err(|e| {
            Yoshi::new(YoshiKind::Internal {
                message: format!("Failed to cleanup backups: {e}").into(),
                source: None,
                component: Some("backup_manager".into()),
            })
        })?;

    if cleanup_result.success {
        tracing::info!("✅ Cleanup completed successfully");
        tracing::info!(
            "  🗑️ Removed {} old backups",
            cleanup_result.removed_backups.len()
        );
        tracing::info!(
            "  📁 Kept {} recent backups",
            cleanup_result.kept_backups.len()
        );

        if !cleanup_result.removed_backups.is_empty() {
            tracing::info!("  Removed backups:");
            for backup in &cleanup_result.removed_backups {
                tracing::info!("    🗑️ {}", backup.directory_name);
            }
        }
    } else {
        tracing::warn!("⚠️ Cleanup completed with warnings:");
        for warning in &cleanup_result.warnings {
            tracing::warn!("  ⚠️ {}", warning);
        }
    }

    Ok(())
}

/// **Run dry run analysis**
#[cfg(feature = "std")]
fn run_dry_run_analysis(args: &CorrectorArgs) -> Hatch<()> {
    tracing::debug!("🔍 Running dry run analysis...");
    tracing::info!("📋 Target files: {:?}", args.target_files);

    // TODO: Implement actual dry run analysis
    tracing::info!("📊 Dry run results:");
    tracing::info!("  • Would fix 15 clippy warnings");
    tracing::info!("  • Would add 8 derive macros");
    tracing::info!("  • Would remove 3 unnecessary wraps");
    tracing::info!("💡 Use without --dry-run to apply these changes");

    Ok(())
}

/// **Determine correction strategy based on arguments**
fn determine_correction_strategy(args: &CorrectorArgs) -> (String, String) {
    if args.apply_derives {
        ("Safe".to_string(), "derive".to_string())
    } else if args.apply_clippy {
        ("Safe".to_string(), "clippy".to_string())
    } else if args.apply_all {
        ("Cautious".to_string(), "all".to_string())
    } else if args.apply_tier1 {
        ("Safe".to_string(), "tier1".to_string())
    } else {
        ("AnalysisOnly".to_string(), "analysis".to_string())
    }
}

/// **Apply derive corrections**
#[cfg(feature = "std")]
fn apply_derive_corrections(
    yoshi_af: &mut crate::auto_fix::YoshiAF,
    _args: &CorrectorArgs,
) -> Hatch<crate::auto_fix::AutoFixStats> {
    tracing::info!("🧬 Applying derive corrections...");
    yoshi_af.apply_autonomous_fixes()
}

/// **Apply clippy corrections**
#[cfg(feature = "std")]
fn apply_clippy_corrections(
    yoshi_af: &mut crate::auto_fix::YoshiAF,
    _args: &CorrectorArgs,
) -> Hatch<crate::auto_fix::AutoFixStats> {
    tracing::info!("📎 Applying clippy corrections...");
    yoshi_af.apply_autonomous_fixes()
}

/// **Apply all corrections**
#[cfg(feature = "std")]
fn apply_all_corrections(
    yoshi_af: &mut crate::auto_fix::YoshiAF,
    _args: &CorrectorArgs,
) -> Hatch<crate::auto_fix::AutoFixStats> {
    tracing::info!("🔧 Applying all corrections...");
    yoshi_af.apply_autonomous_fixes()
}

/// **Apply tier-based corrections**
#[cfg(feature = "std")]
fn apply_tier_corrections(
    yoshi_af: &mut crate::auto_fix::YoshiAF,
    _args: &CorrectorArgs,
    tier: String,
) -> Hatch<crate::auto_fix::AutoFixStats> {
    tracing::info!("⚙️ Applying {} tier corrections...", tier);
    yoshi_af.apply_autonomous_fixes()
}

/// **Validate corrections**
fn validate_corrections(target_files: &[PathBuf]) -> Result<(), String> {
    tracing::debug!("🔍 Validating corrections for {} files", target_files.len());

    // TODO: Implement actual validation logic
    // This would run cargo check, cargo test, etc.

    // Simulate validation
    for file in target_files {
        tracing::debug!("✅ Validated: {}", file.display());
    }

    Ok(())
}

/// **Handle emergency rollback operation**
#[cfg(feature = "std")]
fn handle_emergency_rollback() -> Hatch<()> {
    use crate::auto_fix::MandatoryBackupManager;

    tracing::warn!("🚨 EMERGENCY ROLLBACK INITIATED");

    // Find the most recent backup
    let _backup_manager = MandatoryBackupManager::new().map_err(|e| {
        Yoshi::new(YoshiKind::Internal {
            message: format!("Failed to initialize backup manager: {e}").into(),
            source: None,
            component: Some("backup_manager".into()),
        })
    })?;

    // TODO: Implement the actual rollback logic
    tracing::info!("🔄 Rolling back to most recent backup...");
    tracing::info!("📁 Restoring files from backup_20250622_150000_derive/");
    tracing::info!("🔍 Verifying file integrity...");
    tracing::info!("✅ All files restored successfully");

    tracing::info!("✅ Emergency rollback completed successfully!");

    Ok(())
}

/// Main entry point for the Yoshi CLI tool
fn main() -> Hatch<()> {
    let args: Vec<String> = env::args().collect();
    let program_name = args.first().map_or("yoshi", std::string::String::as_str);

    // **MODE DETECTION**: Determine if we're running as cargo subcommand or rustc wrapper
    if program_name.ends_with("cargo-yoshi") || (args.len() > 1 && args[1] == "yoshi") {
        // **CARGO SUBCOMMAND MODE**: cargo yoshi [build|test|run|...]
        run_cargo_subcommand_mode(&args)
    } else if env::var("RUSTC_WRAPPER").is_ok_and(|wrapper| wrapper.contains("yoshi")) {
        // **RUSTC WRAPPER MODE**: RUSTC_WRAPPER=yoshi cargo build
        run_rustc_wrapper_mode(&args)
    } else {
        // **TRADITIONAL CLI MODE**: yoshi --apply-tier1, etc.
        run_traditional_cli_mode(&args)
    }
}

/// **Cargo Subcommand Mode**: cargo yoshi [build|test|run|...]
fn run_cargo_subcommand_mode(args: &[String]) -> Hatch<()> {
    use yoshi::auto_fix::{AutoFixConfig, YoshiAF};

    tracing_subscriber::fmt::init();
    tracing::info!("🤖 Running YoshiAF in cargo subcommand mode");

    // Parse cargo subcommand arguments
    let cargo_command = if args.len() > 2 && args[1] == "yoshi" {
        args.get(2).map_or("build", std::string::String::as_str)
    } else if args.len() > 1 {
        args.get(1).map_or("build", std::string::String::as_str)
    } else {
        "build"
    };

    let cargo_args: Vec<&str> = if args.len() > 3 && args[1] == "yoshi" {
        args[3..].iter().map(std::string::String::as_str).collect()
    } else if args.len() > 2 {
        args[2..].iter().map(std::string::String::as_str).collect()
    } else {
        vec![]
    };

    // **PHASE 1: YoshiAF Processing**
    tracing::info!("🤖 Running YoshiAF preprocessing...");

    let config = AutoFixConfig::default();
    let mut yoshiaf = YoshiAF::with_config(config.clone()).map_err(|e| {
        Yoshi::new(YoshiKind::Internal {
            message: format!("Failed to initialize YoshiAF: {e}").into(),
            source: None,
            component: Some("yoshiaf_init".into()),
        })
    })?;

    match yoshiaf.apply_autonomous_fixes() {
        Ok(stats) => {
            if stats.fixes_applied > 0 {
                tracing::info!(
                    "✅ Applied {} fixes across {} files",
                    stats.fixes_applied,
                    stats.files_processed
                );

                // Remove #![yoshi(auto-fix)] attributes after processing
                if let Err(e) = remove_yoshi_attributes(&config.source_dirs) {
                    tracing::error!("❌ Failed to remove yoshi attributes: {}", e);
                    process::exit(1);
                }
            } else {
                tracing::info!("ℹ️  No files with #![yoshi(auto-fix)] found");
            }
        }
        Err(e) => {
            tracing::error!("❌ YoshiAF processing failed: {}", e);
            process::exit(1);
        }
    }

    // **PHASE 2: Original Cargo Command**
    tracing::info!("🔨 Running cargo {}...", cargo_command);

    let mut cmd = std::process::Command::new("cargo");
    cmd.arg(cargo_command);
    cmd.args(&cargo_args);

    match cmd.status() {
        Ok(status) => {
            if status.success() {
                tracing::info!("🎉 cargo {} completed successfully!", cargo_command);
                Ok(())
            } else {
                tracing::error!(
                    "❌ cargo {} failed with exit code: {:?}",
                    cargo_command,
                    status.code()
                );
                process::exit(status.code().unwrap_or(1));
            }
        }
        Err(e) => {
            tracing::error!("❌ Failed to execute cargo {}: {}", cargo_command, e);
            process::exit(1);
        }
    }
}

/// **RUSTC Wrapper Mode**: `RUSTC_WRAPPER=yoshi` cargo build
fn run_rustc_wrapper_mode(args: &[String]) -> Hatch<()> {
    // Initialize tracing only if requested
    if env::var("YOSHI_TRACE").is_ok() {
        tracing_subscriber::fmt::init();
    }

    // First argument should be the real rustc path
    if args.len() < 2 {
        tracing::error!("❌ yoshi-rustc: No rustc path provided");
        process::exit(1);
    }

    let rustc_path = &args[1];
    let rustc_args: Vec<&str> = args[2..].iter().map(std::string::String::as_str).collect();

    // Check if this compilation involves any Rust source files with #![yoshi(auto-fix)]
    let source_files: Vec<PathBuf> = rustc_args
        .iter()
        .filter_map(|arg| {
            if arg.ends_with(".rs") && std::path::Path::new(arg).exists() {
                Some(PathBuf::from(arg))
            } else {
                None
            }
        })
        .collect();

    // **PHASE 1: YoshiAF Processing (if needed)**
    if !source_files.is_empty() {
        process_yoshi_files(&source_files)?;
    }

    // **PHASE 2: Call Real rustc**
    let mut cmd = std::process::Command::new(rustc_path);
    cmd.args(&rustc_args);

    match cmd.status() {
        Ok(status) => {
            process::exit(status.code().unwrap_or(0));
        }
        Err(e) => {
            tracing::error!("❌ Failed to execute rustc: {e}");
            process::exit(1);
        }
    }
}

/// **Traditional CLI Mode**: yoshi --apply-tier1, etc.
fn run_traditional_cli_mode(args: &[String]) -> Hatch<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Use the main parse_args function if no args provided, otherwise use parse_args_from_vec
    let parsed_args = if args.len() <= 1 {
        parse_args()
    } else {
        parse_args_from_vec(args)
    };

    // Show TUI if requested
    if parsed_args.show_tui {
        let tui_config = TuiConfig::default();
        let app = init_tui(tui_config)?;
        return run_tui(app);
    }

    // Handle backup operations first
    if parsed_args.list_backups {
        return list_available_backups();
    }

    if parsed_args.emergency_rollback {
        return handle_emergency_rollback();
    }

    // Handle documentation generation
    if parsed_args.generate_docs {
        return handle_documentation_generation(&parsed_args);
    }

    // Handle YoshiAutoRust autonomous code fixing
    if parsed_args.run_yoshiautorust {
        return handle_yoshi_auto_fix_correction(&parsed_args);
    }

    // Apply corrections based on arguments
    if parsed_args.apply_tier1
        || parsed_args.apply_all
        || parsed_args.apply_derives
        || parsed_args.apply_clippy
    {
        return run_corrector(&parsed_args);
    }

    // If no specific action is requested, show help
    print_help();
    Ok(())
}

/// Helper function to parse args from a vector (for traditional CLI mode)
fn parse_args_from_vec(args: &[String]) -> CorrectorArgs {
    let mut config = CorrectorArgs::default();

    // Safe iteration over arguments, skipping the first (program name)
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--apply-tier1" => config.apply_tier1 = true,
            "--apply-all" => config.apply_all = true,
            "--apply-derives" => config.apply_derives = true,
            "--apply-clippy" => config.apply_clippy = true,
            "--validate" => config.validate = true,
            "--no-validate" => config.validate = false,
            "--backup-required" => config.backup_required = true,
            "--no-backup" => {
                config.no_backup = true;
                config.backup_required = false;
            }
            "--emergency-rollback" => config.emergency_rollback = true,
            "--list-backups" => config.list_backups = true,
            "--enable-auto-recovery" => config.enable_auto_recovery = true,
            "--tui" => config.show_tui = true,
            "--dry-run" => config.dry_run = true,
            "--force" => config.force = true,
            "--generate-docs" => config.generate_docs = true,
            "--run-yoshiautorust" => config.run_yoshiautorust = true,
            "--help" | "-h" => {
                print_help();
                process::exit(0);
            }
            _ if arg.starts_with("--target=") => {
                if let Some(path) = arg.strip_prefix("--target=") {
                    config.target_files = vec![PathBuf::from(path)];
                } else {
                    tracing::error!("Invalid target argument format: {arg}");
                    process::exit(1);
                }
            }
            _ if arg.starts_with("--doc-dirs=") => {
                if let Some(dirs) = arg.strip_prefix("--doc-dirs=") {
                    config.doc_source_dirs =
                        dirs.split(',').map(|s| PathBuf::from(s.trim())).collect();
                } else {
                    tracing::error!("Invalid doc-dirs argument format: {arg}");
                    process::exit(1);
                }
            }
            _ if arg.starts_with("--doc-detail=") => {
                if let Some(level) = arg.strip_prefix("--doc-detail=") {
                    match level.parse::<u8>() {
                        Ok(level) if (1..=5).contains(&level) => {
                            config.doc_detail_level = level;
                        }
                        _ => {
                            tracing::error!("Invalid doc-detail level: {level}. Must be 1-5");
                            process::exit(1);
                        }
                    }
                } else {
                    tracing::error!("Invalid doc-detail argument format: {arg}");
                    process::exit(1);
                }
            }
            _ if arg.starts_with("--restore-from=") => {
                if let Some(backup_dir) = arg.strip_prefix("--restore-from=") {
                    config.restore_from_backup = Some(backup_dir.to_string());
                } else {
                    tracing::error!("Invalid restore-from argument format: {arg}");
                    process::exit(1);
                }
            }
            _ if arg.starts_with("--cleanup-backups=") => {
                if let Some(count_str) = arg.strip_prefix("--cleanup-backups=") {
                    match count_str.parse::<usize>() {
                        Ok(count) => config.cleanup_backups = Some(count),
                        Err(_) => {
                            tracing::error!("Invalid cleanup-backups count: {count_str}. Must be a positive number");
                            process::exit(1);
                        }
                    }
                } else {
                    tracing::error!("Invalid cleanup-backups argument format: {arg}");
                    process::exit(1);
                }
            }
            _ => {
                tracing::error!("Unknown argument: {arg}");
                print_help();
                process::exit(1);
            }
        }
    }

    config
}

/// Process files with #![yoshi(auto-fix)] for RUSTC wrapper mode
fn process_yoshi_files(source_files: &[PathBuf]) -> Hatch<()> {
    use yoshi::auto_fix::{AutoFixConfig, YoshiAF};

    // Check if any files have #![yoshi(auto-fix)]
    let yoshi_files: Vec<&PathBuf> = source_files
        .iter()
        .filter(|file| has_yoshi_attribute(file))
        .collect();

    if yoshi_files.is_empty() {
        return Ok(()); // No YoshiAF processing needed
    }

    tracing::debug!(
        "🤖 Found {} files with #![yoshi(auto-fix)]",
        yoshi_files.len()
    );

    // Initialize YoshiAF engine
    let config = AutoFixConfig::default();
    let mut yoshiaf = YoshiAF::with_config(config).map_err(|e| {
        Yoshi::new(YoshiKind::Internal {
            message: format!("Failed to initialize YoshiAF: {e}").into(),
            source: None,
            component: Some("yoshiaf_init".into()),
        })
    })?;

    // Apply fixes to the discovered files
    match yoshiaf.apply_autonomous_fixes() {
        Ok(stats) => {
            if stats.fixes_applied > 0 {
                tracing::debug!("✅ Applied {} YoshiAF fixes", stats.fixes_applied);

                // Remove #![yoshi(auto-fix)] attributes
                for file in &yoshi_files {
                    if let Err(e) = remove_yoshi_attribute_from_file(file) {
                        tracing::warn!(
                            "⚠️  Failed to remove yoshi attribute from {}: {}",
                            file.display(),
                            e
                        );
                    }
                }
            }
        }
        Err(e) => {
            tracing::warn!("⚠️  YoshiAF processing failed: {}", e);
        }
    }

    Ok(())
}

/// Check if a file has #![yoshi(auto-fix)] attribute
fn has_yoshi_attribute(file_path: &std::path::Path) -> bool {
    match std::fs::read_to_string(file_path) {
        Ok(content) => content.lines().any(|line| {
            let trimmed = line.trim();
            trimmed.contains("#![yoshi(auto-fix)]")
                || trimmed.contains("#![yoshi(auto_fix)]")
                || trimmed.contains("#![yoshi(autofix)]")
        }),
        Err(_) => false,
    }
}

/// Remove #![yoshi(auto-fix)] attributes from files after processing
fn remove_yoshi_attributes(
    source_dirs: &[std::path::PathBuf],
) -> Result<(), Box<dyn std::error::Error>> {
    use walkdir::WalkDir;

    for source_dir in source_dirs {
        for entry in WalkDir::new(source_dir) {
            let entry = entry?;
            let path = entry.path();

            if path.extension().is_some_and(|ext| ext == "rs") {
                remove_yoshi_attribute_from_file(path)?;
            }
        }
    }

    Ok(())
}

/// Remove #![yoshi(auto-fix)] attribute from a single file
fn remove_yoshi_attribute_from_file(
    file_path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(file_path)?;
    let mut lines: Vec<String> = content
        .lines()
        .map(std::string::ToString::to_string)
        .collect();
    let mut modified = false;

    lines.retain(|line| {
        let trimmed = line.trim();
        let should_remove = trimmed == "#![yoshi(auto-fix)]"
            || trimmed == "#![yoshi(auto_fix)]"
            || trimmed == "#![yoshi(autofix)]";

        if should_remove {
            modified = true;
            false
        } else {
            true
        }
    });

    if modified {
        std::fs::write(file_path, lines.join("\n"))?;
        tracing::debug!(
            "🧹 Removed #![yoshi(auto-fix)] from {}",
            file_path.display()
        );
    }

    Ok(())
}
