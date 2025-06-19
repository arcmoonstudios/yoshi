/* yoshi-deluxe/src/types.rs */
//! **Brief:** Core data structures and type definitions for yoshi-deluxe.
//!
//! This module contains all the fundamental data structures used throughout the
//! auto-correction system, including diagnostic information, AST context, correction
//! proposals, and system configuration types with comprehensive validation.

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt,
    path::PathBuf,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{Duration, SystemTime},
};

//--------------------------------------------------------------------------------------------------
// Compiler Diagnostic Types
//--------------------------------------------------------------------------------------------------

/// Comprehensive representation of a compiler diagnostic with enhanced metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerDiagnostic {
    /// Unique diagnostic identifier for tracking
    pub id: String,
    /// Error message content
    pub message: String,
    /// Error code (e.g., "E0599")
    pub code: Option<String>,
    /// Severity level
    pub level: DiagnosticLevel,
    /// File location information with precise mapping
    pub spans: Vec<DiagnosticSpan>,
    /// Child diagnostics with suggestions
    pub children: Vec<CompilerDiagnostic>,
    /// Suggested replacements from compiler
    pub suggested_replacement: Option<String>,
    /// Machine-applicable signpost from clippy --fix for autonomous corrections
    pub machine_applicable_signpost: Option<String>,
    /// Additional metadata for correction context
    pub metadata: HashMap<String, String>,
    /// Diagnostic creation timestamp
    pub created_at: SystemTime,
    /// Whether this diagnostic has been processed
    pub processed: bool,
}

impl CompilerDiagnostic {
    /// Create a new diagnostic with basic information
    #[must_use]
    pub fn new(id: impl Into<String>, message: impl Into<String>, level: DiagnosticLevel) -> Self {
        Self {
            id: id.into(),
            message: message.into(),
            code: None,
            level,
            spans: Vec::new(),
            children: Vec::new(),
            suggested_replacement: None,
            machine_applicable_signpost: None,
            metadata: HashMap::new(),
            created_at: SystemTime::now(),
            processed: false,
        }
    }

    /// Get the primary span for this diagnostic
    #[must_use]
    pub fn primary_span(&self) -> Option<&DiagnosticSpan> {
        self.spans
            .iter()
            .find(|span| span.is_primary)
            .or_else(|| self.spans.first())
    }

    /// Check if this diagnostic represents an error
    #[must_use]
    pub const fn is_error(&self) -> bool {
        matches!(self.level, DiagnosticLevel::Error)
    }

    /// Get a short description for this diagnostic
    #[must_use]
    pub fn short_description(&self) -> String {
        format!(
            "{}: {}",
            self.level,
            self.message.chars().take(100).collect::<String>()
        )
    }

    /// Add metadata to the diagnostic
    pub fn add_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }

    /// Mark diagnostic as processed
    pub fn mark_processed(&mut self) {
        self.processed = true;
    }
}

/// Enhanced diagnostic severity levels with priority scoring
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticLevel {
    /// Critical errors that prevent compilation
    Error,
    /// Warnings that should be addressed
    Warning,
    /// Informational notes
    Note,
    /// Help suggestions
    Help,
}

impl DiagnosticLevel {
    /// Get numeric priority for this level
    #[must_use]
    pub const fn priority(&self) -> u8 {
        match self {
            Self::Error => 255,
            Self::Warning => 128,
            Self::Note => 64,
            Self::Help => 32,
        }
    }
}

impl fmt::Display for DiagnosticLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => write!(f, "error"),
            Self::Warning => write!(f, "warning"),
            Self::Note => write!(f, "note"),
            Self::Help => write!(f, "help"),
        }
    }
}

/// Precise source code location with enhanced mapping capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticSpan {
    /// Source file path (canonicalized)
    pub file_name: PathBuf,
    /// Byte-level start position (0-indexed)
    pub byte_start: usize,
    /// Byte-level end position (0-indexed)
    pub byte_end: usize,
    /// Line number (1-indexed)
    pub line_start: usize,
    /// Line number (1-indexed)
    pub line_end: usize,
    /// Column number (1-indexed)
    pub column_start: usize,
    /// Column number (1-indexed)
    pub column_end: usize,
    /// Actual text content
    pub text: String,
    /// Primary span indicator
    pub is_primary: bool,
    /// Span label if available
    pub label: Option<String>,
    /// Suggested replacement text for this span
    pub suggested_replacement: Option<String>,
    /// Expansion information for macro spans
    pub expansion: Option<Box<DiagnosticSpan>>,
}

impl DiagnosticSpan {
    /// Create a new diagnostic span
    #[must_use]
    pub fn new(
        file_name: PathBuf,
        byte_start: usize,
        byte_end: usize,
        line_start: usize,
        line_end: usize,
        column_start: usize,
        column_end: usize,
        text: String,
    ) -> Self {
        Self {
            file_name,
            byte_start,
            byte_end,
            line_start,
            line_end,
            column_start,
            column_end,
            text,
            is_primary: false,
            label: None,
            suggested_replacement: None,
            expansion: None,
        }
    }

    /// Calculate byte length of this span
    #[must_use]
    pub fn byte_length(&self) -> usize {
        self.byte_end.saturating_sub(self.byte_start)
    }

    /// Check if span contains a byte offset
    #[must_use]
    pub fn contains_byte_offset(&self, offset: usize) -> bool {
        offset >= self.byte_start && offset <= self.byte_end
    }

    /// Get line/column range as tuple
    #[must_use]
    pub fn line_column_range(&self) -> ((usize, usize), (usize, usize)) {
        (
            (self.line_start, self.column_start),
            (self.line_end, self.column_end),
        )
    }

    /// Check if this span represents a single line
    #[must_use]
    pub const fn is_single_line(&self) -> bool {
        self.line_start == self.line_end
    }

    /// Get a display string for this span's location
    #[must_use]
    pub fn location_display(&self) -> String {
        if self.is_single_line() {
            format!(
                "{}:{}:{}",
                self.file_name.display(),
                self.line_start,
                self.column_start
            )
        } else {
            format!(
                "{}:{}:{}-{}:{}",
                self.file_name.display(),
                self.line_start,
                self.column_start,
                self.line_end,
                self.column_end
            )
        }
    }

    /// Mark this span as primary
    pub fn mark_primary(&mut self) {
        self.is_primary = true;
    }

    /// Set the label for this span
    pub fn set_label(&mut self, label: impl Into<String>) {
        self.label = Some(label.into());
    }
}

//--------------------------------------------------------------------------------------------------
// Documentation and API Types
//--------------------------------------------------------------------------------------------------

/// Cached documentation data with intelligent expiration and versioning
#[derive(Debug, Clone)]
pub struct CachedDocsData {
    /// Cache format version for compatibility
    pub version: u32,
    /// Target crate information
    pub crate_info: CrateInfo,
    /// API method signatures with enhanced metadata
    pub methods: Vec<MethodSignature>,
    /// Implementation details with trait mappings
    pub implementations: Vec<TraitImplementation>,
    /// Usage examples with context
    pub examples: Vec<CodeExample>,
    /// Cache creation timestamp
    pub cached_at: SystemTime,
    /// Cache access count for LRU eviction
    pub access_count: Arc<AtomicU64>,
    /// Data source for provenance tracking
    pub source: DataSource,
}

impl CachedDocsData {
    /// Create new cached docs data
    #[must_use]
    pub fn new(
        crate_info: CrateInfo,
        methods: Vec<MethodSignature>,
        implementations: Vec<TraitImplementation>,
        examples: Vec<CodeExample>,
        source: DataSource,
    ) -> Self {
        Self {
            version: 1,
            crate_info,
            methods,
            implementations,
            examples,
            cached_at: SystemTime::now(),
            access_count: Arc::new(AtomicU64::new(1)),
            source,
        }
    }

    /// Check if cache entry is still valid
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.cached_at.elapsed().unwrap_or(Duration::MAX) < crate::constants::DOCS_CACHE_EXPIRY
            && self.version >= 1
    }

    /// Update access count for LRU tracking
    pub fn touch(&self) {
        self.access_count.fetch_add(1, Ordering::Relaxed);
    }

    /// Get current access count
    #[must_use]
    pub fn access_count(&self) -> u64 {
        self.access_count.load(Ordering::Relaxed)
    }
}

/// Crate information for documentation context
#[derive(Debug, Clone)]
pub struct CrateInfo {
    /// Crate name
    pub name: String,
    /// Crate version
    pub version: String,
    /// Documentation URL
    pub docs_url: String,
    /// Repository URL if available
    pub repository: Option<String>,
    /// Crate description
    pub description: Option<String>,
    /// License information
    pub license: Option<String>,
}

impl CrateInfo {
    /// Create new crate info
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        docs_url: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            docs_url: docs_url.into(),
            repository: None,
            description: None,
            license: None,
        }
    }
}

/// Data source enumeration for provenance
#[derive(Debug, Clone)]
pub enum DataSource {
    /// docs.rs HTML scraping
    DocsRs {
        /// The URL that was scraped
        url: String,
    },
    /// Structured API data
    StructuredApi {
        /// The API endpoint that was queried
        endpoint: String,
    },
    /// Local analysis
    LocalAnalysis,
    /// Cached from previous source
    Cached {
        /// The original data source
        original_source: Box<DataSource>,
    },
}

/// Method signature with comprehensive metadata and validation
#[derive(Debug, Clone)]
pub struct MethodSignature {
    /// Method name
    pub name: String,
    /// Parameter types and names with defaults
    pub parameters: Vec<Parameter>,
    /// Return type with full path
    pub return_type: Option<String>,
    /// Documentation string (cleaned)
    pub documentation: String,
    /// Visibility modifier
    pub visibility: String,
    /// Method attributes (async, const, etc.)
    pub attributes: Vec<String>,
    /// Generic parameters if any
    pub generics: Vec<String>,
    /// Where clause constraints
    pub where_clause: Option<String>,
    /// Deprecation information
    pub deprecation: Option<DeprecationInfo>,
    /// Stability attributes
    pub stability: StabilityInfo,
}

impl MethodSignature {
    /// Create a new method signature
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            parameters: Vec::new(),
            return_type: None,
            documentation: String::new(),
            visibility: "pub".to_string(),
            attributes: Vec::new(),
            generics: Vec::new(),
            where_clause: None,
            deprecation: None,
            stability: StabilityInfo::default(),
        }
    }

    /// Generate a canonical signature string for comparison
    #[must_use]
    pub fn canonical_signature(&self) -> String {
        let params = self
            .parameters
            .iter()
            .map(|p| format!("{}: {}", p.name, p.param_type))
            .collect::<Vec<_>>()
            .join(", ");

        let return_part = self
            .return_type
            .as_ref()
            .map_or_else(String::new, |rt| format!(" -> {rt}"));

        format!("{}({}){}", self.name, params, return_part)
    }

    /// Check if method matches a search pattern
    #[must_use]
    pub fn matches_pattern(&self, pattern: &str) -> bool {
        self.name.contains(pattern)
            || self
                .documentation
                .to_lowercase()
                .contains(&pattern.to_lowercase())
    }

    /// Calculate complexity score for this method
    #[must_use]
    pub fn complexity_score(&self) -> u8 {
        let mut score = 1;
        score += self.parameters.len().min(10) as u8; // Max 10 points for parameters
        if self.return_type.is_some() {
            score += 1;
        }
        if !self.generics.is_empty() {
            score += 2;
        }
        if self.where_clause.is_some() {
            score += 1;
        }
        score.min(15) // Cap at 15
    }

    /// Add a parameter to this method
    pub fn add_parameter(&mut self, parameter: Parameter) {
        self.parameters.push(parameter);
    }

    /// Set the return type
    pub fn set_return_type(&mut self, return_type: impl Into<String>) {
        self.return_type = Some(return_type.into());
    }
}

/// Function parameter with enhanced type information
#[derive(Debug, Clone)]
pub struct Parameter {
    /// Parameter name
    pub name: String,
    /// Parameter type with full path
    pub param_type: String,
    /// Default value if any
    pub default_value: Option<String>,
    /// Whether parameter is mutable
    pub is_mutable: bool,
    /// Parameter attributes
    pub attributes: Vec<String>,
}

impl Parameter {
    /// Create a new parameter
    #[must_use]
    pub fn new(name: impl Into<String>, param_type: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            param_type: param_type.into(),
            default_value: None,
            is_mutable: false,
            attributes: Vec::new(),
        }
    }

    /// Mark parameter as mutable
    pub fn mark_mutable(&mut self) {
        self.is_mutable = true;
    }

    /// Set default value
    pub fn set_default(&mut self, value: impl Into<String>) {
        self.default_value = Some(value.into());
    }
}

/// Deprecation information
#[derive(Debug, Clone)]
pub struct DeprecationInfo {
    /// Deprecation reason
    pub reason: String,
    /// Suggested alternative
    pub alternative: Option<String>,
    /// Version when deprecated
    pub since: Option<String>,
}

impl DeprecationInfo {
    /// Create new deprecation info
    #[must_use]
    pub fn new(reason: impl Into<String>) -> Self {
        Self {
            reason: reason.into(),
            alternative: None,
            since: None,
        }
    }
}

/// Stability information
#[derive(Debug, Clone)]
pub struct StabilityInfo {
    /// Stability level
    pub level: StabilityLevel,
    /// Version when stabilized
    pub since: Option<String>,
    /// Feature gate if unstable
    pub feature: Option<String>,
}

impl Default for StabilityInfo {
    fn default() -> Self {
        Self {
            level: StabilityLevel::Stable,
            since: None,
            feature: None,
        }
    }
}

/// API stability levels
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StabilityLevel {
    /// Stable API
    Stable,
    /// Unstable/experimental API
    Unstable,
    /// Internal API
    Internal,
}

/// Trait implementation details with enhanced metadata
#[derive(Debug, Clone)]
pub struct TraitImplementation {
    /// Trait name with full path
    pub trait_name: String,
    /// Type implementing the trait with full path
    pub implementing_type: String,
    /// Available methods from this implementation
    pub methods: Vec<String>,
    /// Generic parameters
    pub generics: Vec<String>,
    /// Where clause constraints
    pub where_clause: Option<String>,
    /// Implementation attributes
    pub attributes: Vec<String>,
}

impl TraitImplementation {
    /// Create new trait implementation
    #[must_use]
    pub fn new(trait_name: impl Into<String>, implementing_type: impl Into<String>) -> Self {
        Self {
            trait_name: trait_name.into(),
            implementing_type: implementing_type.into(),
            methods: Vec::new(),
            generics: Vec::new(),
            where_clause: None,
            attributes: Vec::new(),
        }
    }

    /// Add a method to this implementation
    pub fn add_method(&mut self, method: impl Into<String>) {
        self.methods.push(method.into());
    }
}

/// Code example with enhanced context and validation
#[derive(Debug, Clone)]
pub struct CodeExample {
    /// Example code content
    pub code: String,
    /// Context description
    pub description: String,
    /// Complexity rating (1-5)
    pub complexity: u8,
    /// Whether example compiles
    pub compiles: Option<bool>,
    /// Required features for this example
    pub required_features: Vec<String>,
    /// Minimum Rust version required
    pub min_rust_version: Option<String>,
}

impl CodeExample {
    /// Create new code example
    #[must_use]
    pub fn new(code: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            description: description.into(),
            complexity: 3,
            compiles: None,
            required_features: Vec::new(),
            min_rust_version: None,
        }
    }

    /// Set complexity level
    pub fn set_complexity(&mut self, complexity: u8) {
        self.complexity = complexity.min(5);
    }

    /// Mark as compiling or not
    pub fn set_compiles(&mut self, compiles: bool) {
        self.compiles = Some(compiles);
    }
}

//--------------------------------------------------------------------------------------------------
// Correction Types and Strategies
//--------------------------------------------------------------------------------------------------

/// Comprehensive correction proposal with safety metadata
#[derive(Debug, Clone)]
pub struct CorrectionProposal {
    /// Original problematic code
    pub original_code: String,
    /// Suggested corrected code
    pub corrected_code: String,
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
    /// Correction strategy used
    pub strategy: CorrectionStrategy,
    /// Supporting documentation
    pub documentation_source: Option<String>,
    /// Additional context metadata
    pub context_metadata: HashMap<String, String>,
    /// Byte range for precise application
    pub byte_range: (usize, usize),
    /// Safety level of this correction
    pub safety_level: SafetyLevel,
}

impl CorrectionProposal {
    /// Create a new correction proposal
    #[must_use]
    pub fn new(
        original_code: impl Into<String>,
        corrected_code: impl Into<String>,
        confidence: f64,
        strategy: CorrectionStrategy,
    ) -> Self {
        Self {
            original_code: original_code.into(),
            corrected_code: corrected_code.into(),
            confidence,
            strategy,
            documentation_source: None,
            context_metadata: HashMap::new(),
            byte_range: (0, 0),
            safety_level: SafetyLevel::RequiresReview,
        }
    }

    /// Check if this proposal is considered safe for automatic application
    #[must_use]
    pub const fn is_auto_applicable(&self) -> bool {
        matches!(self.safety_level, SafetyLevel::Safe) && self.confidence > 0.9
    }

    /// Get a description of the correction strategy
    #[must_use]
    pub fn strategy_description(&self) -> String {
        match &self.strategy {
            CorrectionStrategy::MethodNameCorrection { similarity_score } => {
                format!(
                    "Method name correction (similarity: {:.2})",
                    similarity_score
                )
            }
            CorrectionStrategy::TypeConversion {
                from_type, to_type, ..
            } => {
                format!("Type conversion from {from_type} to {to_type}")
            }
            CorrectionStrategy::ImportAddition { import_path } => {
                format!("Add import: {import_path}")
            }
            CorrectionStrategy::TraitImport {
                trait_name,
                method_name,
            } => {
                format!("Import trait {trait_name} for method {method_name}")
            }
            CorrectionStrategy::Generic { description } => description.clone(),
            _ => "Code correction".to_string(),
        }
    }

    /// Add metadata to the proposal
    pub fn add_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.context_metadata.insert(key.into(), value.into());
    }

    /// Set the safety level
    pub fn set_safety_level(&mut self, level: SafetyLevel) {
        self.safety_level = level;
    }

    /// Set byte range for application
    pub fn set_byte_range(&mut self, start: usize, end: usize) {
        self.byte_range = (start, end);
    }
}

/// Enhanced correction strategies with comprehensive classification
#[derive(Debug, Clone)]
pub enum CorrectionStrategy {
    /// Method name correction with similarity metrics
    MethodNameCorrection {
        /// Similarity score
        similarity_score: f64,
    },
    /// Type conversion with method specification
    TypeConversion {
        /// Original type
        from_type: String,
        /// Target type
        to_type: String,
        /// Method used for conversion
        conversion_method: String,
    },
    /// Reference/dereference operation
    ReferenceCorrection {
        /// The operation performed
        operation: String,
    },
    /// Numeric conversion with safety info
    NumericConversion {
        /// Original type
        from_type: String,
        /// Target type
        to_type: String,
        /// Method used for conversion
        method: String,
    },
    /// Import addition
    ImportAddition {
        /// Path of the import to add
        import_path: String,
    },
    /// Trait import for method access
    TraitImport {
        /// Name of the trait to import
        trait_name: String,
        /// Name of the method enabled by the trait
        method_name: String,
    },
    /// Trait implementation generation
    TraitImplementation {
        /// Name of the trait
        trait_name: String,
        /// The type implementing the trait
        implementing_type: String,
    },
    /// Argument count correction
    ArgumentCorrection {
        /// Number of expected arguments
        expected_count: usize,
        /// Number of provided arguments
        provided_count: usize,
    },
    /// Struct field operation
    StructFieldCorrection {
        /// Name of the field being corrected
        field_name: String,
        /// Name of the struct
        struct_name: String,
        /// The operation performed
        operation: String,
    },
    /// Field access correction
    FieldAccessCorrection {
        /// The original field name
        original_field: String,
        /// The suggested field name
        suggested_field: String,
        /// The name of the type containing the field
        type_name: String,
    },
    /// Borrowing and lifetime correction
    BorrowingCorrection {
        /// The operation performed
        operation: String,
    },
    /// Visibility modifier correction
    VisibilityCorrection {
        /// The operation performed
        operation: String,
    },
    /// Macro usage correction
    MacroCorrection {
        /// Name of the macro
        macro_name: String,
        /// Type of correction applied
        correction_type: String,
    },
    /// Generic correction with description
    Generic {
        /// A description of the correction
        description: String,
    },
}

/// Safety level classification for corrections
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SafetyLevel {
    /// Safe to apply automatically
    Safe,
    /// Requires manual review before application
    RequiresReview,
    /// Potentially unsafe, should not be auto-applied
    Unsafe,
}

impl fmt::Display for SafetyLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Safe => write!(f, "safe"),
            Self::RequiresReview => write!(f, "requires review"),
            Self::Unsafe => write!(f, "unsafe"),
        }
    }
}

/// Suggestion for a field access correction
#[derive(Debug, Clone)]
pub struct FieldSuggestion {
    /// Suggested field name
    pub name: String,
    /// Confidence in this suggestion
    pub confidence: f64,
    /// Description of the suggestion
    pub description: String,
}

impl FieldSuggestion {
    /// Create new field suggestion
    #[must_use]
    pub fn new(name: impl Into<String>, confidence: f64, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            confidence,
            description: description.into(),
        }
    }
}

/// Enhanced method suggestion with comprehensive metadata
#[derive(Debug, Clone)]
pub struct MethodSuggestion {
    /// Suggested method name
    pub method_name: String,
    /// Similarity score (0.0-1.0)
    pub similarity_score: f64,
    /// Method signature
    pub signature: String,
    /// Method documentation
    pub documentation: String,
    /// Deprecation information if applicable
    pub deprecation: Option<DeprecationInfo>,
    /// Stability information
    pub stability: StabilityInfo,
}

impl MethodSuggestion {
    /// Create new method suggestion
    #[must_use]
    pub fn new(
        method_name: impl Into<String>,
        similarity_score: f64,
        signature: impl Into<String>,
        documentation: impl Into<String>,
    ) -> Self {
        Self {
            method_name: method_name.into(),
            similarity_score,
            signature: signature.into(),
            documentation: documentation.into(),
            deprecation: None,
            stability: StabilityInfo::default(),
        }
    }
}

//--------------------------------------------------------------------------------------------------
// System Results and Tracking
//--------------------------------------------------------------------------------------------------

/// Complete correction information for a project file
#[derive(Debug, Clone)]
pub struct ProjectCorrection {
    /// File path that needs correction
    pub file_path: PathBuf,
    /// Original diagnostic that triggered the correction
    pub diagnostic: CompilerDiagnostic,
    /// Generated correction proposals
    pub proposals: Vec<CorrectionProposal>,
    /// Creation timestamp
    pub created_at: SystemTime,
    /// Whether correction has been applied
    pub applied: bool,
}

impl ProjectCorrection {
    /// Create new project correction
    #[must_use]
    pub fn new(file_path: PathBuf, diagnostic: CompilerDiagnostic) -> Self {
        Self {
            file_path,
            diagnostic,
            proposals: Vec::new(),
            created_at: SystemTime::now(),
            applied: false,
        }
    }

    /// Get the best (highest confidence) proposal
    #[must_use]
    pub fn best_proposal(&self) -> Option<&CorrectionProposal> {
        self.proposals.first()
    }

    /// Check if this correction has any auto-applicable proposals
    #[must_use]
    pub fn has_auto_applicable_proposals(&self) -> bool {
        self.proposals.iter().any(|p| p.is_auto_applicable())
    }

    /// Get a summary of this correction
    #[must_use]
    pub fn summary(&self) -> String {
        format!(
            "{}: {} ({} proposals)",
            self.file_path.display(),
            self.diagnostic.short_description(),
            self.proposals.len()
        )
    }

    /// Add a proposal to this correction
    pub fn add_proposal(&mut self, proposal: CorrectionProposal) {
        self.proposals.push(proposal);
        // Keep proposals sorted by confidence
        self.proposals.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    /// Mark as applied
    pub fn mark_applied(&mut self) {
        self.applied = true;
    }
}

/// Record of an applied correction with backup information
#[derive(Debug, Clone)]
pub struct AppliedCorrection {
    /// File path that was corrected
    pub file_path: PathBuf,
    /// Original problematic code
    pub original_code: String,
    /// Applied corrected code
    pub corrected_code: String,
    /// Strategy used for the correction
    pub strategy: CorrectionStrategy,
    /// Application timestamp
    pub applied_at: SystemTime,
    /// Backup file path
    pub backup_path: Option<PathBuf>,
}

impl AppliedCorrection {
    /// Create new applied correction
    #[must_use]
    pub fn new(
        file_path: PathBuf,
        original_code: String,
        corrected_code: String,
        strategy: CorrectionStrategy,
    ) -> Self {
        Self {
            file_path,
            original_code,
            corrected_code,
            strategy,
            applied_at: SystemTime::now(),
            backup_path: None,
        }
    }

    /// Check if this correction can be reverted (has backup)
    #[must_use]
    pub fn can_revert(&self) -> bool {
        self.backup_path.as_ref().map_or(false, |p| p.exists())
    }

    /// Get a summary of this applied correction
    #[must_use]
    pub fn summary(&self) -> String {
        format!(
            "{}: {} -> {}",
            self.file_path.display(),
            self.original_code.chars().take(50).collect::<String>(),
            self.corrected_code.chars().take(50).collect::<String>()
        )
    }

    /// Set backup path
    pub fn set_backup_path(&mut self, path: PathBuf) {
        self.backup_path = Some(path);
    }
}

//--------------------------------------------------------------------------------------------------
// Configuration Types
//--------------------------------------------------------------------------------------------------

/// Enhanced system configuration with production settings
#[derive(Debug, Clone)]
pub struct SystemConfig {
    /// Maximum correction proposals per diagnostic
    pub max_proposals_per_diagnostic: usize,
    /// Minimum confidence threshold for proposals
    pub min_confidence_threshold: f64,
    /// Enable parallel processing
    pub enable_parallel_processing: bool,
    /// Cache size limits
    pub max_cache_size: usize,
    /// Documentation scraping enabled
    pub enable_docs_scraping: bool,
    /// Maximum concurrent operations
    pub max_concurrent_operations: usize,
    /// Safety level filter
    pub min_safety_level: SafetyLevel,
    /// Enable metrics collection
    pub enable_metrics: bool,
    /// Auto-apply safe corrections
    pub auto_apply_safe_corrections: bool,
    /// Create backup files
    pub create_backup_files: bool,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            max_proposals_per_diagnostic: 3,
            min_confidence_threshold: 0.6,
            enable_parallel_processing: true,
            max_cache_size: 500,
            enable_docs_scraping: true,
            max_concurrent_operations: 6,
            min_safety_level: SafetyLevel::RequiresReview,
            enable_metrics: true,
            auto_apply_safe_corrections: false,
            create_backup_files: true,
        }
    }
}

impl SystemConfig {
    /// Create a new system configuration with validation
    pub fn new() -> crate::Hatch<Self> {
        let config = Self::default();
        config.validate()?;
        Ok(config)
    }

    /// Validate configuration parameters
    pub fn validate(&self) -> crate::Hatch<()> {
        use crate::err::YoshiACE;

        if self.max_proposals_per_diagnostic == 0 {
            return Err(YoshiACE::Configuration {
                _parameter: "max_proposals_per_diagnostic".to_string(),
                _value: "0".to_string(),
            }
            .into());
        }

        if !(0.0..=1.0).contains(&self.min_confidence_threshold) {
            return Err(YoshiACE::Configuration {
                _parameter: "min_confidence_threshold".to_string(),
                _value: self.min_confidence_threshold.to_string(),
            }
            .into());
        }

        if self.max_concurrent_operations == 0 {
            return Err(YoshiACE::Configuration {
                _parameter: "max_concurrent_operations".to_string(),
                _value: "0".to_string(),
            }
            .into());
        }

        Ok(())
    }

    /// Create a high-performance configuration
    #[must_use]
    pub fn high_performance() -> Self {
        Self {
            max_proposals_per_diagnostic: 10,
            min_confidence_threshold: 0.5,
            enable_parallel_processing: true,
            max_cache_size: 2000,
            enable_docs_scraping: true,
            max_concurrent_operations: 16,
            min_safety_level: SafetyLevel::RequiresReview,
            enable_metrics: true,
            auto_apply_safe_corrections: false,
            create_backup_files: true,
        }
    }

    /// Create a conservative configuration
    #[must_use]
    pub fn conservative() -> Self {
        Self {
            max_proposals_per_diagnostic: 1,
            min_confidence_threshold: 0.9,
            enable_parallel_processing: false,
            max_cache_size: 100,
            enable_docs_scraping: false,
            max_concurrent_operations: 1,
            min_safety_level: SafetyLevel::Safe,
            enable_metrics: false,
            auto_apply_safe_corrections: false,
            create_backup_files: true,
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Performance and Optimization Types
//--------------------------------------------------------------------------------------------------

/// **Performance Impact Classification**
///
/// Categorizes the performance impact level of optimizations and corrections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PerformanceImpact {
    /// High performance impact - significant improvement expected
    High,
    /// Medium performance impact - moderate improvement expected
    Medium,
    /// Low performance impact - minimal improvement expected
    Low,
}

impl PerformanceImpact {
    /// Get the impact level as a numeric score (0.0 to 1.0)
    #[must_use]
    pub fn score(&self) -> f64 {
        match self {
            Self::High => 1.0,
            Self::Medium => 0.6,
            Self::Low => 0.3,
        }
    }

    /// Get the impact level as a descriptive string
    #[must_use]
    pub fn description(&self) -> &'static str {
        match self {
            Self::High => "High performance impact",
            Self::Medium => "Medium performance impact",
            Self::Low => "Low performance impact",
        }
    }
}

impl std::fmt::Display for PerformanceImpact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

/// **Optimization Opportunity**
///
/// Represents a specific optimization opportunity detected in code.
#[derive(Debug, Clone)]
pub struct OptimizationOpportunity {
    /// Description of the optimization opportunity
    pub description: String,
    /// Location in the code where the optimization can be applied
    pub location: CodeLocation,
    /// Expected performance impact of applying this optimization
    pub performance_impact: PerformanceImpact,
    /// Confidence level of the optimization (0.0 to 1.0)
    pub confidence: f64,
    /// Type of optimization (e.g., "vec_allocation", "error_handling")
    pub optimization_type: String,
    /// Suggested code replacement
    pub suggested_fix: Option<String>,
}

/// **Code Location**
///
/// Represents a specific location in source code.
#[derive(Debug, Clone)]
pub struct CodeLocation {
    /// Line number (1-based)
    pub line: usize,
    /// Column number (1-based)
    pub column: usize,
    /// Byte offset in the file
    pub byte_offset: Option<usize>,
    /// Length of the code span
    pub length: Option<usize>,
}

/// **Optimization Engine**
///
/// Core engine for detecting and applying code optimizations.
#[derive(Debug, Clone)]
pub struct OptimizationEngine {
    /// Engine configuration
    config: OptimizationConfig,
    /// Cache of previously analyzed patterns
    pattern_cache: std::collections::HashMap<String, Vec<OptimizationOpportunity>>,
}

impl OptimizationEngine {
    /// Create a new optimization engine with default configuration
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: OptimizationConfig::default(),
            pattern_cache: std::collections::HashMap::new(),
        }
    }

    /// Create a new optimization engine with custom configuration
    #[must_use]
    pub fn with_config(config: OptimizationConfig) -> Self {
        Self {
            config,
            pattern_cache: std::collections::HashMap::new(),
        }
    }

    /// Detect optimization opportunities in the given code
    #[must_use]
    pub fn detect_optimization_opportunities(&self, code: &str) -> Vec<OptimizationOpportunity> {
        let mut opportunities = Vec::new();

        // Check for Vec allocation patterns
        if code.contains("Vec::new()") && code.contains(".push(") {
            let push_count = code.matches(".push(").count();
            if push_count > 3 {
                opportunities.push(OptimizationOpportunity {
                    description: format!(
                        "Consider using Vec::with_capacity({}) to pre-allocate",
                        push_count
                    ),
                    location: CodeLocation {
                        line: 1, // Simplified - would need proper parsing
                        column: 1,
                        byte_offset: None,
                        length: None,
                    },
                    performance_impact: if push_count > 10 {
                        PerformanceImpact::High
                    } else {
                        PerformanceImpact::Medium
                    },
                    confidence: 0.8,
                    optimization_type: "vec_allocation".to_string(),
                    suggested_fix: Some(format!("Vec::with_capacity({})", push_count)),
                });
            }
        }

        // Check for unwrap() usage
        if code.contains(".unwrap()") {
            opportunities.push(OptimizationOpportunity {
                description: "Replace .unwrap() with proper error handling".to_string(),
                location: CodeLocation {
                    line: 1, // Simplified - would need proper parsing
                    column: 1,
                    byte_offset: None,
                    length: None,
                },
                performance_impact: PerformanceImpact::Low,
                confidence: 0.9,
                optimization_type: "error_handling".to_string(),
                suggested_fix: Some("Use .map_err() or ? operator".to_string()),
            });
        }

        // Check for string concatenation in loops
        if code.contains("for ") && code.contains(" + ") && code.contains("String") {
            opportunities.push(OptimizationOpportunity {
                description: "Consider using String::with_capacity() or format! macro".to_string(),
                location: CodeLocation {
                    line: 1, // Simplified - would need proper parsing
                    column: 1,
                    byte_offset: None,
                    length: None,
                },
                performance_impact: PerformanceImpact::High,
                confidence: 0.7,
                optimization_type: "string_allocation".to_string(),
                suggested_fix: Some("Use String::with_capacity() or collect()".to_string()),
            });
        }

        opportunities
    }

    /// Apply optimizations to the given code
    ///
    /// # Errors
    ///
    /// Returns an error if optimization application fails
    pub fn apply_optimizations(
        &self,
        code: &str,
        opportunities: &[OptimizationOpportunity],
    ) -> Result<String, String> {
        let mut optimized_code = code.to_string();

        for opportunity in opportunities {
            match opportunity.optimization_type.as_str() {
                "vec_allocation" => {
                    if let Some(fix) = &opportunity.suggested_fix {
                        optimized_code = optimized_code.replace("Vec::new()", fix);
                    }
                }
                "error_handling" => {
                    // For demo purposes, just add a comment
                    optimized_code = optimized_code
                        .replace(".unwrap()", ".expect(\"TODO: Handle this error properly\")");
                }
                "string_allocation" => {
                    // For demo purposes, just add a comment
                    if optimized_code.contains("String::new()") {
                        optimized_code =
                            optimized_code.replace("String::new()", "String::with_capacity(64)");
                    }
                }
                _ => {
                    // Unknown optimization type - skip
                }
            }
        }

        Ok(optimized_code)
    }

    /// Get engine configuration
    #[must_use]
    pub fn config(&self) -> &OptimizationConfig {
        &self.config
    }
}

impl Default for OptimizationEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// **Optimization Engine Configuration**
///
/// Configuration options for the optimization engine.
#[derive(Debug, Clone)]
pub struct OptimizationConfig {
    /// Enable aggressive optimizations that may change semantics
    pub aggressive_mode: bool,
    /// Maximum number of optimization passes
    pub max_passes: usize,
    /// Minimum confidence threshold for applying optimizations
    pub confidence_threshold: f64,
    /// Enable experimental optimizations
    pub experimental: bool,
    /// Maximum number of opportunities to detect per file
    pub max_opportunities: usize,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            aggressive_mode: false,
            max_passes: 3,
            confidence_threshold: 0.7,
            experimental: false,
            max_opportunities: 50,
        }
    }
}

/// **Clippy Strategy Trait**
///
/// Trait for implementing different clippy lint strategies.
pub trait ClippyStrategy {
    /// Get the name of the lint this strategy handles
    fn lint_name(&self) -> &str;

    /// Get a description of what this strategy does
    fn description(&self) -> &str;

    /// Check if this strategy can handle the given lint
    fn can_handle(&self, lint_name: &str) -> bool;

    /// Apply the strategy to fix the lint in the given code
    ///
    /// # Errors
    ///
    /// Returns an error if the strategy cannot be applied
    fn apply(&self, code: &str, lint_info: &LintInfo) -> Result<String, String>;

    /// Get the confidence level of this strategy (0.0 to 1.0)
    fn confidence(&self) -> f64;
}

/// **Lint Information**
///
/// Information about a specific lint violation.
#[derive(Debug, Clone)]
pub struct LintInfo {
    /// Name of the lint (e.g., "unused_variables")
    pub lint_name: String,
    /// Lint message
    pub message: String,
    /// Location of the lint violation
    pub location: CodeLocation,
    /// Suggested fix from clippy (if available)
    pub suggested_fix: Option<String>,
    /// Severity level
    pub severity: LintSeverity,
}

/// **Lint Severity**
///
/// Severity levels for lint violations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LintSeverity {
    /// Error - must be fixed
    Error,
    /// Warning - should be fixed
    Warning,
    /// Note - informational
    Note,
}

impl std::fmt::Display for LintSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error => write!(f, "error"),
            Self::Warning => write!(f, "warning"),
            Self::Note => write!(f, "note"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiler_diagnostic_creation() {
        let diagnostic = CompilerDiagnostic::new("test_id", "test message", DiagnosticLevel::Error);

        assert_eq!(diagnostic.id, "test_id");
        assert_eq!(diagnostic.message, "test message");
        assert!(diagnostic.is_error());
        assert!(!diagnostic.processed);
    }

    #[test]
    fn test_diagnostic_span_operations() {
        let mut span = DiagnosticSpan::new(
            PathBuf::from("test.rs"),
            10,
            20,
            1,
            1,
            10,
            20,
            "test_code".to_string(),
        );

        assert_eq!(span.byte_length(), 10);
        assert!(span.contains_byte_offset(15));
        assert!(!span.contains_byte_offset(25));
        assert!(span.is_single_line());

        span.mark_primary();
        assert!(span.is_primary);
    }

    #[test]
    fn test_correction_proposal() {
        let mut proposal = CorrectionProposal::new(
            "old_code",
            "new_code",
            0.95,
            CorrectionStrategy::Generic {
                description: "test".to_string(),
            },
        );

        proposal.set_safety_level(SafetyLevel::Safe);
        assert!(proposal.is_auto_applicable());

        proposal.add_metadata("test_key", "test_value");
        assert!(proposal.context_metadata.contains_key("test_key"));
    }

    #[test]
    fn test_system_config_validation() {
        let mut config = SystemConfig::default();
        assert!(config.validate().is_ok());

        config.max_proposals_per_diagnostic = 0;
        assert!(config.validate().is_err());

        config = SystemConfig::default();
        config.min_confidence_threshold = 1.5;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_method_signature() {
        let mut method = MethodSignature::new("test_method");
        method.add_parameter(Parameter::new("param1", "String"));
        method.set_return_type("bool");

        let signature = method.canonical_signature();
        assert!(signature.contains("test_method"));
        assert!(signature.contains("param1: String"));
        assert!(signature.contains("-> bool"));

        assert_eq!(method.complexity_score(), 3); // 1 base + 1 param + 1 return
    }

    #[test]
    fn test_cached_docs_data() {
        let crate_info = CrateInfo::new("test_crate", "1.0.0", "https://docs.rs/test_crate");
        let docs_data = CachedDocsData::new(
            crate_info,
            vec![],
            vec![],
            vec![],
            DataSource::LocalAnalysis,
        );

        assert!(docs_data.is_valid());
        assert_eq!(docs_data.access_count(), 1);

        docs_data.touch();
        assert_eq!(docs_data.access_count(), 2);
    }

    #[test]
    fn test_project_correction() {
        let diagnostic = CompilerDiagnostic::new("test", "test error", DiagnosticLevel::Error);
        let mut correction = ProjectCorrection::new(PathBuf::from("test.rs"), diagnostic);

        let proposal = CorrectionProposal::new(
            "old",
            "new",
            0.8,
            CorrectionStrategy::Generic {
                description: "test".to_string(),
            },
        );

        correction.add_proposal(proposal);
        assert_eq!(correction.proposals.len(), 1);
        assert!(correction.best_proposal().is_some());

        let summary = correction.summary();
        assert!(summary.contains("test.rs"));
        assert!(summary.contains("1 proposals"));
    }
}
