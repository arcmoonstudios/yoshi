/* yoshi-deluxe/src/diagnostics.rs */
//! **Brief:** Compiler diagnostic processor with robust JSON parsing for yoshi-deluxe.
//!
//! This module provides comprehensive diagnostic processing capabilities that parse cargo
//! check and clippy output, extract meaningful error information, and integrate with the
//! yoshi error framework for structured error handling and recovery strategies.

use crate::{
    constants::MAX_DIAGNOSTIC_BATCH_SIZE,
    err::{Hatch, Hatchling},
    types::{CompilerDiagnostic, DiagnosticLevel, DiagnosticSpan},
};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{Duration, SystemTime},
};
use tokio::sync::RwLock;
use yoshi_std::LayText;

//--------------------------------------------------------------------------------------------------
// Diagnostic Processor with Enhanced JSON Parsing
//--------------------------------------------------------------------------------------------------

/// Production-grade compiler diagnostic processor with robust parsing
pub struct CompilerDiagnosticProcessor {
    /// Cached parsed diagnostics with TTL
    diagnostic_cache: Arc<RwLock<HashMap<String, CachedDiagnostics>>>,
    /// Performance metrics
    metrics: ProcessingMetrics,
}

/// Cached diagnostics with expiration
#[derive(Debug, Clone)]
struct CachedDiagnostics {
    /// Parsed diagnostics
    diagnostics: Vec<CompilerDiagnostic>,
    /// Cache timestamp
    cached_at: SystemTime,
    /// Project modification time when cached
    project_modified: SystemTime,
}

impl CachedDiagnostics {
    /// Check if cache is still valid
    fn is_valid(&self, project_path: &Path) -> bool {
        if self.cached_at.elapsed().unwrap_or(Duration::MAX) > Duration::from_secs(300) {
            return false;
        }

        if let Ok(metadata) = fs::metadata(project_path) {
            if let Ok(modified) = metadata.modified() {
                return modified <= self.project_modified;
            }
        }
        true
    }

    /// Create new cached diagnostics
    fn new(diagnostics: Vec<CompilerDiagnostic>, project_path: &Path) -> Self {
        let project_modified = fs::metadata(project_path)
            .and_then(|m| m.modified())
            .unwrap_or_else(|_| SystemTime::now());

        Self {
            diagnostics,
            cached_at: SystemTime::now(),
            project_modified,
        }
    }
}

/// Performance tracking metrics
#[derive(Debug, Default)]
pub struct ProcessingMetrics {
    /// Total diagnostics processed
    pub total_processed: AtomicU64,
    /// Cache hit ratio
    pub cache_hits: AtomicU64,
    /// Parse errors encountered
    pub parse_errors: AtomicU64,
    /// Successful cargo operations
    pub successful_cargo_ops: AtomicU64,
    /// Failed cargo operations
    pub failed_cargo_ops: AtomicU64,
}

impl ProcessingMetrics {
    /// Record successful processing
    pub fn record_processed(&self, count: usize) {
        self.total_processed
            .fetch_add(count as u64, Ordering::Relaxed);
    }

    /// Record cache hit
    pub fn record_cache_hit(&self) {
        self.cache_hits.fetch_add(1, Ordering::Relaxed);
    }

    /// Record parse error
    pub fn record_parse_error(&self) {
        self.parse_errors.fetch_add(1, Ordering::Relaxed);
    }

    /// Record successful cargo operation
    pub fn record_successful_cargo_op(&self) {
        self.successful_cargo_ops.fetch_add(1, Ordering::Relaxed);
    }

    /// Record failed cargo operation
    pub fn record_failed_cargo_op(&self) {
        self.failed_cargo_ops.fetch_add(1, Ordering::Relaxed);
    }

    /// Get cache hit ratio
    #[must_use]
    pub fn cache_hit_ratio(&self) -> f64 {
        let hits = self.cache_hits.load(Ordering::Relaxed) as f64;
        let total = self.total_processed.load(Ordering::Relaxed) as f64;
        if total > 0.0 {
            hits / total
        } else {
            0.0
        }
    }

    /// Get success rate for cargo operations
    #[must_use]
    pub fn cargo_success_rate(&self) -> f64 {
        let success = self.successful_cargo_ops.load(Ordering::Relaxed) as f64;
        let total = success + self.failed_cargo_ops.load(Ordering::Relaxed) as f64;
        if total > 0.0 {
            success / total
        } else {
            0.0
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Diagnostic Processing Implementation
//--------------------------------------------------------------------------------------------------

impl CompilerDiagnosticProcessor {
    /// Creates a new diagnostic processor with optimized configuration
    #[must_use]
    pub fn new() -> Self {
        Self {
            diagnostic_cache: Arc::new(RwLock::new(HashMap::new())),
            metrics: ProcessingMetrics::default(),
        }
    }

    /// Executes cargo check with JSON output and parses diagnostics comprehensively
    ///
    /// # Errors
    ///
    /// Returns a yoshi error if cargo commands fail or diagnostics cannot be parsed
    pub async fn analyze_project(&self, project_path: &Path) -> Hatch<Vec<CompilerDiagnostic>> {
        let cache_key = project_path.to_string_lossy().to_string();

        if let Some(cached) = self.get_cached_diagnostics(&cache_key, project_path).await {
            self.metrics.record_cache_hit();
            return Ok(cached);
        }

        let check_diagnostics = self
            .run_cargo_check(project_path)
            .await
            .lay("Running cargo check command")?;

        let clippy_diagnostics = self
            .run_cargo_clippy(project_path)
            .await
            .lay("Running cargo clippy command")?;

        let mut all_diagnostics = check_diagnostics;
        all_diagnostics.extend(clippy_diagnostics);

        let diagnostics = self.deduplicate_and_prioritize_diagnostics(all_diagnostics);

        self.cache_diagnostics(cache_key, diagnostics.clone(), project_path)
            .await;
        self.metrics.record_processed(diagnostics.len());

        Ok(diagnostics)
    }

    /// Run cargo check with robust error handling
    async fn run_cargo_check(&self, project_path: &Path) -> Hatch<Vec<CompilerDiagnostic>> {
        let output = Command::new("cargo")
            .current_dir(project_path)
            .args([
                "check",
                "--message-format=json",
                "--all-targets",
                "--all-features",
                "--workspace",
                "--color=never",
            ])
            .output()
            .with_operation_context("cargo_check")
            .lay("Executing cargo check command")?;

        if output.status.success() {
            self.metrics.record_successful_cargo_op();
        } else {
            self.metrics.record_failed_cargo_op();
            // Don't fail on non-zero exit code, as compile errors are expected
        }

        self.parse_cargo_output(&output.stdout, "cargo-check")
            .await
            .lay("Parsing cargo check output")
    }

    /// Run cargo clippy with comprehensive lints and machine-applicable suggestions
    async fn run_cargo_clippy(&self, project_path: &Path) -> Hatch<Vec<CompilerDiagnostic>> {
        let output = Command::new("cargo")
            .current_dir(project_path)
            .args([
                "clippy",
                "--message-format=json",
                "--all-targets",
                "--all-features",
                "--workspace",
                "--color=never",
                "--",
                "-W",
                "clippy::all",
                "-W",
                "clippy::pedantic",
                "-W",
                "clippy::nursery",
                "-W",
                "clippy::cargo",
                "-W",
                "clippy::complexity",
                "-W",
                "clippy::correctness",
                "-W",
                "clippy::perf",
                "-W",
                "clippy::style",
                "-W",
                "clippy::suspicious",
                // Enable machine-applicable suggestions
                "-A",
                "clippy::manual_let_else", // Allow manual patterns for better suggestions
                "-A",
                "clippy::redundant_closure", // Allow for better closure analysis
            ])
            .output()
            .with_operation_context("cargo_clippy")
            .lay("Executing cargo clippy command")?;

        if output.status.success() {
            self.metrics.record_successful_cargo_op();
        } else {
            self.metrics.record_failed_cargo_op();
            // Don't fail on non-zero exit code, as lints are expected
        }

        let mut diagnostics = self
            .parse_cargo_output(&output.stdout, "cargo-clippy")
            .await
            .lay("Parsing cargo clippy output")?;

        // Enhance diagnostics with machine-applicable suggestions
        self.enhance_with_machine_applicable_suggestions(&mut diagnostics, project_path)
            .await
            .lay("Enhancing diagnostics with machine-applicable suggestions")?;

        Ok(diagnostics)
    }

    /// Extract machine-applicable suggestions from clippy output
    async fn enhance_with_machine_applicable_suggestions(
        &self,
        diagnostics: &mut Vec<CompilerDiagnostic>,
        project_path: &Path,
    ) -> Hatch<()> {
        // Run clippy with --fix to get machine-applicable suggestions
        let fix_output = Command::new("cargo")
            .current_dir(project_path)
            .args([
                "clippy",
                "--fix",
                "--allow-dirty",
                "--allow-staged",
                "--message-format=json",
                "--all-targets",
                "--all-features",
                "--workspace",
                "--color=never",
                "--",
                "-W",
                "clippy::all",
                "-W",
                "clippy::pedantic",
                "-W",
                "clippy::nursery",
            ])
            .output()
            .with_operation_context("cargo_clippy_fix")
            .lay("Executing cargo clippy --fix command")?;

        // Parse the fix suggestions and integrate them into diagnostics
        let fix_diagnostics = self
            .parse_cargo_output(&fix_output.stdout, "cargo-clippy-fix")
            .await
            .lay("Parsing cargo clippy --fix output")?;

        // Merge machine-applicable suggestions into original diagnostics
        self.merge_machine_applicable_suggestions(diagnostics, &fix_diagnostics)
            .lay("Merging machine-applicable suggestions")?;

        Ok(())
    }

    /// Merge machine-applicable suggestions from clippy --fix into diagnostics
    fn merge_machine_applicable_suggestions(
        &self,
        original_diagnostics: &mut Vec<CompilerDiagnostic>,
        fix_diagnostics: &[CompilerDiagnostic],
    ) -> Hatch<()> {
        for original in original_diagnostics.iter_mut() {
            // Find corresponding fix diagnostic
            if let Some(fix_diagnostic) = fix_diagnostics.iter().find(|fix| {
                fix.code == original.code
                    && fix
                        .message
                        .contains(&original.message[..std::cmp::min(50, original.message.len())])
            }) {
                // Extract machine-applicable signpost from the fix diagnostic
                if let Some(signpost) = self.extract_machine_applicable_signpost(fix_diagnostic) {
                    original.machine_applicable_signpost = Some(signpost);
                }
            }
        }
        Ok(())
    }

    /// Extract machine-applicable signpost from a clippy fix diagnostic
    fn extract_machine_applicable_signpost(
        &self,
        diagnostic: &CompilerDiagnostic,
    ) -> Option<String> {
        // Look for "suggestion" field in the diagnostic JSON
        // This would contain the exact code replacement that clippy suggests
        diagnostic
            .spans
            .iter()
            .find_map(|span| span.suggested_replacement.clone())
            .or_else(|| {
                // Fallback: extract from diagnostic message if it contains code suggestions
                if diagnostic.message.contains("try:") || diagnostic.message.contains("help:") {
                    Some(diagnostic.message.clone())
                } else {
                    None
                }
            })
    }

    /// Parse cargo JSON output with robust error handling
    async fn parse_cargo_output(
        &self,
        output: &[u8],
        source: &str,
    ) -> Hatch<Vec<CompilerDiagnostic>> {
        let output_str = String::from_utf8_lossy(output);
        let lines: Vec<&str> = output_str
            .lines()
            .filter(|line| !line.trim().is_empty() && line.starts_with('{'))
            .collect();

        let mut diagnostics = Vec::new();
        let mut parse_errors = 0;

        for line in lines {
            match self.parse_diagnostic_line(line, source) {
                Ok(Some(diag)) => diagnostics.push(diag),
                Ok(None) => {} // Not a diagnostic line
                Err(_) => {
                    parse_errors += 1;
                    self.metrics.record_parse_error();
                }
            }
        }

        if parse_errors > 0 {
            tracing::warn!(
                "Failed to parse {} diagnostic lines from {}",
                parse_errors,
                source
            );
        }

        Ok(diagnostics)
    }

    /// Parse individual diagnostic line with comprehensive error recovery
    fn parse_diagnostic_line(&self, line: &str, source: &str) -> Hatch<Option<CompilerDiagnostic>> {
        let json_value: serde_json::Value = serde_json::from_str(line)
            .with_operation_context("json_parsing")
            .lay("Parsing JSON diagnostic line")?;

        if json_value["reason"] != "compiler-message" {
            return Ok(None);
        }

        self.parse_diagnostic_json(&json_value["message"], source)
            .map(Some)
    }

    /// Convert JSON diagnostic to structured format with enhanced parsing
    fn parse_diagnostic_json(
        &self,
        json: &serde_json::Value,
        source: &str,
    ) -> Hatch<CompilerDiagnostic> {
        let message = json["message"].as_str().unwrap_or("").to_string();
        let code = json["code"]["code"].as_str().map(String::from);

        let level = match json["level"].as_str().unwrap_or("error") {
            "error" => DiagnosticLevel::Error,
            "warning" => DiagnosticLevel::Warning,
            "note" => DiagnosticLevel::Note,
            "help" => DiagnosticLevel::Help,
            _ => DiagnosticLevel::Error,
        };

        let spans = json["spans"]
            .as_array()
            .map(|spans| {
                spans
                    .iter()
                    .filter_map(|span| self.parse_span_json(span))
                    .collect()
            })
            .unwrap_or_default();

        let children: Vec<CompilerDiagnostic> = json["children"]
            .as_array()
            .map(|children| {
                children
                    .iter()
                    .filter_map(|child| self.parse_diagnostic_json(child, source).ok())
                    .collect()
            })
            .unwrap_or_default();

        let suggested_replacement = self.extract_suggested_replacement(&children);
        let id = format!("{source}::{}", message.chars().take(50).collect::<String>());

        let mut diagnostic = CompilerDiagnostic::new(id, message, level);
        diagnostic.code = code;
        diagnostic.spans = spans;
        diagnostic.children = children;
        diagnostic.suggested_replacement = suggested_replacement;
        diagnostic.add_metadata("source", source);

        Ok(diagnostic)
    }

    /// Parse span information with enhanced validation
    fn parse_span_json(&self, json: &serde_json::Value) -> Option<DiagnosticSpan> {
        let file_name = PathBuf::from(json["file_name"].as_str()?);
        let byte_start = json["byte_start"].as_u64()? as usize;
        let byte_end = json["byte_end"].as_u64()? as usize;
        let line_start = json["line_start"].as_u64()? as usize;
        let line_end = json["line_end"].as_u64()? as usize;
        let column_start = json["column_start"].as_u64()? as usize;
        let column_end = json["column_end"].as_u64()? as usize;

        // Validate span ranges
        if byte_start > byte_end
            || line_start > line_end
            || (line_start == line_end && column_start > column_end)
        {
            return None;
        }

        let text = json["text"]
            .as_array()?
            .first()?
            .get("text")?
            .as_str()?
            .to_string();

        let is_primary = json["is_primary"].as_bool().unwrap_or(false);
        let label = json["label"].as_str().map(String::from);

        let expansion = json["expansion"]
            .as_object()
            .and_then(|exp| self.parse_span_json(&serde_json::Value::Object(exp.clone())))
            .map(Box::new);

        let mut span = DiagnosticSpan::new(
            file_name,
            byte_start,
            byte_end,
            line_start,
            line_end,
            column_start,
            column_end,
            text,
        );

        if is_primary {
            span.mark_primary();
        }

        if let Some(label) = label {
            span.set_label(label);
        }

        span.expansion = expansion;

        Some(span)
    }

    /// Extract suggested replacement with enhanced heuristics
    fn extract_suggested_replacement(&self, children: &[CompilerDiagnostic]) -> Option<String> {
        children.iter().find_map(|child| {
            if matches!(child.level, DiagnosticLevel::Help) && !child.spans.is_empty() {
                child.spans.first().map(|span| span.text.clone())
            } else {
                None
            }
        })
    }

    /// Deduplicate and prioritize diagnostics
    fn deduplicate_and_prioritize_diagnostics(
        &self,
        mut diagnostics: Vec<CompilerDiagnostic>,
    ) -> Vec<CompilerDiagnostic> {
        // Sort by priority and location
        diagnostics.sort_by(|a, b| {
            b.level.priority().cmp(&a.level.priority()).then_with(|| {
                a.spans
                    .first()
                    .map_or(0, |s| s.byte_start)
                    .cmp(&b.spans.first().map_or(0, |s| s.byte_start))
            })
        });

        // Remove duplicates based on message, code, and location
        let mut seen = HashSet::new();
        diagnostics.retain(|diag| {
            let key = format!(
                "{}:{}:{}",
                diag.message,
                diag.code.as_deref().unwrap_or(""),
                diag.spans.first().map_or_else(String::new, |s| format!(
                    "{}:{}:{}",
                    s.file_name.display(),
                    s.line_start,
                    s.column_start
                ))
            );
            seen.insert(key)
        });

        // Limit to manageable batch size
        diagnostics.truncate(MAX_DIAGNOSTIC_BATCH_SIZE);
        diagnostics
    }

    /// Get cached diagnostics if valid
    async fn get_cached_diagnostics(
        &self,
        key: &str,
        project_path: &Path,
    ) -> Option<Vec<CompilerDiagnostic>> {
        let cache = self.diagnostic_cache.read().await;
        if let Some(cached) = cache.get(key) {
            if cached.is_valid(project_path) {
                return Some(cached.diagnostics.clone());
            }
        }
        None
    }

    /// Cache diagnostics with project state
    async fn cache_diagnostics(
        &self,
        key: String,
        diagnostics: Vec<CompilerDiagnostic>,
        project_path: &Path,
    ) {
        let mut cache = self.diagnostic_cache.write().await;
        cache.insert(key, CachedDiagnostics::new(diagnostics, project_path));
    }

    /// Get processing metrics
    #[must_use]
    pub fn metrics(&self) -> &ProcessingMetrics {
        &self.metrics
    }

    /// Clear diagnostic cache
    pub async fn clear_cache(&self) {
        let mut cache = self.diagnostic_cache.write().await;
        cache.clear();
    }

    /// Get cache statistics
    pub async fn cache_stats(&self) -> DiagnosticCacheStats {
        let cache = self.diagnostic_cache.read().await;
        DiagnosticCacheStats {
            cache_size: cache.len(),
            total_processed: self.metrics.total_processed.load(Ordering::Relaxed),
            cache_hit_ratio: self.metrics.cache_hit_ratio(),
            parse_errors: self.metrics.parse_errors.load(Ordering::Relaxed),
            cargo_success_rate: self.metrics.cargo_success_rate(),
        }
    }

    /// Run custom cargo command with JSON output
    pub async fn run_custom_cargo_command(
        &self,
        project_path: &Path,
        command: &str,
        args: &[&str],
    ) -> Hatch<Vec<CompilerDiagnostic>> {
        let mut cmd = Command::new("cargo");
        cmd.current_dir(project_path)
            .arg(command)
            .args(args)
            .args(["--message-format=json", "--color=never"]);

        let output = cmd
            .output()
            .with_operation_context(&format!("cargo_{command}"))
            .lay("Executing custom cargo command")?;

        if output.status.success() {
            self.metrics.record_successful_cargo_op();
        } else {
            self.metrics.record_failed_cargo_op();
        }

        self.parse_cargo_output(&output.stdout, &format!("cargo-{command}"))
            .await
    }

    /// Analyze specific file with targeted checking
    pub async fn analyze_file(
        &self,
        project_path: &Path,
        file_path: &Path,
    ) -> Hatch<Vec<CompilerDiagnostic>> {
        // Use cargo check with specific file focus
        let output = Command::new("cargo")
            .current_dir(project_path)
            .args([
                "check",
                "--message-format=json",
                "--color=never",
                "--bin",
                file_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("main"),
            ])
            .output()
            .with_file_context(file_path)
            .lay("Executing cargo check for specific file")?;

        if output.status.success() {
            self.metrics.record_successful_cargo_op();
        } else {
            self.metrics.record_failed_cargo_op();
        }

        let diagnostics = self
            .parse_cargo_output(&output.stdout, "cargo-check-file")
            .await?;

        // Filter diagnostics to only include the target file
        let filtered_diagnostics: Vec<_> = diagnostics
            .into_iter()
            .filter(|diag| diag.spans.iter().any(|span| span.file_name == file_path))
            .collect();

        Ok(filtered_diagnostics)
    }
}

impl Default for CompilerDiagnosticProcessor {
    fn default() -> Self {
        Self::new()
    }
}

//--------------------------------------------------------------------------------------------------
// Diagnostic Analysis and Filtering
//--------------------------------------------------------------------------------------------------

/// Advanced diagnostic analysis capabilities
pub struct DiagnosticAnalyzer;

impl DiagnosticAnalyzer {
    /// Analyze diagnostic patterns and categorize
    #[must_use]
    pub fn analyze_diagnostics(diagnostics: &[CompilerDiagnostic]) -> DiagnosticAnalysis {
        let mut error_count = 0;
        let mut warning_count = 0;
        let mut note_count = 0;
        let mut help_count = 0;
        let mut error_codes = HashMap::new();
        let mut file_distribution = HashMap::new();

        for diagnostic in diagnostics {
            match diagnostic.level {
                DiagnosticLevel::Error => error_count += 1,
                DiagnosticLevel::Warning => warning_count += 1,
                DiagnosticLevel::Note => note_count += 1,
                DiagnosticLevel::Help => help_count += 1,
            }

            if let Some(code) = &diagnostic.code {
                *error_codes.entry(code.clone()).or_insert(0) += 1;
            }

            for span in &diagnostic.spans {
                let file_key = span.file_name.display().to_string();
                *file_distribution.entry(file_key).or_insert(0) += 1;
            }
        }

        let most_common_errors: Vec<_> = {
            let mut codes: Vec<_> = error_codes.iter().collect();
            codes.sort_by(|a, b| b.1.cmp(a.1));
            codes
                .into_iter()
                .take(5)
                .map(|(k, v)| (k.clone(), *v))
                .collect()
        };

        let files_with_most_issues: Vec<_> = {
            let mut files: Vec<_> = file_distribution.iter().collect();
            files.sort_by(|a, b| b.1.cmp(a.1));
            files
                .into_iter()
                .take(5)
                .map(|(k, v)| (k.clone(), *v))
                .collect()
        };

        DiagnosticAnalysis {
            total_diagnostics: diagnostics.len(),
            error_count,
            warning_count,
            note_count,
            help_count,
            unique_error_codes: error_codes.len(),
            affected_files: file_distribution.len(),
            most_common_errors,
            files_with_most_issues,
            has_compilation_errors: error_count > 0,
            severity_distribution: vec![
                ("error".to_string(), error_count),
                ("warning".to_string(), warning_count),
                ("note".to_string(), note_count),
                ("help".to_string(), help_count),
            ],
        }
    }

    /// Filter diagnostics by criteria
    #[must_use]
    pub fn filter_diagnostics(
        diagnostics: &[CompilerDiagnostic],
        filter: &DiagnosticFilter,
    ) -> Vec<CompilerDiagnostic> {
        diagnostics
            .iter()
            .filter(|diag| {
                // Filter by level
                if let Some(ref levels) = filter.levels {
                    if !levels.contains(&diag.level) {
                        return false;
                    }
                }

                // Filter by error codes
                if let Some(ref codes) = filter.error_codes {
                    if let Some(ref diag_code) = diag.code {
                        if !codes.contains(diag_code) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                // Filter by file paths
                if let Some(ref files) = filter.file_paths {
                    let diag_files: Vec<_> = diag.spans.iter().map(|s| &s.file_name).collect();
                    if !files.iter().any(|f| diag_files.contains(&f)) {
                        return false;
                    }
                }

                // Filter by message content
                if let Some(ref message_pattern) = filter.message_contains {
                    if !diag.message.contains(message_pattern) {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect()
    }

    /// Group diagnostics by file
    #[must_use]
    pub fn group_by_file(
        diagnostics: &[CompilerDiagnostic],
    ) -> HashMap<PathBuf, Vec<CompilerDiagnostic>> {
        let mut groups = HashMap::new();

        for diagnostic in diagnostics {
            for span in &diagnostic.spans {
                groups
                    .entry(span.file_name.clone())
                    .or_insert_with(Vec::new)
                    .push(diagnostic.clone());
            }
        }

        groups
    }

    /// Group diagnostics by error code
    #[must_use]
    pub fn group_by_error_code(
        diagnostics: &[CompilerDiagnostic],
    ) -> HashMap<String, Vec<CompilerDiagnostic>> {
        let mut groups = HashMap::new();

        for diagnostic in diagnostics {
            let code = diagnostic
                .code
                .clone()
                .unwrap_or_else(|| "unknown".to_string());
            groups
                .entry(code)
                .or_insert_with(Vec::new)
                .push(diagnostic.clone());
        }

        groups
    }
}

/// Diagnostic analysis results
#[derive(Debug, Clone)]
pub struct DiagnosticAnalysis {
    /// Total number of diagnostics
    pub total_diagnostics: usize,
    /// Number of errors
    pub error_count: usize,
    /// Number of warnings
    pub warning_count: usize,
    /// Number of notes
    pub note_count: usize,
    /// Number of help messages
    pub help_count: usize,
    /// Number of unique error codes
    pub unique_error_codes: usize,
    /// Number of affected files
    pub affected_files: usize,
    /// Most common error codes
    pub most_common_errors: Vec<(String, usize)>,
    /// Files with most issues
    pub files_with_most_issues: Vec<(String, usize)>,
    /// Whether there are compilation errors
    pub has_compilation_errors: bool,
    /// Distribution by severity
    pub severity_distribution: Vec<(String, usize)>,
}

/// Filter criteria for diagnostics
#[derive(Debug, Clone, Default)]
pub struct DiagnosticFilter {
    /// Filter by diagnostic levels
    pub levels: Option<Vec<DiagnosticLevel>>,
    /// Filter by error codes
    pub error_codes: Option<Vec<String>>,
    /// Filter by file paths
    pub file_paths: Option<Vec<PathBuf>>,
    /// Filter by message content
    pub message_contains: Option<String>,
}

impl DiagnosticFilter {
    /// Create new filter
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter only errors
    #[must_use]
    pub fn errors_only() -> Self {
        Self {
            levels: Some(vec![DiagnosticLevel::Error]),
            ..Default::default()
        }
    }

    /// Filter only warnings
    #[must_use]
    pub fn warnings_only() -> Self {
        Self {
            levels: Some(vec![DiagnosticLevel::Warning]),
            ..Default::default()
        }
    }

    /// Filter by specific error codes
    #[must_use]
    pub fn by_error_codes(codes: Vec<String>) -> Self {
        Self {
            error_codes: Some(codes),
            ..Default::default()
        }
    }

    /// Filter by file path
    #[must_use]
    pub fn by_file(file_path: PathBuf) -> Self {
        Self {
            file_paths: Some(vec![file_path]),
            ..Default::default()
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct DiagnosticCacheStats {
    /// Current cache size
    pub cache_size: usize,
    /// Total diagnostics processed
    pub total_processed: u64,
    /// Cache hit ratio
    pub cache_hit_ratio: f64,
    /// Parse errors encountered
    pub parse_errors: u64,
    /// Cargo command success rate
    pub cargo_success_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::err::Hatchling;
    use tempfile::TempDir;
    use tokio::fs;

    async fn _create_test_project() -> Hatch<TempDir> {
        let temp_dir = tempfile::tempdir()
            .with_file_context(&std::env::temp_dir())
            .lay("Creating temporary test directory")?;

        let cargo_toml = r#"
[package]
name = "test-project"
version = "0.1.0"
edition = "2021"
"#;

        let main_rs = r#"
fn main() {
    let x = 5
    println!("Missing semicolon");
}
"#;

        fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml)
            .await
            .with_file_context(&temp_dir.path().join("Cargo.toml"))
            .lay("Writing Cargo.toml")?;

        let src_dir = temp_dir.path().join("src");
        fs::create_dir(&src_dir)
            .await
            .with_file_context(&src_dir)
            .lay("Creating src directory")?;

        fs::write(src_dir.join("main.rs"), main_rs)
            .await
            .with_file_context(&src_dir.join("main.rs"))
            .lay("Writing main.rs")?;

        Ok(temp_dir)
    }

    #[test]
    fn test_processor_creation() {
        let processor = CompilerDiagnosticProcessor::new();
        assert_eq!(processor.metrics().cache_hit_ratio(), 0.0);
    }

    #[test]
    fn test_diagnostic_creation() {
        let diagnostic = CompilerDiagnostic::new("test_id", "test message", DiagnosticLevel::Error);

        assert_eq!(diagnostic.id, "test_id");
        assert_eq!(diagnostic.message, "test message");
        assert!(diagnostic.is_error());
        assert!(!diagnostic.processed);
    }

    #[test]
    fn test_diagnostic_span_validation() {
        let processor = CompilerDiagnosticProcessor::new();

        // Valid span
        let valid_json = serde_json::json!({
            "file_name": "test.rs",
            "byte_start": 10,
            "byte_end": 20,
            "line_start": 1,
            "line_end": 1,
            "column_start": 10,
            "column_end": 20,
            "text": [{"text": "test_code"}],
            "is_primary": true
        });

        let span = processor.parse_span_json(&valid_json);
        assert!(span.is_some());

        // Invalid span (byte_start > byte_end)
        let invalid_json = serde_json::json!({
            "file_name": "test.rs",
            "byte_start": 20,
            "byte_end": 10,
            "line_start": 1,
            "line_end": 1,
            "column_start": 10,
            "column_end": 20,
            "text": [{"text": "test_code"}],
            "is_primary": false
        });

        let span = processor.parse_span_json(&invalid_json);
        assert!(span.is_none());
    }

    #[test]
    fn test_diagnostic_analysis() {
        let diagnostics = vec![
            CompilerDiagnostic::new("1", "error 1", DiagnosticLevel::Error),
            CompilerDiagnostic::new("2", "warning 1", DiagnosticLevel::Warning),
            CompilerDiagnostic::new("3", "error 2", DiagnosticLevel::Error),
        ];

        let analysis = DiagnosticAnalyzer::analyze_diagnostics(&diagnostics);

        assert_eq!(analysis.total_diagnostics, 3);
        assert_eq!(analysis.error_count, 2);
        assert_eq!(analysis.warning_count, 1);
        assert!(analysis.has_compilation_errors);
    }

    #[test]
    fn test_diagnostic_filtering() {
        let diagnostics = vec![
            CompilerDiagnostic::new("1", "error message", DiagnosticLevel::Error),
            CompilerDiagnostic::new("2", "warning message", DiagnosticLevel::Warning),
            CompilerDiagnostic::new("3", "another error", DiagnosticLevel::Error),
        ];

        let filter = DiagnosticFilter::errors_only();
        let filtered = DiagnosticAnalyzer::filter_diagnostics(&diagnostics, &filter);

        assert_eq!(filtered.len(), 2);
        assert!(filtered
            .iter()
            .all(|d| matches!(d.level, DiagnosticLevel::Error)));

        let message_filter = DiagnosticFilter {
            message_contains: Some("error".to_string()),
            ..Default::default()
        };
        let message_filtered =
            DiagnosticAnalyzer::filter_diagnostics(&diagnostics, &message_filter);
        assert_eq!(message_filtered.len(), 2);
    }

    #[tokio::test]
    async fn test_cache_operations() {
        let processor = CompilerDiagnosticProcessor::new();
        let diagnostics = vec![CompilerDiagnostic::new(
            "1",
            "test error",
            DiagnosticLevel::Error,
        )];

        // Test caching
        let temp_path = std::env::temp_dir();
        processor
            .cache_diagnostics("test_key".to_string(), diagnostics.clone(), &temp_path)
            .await;

        // Test cache retrieval
        let cached = processor
            .get_cached_diagnostics("test_key", &temp_path)
            .await;
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().len(), 1);

        // Test cache stats
        let stats = processor.cache_stats().await;
        assert_eq!(stats.cache_size, 1);
    }

    #[test]
    fn test_metrics_operations() {
        let metrics = ProcessingMetrics::default();

        metrics.record_processed(5);
        metrics.record_cache_hit();
        metrics.record_successful_cargo_op();
        metrics.record_failed_cargo_op();

        assert_eq!(metrics.total_processed.load(Ordering::Relaxed), 5);
        assert_eq!(metrics.cache_hits.load(Ordering::Relaxed), 1);
        assert_eq!(metrics.successful_cargo_ops.load(Ordering::Relaxed), 1);
        assert_eq!(metrics.failed_cargo_ops.load(Ordering::Relaxed), 1);
        assert_eq!(metrics.cargo_success_rate(), 0.5);
    }

    #[tokio::test]
    async fn test_json_parsing() {
        let processor = CompilerDiagnosticProcessor::new();

        let diagnostic_json = serde_json::json!({
            "message": "test error message",
            "code": {"code": "E0599"},
            "level": "error",
            "spans": [{
                "file_name": "test.rs",
                "byte_start": 10,
                "byte_end": 20,
                "line_start": 1,
                "line_end": 1,
                "column_start": 10,
                "column_end": 20,
                "text": [{"text": "error_code"}],
                "is_primary": true
            }],
            "children": []
        });

        let result = processor.parse_diagnostic_json(&diagnostic_json, "test_source");
        assert!(result.is_ok());

        let diagnostic = result.unwrap();
        assert_eq!(diagnostic.message, "test error message");
        assert_eq!(diagnostic.code, Some("E0599".to_string()));
        assert!(matches!(diagnostic.level, DiagnosticLevel::Error));
        assert_eq!(diagnostic.spans.len(), 1);
    }
}
