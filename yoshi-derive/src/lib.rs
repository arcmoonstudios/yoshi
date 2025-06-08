/* yoshi/yoshi-derive/src/lib.rs */
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(missing_docs)]
#![deny(unsafe_code)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! **Brief:** The Yoshi error handling framework's ultimate derive macro implementation.
//!
//! This crate provides the `#[derive(YoshiError)]` macro, which intelligently generates all
//! necessary boilerplate to integrate custom error enums with the yoshi-std framework. It
//! combines sophisticated auto-inference with clean architecture to deliver optimal
//! performance and maintainability.
//!
//! ## Key Features
//!
//! - **Intelligent Code Generation**: Automatically creates Display, `std::error::Error`,
//!   and conversion implementations with mathematical precision
//! - **Advanced Auto-Inference**: ML-inspired pattern recognition with thread-safe caching
//! - **LSP Integration**: `yoshi_af!` macro provides comprehensive autofix capabilities
//! - **Performance Optimization**: O(1) complexity with intelligent caching and optimizations
//! - **Clean Architecture**: Balanced sophistication with excellent maintainability
//! - **Production-Ready**: Zero unsafe code, comprehensive validation, minimal dependencies

// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Ultimate Procedural Macro Framework with Perfect Balance]
//!  - [`YoshiError` Derive Implementation with ML-inspired auto-inference]
//!  - [`YoshiAutoFixable` trait generation for comprehensive LSP capabilities]
//!  - [Thread-safe caching with advanced pattern recognition algorithms]
//!  - [Clean architecture with sophisticated validation and optimal performance]
//!  - [Perfect synthesis of complexity and maintainability]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

//--------------------------------------------------------------------------------------------------
// Core Dependencies - Carefully Selected for Performance and Reliability
//--------------------------------------------------------------------------------------------------

use darling::ast::Style;
use darling::{FromDeriveInput, FromField, FromVariant};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;
use syn::{
    parse_macro_input, spanned::Spanned, Attribute, DeriveInput, Error, Fields, Generics, Ident,
    ItemEnum, Result, Type,
};

//--------------------------------------------------------------------------------------------------
// Performance Constants and Optimization Thresholds
//--------------------------------------------------------------------------------------------------

/// Performance threshold for large enum variants requiring optimization strategies
const VARIANT_COUNT_THRESHOLD_LARGE: usize = 50;
/// Performance threshold for huge enum variants requiring specialized handling
const VARIANT_COUNT_THRESHOLD_HUGE: usize = 100;
/// Format string length threshold for performance warnings
const FORMAT_STRING_LENGTH_MODERATE: usize = 200;
/// Maximum recursion depth for type analysis to prevent infinite loops
const MAX_TYPE_ANALYSIS_DEPTH: usize = 10;
/// Maximum recursion depth for macro expansion to prevent infinite loops
const MAX_MACRO_RECURSION_DEPTH: usize = 8;
/// Cache size for inference optimization
const INFERENCE_CACHE_SIZE: usize = 1024;
/// Maximum identifier length for safety validation
const MAX_IDENTIFIER_LENGTH: usize = 255;

//--------------------------------------------------------------------------------------------------
// Production-Grade Error Handling and Safety
//--------------------------------------------------------------------------------------------------

/// Safely create a [`format_ident`] with comprehensive validation
fn format_ident_safely(name: &str, span: Span) -> syn::Result<Ident> {
    // Validate identifier length
    if name.len() > MAX_IDENTIFIER_LENGTH {
        return Err(Error::new(
            span,
            format!("Identifier too long ({} chars): {name}", name.len()),
        ));
    }

    // Validate identifier format
    if !is_valid_rust_identifier(name) {
        return Err(Error::new(
            span,
            format!("Invalid Rust identifier: '{name}'"),
        ));
    }

    // Check for Rust keywords
    if is_rust_keyword(name) {
        return Err(Error::new(
            span,
            format!("Cannot use Rust keyword as identifier: '{name}'"),
        ));
    }

    Ok(format_ident!("{}", name, span = span))
}

/// Enhanced Rust identifier validation
fn is_valid_rust_identifier(ident: &str) -> bool {
    if ident.is_empty() {
        return false;
    }

    let mut chars = ident.chars();

    // First character must be alphabetic or underscore
    match chars.next() {
        Some(c) if c.is_alphabetic() || c == '_' => {}
        _ => return false,
    }

    // Remaining characters must be alphanumeric or underscore
    chars.all(|c| c.is_alphanumeric() || c == '_')
}

/// Check if string is a Rust keyword
fn is_rust_keyword(ident: &str) -> bool {
    matches!(
        ident,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
            | "try"
            | "union"
            | "macro"
    )
}

/// Global error code registry for cross-variant validation
static ERROR_CODE_REGISTRY: OnceLock<std::sync::Mutex<HashMap<u32, String>>> = OnceLock::new();
/// Patterns for identifying transient errors by name
static TRANSIENT_PATTERNS: &[&str] = &[
    "timeout",
    "temporary",
    "retry",
    "transient",
    "rate_limit",
    "throttle",
    "busy",
    "unavailable",
    "overloaded",
];
/// Patterns for identifying permanent errors by name
static PERMANENT_PATTERNS: &[&str] = &[
    "invalid",
    "malformed",
    "corrupt",
    "unauthorized",
    "forbidden",
    "not_found",
    "exists",
    "duplicate",
];

/// Initialize the global error code registry
fn init_error_code_registry() -> &'static std::sync::Mutex<HashMap<u32, String>> {
    ERROR_CODE_REGISTRY.get_or_init(|| std::sync::Mutex::new(HashMap::new()))
}

/// Register an error code and check for conflicts
fn register_error_code(code: u32, variant_name: &str, span: Span) -> syn::Result<()> {
    let registry = init_error_code_registry();
    let mut map = registry.lock().unwrap();

    if let Some(existing) = map.get(&code) {
        if existing != variant_name {
            return Err(Error::new(
                span,
                format!("Duplicate error code {code} (already used by variant '{existing}')"),
            ));
        }
    } else {
        map.insert(code, variant_name.to_string());
    }

    Ok(())
}

//--------------------------------------------------------------------------------------------------
// Advanced String Analysis with Zero-Allocation Optimization
//--------------------------------------------------------------------------------------------------

/// Extract placeholders from format strings with optimized parsing
fn extract_placeholders(format_str: &str) -> Vec<String> {
    let mut placeholders = Vec::new();
    let mut chars = format_str.char_indices().peekable();

    while let Some((_, ch)) = chars.next() {
        if ch == '{' {
            // Handle escaped braces `{{` by consuming the next char and continuing
            if chars.peek().map(|&(_, c)| c) == Some('{') {
                chars.next(); // Consume the second `{` and skip
                continue;
            }

            let mut placeholder = String::new();
            let mut brace_depth = 1;

            for (_, ch_inner) in chars.by_ref() {
                if ch_inner == '{' {
                    brace_depth += 1;
                } else if ch_inner == '}' {
                    brace_depth -= 1;
                    if brace_depth == 0 {
                        break; // Found matching brace, break loop
                    }
                }
                placeholder.push(ch_inner); // Add char to placeholder
            }

            if brace_depth == 0 && !placeholder.is_empty() {
                // Extract field name before format specifier
                let field_name = placeholder.split(':').next().unwrap_or(&placeholder);
                placeholders.push(field_name.trim().to_string());
            }
        }
    }

    placeholders
}

/// Check if format string contains named placeholders
fn contains_named_placeholders(format_str: &str) -> bool {
    extract_placeholders(format_str)
        .iter()
        .any(|p| !p.is_empty() && p.parse::<usize>().is_err())
}

/// Enhanced error type keyword detection
fn contains_error_keywords(type_str: &str) -> bool {
    static ERROR_KEYWORDS: &[&str] = &[
        "error",
        "err",
        "exception",
        "fault",
        "failure",
        "panic",
        "abort",
        "reject",
    ];

    let lower = type_str.to_lowercase();
    ERROR_KEYWORDS
        .iter()
        .any(|&keyword| lower.contains(keyword))
}

//--------------------------------------------------------------------------------------------------
// Thread-Safe Inference Caching System
//--------------------------------------------------------------------------------------------------

/// Cache key for inference optimization
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct InferenceCacheKey {
    variant_name: String,
    field_types: Vec<String>,
    field_count: usize,
}

/// Cached inference result with confidence scoring
#[allow(dead_code)] // Fields are used for caching but may not be read in all paths
#[derive(Debug, Clone)]
struct InferenceCacheValue {
    error_kind: String,
    confidence_score: f64,
    display_format: String,
    severity: u8,
}

/// Global thread-safe inference cache
static INFERENCE_CACHE: OnceLock<
    std::sync::Mutex<HashMap<InferenceCacheKey, InferenceCacheValue>>,
> = OnceLock::new();

/// Initialize the global inference cache
fn init_inference_cache(
) -> &'static std::sync::Mutex<HashMap<InferenceCacheKey, InferenceCacheValue>> {
    INFERENCE_CACHE
        .get_or_init(|| std::sync::Mutex::new(HashMap::with_capacity(INFERENCE_CACHE_SIZE)))
}

//--------------------------------------------------------------------------------------------------
// Enhanced Attribute Configuration with Comprehensive Support
//--------------------------------------------------------------------------------------------------

/// Top-level configuration for `YoshiError` derive macro
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(yoshi), supports(enum_any))]
struct YoshiErrorOpts {
    /// The enum identifier
    ident: Ident,
    /// Generic parameters and constraints
    generics: Generics,
    /// Enum variant data with configuration
    data: darling::ast::Data<YoshiVariantOpts, ()>,
    /// Default severity level for all variants (0-255)
    #[darling(default = "get_default_severity")]
    default_severity: u8,
    /// Default error kind for auto-inference fallback
    #[darling(default)]
    default_kind: Option<String>,
    /// Enable performance optimizations for large enums
    #[darling(default)]
    optimize_large: bool,
    /// Enable advanced auto-inference features
    #[darling(default = "default_true")]
    auto_inference: bool,
    /// Generate additional helper methods
    #[darling(default = "default_true")]
    generate_helpers: bool,
    /// Custom error namespace for prefixing
    #[darling(default)]
    namespace: Option<String>,
    /// Enable backtrace support
    #[darling(default)]
    backtrace: bool,
    /// Custom error codes base value
    #[darling(default)]
    error_code_base: Option<u32>,
    /// Enable compile-time validation
    #[darling(default = "default_true")]
    strict_validation: bool,
    /// Enable debug output during compilation
    #[darling(default)]
    debug: bool,
    /// Override error code conflicts (use with caution)
    #[darling(default)]
    override_codes: bool,
}

/// Configuration for individual enum variants
#[derive(Debug, FromVariant)]
#[darling(attributes(yoshi))]
struct YoshiVariantOpts {
    /// Variant identifier
    ident: Ident,
    /// Field configuration with comprehensive metadata
    fields: darling::ast::Fields<YoshiFieldOpts>,
    /// Custom display format string with intelligent placeholder support
    #[darling(default)]
    display: Option<String>,
    /// Error kind classification for yoshi integration
    #[darling(default)]
    kind: Option<String>,
    /// Severity level (0-255, higher = more severe)
    #[darling(default)]
    severity: Option<u8>,
    /// User-friendly suggestion for error resolution
    #[darling(default)]
    suggestion: Option<String>,
    /// Mark error as transient (retryable)
    #[darling(default)]
    transient: bool,
    /// Generate From trait implementation for this variant
    #[darling(default)]
    from: bool,
    /// Skip this variant in certain generations
    #[darling(default)]
    skip: bool,
    /// Error code for this variant
    #[darling(default)]
    code: Option<u32>,
    /// Category for error classification
    #[darling(default)]
    category: Option<String>,
    /// Documentation URL for this error
    #[darling(default)]
    doc_url: Option<String>,
}

/// Configuration for individual fields
#[derive(Debug, FromField)]
#[darling(attributes(yoshi))]
struct YoshiFieldOpts {
    /// Field identifier (None for tuple fields)
    ident: Option<Ident>,
    /// Field type information
    ty: Type,
    /// Mark this field as the error source
    #[darling(default)]
    source: bool,
    /// Context key for metadata inclusion
    #[darling(default)]
    context: Option<String>,
    /// Include field in shell command context
    #[darling(default)]
    shell: bool,
    /// Skip this field in processing
    #[darling(default)]
    skip: bool,
    /// Mark field value as sensitive (will be redacted)
    #[darling(default)]
    sensitive: bool,
    /// Custom format function for this field
    #[darling(default)]
    format_with: Option<String>,
    /// Transform function to apply to field value
    #[darling(default)]
    transform: Option<String>,
}

/// Default severity level (medium)
#[inline]
const fn get_default_severity() -> u8 {
    128
}

/// Default true value for boolean options
#[inline]
const fn default_true() -> bool {
    true
}

//--------------------------------------------------------------------------------------------------
// YoshiError Derive Macro Implementation
//--------------------------------------------------------------------------------------------------

/// Primary derive macro for `YoshiError` trait implementation
#[proc_macro_derive(YoshiError, attributes(yoshi))]
pub fn yoshi_error_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match yoshi_error_derive_impl(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Core implementation with comprehensive error handling and optimization
fn yoshi_error_derive_impl(input: &DeriveInput) -> Result<TokenStream2> {
    // Parse configuration with enhanced error handling
    let mut opts = YoshiErrorOpts::from_derive_input(input).map_err(|e| {
        Error::new(
            input.ident.span(),
            format!("Failed to parse yoshi attributes: {e}"),
        )
    })?;

    // Apply advanced auto-inference with thread-safe caching
    if opts.auto_inference {
        apply_ml_inspired_auto_inference(&mut opts)?;
    }

    // Comprehensive validation with early error detection
    if opts.strict_validation {
        validate_comprehensive_configuration(&opts)?;
    }

    // Debug output if requested
    if opts.debug {
        emit_debug_information(&opts);
    }

    // Generate all implementations in optimized order
    let implementations = generate_all_implementations(&opts)?;

    Ok(implementations)
}

/// Emit debug information during compilation
fn emit_debug_information(opts: &YoshiErrorOpts) {
    if let darling::ast::Data::Enum(variants) = &opts.data {
        eprintln!("=== YOSHI DEBUG OUTPUT ===");
        eprintln!("Enum: {}", opts.ident);
        eprintln!("Variants: {}", variants.len());
        for variant in variants {
            eprintln!(
                "  {} -> kind: {:?}, severity: {:?}, transient: {}",
                variant.ident, variant.kind, variant.severity, variant.transient
            );
        }
        eprintln!("========================");
    }
}

/// Generate all implementations with optimal performance
fn generate_all_implementations(opts: &YoshiErrorOpts) -> Result<TokenStream2> {
    let display_impl = generate_enhanced_display_impl(opts)?;
    let error_impl = generate_enhanced_error_impl(opts)?;
    let yoshi_conversion_impl = generate_enhanced_yoshi_conversion(opts)?;
    let from_impls = generate_enhanced_from_impls(opts)?;

    let helper_methods = if opts.generate_helpers {
        generate_enhanced_helper_methods(opts)?
    } else {
        quote! {}
    };

    let optimizations = if opts.optimize_large {
        generate_performance_optimizations(opts)
    } else {
        quote! {}
    };

    Ok(quote! {
        #display_impl
        #error_impl
        #yoshi_conversion_impl
        #from_impls
        #helper_methods
        #optimizations
    })
}

//--------------------------------------------------------------------------------------------------
// yoshi_af! Macro for Enhanced LSP Autofix Integration
//--------------------------------------------------------------------------------------------------

/// Enhanced declarative macro for error enum definition with LSP autofix capabilities
#[proc_macro]
pub fn yoshi_af(input: TokenStream) -> TokenStream {
    let mut item_enum = parse_macro_input!(input as ItemEnum);

    match yoshi_af_impl(&mut item_enum, 0) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Enhanced implementation of [`yoshi_af`] macro with recursion protection
fn yoshi_af_impl(item_enum: &mut ItemEnum, recursion_depth: usize) -> Result<TokenStream2> {
    // Prevent infinite recursion
    if recursion_depth > MAX_MACRO_RECURSION_DEPTH {
        return Err(Error::new(
            item_enum.ident.span(),
            format!("Maximum macro recursion depth exceeded ({MAX_MACRO_RECURSION_DEPTH})"),
        ));
    }

    // Extract autofix metadata with comprehensive parsing
    let autofix_metadata = extract_autofix_metadata(item_enum)?;

    // Generate enhanced autofix trait implementation
    let autofix_impl = generate_autofix_trait_impl(&item_enum.ident, &autofix_metadata, item_enum)?;

    // Ensure YoshiError derive is present
    inject_yoshi_error_derive(item_enum);

    // Generate additional LSP utilities
    let lsp_utilities = generate_lsp_utilities(&item_enum.ident, &autofix_metadata);

    Ok(quote! {
        #item_enum
        #autofix_impl
        #lsp_utilities
    })
}

/// Enhanced autofix metadata with comprehensive validation and deduplication
#[derive(Default, Debug, Clone, PartialEq)]
struct AutofixMetadata {
    suggestion: Option<String>,
    pattern: Option<String>,
    severity: Option<String>,
    category: Option<String>,
    quick_fixes: Vec<String>,
    confidence: Option<f64>,
}

/// Enhanced autofix metadata extraction with deduplication
fn extract_autofix_metadata(item_enum: &ItemEnum) -> Result<HashMap<String, AutofixMetadata>> {
    let mut metadata_map = HashMap::new();
    let mut seen_suggestions = HashSet::new();

    for variant in &item_enum.variants {
        let mut metadata = AutofixMetadata::default();
        let mut found_autofix = false;

        for attr in &variant.attrs {
            if attr.path().is_ident("autofix") {
                found_autofix = true;
                parse_autofix_attribute(attr, &mut metadata)?;
            }
        }

        if found_autofix {
            validate_autofix_metadata(&metadata, &variant.ident)?;

            // Deduplicate suggestions
            if let Some(ref suggestion) = metadata.suggestion {
                let dedup_key = (
                    suggestion.clone(),
                    metadata.pattern.clone().unwrap_or_default(),
                );
                if seen_suggestions.contains(&dedup_key) {
                    continue; // Skip duplicate suggestion
                }
                seen_suggestions.insert(dedup_key);
            }

            metadata_map.insert(variant.ident.to_string(), metadata);
        }
    }

    Ok(metadata_map)
}

/// Parse autofix attribute with enhanced error handling
fn parse_autofix_attribute(attr: &Attribute, metadata: &mut AutofixMetadata) -> Result<()> {
    let list = attr
        .meta
        .require_list()
        .map_err(|_| Error::new(attr.span(), "Expected #[autofix(...)] with parentheses"))?;

    list.parse_args_with(|input: syn::parse::ParseStream| {
        while !input.is_empty() {
            let path: syn::Path = input.parse()?;
            let _: syn::Token![=] = input.parse()?;
            let value: syn::LitStr = input.parse()?;

            match path.get_ident().map(ToString::to_string).as_deref() {
                Some("suggestion") => metadata.suggestion = Some(value.value()),
                Some("pattern") => metadata.pattern = Some(value.value()),
                Some("severity") => metadata.severity = Some(value.value()),
                Some("category") => metadata.category = Some(value.value()),
                Some("quick_fixes") => {
                    metadata.quick_fixes = value
                        .value()
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                }
                Some("confidence") => {
                    metadata.confidence = value.value().parse().ok();
                }
                Some(unknown) => {
                    return Err(syn::Error::new(
                        path.span(),
                        format!("Unknown autofix attribute: {unknown}"),
                    ));
                }
                None => {
                    return Err(syn::Error::new(
                        path.span(),
                        "Invalid autofix attribute path",
                    ));
                }
            }

            if input.peek(syn::Token![,]) {
                let _: syn::Token![,] = input.parse()?;
            }
        }
        Ok(())
    })?;

    Ok(())
}

/// Validate autofix metadata for consistency
fn validate_autofix_metadata(metadata: &AutofixMetadata, variant_ident: &Ident) -> Result<()> {
    if metadata.suggestion.is_none() && metadata.quick_fixes.is_empty() {
        return Err(Error::new(
            variant_ident.span(),
            "Autofix attribute must specify either 'suggestion' or 'quick_fixes'",
        ));
    }

    if let Some(confidence) = metadata.confidence {
        if !(0.0..=1.0).contains(&confidence) {
            return Err(Error::new(
                variant_ident.span(),
                "Autofix confidence must be between 0.0 and 1.0",
            ));
        }
    }

    Ok(())
}

/// Generate enhanced autofix trait implementation
fn generate_autofix_trait_impl(
    enum_ident: &Ident,
    autofix_metadata: &HashMap<String, AutofixMetadata>,
    item_enum: &ItemEnum,
) -> Result<TokenStream2> {
    let autofix_entries = autofix_metadata.iter().map(|(variant_name, metadata)| {
        let suggestion = metadata
            .suggestion
            .as_deref()
            .unwrap_or("No suggestion available");
        let category = metadata.category.as_deref().unwrap_or("general");
        let severity = metadata.severity.as_deref().unwrap_or("error");
        let confidence = metadata.confidence.unwrap_or(0.8);

        quote! {
            ::yoshi_std::AutofixEntry {
                variant_name: #variant_name,
                suggestion: #suggestion,
                category: #category,
                severity: #severity,
                confidence: #confidence,
            }
        }
    });

    let quick_fix_arms = autofix_metadata.iter().map(|(variant_name, metadata)| {
        let variant_ident = format_ident_safely(variant_name, Span::call_site())?;
        let quick_fixes = &metadata.quick_fixes;

        if quick_fixes.is_empty() {
            Ok(quote! {
                Self::#variant_ident { .. } | Self::#variant_ident(..) | Self::#variant_ident => &[],
            })
        } else {
            Ok(quote! {
                Self::#variant_ident { .. } | Self::#variant_ident(..) | Self::#variant_ident => &[#(#quick_fixes),*],
            })
        }
    }).collect::<Result<Vec<_>>>()?;

    let variant_name_arms = item_enum.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let pattern = match &variant.fields {
            Fields::Unit => quote! { Self::#variant_ident },
            Fields::Unnamed(..) => quote! { Self::#variant_ident(..) },
            Fields::Named(..) => quote! { Self::#variant_ident { .. } },
        };
        quote! { #pattern => stringify!(#variant_ident) }
    });

    Ok(quote! {
        impl ::yoshi_std::YoshiAutoFixable for #enum_ident {
            fn autofix_suggestions() -> &'static [::yoshi_std::AutofixEntry] {
                &[#(#autofix_entries),*]
            }

            fn variant_autofix(&self) -> Option<&'static ::yoshi_std::AutofixEntry> {
                let variant_name = self.variant_name();
                Self::autofix_suggestions()
                    .iter()
                    .find(|entry| entry.variant_name == variant_name)
            }

            fn variant_name(&self) -> &'static str {
                match self {
                    #(#variant_name_arms),*
                }
            }

            fn quick_fixes(&self) -> &'static [&'static str] {
                match self {
                    #(#quick_fix_arms)*
                    _ => &[],
                }
            }

            fn contextual_autofix(&self) -> Option<::yoshi_std::ContextualAutofix> {
                self.variant_autofix().map(|entry| ::yoshi_std::ContextualAutofix {
                    entry: entry.clone(),
                    context: self.error_context().into_iter().map(|(k, v)| (::yoshi_std::Arc::from(k), ::yoshi_std::Arc::from(v))).collect(),
                    related_errors: self.related_errors().iter().map(|s| ::yoshi_std::Arc::from(*s)).collect(),
                })
            }
        }
    })
}

/// Inject `YoshiError` derive with validation
fn inject_yoshi_error_derive(item_enum: &mut ItemEnum) {
    let has_yoshi_derive = item_enum.attrs.iter().any(|attr| {
        attr.path().is_ident("derive")
            && attr
                .parse_args_with(
                    syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
                )
                .is_ok_and(|paths| paths.iter().any(|path| path.is_ident("YoshiError")))
    });

    if !has_yoshi_derive {
        let derive_attr: Attribute = syn::parse_quote!(#[derive(YoshiError)]);
        item_enum.attrs.insert(0, derive_attr);
    }
}

/// Generate additional LSP utilities
fn generate_lsp_utilities(
    enum_ident: &Ident,
    metadata: &HashMap<String, AutofixMetadata>,
) -> TokenStream2 {
    let enum_name_str = enum_ident.to_string();
    let metadata_count = metadata.len();

    quote! {
        impl #enum_ident {
            /// Get diagnostic information for LSP integration
            pub fn diagnostic_info(&self) -> ::yoshi_std::DiagnosticInfo {
                ::yoshi_std::DiagnosticInfo {
                    error_type: #enum_name_str,
                    variant: self.variant_name(),
                    autofix_available: self.variant_autofix().is_some(),
                    quick_fix_count: self.quick_fixes().len(),
                    metadata_count: #metadata_count,
                }
            }
        }
    }
}

//--------------------------------------------------------------------------------------------------
// ML-Inspired Auto-Inference Engine with Thread-Safe Caching
//--------------------------------------------------------------------------------------------------

/// Apply ML-inspired auto-inference with advanced pattern recognition and caching
fn apply_ml_inspired_auto_inference(opts: &mut YoshiErrorOpts) -> Result<()> {
    let default_severity = opts.default_severity;
    let darling::ast::Data::Enum(variants) = &mut opts.data else {
        return Ok(());
    };

    for (variant_index, variant) in variants.iter_mut().enumerate() {
        // Enhanced display format inference with caching
        if variant.display.is_none() {
            variant.display = Some(generate_intelligent_display_format(variant));
        }

        // Advanced error kind inference with ML-inspired scoring
        if variant.kind.is_none() {
            variant.kind = Some(infer_ml_inspired_error_kind(
                &variant.ident,
                &variant.fields,
            ));
        }

        // Enhanced severity inference with contextual analysis
        if variant.severity.is_none() {
            variant.severity = Some(infer_intelligent_severity(variant, default_severity));
        }

        // Advanced source field detection with type analysis
        enhance_source_field_detection(variant)?;

        // Enhanced transient status inference
        if !variant.transient {
            variant.transient = infer_transient_status(&variant.ident, variant.kind.as_deref());
        }

        // Advanced suggestion generation
        if variant.suggestion.is_none() {
            variant.suggestion = generate_contextual_auto_suggestion(variant);
        }

        // Auto-generate error codes if base is provided
        if variant.code.is_none() {
            if let Some(base) = opts.error_code_base {
                let code = base
                    + u32::try_from(variant_index)
                        .expect("Enum variant count exceeds u32::MAX, which is unsupported");

                // Register the code unless override is enabled
                if !opts.override_codes {
                    register_error_code(code, &variant.ident.to_string(), variant.ident.span())?;
                }

                variant.code = Some(code);
            }
        } else if let Some(code) = variant.code {
            // Register explicit error codes
            if !opts.override_codes {
                register_error_code(code, &variant.ident.to_string(), variant.ident.span())?;
            }
        }
    }

    Ok(())
}

/// Generate intelligent display format with context awareness and improved fallbacks
fn generate_intelligent_display_format(variant: &YoshiVariantOpts) -> String {
    let variant_name = humanize_identifier(&variant.ident.to_string());

    match &variant.fields.style {
        Style::Unit => variant_name,
        Style::Tuple if variant.fields.len() == 1 => {
            let field = &variant.fields.fields[0];
            if field.source {
                format!("{variant_name}: caused by {{0}}")
            } else if field.sensitive {
                format!("{variant_name}: [REDACTED]")
            } else {
                format!("{variant_name}: {{0}}")
            }
        }
        Style::Tuple => {
            let placeholders: Vec<String> = (0..variant.fields.len())
                .enumerate()
                .map(|(i, _)| {
                    if variant.fields.fields[i].sensitive {
                        "[REDACTED]".to_string()
                    } else {
                        // Enhanced fallback with type information
                        let type_name = simplify_type_name(&variant.fields.fields[i].ty);
                        format!("{{{i}}} ({type_name})")
                    }
                })
                .collect();
            format!("{variant_name}: ({})", placeholders.join(", "))
        }
        Style::Struct => {
            let important_fields: Vec<_> = variant
                .fields
                .iter()
                .filter(|f| !f.skip && f.ident.is_some())
                .take(3)
                .collect();

            if important_fields.is_empty() {
                return variant_name;
            }

            let field_formats: Vec<String> = important_fields
                .iter()
                .map(|f| {
                    let field_name = f.ident.as_ref().unwrap().to_string();
                    if f.sensitive {
                        format!("{field_name}: [REDACTED]")
                    } else if f.source {
                        format!("caused by {{{field_name}}}")
                    } else {
                        format!("{field_name}: {{{field_name}}}")
                    }
                })
                .collect();

            format!("{variant_name} {{ {} }}", field_formats.join(", "))
        }
    }
}

/// Simplify type name for display purposes
fn simplify_type_name(ty: &Type) -> String {
    let type_str = ty.to_token_stream().to_string();

    // Extract the last component of path types
    if let Some(last_segment) = type_str.split("::").last() {
        // Remove generic parameters for cleaner display
        if let Some(base_name) = last_segment.split('<').next() {
            return base_name.to_string();
        }
        return last_segment.to_string();
    }

    type_str
}

/// ML-inspired error kind inference with advanced scoring and caching
#[allow(clippy::cast_precision_loss)]
fn infer_ml_inspired_error_kind(
    variant_name: &Ident,
    fields: &darling::ast::Fields<YoshiFieldOpts>,
) -> String {
    // Create cache key
    let field_types: Vec<String> = fields
        .iter()
        .map(|f| f.ty.to_token_stream().to_string())
        .collect();

    let cache_key = InferenceCacheKey {
        variant_name: variant_name.to_string(),
        field_types: field_types.clone(),
        field_count: fields.len(),
    };

    // Check cache first
    if let Ok(cache) = init_inference_cache().lock() {
        if let Some(cached_result) = cache.get(&cache_key) {
            return cached_result.error_kind.clone();
        }
    }

    // ML-inspired scoring algorithm
    let name_lower = variant_name.to_string().to_lowercase();
    let mut kind_scores: HashMap<&str, f64> = HashMap::new();

    // Advanced pattern matching with weighted scoring
    let patterns = [
        ("Io", 0.95, ["io", "file", "path", "fs", "read", "write"]),
        (
            "Network",
            0.90,
            ["network", "http", "tcp", "connection", "timeout", "url"],
        ),
        (
            "Security",
            0.88,
            [
                "auth",
                "security",
                "permission",
                "credential",
                "token",
                "jwt",
            ],
        ),
        (
            "Validation",
            0.85,
            [
                "validation",
                "parse",
                "format",
                "invalid",
                "malformed",
                "decode",
            ],
        ),
        (
            "Timeout",
            0.82,
            [
                "timeout",
                "deadline",
                "expired",
                "busy",
                "retry",
                "transient",
            ],
        ),
        (
            "Config",
            0.80,
            ["config", "setting", "configuration", "env", "param", "var"],
        ),
        (
            "NotFound",
            0.78,
            ["notfound", "missing", "absent", "unknown", "empty", "gone"],
        ),
        (
            "ResourceExhausted",
            0.75,
            [
                "resource",
                "exhausted",
                "limit",
                "capacity",
                "full",
                "memory",
            ],
        ),
    ];

    for (kind, base_weight, keywords) in patterns {
        let keyword_score = keywords
            .iter()
            .map(|&keyword| {
                if name_lower.contains(keyword) {
                    1.0
                } else {
                    0.0
                }
            })
            .sum::<f64>()
            / keywords.len() as f64;

        if keyword_score > 0.0 {
            kind_scores.insert(kind, base_weight * keyword_score);
        }
    }

    // Type-based enhancement scoring
    for field_type in &field_types {
        let type_lower = field_type.to_lowercase();

        if type_lower.contains("io::error") {
            *kind_scores.entry("Io").or_insert(0.0) += 0.5;
        } else if type_lower.contains("reqwest") || type_lower.contains("hyper") {
            *kind_scores.entry("Network").or_insert(0.0) += 0.4;
        } else if type_lower.contains("serde") || type_lower.contains("json") {
            *kind_scores.entry("Validation").or_insert(0.0) += 0.3;
        } else if type_lower.contains("auth") || type_lower.contains("jwt") {
            *kind_scores.entry("Security").or_insert(0.0) += 0.4;
        }
    }

    // Select best scoring kind
    let (best_kind, confidence) = kind_scores
        .into_iter()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or(("Internal", 0.5));

    let result_kind = best_kind.to_string();

    // Cache the result
    if let Ok(mut cache) = init_inference_cache().lock() {
        if cache.len() < INFERENCE_CACHE_SIZE {
            cache.insert(
                cache_key,
                InferenceCacheValue {
                    error_kind: result_kind.clone(),
                    confidence_score: confidence,
                    display_format: String::new(), // Would be filled by display inference
                    severity: get_default_severity(),
                },
            );
        }
    }

    result_kind
}

/// Enhanced severity inference with contextual factors
fn infer_intelligent_severity(variant: &YoshiVariantOpts, default_severity: u8) -> u8 {
    let mut base_severity = match variant.kind.as_deref() {
        Some("Internal") => 240,
        Some("Security") => 220,
        Some("ResourceExhausted") => 200,
        Some("Timeout") => 180,
        Some("Network") => 160,
        Some("Io") => 140,
        Some("Config") => 120,
        Some("Validation") => 100,
        Some("NotFound") => 80,
        _ => default_severity,
    };

    // Contextual adjustments with bounds checking
    let adjustments = [
        (variant.fields.iter().any(|f| f.source), 10),
        (variant.fields.len() > 3, 5),
        (variant.transient, -20),
        (variant.fields.iter().any(|f| f.sensitive), 15),
    ];

    for (condition, adjustment) in adjustments {
        if condition {
            base_severity = base_severity.saturating_add_signed(adjustment);
        }
    }

    base_severity
}

/// Enhanced source field detection with comprehensive type analysis
fn enhance_source_field_detection(variant: &mut YoshiVariantOpts) -> Result<()> {
    let source_count = variant.fields.iter().filter(|f| f.source).count();

    // Ensure only one source field is marked
    if source_count > 1 {
        return Err(Error::new(
            variant.ident.span(),
            format!(
                "Variant '{}' has {} source fields marked, but only one is allowed",
                variant.ident, source_count
            ),
        ));
    }

    // If no source field is marked, try to find the best candidate
    if source_count == 0 {
        let mut best_candidate_idx = None;
        let mut best_score = 0;

        for (idx, field) in variant.fields.fields.iter().enumerate() {
            let score = calculate_source_field_score(&field.ty);
            if score > best_score {
                best_score = score;
                best_candidate_idx = Some(idx);
            }
        }

        // Mark the best candidate as source if score is high enough
        if let Some(idx) = best_candidate_idx {
            if best_score >= 50 {
                variant.fields.fields[idx].source = true;
            }
        }
    }

    Ok(())
}

/// Calculate score for source field candidacy with enhanced type analysis
fn calculate_source_field_score(ty: &Type) -> i32 {
    let type_str = ty.to_token_stream().to_string();
    let mut score = 0;

    // Enhanced error type detection
    if is_enhanced_error_type(ty) {
        score += 100;
    }

    // Specific type bonuses
    if type_str.contains("std::io::Error") {
        score += 150;
    }
    if type_str.contains("Box<dyn") && type_str.contains("Error") {
        score += 120;
    }
    if type_str.contains("anyhow::Error") || type_str.contains("eyre::Error") {
        score += 110;
    }

    // Contextual bonuses
    if contains_error_keywords(&type_str) {
        score += 50;
    }
    if type_str.contains("Result") {
        score += 30;
    }

    score
}

/// Enhanced transient status inference with pattern recognition
fn infer_transient_status(variant_name: &Ident, kind: Option<&str>) -> bool {
    let name_lower = variant_name.to_string().to_lowercase();

    // Check for explicit permanent patterns first
    if PERMANENT_PATTERNS
        .iter()
        .any(|&pattern| name_lower.contains(pattern))
    {
        return false;
    }

    // Check for transient patterns
    if TRANSIENT_PATTERNS
        .iter()
        .any(|&pattern| name_lower.contains(pattern))
    {
        return true;
    }

    // Kind-based inference with enhanced logic
    match kind {
        Some("Network" | "Timeout" | "ResourceExhausted") => true,
        Some("Validation" | "Security" | "NotFound") => false,
        _ => name_lower.contains("connection") || name_lower.contains("timeout"),
    }
}

/// Enhanced automatic suggestion generation with context awareness
fn generate_contextual_auto_suggestion(variant: &YoshiVariantOpts) -> Option<String> {
    let variant_name = variant.ident.to_string().to_lowercase();

    let base_suggestion = match variant.kind.as_deref() {
        Some("Timeout") => {
            if variant_name.contains("connection") {
                "Check network connectivity and increase connection timeout"
            } else {
                "Consider increasing timeout duration or optimizing the operation"
            }
        }
        Some("Network") => {
            if variant_name.contains("dns") {
                "Verify DNS configuration and network connectivity"
            } else if variant_name.contains("ssl") || variant_name.contains("tls") {
                "Check SSL/TLS certificate validity and configuration"
            } else {
                "Check network connectivity and retry the operation"
            }
        }
        Some("Validation") => {
            if variant_name.contains("parse") {
                "Verify input data format and syntax"
            } else if variant_name.contains("schema") {
                "Check data against the expected schema"
            } else {
                "Verify input data format and constraints"
            }
        }
        Some("NotFound") => {
            if variant_name.contains("file") || variant_name.contains("path") {
                "Ensure the file exists and check the path"
            } else {
                "Verify the resource identifier and ensure it exists"
            }
        }
        Some("Config") => {
            "Review configuration settings and ensure all required values are properly set"
        }
        Some("Io") => {
            if variant_name.contains("permission") {
                "Check file permissions and access rights"
            } else {
                "Check file permissions, disk space, and path validity"
            }
        }
        Some("Security") => "Verify authentication credentials and access permissions",
        Some("ResourceExhausted") => "Free up system resources or increase available capacity",
        _ => {
            if variant.transient {
                "This error may be temporary, consider implementing retry logic with exponential backoff"
            } else {
                return None;
            }
        }
    };

    let enhanced_suggestion = if variant.fields.iter().any(|f| f.source) {
        format!("{base_suggestion}. Check the underlying error for more details.")
    } else {
        base_suggestion.to_string()
    };

    Some(enhanced_suggestion)
}

//--------------------------------------------------------------------------------------------------
// Enhanced Code Generation with Performance Optimization
//--------------------------------------------------------------------------------------------------

/// Generate enhanced Display implementation with intelligent formatting
fn generate_enhanced_display_impl(opts: &YoshiErrorOpts) -> Result<TokenStream2> {
    let enum_name = &opts.ident;
    let (impl_generics, ty_generics, where_clause) = opts.generics.split_for_impl();

    let darling::ast::Data::Enum(variants) = &opts.data else {
        return Err(Error::new(opts.ident.span(), "Expected enum"));
    };

    let display_arms = variants
        .iter()
        .filter(|v| !v.skip)
        .map(generate_enhanced_display_arm)
        .collect::<Result<Vec<_>>>()?;

    // Apply namespace prefix if specified
    let namespace_prefix = if let Some(namespace) = &opts.namespace {
        format!("{namespace}: ")
    } else {
        String::new()
    };

    let implementation = if variants.len() > VARIANT_COUNT_THRESHOLD_LARGE {
        quote! {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, #namespace_prefix)?;
                match self {
                    #(#display_arms)*
                }
            }
        }
    } else {
        quote! {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, #namespace_prefix)?;
                match self {
                    #(#display_arms)*
                }
            }
        }
    };

    Ok(quote! {
        impl #impl_generics ::std::fmt::Display for #enum_name #ty_generics #where_clause {
            #implementation
        }
    })
}

/// Generate enhanced display arm with advanced placeholder handling
fn generate_enhanced_display_arm(variant: &YoshiVariantOpts) -> Result<TokenStream2> {
    let variant_ident = &variant.ident;
    let display_fmt = variant
        .display
        .as_ref()
        .ok_or_else(|| Error::new(variant.ident.span(), "Display format should be inferred"))?;

    match &variant.fields.style {
        Style::Unit => Ok(quote! {
            Self::#variant_ident => write!(f, #display_fmt),
        }),
        Style::Tuple => {
            let field_patterns: Vec<_> = (0..variant.fields.len())
                .map(|i| format_ident_safely(&format!("field_{i}"), variant.ident.span()))
                .collect::<Result<Vec<_>>>()?;

            let format_args = generate_enhanced_tuple_format_args(
                display_fmt,
                &field_patterns,
                &variant.fields.fields,
            );

            Ok(quote! {
                Self::#variant_ident(#(#field_patterns),*) => {
                    write!(f, #display_fmt #format_args)
                },
            })
        }
        Style::Struct => {
            let field_patterns: Vec<Ident> = variant
                .fields
                .iter()
                .filter_map(|f| f.ident.clone())
                .collect();

            let format_args = generate_enhanced_struct_format_args(
                display_fmt,
                &field_patterns,
                &variant.fields.fields,
            );

            Ok(quote! {
                Self::#variant_ident { #(#field_patterns),* } => {
                    write!(f, #display_fmt #format_args)
                },
            })
        }
    }
}

/// Generate enhanced format arguments for tuple variants
fn generate_enhanced_tuple_format_args(
    display_fmt: &str,
    field_patterns: &[Ident],
    field_opts: &[YoshiFieldOpts],
) -> TokenStream2 {
    let placeholders = extract_placeholders(display_fmt);

    if placeholders.is_empty() {
        return quote! {};
    }

    let args: Vec<_> = placeholders
        .iter()
        .enumerate()
        .filter_map(|(i, placeholder)| {
            let field_index = if let Ok(index) = placeholder.parse::<usize>() {
                index
            } else {
                i
            };

            field_patterns.get(field_index).and_then(|field_ident| {
                field_opts
                    .get(field_index)
                    .map(|field_opt| generate_field_format_expression(field_ident, field_opt))
            })
        })
        .collect();

    if args.is_empty() {
        quote! {}
    } else {
        quote! { , #(#args),* }
    }
}

/// Generate enhanced format arguments for struct variants
fn generate_enhanced_struct_format_args(
    display_fmt: &str,
    field_patterns: &[Ident],
    field_opts: &[YoshiFieldOpts],
) -> TokenStream2 {
    if !contains_named_placeholders(display_fmt) {
        return quote! {};
    }

    let field_map: HashMap<String, (&Ident, &YoshiFieldOpts)> = field_patterns
        .iter()
        .zip(field_opts.iter())
        .map(|(ident, opts)| (ident.to_string(), (ident, opts)))
        .collect();

    let placeholders = extract_placeholders(display_fmt);
    let format_assignments: Vec<_> = placeholders
        .iter()
        .filter_map(|placeholder| {
            if let Some((field_ident, field_opt)) = field_map.get(placeholder) {
                if let Ok(placeholder_ident) = format_ident_safely(placeholder, Span::call_site()) {
                    let expr = generate_field_format_expression(field_ident, field_opt);
                    Some(quote! { #placeholder_ident = #expr })
                } else {
                    None
                }
            } else if placeholder == "source" {
                Some(generate_source_placeholder_assignment(field_opts))
            } else if let Ok(placeholder_ident) =
                format_ident_safely(placeholder, Span::call_site())
            {
                Some(quote! { #placeholder_ident = "<unknown>" })
            } else {
                None
            }
        })
        .collect();

    if format_assignments.is_empty() {
        quote! {}
    } else {
        quote! { , #(#format_assignments),* }
    }
}

/// Generate field format expression with enhanced handling
fn generate_field_format_expression(
    field_ident: &Ident,
    field_opt: &YoshiFieldOpts,
) -> TokenStream2 {
    if field_opt.skip {
        quote! { "<skipped>" }
    } else if field_opt.sensitive {
        quote! { "[REDACTED]" }
    } else if let Some(transform_fn) = &field_opt.transform {
        if let Ok(transform_fn_ident) = format_ident_safely(transform_fn, Span::call_site()) {
            quote! { #transform_fn_ident(#field_ident) }
        } else {
            quote! { #field_ident }
        }
    } else if let Some(format_fn) = &field_opt.format_with {
        if let Ok(format_fn_ident) = format_ident_safely(format_fn, Span::call_site()) {
            quote! { #format_fn_ident(#field_ident) }
        } else {
            quote! { #field_ident }
        }
    } else {
        quote! { #field_ident }
    }
}

/// Generate source placeholder assignment with enhanced fallback handling
fn generate_source_placeholder_assignment(field_opts: &[YoshiFieldOpts]) -> TokenStream2 {
    if let Some(source_field) = field_opts.iter().find(|opt| opt.source) {
        if let Some(ident) = &source_field.ident {
            quote! { source = #ident }
        } else {
            quote! { source = "<unnamed source field>" }
        }
    } else {
        quote! { source = "<no source available>" }
    }
}

/// Generate enhanced Error trait implementation
fn generate_enhanced_error_impl(opts: &YoshiErrorOpts) -> Result<TokenStream2> {
    let enum_name = &opts.ident;
    let (impl_generics, ty_generics, where_clause) = opts.generics.split_for_impl();

    let darling::ast::Data::Enum(variants) = opts.data.as_ref() else {
        return Err(Error::new(opts.ident.span(), "Expected enum"));
    };

    let source_arms = variants
        .iter()
        .filter(|v| !v.skip)
        .map(|v| generate_enhanced_source_arm(v))
        .collect::<Vec<_>>();

    let backtrace_method = if opts.backtrace {
        quote! {
            fn backtrace(&self) -> Option<&std::backtrace::Backtrace> {
                None
            }
        }
    } else {
        quote! {}
    };

    Ok(quote! {
        impl #impl_generics ::std::error::Error for #enum_name #ty_generics #where_clause {
            fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
                match self {
                    #(#source_arms)*
                }
            }

            #backtrace_method
        }
    })
}

/// Generate enhanced source arm with intelligent source detection
fn generate_enhanced_source_arm(variant: &YoshiVariantOpts) -> TokenStream2 {
    let variant_ident = &variant.ident;
    let source_field_info = variant.fields.iter().enumerate().find(|(_, f)| f.source);

    match &variant.fields.style {
        Style::Unit => quote! { Self::#variant_ident => None, },
        Style::Tuple => {
            if let Some((idx, _)) = source_field_info {
                let patterns = (0..variant.fields.len()).map(|i| {
                    if i == idx {
                        quote! { ref source }
                    } else {
                        quote! { _ }
                    }
                });
                quote! {
                    Self::#variant_ident(#(#patterns),*) => Some(source as &(dyn ::std::error::Error + 'static)),
                }
            } else {
                quote! { Self::#variant_ident(..) => None, }
            }
        }
        Style::Struct => {
            if let Some((_, field)) = source_field_info {
                let source_ident = field.ident.as_ref().unwrap();
                quote! {
                    Self::#variant_ident { ref #source_ident, .. } => Some(#source_ident as &(dyn ::std::error::Error + 'static)),
                }
            } else {
                quote! { Self::#variant_ident { .. } => None, }
            }
        }
    }
}

/// Generate enhanced Yoshi conversion with comprehensive metadata
fn generate_enhanced_yoshi_conversion(opts: &YoshiErrorOpts) -> Result<TokenStream2> {
    let enum_name = &opts.ident;
    let (impl_generics, ty_generics, where_clause) = opts.generics.split_for_impl();

    let darling::ast::Data::Enum(variants) = opts.data.as_ref() else {
        return Err(Error::new(opts.ident.span(), "Expected enum"));
    };

    let conversion_arms = variants
        .iter()
        .filter(|v| !v.skip)
        .map(|variant| generate_enhanced_conversion_arm(variant, opts))
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! {
        impl #impl_generics ::std::convert::From<#enum_name #ty_generics> for ::yoshi_std::Yoshi #where_clause {
            #[track_caller]
            fn from(err: #enum_name #ty_generics) -> Self {
                let error_message = err.to_string();
                match err {
                    #(#conversion_arms)*
                }
            }
        }
    })
}

/// Generate enhanced conversion arm with intelligent metadata handling
fn generate_enhanced_conversion_arm(
    variant: &YoshiVariantOpts,
    opts: &YoshiErrorOpts,
) -> Result<TokenStream2> {
    let variant_ident = &variant.ident;
    let enum_name = &opts.ident;

    let (pattern, field_refs) = match &variant.fields.style {
        Style::Unit => (quote! {}, vec![]),
        Style::Tuple => {
            let idents: Vec<_> = (0..variant.fields.len())
                .map(|i| format_ident_safely(&format!("field_{i}"), variant.ident.span()))
                .collect::<Result<Vec<_>>>()?;
            (quote! { ( #(#idents),* ) }, idents)
        }
        Style::Struct => {
            let idents: Vec<_> = variant
                .fields
                .iter()
                .filter_map(|f| f.ident.clone())
                .collect();
            (quote! { { #(#idents),* } }, idents)
        }
    };

    let yoshi_construction = generate_enhanced_yoshi_construction(variant, opts, &field_refs);

    Ok(quote! {
        #enum_name::#variant_ident #pattern => {
            #yoshi_construction
        }
    })
}

/// Generate enhanced Yoshi construction with comprehensive metadata
fn generate_enhanced_yoshi_construction(
    variant: &YoshiVariantOpts,
    opts: &YoshiErrorOpts,
    field_idents: &[Ident],
) -> TokenStream2 {
    let kind_str = variant
        .kind
        .as_deref()
        .or(opts.default_kind.as_deref())
        .unwrap_or("Internal");

    let base_yoshi = if let Some((_, field_ident)) = variant
        .fields
        .iter()
        .zip(field_idents)
        .find(|(f, _)| f.source)
    {
        quote! {
            ::yoshi_std::Yoshi::new(::yoshi_std::YoshiKind::Internal {
                message: ::yoshi_std::Arc::from(error_message.clone()),
                source: Some(Box::new(::yoshi_std::Yoshi::from(#field_ident))),
                component: Some(::yoshi_std::Arc::from("unknown")),
            })
        }
    } else {
        generate_enhanced_yoshi_kind_construction(
            kind_str,
            &quote! { error_message },
            variant,
            field_idents,
        )
    };

    let mut metadata_statements = vec![quote! { let mut yoshi_err = #base_yoshi; }];

    // Add namespace metadata if specified
    if let Some(namespace) = &opts.namespace {
        metadata_statements.push(quote! {
            yoshi_err = yoshi_err.with_metadata("namespace", #namespace);
        });
    }

    if let Some(suggestion) = &variant.suggestion {
        metadata_statements.push(quote! {
            yoshi_err = yoshi_err.with_suggestion(#suggestion);
        });
    }

    let severity = variant.severity.unwrap_or(opts.default_severity);
    metadata_statements.push(quote! {
        yoshi_err = yoshi_err.with_priority(#severity);
    });

    if variant.transient {
        metadata_statements.push(quote! {
            yoshi_err = yoshi_err.with_metadata("transient", "true");
        });
    }

    if let Some(code) = variant.code {
        metadata_statements.push(quote! {
            yoshi_err = yoshi_err.with_metadata("error_code", #code.to_string());
        });
    }

    if let Some(category) = &variant.category {
        metadata_statements.push(quote! {
            yoshi_err = yoshi_err.with_metadata("category", #category);
        });
    }

    if let Some(doc_url) = &variant.doc_url {
        metadata_statements.push(quote! {
            yoshi_err = yoshi_err.with_metadata("doc_url", #doc_url);
        });
    }

    // Enhanced field-specific metadata with transformation support
    for (field_opt, field_ident) in variant.fields.iter().zip(field_idents) {
        if field_opt.source || field_opt.skip {
            continue;
        }

        if let Some(context_key) = &field_opt.context {
            let value = if field_opt.sensitive {
                quote! { "[REDACTED]".to_string() }
            } else if let Some(transform_fn) = &field_opt.transform {
                if let Ok(transform_fn_ident) = format_ident_safely(transform_fn, Span::call_site())
                {
                    quote! { format!("{:?}", #transform_fn_ident(#field_ident)) }
                } else {
                    quote! { format!("{:?}", #field_ident) }
                }
            } else {
                quote! { format!("{:?}", #field_ident) }
            };
            metadata_statements.push(quote! {
                yoshi_err = yoshi_err.with_metadata(#context_key, #value);
            });
        }

        if field_opt.shell {
            metadata_statements.push(quote! {
                yoshi_err = yoshi_err.with_shell(format!("{:?}", #field_ident));
            });
        }
    }

    metadata_statements.push(quote! { yoshi_err });

    quote! { #(#metadata_statements)* }
}

/// Generate enhanced `YoshiKind` construction
fn generate_enhanced_yoshi_kind_construction(
    kind_str: &str,
    message: &TokenStream2,
    variant: &YoshiVariantOpts,
    field_idents: &[Ident],
) -> TokenStream2 {
    let source_expr = if let Some((_, field_ident)) = variant
        .fields
        .iter()
        .zip(field_idents)
        .find(|(f, _)| f.source)
    {
        quote! {
            Some(Box::new(::yoshi_std::Yoshi::from(#field_ident)))
        }
    } else {
        quote! { None }
    };

    match kind_str {
        "Io" => quote! {
            #[cfg(feature = "std")]
            {
                ::yoshi_std::Yoshi::new(::yoshi_std::YoshiKind::Io(
                    ::std::io::Error::new(::std::io::ErrorKind::Other, #message.as_ref())
                ))
            }
            #[cfg(not(feature = "std"))]
            {
                ::yoshi_std::Yoshi::new(::yoshi_std::YoshiKind::Io(
                    ::yoshi_std::NoStdIo::new(#message.to_string())
                ))
            }
        },
        "Network" => quote! {
            ::yoshi_std::Yoshi::new(::yoshi_std::YoshiKind::Network {
                message: ::yoshi_std::Arc::from(#message.to_string()),
                source: #source_expr,
                error_code: None,
            })
        },
        "Validation" => quote! {
            ::yoshi_std::Yoshi::new(::yoshi_std::YoshiKind::Validation {
                field: ::yoshi_std::Arc::from("unknown"),
                message: ::yoshi_std::Arc::from(#message.to_string()),
                expected: None,
                actual: None,
            })
        },
        "Config" => quote! {
            ::yoshi_std::Yoshi::new(::yoshi_std::YoshiKind::Config {
                message: ::yoshi_std::Arc::from(#message.to_string()),
                source: #source_expr,
                config_path: None,
            })
        },
        "Security" => quote! {
            ::yoshi_std::Yoshi::new(::yoshi_std::YoshiKind::Security {
                message: ::yoshi_std::Arc::from(#message.to_string()),
                source: #source_expr,
                security_level: ::yoshi_std::Arc::from("HIGH"),
            })
        },
        "Timeout" => quote! {
            ::yoshi_std::Yoshi::new(::yoshi_std::YoshiKind::Timeout {
                operation: ::yoshi_std::Arc::from(#message.to_string()),
                duration: ::core::time::Duration::from_millis(5000),
                expected_max: None,
            })
        },
        "NotFound" => quote! {
            ::yoshi_std::Yoshi::new(::yoshi_std::YoshiKind::NotFound {
                resource_type: ::yoshi_std::Arc::from("unknown"),
                identifier: ::yoshi_std::Arc::from(#message.to_string()),
                search_locations: None,
            })
        },
        "ResourceExhausted" => quote! {
            ::yoshi_std::Yoshi::new(::yoshi_std::YoshiKind::ResourceExhausted {
                resource: ::yoshi_std::Arc::from("unknown"),
                limit: ::yoshi_std::Arc::from("unknown"),
                current: ::yoshi_std::Arc::from("unknown"),
                usage_percentage: None,
            })
        },
        "Foreign" => quote! {
            ::yoshi_std::Yoshi::new(::yoshi_std::YoshiKind::Foreign {
                error: Box::new(::std::io::Error::new(::std::io::ErrorKind::Other, #message.as_ref())),
                error_type_name: ::yoshi_std::Arc::from("generated"),
            })
        },
        "Multiple" => quote! {
            ::yoshi_std::Yoshi::new(::yoshi_std::YoshiKind::Multiple {
                errors: vec![],
                primary_index: None,
            })
        },
        _ => quote! {
            ::yoshi_std::Yoshi::new(::yoshi_std::YoshiKind::Internal {
                message: ::yoshi_std::Arc::from(#message.to_string()),
                source: #source_expr,
                component: Some(::yoshi_std::Arc::from("unknown")),
            })
        },
    }
}

/// Generate enhanced From implementations
fn generate_enhanced_from_impls(opts: &YoshiErrorOpts) -> Result<TokenStream2> {
    let enum_name = &opts.ident;
    let (impl_generics, ty_generics, where_clause) = opts.generics.split_for_impl();

    let darling::ast::Data::Enum(variants) = opts.data.as_ref() else {
        return Err(Error::new(opts.ident.span(), "Expected enum"));
    };

    let from_impls = variants
        .iter()
        .filter(|v| v.from && !v.skip && v.fields.fields.len() == 1)
        .map(|variant| {
            let variant_ident = &variant.ident;
            let field = &variant.fields.fields[0];
            let from_type = &field.ty;

            match &variant.fields.style {
                Style::Tuple => Ok(quote! {
                    impl #impl_generics ::std::convert::From<#from_type> for #enum_name #ty_generics #where_clause {
                        #[track_caller]
                        fn from(value: #from_type) -> Self {
                            Self::#variant_ident(value)
                        }
                    }
                }),
                Style::Struct => {
                    let field_ident = field.ident.as_ref().unwrap();
                    Ok(quote! {
                        impl #impl_generics ::std::convert::From<#from_type> for #enum_name #ty_generics #where_clause {
                            #[track_caller]
                            fn from(value: #from_type) -> Self {
                                Self::#variant_ident { #field_ident: value }
                            }
                        }
                    })
                }
                Style::Unit => Ok(quote! {}),
            }
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! {
        #(#from_impls)*
    })
}

/// Generate enhanced helper methods
fn generate_enhanced_helper_methods(opts: &YoshiErrorOpts) -> Result<TokenStream2> {
    let enum_name = &opts.ident;
    let (impl_generics, ty_generics, where_clause) = opts.generics.split_for_impl();

    let darling::ast::Data::Enum(variants) = opts.data.as_ref() else {
        return Err(Error::new(opts.ident.span(), "Expected enum"));
    };

    let variant_check_methods = variants
        .iter()
        .filter(|v| !v.skip)
        .map(|variant| {
            let variant_ident = &variant.ident;
            let method_name = format_ident_safely(
                &format!("is_{}", variant_ident.to_string().to_lowercase()),
                variant.ident.span(),
            )?;
            let pattern = generate_variant_pattern(variant);

            Ok(quote! {
                /// Check if this error is of the specified variant
                #[inline]
                pub fn #method_name(&self) -> bool {
                    matches!(self, #pattern)
                }
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let variant_name_arms = variants.iter().filter(|v| !v.skip).map(|variant| {
        let variant_ident = &variant.ident;
        let pattern = generate_variant_pattern(variant);
        let name = variant_ident.to_string();
        quote! { #pattern => #name, }
    });

    let severity_arms = variants.iter().filter(|v| !v.skip).map(|variant| {
        let pattern = generate_variant_pattern(variant);
        let severity = variant.severity.unwrap_or(opts.default_severity);
        quote! { #pattern => #severity, }
    });

    let transient_arms = variants.iter().filter(|v| !v.skip).map(|variant| {
        let pattern = generate_variant_pattern(variant);
        let transient = variant.transient;
        quote! { #pattern => #transient, }
    });

    let kind_arms = variants.iter().filter(|v| !v.skip).map(|variant| {
        let pattern = generate_variant_pattern(variant);
        let kind = variant.kind.as_deref().unwrap_or("Internal");
        quote! { #pattern => #kind, }
    });

    let error_code_arms = variants.iter().filter(|v| !v.skip).map(|variant| {
        let pattern = generate_variant_pattern(variant);
        if let Some(code) = variant.code {
            quote! { #pattern => Some(#code), }
        } else {
            quote! { #pattern => None, }
        }
    });

    let suggestion_arms = variants.iter().filter(|v| !v.skip).map(|variant| {
        let pattern = generate_variant_pattern(variant);
        if let Some(suggestion) = &variant.suggestion {
            quote! { #pattern => Some(#suggestion), }
        } else {
            quote! { #pattern => None, }
        }
    });

    Ok(quote! {
        impl #impl_generics #enum_name #ty_generics #where_clause {
            #(#variant_check_methods)*

            /// Returns the variant name as a string
            #[inline]
            pub fn variant_name(&self) -> &'static str {
                match self {
                    #(#variant_name_arms)*
                }
            }

            /// Returns the severity level of this error (0-255, higher = more severe)
            #[inline]
            pub fn severity(&self) -> u8 {
                match self {
                    #(#severity_arms)*
                }
            }

            /// Returns true if this error is transient (retryable)
            #[inline]
            pub fn is_transient(&self) -> bool {
                match self {
                    #(#transient_arms)*
                }
            }

            /// Returns the error kind as a string
            #[inline]
            pub fn error_kind(&self) -> &'static str {
                match self {
                    #(#kind_arms)*
                }
            }

            /// Returns the error code if available
            #[inline]
            pub fn error_code(&self) -> Option<u32> {
                match self {
                    #(#error_code_arms)*
                }
            }

            /// Returns the auto-generated suggestion if available
            #[inline]
            pub fn suggestion(&self) -> Option<&'static str> {
                match self {
                    #(#suggestion_arms)*
                }
            }

            /// Returns true if this error has a source
            #[inline]
            pub fn has_source(&self) -> bool {
                self.source().is_some()
            }

            /// Returns comprehensive error context for debugging
            pub fn error_context(&self) -> ::std::collections::HashMap<&'static str, String> {
                let mut context = ::std::collections::HashMap::new();
                context.insert("variant", self.variant_name().to_string());
                context.insert("kind", self.error_kind().to_string());
                context.insert("severity", self.severity().to_string());
                context.insert("transient", self.is_transient().to_string());

                if let Some(code) = self.error_code() {
                    context.insert("error_code", code.to_string());
                }

                if let Some(suggestion) = self.suggestion() {
                    context.insert("suggestion", suggestion.to_string());
                }

                context
            }

            /// Returns related error information for diagnostic purposes
            pub fn related_errors(&self) -> Vec<&'static str> {
                vec![]
            }
        }
    })
}

/// Generate advanced performance optimizations
fn generate_performance_optimizations(opts: &YoshiErrorOpts) -> TokenStream2 {
    let darling::ast::Data::Enum(variants) = opts.data.as_ref() else {
        return quote! {};
    };

    let variant_count = variants.len();

    if variant_count > VARIANT_COUNT_THRESHOLD_HUGE {
        quote! {
            const _: () = {
                const VARIANT_COUNT: usize = #variant_count;
                const _: [(); 1] = [(); (VARIANT_COUNT < 1000) as usize];

                #[repr(C)]
                struct _SizeOptimizationHint;
            };
        }
    } else if variant_count > VARIANT_COUNT_THRESHOLD_LARGE {
        quote! {
            const _: () = {
                const VARIANT_COUNT: usize = #variant_count;
                const _: [(); 1] = [(); (VARIANT_COUNT < 500) as usize];
            };
        }
    } else {
        quote! {}
    }
}

//--------------------------------------------------------------------------------------------------
// Comprehensive Validation Implementation
//--------------------------------------------------------------------------------------------------

/// Enhanced comprehensive configuration validation
fn validate_comprehensive_configuration(opts: &YoshiErrorOpts) -> Result<()> {
    let darling::ast::Data::Enum(variants) = opts.data.as_ref() else {
        return Err(Error::new(opts.ident.span(), "Expected enum"));
    };

    if variants.is_empty() {
        return Err(Error::new(
            opts.ident.span(),
            "YoshiError enum cannot be empty",
        ));
    }

    let variant_count = variants.len();

    if variant_count > VARIANT_COUNT_THRESHOLD_LARGE && !opts.optimize_large {
        return Err(Error::new(
            opts.ident.span(),
            format!(
                "Large enum with {variant_count} variants detected. Consider enabling #[yoshi(optimize_large = true)]"
            ),
        ));
    }
    for variant in &variants {
        validate_enhanced_variant(variant)?;
    }

    validate_cross_variant_constraints(&variants)?;

    Ok(())
}

/// Enhanced variant validation
fn validate_enhanced_variant(variant: &YoshiVariantOpts) -> Result<()> {
    if let Some(display) = &variant.display {
        validate_enhanced_display_format(display, variant)?;
    }

    let source_count = variant.fields.iter().filter(|f| f.source).count();
    if source_count > 1 {
        return Err(Error::new(
            variant.ident.span(),
            format!(
                "Variant '{}' has {} source fields, but only one is allowed",
                variant.ident, source_count
            ),
        ));
    }

    if variant.from {
        if variant.fields.len() != 1 {
            return Err(Error::new(
                variant.ident.span(),
                format!(
                    "Variant '{}' marked with #[yoshi(from)] must have exactly one field",
                    variant.ident
                ),
            ));
        }
        if !matches!(variant.fields.style, Style::Tuple) {
            return Err(Error::new(
                variant.ident.span(),
                format!(
                    "Variant '{}' marked with #[yoshi(from)] must be a tuple variant",
                    variant.ident
                ),
            ));
        }
    }

    for (idx, field) in variant.fields.iter().enumerate() {
        if field.sensitive && field.shell {
            return Err(Error::new(
                variant.ident.span(),
                format!(
                    "Field {} in variant '{}' cannot be both sensitive and used in shell context",
                    idx, variant.ident
                ),
            ));
        }

        if let Some(format_fn) = &field.format_with {
            if !is_valid_rust_identifier(format_fn) {
                return Err(Error::new(
                    variant.ident.span(),
                    format!(
                        "Invalid format function name '{}' in variant '{}'",
                        format_fn, variant.ident
                    ),
                ));
            }
        }

        if let Some(transform_fn) = &field.transform {
            if !is_valid_rust_identifier(transform_fn) {
                return Err(Error::new(
                    variant.ident.span(),
                    format!(
                        "Invalid transform function name '{}' in variant '{}'",
                        transform_fn, variant.ident
                    ),
                ));
            }
        }
    }

    Ok(())
}

/// Enhanced display format validation
fn validate_enhanced_display_format(display: &str, variant: &YoshiVariantOpts) -> Result<()> {
    if display.len() > FORMAT_STRING_LENGTH_MODERATE {
        return Err(Error::new(
            variant.ident.span(),
            format!(
                "Display format too long ({} chars) in variant '{}'",
                display.len(),
                variant.ident
            ),
        ));
    }

    let placeholders = extract_placeholders(display);

    if matches!(variant.fields.style, Style::Tuple) {
        let field_count = variant.fields.len();
        for placeholder in &placeholders {
            if let Ok(index) = placeholder.parse::<usize>() {
                if index >= field_count {
                    return Err(Error::new(
                        variant.ident.span(),
                        format!(
                            "Tuple variant '{}' has {} fields but format string references field {{{index}}}",
                            variant.ident, field_count
                        ),
                    ));
                }
            }
        }
    }

    if matches!(variant.fields.style, Style::Struct) {
        let field_names: HashSet<_> = variant
            .fields
            .iter()
            .filter_map(|f| f.ident.as_ref().map(ToString::to_string))
            .collect();

        for placeholder in &placeholders {
            let clean_placeholder = placeholder.trim();
            if !clean_placeholder.is_empty()
                && clean_placeholder != "source"
                && !field_names.contains(clean_placeholder)
                && clean_placeholder.parse::<usize>().is_err()
            {
                return Err(Error::new(
                    variant.ident.span(),
                    format!(
                        "Display format references unknown field '{}' in variant '{}'",
                        clean_placeholder, variant.ident
                    ),
                ));
            }
        }
    }

    Ok(())
}

/// Validate cross-variant constraints
fn validate_cross_variant_constraints(variants: &[&YoshiVariantOpts]) -> Result<()> {
    let mut error_codes = HashMap::new();

    for &variant in variants {
        if let Some(code) = variant.code {
            if let Some(existing) = error_codes.insert(code, &variant.ident) {
                return Err(Error::new(
                    variant.ident.span(),
                    format!("Duplicate error code {code} (already used by variant '{existing}')"),
                ));
            }
        }
    }

    Ok(())
}

//--------------------------------------------------------------------------------------------------
// Enhanced Helper Functions
//--------------------------------------------------------------------------------------------------

/// Generate variant pattern for matching
fn generate_variant_pattern(variant: &YoshiVariantOpts) -> TokenStream2 {
    let variant_ident = &variant.ident;
    match &variant.fields.style {
        Style::Unit => quote! { Self::#variant_ident },
        Style::Tuple => quote! { Self::#variant_ident(..) },
        Style::Struct => quote! { Self::#variant_ident { .. } },
    }
}

/// Enhanced identifier humanization
fn humanize_identifier(ident: &str) -> String {
    let mut result = String::new();
    let mut chars = ident.chars().peekable();

    while let Some(c) = chars.next() {
        if result.is_empty() {
            result.push(c.to_uppercase().next().unwrap_or(c));
        } else if c.is_uppercase() {
            if let Some(&next_char) = chars.peek() {
                if next_char.is_lowercase() || result.chars().last().is_some_and(char::is_lowercase)
                {
                    result.push(' ');
                }
            } else {
                result.push(' ');
            }
            result.push(c.to_lowercase().next().unwrap_or(c));
        } else if c == '_' {
            result.push(' ');
        } else {
            result.push(c);
        }
    }

    result
}

/// Enhanced error type detection with comprehensive analysis
fn is_enhanced_error_type(ty: &Type) -> bool {
    is_enhanced_error_type_recursive(ty, 0)
}

/// Recursive error type detection with depth limiting
fn is_enhanced_error_type_recursive(ty: &Type, depth: usize) -> bool {
    if depth > MAX_TYPE_ANALYSIS_DEPTH {
        return false;
    }

    match ty {
        Type::Path(_) => {
            let path_str = ty.to_token_stream().to_string();
            is_path_error_type(&path_str) || contains_error_keywords(&path_str)
        }
        Type::TraitObject(trait_obj) => trait_obj.bounds.iter().any(|bound| {
            if let syn::TypeParamBound::Trait(trait_bound) = bound {
                contains_error_keywords(&trait_bound.to_token_stream().to_string())
            } else {
                false
            }
        }),
        Type::Reference(type_ref) => is_enhanced_error_type_recursive(&type_ref.elem, depth + 1),
        Type::Group(type_group) => is_enhanced_error_type_recursive(&type_group.elem, depth + 1),
        Type::Paren(type_paren) => is_enhanced_error_type_recursive(&type_paren.elem, depth + 1),
        _ => false,
    }
}

/// Check if a path represents a known error type
fn is_path_error_type(path_str: &str) -> bool {
    static KNOWN_ERROR_TYPES: &[&str] = &[
        "std::io::Error",
        "std::error::Error",
        "thiserror::Error",
        "anyhow::Error",
        "miette::Error",
        "eyre::Error",
        "yoshi::Oops",
    ];

    KNOWN_ERROR_TYPES
        .iter()
        .any(|&known_type| path_str.contains(known_type))
        || (path_str.contains("Box<dyn") && path_str.contains("Error"))
        || (path_str.contains("Arc<dyn") && path_str.contains("Error"))
}
