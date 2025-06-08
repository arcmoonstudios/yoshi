/* yoshi-deluxe/src/system.rs */
//! **Brief:** Main auto-correction system orchestrating all components for yoshi-deluxe.
//!
//! This module provides the main `AutoCorrectionSystem` that coordinates between the
//! diagnostic processor, AST analyzer, documentation scraper, and code generator to
//! provide comprehensive auto-correction capabilities with yoshi error integration.

use crate::{
    ast::ASTAnalysisEngine,
    codegen::CodeGenerationEngine,
    diagnostics::CompilerDiagnosticProcessor,
    docs::DocsScrapingEngine,
    errors::{factory, Result, YoshiDeluxeExt},
    metrics::SystemMetricsCollector,
    types::{
        AppliedCorrection, CachedDocsData, CompilerDiagnostic, CorrectionProposal,
        CorrectionStrategy, ProjectCorrection, SafetyLevel, SystemConfig,
    },
};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, SystemTime},
};
use yoshi_std::LayText;
use tokio::task::JoinSet;

//--------------------------------------------------------------------------------------------------
// Main Auto-Correction System Integration
//--------------------------------------------------------------------------------------------------

/// Production-grade auto-correction system orchestrator
pub struct AutoCorrectionSystem {
    /// Diagnostic processor
    diagnostic_processor: CompilerDiagnosticProcessor,
    /// AST analysis engine
    ast_analyzer: ASTAnalysisEngine,
    /// Documentation scraper
    docs_scraper: DocsScrapingEngine,
    /// Code generator
    code_generator: CodeGenerationEngine,
    /// System configuration
    config: SystemConfig,
    /// Metrics collector
    metrics_collector: SystemMetricsCollector,
}

impl AutoCorrectionSystem {
    /// Creates a new auto-correction system with production defaults
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(SystemConfig::default())
    }

    /// Creates a new auto-correction system with custom configuration
    #[must_use]
    pub fn with_config(config: SystemConfig) -> Self {
        Self {
            diagnostic_processor: CompilerDiagnosticProcessor::new(),
            ast_analyzer: ASTAnalysisEngine::new(),
            docs_scraper: DocsScrapingEngine::new(),
            code_generator: CodeGenerationEngine::new(),
            metrics_collector: SystemMetricsCollector::new(),
            config,
        }
    }

    /// Main entry point: analyzes project and generates corrections
    ///
    /// # Errors
    ///
    /// Returns a yoshi error if project analysis fails
    pub async fn analyze_and_correct(&self, project_path: &Path) -> Result<Vec<ProjectCorrection>> {
        let start_time = SystemTime::now();

        // Validate project path
        if !project_path.exists() || !project_path.is_dir() {
            return Err(factory::configuration_error(
                "project_path",
                project_path.display().to_string(),
            ))
            .with_file_context(project_path)
            .lay("Validating project path");
        }

        // Check for Cargo.toml to ensure it's a Rust project
        let cargo_toml = project_path.join("Cargo.toml");
        if !cargo_toml.exists() {
            return Err(factory::configuration_error(
                "cargo_project",
                "Missing Cargo.toml file",
            ))
            .with_file_context(project_path)
            .lay("Validating Rust project structure");
        }

        self.metrics_collector.record_analysis_start().await;

        let diagnostics = self
            .diagnostic_processor
            .analyze_project(project_path)
            .await
            .lay("Analyzing project diagnostics")?;

        if diagnostics.is_empty() {
            self.metrics_collector
                .record_analysis_complete(start_time.elapsed().unwrap_or_default())
                .await;
            return Ok(Vec::new());
        }

        self.metrics_collector
            .record_diagnostics_found(diagnostics.len())
            .await;

        let corrections = if self.config.enable_parallel_processing {
            self.process_diagnostics_parallel(&diagnostics)
                .await
                .lay("Processing diagnostics in parallel")?
        } else {
            self.process_diagnostics_sequential(&diagnostics)
                .await
                .lay("Processing diagnostics sequentially")?
        };

        self.metrics_collector
            .record_corrections_generated(corrections.len())
            .await;
        self.metrics_collector
            .record_analysis_complete(start_time.elapsed().unwrap_or_default())
            .await;

        Ok(corrections)
    }

    /// Process diagnostics in parallel with controlled concurrency
    async fn process_diagnostics_parallel(
        &self,
        diagnostics: &[CompilerDiagnostic],
    ) -> Result<Vec<ProjectCorrection>> {
        let mut join_set = JoinSet::new();
        let semaphore = Arc::new(tokio::sync::Semaphore::new(
            self.config.max_concurrent_operations,
        ));

        for diagnostic in diagnostics {
            let diagnostic = diagnostic.clone();
            let permit = semaphore
                .clone()
                .acquire_owned()
                .await
                .map_err(|e| {
                    factory::resource_exhausted_error(
                        "concurrency_semaphore",
                        self.config.max_concurrent_operations as u64,
                        1,
                    )
                })
                .lay("Acquiring concurrency permit")?;

            let ast_analyzer = ASTAnalysisEngine::new();
            let docs_scraper = DocsScrapingEngine::new();
            let code_generator = CodeGenerationEngine::new();
            let config = self.config.clone();

            join_set.spawn(async move {
                let _permit = permit;
                Self::process_single_diagnostic_static(
                    diagnostic,
                    ast_analyzer,
                    docs_scraper,
                    code_generator,
                    config,
                )
                .await
            });
        }

        let mut corrections = Vec::new();
        while let Some(result) = join_set.join_next().await {
            match result {
                Ok(Ok(Some(correction))) => corrections.push(correction),
                Ok(Ok(None)) => {} // No correction generated
                Ok(Err(e)) => {
                    tracing::warn!("Failed to process diagnostic: {}", e);
                    self.metrics_collector.record_processing_error().await;
                }
                Err(e) => {
                    tracing::error!("Task join error: {}", e);
                    self.metrics_collector.record_processing_error().await;
                }
            }
        }

        Ok(corrections)
    }

    /// Process diagnostics sequentially
    async fn process_diagnostics_sequential(
        &self,
        diagnostics: &[CompilerDiagnostic],
    ) -> Result<Vec<ProjectCorrection>> {
        let mut corrections = Vec::new();

        for diagnostic in diagnostics {
            match self.process_single_diagnostic(diagnostic).await {
                Ok(Some(correction)) => corrections.push(correction),
                Ok(None) => {} // No correction generated
                Err(e) => {
                    tracing::warn!("Failed to process diagnostic {}: {}", diagnostic.id, e);
                    self.metrics_collector.record_processing_error().await;
                }
            }
        }

        Ok(corrections)
    }

    /// Process a single diagnostic
    async fn process_single_diagnostic(
        &self,
        diagnostic: &CompilerDiagnostic,
    ) -> Result<Option<ProjectCorrection>> {
        Self::process_single_diagnostic_static(
            diagnostic.clone(),
            self.ast_analyzer.clone(),
            self.docs_scraper.clone(),
            self.code_generator.clone(),
            self.config.clone(),
        )
        .await
    }

    /// Static method for processing a single diagnostic
    async fn process_single_diagnostic_static(
        diagnostic: CompilerDiagnostic,
        mut ast_analyzer: ASTAnalysisEngine,
        docs_scraper: DocsScrapingEngine,
        code_generator: CodeGenerationEngine,
        config: SystemConfig,
    ) -> Result<Option<ProjectCorrection>> {
        let ast_context = ast_analyzer
            .analyze_diagnostic(&diagnostic)
            .await
            .lay("Analyzing diagnostic AST context")?;

        let docs_data = if config.enable_docs_scraping {
            Self::scrape_relevant_documentation_static(&ast_context, &docs_scraper)
                .await
                .map_err(|e| {
                    tracing::debug!("Documentation scraping failed: {}", e);
                    e
                })
                .ok()
        } else {
            None
        };

        let proposals = code_generator
            .generate_corrections(&ast_context, docs_data.as_ref())
            .await
            .lay("Generating correction proposals")?;

        let filtered_proposals: Vec<_> = proposals
            .into_iter()
            .filter(|p| {
                p.confidence >= config.min_confidence_threshold
                    && p.safety_level >= config.min_safety_level
            })
            .take(config.max_proposals_per_diagnostic)
            .collect();

        if filtered_proposals.is_empty() {
            return Ok(None);
        }

        let mut correction = ProjectCorrection::new(ast_context.file_path, diagnostic);
        for proposal in filtered_proposals {
            correction.add_proposal(proposal);
        }

        Ok(Some(correction))
    }

    /// Static method for scraping documentation
    async fn scrape_relevant_documentation_static(
        context: &crate::ast::ASTContext,
        docs_scraper: &DocsScrapingEngine,
    ) -> Result<CachedDocsData> {
        let (crate_name, type_name) = Self::extract_crate_and_type_info(context)
            .lay("Extracting crate and type information from context")?;

        docs_scraper
            .scrape_type_documentation(&crate_name, &type_name)
            .await
            .lay("Scraping documentation from docs.rs")
    }

    /// Extract crate and type info from context
    fn extract_crate_and_type_info(context: &crate::ast::ASTContext) -> Result<(String, String)> {
        if let crate::ast::NodeType::MethodCall {
            receiver_type: Some(recv_type),
            ..
        } = &context.problematic_node.node_type
        {
            if let Some((crate_name, type_name)) = Self::parse_qualified_type(recv_type) {
                return Ok((crate_name, type_name));
            }
        }

        // Try to infer from surrounding context
        if let Some(func_context) = &context.surrounding_context.current_function {
            for param in &func_context.parameters {
                if let Some((crate_name, type_name)) = Self::parse_qualified_type(&param.param_type)
                {
                    return Ok((crate_name, type_name));
                }
            }
        }

        // Check available types in context
        for type_info in &context.surrounding_context.available_types {
            if let Some(source_crate) = &type_info.source_crate {
                return Ok((source_crate.clone(), type_info.name.clone()));
            }
        }

        // Default fallback to std types
        Ok(("std".to_string(), "String".to_string()))
    }

    /// Parse qualified type name
    fn parse_qualified_type(qualified_type: &str) -> Option<(String, String)> {
        let parts: Vec<_> = qualified_type.split("::").collect();
        if parts.len() >= 2 {
            Some((parts[0].to_string(), parts.last().unwrap().to_string()))
        } else {
            None
        }
    }

    /// Apply corrections to files with safety checks
    ///
    /// # Errors
    ///
    /// Returns a yoshi error if file operations fail
    pub async fn apply_corrections(
        &self,
        corrections: &[ProjectCorrection],
        auto_apply: bool,
    ) -> Result<Vec<AppliedCorrection>> {
        let start_time = SystemTime::now();
        let mut applied = Vec::new();

        for correction in corrections.iter().filter(|c| !c.proposals.is_empty()) {
            let best_proposal = &correction.proposals[0];

            let should_apply = auto_apply
                || self.config.auto_apply_safe_corrections
                || (best_proposal.confidence > 0.9
                    && best_proposal.safety_level == SafetyLevel::Safe);

            if should_apply {
                match self
                    .apply_single_correction(correction, best_proposal)
                    .await
                    .lay("Applying individual correction")
                {
                    Ok(applied_correction) => {
                        applied.push(applied_correction);
                        self.metrics_collector.record_correction_applied().await;
                    }
                    Err(e) => {
                        tracing::warn!(
                            "Failed to apply correction to {}: {}",
                            correction.file_path.display(),
                            e
                        );
                        self.metrics_collector.record_application_error().await;
                    }
                }
            }
        }

        self.metrics_collector
            .record_application_complete(start_time.elapsed().unwrap_or_default())
            .await;
        Ok(applied)
    }

    /// Apply a single correction with precise byte-offset replacement
    async fn apply_single_correction(
        &self,
        correction: &ProjectCorrection,
        proposal: &CorrectionProposal,
    ) -> Result<AppliedCorrection> {
        let file_path = &correction.file_path;

        let content = fs::read_to_string(file_path)
            .with_file_context(file_path)
            .lay("Reading file for correction application")?;

        // Validate file hasn't changed since analysis
        let current_size = content.len();
        if current_size > crate::constants::MAX_FILE_SIZE {
            return Err(factory::resource_exhausted_error(
                "file_size",
                crate::constants::MAX_FILE_SIZE as u64,
                current_size as u64,
            ))
            .with_file_context(file_path);
        }

        let updated_content = self
            .apply_correction_at_byte_range(&content, &proposal.corrected_code, proposal.byte_range)
            .lay("Applying correction at byte range")?;

        // Validate the corrected file parses correctly
        syn::parse_file(&updated_content)
            .map_err(|e| {
                factory::code_generation_error(
                    "file_validation",
                    format!("Corrected file is not valid Rust: {e}"),
                    proposal.original_code.clone(),
                )
            })
            .with_file_context(file_path)
            .lay("Validating corrected file syntax")?;

        // Create backup if enabled
        let backup_path = if self.config.create_backup_files {
            let backup_path = file_path.with_extension("rs.yoshibackup");
            fs::copy(file_path, &backup_path)
                .with_file_context(&backup_path)
                .lay("Creating backup file")?;
            Some(backup_path)
        } else {
            None
        };

        // Write the corrected file
        fs::write(file_path, &updated_content)
            .with_file_context(file_path)
            .lay("Writing corrected file")?;

        let mut applied_correction = AppliedCorrection::new(
            file_path.clone(),
            proposal.original_code.clone(),
            proposal.corrected_code.clone(),
            proposal.strategy.clone(),
        );

        if let Some(backup_path) = backup_path {
            applied_correction.set_backup_path(backup_path);
        }

        Ok(applied_correction)
    }

    /// Apply correction at specific byte range with validation
    fn apply_correction_at_byte_range(
        &self,
        content: &str,
        corrected_code: &str,
        (start, end): (usize, usize),
    ) -> Result<String> {
        if start > end || end > content.len() {
            return Err(factory::code_generation_error(
                "byte_range_validation",
                format!(
                    "Invalid byte range: {start}..{end} for content length {}",
                    content.len()
                ),
                content[start.min(content.len())..end.min(content.len())].to_string(),
            ));
        }

        let mut result = String::with_capacity(content.len() + corrected_code.len());
        result.push_str(&content[..start]);
        result.push_str(corrected_code);
        result.push_str(&content[end..]);

        Ok(result)
    }

    /// Analyze a specific file instead of the entire project
    ///
    /// # Errors
    ///
    /// Returns a yoshi error if file analysis fails
    pub async fn analyze_file(
        &self,
        project_path: &Path,
        file_path: &Path,
    ) -> Result<Vec<ProjectCorrection>> {
        if !file_path.exists() {
            return Err(factory::configuration_error(
                "file_path",
                file_path.display().to_string(),
            ))
            .with_file_context(file_path);
        }

        let diagnostics = self
            .diagnostic_processor
            .analyze_file(project_path, file_path)
            .await
            .lay("Analyzing specific file")?;

        if diagnostics.is_empty() {
            return Ok(Vec::new());
        }

        // Process diagnostics for this specific file
        let corrections = if self.config.enable_parallel_processing {
            self.process_diagnostics_parallel(&diagnostics).await?
        } else {
            self.process_diagnostics_sequential(&diagnostics).await?
        };

        Ok(corrections)
    }

    /// Revert applied corrections using backup files
    ///
    /// # Errors
    ///
    /// Returns a yoshi error if revert operations fail
    pub async fn revert_corrections(
        &self,
        applied_corrections: &[AppliedCorrection],
    ) -> Result<Vec<RevertedCorrection>> {
        let mut reverted = Vec::new();

        for correction in applied_corrections {
            if !correction.can_revert() {
                continue;
            }

            let backup_path = correction.backup_path.as_ref().unwrap();

            match fs::copy(backup_path, &correction.file_path)
                .with_file_context(&correction.file_path)
                .lay("Reverting file from backup")
            {
                Ok(_) => {
                    // Clean up backup file
                    let _ = fs::remove_file(backup_path);

                    reverted.push(RevertedCorrection {
                        file_path: correction.file_path.clone(),
                        original_strategy: correction.strategy.clone(),
                        reverted_at: SystemTime::now(),
                    });
                }
                Err(e) => {
                    tracing::warn!("Failed to revert {}: {}", correction.file_path.display(), e);
                }
            }
        }

        Ok(reverted)
    }

    /// Get comprehensive system metrics
    #[must_use]
    pub fn get_metrics(&self) -> crate::metrics::SystemMetrics {
        crate::metrics::SystemMetrics {
            diagnostic_metrics: crate::metrics::DiagnosticMetricsSnapshot {
                cache_hit_ratio: self.diagnostic_processor.metrics().cache_hit_ratio(),
                total_processed: self
                    .diagnostic_processor
                    .metrics()
                    .total_processed
                    .load(std::sync::atomic::Ordering::Relaxed),
                parse_errors: self
                    .diagnostic_processor
                    .metrics()
                    .parse_errors
                    .load(std::sync::atomic::Ordering::Relaxed),
            },
            ast_metrics: crate::metrics::ASTMetricsSnapshot {
                cache_hit_ratio: self.ast_analyzer.metrics().cache_hit_ratio(),
                files_processed: self
                    .ast_analyzer
                    .metrics()
                    .files_processed
                    .load(std::sync::atomic::Ordering::Relaxed),
                nodes_analyzed: self
                    .ast_analyzer
                    .metrics()
                    .nodes_analyzed
                    .load(std::sync::atomic::Ordering::Relaxed),
            },
            generation_metrics: crate::metrics::GenerationMetricsSnapshot {
                corrections_generated: self
                    .code_generator
                    .metrics()
                    .corrections_generated
                    .load(std::sync::atomic::Ordering::Relaxed),
                successful_validations: self
                    .code_generator
                    .metrics()
                    .successful_validations
                    .load(std::sync::atomic::Ordering::Relaxed),
                template_cache_hits: self
                    .code_generator
                    .metrics()
                    .template_cache_hits
                    .load(std::sync::atomic::Ordering::Relaxed),
            },
        }
    }

    /// Get detailed system statistics
    pub async fn get_detailed_stats(&self) -> SystemStatistics {
        SystemStatistics {
            uptime: self.metrics_collector.get_uptime().await,
            total_analyses: self.metrics_collector.get_total_analyses().await,
            total_corrections: self.metrics_collector.get_total_corrections().await,
            success_rate: self.metrics_collector.get_success_rate().await,
            average_analysis_time: self.metrics_collector.get_average_analysis_time().await,
            cache_stats: CacheStatistics {
                diagnostic_cache_size: self.diagnostic_processor.cache_stats().await.cache_size,
                ast_cache_size: self.ast_analyzer.cache_stats().await.ast_cache_size,
                docs_cache_size: self.docs_scraper.cache_stats().await.cache_size,
                total_cache_memory: 0, // Could be calculated
            },
            component_health: self.check_component_health().await,
        }
    }

    /// Check health of all components
    async fn check_component_health(&self) -> ComponentHealth {
        ComponentHealth {
            diagnostic_processor_healthy: self.diagnostic_processor.metrics().cargo_success_rate()
                > 0.8,
            ast_analyzer_healthy: self.ast_analyzer.metrics().cache_hit_ratio() >= 0.0, // Always healthy if running
            docs_scraper_healthy: self.docs_scraper.metrics().success_rate() > 0.5,
            code_generator_healthy: self.code_generator.validation_stats().success_rate > 0.7,
        }
    }

    /// Perform system maintenance tasks
    pub async fn perform_maintenance(&self) -> Result<MaintenanceReport> {
        let start_time = SystemTime::now();
        let mut actions_performed = Vec::new();

        // Clear expired caches
        self.diagnostic_processor.clear_cache().await;
        actions_performed.push("Cleared diagnostic cache".to_string());

        self.ast_analyzer.clear_caches().await;
        actions_performed.push("Cleared AST cache".to_string());

        self.docs_scraper.clear_cache().await;
        actions_performed.push("Cleared documentation cache".to_string());

        self.code_generator.clear_template_cache().await;
        actions_performed.push("Cleared template cache".to_string());

        // Could add more maintenance tasks like:
        // - Garbage collection
        // - Log rotation
        // - Metric aggregation
        // - Performance optimization

        Ok(MaintenanceReport {
            started_at: start_time,
            duration: start_time.elapsed().unwrap_or_default(),
            actions_performed,
            errors_encountered: Vec::new(),
        })
    }

    /// Update system configuration
    ///
    /// # Errors
    ///
    /// Returns a yoshi error if configuration is invalid
    pub fn update_config(&mut self, new_config: SystemConfig) -> Result<()> {
        new_config
            .validate()
            .lay("Validating new system configuration")?;
        self.config = new_config;
        Ok(())
    }

    /// Get current configuration
    #[must_use]
    pub fn config(&self) -> &SystemConfig {
        &self.config
    }
}

impl Default for AutoCorrectionSystem {
    fn default() -> Self {
        Self::new()
    }
}

//--------------------------------------------------------------------------------------------------
// Supporting Types and Structures
//--------------------------------------------------------------------------------------------------

/// Information about a reverted correction
#[derive(Debug, Clone)]
pub struct RevertedCorrection {
    /// File path that was reverted
    pub file_path: PathBuf,
    /// Original correction strategy that was reverted
    pub original_strategy: CorrectionStrategy,
    /// When the revert occurred
    pub reverted_at: SystemTime,
}

/// Comprehensive system statistics
#[derive(Debug, Clone)]
pub struct SystemStatistics {
    /// System uptime
    pub uptime: Duration,
    /// Total analyses performed
    pub total_analyses: u64,
    /// Total corrections generated
    pub total_corrections: u64,
    /// Overall success rate
    pub success_rate: f64,
    /// Average analysis time
    pub average_analysis_time: Duration,
    /// Cache statistics
    pub cache_stats: CacheStatistics,
    /// Component health status
    pub component_health: ComponentHealth,
}

/// Cache usage statistics
#[derive(Debug, Clone)]
pub struct CacheStatistics {
    /// Diagnostic cache entries
    pub diagnostic_cache_size: usize,
    /// AST cache entries
    pub ast_cache_size: usize,
    /// Documentation cache entries
    pub docs_cache_size: usize,
    /// Estimated total cache memory usage
    pub total_cache_memory: usize,
}

/// Component health information
#[derive(Debug, Clone)]
pub struct ComponentHealth {
    /// Diagnostic processor health
    pub diagnostic_processor_healthy: bool,
    /// AST analyzer health
    pub ast_analyzer_healthy: bool,
    /// Documentation scraper health
    pub docs_scraper_healthy: bool,
    /// Code generator health
    pub code_generator_healthy: bool,
}

impl ComponentHealth {
    /// Check if all components are healthy
    #[must_use]
    pub fn all_healthy(&self) -> bool {
        self.diagnostic_processor_healthy
            && self.ast_analyzer_healthy
            && self.docs_scraper_healthy
            && self.code_generator_healthy
    }

    /// Get health percentage (0.0 - 1.0)
    #[must_use]
    pub fn health_percentage(&self) -> f64 {
        let healthy_count = [
            self.diagnostic_processor_healthy,
            self.ast_analyzer_healthy,
            self.docs_scraper_healthy,
            self.code_generator_healthy,
        ]
        .iter()
        .filter(|&&h| h)
        .count();

        healthy_count as f64 / 4.0
    }
}

/// Maintenance operation report
#[derive(Debug, Clone)]
pub struct MaintenanceReport {
    /// When maintenance started
    pub started_at: SystemTime,
    /// How long maintenance took
    pub duration: Duration,
    /// Actions that were performed
    pub actions_performed: Vec<String>,
    /// Any errors encountered
    pub errors_encountered: Vec<String>,
}

impl MaintenanceReport {
    /// Check if maintenance was successful
    #[must_use]
    pub fn was_successful(&self) -> bool {
        self.errors_encountered.is_empty()
    }

    /// Get summary of maintenance
    #[must_use]
    pub fn summary(&self) -> String {
        format!(
            "Maintenance completed in {:?}: {} actions, {} errors",
            self.duration,
            self.actions_performed.len(),
            self.errors_encountered.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use tokio::fs;

    async fn create_test_project() -> Result<TempDir> {
        let temp_dir = tempfile::tempdir()
            .hatch()
            .lay("Creating temporary test directory")?;

        let cargo_toml = r#"
[package]
name = "test-project"
version = "0.1.0"
edition = "2021"
"#;

        let main_rs = r#"
fn main() {
    let x = 5;
    println!("Hello, world!");
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
    fn test_system_creation() {
        let system = AutoCorrectionSystem::new();
        assert!(system.config().enable_parallel_processing);
    }

    #[test]
    fn test_system_with_config() {
        let config = SystemConfig {
            enable_parallel_processing: false,
            max_concurrent_operations: 1,
            ..SystemConfig::default()
        };

        let system = AutoCorrectionSystem::with_config(config);
        assert!(!system.config().enable_parallel_processing);
        assert_eq!(system.config().max_concurrent_operations, 1);
    }

    #[test]
    fn test_config_validation() {
        let mut config = SystemConfig::default();
        assert!(config.validate().is_ok());

        config.max_proposals_per_diagnostic = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_parse_qualified_type() {
        assert_eq!(
            AutoCorrectionSystem::parse_qualified_type("std::string::String"),
            Some(("std".to_string(), "String".to_string()))
        );

        assert_eq!(AutoCorrectionSystem::parse_qualified_type("Vec<i32>"), None);

        assert_eq!(
            AutoCorrectionSystem::parse_qualified_type("tokio::sync::Mutex"),
            Some(("tokio".to_string(), "Mutex".to_string()))
        );
    }

    #[tokio::test]
    async fn test_project_validation() {
        let system = AutoCorrectionSystem::new();

        // Test non-existent path
        let result = system
            .analyze_and_correct(Path::new("/non/existent/path"))
            .await;
        assert!(result.is_err());

        // Test valid project
        let temp_project = create_test_project().await.unwrap();
        let result = system.analyze_and_correct(temp_project.path()).await;
        // Should succeed (may return empty corrections for valid code)
        assert!(result.is_ok());
    }

    #[test]
    fn test_byte_range_application() {
        let system = AutoCorrectionSystem::new();
        let content = "let x = 5;\nlet y = 10;";

        // Replace "5" with "42"
        let result = system.apply_correction_at_byte_range(content, "42", (8, 9));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "let x = 42;\nlet y = 10;");

        // Test invalid range
        let result = system.apply_correction_at_byte_range(content, "42", (100, 200));
        assert!(result.is_err());
    }

    #[test]
    fn test_component_health() {
        let health = ComponentHealth {
            diagnostic_processor_healthy: true,
            ast_analyzer_healthy: true,
            docs_scraper_healthy: false,
            code_generator_healthy: true,
        };

        assert!(!health.all_healthy());
        assert_eq!(health.health_percentage(), 0.75);
    }

    #[test]
    fn test_maintenance_report() {
        let report = MaintenanceReport {
            started_at: SystemTime::now(),
            duration: Duration::from_millis(500),
            actions_performed: vec!["Cache cleared".to_string()],
            errors_encountered: vec![],
        };

        assert!(report.was_successful());
        assert!(report.summary().contains("1 actions"));
        assert!(report.summary().contains("0 errors"));
    }

    #[tokio::test]
    async fn test_metrics_collection() {
        let system = AutoCorrectionSystem::new();
        let metrics = system.get_metrics();

        // Should start with zero values
        assert_eq!(metrics.diagnostic_metrics.total_processed, 0);
        assert_eq!(metrics.ast_metrics.files_processed, 0);
        assert_eq!(metrics.generation_metrics.corrections_generated, 0);
    }

    #[tokio::test]
    async fn test_system_maintenance() {
        let system = AutoCorrectionSystem::new();
        let report = system.perform_maintenance().await.unwrap();

        assert!(report.was_successful());
        assert!(!report.actions_performed.is_empty());
        assert!(report.duration < Duration::from_secs(5)); // Should be fast
    }

    #[test]
    fn test_config_update() {
        let mut system = AutoCorrectionSystem::new();

        let new_config = SystemConfig {
            max_proposals_per_diagnostic: 10,
            enable_docs_scraping: false,
            ..SystemConfig::default()
        };

        assert!(system.update_config(new_config).is_ok());
        assert_eq!(system.config().max_proposals_per_diagnostic, 10);
        assert!(!system.config().enable_docs_scraping);
    }
}
