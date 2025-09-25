/* yoshi-deluxe/src/ast.rs */
//! **Brief:** AST analysis engine with precise mapping and context extraction for yoshi-deluxe.
//!
//! This module provides comprehensive AST analysis capabilities with byte-offset mapping,
//! context extraction, and intelligent scope analysis. It integrates seamlessly with the
//! yoshi error framework to provide detailed diagnostic information and recovery strategies.

use crate::{
    constants::{BYTE_OFFSET_TOLERANCE, MAX_FILE_SIZE},
    errors::{factory, Result, YoshiDeluxeExt},
    types::{CompilerDiagnostic, DiagnosticSpan},
};
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::SystemTime,
};
use syn::{
    parse_file, visit::Visit, Expr, File, Item, ItemFn, Local, Pat, PatIdent, PatType, Stmt,
};
use tokio::sync::RwLock;
use yoshi_std::LayText;
use syn::spanned::Spanned;

//--------------------------------------------------------------------------------------------------
// AST Analysis Engine with Precise Mapping
//--------------------------------------------------------------------------------------------------

/// Production-grade AST analysis engine with byte-offset mapping
pub struct ASTAnalysisEngine {
    /// File cache for parsed ASTs with source mapping
    ast_cache: Arc<RwLock<HashMap<PathBuf, CachedAst>>>,
    /// Source map cache for byte-offset to AST node mapping
    source_map_cache: Arc<RwLock<HashMap<PathBuf, SourceMap>>>,
    /// Analysis metrics
    metrics: AnalysisMetrics,
}

/// Cached AST with source mapping information
#[derive(Debug, Clone)]
struct CachedAst {
    /// Parsed syntax tree
    ast: File,
    /// Source file content for mapping
    source_content: String,
    /// File modification time for cache invalidation
    modified_at: SystemTime,
    /// Parse timestamp
    parsed_at: SystemTime,
}

/// Source map for precise byte-offset to AST node mapping
#[derive(Debug, Clone)]
pub struct SourceMap {
    /// Map from byte ranges to AST node information
    node_map: Vec<NodeMapping>,
    /// Line start byte offsets
    line_starts: Vec<usize>,
    /// Source content for validation
    source_content: String,
}

/// Mapping between byte range and AST node
#[derive(Debug, Clone)]
struct NodeMapping {
    /// Start byte offset
    start: usize,
    /// End byte offset
    end: usize,
    /// Node type classification
    node_type: NodeType,
    /// Node path in AST (for navigation)
    node_path: Vec<String>,
    /// Source text for this node
    text: String,
}

/// Performance metrics for AST analysis
#[derive(Debug, Default)]
pub struct AnalysisMetrics {
    /// Files analyzed
    pub files_processed: AtomicU64,
    /// AST nodes analyzed
    pub nodes_analyzed: AtomicU64,
    /// Successful mappings
    pub successful_mappings: AtomicU64,
    /// Cache hit ratio
    pub cache_hits: AtomicU64,
}

impl AnalysisMetrics {
    /// Record a successful file processing
    pub fn record_file_processed(&self) {
        self.files_processed.fetch_add(1, Ordering::Relaxed);
    }

    /// Record cache hit
    pub fn record_cache_hit(&self) {
        self.cache_hits.fetch_add(1, Ordering::Relaxed);
    }

    /// Get cache hit ratio
    #[must_use]
    pub fn cache_hit_ratio(&self) -> f64 {
        let hits = self.cache_hits.load(Ordering::Relaxed) as f64;
        let total = self.files_processed.load(Ordering::Relaxed) as f64;
        if total > 0.0 {
            hits / total
        } else {
            0.0
        }
    }

    /// Record nodes analyzed
    pub fn record_nodes_analyzed(&self, count: usize) {
        self.nodes_analyzed
            .fetch_add(count as u64, Ordering::Relaxed);
    }

    /// Record successful mapping
    pub fn record_successful_mapping(&self) {
        self.successful_mappings.fetch_add(1, Ordering::Relaxed);
    }
}

impl SourceMap {
    /// Find AST node at specific byte offset with tolerance
    pub fn find_node_at_offset(&self, offset: usize) -> Option<&NodeMapping> {
        // Use binary search for O(log n) lookup
        self.node_map.iter().find(|mapping| {
            offset >= mapping.start.saturating_sub(BYTE_OFFSET_TOLERANCE)
                && offset <= mapping.end.saturating_add(BYTE_OFFSET_TOLERANCE)
        })
    }

    /// Get line/column from byte offset
    pub fn byte_to_line_column(&self, offset: usize) -> (usize, usize) {
        let line_idx = self
            .line_starts
            .binary_search(&offset)
            .unwrap_or_else(|idx| idx.saturating_sub(1));

        let line_start = self.line_starts.get(line_idx).copied().unwrap_or(0);
        let column = offset.saturating_sub(line_start);

        (line_idx + 1, column + 1) // 1-indexed
    }

    /// Get total number of lines
    #[must_use]
    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }

    /// Get all nodes in a line range
    pub fn nodes_in_line_range(&self, start_line: usize, end_line: usize) -> Vec<&NodeMapping> {
        self.node_map
            .iter()
            .filter(|mapping| {
                let (start_l, _) = self.byte_to_line_column(mapping.start);
                let (end_l, _) = self.byte_to_line_column(mapping.end);
                start_l >= start_line && end_l <= end_line
            })
            .collect()
    }

    /// Find nodes by type
    pub fn find_nodes_by_type(&self, node_type: &NodeType) -> Vec<&NodeMapping> {
        self.node_map
            .iter()
            .filter(|mapping| {
                std::mem::discriminant(&mapping.node_type) == std::mem::discriminant(node_type)
            })
            .collect()
    }
}

//--------------------------------------------------------------------------------------------------
// AST Context and Node Information
//--------------------------------------------------------------------------------------------------

/// AST analysis context with comprehensive metadata
#[derive(Debug, Clone)]
pub struct ASTContext {
    /// Source file path
    pub file_path: PathBuf,
    /// Problematic AST node information with precise mapping
    pub problematic_node: NodeInfo,
    /// Surrounding code context with scope analysis
    pub surrounding_context: SurroundingContext,
    /// Original diagnostic information
    pub diagnostic_info: CompilerDiagnostic,
    /// Source mapping for navigation
    pub source_map: Option<SourceMap>,
}

/// Detailed AST node information with precise location data
#[derive(Debug, Clone)]
pub struct NodeInfo {
    /// Node type classification
    pub node_type: NodeType,
    /// Node content as string (for safe replacement)
    pub content: String,
    /// Precise byte range in source
    pub byte_range: (usize, usize),
    /// Line/column location
    pub line_column_range: (usize, usize),
    /// Path to this node in AST
    pub node_path: Vec<String>,
    /// Associated metadata
    pub metadata: HashMap<String, String>,
}

impl NodeInfo {
    /// Create new node info
    #[must_use]
    pub fn new(
        node_type: NodeType,
        content: String,
        byte_range: (usize, usize),
        line_column_range: (usize, usize),
    ) -> Self {
        Self {
            node_type,
            content,
            byte_range,
            line_column_range,
            node_path: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add metadata to the node
    pub fn add_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }

    /// Get byte length of this node
    #[must_use]
    pub fn byte_length(&self) -> usize {
        self.byte_range.1.saturating_sub(self.byte_range.0)
    }
}

/// Enhanced classification of AST node types with comprehensive coverage
#[derive(Debug, Clone)]
pub enum NodeType {
    /// Function call with receiver and arguments
    FunctionCall {
        /// The name of the function being called.
        function_name: String,
        /// The arguments passed to the function.
        args: Vec<String>,
        /// The receiver of the call, if any.
        receiver: Option<String>,
    },
    /// Method call with full context
    MethodCall {
        /// The receiver of the method call.
        receiver: String,
        /// The name of the method.
        method_name: String,
        /// The arguments passed to the method.
        args: Vec<String>,
        /// The type of the receiver, if known.
        receiver_type: Option<String>,
    },
    /// Type annotation or reference
    TypeAnnotation {
        /// The name of the type.
        type_name: String,
        /// Generic arguments for the type.
        generic_args: Vec<String>,
    },
    /// Variable declaration with full context
    VariableDeclaration {
        /// The name of the variable.
        variable_name: String,
        /// The type annotation, if explicit.
        type_annotation: Option<String>,
        /// Whether the variable is mutable.
        is_mutable: bool,
        /// The initializer expression, if any.
        initializer: Option<String>,
    },
    /// Import/use statement
    ImportStatement {
        /// The path of the import.
        import_path: String,
        /// Specific items imported from the path.
        imported_items: Vec<String>,
    },
    /// Struct definition
    StructDefinition {
        /// The name of the struct.
        struct_name: String,
        /// The fields of the struct.
        fields: Vec<String>,
        /// Generic parameters of the struct.
        generics: Vec<String>,
    },
    /// Enum definition
    EnumDefinition {
        /// The name of the enum.
        enum_name: String,
        /// The variants of the enum.
        variants: Vec<String>,
        /// Generic parameters of the enum.
        generics: Vec<String>,
    },
    /// Function definition
    FunctionDefinition {
        /// The name of the function.
        function_name: String,
        /// The parameters of the function.
        parameters: Vec<String>,
        /// The return type, if any.
        return_type: Option<String>,
        /// Generic parameters of the function.
        generics: Vec<String>,
    },
    /// Implementation block
    ImplBlock {
        /// The type the impl block is for.
        implementing_type: String,
        /// The trait being implemented, if any.
        trait_name: Option<String>,
        /// Methods defined in the impl block.
        methods: Vec<String>,
    },
    /// Expression with classification
    Expression {
        /// The type of expression.
        expression_type: String,
        /// Sub-expressions contained within.
        subexpressions: Vec<String>,
    },
    /// Statement with classification
    Statement {
        /// The type of statement.
        statement_type: String,
        /// Components of the statement.
        components: Vec<String>,
    },
    /// Pattern matching
    Pattern {
        /// The type of pattern.
        pattern_type: String,
        /// Bindings created by the pattern.
        bindings: Vec<String>,
    },
    /// Generic/unknown node with description
    Other {
        /// A description of the node.
        description: String,
        /// The Rust type name of the node from syn.
        rust_type: String,
    },
}

/// Context information about surrounding code with enhanced scope analysis
#[derive(Debug, Clone)]
pub struct SurroundingContext {
    /// Current function context with full signature
    pub current_function: Option<FunctionContext>,
    /// Available imports with aliasing info
    pub imports: Vec<ImportInfo>,
    /// Local variables in scope with types
    pub local_variables: Vec<VariableInfo>,
    /// Available types with their origins
    pub available_types: Vec<TypeInfo>,
    /// Current module path
    pub module_path: Vec<String>,
    /// Trait implementations in scope
    pub trait_impls: Vec<TraitImplInfo>,
    /// Macro invocations and definitions
    pub macros: Vec<MacroInfo>,
}

impl Default for SurroundingContext {
    fn default() -> Self {
        Self {
            current_function: None,
            imports: Vec::new(),
            local_variables: Vec::new(),
            available_types: Vec::new(),
            module_path: Vec::new(),
            trait_impls: Vec::new(),
            macros: Vec::new(),
        }
    }
}

/// Enhanced function context information
#[derive(Debug, Clone)]
pub struct FunctionContext {
    /// Function name
    pub name: String,
    /// Parameters with full type information
    pub parameters: Vec<crate::types::Parameter>,
    /// Return type with path
    pub return_type: Option<String>,
    /// Generic parameters
    pub generics: Vec<String>,
    /// Where clause
    pub where_clause: Option<String>,
    /// Function attributes
    pub attributes: Vec<String>,
    /// Whether function is async
    pub is_async: bool,
    /// Whether function is const
    pub is_const: bool,
}

/// Import information with aliasing and visibility
#[derive(Debug, Clone)]
pub struct ImportInfo {
    /// Import path
    pub path: String,
    /// Imported items
    pub items: Vec<String>,
    /// Alias if used
    pub alias: Option<String>,
    /// Visibility (pub, pub(crate), etc.)
    pub visibility: Option<String>,
}

/// Enhanced variable information with scope data
#[derive(Debug, Clone)]
pub struct VariableInfo {
    /// Variable name
    pub name: String,
    /// Variable type with full path
    pub var_type: Option<String>,
    /// Whether variable is mutable
    pub is_mutable: bool,
    /// Scope depth (0 = function level)
    pub scope_depth: usize,
    /// Declaration location
    pub declared_at: (usize, usize), // line, column
}

/// Type information with origin tracking
#[derive(Debug, Clone)]
pub struct TypeInfo {
    /// Type name
    pub name: String,
    /// Full path to type
    pub full_path: String,
    /// Type kind (struct, enum, trait, etc.)
    pub kind: String,
    /// Source crate if external
    pub source_crate: Option<String>,
    /// Generic parameters
    pub generics: Vec<String>,
}

/// Trait implementation information
#[derive(Debug, Clone)]
pub struct TraitImplInfo {
    /// Trait name
    pub trait_name: String,
    /// Implementing type
    pub implementing_type: String,
    /// Available methods from this impl
    pub methods: Vec<String>,
}

/// Macro information
#[derive(Debug, Clone)]
pub struct MacroInfo {
    /// Macro name
    pub name: String,
    /// Macro type (declarative, procedural, etc.)
    pub macro_type: String,
    /// Expected arguments
    pub arguments: Vec<String>,
}

//--------------------------------------------------------------------------------------------------
// AST Analysis Engine Implementation
//--------------------------------------------------------------------------------------------------

impl ASTAnalysisEngine {
    /// Creates a new AST analysis engine with optimized configuration
    #[must_use]
    pub fn new() -> Self {
        Self {
            ast_cache: Arc::new(RwLock::new(HashMap::new())),
            source_map_cache: Arc::new(RwLock::new(HashMap::new())),
            metrics: AnalysisMetrics::default(),
        }
    }

    /// Analyzes a diagnostic and extracts precise AST context
    ///
    /// # Errors
    ///
    /// Returns a yoshi error if:
    /// - The diagnostic has no spans
    /// - File cannot be read or parsed
    /// - AST analysis fails
    pub async fn analyze_diagnostic(
        &mut self,
        diagnostic: &CompilerDiagnostic,
    ) -> Result<ASTContext> {
        let primary_span = diagnostic
            .primary_span()
            .ok_or_else(|| {
                factory::diagnostic_processing_error(
                    "No spans available for analysis",
                    std::env::current_dir().unwrap_or_default(),
                )
            })
            .lay("Extracting primary span from diagnostic")?;

        // Load and parse the file with source mapping
        let (file_ast, source_map) = self
            .load_file_with_mapping(&primary_span.file_name)
            .await
            .lay("Loading and parsing source file")?;

        // Extract the problematic node using precise mapping
        let problematic_node = self
            .extract_node_at_span(&source_map, primary_span)
            .lay("Extracting problematic AST node")?;

        // Analyze surrounding context with enhanced scope analysis
        let context = self
            .analyze_surrounding_context(&file_ast, &source_map, primary_span)
            .lay("Analyzing surrounding code context")?;

        self.metrics.record_file_processed();

        Ok(ASTContext {
            file_path: primary_span.file_name.clone(),
            problematic_node,
            surrounding_context: context,
            diagnostic_info: diagnostic.clone(),
            source_map: Some(source_map),
        })
    }

    /// Loads file and creates comprehensive source mapping
    async fn load_file_with_mapping(&self, file_path: &Path) -> Result<(File, SourceMap)> {
        let canonical_path = file_path
            .canonicalize()
            .with_file_context(file_path)
            .lay("Canonicalizing file path")?;

        // Check cache first
        {
            let cache = self.ast_cache.read().await;
            let map_cache = self.source_map_cache.read().await;

            if let (Some(cached_ast), Some(cached_map)) =
                (cache.get(&canonical_path), map_cache.get(&canonical_path))
            {
                // Verify cache validity
                if let Ok(metadata) = fs::metadata(&canonical_path) {
                    if let Ok(modified) = metadata.modified() {
                        if modified <= cached_ast.modified_at {
                            self.metrics.record_cache_hit();
                            return Ok((cached_ast.ast.clone(), cached_map.clone()));
                        }
                    }
                }
            }
        }

        // Read and parse file
        let content = fs::read_to_string(&canonical_path)
            .with_file_context(&canonical_path)
            .lay("Reading source file content")?;

        if content.len() > MAX_FILE_SIZE {
            return Err(factory::resource_exhausted_error(
                "file_size",
                MAX_FILE_SIZE as u64,
                content.len() as u64,
            ))
            .lay("File size exceeds maximum allowed limit");
        }

        let ast = parse_file(&content)
            .map_err(|e| {
                factory::ast_analysis_error(
                    "Failed to parse Rust source",
                    canonical_path.clone(),
                    0,
                    0,
                    e,
                )
            })
            .lay("Parsing Rust source file")?;

        // Create comprehensive source mapping
        let source_map = self
            .create_source_map(&ast, &content)
            .lay("Creating source mapping")?;

        // Cache results
        {
            let mut cache = self.ast_cache.write().await;
            let mut map_cache = self.source_map_cache.write().await;

            let metadata = fs::metadata(&canonical_path)
                .with_file_context(&canonical_path)
                .lay("Reading file metadata")?;
            let modified_at = metadata
                .modified()
                .with_file_context(&canonical_path)
                .lay("Getting file modification time")?;

            cache.insert(
                canonical_path.clone(),
                CachedAst {
                    ast: ast.clone(),
                    source_content: content.clone(),
                    modified_at,
                    parsed_at: SystemTime::now(),
                },
            );

            map_cache.insert(canonical_path, source_map.clone());
        }

        Ok((ast, source_map))
    }

    /// Creates comprehensive source mapping for byte-offset to AST navigation
    fn create_source_map(&self, ast: &File, content: &str) -> Result<SourceMap> {
        let mut visitor = SourceMapVisitor::new(content);
        visitor.visit_file(ast);

        // Calculate line start positions
        let line_starts: Vec<usize> = std::iter::once(0)
            .chain(content.match_indices('\n').map(|(idx, _)| idx + 1))
            .collect();

        self.metrics.record_nodes_analyzed(visitor.mappings.len());

        Ok(SourceMap {
            node_map: visitor.mappings,
            line_starts,
            source_content: content.to_string(),
        })
    }

    /// Extracts the specific AST node at the given span with precise mapping
    fn extract_node_at_span(
        &self,
        source_map: &SourceMap,
        span: &DiagnosticSpan,
    ) -> Result<NodeInfo> {
        let mapping = source_map
            .find_node_at_offset(span.byte_start)
            .or_else(|| source_map.find_node_at_offset(span.byte_end))
            .ok_or_else(|| {
                factory::ast_analysis_error(
                    format!(
                        "No AST node found at byte range {}..{}",
                        span.byte_start, span.byte_end
                    ),
                    span.file_name.clone(),
                    span.line_start,
                    span.column_start,
                    syn::Error::new(Span::call_site(), "Node not found"),
                )
            })
            .lay("Finding AST node at diagnostic span")?;

        self.metrics.record_successful_mapping();

        Ok(NodeInfo {
            node_type: mapping.node_type.clone(),
            content: mapping.text.clone(),
            byte_range: (mapping.start, mapping.end),
            line_column_range: source_map.byte_to_line_column(mapping.start),
            node_path: mapping.node_path.clone(),
            metadata: HashMap::new(),
        })
    }

    /// Analyzes context around the problematic code with enhanced scope detection
    fn analyze_surrounding_context(
        &self,
        file_ast: &File,
        source_map: &SourceMap,
        span: &DiagnosticSpan,
    ) -> Result<SurroundingContext> {
        let mut analyzer = ContextAnalyzer::new(span.byte_start, span.byte_end, source_map);
        analyzer.visit_file(file_ast);

        Ok(analyzer.context)
    }

    /// Get performance metrics
    #[must_use]
    pub fn metrics(&self) -> &AnalysisMetrics {
        &self.metrics
    }

    /// Clear caches to free memory
    pub async fn clear_caches(&self) {
        let mut ast_cache = self.ast_cache.write().await;
        let mut map_cache = self.source_map_cache.write().await;
        ast_cache.clear();
        map_cache.clear();
    }

    /// Get cache statistics
    pub async fn cache_stats(&self) -> CacheStats {
        let ast_cache = self.ast_cache.read().await;
        let map_cache = self.source_map_cache.read().await;

        CacheStats {
            ast_cache_size: ast_cache.len(),
            source_map_cache_size: map_cache.len(),
            total_files_processed: self.metrics.files_processed.load(Ordering::Relaxed),
            cache_hit_ratio: self.metrics.cache_hit_ratio(),
        }
    }
}

impl Default for ASTAnalysisEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Number of cached ASTs
    pub ast_cache_size: usize,
    /// Number of cached source maps
    pub source_map_cache_size: usize,
    /// Total files processed
    pub total_files_processed: u64,
    /// Cache hit ratio
    pub cache_hit_ratio: f64,
}

//--------------------------------------------------------------------------------------------------
// Source Map Visitor for Precise AST Mapping
//--------------------------------------------------------------------------------------------------

/// Visitor that creates comprehensive source mapping
struct SourceMapVisitor<'a> {
    /// Source content for position calculation
    source: &'a str,
    /// Collected node mappings
    mappings: Vec<NodeMapping>,
    /// Current AST path
    current_path: Vec<String>,
}

impl<'a> SourceMapVisitor<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            mappings: Vec::new(),
            current_path: Vec::new(),
        }
    }

    /// Add a node mapping with position calculation
    fn add_mapping(&mut self, span: Span, node_type: NodeType) {
        let start_byte = span.start().byte;
        let end_byte = span.end().byte;

        let text = if start_byte < self.source.len()
            && end_byte <= self.source.len()
            && start_byte < end_byte
        {
            self.source[start_byte..end_byte].to_string()
        } else {
            String::new()
        };

        self.mappings.push(NodeMapping {
            start: start_byte,
            end: end_byte,
            node_type,
            node_path: self.current_path.clone(),
            text,
        });
    }
}

impl<'a, 'ast> Visit<'ast> for SourceMapVisitor<'a> {
    fn visit_item(&mut self, item: &'ast Item) {
        match item {
            Item::Fn(func) => {
                self.current_path.push(format!("fn::{}", func.sig.ident));

                self.add_mapping(
                    func.span(),
                    NodeType::FunctionDefinition {
                        function_name: func.sig.ident.to_string(),
                        parameters: func
                            .sig
                            .inputs
                            .iter()
                            .map(|input| input.to_token_stream().to_string())
                            .collect(),
                        return_type: match &func.sig.output {
                            syn::ReturnType::Type(_, ty) => Some(ty.to_token_stream().to_string()),
                            _ => None,
                        },
                        generics: func
                            .sig
                            .generics
                            .params
                            .iter()
                            .map(|p| p.to_token_stream().to_string())
                            .collect(),
                    },
                );

                syn::visit::visit_item_fn(self, func);
                self.current_path.pop();
            }
            Item::Struct(struct_item) => {
                self.current_path
                    .push(format!("struct::{}", struct_item.ident));

                self.add_mapping(
                    struct_item.span(),
                    NodeType::StructDefinition {
                        struct_name: struct_item.ident.to_string(),
                        fields: match &struct_item.fields {
                            syn::Fields::Named(fields) => fields
                                .named
                                .iter()
                                .map(|f| {
                                    f.ident.as_ref().map_or_else(String::new, |i| i.to_string())
                                })
                                .collect(),
                            _ => vec![],
                        },
                        generics: struct_item
                            .generics
                            .params
                            .iter()
                            .map(|p| p.to_token_stream().to_string())
                            .collect(),
                    },
                );

                syn::visit::visit_item_struct(self, struct_item);
                self.current_path.pop();
            }
            Item::Enum(enum_item) => {
                self.current_path.push(format!("enum::{}", enum_item.ident));

                self.add_mapping(
                    enum_item.span(),
                    NodeType::EnumDefinition {
                        enum_name: enum_item.ident.to_string(),
                        variants: enum_item
                            .variants
                            .iter()
                            .map(|v| v.ident.to_string())
                            .collect(),
                        generics: enum_item
                            .generics
                            .params
                            .iter()
                            .map(|p| p.to_token_stream().to_string())
                            .collect(),
                    },
                );

                syn::visit::visit_item_enum(self, enum_item);
                self.current_path.pop();
            }
            Item::Use(use_item) => {
                self.add_mapping(
                    use_item.span(),
                    NodeType::ImportStatement {
                        import_path: use_item.tree.to_token_stream().to_string(),
                        imported_items: vec![], // Could parse use tree for specifics
                    },
                );

                syn::visit::visit_item_use(self, use_item);
            }
            Item::Impl(impl_item) => {
                let implementing_type = impl_item.self_ty.to_token_stream().to_string();
                let trait_name = impl_item
                    .trait_
                    .as_ref()
                    .map(|(_, path, _)| path.to_token_stream().to_string());

                let methods = impl_item
                    .items
                    .iter()
                    .filter_map(|item| {
                        if let syn::ImplItem::Fn(method) = item {
                            Some(method.sig.ident.to_string())
                        } else {
                            None
                        }
                    })
                    .collect();

                self.add_mapping(
                    impl_item.span(),
                    NodeType::ImplBlock {
                        implementing_type,
                        trait_name,
                        methods,
                    },
                );

                syn::visit::visit_item_impl(self, impl_item);
            }
            _ => {
                syn::visit::visit_item(self, item);
            }
        }
    }

    fn visit_stmt(&mut self, stmt: &'ast Stmt) {
        if let Stmt::Local(local) = stmt {
            let var_name = match &local.pat {
                Pat::Ident(ident) => ident.ident.to_string(),
                _ => "pattern".to_string(),
            };

            let type_annotation = if let Pat::Type(PatType { ty, .. }) = &local.pat {
                Some(ty.to_token_stream().to_string())
            } else {
                None
            };

            self.add_mapping(
                local.span(),
                NodeType::VariableDeclaration {
                    variable_name: var_name,
                    type_annotation,
                    is_mutable: matches!(&local.pat, Pat::Ident(ident) if ident.mutability.is_some()),
                    initializer: local.init.as_ref().map(|init| init.expr.to_token_stream().to_string()),
                },
            );
        }

        syn::visit::visit_stmt(self, stmt);
    }

    fn visit_expr(&mut self, expr: &'ast Expr) {
        if let Expr::MethodCall(mc) = expr {
            self.add_mapping(
                expr.span(),
                NodeType::MethodCall {
                    receiver: mc.receiver.to_token_stream().to_string(),
                    method_name: mc.method.to_string(),
                    args: mc
                        .args
                        .iter()
                        .map(|arg| arg.to_token_stream().to_string())
                        .collect(),
                    receiver_type: None, // Could be inferred with type analysis
                },
            );
        } else if let Expr::Call(call) = expr {
            if let Expr::Path(path) = &*call.func {
                if let Some(ident) = path.path.get_ident() {
                    self.add_mapping(
                        expr.span(),
                        NodeType::FunctionCall {
                            function_name: ident.to_string(),
                            args: call
                                .args
                                .iter()
                                .map(|arg| arg.to_token_stream().to_string())
                                .collect(),
                            receiver: None,
                        },
                    );
                }
            }
        }

        syn::visit::visit_expr(self, expr);
    }
}

//--------------------------------------------------------------------------------------------------
// Enhanced Context Analyzer with Scope Detection
//--------------------------------------------------------------------------------------------------

/// Enhanced context analyzer for surrounding code with scope tracking
struct ContextAnalyzer<'a> {
    target_start: usize,
    target_end: usize,
    source_map: &'a SourceMap,
    context: SurroundingContext,
    current_scope_depth: usize,
}

impl<'a> ContextAnalyzer<'a> {
    fn new(start: usize, end: usize, source_map: &'a SourceMap) -> Self {
        Self {
            target_start: start,
            target_end: end,
            source_map,
            context: SurroundingContext::default(),
            current_scope_depth: 0,
        }
    }
}

impl<'a, 'ast> Visit<'ast> for ContextAnalyzer<'a> {
    fn visit_file(&mut self, file: &'ast File) {
        // Extract module-level information
        for item in &file.items {
            match item {
                Item::Use(use_item) => {
                    self.context.imports.push(ImportInfo {
                        path: use_item.tree.to_token_stream().to_string(),
                        items: vec![], // Could parse use tree for specifics
                        alias: None,
                        visibility: if use_item.vis.to_token_stream().to_string().is_empty() {
                            None
                        } else {
                            Some(use_item.vis.to_token_stream().to_string())
                        },
                    });
                }
                Item::Struct(struct_item) => {
                    self.context.available_types.push(TypeInfo {
                        name: struct_item.ident.to_string(),
                        full_path: struct_item.ident.to_string(), // Could be enhanced with module path
                        kind: "struct".to_string(),
                        source_crate: None,
                        generics: struct_item
                            .generics
                            .params
                            .iter()
                            .map(|p| p.to_token_stream().to_string())
                            .collect(),
                    });
                }
                Item::Enum(enum_item) => {
                    self.context.available_types.push(TypeInfo {
                        name: enum_item.ident.to_string(),
                        full_path: enum_item.ident.to_string(),
                        kind: "enum".to_string(),
                        source_crate: None,
                        generics: enum_item
                            .generics
                            .params
                            .iter()
                            .map(|p| p.to_token_stream().to_string())
                            .collect(),
                    });
                }
                Item::Impl(impl_item) => {
                    let implementing_type = impl_item.self_ty.to_token_stream().to_string();
                    let trait_name = impl_item
                        .trait_
                        .as_ref()
                        .map(|(_, path, _)| path.to_token_stream().to_string());

                    if let Some(trait_name) = trait_name {
                        let methods = impl_item
                            .items
                            .iter()
                            .filter_map(|item| {
                                if let syn::ImplItem::Fn(method) = item {
                                    Some(method.sig.ident.to_string())
                                } else {
                                    None
                                }
                            })
                            .collect();

                        self.context.trait_impls.push(TraitImplInfo {
                            trait_name,
                            implementing_type,
                            methods,
                        });
                    }
                }
                _ => {}
            }
        }

        // Analyze items for target context
        for item in &file.items {
            self.visit_item(item);
        }
    }

    fn visit_item_fn(&mut self, func: &'ast ItemFn) {
        let span = func.span();
        let start_byte = span.start().byte;
        let end_byte = span.end().byte;

        // Check if target is within this function
        if self.target_start >= start_byte && self.target_end <= end_byte {
            // Extract function parameters
            let parameters = func
                .sig
                .inputs
                .iter()
                .filter_map(|input| {
                    if let syn::FnArg::Typed(typed) = input {
                        Some(crate::types::Parameter::new(
                            typed.pat.to_token_stream().to_string(),
                            typed.ty.to_token_stream().to_string(),
                        ))
                    } else {
                        None
                    }
                })
                .collect();

            let return_type = match &func.sig.output {
                syn::ReturnType::Type(_, ty) => Some(ty.to_token_stream().to_string()),
                _ => None,
            };

            self.context.current_function = Some(FunctionContext {
                name: func.sig.ident.to_string(),
                parameters,
                return_type,
                generics: func
                    .sig
                    .generics
                    .params
                    .iter()
                    .map(|p| p.to_token_stream().to_string())
                    .collect(),
                where_clause: func
                    .sig
                    .generics
                    .where_clause
                    .as_ref()
                    .map(|w| w.to_token_stream().to_string()),
                attributes: func
                    .attrs
                    .iter()
                    .map(|attr| attr.to_token_stream().to_string())
                    .collect(),
                is_async: func.sig.asyncness.is_some(),
                is_const: func.sig.constness.is_some(),
            });

            // Analyze function body for local variables
            for stmt in &func.block.stmts {
                self.visit_stmt(stmt);
            }
        }
    }

    fn visit_stmt(&mut self, stmt: &'ast Stmt) {
        if let Stmt::Local(local) = stmt {
            if let Pat::Ident(ident) = &local.pat {
                let span = local.span();
                let (line, column) = self.source_map.byte_to_line_column(span.start().byte);

                self.context.local_variables.push(VariableInfo {
                    name: ident.ident.to_string(),
                    var_type: if let Some(init) = &local.init {
                        // Basic type inference placeholder
                        Some(init.expr.to_token_stream().to_string())
                    } else {
                        None
                    },
                    is_mutable: ident.mutability.is_some(),
                    scope_depth: self.current_scope_depth,
                    declared_at: (line, column),
                });
            }
        }

        syn::visit::visit_stmt(self, stmt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use tokio::io::AsyncWriteExt;

    async fn create_test_file(content: &str) -> Result<NamedTempFile> {
        let mut file = NamedTempFile::new()
            .hatch()
            .lay("Creating temporary test file")?;

        tokio::fs::write(file.path(), content)
            .await
            .with_file_context(file.path())
            .lay("Writing test content to file")?;

        Ok(file)
    }

    #[tokio::test]
    async fn test_ast_engine_creation() {
        let engine = ASTAnalysisEngine::new();
        assert_eq!(engine.metrics().cache_hit_ratio(), 0.0);
    }

    #[tokio::test]
    async fn test_source_file_parsing() -> Result<()> {
        let content = r#"
fn test_function(x: i32) -> bool {
    let y = x + 1;
    y > 0
}
"#;
        let file = create_test_file(content).await?;
        let mut engine = ASTAnalysisEngine::new();

        let result = engine.load_file_with_mapping(file.path()).await;
        assert!(result.is_ok());

        let (ast, source_map) = result.unwrap();
        assert!(!ast.items.is_empty());
        assert!(!source_map.node_map.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_source_map_byte_to_line_column() {
        let content = "line 1\nline 2\nline 3";
        let line_starts = vec![0, 7, 14];
        let source_map = SourceMap {
            node_map: vec![],
            line_starts,
            source_content: content.to_string(),
        };

        assert_eq!(source_map.byte_to_line_column(0), (1, 1));
        assert_eq!(source_map.byte_to_line_column(7), (2, 1));
        assert_eq!(source_map.byte_to_line_column(14), (3, 1));
    }

    #[tokio::test]
    async fn test_node_info_operations() {
        let mut node = NodeInfo::new(
            NodeType::FunctionCall {
                function_name: "test".to_string(),
                args: vec![],
                receiver: None,
            },
            "test_code".to_string(),
            (10, 20),
            (1, 1),
        );

        assert_eq!(node.byte_length(), 10);

        node.add_metadata("test_key", "test_value");
        assert!(node.metadata.contains_key("test_key"));
    }

    #[tokio::test]
    async fn test_cache_stats() -> Result<()> {
        let content = "fn main() {}";
        let file = create_test_file(content).await?;
        let mut engine = ASTAnalysisEngine::new();

        // Load file to populate cache
        let _ = engine.load_file_with_mapping(file.path()).await?;

        let stats = engine.cache_stats().await;
        assert_eq!(stats.ast_cache_size, 1);
        assert_eq!(stats.source_map_cache_size, 1);

        Ok(())
    }

    #[test]
    fn test_surrounding_context_default() {
        let context = SurroundingContext::default();
        assert!(context.imports.is_empty());
        assert!(context.local_variables.is_empty());
        assert!(context.available_types.is_empty());
        assert!(context.current_function.is_none());
    }
}
