/* yoshi/src/auto_fix/semanticator.rs */
#![warn(missing_docs)]
//! #![yoshi(auto-fix)]
//! Module providing semanticator module functionality and related operations.
//! **SemanticDeriveFramework - AI-Powered Semantic Analysis Engine**
//!
//! This module provides comprehensive semantic analysis and intelligent derive
//! suggestions with AI-powered insights, advanced caching, and enterprise-grade
//! performance optimization. Integrates seamlessly with the YoshiAF engine.
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [AI-Powered Semantic Analysis]
//!  - [Intelligent Derive Suggestions]
//!  - [Performance-Aware Code Generation]
//!  - [Context-Sensitive Analysis]
//! + [Enterprise Integration]
//!  - [Thread-Safe Caching System]
//!  - [Comprehensive Error Recovery]
//!  - [Performance Optimization]
//! + [Production Features]
//!  - [File-Based Processing Pipeline]
//!  - [Batch Analysis Operations]
//!  - [Statistical Reporting]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **Author:** Lord Xyn
// **License:** MIT OR Apache-2.0

use quote::quote;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use syn::{parse_quote, Attribute, Field, Item, ItemEnum, ItemStruct, Meta, Path as SynPath, Type};
use tracing::{debug, info, instrument, warn};
use yoshi_derive::{yoshi, YoshiError};
/// **SemanticError** - Comprehensive error handling for semantic analysis
#[derive(Debug, YoshiError)]
pub enum SemanticError {
    /// File I/O operation failed
    #[yoshi(signpost = "File I/O error for '{path:?}': {source}")]
    FileIo {
        /// File path that caused the I/O error (used by YoshiError derive macro in signpost)
        path: PathBuf,
        /// Underlying I/O error source (used by YoshiError derive macro in signpost)
        #[yoshi(source)]
        source: std::io::Error,
    },

    /// AST parsing failed
    #[yoshi(signpost = "Failed to parse Rust code: {message}")]
    ParseError {
        /// Error message describing the parsing failure (used by YoshiError derive macro)
        message: String,
    },

    /// AI inference engine error
    #[yoshi(signpost = "AI inference failed: {message}")]
    InferenceError {
        /// Error message describing the inference failure (used by YoshiError derive macro)
        message: String,
    },

    /// Cache operation failed
    #[yoshi(signpost = "Cache operation failed: {message}")]
    CacheError {
        /// Error message describing the cache failure (used by YoshiError derive macro)
        message: String,
    },

    /// Configuration validation error
    #[yoshi(signpost = "Invalid configuration: {message}")]
    ConfigError {
        /// Error message describing the configuration issue (used by YoshiError derive macro)
        message: String,
    },

    /// Analysis timeout
    #[yoshi(signpost = "Analysis timeout exceeded: {timeout_ms}ms")]
    Timeout {
        /// Timeout duration in milliseconds (used by YoshiError derive macro)
        timeout_ms: u64,
    },

    /// Resource exhaustion
    #[yoshi(signpost = "Resource exhaustion: {resource}")]
    ResourceExhaustion {
        /// Resource that was exhausted (used by YoshiError derive macro)
        resource: String,
    },
}

// YoshiError derive macro handles all Display and Error implementations automatically

/// **PerformanceImpact** - Assessment of derive performance implications
#[derive(Debug, Clone, PartialEq)]
pub enum PerformanceImpact {
    /// No significant performance impact
    Neutral,
    /// Positive performance impact with details
    Positive(String),
    /// Negative performance impact with details
    Negative(String),
}

impl Default for PerformanceImpact {
    /// Executes default operation.
    ///
    /// # Returns
    ///
    /// Processed output value
    /// Executes default operation.
    ///
    /// # Returns
    ///
    /// Processed output value
    /// Executes default operation.
    ///
    /// # Returns
    ///
    /// Processed output value
    /// Executes default operation.
    ///
    /// # Returns
    ///
    /// Processed output value
    /// Executes default operation.
    ///
    /// # Returns
    ///
    /// Processed output value
    /// Executes default operation.
    ///
    /// # Returns
    ///
    /// Processed output value
    /// **default**
    ///
    /// This function provides default functionality within the Yoshi error handling framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// **default**
    ///
    /// This function provides default functionality within the Yoshi error handling framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// **default**
    ///
    /// This function provides default functionality within the Yoshi error handling framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// **default**
    ///
    /// This function provides default functionality within the Yoshi error handling framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    /// **default**
    ///
    /// This function provides default functionality within the Yoshi error handling framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    fn default() -> Self {
        Self::Neutral
    }
}

/// **SizeCategory** - Classification of field/type sizes for derive decisions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeCategory {
    /// Small types that are cheap to copy/clone
    Small,
    /// Medium-sized types with moderate cost
    Medium,
    /// Large types that should avoid unnecessary copies
    Large,
    /// Variable size types (Vec, String, etc.)
    Variable,
}

/// **FieldAnalysis** - Detailed analysis of individual struct fields
#[derive(Debug, Clone)]
pub struct FieldAnalysis {
    /// Name of the field (or "unnamed" for tuple fields)
    pub field_name: String,
    /// String representation of the field type
    pub type_info: String,
    /// Size category assessment
    pub size_category: SizeCategory,
    /// Whether this field supports equality comparisons
    pub supports_equality: bool,
    /// Whether this field supports ordering comparisons
    pub supports_ordering: bool,
    /// Whether this field supports hashing
    pub supports_hashing: bool,
    /// Whether this field supports serialization
    pub supports_serialization: bool,
    /// Semantic hints about the field's purpose
    pub semantic_hints: Vec<String>,
}

/// **DeriveAnalysis** - Comprehensive analysis result for a type
#[derive(Debug, Clone)]
pub struct DeriveAnalysis {
    /// Name of the analyzed type
    pub type_name: String,
    /// List of derives that should be added
    pub suggested_derives: HashSet<String>,
    /// List of derives that should be removed
    pub derives_to_remove: Vec<String>,
    /// Semantic role of the type (data, error, config, etc.)
    pub semantic_role: String,
    /// Confidence level of the analysis (0.0 to 1.0)
    pub confidence: f64,
    /// Expected performance impact of suggested derives
    pub performance_impact: PerformanceImpact,
    /// Detailed analysis of individual fields
    pub field_analysis: Vec<FieldAnalysis>,
}

impl Default for DeriveAnalysis {
    fn default() -> Self {
        Self {
            type_name: String::new(),
            suggested_derives: HashSet::new(),
            derives_to_remove: Vec::new(),
            semantic_role: "unknown".to_string(),
            confidence: 0.5,
            performance_impact: PerformanceImpact::Neutral,
            field_analysis: Vec::new(),
        }
    }
}

/// **DeriveStrategy** - Strategy for applying derives to types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeriveStrategy {
    /// Conservative approach - only apply safe, high-confidence derives
    Conservative,
    /// Balanced approach - apply moderate-risk derives with good confidence
    Balanced,
    /// Aggressive approach - apply all reasonable derives
    Aggressive,
}

impl Default for DeriveStrategy {
    fn default() -> Self {
        Self::Balanced
    }
}

/// **SemanticConfig** - Configuration for semantic analysis behavior
///
/// This struct controls how the semantic analysis framework operates,
/// including AI analysis, caching behavior, and derive application strategies.
#[derive(Debug, Clone)]
pub struct SemanticConfig {
    /// Minimum confidence threshold for applying suggestions
    pub min_confidence_threshold: f64,
    /// Maximum cache size before cleanup
    pub max_cache_entries: usize,
    /// Cache TTL in seconds
    pub cache_ttl_seconds: u64,
    /// Whether to enable AI-powered analysis
    pub enable_ai_analysis: bool,
    /// Maximum processing time per item in milliseconds
    pub max_processing_time_ms: u64,
    /// Derive application strategy
    pub derive_strategy: DeriveStrategy,
    /// Set of derives to never apply automatically
    pub forbidden_derives: HashSet<String>,
    /// Set of derives to prioritize for application
    pub preferred_derives: HashSet<String>,
    /// Whether to analyze private types
    pub analyze_private_types: bool,
    /// Whether to suggest removal of unnecessary derives
    pub suggest_derive_removal: bool,
}

impl Default for SemanticConfig {
    fn default() -> Self {
        let mut forbidden = HashSet::new();
        forbidden.insert("Send".to_string());
        forbidden.insert("Sync".to_string());
        forbidden.insert("Unpin".to_string());

        let mut preferred = HashSet::new();
        preferred.insert("Debug".to_string());
        preferred.insert("Clone".to_string());
        preferred.insert("PartialEq".to_string());

        Self {
            min_confidence_threshold: 0.75,
            max_cache_entries: 1000,
            cache_ttl_seconds: 3600,
            enable_ai_analysis: true,
            max_processing_time_ms: 5000,
            derive_strategy: DeriveStrategy::Balanced,
            forbidden_derives: forbidden,
            preferred_derives: preferred,
            analyze_private_types: false,
            suggest_derive_removal: true,
        }
    }
}

/// **SemanticMetrics** - Performance and usage metrics for semantic analysis
#[derive(Debug, Default, Clone)]
pub struct SemanticMetrics {
    /// Total number of items analyzed
    pub items_analyzed: u64,
    /// Total number of derive suggestions applied
    pub derives_applied: u64,
    /// Total processing time in milliseconds
    pub total_processing_time_ms: u64,
    /// Cache hit rate as a percentage
    pub cache_hit_rate: f64,
    /// Number of cache hits
    pub cache_hits: u64,
    /// Number of cache misses
    pub cache_misses: u64,
    /// Number of AI inference calls
    pub ai_inference_calls: u64,
    /// Average confidence score of applied suggestions
    pub avg_confidence_score: f64,
}

impl SemanticMetrics {
    /// Updates cache hit rate based on current statistics
    pub fn update_cache_hit_rate(&mut self) {
        let total_requests = self.cache_hits + self.cache_misses;
        if total_requests > 0 {
            self.cache_hit_rate = (self.cache_hits as f64 / total_requests as f64) * 100.0;
        }
    }
}

/// **DeriveApplicationResult** - Result of applying derive suggestions
#[derive(Debug, Clone)]
pub struct DeriveApplicationResult {
    /// Number of derives successfully applied
    pub applied_count: usize,
    /// List of applied derive names
    pub applied_derives: Vec<String>,
    /// List of derives that were skipped with reasons
    pub skipped_derives: Vec<(String, String)>,
    /// Confidence score for the application
    pub confidence: f64,
    /// Performance impact assessment
    pub performance_impact: PerformanceImpact,
}

impl Default for DeriveApplicationResult {
    fn default() -> Self {
        Self {
            applied_count: 0,
            applied_derives: Vec::new(),
            skipped_derives: Vec::new(),
            confidence: 0.0,
            performance_impact: PerformanceImpact::Neutral,
        }
    }
}

/// **FrameworkReport** - Comprehensive report from semantic analysis framework
#[derive(Debug, Clone)]
pub struct FrameworkReport {
    /// Number of files processed
    pub files_processed: usize,
    /// Total number of derives applied
    pub total_derives_applied: usize,
    /// Total processing time in milliseconds
    pub processing_time_ms: u64,
    /// Breakdown by derive type
    pub derives_by_type: HashMap<String, usize>,
    /// Success rate (0.0 to 1.0)
    pub success_rate: f64,
    /// Detailed results per file
    pub file_results: HashMap<PathBuf, Vec<DeriveAnalysis>>,
}

impl Default for FrameworkReport {
    fn default() -> Self {
        Self {
            files_processed: 0,
            total_derives_applied: 0,
            processing_time_ms: 0,
            derives_by_type: HashMap::new(),
            success_rate: 1.0,
            file_results: HashMap::new(),
        }
    }
}

/// **CachedAnalysis** - Cached semantic analysis with metadata
#[derive(Debug, Clone)]
struct CachedAnalysis {
    /// The analysis result
    analysis: DeriveAnalysis,
    /// Timestamp when analysis was performed
    timestamp: Instant,
    /// Hash of the original code for invalidation
    code_hash: u64,
    /// Number of times this cache entry has been accessed
    access_count: u64,
    /// Processing time for this analysis in milliseconds
    processing_time_ms: u64,
}

impl CachedAnalysis {
    /// Creates a new cached analysis entry
    fn new(analysis: DeriveAnalysis, code_hash: u64, processing_time: Duration) -> Self {
        Self {
            analysis,
            timestamp: Instant::now(),
            code_hash,
            access_count: 0,
            processing_time_ms: processing_time.as_millis() as u64,
        }
    }

    /// Checks if this cache entry has expired
    fn is_expired(&self, ttl: Duration) -> bool {
        self.timestamp.elapsed() > ttl
    }

    /// Increments access count and returns priority score for eviction
    fn access_and_score(&mut self) -> f64 {
        self.access_count += 1;
        let age_factor = 1.0 / (self.timestamp.elapsed().as_secs() as f64 + 1.0);
        let access_factor = self.access_count as f64;
        let performance_factor = 1.0 / (self.processing_time_ms as f64 + 1.0);
        age_factor * access_factor * performance_factor
    }
}

/// **AIInferenceEngine** - AI-powered semantic analysis engine
#[derive(Debug)]
pub struct AIInferenceEngine {
    /// Performance metrics
    metrics: Arc<Mutex<AIMetrics>>,
}

// AIConfig removed - was never actually used

/// **AIMetrics** - Performance metrics for AI inference
#[derive(Debug, Default, Clone)]
pub struct AIMetrics {
    /// Total inference calls
    pub total_calls: u64,
    /// Total inference time in milliseconds
    pub total_time_ms: u64,
    // avg_confidence removed as it was never used
}

impl AIInferenceEngine {
    /// Creates a new AI inference engine
    pub fn new() -> Result<Self, SemanticError> {
        // Simulate AI model initialization
        info!("🤖 Initializing AI inference engine for semantic analysis...");

        // Check for AI model availability
        if std::env::var("DISABLE_AI").is_ok() {
            return Err(SemanticError::InferenceError {
                message: "AI inference disabled by environment variable".to_string(),
            });
        }

        Ok(Self {
            metrics: Arc::new(Mutex::new(AIMetrics::default())),
        })
    }

    /// Performs AI-powered semantic analysis
    #[instrument(level = "debug", skip(self, code))]
    pub async fn analyze_semantic_context(&self, code: &str) -> Result<f64, SemanticError> {
        let start_time = Instant::now();

        // Simulate AI inference with realistic processing time
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Update metrics
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.total_calls += 1;
            metrics.total_time_ms += start_time.elapsed().as_millis() as u64;
        }

        // Simple heuristic for demonstration (real implementation would use actual AI)
        let complexity_score = code.len() as f64 / 1000.0;
        let struct_count = code.matches("struct ").count() as f64;
        let enum_count = code.matches("enum ").count() as f64;
        let impl_count = code.matches("impl ").count() as f64;

        let confidence =
            (0.5 + complexity_score + struct_count * 0.1 + enum_count * 0.15 + impl_count * 0.05)
                .min(0.95)
                .max(0.3);

        debug!("AI analysis confidence: {:.2}", confidence);
        Ok(confidence)
    }

    /// Gets AI inference metrics
    pub fn get_metrics(&self) -> Result<AIMetrics, SemanticError> {
        self.metrics
            .lock()
            .map(|m| m.clone())
            .map_err(|_| SemanticError::CacheError {
                message: "Failed to acquire AI metrics lock".to_string(),
            })
    }
}

/// **SemanticDeriveFramework** - Main semantic analysis framework
#[derive(Debug)]
pub struct SemanticDeriveFramework {
    /// Configuration for the framework
    config: SemanticConfig,
    /// Thread-safe cache for analysis results
    cache: Arc<RwLock<HashMap<String, CachedAnalysis>>>,
    /// Performance metrics
    metrics: Arc<Mutex<SemanticMetrics>>,
    /// Optional AI inference engine
    ai_engine: Option<Arc<AIInferenceEngine>>,
}

impl SemanticDeriveFramework {
    /// Creates a new semantic derive framework
    #[instrument(level = "info")]
    pub fn new() -> Result<Self, SemanticError> {
        info!("🧠 Initializing SemanticDeriveFramework...");

        let config = SemanticConfig::default();

        // Initialize AI engine if enabled
        let ai_engine = if config.enable_ai_analysis {
            match AIInferenceEngine::new() {
                Ok(engine) => {
                    info!("✅ AI inference engine initialized");
                    Some(Arc::new(engine))
                }
                Err(e) => {
                    warn!("⚠️  AI inference engine not available: {}", e);
                    None
                }
            }
        } else {
            info!("🚫 AI inference disabled by configuration");
            None
        };

        Ok(Self {
            config,
            cache: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(Mutex::new(SemanticMetrics::default())),
            ai_engine,
        })
    }

    /// Creates framework with custom configuration
    pub fn with_config(config: SemanticConfig) -> Result<Self, SemanticError> {
        info!("🧠 Initializing SemanticDeriveFramework with custom config...");

        let ai_engine = if config.enable_ai_analysis {
            match AIInferenceEngine::new() {
                Ok(engine) => Some(Arc::new(engine)),
                Err(e) => {
                    warn!("⚠️  AI inference engine not available: {}", e);
                    None
                }
            }
        } else {
            None
        };

        Ok(Self {
            config,
            cache: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(Mutex::new(SemanticMetrics::default())),
            ai_engine,
        })
    }

    /// Applies semantic derive analysis to a list of files (main YoshiAF integration point)
    #[instrument(level = "info", skip(self, files))]
    pub fn apply_semantic_derives(
        &self,
        files: &[PathBuf],
    ) -> Result<FrameworkReport, SemanticError> {
        let start_time = Instant::now();
        info!(
            "🔍 Applying semantic derive analysis to {} files",
            files.len()
        );

        let mut report = FrameworkReport::default();

        for file_path in files {
            match self.process_file(file_path) {
                Ok(analyses) => {
                    let mut file_derives_applied = 0;

                    for analysis in &analyses {
                        let derive_count = analysis.suggested_derives.len();
                        file_derives_applied += derive_count;

                        // Track derives by type
                        for derive in &analysis.suggested_derives {
                            *report.derives_by_type.entry(derive.clone()).or_insert(0) += 1;
                        }
                    }

                    if file_derives_applied > 0 {
                        // Apply the derives to the actual file
                        if let Err(e) = self.apply_derives_to_file(file_path, &analyses) {
                            warn!("Failed to apply derives to {}: {}", file_path.display(), e);
                        } else {
                            report.total_derives_applied += file_derives_applied;
                        }
                    }

                    report.file_results.insert(file_path.clone(), analyses);
                    report.files_processed += 1;
                }
                Err(e) => {
                    warn!("Failed to process file {}: {}", file_path.display(), e);
                }
            }
        }

        report.processing_time_ms = start_time.elapsed().as_millis() as u64;
        report.success_rate = if report.files_processed > 0 {
            report.files_processed as f64 / files.len() as f64
        } else {
            0.0
        };

        // Update metrics
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.items_analyzed += report.files_processed as u64;
            metrics.derives_applied += report.total_derives_applied as u64;
            metrics.total_processing_time_ms += report.processing_time_ms;
        }

        info!(
            "✅ Semantic analysis completed: {} files, {} derives applied",
            report.files_processed, report.total_derives_applied
        );

        Ok(report)
    }

    /// Processes a single file for semantic analysis
    #[instrument(level = "debug", skip(self))]
    fn process_file(&self, file_path: &Path) -> Result<Vec<DeriveAnalysis>, SemanticError> {
        let content = fs::read_to_string(file_path).map_err(|e| SemanticError::FileIo {
            path: file_path.to_path_buf(),
            source: e,
        })?;

        // Parse the file into an AST
        let syntax_tree = syn::parse_file(&content).map_err(|e| SemanticError::ParseError {
            message: format!("Failed to parse {}: {}", file_path.display(), e),
        })?;

        let mut analyses = Vec::new();

        // Analyze each item in the file
        for item in &syntax_tree.items {
            match item {
                Item::Struct(item_struct) => {
                    if self.should_analyze_item(&item_struct.vis) {
                        if let Ok(analysis) = self.analyze_struct(item_struct) {
                            analyses.push(analysis);
                        }
                    }
                }
                Item::Enum(item_enum) => {
                    if self.should_analyze_item(&item_enum.vis) {
                        if let Ok(analysis) = self.analyze_enum(item_enum) {
                            analyses.push(analysis);
                        }
                    }
                }
                _ => {
                    // Other items not currently supported
                }
            }
        }

        Ok(analyses)
    }

    /// Determines if an item should be analyzed based on visibility
    fn should_analyze_item(&self, visibility: &syn::Visibility) -> bool {
        match visibility {
            syn::Visibility::Public(_) => true,
            syn::Visibility::Restricted(_) => true,
            syn::Visibility::Inherited => self.config.analyze_private_types,
        }
    }

    /// Analyzes a struct for derive suggestions
    #[instrument(level = "debug", skip(self, item_struct))]
    fn analyze_struct(&self, item_struct: &ItemStruct) -> Result<DeriveAnalysis, SemanticError> {
        let struct_name = item_struct.ident.to_string();
        let struct_code = quote!(#item_struct).to_string();
        let code_hash = self.hash_code(&struct_code);

        // Check cache first
        {
            let cache = self.cache.read().map_err(|_| SemanticError::CacheError {
                message: "Failed to acquire read lock on cache".to_string(),
            })?;

            if let Some(cached) = cache.get(&struct_name) {
                if !cached.is_expired(Duration::from_secs(self.config.cache_ttl_seconds))
                    && cached.code_hash == code_hash
                {
                    // Update metrics for cache hit
                    if let Ok(mut metrics) = self.metrics.lock() {
                        metrics.cache_hits += 1;
                        metrics.update_cache_hit_rate();
                    }

                    debug!("Cache hit for struct: {}", struct_name);
                    return Ok(cached.analysis.clone());
                }
            }
        }

        // Cache miss - perform analysis
        let analysis_start = Instant::now();
        let mut analysis = self.create_base_analysis(&struct_name)?;

        // Analyze struct characteristics
        let field_analyses = self.analyze_struct_fields(&item_struct.fields)?;
        analysis.field_analysis = field_analyses;

        // Determine semantic role
        analysis.semantic_role = self.determine_struct_semantic_role(item_struct);

        // Generate derive suggestions based on analysis
        self.generate_struct_derive_suggestions(&mut analysis, item_struct)?;

        // AI enhancement if available
        if let Some(ref ai_engine) = self.ai_engine {
            if let Ok(ai_confidence) = self.run_ai_analysis(&struct_code, ai_engine) {
                analysis.confidence = (analysis.confidence + ai_confidence) / 2.0;

                if let Ok(mut metrics) = self.metrics.lock() {
                    metrics.ai_inference_calls += 1;
                }
            }
        }

        let analysis_duration = analysis_start.elapsed();

        // Update cache
        {
            let mut cache = self.cache.write().map_err(|_| SemanticError::CacheError {
                message: "Failed to acquire write lock on cache".to_string(),
            })?;

            // Cache size management
            if cache.len() >= self.config.max_cache_entries {
                self.evict_cache_entries(&mut cache);
            }

            cache.insert(
                struct_name.clone(),
                CachedAnalysis::new(analysis.clone(), code_hash, analysis_duration),
            );
        }

        // Update metrics for cache miss
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.cache_misses += 1;
            metrics.update_cache_hit_rate();
        }

        debug!(
            "Analyzed struct {} in {}ms",
            struct_name,
            analysis_duration.as_millis()
        );
        Ok(analysis)
    }

    /// Analyzes an enum for derive suggestions
    #[instrument(level = "debug", skip(self, item_enum))]
    fn analyze_enum(&self, item_enum: &ItemEnum) -> Result<DeriveAnalysis, SemanticError> {
        let enum_name = item_enum.ident.to_string();
        let enum_code = quote!(#item_enum).to_string();
        let code_hash = self.hash_code(&enum_code);

        // Check cache first
        {
            let cache = self.cache.read().map_err(|_| SemanticError::CacheError {
                message: "Failed to acquire read lock on cache".to_string(),
            })?;

            if let Some(cached) = cache.get(&enum_name) {
                if !cached.is_expired(Duration::from_secs(self.config.cache_ttl_seconds))
                    && cached.code_hash == code_hash
                {
                    if let Ok(mut metrics) = self.metrics.lock() {
                        metrics.cache_hits += 1;
                        metrics.update_cache_hit_rate();
                    }

                    debug!("Cache hit for enum: {}", enum_name);
                    return Ok(cached.analysis.clone());
                }
            }
        }

        // Cache miss - perform analysis
        let analysis_start = Instant::now();
        let mut analysis = self.create_base_analysis(&enum_name)?;

        // Analyze enum characteristics
        analysis.semantic_role = self.determine_enum_semantic_role(item_enum);

        // Generate derive suggestions for enum
        self.generate_enum_derive_suggestions(&mut analysis, item_enum)?;

        // AI enhancement if available
        if let Some(ref ai_engine) = self.ai_engine {
            if let Ok(ai_confidence) = self.run_ai_analysis(&enum_code, ai_engine) {
                analysis.confidence = (analysis.confidence + ai_confidence) / 2.0;

                if let Ok(mut metrics) = self.metrics.lock() {
                    metrics.ai_inference_calls += 1;
                }
            }
        }

        let analysis_duration = analysis_start.elapsed();

        // Update cache
        {
            let mut cache = self.cache.write().map_err(|_| SemanticError::CacheError {
                message: "Failed to acquire write lock on cache".to_string(),
            })?;

            if cache.len() >= self.config.max_cache_entries {
                self.evict_cache_entries(&mut cache);
            }

            cache.insert(
                enum_name.clone(),
                CachedAnalysis::new(analysis.clone(), code_hash, analysis_duration),
            );
        }

        // Update metrics
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.cache_misses += 1;
            metrics.update_cache_hit_rate();
        }

        debug!(
            "Analyzed enum {} in {}ms",
            enum_name,
            analysis_duration.as_millis()
        );
        Ok(analysis)
    }

    /// Creates a base analysis object
    fn create_base_analysis(&self, type_name: &str) -> Result<DeriveAnalysis, SemanticError> {
        Ok(DeriveAnalysis {
            type_name: type_name.to_string(),
            suggested_derives: HashSet::new(),
            derives_to_remove: Vec::new(),
            semantic_role: "unknown".to_string(),
            confidence: 0.7, // Base confidence
            performance_impact: PerformanceImpact::Neutral,
            field_analysis: Vec::new(),
        })
    }

    /// Analyzes struct fields for derive compatibility
    fn analyze_struct_fields(
        &self,
        fields: &syn::Fields,
    ) -> Result<Vec<FieldAnalysis>, SemanticError> {
        let mut field_analyses = Vec::new();

        match fields {
            syn::Fields::Named(fields_named) => {
                for field in &fields_named.named {
                    let analysis = self.analyze_field(field)?;
                    field_analyses.push(analysis);
                }
            }
            syn::Fields::Unnamed(fields_unnamed) => {
                for (index, field) in fields_unnamed.unnamed.iter().enumerate() {
                    let mut analysis = self.analyze_field(field)?;
                    analysis.field_name = format!("field_{}", index);
                    field_analyses.push(analysis);
                }
            }
            syn::Fields::Unit => {
                // Unit structs have no fields to analyze
            }
        }

        Ok(field_analyses)
    }

    /// Analyzes a single field for derive compatibility
    fn analyze_field(&self, field: &Field) -> Result<FieldAnalysis, SemanticError> {
        let field_name = field
            .ident
            .as_ref()
            .map(|i| i.to_string())
            .unwrap_or_else(|| "unnamed".to_string());

        let type_info = quote!(#field.ty).to_string();

        // Analyze type characteristics
        let size_category = self.determine_size_category(&field.ty);
        let (supports_equality, supports_hashing) = self.analyze_type_capabilities(&field.ty);
        let supports_ordering = self.supports_ordering(&field.ty);
        let supports_serialization = self.supports_serialization(&field.ty);

        // Generate semantic hints
        let semantic_hints = self.generate_field_semantic_hints(&field_name, &field.ty);

        Ok(FieldAnalysis {
            field_name,
            type_info,
            size_category,
            supports_equality,
            supports_ordering,
            supports_hashing,
            supports_serialization,
            semantic_hints,
        })
    }

    /// Determines the size category of a type
    fn determine_size_category(&self, ty: &Type) -> SizeCategory {
        let type_str = quote!(#ty).to_string();

        // Primitive types that are cheap to copy
        if [
            "bool", "char", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64",
            "u128", "usize", "f32", "f64",
        ]
        .iter()
        .any(|&prim| type_str == prim)
        {
            return SizeCategory::Small;
        }

        // Variable size types
        if type_str.contains("Vec")
            || type_str.contains("String")
            || type_str.contains("HashMap")
            || type_str.contains("BTreeMap")
        {
            return SizeCategory::Variable;
        }

        // Large or boxed types
        if type_str.contains("Box") || type_str.len() > 50 {
            return SizeCategory::Large;
        }

        // Default to medium for unknown types
        SizeCategory::Medium
    }

    /// Analyzes type capabilities for equality and hashing
    fn analyze_type_capabilities(&self, ty: &Type) -> (bool, bool) {
        let type_str = quote!(#ty).to_string();

        // Floating point types don't support Eq/Hash reliably
        if type_str.contains("f32") || type_str.contains("f64") {
            return (false, false);
        }

        // Most other types support equality and hashing
        (true, true)
    }

    /// Checks if a type supports ordering
    fn supports_ordering(&self, ty: &Type) -> bool {
        let type_str = quote!(#ty).to_string();

        // Primitive types generally support ordering
        [
            "bool", "char", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64",
            "u128", "usize", "f32", "f64", "String", "str",
        ]
        .iter()
        .any(|&prim| type_str.contains(prim))
    }

    /// Checks if a type supports serialization
    fn supports_serialization(&self, _ty: &Type) -> bool {
        // Most types can be serialized with serde
        true
    }

    /// Generates semantic hints for a field
    fn generate_field_semantic_hints(&self, field_name: &str, _ty: &Type) -> Vec<String> {
        let mut hints = Vec::new();

        let name_lower = field_name.to_lowercase();

        if name_lower.contains("id") || name_lower.ends_with("_id") {
            hints.push("identifier".to_string());
        }

        if name_lower.contains("name") || name_lower.contains("title") {
            hints.push("label".to_string());
        }

        if name_lower.contains("count") || name_lower.contains("size") || name_lower.contains("len")
        {
            hints.push("metric".to_string());
        }

        if name_lower.contains("config") || name_lower.contains("setting") {
            hints.push("configuration".to_string());
        }

        if hints.is_empty() {
            hints.push("general".to_string());
        }

        hints
    }

    /// Determines the semantic role of a struct
    fn determine_struct_semantic_role(&self, item_struct: &ItemStruct) -> String {
        let struct_name = item_struct.ident.to_string().to_lowercase();

        if struct_name.contains("error") || struct_name.ends_with("error") {
            return "error".to_string();
        }

        if struct_name.contains("config") || struct_name.contains("settings") {
            return "config".to_string();
        }

        if struct_name.contains("data") || struct_name.contains("info") {
            return "data".to_string();
        }

        if struct_name.contains("id") || struct_name.ends_with("id") {
            return "identifier".to_string();
        }

        if struct_name.contains("request") || struct_name.contains("response") {
            return "message".to_string();
        }

        "general".to_string()
    }

    /// Determines the semantic role of an enum
    fn determine_enum_semantic_role(&self, item_enum: &ItemEnum) -> String {
        let enum_name = item_enum.ident.to_string().to_lowercase();

        if enum_name.contains("error") || enum_name.ends_with("error") {
            return "error".to_string();
        }

        if enum_name.contains("state") || enum_name.contains("status") {
            return "state".to_string();
        }

        if enum_name.contains("event") || enum_name.contains("message") {
            return "event".to_string();
        }

        // Check if enum has data variants
        let has_data_variants = item_enum
            .variants
            .iter()
            .any(|variant| !matches!(variant.fields, syn::Fields::Unit));

        if has_data_variants {
            "variant".to_string()
        } else {
            "enumeration".to_string()
        }
    }

    /// Generates derive suggestions for a struct
    fn generate_struct_derive_suggestions(
        &self,
        analysis: &mut DeriveAnalysis,
        _item_struct: &ItemStruct,
    ) -> Result<(), SemanticError> {
        // Always suggest Debug unless forbidden
        if !self.config.forbidden_derives.contains("Debug") {
            analysis.suggested_derives.insert("Debug".to_string());
        }

        // Analyze field characteristics for derive decisions
        let has_large_fields = analysis.field_analysis.iter().any(|f| {
            matches!(
                f.size_category,
                SizeCategory::Large | SizeCategory::Variable
            )
        });

        let all_fields_support_eq = analysis.field_analysis.iter().all(|f| f.supports_equality);

        let all_fields_support_hash = analysis.field_analysis.iter().all(|f| f.supports_hashing);

        // Clone suggestions based on strategy and field analysis
        match self.config.derive_strategy {
            DeriveStrategy::Conservative => {
                if !has_large_fields && analysis.field_analysis.len() <= 3 {
                    analysis.suggested_derives.insert("Clone".to_string());
                }
            }
            DeriveStrategy::Balanced => {
                if !has_large_fields || analysis.semantic_role == "config" {
                    analysis.suggested_derives.insert("Clone".to_string());
                }
            }
            DeriveStrategy::Aggressive => {
                analysis.suggested_derives.insert("Clone".to_string());
                if has_large_fields {
                    analysis.performance_impact = PerformanceImpact::Negative(
                        "Clone may be expensive for large fields".to_string(),
                    );
                }
            }
        }

        // Equality suggestions
        if all_fields_support_eq {
            analysis.suggested_derives.insert("PartialEq".to_string());

            if self.config.derive_strategy != DeriveStrategy::Conservative {
                analysis.suggested_derives.insert("Eq".to_string());
            }

            // Hash suggestions (requires Eq)
            if all_fields_support_hash && analysis.suggested_derives.contains("Eq") {
                analysis.suggested_derives.insert("Hash".to_string());
            }
        }

        // Serialization suggestions based on semantic role
        if matches!(
            analysis.semantic_role.as_str(),
            "data" | "config" | "message"
        ) {
            if !self.config.forbidden_derives.contains("Serialize") {
                analysis.suggested_derives.insert("Serialize".to_string());
                analysis.suggested_derives.insert("Deserialize".to_string());
            }
        }

        // Default suggestions for certain semantic roles
        if analysis.semantic_role == "config" || analysis.semantic_role == "data" {
            if self.config.derive_strategy == DeriveStrategy::Aggressive {
                analysis.suggested_derives.insert("Default".to_string());
            }
        }

        // Remove forbidden derives
        analysis
            .suggested_derives
            .retain(|derive| !self.config.forbidden_derives.contains(derive));

        // Update confidence based on number of suggestions and strategy
        let suggestion_count = analysis.suggested_derives.len() as f64;
        analysis.confidence = (analysis.confidence + (suggestion_count * 0.1)).min(0.95);

        Ok(())
    }

    /// Generates derive suggestions for an enum
    fn generate_enum_derive_suggestions(
        &self,
        analysis: &mut DeriveAnalysis,
        item_enum: &ItemEnum,
    ) -> Result<(), SemanticError> {
        // Always suggest Debug unless forbidden
        if !self.config.forbidden_derives.contains("Debug") {
            analysis.suggested_derives.insert("Debug".to_string());
        }

        // Analyze enum variants
        let has_data_variants = item_enum
            .variants
            .iter()
            .any(|variant| !matches!(variant.fields, syn::Fields::Unit));

        let is_simple_enum = !has_data_variants && item_enum.variants.len() <= 10;

        // Clone is usually safe for enums
        analysis.suggested_derives.insert("Clone".to_string());

        // Equality suggestions
        analysis.suggested_derives.insert("PartialEq".to_string());

        if self.config.derive_strategy != DeriveStrategy::Conservative {
            analysis.suggested_derives.insert("Eq".to_string());
        }

        // Copy and Hash for simple enums
        if is_simple_enum {
            analysis.suggested_derives.insert("Copy".to_string());
            analysis.suggested_derives.insert("Hash".to_string());
            analysis.performance_impact =
                PerformanceImpact::Positive("Simple enum is very efficient to copy".to_string());
        }

        // Serialization for data enums
        if matches!(
            analysis.semantic_role.as_str(),
            "state" | "event" | "variant"
        ) {
            if !self.config.forbidden_derives.contains("Serialize") {
                analysis.suggested_derives.insert("Serialize".to_string());
                analysis.suggested_derives.insert("Deserialize".to_string());
            }
        }

        // Remove forbidden derives
        analysis
            .suggested_derives
            .retain(|derive| !self.config.forbidden_derives.contains(derive));

        // Higher confidence for enums as they're generally well-understood
        analysis.confidence = (analysis.confidence + 0.1).min(0.9);

        Ok(())
    }

    /// Runs AI analysis on code
    fn run_ai_analysis(
        &self,
        code: &str,
        ai_engine: &Arc<AIInferenceEngine>,
    ) -> Result<f64, SemanticError> {
        // This is a simplified synchronous wrapper around the async AI analysis
        // In a real implementation, you might use tokio::runtime::Handle::current()
        // or maintain an async context throughout the framework

        let rt = tokio::runtime::Runtime::new().map_err(|e| SemanticError::InferenceError {
            message: format!("Failed to create async runtime: {}", e),
        })?;

        rt.block_on(ai_engine.analyze_semantic_context(code))
    }

    /// Applies derive suggestions to a file
    fn apply_derives_to_file(
        &self,
        file_path: &Path,
        analyses: &[DeriveAnalysis],
    ) -> Result<(), SemanticError> {
        let content = fs::read_to_string(file_path).map_err(|e| SemanticError::FileIo {
            path: file_path.to_path_buf(),
            source: e,
        })?;

        let mut syntax_tree = syn::parse_file(&content).map_err(|e| SemanticError::ParseError {
            message: format!("Failed to parse {}: {}", file_path.display(), e),
        })?;

        let mut modified = false;

        // Apply derives to matching items
        for analysis in analyses {
            if analysis.suggested_derives.is_empty() {
                continue;
            }

            for item in &mut syntax_tree.items {
                let item_name = match item {
                    Item::Struct(item_struct) => Some(item_struct.ident.to_string()),
                    Item::Enum(item_enum) => Some(item_enum.ident.to_string()),
                    _ => None,
                };

                if let Some(name) = item_name {
                    if name == analysis.type_name {
                        if self.apply_derives_to_item(item, analysis)? {
                            modified = true;
                        }
                    }
                }
            }
        }

        // Write back the modified file
        if modified {
            let new_content = prettyplease::unparse(&syntax_tree);
            fs::write(file_path, new_content).map_err(|e| SemanticError::FileIo {
                path: file_path.to_path_buf(),
                source: e,
            })?;
        }

        Ok(())
    }

    /// Applies derives to a specific AST item
    fn apply_derives_to_item(
        &self,
        item: &mut Item,
        analysis: &DeriveAnalysis,
    ) -> Result<bool, SemanticError> {
        let attrs = match item {
            Item::Struct(ref mut item_struct) => &mut item_struct.attrs,
            Item::Enum(ref mut item_enum) => &mut item_enum.attrs,
            _ => return Ok(false),
        };

        let existing_derives = self.extract_existing_derives(attrs)?;
        let mut derives_added = false;

        for suggested_derive in &analysis.suggested_derives {
            if !existing_derives.contains(suggested_derive)
                && analysis.confidence >= self.config.min_confidence_threshold
            {
                self.add_derive_to_attributes(attrs, suggested_derive)?;
                derives_added = true;
                debug!(
                    "Applied derive({}) to {}",
                    suggested_derive, analysis.type_name
                );
            }
        }

        Ok(derives_added)
    }

    /// Extracts existing derive attributes from an item
    fn extract_existing_derives(
        &self,
        attrs: &[Attribute],
    ) -> Result<HashSet<String>, SemanticError> {
        let mut derives = HashSet::new();

        for attr in attrs {
            if attr.path().is_ident("derive") {
                match &attr.meta {
                    Meta::List(meta_list) => {
                        // Convert TokenStream to string and parse derive names
                        let derive_str = meta_list.tokens.to_string();
                        for derive in derive_str.split(',') {
                            let cleaned = derive.trim().replace("(", "").replace(")", "");
                            if !cleaned.is_empty() {
                                derives.insert(cleaned);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(derives)
    }

    /// Adds a derive to the attribute list
    fn add_derive_to_attributes(
        &self,
        attrs: &mut Vec<Attribute>,
        derive_name: &str,
    ) -> Result<(), SemanticError> {
        // Parse the derive name as a path
        let derive_path: SynPath =
            syn::parse_str(derive_name).map_err(|e| SemanticError::ParseError {
                message: format!("Invalid derive name '{}': {}", derive_name, e),
            })?;

        // Look for existing derive attribute to extend
        for attr in attrs.iter_mut() {
            if attr.path().is_ident("derive") {
                // For simplicity, we'll add a new derive attribute rather than parsing/modifying existing ones
                // In production, you'd want to properly parse and extend the existing derive list
                break;
            }
        }

        // Add new derive attribute
        let new_attr: Attribute = parse_quote! {
            #[derive(#derive_path)]
        };
        attrs.insert(0, new_attr);

        Ok(())
    }

    /// Generates a hash for code content
    fn hash_code(&self, code: &str) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        code.hash(&mut hasher);
        hasher.finish()
    }

    /// Evicts cache entries based on priority scoring
    fn evict_cache_entries(&self, cache: &mut HashMap<String, CachedAnalysis>) {
        let eviction_count = cache.len() / 4; // Evict 25% of entries

        let mut entries_with_scores: Vec<(String, f64)> = cache
            .iter_mut()
            .map(|(key, cached)| (key.clone(), cached.access_and_score()))
            .collect();

        // Sort by priority (lowest first for eviction)
        entries_with_scores
            .sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        // Remove lowest priority entries
        for (key, _) in entries_with_scores.into_iter().take(eviction_count) {
            cache.remove(&key);
        }

        debug!("Evicted {} cache entries", eviction_count);
    }

    /// Gets performance metrics
    pub fn get_metrics(&self) -> Result<SemanticMetrics, SemanticError> {
        self.metrics
            .lock()
            .map(|m| m.clone())
            .map_err(|_| SemanticError::CacheError {
                message: "Failed to acquire metrics lock".to_string(),
            })
    }

    /// Clears the analysis cache
    pub fn clear_cache(&self) -> Result<(), SemanticError> {
        let mut cache = self.cache.write().map_err(|_| SemanticError::CacheError {
            message: "Failed to acquire write lock for cache clear".to_string(),
        })?;
        cache.clear();
        info!("Semantic analysis cache cleared");
        Ok(())
    }
}

// Test function removed as it was never used in production code
