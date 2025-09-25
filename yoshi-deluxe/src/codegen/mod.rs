/* yoshi-deluxe/src/codegen.rs */
//! **Brief:** Code generation engine with safe AST-based modifications for yoshi-deluxe.
//!
//! This module provides advanced code generation capabilities with safe AST-based
//! modifications, comprehensive validation, and intelligent correction strategies.
//! It integrates with the yoshi error framework for robust error handling and recovery.

use crate::{
    ast::ASTContext,
    constants::{CODEGEN_MAX_ITERATIONS, REGEX_PATTERNS},
    errors::{factory, Result, YoshiDeluxeExt},
    types::{CachedDocsData, CorrectionProposal, CorrectionStrategy, SafetyLevel},
};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};
use syn::{parse_str, Expr, Item, Stmt};
use tokio::sync::RwLock;
use yoshi_std::LayText;

//--------------------------------------------------------------------------------------------------
// Code Generation Engine with Safe AST Modifications
//--------------------------------------------------------------------------------------------------

/// Advanced code generation engine with safe AST-based modifications
pub struct CodeGenerationEngine {
    /// Template cache for common corrections
    template_cache: Arc<RwLock<HashMap<String, CorrectionTemplate>>>,
    /// Validation engine for generated code
    validator: Arc<Mutex<CodeValidator>>,
    /// Generation metrics
    metrics: GenerationMetrics,
}

/// Correction template for common patterns
#[derive(Debug, Clone)]
struct CorrectionTemplate {
    /// Template pattern
    pattern: String,
    /// Replacement template with placeholders
    replacement: String,
    /// Confidence score for this template
    confidence: f64,
    /// Required context for application
    required_context: Vec<String>,
    /// Safety level of this template
    safety_level: SafetyLevel,
    /// Usage count for popularity tracking
    usage_count: u64,
}

impl CorrectionTemplate {
    /// Create new correction template
    fn new(
        pattern: impl Into<String>,
        replacement: impl Into<String>,
        confidence: f64,
        safety_level: SafetyLevel,
    ) -> Self {
        Self {
            pattern: pattern.into(),
            replacement: replacement.into(),
            confidence,
            required_context: Vec::new(),
            safety_level,
            usage_count: 0,
        }
    }

    /// Increment usage count
    fn use_template(&mut self) {
        self.usage_count += 1;
    }

    /// Get effectiveness score based on usage and confidence
    fn effectiveness_score(&self) -> f64 {
        let usage_factor = (self.usage_count as f64).ln().max(1.0);
        self.confidence * usage_factor
    }
}

/// Code validator for generated corrections
struct CodeValidator {
    /// Validation cache
    validation_cache: HashMap<String, ValidationResult>,
    /// Validation metrics
    validation_count: AtomicU64,
    /// Successful validations
    successful_validations: AtomicU64,
}

/// Validation result
#[derive(Debug, Clone)]
struct ValidationResult {
    /// Whether code is valid
    is_valid: bool,
    /// Validation errors if any
    errors: Vec<String>,
    /// Warnings
    warnings: Vec<String>,
    /// Validation timestamp
    validated_at: Instant,
}

/// Generation performance metrics
#[derive(Debug, Default)]
pub struct GenerationMetrics {
    /// Total corrections generated
    pub corrections_generated: AtomicU64,
    /// Successful validations
    pub successful_validations: AtomicU64,
    /// Template cache hits
    pub template_cache_hits: AtomicU64,
    /// Average generation time
    generation_times: Arc<RwLock<Vec<Duration>>>,
    /// Strategy usage counts
    strategy_usage: Arc<RwLock<HashMap<String, u64>>>,
}

impl GenerationMetrics {
    /// Record correction generation
    pub fn record_generation(&self, strategy: &str, duration: Duration) {
        self.corrections_generated.fetch_add(1, Ordering::Relaxed);

        // Record timing
        if let Ok(mut times) = self.generation_times.try_write() {
            times.push(duration);
            // Keep only recent measurements
            if times.len() > 1000 {
                times.drain(0..500);
            }
        }

        // Record strategy usage
        if let Ok(mut usage) = self.strategy_usage.try_write() {
            *usage.entry(strategy.to_string()).or_insert(0) += 1;
        }
    }

    /// Record successful validation
    pub fn record_successful_validation(&self) {
        self.successful_validations.fetch_add(1, Ordering::Relaxed);
    }

    /// Record template cache hit
    pub fn record_template_cache_hit(&self) {
        self.template_cache_hits.fetch_add(1, Ordering::Relaxed);
    }

    /// Get average generation time
    pub async fn average_generation_time(&self) -> Duration {
        let times = self.generation_times.read().await;
        if times.is_empty() {
            Duration::ZERO
        } else {
            let total: Duration = times.iter().sum();
            total / times.len() as u32
        }
    }

    /// Get most popular strategies
    pub async fn popular_strategies(&self, limit: usize) -> Vec<(String, u64)> {
        let usage = self.strategy_usage.read().await;
        let mut strategies: Vec<_> = usage.iter().map(|(k, v)| (k.clone(), *v)).collect();
        strategies.sort_by(|a, b| b.1.cmp(&a.1));
        strategies.truncate(limit);
        strategies
    }
}

impl CodeValidator {
    /// Create new code validator
    fn new() -> Self {
        Self {
            validation_cache: HashMap::new(),
            validation_count: AtomicU64::new(0),
            successful_validations: AtomicU64::new(0),
        }
    }

    /// Validate that generated code is syntactically correct
    fn validate_syntax(&mut self, code: &str) -> Result<()> {
        self.validation_count.fetch_add(1, Ordering::Relaxed);

        // Check cache first
        if let Some(cached) = self.validation_cache.get(code) {
            if cached.validated_at.elapsed() < Duration::from_secs(300) {
                return if cached.is_valid {
                    self.successful_validations.fetch_add(1, Ordering::Relaxed);
                    Ok(())
                } else {
                    Err(factory::code_generation_error(
                        "syntax_validation",
                        "Cached validation failed",
                        code,
                    ))
                };
            }
        }

        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut is_valid = true;

        // Try parsing as different syntax elements
        let parse_results = vec![
            ("expression", parse_str::<Expr>(code).map(|_| ())),
            ("statement", parse_str::<Stmt>(code).map(|_| ())),
            ("item", parse_str::<Item>(code).map(|_| ())),
        ];

        let any_valid = parse_results.iter().any(|(_, result)| result.is_ok());

        if !any_valid {
            is_valid = false;
            for (syntax_type, result) in parse_results {
                if let Err(e) = result {
                    errors.push(format!("Failed to parse as {syntax_type}: {e}"));
                }
            }
        }

        // Additional semantic checks
        if is_valid {
            self.perform_semantic_validation(code, &mut warnings);
        }

        let result = ValidationResult {
            is_valid,
            errors: errors.clone(),
            warnings,
            validated_at: Instant::now(),
        };

        self.validation_cache.insert(code.to_string(), result);

        if is_valid {
            self.successful_validations.fetch_add(1, Ordering::Relaxed);
            Ok(())
        } else {
            Err(factory::code_generation_error(
                "syntax_validation",
                format!(
                    "Generated code is not syntactically valid: {}",
                    errors.join("; ")
                ),
                code,
            ))
        }
    }

    /// Perform semantic validation checks
    fn perform_semantic_validation(&self, code: &str, warnings: &mut Vec<String>) {
        // Check for common anti-patterns
        if code.contains("unwrap()") {
            warnings.push("Contains .unwrap() which may panic".to_string());
        }

        if code.contains("todo!()") || code.contains("unimplemented!()") {
            warnings.push("Contains incomplete implementation macros".to_string());
        }

        if code.contains("unsafe") {
            warnings.push("Contains unsafe code".to_string());
        }

        // Check for suspicious patterns
        if code.matches('{').count() != code.matches('}').count() {
            warnings.push("Unbalanced braces detected".to_string());
        }

        if code.matches('(').count() != code.matches(')').count() {
            warnings.push("Unbalanced parentheses detected".to_string());
        }
    }

    /// Validate semantic correctness where possible
    fn validate_semantics(&mut self, code: &str, context: &ASTContext) -> Result<()> {
        if code.trim().is_empty() {
            return Err(factory::code_generation_error(
                "semantic_validation",
                "Generated code is empty",
                code,
            ))
            .with_file_context(&context.file_path);
        }

        // Check if the generated code fits the context
        if let Some(func_context) = &context.surrounding_context.current_function {
            if code.contains("return") && func_context.return_type.is_none() {
                return Err(factory::code_generation_error(
                    "semantic_validation",
                    "Generated return statement in function with no return type",
                    code,
                ))
                .with_file_context(&context.file_path);
            }
        }

        Ok(())
    }

    /// Get validation statistics
    fn validation_stats(&self) -> ValidationStats {
        ValidationStats {
            total_validations: self.validation_count.load(Ordering::Relaxed),
            successful_validations: self.successful_validations.load(Ordering::Relaxed),
            cache_size: self.validation_cache.len(),
            success_rate: {
                let total = self.validation_count.load(Ordering::Relaxed) as f64;
                let successful = self.successful_validations.load(Ordering::Relaxed) as f64;
                if total > 0.0 {
                    successful / total
                } else {
                    0.0
                }
            },
        }
    }
}

/// Validation statistics
#[derive(Debug, Clone)]
pub struct ValidationStats {
    /// Total validations performed
    pub total_validations: u64,
    /// Successful validations
    pub successful_validations: u64,
    /// Validation cache size
    pub cache_size: usize,
    /// Success rate (0.0-1.0)
    pub success_rate: f64,
}

//--------------------------------------------------------------------------------------------------
// Code Generation Engine Implementation
//--------------------------------------------------------------------------------------------------

impl CodeGenerationEngine {
    /// Creates a new code generation engine
    #[must_use]
    pub fn new() -> Self {
        let engine = Self {
            template_cache: Arc::new(RwLock::new(HashMap::new())),
            validator: Arc::new(Mutex::new(CodeValidator::new())),
            metrics: GenerationMetrics::default(),
        };

        // Initialize with common templates in background
        let template_cache = Arc::clone(&engine.template_cache);
        tokio::spawn(async move {
            let mut cache = template_cache.write().await;
            // Initialize common templates
            cache.insert("error_handling".to_string(), "Result<T, E>".to_string());
            cache.insert("option_handling".to_string(), "Option<T>".to_string());
            cache.insert("trait_implementation".to_string(), "impl Trait for Type".to_string());
        });

        engine
    }

    /// Initialize common correction templates
    async fn initialize_common_templates(&self) {
        let mut cache = self.template_cache.write().await;

        // String conversion templates
        cache.insert(
            "string_to_str".to_string(),
            CorrectionTemplate::new("{}.to_string()", "{}.as_str()", 0.95, SafetyLevel::Safe),
        );

        cache.insert(
            "str_to_string".to_string(),
            CorrectionTemplate::new("{}.as_str()", "{}.to_string()", 0.95, SafetyLevel::Safe),
        );

        // Option handling templates
        cache.insert(
            "some_wrapper".to_string(),
            CorrectionTemplate::new("{}", "Some({})", 0.90, SafetyLevel::Safe),
        );

        cache.insert(
            "unwrap_to_expect".to_string(),
            CorrectionTemplate::new(
                "{}.unwrap()",
                "{}.expect(\"TODO: add meaningful error message\")",
                0.85,
                SafetyLevel::RequiresReview,
            ),
        );

        // Reference handling templates
        cache.insert(
            "add_reference".to_string(),
            CorrectionTemplate::new("{}", "&{}", 0.85, SafetyLevel::Safe),
        );

        cache.insert(
            "clone_to_fix_move".to_string(),
            CorrectionTemplate::new("{}", "{}.clone()", 0.80, SafetyLevel::RequiresReview),
        );

        // Numeric conversion templates
        cache.insert(
            "numeric_conversion".to_string(),
            CorrectionTemplate::new("{}", "{} as {}", 0.75, SafetyLevel::RequiresReview),
        );
    }

    /// Generates correction proposals based on comprehensive analysis
    ///
    /// # Errors
    ///
    /// Returns a yoshi error if code generation fails for all strategies
    pub async fn generate_corrections(
        &self,
        context: &ASTContext,
        docs_data: Option<&CachedDocsData>,
    ) -> Result<Vec<CorrectionProposal>> {
        let start_time = Instant::now();
        let mut proposals = Vec::new();
        let diagnostic_code = context.diagnostic_info.code.as_deref();

        // Generate corrections based on error code
        match diagnostic_code {
            Some("E0599") => {
                proposals.extend(
                    self.generate_method_corrections(context, docs_data)
                        .await
                        .lay("Generating method-related corrections")?,
                );
            }
            Some("E0308") => {
                proposals.extend(
                    self.generate_type_corrections(context)
                        .await
                        .lay("Generating type mismatch corrections")?,
                );
            }
            Some("E0425") => {
                proposals.extend(
                    self.generate_unresolved_name_corrections(context)
                        .await
                        .lay("Generating unresolved name corrections")?,
                );
            }
            Some("E0560") | Some("E0559") => {
                proposals.extend(
                    self.generate_struct_field_corrections(context)
                        .await
                        .lay("Generating struct field corrections")?,
                );
            }
            _ => {
                proposals.extend(
                    self.generate_generic_corrections(context)
                        .await
                        .lay("Generating generic corrections")?,
                );
            }
        }

        // Validate all proposals
        let mut validated_proposals = Vec::new();
        for mut proposal in proposals {
            let mut validator = self.validator.lock().unwrap();
            if validator
                .validate_syntax(&proposal.corrected_code)
                .is_ok()
                && validator
                    .validate_semantics(&proposal.corrected_code, context)
                    .is_ok()
            {
                // Enhance proposal with additional metadata
                proposal.add_metadata(
                    "generated_at",
                    format!("{:?}", std::time::SystemTime::now()),
                );
                proposal.add_metadata("validation_passed", "true");
                proposal.add_metadata("context_file", context.file_path.display().to_string());

                validated_proposals.push(proposal);
                self.metrics.record_successful_validation();
            }
        }

        // Sort by confidence and limit results
        validated_proposals.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        validated_proposals.truncate(5);

        let strategy_name = diagnostic_code.unwrap_or("generic");
        self.metrics
            .record_generation(strategy_name, start_time.elapsed());

        Ok(validated_proposals)
    }

    /// Generates method-related corrections
    async fn generate_method_corrections(
        &self,
        context: &ASTContext,
        docs_data: Option<&CachedDocsData>,
    ) -> Result<Vec<CorrectionProposal>> {
        let crate::ast::NodeType::MethodCall {
            receiver,
            method_name,
            args,
            ..
        } = &context.problematic_node.node_type
        else {
            return Ok(vec![]);
        };

        let mut proposals = Vec::new();

        // Check documentation-based suggestions
        if let Some(docs) = docs_data {
            for method in &docs.methods {
                if method.name != *method_name {
                    let similarity = self.calculate_method_similarity(method_name, &method.name);
                    if similarity > crate::constants::DEFAULT_SIMILARITY_THRESHOLD {
                        let mut proposal = CorrectionProposal::new(
                            context.problematic_node.content.clone(),
                            format!("{receiver}.{}", method.name),
                            similarity,
                            CorrectionStrategy::MethodNameCorrection {
                                similarity_score: similarity,
                            },
                        );

                        proposal.set_safety_level(if similarity > 0.9 {
                            SafetyLevel::Safe
                        } else {
                            SafetyLevel::RequiresReview
                        });

                        proposal.documentation_source =
                            Some(format!("docs.rs: {}", method.canonical_signature()));
                        proposal.add_metadata("method_signature", method.canonical_signature());
                        proposal.add_metadata(
                            "method_docs",
                            method.documentation.chars().take(200).collect::<String>(),
                        );

                        proposals.push(proposal);
                    }
                }
            }
        }

        // Check context-based suggestions (similar methods in scope)
        proposals.extend(
            self.generate_context_based_method_suggestions(context, method_name)
                .await
                .lay("Generating context-based method suggestions")?,
        );

        // Try template-based corrections
        proposals.extend(
            self.apply_method_templates(context, receiver, method_name, args)
                .await,
        );

        Ok(proposals)
    }

    /// Generate context-based method suggestions from surrounding scope
    async fn generate_context_based_method_suggestions(
        &self,
        context: &ASTContext,
        target_method: &str,
    ) -> Result<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();

        // Check trait implementations in scope
        for trait_impl in &context.surrounding_context.trait_impls {
            for method in &trait_impl.methods {
                let similarity = self.calculate_method_similarity(target_method, method);
                if similarity > crate::constants::DEFAULT_SIMILARITY_THRESHOLD {
                    let mut proposal = CorrectionProposal::new(
                        context.problematic_node.content.clone(),
                        format!(
                            "use {}; // For {method} method\n{}",
                            trait_impl.trait_name, context.problematic_node.content
                        ),
                        similarity * 0.8, // Slightly lower confidence for trait imports
                        CorrectionStrategy::TraitImport {
                            trait_name: trait_impl.trait_name.clone(),
                            method_name: method.clone(),
                        },
                    );

                    proposal.set_safety_level(SafetyLevel::Safe);
                    proposal.add_metadata("trait_name", trait_impl.trait_name.clone());
                    proposal
                        .add_metadata("implementing_type", trait_impl.implementing_type.clone());

                    proposals.push(proposal);
                }
            }
        }

        Ok(proposals)
    }

    /// Apply method-specific templates
    async fn apply_method_templates(
        &self,
        context: &ASTContext,
        receiver: &str,
        method_name: &str,
        args: &[String],
    ) -> Vec<CorrectionProposal> {
        let mut proposals = Vec::new();
        let cache = self.template_cache.read().await;

        // Check for common method correction patterns
        if method_name == "len" {
            if let Some(template) = cache.get("add_reference") {
                let mut proposal = CorrectionProposal::new(
                    context.problematic_node.content.clone(),
                    format!("(&{receiver}).len()"),
                    template.confidence,
                    CorrectionStrategy::ReferenceCorrection {
                        operation: "add_reference_for_len".to_string(),
                    },
                );
                proposal.set_safety_level(template.safety_level);
                proposals.push(proposal);
            }
        }

        // Check for iterator method corrections
        if method_name.starts_with("map") || method_name.starts_with("filter") {
            if let Some(template) = cache.get("add_iter") {
                let mut proposal = CorrectionProposal::new(
                    context.problematic_node.content.clone(),
                    format!("{receiver}.iter().{method_name}({})", args.join(", ")),
                    template.confidence,
                    CorrectionStrategy::MethodNameCorrection {
                        similarity_score: 0.9,
                    },
                );
                proposal.set_safety_level(SafetyLevel::Safe);
                proposals.push(proposal);
            }
        }

        proposals
    }

    /// Generates corrections for type mismatches
    async fn generate_type_corrections(
        &self,
        context: &ASTContext,
    ) -> Result<Vec<CorrectionProposal>> {
        let Some(regex) = REGEX_PATTERNS.get("type_mismatch") else {
            return Ok(vec![]);
        };
        let Some(captures) = regex.captures(&context.diagnostic_info.message) else {
            return Ok(vec![]);
        };

        let expected = captures.get(1).map_or("", |m| m.as_str());
        let found = captures.get(2).map_or("", |m| m.as_str());

        self.generate_type_conversion_corrections(expected, found, context)
            .await
            .lay("Generating type conversion corrections")
    }

    /// Generates corrections based on type conversion patterns
    async fn generate_type_conversion_corrections(
        &self,
        expected: &str,
        found: &str,
        context: &ASTContext,
    ) -> Result<Vec<CorrectionProposal>> {
        let conversions = self.get_type_conversion_patterns();
        let original_code = &context.problematic_node.content;
        let mut proposals = Vec::new();

        for ((from_pattern, to_pattern), conversion, confidence, safety) in conversions {
            if self.type_matches(found, &from_pattern) && self.type_matches(expected, &to_pattern) {
                let corrected_code = if conversion.contains("{}") {
                    conversion.replace("{}", original_code)
                } else if conversion.is_empty() {
                    original_code.clone() // Direct coercion
                } else {
                    format!("{original_code}{conversion}")
                };

                let mut proposal = CorrectionProposal::new(
                    original_code.clone(),
                    corrected_code,
                    confidence,
                    CorrectionStrategy::TypeConversion {
                        from_type: from_pattern.to_string(),
                        to_type: to_pattern.to_string(),
                        conversion_method: conversion.to_string(),
                    },
                );

                proposal.set_safety_level(safety);
                proposal.documentation_source = Some("Standard type conversions".to_string());
                proposal.add_metadata("expected_type", expected);
                proposal.add_metadata("found_type", found);
                proposal.add_metadata("conversion_method", conversion);

                proposals.push(proposal);
            }
        }

        Ok(proposals)
    }

    /// Get type conversion patterns
    fn get_type_conversion_patterns(&self) -> Vec<((&str, &str), &str, f64, SafetyLevel)> {
        vec![
            // String conversions
            (("&str", "String"), ".to_string()", 0.95, SafetyLevel::Safe),
            (("String", "&str"), ".as_str()", 0.95, SafetyLevel::Safe),
            (("&String", "&str"), "", 0.95, SafetyLevel::Safe), // Coercion
            (("str", "String"), ".to_string()", 0.95, SafetyLevel::Safe),
            // Option conversions
            (("T", "Option<T>"), "Some({})", 0.90, SafetyLevel::Safe),
            (
                ("Option<T>", "T"),
                ".unwrap()",
                0.70,
                SafetyLevel::RequiresReview,
            ),
            (
                ("Option<T>", "T"),
                ".expect(\"value\")",
                0.75,
                SafetyLevel::RequiresReview,
            ),
            // Result conversions
            (("T", "Result<T, E>"), "Ok({})", 0.85, SafetyLevel::Safe),
            (
                ("Result<T, E>", "T"),
                ".unwrap()",
                0.65,
                SafetyLevel::RequiresReview,
            ),
            (
                ("Result<T, E>", "T"),
                ".expect(\"success\")",
                0.70,
                SafetyLevel::RequiresReview,
            ),
            // Reference conversions
            (("T", "&T"), "&{}", 0.90, SafetyLevel::Safe),
            (("&T", "T"), ".clone()", 0.85, SafetyLevel::RequiresReview),
            (("&T", "T"), "*{}", 0.80, SafetyLevel::RequiresReview),
            // Numeric conversions
            (
                ("i32", "u32"),
                "{} as u32",
                0.75,
                SafetyLevel::RequiresReview,
            ),
            (
                ("u32", "i32"),
                "{} as i32",
                0.75,
                SafetyLevel::RequiresReview,
            ),
            (("i32", "f64"), "f64::from({})", 0.85, SafetyLevel::Safe),
            (("f32", "f64"), "f64::from({})", 0.90, SafetyLevel::Safe),
            // Collection conversions
            (("Vec<T>", "&[T]"), ".as_slice()", 0.90, SafetyLevel::Safe),
            (("&[T]", "Vec<T>"), ".to_vec()", 0.85, SafetyLevel::Safe),
            (("Vec<T>", "slice"), ".as_slice()", 0.90, SafetyLevel::Safe),
        ]
    }

    /// Generate corrections for unresolved names (E0425)
    async fn generate_unresolved_name_corrections(
        &self,
        context: &ASTContext,
    ) -> Result<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let message = &context.diagnostic_info.message;

        // Extract the unresolved name
        if let Some(captures) = REGEX_PATTERNS
            .get("variable_not_found")
            .and_then(|r| r.captures(message))
        {
            let unresolved_name = captures.get(1).map_or("", |m| m.as_str());

            // Check for similar variable names in scope
            for var in &context.surrounding_context.local_variables {
                let similarity = self.calculate_method_similarity(unresolved_name, &var.name);
                if similarity > crate::constants::DEFAULT_SIMILARITY_THRESHOLD {
                    let mut proposal = CorrectionProposal::new(
                        context.problematic_node.content.clone(),
                        var.name.clone(),
                        similarity * 0.9,
                        CorrectionStrategy::Generic {
                            description: format!(
                                "Variable name correction: {unresolved_name} -> {}",
                                var.name
                            ),
                        },
                    );

                    proposal.set_safety_level(SafetyLevel::Safe);
                    proposal.documentation_source = Some("Local variable scope".to_string());
                    proposal.add_metadata("original_name", unresolved_name);
                    proposal.add_metadata("suggested_name", var.name.clone());
                    proposal.add_metadata(
                        "variable_type",
                        var.var_type
                            .clone()
                            .unwrap_or_else(|| "unknown".to_string()),
                    );

                    proposals.push(proposal);
                }
            }

            // Check for similar type names
            for type_info in &context.surrounding_context.available_types {
                let similarity = self.calculate_method_similarity(unresolved_name, &type_info.name);
                if similarity > crate::constants::DEFAULT_SIMILARITY_THRESHOLD {
                    let mut proposal = CorrectionProposal::new(
                        context.problematic_node.content.clone(),
                        type_info.name.clone(),
                        similarity * 0.85,
                        CorrectionStrategy::Generic {
                            description: format!(
                                "Type name correction: {unresolved_name} -> {}",
                                type_info.name
                            ),
                        },
                    );

                    proposal.set_safety_level(SafetyLevel::Safe);
                    proposal.documentation_source = Some("Type scope".to_string());
                    proposal.add_metadata("original_name", unresolved_name);
                    proposal.add_metadata("suggested_name", type_info.name.clone());
                    proposal.add_metadata("type_kind", type_info.kind.clone());

                    proposals.push(proposal);
                }
            }
        }

        Ok(proposals)
    }

    /// Generate corrections for struct field errors
    async fn generate_struct_field_corrections(
        &self,
        context: &ASTContext,
    ) -> Result<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let message = &context.diagnostic_info.message;

        // Handle missing fields
        if let Some(captures) = REGEX_PATTERNS
            .get("missing_field")
            .and_then(|r| r.captures(message))
        {
            let field_name = captures.get(1).map_or("", |m| m.as_str());
            let struct_name = captures.get(2).map_or("", |m| m.as_str());

            let mut proposal = CorrectionProposal::new(
                context.problematic_node.content.clone(),
                format!(
                    "{}, {field_name}: Default::default()",
                    context.problematic_node.content
                ),
                0.8,
                CorrectionStrategy::StructFieldCorrection {
                    field_name: field_name.to_string(),
                    struct_name: struct_name.to_string(),
                    operation: "add_missing_field".to_string(),
                },
            );

            proposal.set_safety_level(SafetyLevel::RequiresReview);
            proposal.documentation_source = Some("Struct field analysis".to_string());
            proposal.add_metadata("struct_name", struct_name);
            proposal.add_metadata("missing_field", field_name);
            proposal.add_metadata("correction_type", "add_default_value");

            proposals.push(proposal);

            // Alternative: add todo!() for manual implementation
            let mut todo_proposal = CorrectionProposal::new(
                context.problematic_node.content.clone(),
                format!(
                    "{}, {field_name}: todo!(\"implement {field_name}\")",
                    context.problematic_node.content
                ),
                0.7,
                CorrectionStrategy::StructFieldCorrection {
                    field_name: field_name.to_string(),
                    struct_name: struct_name.to_string(),
                    operation: "add_todo_field".to_string(),
                },
            );

            todo_proposal.set_safety_level(SafetyLevel::RequiresReview);
            todo_proposal.add_metadata("correction_type", "add_todo_value");

            proposals.push(todo_proposal);
        }

        // Handle unknown fields
        if let Some(captures) = REGEX_PATTERNS
            .get("unknown_field")
            .and_then(|r| r.captures(message))
        {
            let field_name = captures.get(1).map_or("", |m| m.as_str());
            let type_name = captures.get(2).map_or("", |m| m.as_str());

            // Generate suggestions for similar field names
            let field_suggestions = self.generate_field_suggestions(field_name, type_name, context);
            for suggestion in field_suggestions {
                let mut proposal = CorrectionProposal::new(
                    context.problematic_node.content.clone(),
                    context
                        .problematic_node
                        .content
                        .replace(field_name, &suggestion.name),
                    suggestion.confidence,
                    CorrectionStrategy::FieldAccessCorrection {
                        original_field: field_name.to_string(),
                        suggested_field: suggestion.name.clone(),
                        type_name: type_name.to_string(),
                    },
                );

                proposal.set_safety_level(SafetyLevel::RequiresReview);
                proposal.documentation_source = Some("Field name analysis".to_string());
                proposal.add_metadata("original_field", field_name);
                proposal.add_metadata("suggested_field", suggestion.name);
                proposal.add_metadata("suggestion_reason", suggestion.description);

                proposals.push(proposal);
            }
        }

        Ok(proposals)
    }

    /// Generate field name suggestions
    fn generate_field_suggestions(
        &self,
        field_name: &str,
        _type_name: &str,
        _context: &ASTContext,
    ) -> Vec<FieldSuggestion> {
        // Common field name patterns and corrections
        let common_corrections = vec![
            ("lenght", "length", 0.95),
            ("widht", "width", 0.95),
            ("heigth", "height", 0.95),
            ("vlaue", "value", 0.95),
            ("naem", "name", 0.95),
            ("tpye", "type", 0.95),
        ];

        let mut suggestions = Vec::new();

        for (typo, correction, confidence) in common_corrections {
            if field_name.contains(typo) {
                let corrected = field_name.replace(typo, correction);
                suggestions.push(FieldSuggestion::new(
                    corrected,
                    confidence,
                    format!("Common typo correction: {typo} -> {correction}"),
                ));
            }
        }

        // If no specific corrections found, generate phonetic suggestions
        if suggestions.is_empty() {
            suggestions.extend(self.generate_phonetic_suggestions(field_name));
        }

        suggestions
    }

    /// Generate phonetic suggestions for field names
    fn generate_phonetic_suggestions(&self, field_name: &str) -> Vec<FieldSuggestion> {
        let common_fields = vec![
            "id",
            "name",
            "value",
            "data",
            "type",
            "kind",
            "size",
            "length",
            "width",
            "height",
            "count",
            "index",
            "key",
            "item",
            "element",
            "content",
            "text",
            "title",
            "description",
        ];

        common_fields
            .iter()
            .filter_map(|&common_field| {
                let similarity = self.calculate_method_similarity(field_name, common_field);
                if similarity > 0.6 {
                    Some(FieldSuggestion::new(
                        common_field.to_string(),
                        similarity,
                        format!("Phonetic similarity to common field: {common_field}"),
                    ))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Generates corrections for various common error patterns
    async fn generate_generic_corrections(
        &self,
        context: &ASTContext,
    ) -> Result<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let message = &context.diagnostic_info.message;

        // Borrowing and lifetime corrections
        if REGEX_PATTERNS
            .get("borrowing_error")
            .map_or(false, |r| r.is_match(message))
            || REGEX_PATTERNS
                .get("lifetime_error")
                .map_or(false, |r| r.is_match(message))
        {
            proposals.extend(self.generate_borrowing_corrections(context).await);
        }

        // Unused import corrections
        if let Some(regex) = REGEX_PATTERNS.get("unused_import") {
            if let Some(captures) = regex.captures(message) {
                let import_path = captures.get(1).map_or("", |m| m.as_str());
                let mut proposal = CorrectionProposal::new(
                    context.problematic_node.content.clone(),
                    String::new(), // Remove the import
                    0.95,
                    CorrectionStrategy::Generic {
                        description: format!("Remove unused import: {import_path}"),
                    },
                );

                proposal.set_safety_level(SafetyLevel::Safe);
                proposal.documentation_source = Some("Unused import cleanup".to_string());
                proposal.add_metadata("import_path", import_path);
                proposal.add_metadata("action", "remove_import");

                proposals.push(proposal);
            }
        }

        Ok(proposals)
    }

    /// Generate borrowing-related corrections
    async fn generate_borrowing_corrections(
        &self,
        context: &ASTContext,
    ) -> Vec<CorrectionProposal> {
        let mut proposals = Vec::new();
        let original_code = &context.problematic_node.content;

        // Try adding a reference
        let mut ref_proposal = CorrectionProposal::new(
            original_code.clone(),
            format!("&{original_code}"),
            0.8,
            CorrectionStrategy::BorrowingCorrection {
                operation: "add_reference".to_string(),
            },
        );
        ref_proposal.set_safety_level(SafetyLevel::Safe);
        ref_proposal.add_metadata("operation", "add_reference");
        proposals.push(ref_proposal);

        // Try cloning if it's a move issue
        if context.diagnostic_info.message.contains("move") {
            let mut clone_proposal = CorrectionProposal::new(
                original_code.clone(),
                format!("{original_code}.clone()"),
                0.75,
                CorrectionStrategy::BorrowingCorrection {
                    operation: "clone_value".to_string(),
                },
            );
            clone_proposal.set_safety_level(SafetyLevel::RequiresReview);
            clone_proposal.add_metadata("operation", "clone_value");
            proposals.push(clone_proposal);
        }

        proposals
    }

    /// Heuristically checks if a type string matches a pattern
    fn type_matches(&self, actual: &str, pattern: &str) -> bool {
        let (pattern_base, pattern_generic) = pattern.split_once('<').unwrap_or((pattern, ""));
        let (actual_base, actual_generic) = actual.split_once('<').unwrap_or((actual, ""));

        if pattern_base != "T" && pattern_base != actual_base {
            return false;
        }
        if pattern_generic.is_empty() {
            return true;
        }

        let pattern_generic_inner = &pattern_generic[..pattern_generic.len().saturating_sub(1)];
        let actual_generic_inner = &actual_generic[..actual_generic.len().saturating_sub(1)];

        self.type_matches(actual_generic_inner, pattern_generic_inner)
    }

    /// Calculates similarity between two method names
    pub fn calculate_method_similarity(&self, a: &str, b: &str) -> f64 {
        let levenshtein = self.levenshtein_similarity(a, b);
        let jaro_winkler = self.jaro_winkler_similarity(a, b);
        let common_prefix = self.common_prefix_similarity(a, b);
        0.5 * levenshtein + 0.3 * jaro_winkler + 0.2 * common_prefix
    }

    /// Levenshtein similarity calculation
    fn levenshtein_similarity(&self, a: &str, b: &str) -> f64 {
        let (a_len, b_len) = (a.chars().count(), b.chars().count());
        if a_len == 0 {
            return if b_len == 0 { 1.0 } else { 0.0 };
        }
        if b_len == 0 {
            return 0.0;
        }
        let mut column: Vec<usize> = (0..=a_len).collect();
        for (_j, b_char) in b.chars().enumerate() {
            let mut last_diag = column[0];
            column[0] += 1;
            for (i, a_char) in a.chars().enumerate() {
                let old_diag = column[i + 1];
                let cost = if a_char == b_char { 0 } else { 1 };
                column[i + 1] = (column[i + 1] + 1).min(column[i] + 1).min(last_diag + cost);
                last_diag = old_diag;
            }
        }
        let distance = column[a_len];
        1.0 - (distance as f64 / a_len.max(b_len) as f64)
    }

    /// Jaro-Winkler similarity calculation
    fn jaro_winkler_similarity(&self, a: &str, b: &str) -> f64 {
        if a == b {
            return 1.0;
        }
        let (a_len, b_len) = (a.len(), b.len());
        if a_len == 0 || b_len == 0 {
            return 0.0;
        }

        let common_prefix = a
            .chars()
            .zip(b.chars())
            .take(4)
            .take_while(|(c1, c2)| c1 == c2)
            .count();
        let common_chars = a.chars().filter(|&c| b.contains(c)).count();
        let jaro = common_chars as f64 / a_len.max(b_len) as f64;

        jaro + (0.1 * common_prefix as f64 * (1.0 - jaro))
    }

    /// Common prefix similarity calculation
    fn common_prefix_similarity(&self, a: &str, b: &str) -> f64 {
        let common_prefix = a
            .chars()
            .zip(b.chars())
            .take_while(|(c1, c2)| c1 == c2)
            .count();
        let max_len = a.len().max(b.len());
        if max_len == 0 {
            1.0
        } else {
            common_prefix as f64 / max_len as f64
        }
    }

    /// Get generation metrics
    #[must_use]
    pub fn metrics(&self) -> &GenerationMetrics {
        &self.metrics
    }

    /// Get validation statistics
    #[must_use]
    pub fn validation_stats(&self) -> ValidationStats {
        self.validator.validation_stats()
    }

    /// Clear template cache
    pub async fn clear_template_cache(&self) {
        let mut cache = self.template_cache.write().await;
        cache.clear();
    }

    /// Get template cache statistics
    pub async fn template_cache_stats(&self) -> TemplateCacheStats {
        let cache = self.template_cache.read().await;
        let total_usage: u64 = cache.values().map(|t| t.usage_count).sum();

        TemplateCacheStats {
            cache_size: cache.len(),
            total_usage,
            most_used_templates: {
                let mut templates: Vec<_> = cache
                    .iter()
                    .map(|(name, template)| (name.clone(), template.usage_count))
                    .collect();
                templates.sort_by(|a, b| b.1.cmp(&a.1));
                templates.truncate(10);
                templates
            },
        }
    }
}

impl Default for CodeGenerationEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Field suggestion helper
use crate::types::FieldSuggestion;

impl FieldSuggestion {
    /// Create new field suggestion
    pub fn new(name: impl Into<String>, confidence: f64, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            confidence,
            description: description.into(),
        }
    }
}

/// Template cache statistics
#[derive(Debug, Clone)]
pub struct TemplateCacheStats {
    /// Current cache size
    pub cache_size: usize,
    /// Total template usage count
    pub total_usage: u64,
    /// Most frequently used templates
    pub most_used_templates: Vec<(String, u64)>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{CompilerDiagnostic, DiagnosticLevel};
    use std::path::PathBuf;

    fn create_test_context() -> ASTContext {
        ASTContext {
            file_path: PathBuf::from("test.rs"),
            problematic_node: crate::ast::NodeInfo::new(
                crate::ast::NodeType::MethodCall {
                    receiver: "test_var".to_string(),
                    method_name: "len".to_string(),
                    args: vec![],
                    receiver_type: Some("String".to_string()),
                },
                "test_var.len()".to_string(),
                (10, 20),
                (1, 1),
            ),
            surrounding_context: crate::ast::SurroundingContext::default(),
            diagnostic_info: CompilerDiagnostic::new(
                "test_diagnostic",
                "no method named `len` found for type `String`",
                DiagnosticLevel::Error,
            ),
            source_map: None,
        }
    }

    #[test]
    fn test_code_generator_creation() {
        let generator = CodeGenerationEngine::new();
        assert_eq!(
            generator
                .metrics()
                .corrections_generated
                .load(Ordering::Relaxed),
            0
        );
    }

    #[test]
    fn test_method_similarity_calculation() {
        let generator = CodeGenerationEngine::new();

        // Test identical strings
        assert_eq!(generator.calculate_method_similarity("test", "test"), 1.0);

        // Test similar strings
        let sim1 = generator.calculate_method_similarity("method_name", "method_nam");
        assert!(sim1 > 0.8);

        // Test different strings
        let sim2 = generator.calculate_method_similarity("completely", "different");
        assert!(sim2 < 0.5);
    }

    #[test]
    fn test_type_matching() {
        let generator = CodeGenerationEngine::new();

        assert!(generator.type_matches("String", "String"));
        assert!(generator.type_matches("Vec<i32>", "Vec<T>"));
        assert!(generator.type_matches("Option<String>", "Option<T>"));
        assert!(!generator.type_matches("String", "i32"));
    }

    #[tokio::test]
    async fn test_template_initialization() {
        let generator = CodeGenerationEngine::new();

        // Wait a bit for async initialization
        tokio::time::sleep(Duration::from_millis(100)).await;

        let stats = generator.template_cache_stats().await;
        assert!(stats.cache_size > 0);
    }

    #[test]
    fn test_code_validation() {
        let mut validator = CodeValidator::new();

        // Test valid code
        assert!(validator.validate_syntax("let x = 5;").is_ok());
        assert!(validator.validate_syntax("println!(\"Hello\")").is_ok());

        // Test invalid code
        assert!(validator.validate_syntax("let x = ;").is_err());
        assert!(validator.validate_syntax("fn incomplete").is_err());

        let stats = validator.validation_stats();
        assert!(stats.total_validations > 0);
    }

    #[test]
    fn test_field_suggestions() {
        let generator = CodeGenerationEngine::new();
        let context = create_test_context();

        let suggestions = generator.generate_field_suggestions("lenght", "TestStruct", &context);
        assert!(!suggestions.is_empty());

        // Should suggest "length" for "lenght"
        let length_suggestion = suggestions.iter().find(|s| s.name == "length");
        assert!(length_suggestion.is_some());
        assert!(length_suggestion.unwrap().confidence > 0.9);
    }

    #[test]
    fn test_phonetic_suggestions() {
        let generator = CodeGenerationEngine::new();

        let suggestions = generator.generate_phonetic_suggestions("naem");
        assert!(!suggestions.is_empty());

        // Should suggest "name" for "naem"
        let name_suggestion = suggestions.iter().find(|s| s.name == "name");
        assert!(name_suggestion.is_some());
    }

    #[tokio::test]
    async fn test_correction_generation() {
        let generator = CodeGenerationEngine::new();
        let context = create_test_context();

        let corrections = generator.generate_corrections(&context, None).await;
        assert!(corrections.is_ok());

        let proposals = corrections.unwrap();
        // Should generate at least some correction proposals
        assert!(!proposals.is_empty());
    }

    #[test]
    fn test_template_operations() {
        let mut template =
            CorrectionTemplate::new("test_pattern", "test_replacement", 0.9, SafetyLevel::Safe);

        assert_eq!(template.confidence, 0.9);
        assert_eq!(template.usage_count, 0);

        template.use_template();
        assert_eq!(template.usage_count, 1);

        let effectiveness = template.effectiveness_score();
        assert!(effectiveness > 0.0);
    }
}
