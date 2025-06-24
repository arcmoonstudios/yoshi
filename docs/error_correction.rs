/* yoshi-deluxe/src/strategies/error_correction.rs */
#![allow(dead_code)]
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_docs_in_private_items)]
//! **Brief:** Implementations of `CorrectionStrategy` for specific error codes.
//!
//! Each struct in this file provides a concrete, AST-aware strategy for fixing
//! a specific Rust compiler error. Organized numerically by error code.
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

use super::error_codes::ErrorCode;
use crate::types::CorrectionStrategy;
use crate::{
    ast::{ASTContext, NodeType},
    err::Hatch,
    types::{CorrectionProposal, CorrectionStrategy as ProposalStrategy, SafetyLevel},
};
use regex::Regex;

//--------------------------------------------------------------------------------------------------
// Module-Specific Error Type
//--------------------------------------------------------------------------------------------------

/// Defines errors that can occur during the generation of correction proposals.
#[derive(Debug, yoshi_derive::YoshiError)]
enum CorrectionStrategyError {
    /// Indicates that a regular expression failed to compile.
    #[yoshi(display = "Regex compilation failed for pattern: `{pattern}`")]
    RegexCompilation { pattern: String },
    /// Indicates that information could not be parsed from a diagnostic message.
    #[yoshi(display = "Failed to parse diagnostic message: {context}")]
    ParseFailed { context: String },
    /// Indicates that an expected AST node or code structure was not found.
    #[yoshi(display = "AST analysis failed: {context}")]
    AstAnalysisFailed { context: String },
}

//--------------------------------------------------------------------------------------------------
// E0004: Non-exhaustive patterns in match expression
//--------------------------------------------------------------------------------------------------

/// **Strategy for E0004: non-exhaustive patterns in `match` expression.**
///
/// This strategy provides two high-confidence fixes:
/// - Add a `_ => todo!()` wildcard arm to make the match exhaustive.
/// - (If applicable) Add specific arms for missing enum variants.
#[derive(Debug)]
pub(super) struct E0004NonExhaustivePatterns;

impl CorrectionStrategy for E0004NonExhaustivePatterns {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0004
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;

        // Proposal 1: Add a wildcard arm. This is always a safe fallback.
        if let NodeType::Expression { .. } = &context.primary_node.node_type {
            // A simple heuristic to find where to insert the new arm.
            if let Some(last_brace) = original_code.rfind('}') {
                let mut corrected_code = original_code.clone();
                corrected_code.insert_str(last_brace, "    _ => todo!(),\n");

                proposals.push(CorrectionProposal::from_strategy(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: corrected_code,
                    },
                    0.95,
                    SafetyLevel::Safe,
                    Some("Add a wildcard `_` arm to make the match exhaustive.".to_string()),
                ));
            }
        }

        // Proposal 2: AST analysis to find the `match` expression,
        // analyze the enum definition, and add specific arms for missing variants.
        if let Ok(missing_variants) =
            extract_missing_variants_from_message(&context.diagnostic.message)
        {
            if let Ok(corrected_code) =
                add_missing_arms_to_match_expr_string(original_code, &missing_variants)?
            {
                proposals.push(CorrectionProposal::from_strategy(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: corrected_code,
                    },
                    0.9,
                    SafetyLevel::Safe,
                    Some(format!(
                        "Add missing enum variants: {}",
                        missing_variants.join(", ")
                    )),
                ));
            }
        }

        Ok(proposals)
    }
}

//--------------------------------------------------------------------------------------------------
// E0005: Refutable pattern in let binding
//--------------------------------------------------------------------------------------------------

/// **Strategy for E0005: refutable pattern in let binding.**
///
/// This strategy provides solutions for refutable patterns used in `let` bindings.
/// Suggests converting to `if let` or `match` patterns for proper handling.
#[derive(Debug)]
pub(super) struct E0005RefutablePattern;

impl CorrectionStrategy for E0005RefutablePattern {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0005
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        // This is a complex transformation that requires rewriting a block of code.
        // A simple text replacement is not sufficient or safe.
        // The proposal will be to suggest the `if let` pattern.
        if let NodeType::Statement { .. } = &context.primary_node.node_type {
            let original_code = &context.primary_node.content;
            let corrected_code = format!(
                "if {} {{\n    // code that uses bindings from the pattern goes here\n}}",
                original_code.trim_end_matches(';')
            );

            let proposal = CorrectionProposal::from_strategy(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: corrected_code,
                },
                0.9,
                SafetyLevel::RequiresReview, // User must move dependent code
                Some("Change `let` to `if let` to handle refutable patterns.".to_string()),
            );
            return Ok(vec![proposal]);
        }
        Ok(vec![])
    }
}

//--------------------------------------------------------------------------------------------------
// E0023: Wrong number of fields in tuple struct pattern
//--------------------------------------------------------------------------------------------------

/// **Strategy for E0023: wrong number of fields in tuple struct pattern.**
#[derive(Debug)]
pub(super) struct E0023WrongNumberOfFieldsTuple;

impl CorrectionStrategy for E0023WrongNumberOfFieldsTuple {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0023
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let msg = &context.diagnostic.message;
        let original = &context.primary_node.content;

        // Extract expected and found field counts
        if let Ok((expected, found)) = extract_tuple_field_counts(msg)? {
            // Case 1: Too many fields - remove extras
            if found > expected {
                if let Ok(corrected) = adjust_tuple_pattern_fields(original, expected, found)? {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original.clone(),
                            replacement: corrected,
                        },
                        0.9,
                        SafetyLevel::RequiresReview,
                        Some(format!(
                            "Remove {} extra field(s) from tuple pattern",
                            found - expected
                        )),
                    ));
                }
            }
            // Case 2: Too few fields - add placeholders
            else if found < expected {
                let missing = expected - found;
                if let Ok(corrected) = add_tuple_pattern_placeholders(original, missing)? {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original.clone(),
                            replacement: corrected,
                        },
                        0.9,
                        SafetyLevel::RequiresReview,
                        Some(format!(
                            "Add {missing} placeholder field(s) to tuple pattern"
                        )),
                    ));
                }
            }
            // Offer rest pattern as alternative
            if let Ok(rest_pattern) = add_rest_pattern_to_tuple(original)? {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original.clone(),
                        replacement: rest_pattern,
                    },
                    0.8,
                    SafetyLevel::RequiresReview,
                    Some("Add `..` rest pattern to tuple".to_string()),
                ));
            }
        }

        // Fallback if no specific proposals generated
        if proposals.is_empty() {
            proposals.push(CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original.clone(),
                    replacement: original.clone(),
                },
                0.7,
                SafetyLevel::RequiresReview,
                Some("Adjust tuple fields to match expected count".to_string()),
            ));
        }

        Ok(proposals)
    }
}

//--------------------------------------------------------------------------------------------------
// E0025: Field bound multiple times in pattern
//--------------------------------------------------------------------------------------------------

/// **Strategy for E0025: field bound multiple times in pattern.**
#[derive(Debug)]
pub(super) struct E0025FieldBoundMultipleTimes;

impl CorrectionStrategy for E0025FieldBoundMultipleTimes {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0025
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;
        let msg = &context.diagnostic.message;

        // Extract the duplicate field name from error message
        if let Ok(duplicate_field) = extract_duplicate_field_name(msg)? {
            // Strategy 1: Remove the duplicate binding (keep first occurrence)
            if let Ok(corrected_code) =
                remove_duplicate_field_binding(original_code, &duplicate_field)?
            {
                if corrected_code != *original_code {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: corrected_code,
                        },
                        0.95,
                        SafetyLevel::RequiresReview,
                        Some(format!(
                            "Remove duplicate binding for field '{duplicate_field}'"
                        )),
                    ));
                }
            }

            // Strategy 2: Rename one occurrence to avoid conflict
            if let Ok(renamed_code) =
                rename_duplicate_field_binding(original_code, &duplicate_field)?
            {
                if renamed_code != *original_code {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: renamed_code,
                        },
                        0.8,
                        SafetyLevel::RequiresReview,
                        Some(format!(
                            "Rename second occurrence of field '{duplicate_field}'"
                        )),
                    ));
                }
            }

            // Strategy 3: Use pattern guards if applicable
            if original_code.contains("if") || can_use_pattern_guard(original_code) {
                if let Ok(guard_code) = convert_to_pattern_guard(original_code, &duplicate_field)? {
                    if guard_code != *original_code {
                        proposals.push(CorrectionProposal::new(
                            ProposalStrategy::ReplaceText {
                                original: original_code.clone(),
                                replacement: guard_code,
                            },
                            0.7,
                            SafetyLevel::RequiresReview,
                            Some("Use pattern guard to handle field condition".to_string()),
                        ));
                    }
                }
            }
        }

        Ok(proposals)
    }
}

//--------------------------------------------------------------------------------------------------
// E0026: Struct has no field with the given name
//--------------------------------------------------------------------------------------------------

/// **Strategy for E0026: struct has no field with the given name.**
#[derive(Debug)]
pub(super) struct E0026NonexistentField;

impl CorrectionStrategy for E0026NonexistentField {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0026
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;
        let msg = &context.diagnostic.message;

        // Extract nonexistent field name from error message
        if let Ok((nonexistent_field, struct_name)) = extract_nonexistent_field_info(msg)? {
            // Strategy 1: Remove the nonexistent field from pattern
            if let Ok(corrected_code) = remove_nonexistent_field(original_code, &nonexistent_field)?
            {
                if corrected_code != *original_code {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: corrected_code,
                        },
                        0.9,
                        SafetyLevel::RequiresReview,
                        Some(format!("Remove nonexistent field '{nonexistent_field}'")),
                    ));
                }
            }

            // Strategy 2: Suggest similar field names (typo correction)
            if let Ok(similar_fields) =
                suggest_similar_field_names(&nonexistent_field, &struct_name, context)?
            {
                for similar_field in similar_fields.into_iter().take(3) {
                    if let Ok(typo_corrected) =
                        replace_field_name(original_code, &nonexistent_field, &similar_field)?
                    {
                        if typo_corrected != *original_code {
                            proposals.push(CorrectionProposal::new(
                                ProposalStrategy::ReplaceText {
                                    original: original_code.clone(),
                                    replacement: typo_corrected,
                                },
                                0.85,
                                SafetyLevel::RequiresReview,
                                Some(format!(
                                    "Replace '{nonexistent_field}' with similar field '{similar_field}'"
                                )),
                            ));
                        }
                    }
                }
            }

            // Strategy 3: Add .. pattern to ignore unknown fields
            if !original_code.contains("..") {
                if let Ok(with_rest) = add_rest_pattern_to_struct(original_code)? {
                    if with_rest != *original_code {
                        proposals.push(CorrectionProposal::new(
                            ProposalStrategy::ReplaceText {
                                original: original_code.clone(),
                                replacement: with_rest,
                            },
                            0.75,
                            SafetyLevel::RequiresReview,
                            Some("Add .. pattern to ignore remaining fields".to_string()),
                        ));
                    }
                }
            }
        }

        Ok(proposals)
    }
}

//--------------------------------------------------------------------------------------------------
// E0027: Pattern missing fields from struct
//--------------------------------------------------------------------------------------------------

/// **Strategy for E0027: pattern missing fields from struct.**
#[derive(Debug)]
pub(super) struct E0027MissingStructFields;

impl CorrectionStrategy for E0027MissingStructFields {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0027
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;
        let msg = &context.diagnostic.message;

        // Extract missing field information from error message
        if let Ok((missing_fields, struct_name)) = extract_missing_fields_info(msg)? {
            // Strategy 1: Add all missing fields with placeholder bindings
            if let Ok(with_missing_fields) =
                add_missing_struct_fields(original_code, &missing_fields)?
            {
                if with_missing_fields != *original_code {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: with_missing_fields,
                        },
                        0.9,
                        SafetyLevel::RequiresReview,
                        Some(format!("Add missing fields: {}", missing_fields.join(", "))),
                    ));
                }
            }

            // Strategy 2: Add .. pattern to ignore missing fields
            if !original_code.contains("..") {
                if let Ok(with_rest_pattern) = add_rest_pattern_to_struct(original_code)? {
                    if with_rest_pattern != *original_code {
                        proposals.push(CorrectionProposal::new(
                            ProposalStrategy::ReplaceText {
                                original: original_code.clone(),
                                replacement: with_rest_pattern,
                            },
                            0.85,
                            SafetyLevel::RequiresReview,
                            Some("Add .. pattern to ignore missing fields".to_string()),
                        ));
                    }
                }
            }

            // Strategy 3: Convert to destructuring assignment if in binding context
            if is_in_binding_context(context) {
                if let Ok(destructuring_assignment) = convert_to_destructuring_assignment(
                    original_code,
                    &missing_fields,
                    &struct_name,
                )? {
                    if destructuring_assignment != *original_code {
                        proposals.push(CorrectionProposal::new(
                            ProposalStrategy::ReplaceText {
                                original: original_code.clone(),
                                replacement: destructuring_assignment,
                            },
                            0.8,
                            SafetyLevel::RequiresReview,
                            Some("Convert to multiple binding statements".to_string()),
                        ));
                    }
                }
            }
        }

        Ok(proposals)
    }
}

//--------------------------------------------------------------------------------------------------
// E0107: Wrong number of generic arguments
//--------------------------------------------------------------------------------------------------

/// **Strategy for E0107: wrong number of generic arguments.**
#[derive(Debug)]
pub(super) struct E0107WrongNumberOfGenericArgs;

impl CorrectionStrategy for E0107WrongNumberOfGenericArgs {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0107
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;

        // Extract information from the diagnostic message
        let msg = &context.diagnostic.message;
        // Most E0107 errors follow patterns like:
        // "this struct takes X generic parameter but Y parameters were supplied"
        // or "expected X type parameters, found Y"

        if let Ok((expected_count, found_count)) = extract_generic_param_counts(msg)? {
            // Case 1: Too many generic parameters - remove extras
            if found_count > expected_count && found_count > 0 {
                // Try to locate the generic params in the original code by looking for angle brackets
                if let Ok((open_bracket, close_bracket)) = find_angle_brackets(original_code)? {
                    let generic_args = &original_code[open_bracket + 1..close_bracket];
                    let params: Vec<&str> = generic_args.split(',').map(str::trim).collect();

                    // If we can identify the correct number of parameters, create a proposal
                    if params.len() >= found_count && expected_count <= params.len() {
                        let corrected_params = params
                            .into_iter()
                            .take(expected_count)
                            .collect::<Vec<_>>()
                            .join(", ");

                        let mut corrected_code = original_code.to_string();
                        corrected_code
                            .replace_range(open_bracket + 1..close_bracket, &corrected_params);

                        proposals.push(CorrectionProposal::new(
                            ProposalStrategy::ReplaceText {
                                original: original_code.clone(),
                                replacement: corrected_code,
                            },
                            0.9,
                            SafetyLevel::RequiresReview,
                            Some(format!("Remove extra generic parameters (keeping the first {expected_count})")),
                        ));
                    }
                }
            }

            // Case 2: Too few generic parameters - add placeholders
            if found_count < expected_count {
                let missing_count = expected_count - found_count;

                // Try to locate the generic params section
                let replacement = if let Ok((open_bracket, close_bracket)) =
                    find_angle_brackets(original_code)?
                {
                    // There are some generic parameters already
                    let existing_params = &original_code[open_bracket + 1..close_bracket].trim();

                    let mut placeholders = Vec::new();
                    for i in 0..missing_count {
                        placeholders.push(format!("Type{}", i + found_count + 1));
                    }

                    let new_params = if existing_params.is_empty() {
                        placeholders.join(", ")
                    } else {
                        format!("{}, {}", existing_params, placeholders.join(", "))
                    };

                    let mut result = original_code.clone();
                    result.replace_range(open_bracket + 1..close_bracket, &new_params);
                    result
                } else {
                    // No angle brackets found, try to add them
                    if let Ok(last_ident) = find_last_identifier(original_code)? {
                        let pos = original_code.rfind(last_ident).unwrap_or(0) + last_ident.len();
                        let mut corrected = original_code[..pos].to_string();
                        corrected.push('<');
                        for i in 0..missing_count {
                            if i > 0 {
                                corrected.push_str(", ");
                            }
                            corrected.push_str(&format!("Type{}", i + 1));
                        }
                        corrected.push('>');
                        corrected.push_str(&original_code[pos..]);
                        corrected
                    } else {
                        original_code.clone() // Fallback
                    }
                };

                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement,
                    },
                    0.85,
                    SafetyLevel::RequiresReview,
                    Some(format!(
                        "Add {missing_count} missing generic type parameters"
                    )),
                ));
            }
        }

        // If we couldn't generate specific fixes, provide a generic guidance proposal
        if proposals.is_empty() {
            proposals.push(CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: format!(
                        "// TODO: Fix generic parameter count in {}",
                        context
                            .problematic_node
                            .node_path
                            .last()
                            .map_or("this type", |s| s.as_str())
                    ),
                },
                0.7,
                SafetyLevel::RequiresReview,
                Some(
                    "Adjust the number of generic parameters to match the type definition"
                        .to_string(),
                ),
            ));
        }

        Ok(proposals)
    }
}

//--------------------------------------------------------------------------------------------------
// E0277: Trait not implemented
//--------------------------------------------------------------------------------------------------

/// **Strategy for E0277: trait not implemented.**
#[derive(Debug)]
pub(super) struct E0277TraitNotImplemented;

impl CorrectionStrategy for E0277TraitNotImplemented {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0277
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;

        // Parse the error message to extract trait and type information
        let msg = &context.diagnostic.message;
        // Common pattern: "the trait bound `Type: Trait` is not satisfied"
        if let Ok((type_name, trait_name)) = extract_trait_bound(msg)? {
            // Proposal 1: Implement the trait
            if generate_trait_implementation(&type_name, &trait_name).is_ok() {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::Generic {
                        description: format!("Implement {trait_name} for {type_name}"),
                    },
                    0.9,
                    SafetyLevel::RequiresReview,
                    Some(format!("Add trait implementation for {trait_name}")),
                ));
            }

            // Proposal 2: Import trait if it might not be in scope
            if !trait_name.contains("::") {
                match is_trait_in_scope(&trait_name, context)? {
                    false => {
                        let mut common_imports = Vec::new();

                        // Check common crates for this trait
                        for crate_name in &["std", "core", "alloc"] {
                            common_imports.push(format!("use {crate_name}::{trait_name};\n"));
                        }

                        // Also suggest a generic import for custom traits
                        common_imports.push(format!("use crate::traits::{trait_name};\n"));

                        for import in &common_imports {
                            proposals.push(CorrectionProposal::new(
                                ProposalStrategy::ImportAddition {
                                    import_path: import.clone(),
                                },
                                0.8,
                                SafetyLevel::RequiresReview,
                                Some(format!("Import the {trait_name} trait")),
                            ));
                        }
                    }
                    true => {}
                }
            }

            // Proposal 3: Use a type that implements the trait
            if let Ok(alternative_types) = get_alternative_types_for_trait(&type_name, &trait_name)?
            {
                for alt_type in alternative_types {
                    let suggestion = format!("Use {alt_type} which implements {trait_name}");

                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::Generic {
                            description: format!("Replace {type_name} with {alt_type}"),
                        },
                        0.7,
                        SafetyLevel::RequiresReview,
                        Some(suggestion),
                    ));
                }
            } else {
                // Handle error silently, we'll provide fallback proposal later
            }
        } else {
            // Unable to extract trait and type information, handled by fallback later
        }

        // Special case: Check for `Copy` trait not implemented for borrowed types
        if msg.contains("Copy") && msg.contains("borrowed") {
            // Suggest adding .clone()
            proposals.push(CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Add .clone() to create an owned copy".to_string(),
                },
                0.85,
                SafetyLevel::RequiresReview,
                Some("Replace the borrowed value with a cloned version".to_string()),
            ));
        }

        // Special case: Suggest deriving common traits
        for trait_name in &["Clone", "Copy", "Debug", "PartialEq", "Eq", "Hash"] {
            if msg.contains(trait_name) {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::Generic {
                        description: format!("Add #[derive({trait_name})]"),
                    },
                    0.8,
                    SafetyLevel::RequiresReview,
                    Some(format!("Derive the {trait_name} trait for this type")),
                ));
            }
        }

        // If we couldn't generate specific fixes, provide a generic guidance proposal
        if proposals.is_empty() {
            proposals.push(CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: format!(
                        "// TODO: Implement required trait for {}",
                        context
                            .problematic_node
                            .node_path
                            .last()
                            .map_or("this type", |s| s.as_str())
                    ),
                },
                0.7,
                SafetyLevel::RequiresReview,
                Some("Implement the required trait".to_string()),
            ));
        }

        Ok(proposals)
    }
}

//--------------------------------------------------------------------------------------------------
// E0308: Type mismatch
//--------------------------------------------------------------------------------------------------

/// **Strategy for E0308: type mismatch.**
#[derive(Debug)]
pub(super) struct E0308TypeMismatch;

impl CorrectionStrategy for E0308TypeMismatch {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0308
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;

        // Parse the error message to extract expected and found type information
        let msg = &context.diagnostic.message;
        if let Ok((expected_type, found_type)) = extract_type_mismatch(msg)? {
            // Proposal 1: Type conversion with common methods
            let conversions = get_type_conversions(&found_type, &expected_type)?;

            for (conversion_method, confidence) in conversions {
                if let Ok(conversion_code) = apply_conversion(original_code, &conversion_method)? {
                    if conversion_code != *original_code {
                        proposals.push(CorrectionProposal::new(
                            ProposalStrategy::TypeConversion {
                                from_type: found_type.clone(),
                                to_type: expected_type.clone(),
                                conversion_method: conversion_method.clone(),
                            },
                            confidence,
                            SafetyLevel::RequiresReview,
                            Some(format!(
                                "Convert {found_type} to {expected_type} using {conversion_method}"
                            )),
                        ));

                        // Also add a replacement proposal with the actual code
                        proposals.push(CorrectionProposal::new(
                            ProposalStrategy::ReplaceText {
                                original: original_code.clone(),
                                replacement: conversion_code,
                            },
                            confidence * 0.9, // Slightly lower confidence for direct replacement
                            SafetyLevel::RequiresReview,
                            Some(format!("Apply conversion: {conversion_method}")),
                        ));
                    }
                }
            }

            // Proposal 2: Add explicit type annotation
            let mut with_annotation = original_code.clone();

            // Simple heuristic to add a type annotation at the end
            if !with_annotation.contains(':') {
                with_annotation.push_str(": ");
                with_annotation.push_str(&expected_type);
            }

            proposals.push(CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: with_annotation,
                },
                0.7,
                SafetyLevel::RequiresReview,
                Some(format!("Add explicit type annotation: {expected_type}")),
            ));

            // Proposal 3: Special case for Option/Result unwrapping
            if expected_type.contains("Option<") || expected_type.contains("Result<") {
                let unwrap_code = format!("{}.unwrap()", original_code.trim());
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: unwrap_code,
                    },
                    0.6, // Lower confidence because unwrap can panic
                    SafetyLevel::RequiresReview,
                    Some("Unwrap the Option/Result to get inner value".to_string()),
                ));

                // Safer alternative with default/fallback
                if expected_type.contains("Option<") {
                    let unwrap_or_code =
                        format!("{}.unwrap_or(Default::default())", original_code.trim());
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: unwrap_or_code,
                        },
                        0.7,
                        SafetyLevel::RequiresReview,
                        Some("Safely unwrap Option with default value".to_string()),
                    ));
                } else if expected_type.contains("Result<") {
                    let unwrap_or_code = format!(
                        "{}.unwrap_or_else(|_| Default::default())",
                        original_code.trim()
                    );
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: unwrap_or_code,
                        },
                        0.7,
                        SafetyLevel::RequiresReview,
                        Some("Safely unwrap Result with default value".to_string()),
                    ));
                }
            }

            // Proposal 4: Special case for reference mismatches
            if (expected_type.starts_with('&') && !found_type.starts_with('&'))
                || (!expected_type.starts_with('&') && found_type.starts_with('&'))
            {
                if !found_type.starts_with('&') && expected_type.starts_with('&') {
                    // Need to add a reference
                    let with_ref = format!("&{}", original_code.trim());
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: with_ref,
                        },
                        0.85,
                        SafetyLevel::RequiresReview,
                        Some("Add reference to match expected type".to_string()),
                    ));
                } else if found_type.starts_with('&') && !expected_type.starts_with('&') {
                    // Need to dereference or clone
                    let with_deref = format!("*{}", original_code.trim());
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: with_deref,
                        },
                        0.8,
                        SafetyLevel::RequiresReview,
                        Some("Dereference to match expected type".to_string()),
                    ));

                    let with_clone = format!("{}.clone()", original_code.trim());
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: with_clone,
                        },
                        0.75,
                        SafetyLevel::RequiresReview,
                        Some("Clone to get owned value".to_string()),
                    ));
                }
            }

            // Proposal 5: Replace with a call to collect() for iterator type mismatches
            if found_type.contains("Iter") && !expected_type.contains("Iter") {
                let collect_code =
                    format!("{}.collect::<{}>()", original_code.trim(), expected_type);
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: collect_code,
                    },
                    0.75,
                    SafetyLevel::RequiresReview,
                    Some(format!("Collect iterator into {expected_type}")),
                ));
            }
        }

        // If we couldn't generate specific fixes, provide a generic guidance proposal
        if proposals.is_empty() {
            proposals.push(CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: format!(
                        "// TODO: Fix type mismatch in {}",
                        context
                            .problematic_node
                            .node_path
                            .last()
                            .map_or("this expression", |s| s.as_str())
                    ),
                },
                0.7,
                SafetyLevel::RequiresReview,
                Some("Fix the type mismatch".to_string()),
            ));
        }

        Ok(proposals)
    }
}

//--------------------------------------------------------------------------------------------------
// E0425: Unresolved name
//--------------------------------------------------------------------------------------------------

/// **Strategy for E0425: unresolved name.**
#[derive(Debug)]
pub(super) struct E0425UnresolvedName;

impl CorrectionStrategy for E0425UnresolvedName {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0425
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;
        let msg = &context.diagnostic.message;

        if let Ok(unresolved_name) = extract_unresolved_name(msg)? {
            // Strategy 1: Suggest similar names in scope (typo correction)
            if let Ok(similar_names) = find_similar_names_in_scope(&unresolved_name, context)? {
                for similar_name in similar_names.into_iter().take(3) {
                    let corrected_code =
                        replace_identifier(original_code, &unresolved_name, &similar_name)?;

                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: corrected_code,
                        },
                        0.9,
                        SafetyLevel::RequiresReview,
                        Some(format!("Replace '{unresolved_name}' with '{similar_name}'")),
                    ));
                }
            }

            // Strategy 2: Suggest common imports for the identifier
            if let Ok(import_suggestions) = suggest_imports_for_identifier(&unresolved_name)? {
                for import_path in import_suggestions.into_iter().take(3) {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ImportAddition {
                            import_path: format!("use {import_path};\n"),
                        },
                        0.8,
                        SafetyLevel::RequiresReview,
                        Some(format!("Import {import_path}")),
                    ));
                }
            }

            // Strategy 3: Declare variable if it looks like a variable usage
            if is_variable_usage_context(context) && is_valid_identifier(&unresolved_name) {
                let variable_declaration =
                    format!("let {unresolved_name} = /* TODO: provide value */;\n{original_code}");

                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: variable_declaration,
                    },
                    0.7,
                    SafetyLevel::RequiresReview,
                    Some(format!("Declare variable '{unresolved_name}'")),
                ));
            }
        }

        Ok(proposals)
    }
}

//--------------------------------------------------------------------------------------------------
// E0433: Failed to resolve
//--------------------------------------------------------------------------------------------------

/// **Strategy for E0433: failed to resolve.**
///
/// Handles cases where Rust fails to resolve a path, typically due to missing imports
/// or incorrect module paths. This strategy provides intelligent suggestions for:
///
/// - Adding missing `use` statements
/// - Correcting module path typos
/// - Suggesting alternative import paths
/// - Converting relative to absolute paths
///
/// ```rust
/// // Error: failed to resolve: use of undeclared crate or module `serde`
/// use serde::Serialize; // E0433
///
/// // Suggested fixes:
/// // 1. Add serde to Cargo.toml dependencies
/// // 2. Use std alternative: #[derive(Debug)]
/// // 3. Check for typos: use std::serialize
///```
#[derive(Debug)]
pub(super) struct E0433FailedToResolve;

impl CorrectionStrategy for E0433FailedToResolve {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0433
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;
        let msg = &context.diagnostic.message;

        // Extract the failed resolution path from error message
        if let Ok((failed_path, resolution_type)) = extract_failed_resolution_info(msg)? {
            // Strategy 1: Suggest common crate imports
            if let Ok(crate_suggestions) = suggest_crate_imports(&failed_path)? {
                for (crate_name, import_path) in crate_suggestions.into_iter().take(3) {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ImportAddition {
                            import_path: format!("use {import_path};\n"),
                        },
                        0.9,
                        SafetyLevel::RequiresReview,
                        Some(format!(
                            "Add {crate_name} dependency and import {import_path}"
                        )),
                    ));
                }
            }

            // Strategy 2: Fix path typos
            if let Ok(corrected_paths) = suggest_path_corrections(&failed_path, &resolution_type)? {
                for corrected_path in corrected_paths.into_iter().take(3) {
                    let corrected_code =
                        replace_path_in_code(original_code, &failed_path, &corrected_path)?;

                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: corrected_code,
                        },
                        0.85,
                        SafetyLevel::RequiresReview,
                        Some(format!("Replace '{failed_path}' with '{corrected_path}'")),
                    ));
                }
            }

            // Strategy 3: Convert to absolute path
            if failed_path.starts_with("crate::") || failed_path.contains("::") {
                if let Ok(absolute_path) = convert_to_absolute_path(&failed_path)? {
                    let absolute_code =
                        replace_path_in_code(original_code, &failed_path, &absolute_path)?;

                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: absolute_code,
                        },
                        0.8,
                        SafetyLevel::RequiresReview,
                        Some("Convert to absolute path".to_string()),
                    ));
                }
            }

            // Strategy 4: Suggest std library alternatives
            if let Ok(std_alternatives) = suggest_std_alternatives(&failed_path)? {
                for alternative in std_alternatives.into_iter().take(2) {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ImportAddition {
                            import_path: format!("use {alternative};\n"),
                        },
                        0.75,
                        SafetyLevel::RequiresReview,
                        Some(format!("Use standard library alternative: {alternative}")),
                    ));
                }
            }
        }

        Ok(proposals)
    }
}

//--------------------------------------------------------------------------------------------------
// E0499: Mutable borrow in loop
//--------------------------------------------------------------------------------------------------

/// **Strategy for E0499: mutable borrow in loop.**
///
/// Handles borrowing conflicts that occur when trying to mutably borrow a value
/// multiple times within a loop. This strategy provides solutions for:
///
/// - Converting to owned values to avoid borrowing issues
/// - Using `RefCell` for interior mutability
/// - Restructuring loops to avoid multiple borrows
/// - Using iterators instead of manual indexing
///
/// ```rust
/// let mut vec = vec![1, 2, 3];
/// for i in 0..vec.len() {
///     let item = &mut vec[i]; // E0499: cannot borrow as mutable more than once
///     // ... use item
/// }
///
/// // Suggested fixes:
/// // 1. Use iterator: for item in vec.iter_mut() { ... }
/// // 2. Use indices differently: while let Some(item) = vec.get_mut(index) { ... }
/// // 3. Clone values: for item in vec.clone().into_iter() { ... }
///```
#[derive(Debug)]
pub(super) struct E0499MutableBorrowInLoop;

impl CorrectionStrategy for E0499MutableBorrowInLoop {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0499
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;
        let msg = &context.diagnostic.message;

        // Extract borrow information from error message
        if let Ok(borrow_info) = extract_mutable_borrow_info(msg)? {
            // Strategy 1: Convert to iterator-based approach
            if let Ok(iterator_code) =
                convert_to_iterator_pattern_e0499(original_code, &borrow_info)?
            {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: iterator_code,
                    },
                    0.95,
                    SafetyLevel::RequiresReview,
                    Some("Convert to iterator-based loop to avoid borrowing conflicts".to_string()),
                ));
            }

            // Strategy 2: Use RefCell for interior mutability
            if let Ok(refcell_code) = convert_to_refcell_pattern(original_code, &borrow_info)? {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: refcell_code,
                    },
                    0.85,
                    SafetyLevel::RequiresReview,
                    Some("Use RefCell for interior mutability".to_string()),
                ));
            }

            // Strategy 3: Clone values to avoid borrowing
            if let Ok(clone_code) = convert_to_owned_values(original_code, &borrow_info)? {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: clone_code,
                    },
                    0.8,
                    SafetyLevel::RequiresReview,
                    Some("Clone values to avoid borrowing conflicts".to_string()),
                ));
            }

            // Strategy 4: Restructure loop to avoid multiple borrows
            if let Ok(restructured_code) = restructure_loop_borrowing(original_code, &borrow_info)?
            {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: restructured_code,
                    },
                    0.9,
                    SafetyLevel::RequiresReview,
                    Some("Restructure loop to avoid multiple mutable borrows".to_string()),
                ));
            }

            // Note: Unsafe strategies are intentionally omitted for safety
        }

        Ok(proposals)
    }
}

//--------------------------------------------------------------------------------------------------
// E0597: Borrowed value does not live long enough
//--------------------------------------------------------------------------------------------------

/// **Strategy for E0597: borrowed value does not live long enough.**
///
/// Handles lifetime issues where a borrowed value doesn't live long enough for its usage.
/// This strategy provides solutions for:
///
/// - Moving values to outer scopes to extend lifetimes
/// - Cloning values to create owned copies
/// - Using `Cow` for efficient clone-on-write semantics
/// - Restructuring code to avoid temporary borrows
///
/// ```rust
/// // Error case: borrowed value does not live long enough - showing the fixes
///
/// // Fix 1: Return owned value
/// fn get_string_owned() -> String {
///     String::from("hello")
/// }
///
/// // Fix 2: Use static string
/// fn get_string_static() -> &'static str {
///     "hello"
/// }
///
/// // Fix 3: Accept lifetime parameter
/// fn get_string_with_lifetime(s: &str) -> &str {
///     s
/// }
///```
#[derive(Debug)]
pub(super) struct E0597BorrowedValueDoesNotLiveLongEnough;

impl CorrectionStrategy for E0597BorrowedValueDoesNotLiveLongEnough {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0597
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;
        let msg = &context.diagnostic.message;

        // Extract lifetime information from error message
        if let Ok(lifetime_info) = extract_lifetime_info(msg)? {
            // Strategy 1: Convert to owned value
            if let Ok(owned_code) = convert_to_owned_value(original_code, &lifetime_info)? {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: owned_code,
                    },
                    0.9,
                    SafetyLevel::RequiresReview,
                    Some("Convert to owned value to extend lifetime".to_string()),
                ));
            }

            // Strategy 2: Move value to outer scope
            if let Ok(moved_code) = move_value_to_outer_scope(original_code, &lifetime_info)? {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: moved_code,
                    },
                    0.85,
                    SafetyLevel::RequiresReview,
                    Some("Move value to outer scope to extend lifetime".to_string()),
                ));
            }

            // Strategy 3: Use Cow for efficient clone-on-write
            if let Ok(cow_code) = convert_to_cow_pattern(original_code, &lifetime_info)? {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: cow_code,
                    },
                    0.8,
                    SafetyLevel::RequiresReview,
                    Some("Use Cow for efficient clone-on-write semantics".to_string()),
                ));
            }

            // Strategy 4: Add explicit lifetime parameters
            if let Ok(lifetime_code) = add_explicit_lifetimes(original_code, &lifetime_info)? {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: lifetime_code,
                    },
                    0.75,
                    SafetyLevel::RequiresReview,
                    Some("Add explicit lifetime parameters".to_string()),
                ));
            }

            // Strategy 5: Use static references where appropriate
            if lifetime_info.can_use_static {
                if let Ok(static_code) = convert_to_static_reference(original_code, &lifetime_info)?
                {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: static_code,
                        },
                        0.7,
                        SafetyLevel::RequiresReview,
                        Some("Use static reference for compile-time known values".to_string()),
                    ));
                }
            }
        }

        Ok(proposals)
    }
}

//--------------------------------------------------------------------------------------------------
// E0599: Method not found
//--------------------------------------------------------------------------------------------------

/// **Strategy for E0599: method not found.**
///
/// Handles cases where a method is called on a type that doesn't implement it.
/// This strategy provides solutions for:
///
/// - Suggesting similar method names (typo correction)
/// - Importing traits that provide the method
/// - Converting to associated function calls
/// - Adding dereference for wrapped types
/// - Suggesting alternative types that have the method
///
/// ```rust
/// // Error case: no method named `push_str` found for type `Vec<char>` - showing the fixes
///
/// // Fix 1: Use correct method for Vec<char>
/// let mut chars = vec!['h', 'e'];
/// chars.extend("llo".chars());
/// assert_eq!(chars, vec!['h', 'e', 'l', 'l', 'o']);
///
/// // Fix 2: Convert to String first
/// let mut chars2 = vec!['h', 'e'];
/// let mut s = String::from_iter(chars2);
/// s.push_str("llo");
/// assert_eq!(s, "hello");
///```
#[derive(Debug)]
pub(super) struct E0599MethodNotFound;

impl CorrectionStrategy for E0599MethodNotFound {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0599
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;
        let msg = &context.diagnostic.message;

        if let Ok((method_name, type_name)) = extract_method_not_found_info(msg)? {
            // Strategy 1: Suggest similar method names (typo correction)
            if let Ok(similar_methods) =
                find_similar_methods_for_type(&method_name, &type_name, context)?
            {
                for similar_method in similar_methods.into_iter().take(3) {
                    let corrected_code =
                        replace_method_name(original_code, &method_name, &similar_method)?;

                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: corrected_code,
                        },
                        0.9,
                        SafetyLevel::RequiresReview,
                        Some(format!(
                            "Replace '{method_name}' with similar method '{similar_method}'"
                        )),
                    ));
                }
            }

            // Strategy 2: Import traits that provide the method
            if let Ok(trait_imports) = find_traits_providing_method(&method_name, &type_name)? {
                for trait_path in trait_imports.into_iter().take(3) {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ImportAddition {
                            import_path: format!("use {trait_path};\n"),
                        },
                        0.85,
                        SafetyLevel::RequiresReview,
                        Some(format!("Import trait {trait_path} to enable method")),
                    ));
                }
            }

            // Strategy 3: Convert to associated function call
            if is_static_method_available(&method_name, &type_name, context) {
                let associated_call =
                    convert_to_associated_function(original_code, &method_name, &type_name)?;

                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: associated_call,
                    },
                    0.8,
                    SafetyLevel::RequiresReview,
                    Some(format!(
                        "Convert to {type_name}::{method_name} associated function call"
                    )),
                ));
            }

            // Strategy 4: Add dereference if method exists on inner type
            if type_name.contains('&')
                || type_name.contains("Box")
                || type_name.contains("Rc")
                || type_name.contains("Arc")
            {
                if let Ok(dereferenced_call) = add_dereference_for_method(original_code)? {
                    if dereferenced_call != *original_code {
                        proposals.push(CorrectionProposal::new(
                            ProposalStrategy::ReplaceText {
                                original: original_code.clone(),
                                replacement: dereferenced_call,
                            },
                            0.75,
                            SafetyLevel::RequiresReview,
                            Some("Add dereference to access method on inner type".to_string()),
                        ));
                    }
                }
            }

            // Strategy 5: Convert type to one that has the method
            if let Ok(compatible_types) = find_types_with_method(&method_name)? {
                for compatible_type in compatible_types.into_iter().take(2) {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::Generic {
                            description: format!(
                                "Convert to {compatible_type} which has method {method_name}"
                            ),
                        },
                        0.7,
                        SafetyLevel::RequiresReview,
                        Some(format!("Consider using {compatible_type} instead")),
                    ));
                }
            }
        }

        Ok(proposals)
    }
}

//--------------------------------------------------------------------------------------------------
// E0659: Ambiguous item
//--------------------------------------------------------------------------------------------------

/// **Strategy for E0659: ambiguous item.**
///
/// Handles cases where an item name is ambiguous due to multiple imports.
/// This strategy provides solutions for:
///
/// - Using fully qualified syntax to disambiguate
/// - Creating aliased imports to resolve conflicts
/// - Removing conflicting imports and using explicit paths
/// - Scoping usage to resolve ambiguity
///
/// ```rust
/// // Error case: `Result` is ambiguous - showing the fix
/// use std::io::Result as IoResult;
/// use std::fmt::Result as FmtResult;
///
/// // Now we can use both without ambiguity:
/// fn example() -> IoResult<()> {
///     Ok(())
/// }
///
/// fn format_example() -> FmtResult {
///     Ok(())
/// }
///```
#[derive(Debug)]
pub(super) struct E0659AmbiguousItem;

impl CorrectionStrategy for E0659AmbiguousItem {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0659
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;
        let msg = &context.diagnostic.message;

        if let Ok((ambiguous_item, conflicting_paths)) = extract_ambiguous_item_info(msg)? {
            // Strategy 1: Use fully qualified syntax for each conflicting path
            for (i, qualified_path) in conflicting_paths.into_iter().enumerate().take(3) {
                let disambiguated_code =
                    replace_with_qualified_syntax(original_code, &ambiguous_item, &qualified_path)?;

                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: disambiguated_code,
                    },
                    0.9 - (i as f64 * 0.1), // Decrease confidence for later options
                    SafetyLevel::RequiresReview,
                    Some(format!("Use fully qualified path: {qualified_path}")),
                ));
            }

            // Strategy 2: Use aliased imports to resolve ambiguity
            if let Ok(alias_suggestions) = suggest_import_aliases(&ambiguous_item, context)? {
                for (alias, import_path) in alias_suggestions.into_iter().take(2) {
                    let aliased_import = format!("use {import_path} as {alias};\n");
                    if let Ok(aliased_code) =
                        replace_identifier(original_code, &ambiguous_item, &alias)?
                    {
                        if aliased_code != *original_code {
                            proposals.push(CorrectionProposal::new(
                                ProposalStrategy::ReplaceText {
                                    original: original_code.clone(),
                                    replacement: format!("{aliased_import}\n{aliased_code}"),
                                },
                                0.85,
                                SafetyLevel::RequiresReview,
                                Some(format!("Use aliased import: {import_path} as {alias}")),
                            ));
                        }
                    }
                }
            }

            // Strategy 3: Remove conflicting imports and use explicit paths
            if let Ok(explicit_usage) =
                convert_to_explicit_usage(original_code, &ambiguous_item, context)?
            {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: explicit_usage,
                    },
                    0.8,
                    SafetyLevel::RequiresReview,
                    Some("Remove ambiguous imports and use explicit paths".to_string()),
                ));
            }

            // Strategy 4: Scope the usage to resolve ambiguity
            if let Ok(scoped_usage) = create_scoped_usage(original_code, &ambiguous_item)? {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: scoped_usage,
                    },
                    0.75,
                    SafetyLevel::RequiresReview,
                    Some("Use scoped resolution to disambiguate".to_string()),
                ));
            }
        }

        Ok(proposals)
    }
}

//--------------------------------------------------------------------------------------------------
// Helper Functions for Error Correction Strategies (Alphabetically Organized)
//--------------------------------------------------------------------------------------------------

/// Information about mutable borrow conflicts
#[derive(Debug)]
struct MutableBorrowInfo {
    variable_name: String,
}

/// Information about lifetime issues
#[derive(Debug)]
struct LifetimeInfo {
    variable_name: String,
    can_use_static: bool,
    is_string_literal: bool,
}

/// Add dereference for method access
fn add_dereference_for_method(code: &str) -> Result<String, CorrectionStrategyError> {
    // Simple heuristic to add dereference
    if let Some(dot_pos) = code.find('.') {
        let before_dot = &code[..dot_pos];
        let after_dot = &code[dot_pos..];

        if !before_dot.trim().starts_with('*') {
            return Ok(format!("(*{before_dot}){after_dot}"));
        }
    }

    Ok(code.to_string())
}

/// Add explicit lifetimes
fn add_explicit_lifetimes(
    code: &str,
    _lifetime_info: &LifetimeInfo,
) -> Result<String, CorrectionStrategyError> {
    let lifetime_code = code
        .replace("fn ", "fn ")
        .replace("() ->", "<'a>() ->")
        .replace("&str", "&'a str")
        .replace("&String", "&'a String");
    Ok(lifetime_code)
}

/// Add missing fields to struct pattern
fn add_missing_struct_fields(
    code: &str,
    missing_fields: &[String],
) -> Result<String, CorrectionStrategyError> {
    let field_bindings: Vec<String> = missing_fields
        .iter()
        .map(|field| format!("{}: _{}", field, field))
        .collect();

    let result = if code.trim().ends_with('}') {
        let insert_pos =
            code.rfind('}')
                .ok_or_else(|| CorrectionStrategyError::AstAnalysisFailed {
                    context: "Could not find closing brace in struct pattern".to_string(),
                })?;
        format!(
            "{}, {}{}",
            &code[..insert_pos],
            field_bindings.join(", "),
            &code[insert_pos..]
        )
    } else {
        format!("{}, {}", code, field_bindings.join(", "))
    };
    Ok(result)
}

/// Add rest pattern to struct pattern
fn add_rest_pattern_to_struct(code: &str) -> Result<String, CorrectionStrategyError> {
    let result = if code.trim().ends_with('}') {
        let insert_pos =
            code.rfind('}')
                .ok_or_else(|| CorrectionStrategyError::AstAnalysisFailed {
                    context: "Could not find closing brace in struct pattern".to_string(),
                })?;
        format!("{}, ..{}", &code[..insert_pos], &code[insert_pos..])
    } else {
        format!("{}, ..", code)
    };
    Ok(result)
}

/// Add rest pattern (..) to tuple
fn add_rest_pattern_to_tuple(code: &str) -> Result<String, CorrectionStrategyError> {
    let result = if code.trim().ends_with(')') {
        let insert_pos =
            code.rfind(')')
                .ok_or_else(|| CorrectionStrategyError::AstAnalysisFailed {
                    context: "Could not find closing parenthesis in tuple pattern".to_string(),
                })?;
        format!("{}, ..{}", &code[..insert_pos], &code[insert_pos..])
    } else {
        format!("{}, ..", code)
    };
    Ok(result)
}

/// Add placeholder fields to tuple pattern
fn add_tuple_pattern_placeholders(
    code: &str,
    missing_count: usize,
) -> Result<String, CorrectionStrategyError> {
    let placeholders: Vec<String> = (0..missing_count).map(|i| format!("_{}", i + 1)).collect();
    let result = if code.trim().ends_with(')') {
        let insert_pos =
            code.rfind(')')
                .ok_or_else(|| CorrectionStrategyError::AstAnalysisFailed {
                    context: "Could not find closing parenthesis in tuple pattern".to_string(),
                })?;
        format!(
            "{}, {}{}",
            &code[..insert_pos],
            placeholders.join(", "),
            &code[insert_pos..]
        )
    } else {
        format!("{}, {}", code, placeholders.join(", "))
    };
    Ok(result)
}

/// Adjust tuple pattern to have correct number of fields
fn adjust_tuple_pattern_fields(
    code: &str,
    expected: usize,
    found: usize,
) -> Result<String, CorrectionStrategyError> {
    if found > expected {
        // Remove extra fields from the end
        let mut result = code.to_string();
        for _ in 0..(found - expected) {
            if let Some(last_comma) = result.rfind(',') {
                result = result[..last_comma].trim().to_string();
            }
        }
        Ok(result)
    } else {
        Ok(code.to_string())
    }
}

/// Apply a conversion method to a code snippet
fn apply_conversion(
    original: &str,
    conversion_method: &str,
) -> Result<String, CorrectionStrategyError> {
    // Validate inputs
    if original.is_empty() {
        return Err(CorrectionStrategyError::AstAnalysisFailed {
            context: "Cannot apply conversion to empty code".to_string(),
        });
    }

    if conversion_method.is_empty() {
        return Err(CorrectionStrategyError::AstAnalysisFailed {
            context: "Cannot apply empty conversion method".to_string(),
        });
    }

    let result = if conversion_method.ends_with("()") {
        // Method call
        format!("{}.{}", original.trim(), conversion_method)
    } else if conversion_method.starts_with("as ") {
        // Type cast
        format!("({} {})", original.trim(), conversion_method)
    } else if !conversion_method.contains('(') {
        // Operator or simple suffix
        format!("{} {}", original.trim(), conversion_method)
    } else {
        // Function call
        format!("{}({})", conversion_method, original.trim())
    };

    Ok(result)
}

/// Check if pattern can use pattern guard
fn can_use_pattern_guard(code: &str) -> bool {
    // Simple heuristic: check if we're in a match arm context
    code.contains("=>") || code.contains("match")
}

/// Convert to absolute path
fn convert_to_absolute_path(path: &str) -> Result<String, CorrectionStrategyError> {
    if path.starts_with("crate::") {
        Ok(format!("::{}", &path[7..]))
    } else if !path.starts_with("::") {
        Ok(format!("::{path}"))
    } else {
        Ok(path.to_string())
    }
}

/// Convert to associated function
fn convert_to_associated_function(
    code: &str,
    method: &str,
    type_name: &str,
) -> Result<String, CorrectionStrategyError> {
    let pattern = format!(r"(\w+)\.{}\(", regex::escape(method));
    let regex =
        regex::Regex::new(&pattern).map_err(|e| CorrectionStrategyError::RegexCompilation {
            pattern: format!("Failed to create regex pattern: {e}"),
        })?;

    let replacement = format!("{type_name}::{method}(");
    let result = regex.replace_all(code, replacement.as_str()).to_string();
    Ok(result)
}

/// Convert to Cow pattern
fn convert_to_cow_pattern(
    code: &str,
    _lifetime_info: &LifetimeInfo,
) -> Result<String, CorrectionStrategyError> {
    let cow_code = code
        .replace("&str", "std::borrow::Cow<'_, str>")
        .replace("&String", "std::borrow::Cow<'_, str>");
    Ok(format!("use std::borrow::Cow;\n{cow_code}"))
}

/// Convert to destructuring assignment
fn convert_to_destructuring_assignment(
    code: &str,
    missing_fields: &[String],
    struct_name: &str,
) -> Result<String, CorrectionStrategyError> {
    let mut assignments = Vec::new();

    // Add the original pattern
    assignments.push(code.to_string());

    // Add assignments for missing fields
    for field in missing_fields {
        assignments.push(format!(
            "let {} = {}.{};",
            field,
            struct_name.to_lowercase(),
            field
        ));
    }

    Ok(assignments.join("\n"))
}

/// Convert to explicit usage
fn convert_to_explicit_usage(
    code: &str,
    _item: &str,
    _context: &ASTContext,
) -> Result<String, CorrectionStrategyError> {
    // This would require sophisticated analysis to remove imports and use explicit paths
    Ok(format!(
        "// TODO: Remove conflicting imports and use explicit paths\n{code}"
    ))
}

/// Convert to iterator pattern for E0499
fn convert_to_iterator_pattern_e0499(
    code: &str,
    borrow_info: &MutableBorrowInfo,
) -> Result<String, CorrectionStrategyError> {
    if code.contains("for ") && code.contains(&borrow_info.variable_name) {
        let iterator_version = code
            .replace(
                &format!("for i in 0..{}.len()", borrow_info.variable_name),
                &format!("for item in {}.iter_mut()", borrow_info.variable_name),
            )
            .replace(&format!("&mut {}[i]", borrow_info.variable_name), "item");
        return Ok(iterator_version);
    }

    Ok(format!("// Convert to iterator pattern\n{}.iter_mut().for_each(|item| {{\n    // Process item\n}});", borrow_info.variable_name))
}

/// Convert to owned value
fn convert_to_owned_value(
    code: &str,
    lifetime_info: &LifetimeInfo,
) -> Result<String, CorrectionStrategyError> {
    if lifetime_info.is_string_literal {
        let owned_code = code
            .replace(
                &format!("&{}", lifetime_info.variable_name),
                &lifetime_info.variable_name,
            )
            .replace("&str", "String")
            .replace(".as_str()", "");
        return Ok(owned_code);
    }

    let owned_code = code.replace(
        &format!("&{}", lifetime_info.variable_name),
        &format!("{}.clone()", lifetime_info.variable_name),
    );
    Ok(owned_code)
}

/// Convert to owned values
fn convert_to_owned_values(
    code: &str,
    borrow_info: &MutableBorrowInfo,
) -> Result<String, CorrectionStrategyError> {
    let cloned_code = code.replace(
        &format!("&{}", borrow_info.variable_name),
        &format!("{}.clone()", borrow_info.variable_name),
    );
    Ok(cloned_code)
}

/// Convert duplicate binding to pattern guard
fn convert_to_pattern_guard(
    code: &str,
    field_name: &str,
) -> Result<String, CorrectionStrategyError> {
    // This is a simplified implementation
    // In practice, this would need more sophisticated AST analysis
    let guard_condition = format!("if {field_name}_condition");
    let result = if code.contains("=>") {
        code.replace("=>", &format!(" {guard_condition} =>"))
    } else {
        format!("{code} {guard_condition}")
    };
    Ok(result)
}

/// Convert to RefCell pattern
fn convert_to_refcell_pattern(
    _code: &str,
    borrow_info: &MutableBorrowInfo,
) -> Result<String, CorrectionStrategyError> {
    let refcell_code = format!(
        "use std::cell::RefCell;\nlet {} = RefCell::new({});\n// Use {}.borrow_mut() for mutable access",
        borrow_info.variable_name,
        borrow_info.variable_name,
        borrow_info.variable_name
    );
    Ok(refcell_code)
}

/// Convert to static reference
fn convert_to_static_reference(
    code: &str,
    _lifetime_info: &LifetimeInfo,
) -> Result<String, CorrectionStrategyError> {
    let static_code = code
        .replace("&str", "&'static str")
        .replace("&String", "&'static str");
    Ok(static_code)
}

/// Create scoped usage
fn create_scoped_usage(code: &str, item: &str) -> Result<String, CorrectionStrategyError> {
    Ok(format!("{{\n    use crate::{item};\n    {code}\n}}"))
}

/// Extract ambiguous item information from E0659 error message
fn extract_ambiguous_item_info(
    message: &str,
) -> Result<(String, Vec<String>), CorrectionStrategyError> {
    static PATTERN: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"`([^`]+)` is ambiguous").expect("Valid regex")
    });

    if let Some(captures) = PATTERN.captures(message) {
        let ambiguous_item = captures[1].to_string();

        // Extract conflicting paths - this would need more sophisticated parsing
        let paths = vec![
            format!("crate::{}", ambiguous_item),
            format!("std::{}", ambiguous_item),
            format!("{}::{}", "other_crate", ambiguous_item),
        ];

        return Ok((ambiguous_item, paths));
    }

    Ok(("unknown".to_string(), vec![]))
}

/// Extract duplicate field name from E0025 error message
fn extract_duplicate_field_name(message: &str) -> Result<String, CorrectionStrategyError> {
    static PATTERN: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"field `([^`]+)` bound more than once").expect("Valid regex")
    });

    if let Some(captures) = PATTERN.captures(message) {
        return Ok(captures[1].to_string());
    }

    Err(CorrectionStrategyError::ParseFailed {
        context: "Could not extract duplicate field name from error message".to_string(),
    })
}

/// Extract failed resolution information from E0433 error message
fn extract_failed_resolution_info(
    message: &str,
) -> Result<(String, String), CorrectionStrategyError> {
    static PATTERN: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"failed to resolve: use of undeclared (?:type|crate) `([^`]+)`")
            .expect("Valid regex")
    });

    if let Some(captures) = PATTERN.captures(message) {
        let failed_path = captures[1].to_string();
        let resolution_type = if message.contains("type") {
            "type"
        } else {
            "crate"
        }
        .to_string();
        return Ok((failed_path, resolution_type));
    }

    Ok(("unknown".to_string(), "unknown".to_string()))
}

/// Extracts the expected and found generic parameter counts from an E0107 error message
fn extract_generic_param_counts(message: &str) -> Result<(usize, usize), CorrectionStrategyError> {
    // Pre-compiled regex patterns for better performance
    static PATTERN1: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"takes\s+(\d+)[^0-9]+but\s+(\d+)").expect("Valid regex")
    });
    static PATTERN2: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"expected\s+(\d+)[^0-9]+found\s+(\d+)").expect("Valid regex")
    });

    let captures = PATTERN1
        .captures(message)
        .or_else(|| PATTERN2.captures(message));

    if let Some(captures) = captures {
        let expected =
            captures[1]
                .parse::<usize>()
                .map_err(|_| CorrectionStrategyError::ParseFailed {
                    context: format!(
                        "Could not parse expected param count from '{}'",
                        &captures[1]
                    ),
                })?;
        let found =
            captures[2]
                .parse::<usize>()
                .map_err(|_| CorrectionStrategyError::ParseFailed {
                    context: format!("Could not parse found param count from '{}'", &captures[2]),
                })?;
        return Ok((expected, found));
    }

    Err(CorrectionStrategyError::ParseFailed {
        context: "Could not extract generic parameter counts from error message".to_string(),
    })
}

/// Extract lifetime information from E0597 error message
fn extract_lifetime_info(message: &str) -> Result<LifetimeInfo, CorrectionStrategyError> {
    static PATTERN: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"`([^`]+)` does not live long enough").expect("Valid regex")
    });

    if let Some(captures) = PATTERN.captures(message) {
        let variable_name = captures[1].to_string();
        let can_use_static = message.contains("string literal") || message.contains("static");
        let is_string_literal = message.contains("string") || message.contains("str");

        return Ok(LifetimeInfo {
            variable_name,
            can_use_static,
            is_string_literal,
        });
    }

    Ok(LifetimeInfo {
        variable_name: "unknown".to_string(),
        can_use_static: false,
        is_string_literal: false,
    })
}

/// Extract method not found information from E0599 error message
fn extract_method_not_found_info(
    message: &str,
) -> Result<(String, String), CorrectionStrategyError> {
    static PATTERN: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"no method named `([^`]+)` found for (?:type|struct|enum) `([^`]+)`")
            .expect("Valid regex")
    });

    if let Some(captures) = PATTERN.captures(message) {
        return Ok((captures[1].to_string(), captures[2].to_string()));
    }

    Ok(("unknown_method".to_string(), "unknown_type".to_string()))
}

/// Extract missing fields information from E0027 error message
fn extract_missing_fields_info(
    message: &str,
) -> Result<(Vec<String>, String), CorrectionStrategyError> {
    static PATTERN: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"pattern does not mention field[s]? `([^`]+)`").expect("Valid regex")
    });

    if let Some(captures) = PATTERN.captures(message) {
        let fields_str = captures[1].to_string();
        let fields: Vec<String> = fields_str
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        // Try to extract struct name (simplified)
        let struct_name = "UnknownStruct".to_string(); // Would need more sophisticated parsing

        return Ok((fields, struct_name));
    }

    Err(CorrectionStrategyError::ParseFailed {
        context: "Could not extract missing fields info from error message".to_string(),
    })
}

/// Extract mutable borrow information from E0499 error message
fn extract_mutable_borrow_info(
    message: &str,
) -> Result<MutableBorrowInfo, CorrectionStrategyError> {
    static PATTERN: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"cannot borrow `([^`]+)` as mutable").expect("Valid regex")
    });

    if let Some(captures) = PATTERN.captures(message) {
        let variable_name = captures[1].to_string();

        return Ok(MutableBorrowInfo { variable_name });
    }

    Ok(MutableBorrowInfo {
        variable_name: "unknown".to_string(),
    })
}

/// Extract nonexistent field information from E0026 error message
fn extract_nonexistent_field_info(
    message: &str,
) -> Result<(String, String), CorrectionStrategyError> {
    static PATTERN: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"struct `([^`]+)` has no field named `([^`]+)`").expect("Valid regex")
    });

    if let Some(captures) = PATTERN.captures(message) {
        let struct_name = captures[1].to_string();
        let field_name = captures[2].to_string();
        return Ok((field_name, struct_name));
    }

    Err(CorrectionStrategyError::ParseFailed {
        context: "Could not extract nonexistent field info from error message".to_string(),
    })
}

/// Extracts type and trait names from an E0277 error message using Yoshi error handling patterns
fn extract_trait_bound(message: &str) -> Result<(String, String), CorrectionStrategyError> {
    // Precompiled regex patterns for better performance
    static PATTERN1: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"`([^:]+):\s*([^`]+)`").expect("Valid regex")
    });

    static PATTERN2: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"`([^`]+)`[^`]*implement[^`]*`([^`]+)`").expect("Valid regex")
    });

    // Match pattern like "the trait bound `Type: Trait` is not satisfied"
    if let Some(captures) = PATTERN1.captures(message) {
        let type_name = captures[1].trim().to_string();
        let trait_name = captures[2].trim().to_string();
        return Ok((type_name, trait_name));
    }

    // Alternative pattern: "`Type` doesn't implement `Trait`"
    if let Some(captures) = PATTERN2.captures(message) {
        let type_name = captures[1].trim().to_string();
        let trait_name = captures[2].trim().to_string();
        return Ok((type_name, trait_name));
    }

    Err(CorrectionStrategyError::ParseFailed {
        context: "Could not extract trait bound information from error message".to_string(),
    })
}

/// Extract tuple field counts from E0023 error message
fn extract_tuple_field_counts(message: &str) -> Result<(usize, usize), CorrectionStrategyError> {
    static PATTERN: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"expected (\d+) field[s]?, found (\d+)").expect("Valid regex")
    });

    if let Some(captures) = PATTERN.captures(message) {
        let expected =
            captures[1]
                .parse::<usize>()
                .map_err(|_| CorrectionStrategyError::ParseFailed {
                    context: format!(
                        "Could not parse expected field count from '{}'",
                        &captures[1]
                    ),
                })?;
        let found =
            captures[2]
                .parse::<usize>()
                .map_err(|_| CorrectionStrategyError::ParseFailed {
                    context: format!("Could not parse found field count from '{}'", &captures[2]),
                })?;
        return Ok((expected, found));
    }

    Err(CorrectionStrategyError::ParseFailed {
        context: "Could not extract tuple field counts from error message".to_string(),
    })
}

/// Extracts expected and found types from an E0308 error message
fn extract_type_mismatch(message: &str) -> Result<(String, String), CorrectionStrategyError> {
    // Pre-compiled regex pattern for better performance
    static PATTERN: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"expected[^`]*`([^`]+)`[^`]*found[^`]*`([^`]+)`").expect("Valid regex")
    });

    // Match pattern like "expected type `X`, found `Y`"
    if let Some(captures) = PATTERN.captures(message) {
        let expected_type = captures[1].trim().to_string();
        let found_type = captures[2].trim().to_string();
        return Ok((expected_type, found_type));
    }

    Err(CorrectionStrategyError::ParseFailed {
        context: "Could not extract type mismatch information from error message".to_string(),
    })
}

/// Extract unresolved name from E0425 error message
fn extract_unresolved_name(message: &str) -> Result<String, CorrectionStrategyError> {
    static PATTERN: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"cannot find (?:value|function|type) `([^`]+)`").expect("Valid regex")
    });

    if let Some(captures) = PATTERN.captures(message) {
        return Ok(captures[1].to_string());
    }

    Err(CorrectionStrategyError::ParseFailed {
        context: "Could not extract unresolved name from error message".to_string(),
    })
}

/// Finds angle brackets in a code snippet and returns their positions
fn find_angle_brackets(code: &str) -> Result<(usize, usize), CorrectionStrategyError> {
    let open_pos = code
        .rfind('<')
        .ok_or_else(|| CorrectionStrategyError::AstAnalysisFailed {
            context: "No opening angle bracket found in code".to_string(),
        })?;

    let remaining = &code[open_pos + 1..];

    // Handle nested brackets by counting
    let mut depth = 1;
    let mut close_pos = 0;

    for (i, c) in remaining.char_indices() {
        match c {
            '<' => depth += 1,
            '>' => {
                depth -= 1;
                if depth == 0 {
                    close_pos = i;
                    break;
                }
            }
            _ => {}
        }
    }

    if depth == 0 {
        Ok((open_pos, open_pos + 1 + close_pos))
    } else {
        Err(CorrectionStrategyError::AstAnalysisFailed {
            context: "Unmatched angle brackets in code".to_string(),
        })
    }
}

/// Finds the last identifier in a code snippet
fn find_last_identifier(code: &str) -> Result<&str, CorrectionStrategyError> {
    let tokens: Vec<_> = code
        .split(|c: char| !c.is_alphanumeric() && c != '_')
        .filter(|s| !s.is_empty())
        .collect();

    tokens
        .last()
        .copied()
        .ok_or_else(|| CorrectionStrategyError::AstAnalysisFailed {
            context: "No identifier found in code snippet".to_string(),
        })
}

/// Find similar methods for type
fn find_similar_methods_for_type(
    method: &str,
    type_name: &str,
    _context: &ASTContext,
) -> Result<Vec<String>, CorrectionStrategyError> {
    let mut similar_methods = Vec::new();

    // Common method names based on type
    let common_methods = match type_name {
        t if t.contains("String") => vec![
            "push", "push_str", "pop", "len", "is_empty", "trim", "split",
        ],
        t if t.contains("Vec") => vec!["push", "pop", "len", "is_empty", "get", "insert", "remove"],
        t if t.contains("HashMap") => {
            vec!["get", "insert", "remove", "len", "is_empty", "contains_key"]
        }
        _ => vec!["clone", "to_string", "into", "as_ref", "as_mut"],
    };

    for common in &common_methods {
        if common != &method && levenshtein_distance(method, common) <= 2 {
            similar_methods.push((*common).to_string());
        }
    }

    Ok(similar_methods)
}

/// Find similar names in scope
fn find_similar_names_in_scope(
    name: &str,
    _context: &ASTContext,
) -> Result<Vec<String>, CorrectionStrategyError> {
    // Simplified implementation - would need scope analysis
    let common_names = vec![
        "self", "Self", "std", "vec", "Vec", "String", "str", "Option", "Result", "Some", "None",
        "Ok", "Err", "println", "print", "format", "panic",
    ];

    let mut suggestions = Vec::new();
    for common_name in common_names {
        if levenshtein_distance(name, common_name) <= 2 {
            suggestions.push(common_name.to_string());
        }
    }

    Ok(suggestions)
}

/// Find traits providing method
fn find_traits_providing_method(
    method: &str,
    _type_name: &str,
) -> Result<Vec<String>, CorrectionStrategyError> {
    let mut traits = Vec::new();

    match method {
        "clone" => traits.push("std::clone::Clone".to_string()),
        "into" => traits.push("std::convert::Into".to_string()),
        "from" => traits.push("std::convert::From".to_string()),
        "try_into" => traits.push("std::convert::TryInto".to_string()),
        "iter" => traits.push("std::iter::IntoIterator".to_string()),
        "collect" => traits.push("std::iter::Iterator".to_string()),
        "fmt" => traits.push("std::fmt::Display".to_string()),
        "debug" => traits.push("std::fmt::Debug".to_string()),
        _ => {
            // Generic trait suggestions
            traits.push("std::marker::Send".to_string());
            traits.push("std::marker::Sync".to_string());
        }
    }

    Ok(traits)
}

/// Find types with method
fn find_types_with_method(method: &str) -> Result<Vec<String>, CorrectionStrategyError> {
    let mut types = Vec::new();

    match method {
        "push" | "pop" | "len" => {
            types.extend(vec!["Vec<T>".to_string(), "String".to_string()]);
        }
        "insert" | "remove" | "get" => {
            types.extend(vec![
                "HashMap<K,V>".to_string(),
                "BTreeMap<K,V>".to_string(),
            ]);
        }
        "split" | "trim" => {
            types.push("String".to_string());
            types.push("&str".to_string());
        }
        _ => {
            types.push("String".to_string());
            types.push("Vec<T>".to_string());
        }
    }

    Ok(types)
}

/// Generates a skeleton trait implementation based on type and trait names
fn generate_trait_implementation(
    type_name: &str,
    trait_name: &str,
) -> Result<String, CorrectionStrategyError> {
    if type_name.is_empty() || trait_name.is_empty() {
        return Err(CorrectionStrategyError::AstAnalysisFailed {
            context: "Cannot generate trait implementation with empty type or trait name"
                .to_string(),
        });
    }

    Ok(format!(
        "// Implement {trait_name} for {type_name}\nimpl {trait_name} for {type_name} {{\n    // TODO: Add required methods\n}}\n"
    ))
}

/// Get alternative types that commonly implement the given trait
fn get_alternative_types_for_trait(
    type_name: &str,
    trait_name: &str,
) -> Result<Vec<String>, CorrectionStrategyError> {
    // Validate inputs
    if type_name.is_empty() || trait_name.is_empty() {
        return Err(CorrectionStrategyError::AstAnalysisFailed {
            context: "Cannot suggest alternatives with empty type or trait name".to_string(),
        });
    }

    let mut alternatives = Vec::new();

    // Common replacements for various traits
    match trait_name {
        "Display" | "Debug" => {
            if type_name != "String" && !type_name.ends_with("ToString") {
                alternatives.push("String".to_string());
                alternatives.push("&str".to_string());
            }
        }
        "Copy" | "Clone" => {
            if !type_name.starts_with('&') {
                alternatives.push(format!("{type_name}.clone()"));
            }
        }
        "Default" => {
            if !type_name.contains("Vec") && !type_name.contains("HashMap") {
                alternatives.push("Vec".to_string());
                alternatives.push("HashMap".to_string());
            }
        }
        "Iterator" => {
            if !type_name.contains("iter") {
                alternatives.push(format!("{type_name}.iter()"));
            }
        }
        "IntoIterator" => {
            alternatives.push(format!("{type_name}.into_iter()"));
        }
        _ => {}
    }

    Ok(alternatives)
}

/// Get common conversion methods between types
fn get_type_conversions(
    from_type: &str,
    to_type: &str,
) -> Result<Vec<(String, f64)>, CorrectionStrategyError> {
    // Validate inputs
    if from_type.is_empty() || to_type.is_empty() {
        return Err(CorrectionStrategyError::AstAnalysisFailed {
            context: "Cannot suggest conversions with empty type names".to_string(),
        });
    }

    let mut conversions = Vec::new();

    // String conversions
    if from_type == "&str" && to_type == "String" {
        conversions.push(("to_string()".to_string(), 0.95));
        conversions.push(("String::from".to_string(), 0.9));
    } else if from_type == "String" && to_type == "&str" {
        conversions.push(("as_str()".to_string(), 0.95));
    }

    // Numeric conversions
    if let (Ok(from_is_numeric), Ok(to_is_numeric)) =
        (is_numeric_type(from_type), is_numeric_type(to_type))
    {
        if from_is_numeric? && to_is_numeric? {
            conversions.push((format!("as {to_type}"), 0.9));
            conversions.push((format!("{from_type}.try_into()"), 0.85));
        }
    }

    // Boolean conversions
    if let Ok(from_is_numeric) = is_numeric_type(from_type) {
        if from_is_numeric? && to_type == "bool" {
            conversions.push(("!= 0".to_string(), 0.8));
        }
    }

    // Path conversions
    if (from_type.contains("Path") && to_type.contains("String"))
        || (from_type.contains("PathBuf") && to_type.contains("String"))
    {
        conversions.push(("to_string_lossy().to_string()".to_string(), 0.85));
    }

    // Collection conversions
    if from_type.contains("Vec<") && to_type.contains("&[") {
        conversions.push(("as_slice()".to_string(), 0.9));
    }

    // Generic conversion methods
    conversions.push(("into()".to_string(), 0.7));

    Ok(conversions)
}

/// Check if we're in a binding context
fn is_in_binding_context(_context: &ASTContext) -> bool {
    // Simplified implementation - would need AST analysis
    true
}

/// Helper function to check if a type is numeric
fn is_numeric_type(type_name: &str) -> Result<bool, CorrectionStrategyError> {
    if type_name.is_empty() {
        return Err(CorrectionStrategyError::AstAnalysisFailed {
            context: "Cannot check numeric type with empty type name".to_string(),
        });
    }

    let numeric_types = [
        "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
        "f32", "f64",
    ];

    Ok(numeric_types.contains(&type_name))
}

/// Check if static method is available
fn is_static_method_available(method: &str, type_name: &str, _context: &ASTContext) -> bool {
    // Common static methods
    match (type_name, method) {
        (t, "new") if t.contains("Vec") || t.contains("HashMap") || t.contains("String") => true,
        (t, "with_capacity") if t.contains("Vec") || t.contains("HashMap") => true,
        (t, "from") if t.contains("String") => true,
        _ => false,
    }
}

/// Checks if a trait is likely to be in scope based on context
fn is_trait_in_scope(
    trait_name: &str,
    context: &ASTContext,
) -> Result<bool, CorrectionStrategyError> {
    // Validate inputs
    if trait_name.is_empty() {
        return Err(CorrectionStrategyError::AstAnalysisFailed {
            context: "Cannot check scope with empty trait name".to_string(),
        });
    }

    // Check import statements in the surrounding context
    for import_info in &context.surrounding_context.imports {
        if import_info.path.ends_with(trait_name)
            || import_info.path.contains(&format!("{trait_name}::"))
        {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Check if identifier is valid
fn is_valid_identifier(name: &str) -> bool {
    !name.is_empty()
        && name.chars().next().unwrap().is_alphabetic()
        && name.chars().all(|c| c.is_alphanumeric() || c == '_')
}

/// Check if we're in a variable usage context
fn is_variable_usage_context(_context: &ASTContext) -> bool {
    // Simplified implementation - would need AST analysis
    true
}

/// Calculate Levenshtein distance between two strings
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = (matrix[i][j + 1] + 1)
                .min(matrix[i + 1][j] + 1)
                .min(matrix[i][j] + cost);
        }
    }

    matrix[len1][len2]
}

/// Move value to outer scope
fn move_value_to_outer_scope(
    code: &str,
    lifetime_info: &LifetimeInfo,
) -> Result<String, CorrectionStrategyError> {
    let moved_code = format!(
        "// Move {} to outer scope\nlet {} = /* initialize {} here */;\n{}",
        lifetime_info.variable_name, lifetime_info.variable_name, lifetime_info.variable_name, code
    );
    Ok(moved_code)
}

/// Remove duplicate field binding from pattern
fn remove_duplicate_field_binding(
    code: &str,
    field_name: &str,
) -> Result<String, CorrectionStrategyError> {
    let pattern = format!(r"\b{}\s*:\s*[^,}}]+", regex::escape(field_name));
    let regex =
        regex::Regex::new(&pattern).map_err(|e| CorrectionStrategyError::RegexCompilation {
            pattern: format!("Failed to create regex pattern: {e}"),
        })?;

    let mut matches: Vec<_> = regex.find_iter(code).collect();
    if matches.len() > 1 {
        // Remove the second occurrence
        matches.reverse();
        let second_match = matches[1];
        let mut result = code.to_string();
        result.replace_range(second_match.range(), "");
        // Clean up any double commas
        result = result
            .replace(",,", ",")
            .replace(",}", "}")
            .replace(",)", ")");
        Ok(result.trim().to_string())
    } else {
        Ok(code.to_string())
    }
}

/// Remove nonexistent field from pattern
fn remove_nonexistent_field(
    code: &str,
    field_name: &str,
) -> Result<String, CorrectionStrategyError> {
    let pattern = format!(r"\b{}\s*:\s*[^,}}]+,?\s*", regex::escape(field_name));
    let regex =
        regex::Regex::new(&pattern).map_err(|e| CorrectionStrategyError::RegexCompilation {
            pattern: format!("Failed to create regex pattern: {e}"),
        })?;

    let result = regex.replace_all(code, "").to_string();
    // Clean up any double commas or trailing commas
    let cleaned = result
        .replace(",,", ",")
        .replace(",}", "}")
        .replace(",)", ")")
        .replace("{ ,", "{ ")
        .replace("( ,", "( ");

    Ok(cleaned.trim().to_string())
}

/// Rename duplicate field binding to avoid conflict
fn rename_duplicate_field_binding(
    code: &str,
    field_name: &str,
) -> Result<String, CorrectionStrategyError> {
    let pattern = format!(r"\b{}\s*:\s*([^,}}]+)", regex::escape(field_name));
    let regex =
        regex::Regex::new(&pattern).map_err(|e| CorrectionStrategyError::RegexCompilation {
            pattern: format!("Failed to create regex pattern: {e}"),
        })?;

    let mut result = code.to_string();
    let mut replacement_count = 0;

    result = regex
        .replace_all(&result, |caps: &regex::Captures| {
            replacement_count += 1;
            if replacement_count == 2 {
                format!("{}_alt: {}", field_name, &caps[1])
            } else {
                caps[0].to_string()
            }
        })
        .to_string();

    Ok(result)
}

/// Replace field name in pattern
fn replace_field_name(
    code: &str,
    old_name: &str,
    new_name: &str,
) -> Result<String, CorrectionStrategyError> {
    let pattern = format!(r"\b{}\s*:", regex::escape(old_name));
    let regex =
        regex::Regex::new(&pattern).map_err(|e| CorrectionStrategyError::RegexCompilation {
            pattern: format!("Failed to create regex pattern: {e}"),
        })?;

    let result = regex.replace_all(code, &format!("{new_name}:")).to_string();
    Ok(result)
}

/// Replace identifier in code
fn replace_identifier(
    code: &str,
    old_name: &str,
    new_name: &str,
) -> Result<String, CorrectionStrategyError> {
    let pattern = format!(r"\b{}\b", regex::escape(old_name));
    let regex =
        regex::Regex::new(&pattern).map_err(|e| CorrectionStrategyError::RegexCompilation {
            pattern: format!("Failed to create regex pattern: {e}"),
        })?;

    let result = regex.replace_all(code, new_name).to_string();
    Ok(result)
}

/// Replace method name in code
fn replace_method_name(
    code: &str,
    old_method: &str,
    new_method: &str,
) -> Result<String, CorrectionStrategyError> {
    let pattern = format!(r"\.{}\b", regex::escape(old_method));
    let regex =
        regex::Regex::new(&pattern).map_err(|e| CorrectionStrategyError::RegexCompilation {
            pattern: format!("Failed to create regex pattern: {e}"),
        })?;

    let result = regex
        .replace_all(code, &format!(".{new_method}"))
        .to_string();
    Ok(result)
}

/// Replace path in code
fn replace_path_in_code(
    code: &str,
    old_path: &str,
    new_path: &str,
) -> Result<String, CorrectionStrategyError> {
    let pattern = format!(r"\b{}\b", regex::escape(old_path));
    let regex =
        regex::Regex::new(&pattern).map_err(|e| CorrectionStrategyError::RegexCompilation {
            pattern: format!("Failed to create regex pattern: {e}"),
        })?;

    let result = regex.replace_all(code, new_path).to_string();
    Ok(result)
}

/// Replace with qualified syntax
fn replace_with_qualified_syntax(
    code: &str,
    item: &str,
    qualified_path: &str,
) -> Result<String, CorrectionStrategyError> {
    replace_identifier(code, item, qualified_path)
}

/// Restructure loop borrowing
fn restructure_loop_borrowing(
    code: &str,
    borrow_info: &MutableBorrowInfo,
) -> Result<String, CorrectionStrategyError> {
    if code.contains("for ") {
        let restructured = format!(
            "let mut index = 0;\nwhile index < {}.len() {{\n    if let Some(item) = {}.get_mut(index) {{\n        // Process item\n    }}\n    index += 1;\n}}",
            borrow_info.variable_name,
            borrow_info.variable_name
        );
        return Ok(restructured);
    }

    Ok(code.to_string())
}

/// Suggest common crate imports for failed resolution
fn suggest_crate_imports(
    failed_path: &str,
) -> Result<Vec<(String, String)>, CorrectionStrategyError> {
    let mut suggestions = Vec::new();

    match failed_path {
        "serde" => {
            suggestions.push((
                "serde".to_string(),
                "serde::{Serialize, Deserialize}".to_string(),
            ));
            suggestions.push(("serde_json".to_string(), "serde_json".to_string()));
        }
        "tokio" => {
            suggestions.push(("tokio".to_string(), "tokio".to_string()));
            suggestions.push(("tokio".to_string(), "tokio::main".to_string()));
        }
        "regex" => {
            suggestions.push(("regex".to_string(), "regex::Regex".to_string()));
        }
        "chrono" => {
            suggestions.push(("chrono".to_string(), "chrono::{DateTime, Utc}".to_string()));
        }
        _ => {
            suggestions.push((failed_path.to_string(), format!("{failed_path}::*")));
        }
    }

    Ok(suggestions)
}

/// Suggest import aliases
fn suggest_import_aliases(
    item: &str,
    _context: &ASTContext,
) -> Result<Vec<(String, String)>, CorrectionStrategyError> {
    let mut aliases = Vec::new();

    // Common alias patterns
    aliases.push((format!("{item}_local"), format!("crate::{item}")));
    aliases.push((format!("{item}_std"), format!("std::{item}")));
    aliases.push((format!("Local{item}"), format!("crate::{item}")));

    Ok(aliases)
}

/// Suggest imports for identifier
fn suggest_imports_for_identifier(name: &str) -> Result<Vec<String>, CorrectionStrategyError> {
    let import_map = [
        ("Vec", "std::vec::Vec"),
        ("HashMap", "std::collections::HashMap"),
        ("HashSet", "std::collections::HashSet"),
        ("BTreeMap", "std::collections::BTreeMap"),
        ("BTreeSet", "std::collections::BTreeSet"),
        ("Regex", "regex::Regex"),
        ("Serialize", "serde::Serialize"),
        ("Deserialize", "serde::Deserialize"),
        ("tokio", "tokio"),
        ("async_trait", "async_trait::async_trait"),
    ];

    let suggestions: Vec<String> = import_map
        .iter()
        .filter(|(identifier, _)| identifier.to_lowercase().contains(&name.to_lowercase()))
        .map(|(_, import_path)| (*import_path).to_string())
        .collect();

    Ok(suggestions)
}

/// Suggest path corrections for typos
fn suggest_path_corrections(
    failed_path: &str,
    _resolution_type: &str,
) -> Result<Vec<String>, CorrectionStrategyError> {
    let common_paths = vec![
        "std",
        "core",
        "alloc",
        "std::collections",
        "std::io",
        "std::fs",
        "std::thread",
        "std::sync",
        "std::time",
        "std::env",
        "std::path",
    ];

    let mut corrections = Vec::new();
    for common_path in common_paths {
        if levenshtein_distance(failed_path, common_path) <= 2 && failed_path != common_path {
            corrections.push(common_path.to_string());
        }
    }

    Ok(corrections)
}

/// Suggest similar field names for typo correction
fn suggest_similar_field_names(
    field_name: &str,
    _struct_name: &str,
    _context: &ASTContext,
) -> Result<Vec<String>, CorrectionStrategyError> {
    // This is a simplified implementation
    // In practice, this would analyze the struct definition to find actual field names
    let common_fields = vec![
        "id", "name", "value", "data", "content", "text", "message", "error", "status", "code",
        "type", "kind", "size", "length", "count", "index",
    ];

    let mut suggestions = Vec::new();
    for common_field in common_fields {
        if levenshtein_distance(field_name, common_field) <= 2 {
            suggestions.push(common_field.to_string());
        }
    }

    Ok(suggestions)
}

/// Suggest standard library alternatives
fn suggest_std_alternatives(failed_path: &str) -> Result<Vec<String>, CorrectionStrategyError> {
    let mut alternatives = Vec::new();

    match failed_path {
        "HashMap" | "BTreeMap" => {
            alternatives.push("std::collections::HashMap".to_string());
            alternatives.push("std::collections::BTreeMap".to_string());
        }
        "Vec" => alternatives.push("std::vec::Vec".to_string()),
        "String" => alternatives.push("std::string::String".to_string()),
        "Result" => alternatives.push("std::result::Result".to_string()),
        "Option" => alternatives.push("std::option::Option".to_string()),
        _ => {
            alternatives.push(format!("std::{failed_path}"));
        }
    }

    Ok(alternatives)
}

//--------------------------------------------------------------------------------------------------
// ADDITIONAL ERROR CORRECTION STRATEGY IMPLEMENTATIONS
//--------------------------------------------------------------------------------------------------

/// **Strategy for E0061: wrong number of arguments.**

#[derive(Debug)]
pub(super) struct E0061WrongNumberOfArguments;

impl CorrectionStrategy for E0061WrongNumberOfArguments {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0061
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;
        let msg = &context.diagnostic.message;

        if let Ok((expected_count, found_count, function_name)) = extract_argument_count_info(msg)?
        {
            // Strategy 1: Add missing arguments with placeholders
            if found_count < expected_count {
                let corrected_code =
                    add_missing_arguments(original_code, expected_count - found_count)?;

                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: corrected_code,
                    },
                    0.9,
                    SafetyLevel::RequiresReview,
                    Some(format!(
                        "Add {} missing argument(s)",
                        expected_count - found_count
                    )),
                ));
            }
            // Strategy 2: Remove extra arguments
            else if found_count > expected_count {
                let corrected_code =
                    remove_extra_arguments(original_code, found_count - expected_count)?;

                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: corrected_code,
                    },
                    0.85,
                    SafetyLevel::RequiresReview,
                    Some(format!(
                        "Remove {} extra argument(s)",
                        found_count - expected_count
                    )),
                ));
            }

            // Strategy 3: Suggest method variants if available
            if let Ok(variants) = suggest_function_variants(&function_name, found_count)? {
                for variant in variants.into_iter().take(2) {
                    let variant_code =
                        replace_function_name(original_code, &function_name, &variant)?;

                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: variant_code,
                        },
                        0.8,
                        SafetyLevel::RequiresReview,
                        Some(format!(
                            "Use {variant} variant that accepts {found_count} arguments"
                        )),
                    ));
                }
            }
        }

        Ok(proposals)
    }
}

/// **Strategy for E0063: missing struct field.**

#[derive(Debug)]
pub(super) struct E0063MissingStructField;

impl CorrectionStrategy for E0063MissingStructField {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0063
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;
        let msg = &context.diagnostic.message;

        if let Ok((missing_field, struct_name)) = extract_missing_field_info(msg)? {
            // Strategy 1: Add missing field with default value
            if let Ok(with_default_field) = add_field_with_default(original_code, &missing_field)? {
                if with_default_field != *original_code {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: with_default_field,
                        },
                        0.9,
                        SafetyLevel::RequiresReview,
                        Some(format!(
                            "Add missing field '{missing_field}' with default value"
                        )),
                    ));
                }
            }

            // Strategy 2: Use struct update syntax if available
            if !original_code.contains("..") {
                if let Ok(with_update_syntax) =
                    add_struct_update_syntax(original_code, &struct_name)?
                {
                    if with_update_syntax != *original_code {
                        proposals.push(CorrectionProposal::new(
                            ProposalStrategy::ReplaceText {
                                original: original_code.clone(),
                                replacement: with_update_syntax,
                            },
                            0.85,
                            SafetyLevel::RequiresReview,
                            Some(
                                "Use struct update syntax (..) to fill remaining fields"
                                    .to_string(),
                            ),
                        ));
                    }
                }
            }

            // Strategy 3: Suggest implementing Default for the struct
            proposals.push(CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: format!(
                        "Implement Default for {struct_name} and use {struct_name}::default()"
                    ),
                },
                0.8,
                SafetyLevel::RequiresReview,
                Some(format!("Consider implementing Default for {struct_name}")),
            ));
        }

        Ok(proposals)
    }
}

/// **Strategy for E0106: missing lifetime parameter.**

#[derive(Debug)]
pub(super) struct E0106MissingLifetimeParameter;

impl CorrectionStrategy for E0106MissingLifetimeParameter {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0106
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;

        // Strategy 1: Add explicit lifetime parameter
        if let Ok(with_lifetime) = add_explicit_lifetime_parameter(original_code)? {
            if with_lifetime != *original_code {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: with_lifetime,
                    },
                    0.9,
                    SafetyLevel::RequiresReview,
                    Some("Add explicit lifetime parameter".to_string()),
                ));
            }
        }

        // Strategy 2: Use 'static lifetime if appropriate
        if is_static_lifetime_appropriate(original_code) {
            let with_static = add_static_lifetime(original_code)?;

            proposals.push(CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: with_static,
                },
                0.8,
                SafetyLevel::RequiresReview,
                Some("Use 'static lifetime".to_string()),
            ));
        }

        // Strategy 3: Remove references to avoid lifetime issues
        let without_references = convert_to_owned_types(original_code)?;

        proposals.push(CorrectionProposal::new(
            ProposalStrategy::ReplaceText {
                original: original_code.clone(),
                replacement: without_references,
            },
            0.75,
            SafetyLevel::RequiresReview,
            Some("Convert to owned types to avoid lifetime issues".to_string()),
        ));

        Ok(proposals)
    }
}

//--------------------------------------------------------------------------------------------------
// HELPER FUNCTIONS FOR ADDITIONAL ERROR CORRECTION STRATEGIES
//--------------------------------------------------------------------------------------------------

/// Extract argument count information from E0061 error message
fn extract_argument_count_info(
    message: &str,
) -> Result<(usize, usize, String), CorrectionStrategyError> {
    static PATTERN: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"this function takes (\d+) argument(?:s)? but (\d+) argument(?:s)? (?:was|were) supplied").expect("Valid regex")
    });

    if let Some(captures) = PATTERN.captures(message) {
        let expected = captures[1].parse::<usize>().unwrap_or(0);
        let found = captures[2].parse::<usize>().unwrap_or(0);

        // Try to extract function name
        let function_name = extract_function_name_from_message(message)
            .unwrap_or_else(|_| "unknown_function".to_string());

        return Ok((expected, found, function_name));
    }

    Err(CorrectionStrategyError::ParseFailed {
        context: "Could not extract argument count info from error message".to_string(),
    })
}

/// Extract function name from error message
fn extract_function_name_from_message(message: &str) -> Result<String, CorrectionStrategyError> {
    static PATTERN: std::sync::LazyLock<regex::Regex> =
        std::sync::LazyLock::new(|| regex::Regex::new(r"function `([^`]+)`").expect("Valid regex"));

    if let Some(captures) = PATTERN.captures(message) {
        return Ok(captures[1].to_string());
    }

    Ok("unknown_function".to_string())
}

/// Add missing arguments with placeholders
fn add_missing_arguments(
    code: &str,
    missing_count: usize,
) -> Result<String, CorrectionStrategyError> {
    if let Some(close_paren) = code.rfind(')') {
        let before_paren = &code[..close_paren];
        let after_paren = &code[close_paren..];

        let placeholders: Vec<String> = (0..missing_count)
            .map(|i| format!("/* arg_{} */", i + 1))
            .collect();

        let separator = if before_paren.ends_with('(') {
            ""
        } else {
            ", "
        };
        let new_args = placeholders.join(", ");

        return Ok(format!("{before_paren}{separator}{new_args}{after_paren}"));
    }

    Ok(code.to_string())
}

/// Remove extra arguments
fn remove_extra_arguments(
    code: &str,
    extra_count: usize,
) -> Result<String, CorrectionStrategyError> {
    if let Some(open_paren) = code.find('(') {
        if let Some(close_paren) = code.rfind(')') {
            let before = &code[..=open_paren];
            let after = &code[close_paren..];
            let args_str = &code[open_paren + 1..close_paren];

            let args: Vec<&str> = args_str
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .collect();

            if args.len() > extra_count {
                let kept_args = args[..args.len() - extra_count].join(", ");
                return Ok(format!("{before}{kept_args}{after}"));
            }
        }
    }

    Ok(code.to_string())
}

/// Suggest function variants
fn suggest_function_variants(
    function_name: &str,
    arg_count: usize,
) -> Result<Vec<String>, CorrectionStrategyError> {
    let mut variants = Vec::new();

    // Common function variant patterns
    match function_name {
        "println" => {
            if arg_count == 0 {
                variants.push("println!".to_string());
            } else {
                variants.push("print!".to_string());
            }
        }
        "format" => {
            variants.push("format!".to_string());
            if arg_count == 1 {
                variants.push("to_string".to_string());
            }
        }
        "Vec::new" => {
            if arg_count > 0 {
                variants.push("vec!".to_string());
                variants.push("Vec::with_capacity".to_string());
            }
        }
        _ => {
            // Generic suggestions
            variants.push(format!("{function_name}_with"));
            variants.push(format!("try_{function_name}"));
        }
    }

    Ok(variants)
}

/// Replace function name
fn replace_function_name(
    code: &str,
    old_name: &str,
    new_name: &str,
) -> Result<String, CorrectionStrategyError> {
    let pattern = format!(r"\b{}\b", regex::escape(old_name));
    let regex = regex::Regex::new(&pattern)
        .map_err(|_| CorrectionStrategyError::RegexCompilation { pattern })?;

    let result = regex.replace_all(code, new_name).to_string();
    Ok(result)
}

/// Extract missing field info from E0063 error message
fn extract_missing_field_info(message: &str) -> Result<(String, String), CorrectionStrategyError> {
    static PATTERN: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"missing field `([^`]+)`").expect("Valid regex")
    });

    if let Some(captures) = PATTERN.captures(message) {
        let field_name = captures[1].to_string();
        let struct_name = extract_struct_name_from_message(message)?;
        return Ok((field_name, struct_name));
    }

    Err(CorrectionStrategyError::ParseFailed {
        context: "Could not extract missing field info from error message".to_string(),
    })
}

/// Extract struct name from error message
fn extract_struct_name_from_message(message: &str) -> Result<String, CorrectionStrategyError> {
    static PATTERN: std::sync::LazyLock<regex::Regex> =
        std::sync::LazyLock::new(|| regex::Regex::new(r"struct `([^`]+)`").expect("Valid regex"));

    if let Some(captures) = PATTERN.captures(message) {
        return Ok(captures[1].to_string());
    }

    Ok("UnknownStruct".to_string())
}

/// Add field with default value
fn add_field_with_default(code: &str, field_name: &str) -> Result<String, CorrectionStrategyError> {
    if let Some(close_brace) = code.rfind('}') {
        let before_brace = &code[..close_brace].trim_end();
        let after_brace = &code[close_brace..];

        let separator = if before_brace.ends_with('{') {
            ""
        } else {
            ", "
        };
        let default_field = format!("{field_name}: Default::default()");

        return Ok(format!(
            "{before_brace}{separator}{default_field}{after_brace}"
        ));
    }

    Ok(code.to_string())
}

/// Add struct update syntax
fn add_struct_update_syntax(
    code: &str,
    struct_name: &str,
) -> Result<String, CorrectionStrategyError> {
    if let Some(close_brace) = code.rfind('}') {
        let before_brace = &code[..close_brace].trim_end();
        let after_brace = &code[close_brace..];

        let separator = if before_brace.ends_with('{') {
            ""
        } else {
            ", "
        };
        let update_syntax = format!("..{struct_name}::default()");

        return Ok(format!(
            "{before_brace}{separator}{update_syntax}{after_brace}"
        ));
    }

    Ok(code.to_string())
}

/// Add explicit lifetime parameter
fn add_explicit_lifetime_parameter(code: &str) -> Result<String, CorrectionStrategyError> {
    // Look for function signatures or struct definitions
    if code.contains("fn ") {
        let with_lifetime = code
            .replace("fn ", "fn ")
            .replace('(', "<'a>(")
            .replace('&', "&'a ");
        return Ok(with_lifetime);
    }

    if code.contains("struct ") {
        let with_lifetime = code
            .replace("struct ", "struct ")
            .replace('{', "<'a> {")
            .replace('&', "&'a ");
        return Ok(with_lifetime);
    }

    // Generic case: add lifetime to references
    Ok(code.replace('&', "&'a "))
}

/// Check if static lifetime is appropriate
fn is_static_lifetime_appropriate(code: &str) -> bool {
    // Static lifetime is appropriate for string literals and global constants
    code.contains('"') || code.contains("const ") || code.contains("static ")
}

/// Add static lifetime
fn add_static_lifetime(code: &str) -> Result<String, CorrectionStrategyError> {
    Ok(code
        .replace("&str", "&'static str")
        .replace("&[", "&'static ["))
}

/// Convert to owned types
fn convert_to_owned_types(code: &str) -> Result<String, CorrectionStrategyError> {
    let owned_code = code
        .replace("&str", "String")
        .replace("&[", "Vec<")
        .replace("&mut ", "")
        .replace('&', "");

    Ok(owned_code)
}

//--------------------------------------------------------------------------------------------------
// ADVANCED ERROR CORRECTION STRATEGIES - NEXT GENERATION
//--------------------------------------------------------------------------------------------------

/// **Strategy for E0308 Enhanced: Advanced Type Mismatch Resolution.**

#[derive(Debug)]
pub(super) struct E0308AdvancedTypeMismatch;

impl CorrectionStrategy for E0308AdvancedTypeMismatch {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0308
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;
        let msg = &context.diagnostic.message;

        if let Ok((expected_type, found_type)) = extract_advanced_type_mismatch(msg)? {
            // Strategy 1: Smart generic type inference
            if let Ok(inferred_generics) =
                infer_generic_types(&expected_type, &found_type, context)?
            {
                for generic_solution in inferred_generics.into_iter().take(2) {
                    if let Ok(generic_code) =
                        apply_generic_solution(original_code, &generic_solution)?
                    {
                        if generic_code != *original_code {
                            proposals.push(CorrectionProposal::new(
                                ProposalStrategy::ReplaceText {
                                    original: original_code.clone(),
                                    replacement: generic_code,
                                },
                                0.95,
                                SafetyLevel::RequiresReview,
                                Some(format!(
                                    "Apply generic type solution: {}",
                                    generic_solution.application
                                )),
                            ));
                        }
                    }
                }
            }

            // Strategy 2: Trait-based conversion suggestions
            if let Ok(trait_conversions) =
                suggest_trait_based_conversions(&expected_type, &found_type)?
            {
                for conversion in trait_conversions.into_iter().take(3) {
                    if let Ok(trait_code) = apply_trait_conversion(original_code, &conversion)? {
                        if trait_code != *original_code {
                            proposals.push(CorrectionProposal::new(
                                ProposalStrategy::ReplaceText {
                                    original: original_code.clone(),
                                    replacement: trait_code,
                                },
                                0.9,
                                SafetyLevel::RequiresReview,
                                Some(format!("Use trait conversion: {}", conversion.trait_name)),
                            ));
                        }
                    }
                }
            }

            // Strategy 3: Smart wrapper/unwrapper detection
            if let Ok(wrapper_solution) =
                detect_wrapper_unwrapper_pattern(&expected_type, &found_type)?
            {
                if let Ok(wrapper_code) = apply_wrapper_solution(original_code, &wrapper_solution)?
                {
                    if wrapper_code != *original_code {
                        proposals.push(CorrectionProposal::new(
                            ProposalStrategy::ReplaceText {
                                original: original_code.clone(),
                                replacement: wrapper_code,
                            },
                            0.92,
                            SafetyLevel::RequiresReview,
                            Some(format!(
                                "Apply wrapper pattern: {}",
                                wrapper_solution.operation
                            )),
                        ));
                    }
                }
            }

            // Strategy 4: Context-aware type coercion
            if let Ok(coercion_chain) = build_coercion_chain(&expected_type, &found_type, context)?
            {
                if let Ok(coerced_code) = apply_coercion_chain(original_code, &coercion_chain)? {
                    if coerced_code != *original_code {
                        proposals.push(CorrectionProposal::new(
                            ProposalStrategy::ReplaceText {
                                original: original_code.clone(),
                                replacement: coerced_code,
                            },
                            0.88,
                            SafetyLevel::RequiresReview,
                            Some("Apply intelligent type coercion chain".to_string()),
                        ));
                    }
                }
            }
        }

        Ok(proposals)
    }
}

//--------------------------------------------------------------------------------------------------
// ADVANCED HELPER FUNCTION STRUCTURES
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct GenericSolution {
    type_parameters: Vec<String>,
    constraints: Vec<String>,
    application: String,
}

#[derive(Debug, Clone)]
struct TraitConversion {
    trait_name: String,
    method_name: String,
    confidence: f64,
}

#[derive(Debug, Clone)]
struct WrapperSolution {
    operation: String,
    wrapper_type: String,
    confidence: f64,
}

#[derive(Debug, Clone)]
struct CoercionChain {
    steps: Vec<CoercionStep>,
    final_confidence: f64,
}

#[derive(Debug, Clone)]
struct CoercionStep {
    from_type: String,
    to_type: String,
    method: String,
}

//--------------------------------------------------------------------------------------------------
// ADVANCED HELPER FUNCTION IMPLEMENTATIONS
//--------------------------------------------------------------------------------------------------

/// Extract advanced type mismatch information with context analysis
fn extract_advanced_type_mismatch(
    message: &str,
) -> Result<(String, String), CorrectionStrategyError> {
    static PATTERN: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"expected[^`]*`([^`]+)`[^`]*found[^`]*`([^`]+)`").expect("Valid regex")
    });

    if let Some(captures) = PATTERN.captures(message) {
        let expected_type = normalize_type_name(&captures[1]);
        let found_type = normalize_type_name(&captures[2]);
        return Ok((expected_type, found_type));
    }

    Err(CorrectionStrategyError::ParseFailed {
        context: "Could not extract advanced type mismatch information".to_string(),
    })
}

/// Normalize type names for better matching
fn normalize_type_name(type_name: &str) -> String {
    type_name
        .trim()
        .replace(' ', "")
        .replace("&mut", "&mut ")
        .replace('&', "& ")
        .to_string()
}

/// Infer generic types based on context
fn infer_generic_types(
    expected: &str,
    found: &str,
    _context: &ASTContext,
) -> Result<Vec<GenericSolution>, CorrectionStrategyError> {
    let mut solutions = Vec::new();

    // Pattern 1: Collection type mismatches
    if expected.contains("Vec<") && found.contains("Iterator") {
        solutions.push(GenericSolution {
            type_parameters: vec!["T".to_string()],
            constraints: vec!["T: Clone".to_string()],
            application: ".collect::<Vec<_>>()".to_string(),
        });
    }

    // Pattern 2: Option/Result unwrapping
    if expected.contains("Option<") || expected.contains("Result<") {
        solutions.push(GenericSolution {
            type_parameters: vec!["T".to_string(), "E".to_string()],
            constraints: vec!["T: Default".to_string()],
            application: ".unwrap_or_default()".to_string(),
        });
    }

    // Pattern 3: Reference/ownership mismatches
    if expected.starts_with('&') && !found.starts_with('&') {
        solutions.push(GenericSolution {
            type_parameters: vec![],
            constraints: vec![],
            application: "&".to_string(),
        });
    }

    Ok(solutions)
}

/// Apply generic solution to code
fn apply_generic_solution(
    code: &str,
    solution: &GenericSolution,
) -> Result<String, CorrectionStrategyError> {
    let result = if solution.application.starts_with('.') {
        format!("{}{}", code.trim(), solution.application)
    } else if solution.application.starts_with('&') {
        format!("{}{}", solution.application, code.trim())
    } else {
        format!("{}({})", solution.application, code.trim())
    };

    Ok(result)
}

/// Suggest trait-based conversions
fn suggest_trait_based_conversions(
    expected: &str,
    found: &str,
) -> Result<Vec<TraitConversion>, CorrectionStrategyError> {
    let mut conversions = Vec::new();

    // String conversions
    if expected == "String" && found == "&str" {
        conversions.push(TraitConversion {
            trait_name: "ToString".to_string(),
            method_name: "to_string".to_string(),
            confidence: 0.95,
        });
        conversions.push(TraitConversion {
            trait_name: "From".to_string(),
            method_name: "String::from".to_string(),
            confidence: 0.9,
        });
    }

    // Into conversions
    if expected != found {
        conversions.push(TraitConversion {
            trait_name: "Into".to_string(),
            method_name: "into".to_string(),
            confidence: 0.8,
        });
    }

    // TryInto for fallible conversions
    if is_numeric_type_name(expected)? && is_numeric_type_name(found)? {
        conversions.push(TraitConversion {
            trait_name: "TryInto".to_string(),
            method_name: "try_into".to_string(),
            confidence: 0.85,
        });
    }

    Ok(conversions)
}

/// Check if type name represents a numeric type
fn is_numeric_type_name(type_name: &str) -> Result<bool, CorrectionStrategyError> {
    Ok(matches!(
        type_name,
        "i8" | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "isize"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "usize"
            | "f32"
            | "f64"
    ))
}

/// Apply trait conversion to code
fn apply_trait_conversion(
    code: &str,
    conversion: &TraitConversion,
) -> Result<String, CorrectionStrategyError> {
    let result = if conversion.method_name.contains("::") {
        format!("{}({})", conversion.method_name, code.trim())
    } else {
        format!("{}.{}()", code.trim(), conversion.method_name)
    };

    Ok(result)
}

/// Detect wrapper/unwrapper patterns
fn detect_wrapper_unwrapper_pattern(
    expected: &str,
    found: &str,
) -> Result<WrapperSolution, CorrectionStrategyError> {
    // Option unwrapping
    if found.contains("Option<") && !expected.contains("Option<") {
        return Ok(WrapperSolution {
            operation: "unwrap_option".to_string(),
            wrapper_type: "Option".to_string(),
            confidence: 0.9,
        });
    }

    // Result unwrapping
    if found.contains("Result<") && !expected.contains("Result<") {
        return Ok(WrapperSolution {
            operation: "unwrap_result".to_string(),
            wrapper_type: "Result".to_string(),
            confidence: 0.85,
        });
    }

    // Box unwrapping
    if found.contains("Box<") && !expected.contains("Box<") {
        return Ok(WrapperSolution {
            operation: "unbox".to_string(),
            wrapper_type: "Box".to_string(),
            confidence: 0.8,
        });
    }

    Err(CorrectionStrategyError::AstAnalysisFailed {
        context: "No wrapper/unwrapper pattern detected".to_string(),
    })
}

/// Apply wrapper solution to code
fn apply_wrapper_solution(
    code: &str,
    solution: &WrapperSolution,
) -> Result<String, CorrectionStrategyError> {
    let result = match solution.operation.as_str() {
        "unwrap_option" => format!("{}.unwrap_or_default()", code.trim()),
        "unwrap_result" => format!("{}.unwrap_or_else(|_| Default::default())", code.trim()),
        "unbox" => format!("*{}", code.trim()),
        _ => format!("{}.{}()", code.trim(), solution.operation),
    };

    Ok(result)
}

/// Build intelligent coercion chain
fn build_coercion_chain(
    expected: &str,
    found: &str,
    _context: &ASTContext,
) -> Result<CoercionChain, CorrectionStrategyError> {
    let mut steps = Vec::new();
    let mut current_type = found.to_string();

    // Step 1: Handle reference/dereference
    if expected.starts_with('&') && !current_type.starts_with('&') {
        steps.push(CoercionStep {
            from_type: current_type.clone(),
            to_type: format!("&{current_type}"),
            method: "&".to_string(),
        });
        current_type = format!("&{current_type}");
    } else if !expected.starts_with('&') && current_type.starts_with('&') {
        let inner_type = current_type.trim_start_matches('&');
        steps.push(CoercionStep {
            from_type: current_type.clone(),
            to_type: inner_type.to_string(),
            method: ".clone()".to_string(),
        });
        current_type = inner_type.to_string();
    }

    // Step 2: Handle string conversions
    if expected == "String" && current_type == "&str" {
        steps.push(CoercionStep {
            from_type: current_type.clone(),
            to_type: "String".to_string(),
            method: ".to_string()".to_string(),
        });
        // current_type = "String".to_string();
    }

    // Step 3: Handle collection conversions
    if expected.contains("Vec<") && current_type.contains("Iterator") {
        steps.push(CoercionStep {
            from_type: current_type,
            to_type: expected.to_string(),
            method: ".collect()".to_string(),
        });
    }

    let final_confidence = if steps.is_empty() {
        0.5
    } else {
        0.9 - (steps.len() as f64 * 0.1)
    };

    Ok(CoercionChain {
        steps,
        final_confidence,
    })
}

/// Apply coercion chain to code
fn apply_coercion_chain(
    code: &str,
    chain: &CoercionChain,
) -> Result<String, CorrectionStrategyError> {
    let mut result = code.trim().to_string();

    for step in &chain.steps {
        if step.method.starts_with('&') {
            result = format!("{}{}", step.method, result);
        } else if step.method.starts_with('.') {
            result = format!("{}{}", result, step.method);
        } else {
            result = format!("{}({})", step.method, result);
        }
    }

    Ok(result)
}

/// **Strategy for E0282: type annotations needed.**

#[derive(Debug)]
pub(super) struct E0282TypeAnnotationsNeeded;

impl CorrectionStrategy for E0282TypeAnnotationsNeeded {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0282
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;

        // Strategy 1: Add explicit type annotation to variable
        if let Ok(with_type_annotation) = add_type_annotation_to_variable(original_code)? {
            if with_type_annotation != *original_code {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: with_type_annotation,
                    },
                    0.9,
                    SafetyLevel::RequiresReview,
                    Some("Add explicit type annotation".to_string()),
                ));
            }
        }

        // Strategy 2: Use turbofish syntax for method calls
        if original_code.contains("collect()") || original_code.contains("parse()") {
            let with_turbofish = add_turbofish_syntax(original_code)?;

            proposals.push(CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: with_turbofish,
                },
                0.85,
                SafetyLevel::RequiresReview,
                Some("Add turbofish syntax for type inference".to_string()),
            ));
        }

        // Strategy 3: Suggest common type annotations
        let common_types = suggest_common_type_annotations(original_code)?;

        for type_annotation in common_types.into_iter().take(3) {
            if let Ok(annotated_code) = apply_type_annotation(original_code, &type_annotation)? {
                if annotated_code != *original_code {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: annotated_code,
                        },
                        0.8,
                        SafetyLevel::RequiresReview,
                        Some(format!("Use type annotation: {type_annotation}")),
                    ));
                }
            }
        }

        Ok(proposals)
    }
}

/// **Strategy for E0369: binary operator not supported.**

#[derive(Debug)]
pub(super) struct E0369BinaryOperatorNotSupported;

impl CorrectionStrategy for E0369BinaryOperatorNotSupported {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0369
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;
        let msg = &context.diagnostic.message;

        if let Ok((operator, left_type, right_type)) = extract_binary_operator_info(msg)? {
            // Strategy 1: Suggest trait implementations
            if let Ok(trait_suggestions) =
                suggest_operator_traits(&operator, &left_type, &right_type)?
            {
                for trait_name in trait_suggestions.into_iter().take(2) {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::Generic {
                            description: format!("Implement {trait_name} trait for {left_type}"),
                        },
                        0.9,
                        SafetyLevel::RequiresReview,
                        Some(format!("Add {trait_name} implementation")),
                    ));
                }
            }

            // Strategy 2: Convert to method calls
            if let Ok(method_call) = convert_operator_to_method(&operator, original_code)? {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: method_call,
                    },
                    0.85,
                    SafetyLevel::RequiresReview,
                    Some(format!("Convert {operator} operator to method call")),
                ));
            }

            // Strategy 3: Type conversion suggestions
            if let Ok(conversions) =
                suggest_type_conversions_for_operator(&operator, &left_type, &right_type)?
            {
                for conversion in conversions.into_iter().take(2) {
                    let converted_code =
                        apply_type_conversion_for_operator(original_code, &conversion)?;

                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: converted_code,
                        },
                        0.8,
                        SafetyLevel::RequiresReview,
                        Some(format!("Convert types for {operator} operation")),
                    ));
                }
            }
        }

        Ok(proposals)
    }
}

/// **Strategy for E0381: use of possibly uninitialized variable.**

#[derive(Debug)]
pub(super) struct E0381UseOfPossiblyUninitializedVariable;

impl CorrectionStrategy for E0381UseOfPossiblyUninitializedVariable {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0381
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;
        let msg = &context.diagnostic.message;

        if let Ok(variable_name) = extract_uninitialized_variable_name(msg)? {
            // Strategy 1: Initialize variable with default value
            let with_initialization =
                initialize_variable_with_default(original_code, &variable_name)?;

            proposals.push(CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: with_initialization,
                },
                0.9,
                SafetyLevel::RequiresReview,
                Some(format!("Initialize '{variable_name}' with default value")),
            ));

            // Strategy 2: Use Option wrapper
            if let Ok(with_option) = wrap_variable_in_option(original_code, &variable_name)? {
                if with_option != *original_code {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: with_option,
                        },
                        0.85,
                        SafetyLevel::RequiresReview,
                        Some(format!("Use Option<T> for '{variable_name}'")),
                    ));
                }
            }

            // Strategy 3: Add conditional initialization
            if let Ok(with_conditional) =
                add_conditional_initialization(original_code, &variable_name)?
            {
                if with_conditional != *original_code {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: with_conditional,
                        },
                        0.8,
                        SafetyLevel::RequiresReview,
                        Some(format!(
                            "Add conditional initialization for '{variable_name}'"
                        )),
                    ));
                }
            }
        }

        Ok(proposals)
    }
}

/// **Strategy for E0382: use of moved value.**

#[derive(Debug)]
pub(super) struct E0382UseOfMovedValue;

impl CorrectionStrategy for E0382UseOfMovedValue {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0382
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;
        let msg = &context.diagnostic.message;

        let moved_variable = extract_moved_variable_name(msg)?;

        // Strategy 1: Clone the value before moving
        if let Ok(with_clone) = clone_before_move(original_code, &moved_variable)? {
            if with_clone != *original_code {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: with_clone,
                    },
                    0.9,
                    SafetyLevel::RequiresReview,
                    Some(format!("Clone '{moved_variable}' before moving")),
                ));
            }
        }

        // Strategy 2: Use references instead of moving
        if let Ok(with_references) = use_references_instead_of_move(original_code, &moved_variable)?
        {
            if with_references != *original_code {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: with_references,
                    },
                    0.85,
                    SafetyLevel::RequiresReview,
                    Some(format!("Use references for '{moved_variable}'")),
                ));
            }
        }

        // Strategy 3: Restructure to avoid multiple uses
        if let Ok(restructured) =
            restructure_to_avoid_multiple_uses(original_code, &moved_variable)?
        {
            if restructured != *original_code {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: restructured,
                    },
                    0.8,
                    SafetyLevel::RequiresReview,
                    Some(format!(
                        "Restructure to avoid multiple uses of '{moved_variable}'"
                    )),
                ));
            }
        }

        Ok(proposals)
    }
}

/// **Strategy for E0384: cannot assign to immutable variable.**
#[derive(Debug)]
pub(super) struct E0384CannotAssignToImmutableVariable;

impl CorrectionStrategy for E0384CannotAssignToImmutableVariable {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0384
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;
        let msg = &context.diagnostic.message;

        let immutable_var = extract_immutable_variable_name(msg)?;

        // Strategy 1: Make variable mutable
        if let Ok(with_mut) = make_variable_mutable(original_code, &immutable_var)? {
            if with_mut != *original_code {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: with_mut,
                    },
                    0.95,
                    SafetyLevel::RequiresReview,
                    Some(format!("Make '{immutable_var}' mutable")),
                ));
            }
        }

        // Strategy 2: Create new variable instead of reassigning
        let with_new_var = create_new_variable_instead_of_reassign(original_code, &immutable_var)?;

        proposals.push(CorrectionProposal::new(
            ProposalStrategy::ReplaceText {
                original: original_code.clone(),
                replacement: with_new_var,
            },
            0.8,
            SafetyLevel::RequiresReview,
            Some(format!(
                "Create new variable instead of reassigning '{immutable_var}'"
            )),
        ));

        // Strategy 3: Use interior mutability
        if let Ok(with_interior_mut) = use_interior_mutability(original_code, &immutable_var)? {
            if with_interior_mut != *original_code {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: with_interior_mut,
                    },
                    0.75,
                    SafetyLevel::RequiresReview,
                    Some(format!("Use interior mutability for '{immutable_var}'")),
                ));
            }
        }

        Ok(proposals)
    }
}

/// **Strategy for E0432: unresolved import.**

#[derive(Debug)]
pub(super) struct E0432UnresolvedImport;

impl CorrectionStrategy for E0432UnresolvedImport {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0432
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;
        let msg = &context.diagnostic.message;

        let unresolved_import = extract_unresolved_import_path(msg)?;

        // Strategy 1: Suggest similar import paths
        if let Ok(similar_imports) = suggest_similar_import_paths(&unresolved_import)? {
            for similar_import in similar_imports.into_iter().take(3) {
                let corrected_import =
                    fix_import_path(original_code, &unresolved_import, &similar_import)?;

                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: corrected_import,
                    },
                    0.9,
                    SafetyLevel::RequiresReview,
                    Some(format!(
                        "Replace '{unresolved_import}' with '{similar_import}'"
                    )),
                ));
            }
        }

        // Strategy 2: Suggest adding crate dependencies
        if let Ok(crate_suggestions) = suggest_crate_dependencies(&unresolved_import)? {
            for crate_name in crate_suggestions.into_iter().take(2) {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::Generic {
                        description: format!("Add '{crate_name}' to Cargo.toml dependencies"),
                    },
                    0.85,
                    SafetyLevel::RequiresReview,
                    Some(format!("Add {crate_name} dependency")),
                ));
            }
        }

        // Strategy 3: Remove unused import
        if let Ok(without_import) = remove_unused_import(original_code, &unresolved_import)? {
            if without_import != *original_code {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: without_import,
                    },
                    0.8,
                    SafetyLevel::RequiresReview,
                    Some(format!("Remove unused import '{unresolved_import}'")),
                ));
            }
        }

        Ok(proposals)
    }
}

/// **Strategy for E0502: cannot borrow as mutable/immutable.**

#[derive(Debug)]
pub(super) struct E0502CannotBorrowAsMutableImmutable;

impl CorrectionStrategy for E0502CannotBorrowAsMutableImmutable {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0502
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;

        // Strategy 1: Scope borrows to avoid conflicts
        if let Ok(scoped_borrows) = scope_borrows_to_avoid_conflicts(original_code)? {
            if scoped_borrows != *original_code {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: scoped_borrows,
                    },
                    0.9,
                    SafetyLevel::RequiresReview,
                    Some("Scope borrows to avoid conflicts".to_string()),
                ));
            }
        }

        // Strategy 2: Clone to avoid borrow conflicts
        if let Ok(with_clone) = clone_to_avoid_borrow_conflicts(original_code)? {
            if with_clone != *original_code {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: with_clone,
                    },
                    0.85,
                    SafetyLevel::RequiresReview,
                    Some("Clone values to avoid borrow conflicts".to_string()),
                ));
            }
        }

        // Strategy 3: Separate conflicting operations
        if let Ok(separated) = separate_conflicting_operations(original_code)? {
            if separated != *original_code {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: separated,
                    },
                    0.8,
                    SafetyLevel::RequiresReview,
                    Some("Separate conflicting borrow operations".to_string()),
                ));
            }
        }

        Ok(proposals)
    }
}

/// **Strategy for E0507: cannot move out of borrowed content.**

#[derive(Debug)]
pub(super) struct E0507CannotMoveOutOfBorrowedContent;

impl CorrectionStrategy for E0507CannotMoveOutOfBorrowedContent {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0507
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;

        // Strategy 1: Clone instead of move
        let with_clone = clone_instead_of_move(original_code)?;

        proposals.push(CorrectionProposal::new(
            ProposalStrategy::ReplaceText {
                original: original_code.clone(),
                replacement: with_clone,
            },
            0.9,
            SafetyLevel::RequiresReview,
            Some("Clone instead of moving from borrowed content".to_string()),
        ));

        // Strategy 2: Avoid moving from borrowed content
        let without_move = avoid_moving_from_borrowed(original_code)?;

        proposals.push(CorrectionProposal::new(
            ProposalStrategy::ReplaceText {
                original: original_code.clone(),
                replacement: without_move,
            },
            0.85,
            SafetyLevel::RequiresReview,
            Some("Use references instead of moving".to_string()),
        ));

        // Strategy 3: Take ownership instead of borrowing
        if let Ok(with_ownership) = take_ownership_instead_of_borrow(original_code)? {
            if with_ownership != *original_code {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: with_ownership,
                    },
                    0.8,
                    SafetyLevel::RequiresReview,
                    Some("Take ownership instead of borrowing".to_string()),
                ));
            }
        }

        Ok(proposals)
    }
}

/// **Strategy for E0515: cannot return reference to temporary.**

#[derive(Debug)]
pub(super) struct E0515CannotReturnReferenceToTemporary;

impl CorrectionStrategy for E0515CannotReturnReferenceToTemporary {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0515
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;

        // Strategy 1: Return owned value instead of reference
        if let Ok(owned_return) = return_owned_instead_of_reference(original_code)? {
            if owned_return != *original_code {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: owned_return,
                    },
                    0.95,
                    SafetyLevel::RequiresReview,
                    Some("Return owned value instead of reference".to_string()),
                ));
            }
        }

        // Strategy 2: Store in longer-lived location
        let longer_lived = store_in_longer_lived_location(original_code)?;

        proposals.push(CorrectionProposal::new(
            ProposalStrategy::ReplaceText {
                original: original_code.clone(),
                replacement: longer_lived,
            },
            0.85,
            SafetyLevel::RequiresReview,
            Some("Store value in longer-lived location".to_string()),
        ));

        // Strategy 3: Use static if appropriate
        let with_static = use_static_if_appropriate(original_code)?;

        proposals.push(CorrectionProposal::new(
            ProposalStrategy::ReplaceText {
                original: original_code.clone(),
                replacement: with_static,
            },
            0.8,
            SafetyLevel::RequiresReview,
            Some("Use static lifetime if appropriate".to_string()),
        ));

        Ok(proposals)
    }
}

/// **Strategy for E0621: explicit lifetime required.**

#[derive(Debug)]
pub(super) struct E0621ExplicitLifetimeRequired;

impl CorrectionStrategy for E0621ExplicitLifetimeRequired {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0621
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;

        // Strategy 1: Add explicit lifetimes
        let default_lifetime_info = LifetimeInfo {
            variable_name: "unknown".to_string(),
            can_use_static: false,
            is_string_literal: false,
        };
        if let Ok(with_lifetimes) = add_explicit_lifetimes(original_code, &default_lifetime_info)? {
            if with_lifetimes != *original_code {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: with_lifetimes,
                    },
                    0.9,
                    SafetyLevel::RequiresReview,
                    Some("Add explicit lifetime parameters".to_string()),
                ));
            }
        }

        // Strategy 2: Convert to owned to avoid lifetimes
        if let Ok(owned_version) = convert_to_owned_to_avoid_lifetimes(original_code)? {
            if owned_version != *original_code {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: owned_version,
                    },
                    0.85,
                    SafetyLevel::RequiresReview,
                    Some("Convert to owned types to avoid lifetime issues".to_string()),
                ));
            }
        }

        // Strategy 3: Simplify function signature
        let simplified = simplify_function_signature(original_code)?;

        proposals.push(CorrectionProposal::new(
            ProposalStrategy::ReplaceText {
                original: original_code.clone(),
                replacement: simplified,
            },
            0.8,
            SafetyLevel::RequiresReview,
            Some("Simplify function signature to avoid lifetime complexity".to_string()),
        ));

        Ok(proposals)
    }
}

/// **Strategy for E0716: temporary value dropped while borrowed.**

#[derive(Debug)]
pub(super) struct E0716TemporaryValueDroppedWhileBorrowed;

impl CorrectionStrategy for E0716TemporaryValueDroppedWhileBorrowed {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0716
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;

        // Strategy 1: Store temporary in variable
        let with_variable = store_temporary_in_variable(original_code)?;

        proposals.push(CorrectionProposal::new(
            ProposalStrategy::ReplaceText {
                original: original_code.clone(),
                replacement: with_variable,
            },
            0.95,
            SafetyLevel::RequiresReview,
            Some("Store temporary value in variable".to_string()),
        ));

        // Strategy 2: Clone temporary value
        let with_clone = clone_temporary_value(original_code)?;

        proposals.push(CorrectionProposal::new(
            ProposalStrategy::ReplaceText {
                original: original_code.clone(),
                replacement: with_clone,
            },
            0.85,
            SafetyLevel::RequiresReview,
            Some("Clone temporary value to extend lifetime".to_string()),
        ));

        // Strategy 3: Restructure to avoid temporary
        let restructured = restructure_to_avoid_temporary(original_code)?;

        proposals.push(CorrectionProposal::new(
            ProposalStrategy::ReplaceText {
                original: original_code.clone(),
                replacement: restructured,
            },
            0.8,
            SafetyLevel::RequiresReview,
            Some("Restructure code to avoid temporary value".to_string()),
        ));

        Ok(proposals)
    }
}

//--------------------------------------------------------------------------------------------------
// PLACEHOLDER HELPER FUNCTIONS FOR MISSING STRATEGIES
// TODO: Implement these functions properly based on TODO.md specifications
//--------------------------------------------------------------------------------------------------

// Helper functions for E0282 (Type annotations needed)
fn add_type_annotation_to_variable(code: &str) -> Result<String, CorrectionStrategyError> {
    Ok(format!("let variable: /* TODO: add type */ = {code};"))
}
fn add_turbofish_syntax(code: &str) -> Result<String, CorrectionStrategyError> {
    Ok(code
        .replace("collect()", "collect::<Vec<_>>()")
        .replace("parse()", "parse::<i32>()"))
}
fn suggest_common_type_annotations(_code: &str) -> Result<Vec<String>, CorrectionStrategyError> {
    Ok(vec![
        "i32".to_string(),
        "String".to_string(),
        "Vec<i32>".to_string(),
    ])
}
fn apply_type_annotation(
    code: &str,
    type_annotation: &str,
) -> Result<String, CorrectionStrategyError> {
    Ok(format!("{code}: {type_annotation}"))
}

// Helper functions for E0369 (Binary operator not supported)
fn extract_binary_operator_info(
    _message: &str,
) -> Result<(String, String, String), CorrectionStrategyError> {
    Ok(("+".to_string(), "Type1".to_string(), "Type2".to_string()))
}
fn suggest_operator_traits(
    _op: &str,
    _left: &str,
    _right: &str,
) -> Result<Vec<String>, CorrectionStrategyError> {
    Ok(vec!["Add".to_string(), "PartialEq".to_string()])
}
fn convert_operator_to_method(op: &str, code: &str) -> Result<String, CorrectionStrategyError> {
    Ok(code.replace(op, ".add"))
}
fn suggest_type_conversions_for_operator(
    _op: &str,
    _left: &str,
    _right: &str,
) -> Result<Vec<String>, CorrectionStrategyError> {
    Ok(vec!["to_string()".to_string()])
}
fn apply_type_conversion_for_operator(
    code: &str,
    conversion: &str,
) -> Result<String, CorrectionStrategyError> {
    Ok(format!("{code}.{conversion}"))
}

// Helper functions for E0381 (Use of possibly uninitialized variable)
fn extract_uninitialized_variable_name(_message: &str) -> Result<String, CorrectionStrategyError> {
    Ok("uninitialized_var".to_string())
}
fn initialize_variable_with_default(
    code: &str,
    var_name: &str,
) -> Result<String, CorrectionStrategyError> {
    Ok(format!("let {var_name} = Default::default();\n{code}"))
}
fn wrap_variable_in_option(code: &str, var_name: &str) -> Result<String, CorrectionStrategyError> {
    Ok(format!("let {var_name}: Option<_> = None;\n{code}"))
}
fn add_conditional_initialization(
    code: &str,
    var_name: &str,
) -> Result<String, CorrectionStrategyError> {
    Ok(format!(
        "let {var_name} = if condition {{ value }} else {{ default }};\n{code}"
    ))
}

// Helper functions for E0382 (Use of moved value)
fn extract_moved_variable_name(_message: &str) -> Result<String, CorrectionStrategyError> {
    Ok("moved_var".to_string())
}
fn clone_before_move(code: &str, var_name: &str) -> Result<String, CorrectionStrategyError> {
    Ok(code.replace(var_name, &format!("{var_name}.clone()")))
}
fn use_references_instead_of_move(
    code: &str,
    var_name: &str,
) -> Result<String, CorrectionStrategyError> {
    Ok(code.replace(var_name, &format!("&{var_name}")))
}
fn restructure_to_avoid_multiple_uses(
    code: &str,
    _var_name: &str,
) -> Result<String, CorrectionStrategyError> {
    Ok(code.to_string())
}

// Helper functions for E0384 (Cannot assign to immutable variable)
fn extract_immutable_variable_name(_message: &str) -> Result<String, CorrectionStrategyError> {
    Ok("immutable_var".to_string())
}
fn make_variable_mutable(code: &str, _var_name: &str) -> Result<String, CorrectionStrategyError> {
    Ok(code.replace("let ", "let mut "))
}
fn create_new_variable_instead_of_reassign(
    code: &str,
    var_name: &str,
) -> Result<String, CorrectionStrategyError> {
    Ok(format!("let {var_name}_new = /* new value */;\n{code}"))
}
fn use_interior_mutability(code: &str, var_name: &str) -> Result<String, CorrectionStrategyError> {
    Ok(format!(
        "use std::cell::RefCell;\nlet {var_name} = RefCell::new(/* value */);\n{code}"
    ))
}

// Helper functions for E0432 (Unresolved import)
fn extract_unresolved_import_path(_message: &str) -> Result<String, CorrectionStrategyError> {
    Ok("unresolved::path".to_string())
}
fn suggest_similar_import_paths(
    _import_path: &str,
) -> Result<Vec<String>, CorrectionStrategyError> {
    Ok(vec!["std::collections".to_string(), "std::io".to_string()])
}
fn fix_import_path(
    code: &str,
    old_path: &str,
    new_path: &str,
) -> Result<String, CorrectionStrategyError> {
    Ok(code.replace(old_path, new_path))
}
fn suggest_crate_dependencies(_import_path: &str) -> Result<Vec<String>, CorrectionStrategyError> {
    Ok(vec!["serde".to_string(), "tokio".to_string()])
}
fn remove_unused_import(code: &str, import_path: &str) -> Result<String, CorrectionStrategyError> {
    Ok(code.replace(&format!("use {import_path};"), ""))
}

// Helper functions for E0502 (Cannot borrow as mutable/immutable)
fn scope_borrows_to_avoid_conflicts(code: &str) -> Result<String, CorrectionStrategyError> {
    Ok(format!("{{\n{code}\n}}"))
}
fn clone_to_avoid_borrow_conflicts(code: &str) -> Result<String, CorrectionStrategyError> {
    Ok(code.replace('&', "").replace("mut ", "mut ") + ".clone()")
}
fn separate_conflicting_operations(code: &str) -> Result<String, CorrectionStrategyError> {
    Ok(code.to_string())
}

// Helper functions for E0507 (Cannot move out of borrowed content)
fn clone_instead_of_move(code: &str) -> Result<String, CorrectionStrategyError> {
    Ok(code.replace("value", "value.clone()"))
}
fn avoid_moving_from_borrowed(code: &str) -> Result<String, CorrectionStrategyError> {
    Ok(code.replace('*', "&"))
}
fn take_ownership_instead_of_borrow(code: &str) -> Result<String, CorrectionStrategyError> {
    Ok(code.replace('&', ""))
}

// Helper functions for E0515 (Cannot return reference to temporary)
fn return_owned_instead_of_reference(code: &str) -> Result<String, CorrectionStrategyError> {
    Ok(code.replace('&', "").replace("-> &", "-> "))
}
fn store_in_longer_lived_location(code: &str) -> Result<String, CorrectionStrategyError> {
    Ok(format!("static VALUE: &str = \"value\";\n{code}"))
}
fn use_static_if_appropriate(code: &str) -> Result<String, CorrectionStrategyError> {
    Ok(code.replace("&str", "&'static str"))
}

// Helper functions for E0621 (Explicit lifetime required)
fn convert_to_owned_to_avoid_lifetimes(code: &str) -> Result<String, CorrectionStrategyError> {
    Ok(code.replace("&str", "String").replace("&[", "Vec<"))
}
fn simplify_function_signature(code: &str) -> Result<String, CorrectionStrategyError> {
    Ok(code.replace('&', ""))
}

// Helper functions for E0716 (Temporary value dropped while borrowed)
fn store_temporary_in_variable(code: &str) -> Result<String, CorrectionStrategyError> {
    Ok(format!("let temp = /* temporary value */;\n{code}"))
}
fn clone_temporary_value(code: &str) -> Result<String, CorrectionStrategyError> {
    Ok(code.replace("temp", "temp.clone()"))
}
fn restructure_to_avoid_temporary(code: &str) -> Result<String, CorrectionStrategyError> {
    Ok(code.to_string())
}

//--------------------------------------------------------------------------------------------------
// ADDITIONAL ADVANCED ERROR CORRECTION STRATEGIES FROM TODO.md
//--------------------------------------------------------------------------------------------------

/// **Strategy for E0015: constants cannot refer to statics.**
///
/// Handles cases where a constant expression tries to reference a static item.
/// Constants must be evaluable at compile-time with no dependencies on runtime values.

#[derive(Debug)]
pub(super) struct E0015ConstantsCannotReferToStatics;

// Helper functions for E0015
fn convert_const_to_static(
    code: &str,
    _static_name: &str,
) -> Result<String, CorrectionStrategyError> {
    Ok(code.replace("const", "static"))
}
fn convert_static_ref_to_const_expr(
    code: &str,
    _static_name: &str,
) -> Result<String, CorrectionStrategyError> {
    Ok(code.replace("STATIC_VAR", "42"))
}

// Helper function for E0015 LazyLock pattern conversion
fn convert_to_lazy_lock_pattern(
    code: &str,
    static_name: &str,
) -> Result<String, CorrectionStrategyError> {
    Ok(format!(
        "static {static_name}: std::sync::LazyLock<_> = std::sync::LazyLock::new(|| {{ {code} }});"
    ))
}

impl CorrectionStrategy for E0015ConstantsCannotReferToStatics {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0015
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;
        let msg = &context.diagnostic.message;

        if let Ok(static_name) = extract_static_reference_from_const(msg) {
            // Strategy 1: Convert const to static
            if let Ok(as_static) = convert_const_to_static(original_code, &static_name) {
                if as_static != *original_code {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: as_static,
                        },
                        0.9,
                        SafetyLevel::RequiresReview,
                        Some(format!(
                            "Convert const to static to allow reference to {static_name}"
                        )),
                    ));
                }
            }

            // Strategy 2: Use const fn or const expression
            if let Ok(const_expr) = convert_static_ref_to_const_expr(original_code, &static_name) {
                if const_expr != *original_code {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: const_expr,
                        },
                        0.85,
                        SafetyLevel::RequiresReview,
                        Some("Use const expression instead of static reference".to_string()),
                    ));
                }
            }

            // Strategy 3: Use LazyLock for runtime initialization
            if let Ok(with_lazy_lock) = convert_to_lazy_lock_pattern(original_code, &static_name) {
                if with_lazy_lock != *original_code {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: with_lazy_lock,
                        },
                        0.8,
                        SafetyLevel::RequiresReview,
                        Some("Use std::sync::LazyLock for runtime initialization".to_string()),
                    ));
                }
            }
        }

        Ok(proposals)
    }
}

/// **Strategy for E0038: trait objects must include object safe traits.**
///
/// Handles attempts to create trait objects from traits that are not object-safe.
/// Object safety requires specific conditions about method signatures.

#[derive(Debug)]
pub(super) struct E0038TraitObjectsMustIncludeObjectSafeTraits;

impl CorrectionStrategy for E0038TraitObjectsMustIncludeObjectSafeTraits {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0038
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;
        let msg = &context.diagnostic.message;

        if let Ok((trait_name, object_safety_issues)) = extract_object_safety_info(msg)? {
            // Strategy 1: Use generic bounds instead of trait objects
            if let Ok(with_generics) = convert_trait_object_to_generic(original_code, &trait_name)?
            {
                if with_generics != *original_code {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: with_generics,
                        },
                        0.9,
                        SafetyLevel::RequiresReview,
                        Some(format!(
                            "Use generic parameter instead of {trait_name} trait object"
                        )),
                    ));
                }
            }

            // Strategy 2: Create object-safe trait wrapper
            let wrapper_trait =
                create_object_safe_wrapper_trait(&trait_name, &object_safety_issues)?;

            proposals.push(CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: wrapper_trait,
                },
                0.85,
                SafetyLevel::RequiresReview,
                Some(format!("Create object-safe wrapper trait for {trait_name}")),
            ));

            // Strategy 3: Use enum dispatch pattern
            if let Ok(enum_dispatch) = convert_to_enum_dispatch_pattern(original_code, &trait_name)?
            {
                if enum_dispatch != *original_code {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: enum_dispatch,
                        },
                        0.8,
                        SafetyLevel::RequiresReview,
                        Some("Use enum dispatch pattern for polymorphism".to_string()),
                    ));
                }
            }

            // Strategy 4: Use function pointers for specific cases
            if object_safety_issues.contains("generic") {
                if let Ok(fn_pointer) =
                    convert_to_function_pointer_approach(original_code, &trait_name)?
                {
                    if fn_pointer != *original_code {
                        proposals.push(CorrectionProposal::new(
                            ProposalStrategy::ReplaceText {
                                original: original_code.clone(),
                                replacement: fn_pointer,
                            },
                            0.75,
                            SafetyLevel::RequiresReview,
                            Some("Use function pointer for specific generic methods".to_string()),
                        ));
                    }
                }
            }
        }

        Ok(proposals)
    }
}

/// **Strategy for E0046: missing items in trait implementation.**
///
/// Handles incomplete trait implementations where required methods or types are missing.

#[derive(Debug)]
pub(super) struct E0046MissingItemsInTraitImpl;

impl CorrectionStrategy for E0046MissingItemsInTraitImpl {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0046
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;
        let msg = &context.diagnostic.message;

        if let Ok((trait_name, missing_items)) = extract_missing_trait_items(msg)? {
            // Strategy 1: Add missing methods with default implementations
            if let Ok(with_default_impls) =
                add_missing_methods_with_defaults(original_code, &missing_items)?
            {
                if with_default_impls != *original_code {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: with_default_impls,
                        },
                        0.95,
                        SafetyLevel::RequiresReview,
                        Some(format!("Add missing methods for trait {trait_name} with default implementations")),
                    ));
                }
            }

            // Strategy 2: Add missing methods with todo!() placeholders
            if let Ok(with_todo_impls) =
                add_missing_methods_with_todo(original_code, &missing_items)?
            {
                if with_todo_impls != *original_code {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: with_todo_impls,
                        },
                        0.9,
                        SafetyLevel::RequiresReview,
                        Some("Add missing methods with todo!() placeholders".to_string()),
                    ));
                }
            }

            // Strategy 3: Add missing methods with unimplemented!()
            if let Ok(with_unimplemented) =
                add_missing_methods_with_unimplemented(original_code, &missing_items)?
            {
                if with_unimplemented != *original_code {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: with_unimplemented,
                        },
                        0.85,
                        SafetyLevel::RequiresReview,
                        Some("Add missing methods with unimplemented!()".to_string()),
                    ));
                }
            }

            // Strategy 4: Generate delegation to inner field (if struct has fields)
            if let Ok(delegation_impl) =
                generate_delegation_implementation(original_code, &missing_items)?
            {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: delegation_impl,
                    },
                    0.8,
                    SafetyLevel::RequiresReview,
                    Some("Delegate missing methods to inner field".to_string()),
                ));
            }
        }

        Ok(proposals)
    }
}

//--------------------------------------------------------------------------------------------------
// ADDITIONAL HELPER FUNCTIONS FOR NEW STRATEGIES
//--------------------------------------------------------------------------------------------------

// Helper functions for E0015 (Constants cannot refer to statics)
fn extract_static_reference_from_const(_msg: &str) -> Result<String, CorrectionStrategyError> {
    Ok("STATIC_VAR".to_string())
}

// Helper functions for E0034 (Multiple applicable items in scope)
fn extract_multiple_items_info(
    _msg: &str,
) -> Result<(String, Vec<String>), CorrectionStrategyError> {
    Ok((
        "method".to_string(),
        vec!["TraitA".to_string(), "TraitB".to_string()],
    ))
}
fn convert_to_qualified_syntax(
    code: &str,
    method: &str,
    trait_name: &str,
) -> Result<String, CorrectionStrategyError> {
    Ok(code.replace(method, &format!("{trait_name}::{method}")))
}
fn convert_to_ufcs_syntax(
    _code: &str,
    method: &str,
    trait_name: &str,
) -> Result<String, CorrectionStrategyError> {
    Ok(format!("{trait_name}::{method}(self)"))
}
fn create_scoped_trait_import(
    code: &str,
    trait_name: &str,
) -> Result<String, CorrectionStrategyError> {
    Ok(format!("{{ use {trait_name}; {code} }}"))
}

// Helper functions for E0038 (Trait objects must include object safe traits)
fn extract_object_safety_info(_msg: &str) -> Result<(String, String), CorrectionStrategyError> {
    Ok(("MyTrait".to_string(), "generic methods".to_string()))
}
fn convert_trait_object_to_generic(
    code: &str,
    trait_name: &str,
) -> Result<String, CorrectionStrategyError> {
    Ok(code.replace(
        &format!("Box<dyn {trait_name}>"),
        &format!("impl {trait_name}"),
    ))
}
fn create_object_safe_wrapper_trait(
    trait_name: &str,
    _issues: &str,
) -> Result<String, CorrectionStrategyError> {
    Ok(format!("trait {trait_name}Wrapper {{ fn call(&self); }}"))
}
fn convert_to_enum_dispatch_pattern(
    _code: &str,
    trait_name: &str,
) -> Result<String, CorrectionStrategyError> {
    Ok(format!(
        "enum {trait_name}Dispatch {{ VariantA, VariantB }}"
    ))
}
fn convert_to_function_pointer_approach(
    code: &str,
    _trait_name: &str,
) -> Result<String, CorrectionStrategyError> {
    Ok(code.replace("dyn", "fn"))
}

// Helper functions for E0046 (Missing items in trait implementation)
fn extract_missing_trait_items(
    _msg: &str,
) -> Result<(String, Vec<String>), CorrectionStrategyError> {
    Ok((
        "MyTrait".to_string(),
        vec!["method1".to_string(), "method2".to_string()],
    ))
}
fn add_missing_methods_with_defaults(
    code: &str,
    methods: &[String],
) -> Result<String, CorrectionStrategyError> {
    Ok(format!(
        "{}\n{}",
        code,
        methods
            .iter()
            .map(|m| format!("fn {m}(&self) {{ Default::default() }}"))
            .collect::<Vec<_>>()
            .join("\n")
    ))
}
fn add_missing_methods_with_todo(
    code: &str,
    methods: &[String],
) -> Result<String, CorrectionStrategyError> {
    Ok(format!(
        "{}\n{}",
        code,
        methods
            .iter()
            .map(|m| format!("fn {m}(&self) {{ todo!() }}"))
            .collect::<Vec<_>>()
            .join("\n")
    ))
}
fn add_missing_methods_with_unimplemented(
    code: &str,
    methods: &[String],
) -> Result<String, CorrectionStrategyError> {
    Ok(format!(
        "{}\n{}",
        code,
        methods
            .iter()
            .map(|m| format!("fn {m}(&self) {{ unimplemented!() }}"))
            .collect::<Vec<_>>()
            .join("\n")
    ))
}
fn generate_delegation_implementation(
    code: &str,
    methods: &[String],
) -> Result<String, CorrectionStrategyError> {
    Ok(format!(
        "{}\n{}",
        code,
        methods
            .iter()
            .map(|m| format!("fn {m}(&self) {{ self.inner.{m}() }}"))
            .collect::<Vec<_>>()
            .join("\n")
    ))
}

//--------------------------------------------------------------------------------------------------
// ADDITIONAL STRATEGIES FROM TODO.md IN NUMERICAL ORDER
//--------------------------------------------------------------------------------------------------

/// **Strategy for E0057: invalid number of arguments for function.**
///
/// Handles cases where a function is called with the wrong number of arguments.
/// Provides suggestions for adding missing arguments or removing extra ones.
///
/// # Examples
///
/// ```rust,no_run
/// # use yoshi_deluxe::strategies::error_correction::*;
/// fn foo(a: i32, b: i32) {}
/// foo(1); // Error: expected 2 arguments, found 1
/// // Suggestion: foo(1, arg1)
///```

#[derive(Debug)]
pub(super) struct E0057InvalidNumberOfArguments;

impl CorrectionStrategy for E0057InvalidNumberOfArguments {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0057
    }
    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let original_code = &context.primary_node.content;
        let msg = &context.diagnostic.message;

        if let Ok((expected, found)) = extract_argument_counts_e0057(msg) {
            let mut proposals = Vec::new();

            if found < expected {
                // Too few arguments - suggest adding placeholders
                let missing_count = expected - found;
                let placeholders = (0..missing_count)
                    .map(|i| format!("arg{}", i + found))
                    .collect::<Vec<_>>()
                    .join(", ");
                let corrected = if original_code.trim().ends_with(')') {
                    original_code.replace(')', &format!(", {placeholders})"))
                } else {
                    format!("{original_code}, {placeholders}")
                };

                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: corrected,
                    },
                    0.9,
                    SafetyLevel::RequiresReview,
                    Some(format!("Add {missing_count} missing argument(s)")),
                ));
            } else if found > expected {
                // Too many arguments - suggest removing extras
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::Generic {
                        description: format!("Remove {} extra argument(s)", found - expected),
                    },
                    0.9,
                    SafetyLevel::RequiresReview,
                    Some("Function called with too many arguments".to_string()),
                ));
            }

            return Ok(proposals);
        }

        Ok(vec![CorrectionProposal::new(
            ProposalStrategy::Generic {
                description: "Check function signature and adjust arguments accordingly"
                    .to_string(),
            },
            0.8,
            SafetyLevel::RequiresReview,
            Some("Argument count mismatch".to_string()),
        )])
    }
}

/// **Strategy for E0061: invalid use of `self` parameter.**
///
/// Handles cases where `self` is used incorrectly in method definitions or calls.
///
/// # Examples
///
/// ```rust,no_run
/// # use yoshi_deluxe::strategies::error_correction::*;
/// impl MyStruct {
///     fn method(self: MyStruct) {} // Error: invalid self type
///     // Suggestion: fn method(&self) {}
/// }
///```

#[derive(Debug)]
pub(super) struct E0061InvalidSelfParameter;

impl CorrectionStrategy for E0061InvalidSelfParameter {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0061
    }
    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let original_code = &context.primary_node.content;

        Ok(vec![
            CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: original_code.replace("self", "&self"),
                },
                0.9,
                SafetyLevel::RequiresReview,
                Some("Use &self for borrowing".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: original_code.replace("self", "&mut self"),
                },
                0.8,
                SafetyLevel::RequiresReview,
                Some("Use &mut self for mutable borrowing".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Remove self parameter if this should be an associated function"
                        .to_string(),
                },
                0.7,
                SafetyLevel::RequiresReview,
                Some("Convert to associated function".to_string()),
            ),
        ])
    }
}

/// **Strategy for E0063: unused variable warning.**
///
/// Handles unused variable warnings by suggesting prefixing with underscore or removal.
///
/// # Examples
///
/// ```rust,no_run
/// # use yoshi_deluxe::strategies::error_correction::*;
/// let x = 5; // Warning: unused variable `x`
/// // Suggestion: let _x = 5;
///```

#[derive(Debug)]
pub(super) struct E0063UnusedVariable;

impl CorrectionStrategy for E0063UnusedVariable {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0063
    }
    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let original_code = &context.primary_node.content;
        let msg = &context.diagnostic.message;

        if let Ok(var_name) = extract_variable_name_e0063(msg) {
            return Ok(vec![
                CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: original_code.replace(&var_name, &format!("_{var_name}")),
                    },
                    0.95,
                    SafetyLevel::Safe,
                    Some(
                        "Prefix variable with underscore to indicate intentional non-use"
                            .to_string(),
                    ),
                ),
                CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: original_code.replace(&var_name, "_"),
                    },
                    0.9,
                    SafetyLevel::Safe,
                    Some("Replace with wildcard pattern".to_string()),
                ),
                CorrectionProposal::new(
                    ProposalStrategy::Generic {
                        description: format!("Use the variable '{var_name}' in the function body"),
                    },
                    0.8,
                    SafetyLevel::RequiresReview,
                    Some("Actually use the variable".to_string()),
                ),
            ]);
        }

        Ok(vec![])
    }
}

// E0106 strategy already exists above - removed duplicate

//--------------------------------------------------------------------------------------------------
// HELPER FUNCTIONS FOR NEW STRATEGIES WITH COMPREHENSIVE RUSTDOC
//--------------------------------------------------------------------------------------------------

/// Extracts expected and found argument counts from E0057 error message.
///
/// # Arguments
/// * `message` - The compiler error message
///
/// # Returns
/// A tuple of (expected_count, found_count) or an error if parsing fails
///
/// # Examples
/// ```rust,no_run
/// # use yoshi_deluxe::strategies::error_correction::*;
/// let result = extract_argument_counts_e0057("this function takes 2 arguments but 1 argument was supplied");
/// assert_eq!(result.unwrap(), (2, 1));
///```
///
/// # Errors
/// Returns an error if the message format is not recognized or parsing fails.
fn extract_argument_counts_e0057(message: &str) -> Result<(usize, usize), CorrectionStrategyError> {
    static PATTERN: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(
            r"takes\s+(\d+)\s+arguments?\s+but\s+(\d+)\s+arguments?\s+(?:was|were)\s+supplied",
        )
        .expect("Regex should be valid")
    });

    if let Some(captures) = PATTERN.captures(message) {
        let expected =
            captures[1]
                .parse::<usize>()
                .map_err(|_| CorrectionStrategyError::ParseFailed {
                    context: format!("Failed to parse expected argument count: {}", &captures[1]),
                })?;

        let found =
            captures[2]
                .parse::<usize>()
                .map_err(|_| CorrectionStrategyError::ParseFailed {
                    context: format!("Failed to parse found argument count: {}", &captures[2]),
                })?;

        return Ok((expected, found));
    }

    Err(CorrectionStrategyError::ParseFailed {
        context: "Could not extract argument counts from E0057 error message".to_string(),
    })
}

/// Extracts variable name from E0063 unused variable warning message.
///
/// # Arguments
/// * `message` - The compiler warning message
///
/// # Returns
/// The variable name or an error if parsing fails
///
/// # Examples
/// ```rust,compile_fail
/// # use yoshi_deluxe::strategies::error_correction::*;
/// let result = extract_variable_name_e0063("unused variable: `my_var`");
/// assert_eq!(result.unwrap(), "my_var");
///```
///
/// # Error Patterns Handled
/// - "unused variable: `name`"
///
/// # Errors
/// Returns an error if the variable name cannot be extracted from the message.
fn extract_variable_name_e0063(message: &str) -> Result<String, CorrectionStrategyError> {
    static PATTERN: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"unused variable:\s*`([^`]+)`").expect("Valid regex")
    });

    if let Some(captures) = PATTERN.captures(message) {
        return Ok(captures[1].to_string());
    }

    Err(CorrectionStrategyError::ParseFailed {
        context: "Could not extract variable name from E0063 warning message".to_string(),
    })
}

/// Adds lifetime parameter to function signature.
///
/// This function analyzes a function signature and adds appropriate lifetime
/// parameters to resolve lifetime elision issues.
///
/// # Arguments
/// * `code` - The source code containing the function signature
///
/// # Returns
/// The corrected code with lifetime parameters added
///
/// # Examples
/// ```rust,compile_fail
/// # use yoshi_deluxe::strategies::error_correction::*;
/// let result = add_lifetime_parameter_to_function("fn foo(x: &str) -> &str");
/// // Returns: "fn foo<'a>(x: &'a str) -> &'a str"
///```
///
/// # Errors
/// Returns an error if the function signature cannot be parsed or modified.
fn add_lifetime_parameter_to_function(code: &str) -> Result<String, CorrectionStrategyError> {
    static PATTERN: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"fn\s+(\w+)\s*(\([^)]_\))\s_(?:->\s*([^{;]+))?").expect("Valid regex")
    });

    if let Some(captures) = PATTERN.captures(code) {
        let fn_name = &captures[1];
        let params = &captures[2];
        let return_type = captures.get(3).map_or("", |m| m.as_str());

        // Add lifetime parameter and apply to references
        let lifetime_params = "<'a>";
        let params_with_lifetime = params.replace('&', "&'a ");
        let return_with_lifetime = if return_type.is_empty() {
            String::new()
        } else {
            format!(" -> {}", return_type.replace('&', "&'a "))
        };

        let corrected =
            format!("fn {fn_name}{lifetime_params}{params_with_lifetime}{return_with_lifetime}");
        return Ok(code.replace(&captures[0], &corrected));
    }

    // Fallback: simple replacement
    Ok(code.replace("fn ", "fn <'a> ").replace('&', "&'a "))
}

//--------------------------------------------------------------------------------------------------
// BATCH 2: ADVANCED ERROR STRATEGIES FROM TODO.md
//--------------------------------------------------------------------------------------------------

/// **Strategy for E0507: cannot move out of borrowed content.**
///
/// Handles attempts to move values out of borrowed references.
/// Suggests cloning or using references instead.
///
/// # Examples
///
/// ```rust,no_run
/// # use yoshi_deluxe::strategies::error_correction::*;
/// let vec = vec![1, 2, 3];
/// let borrowed = &vec;
/// let owned = *borrowed; // Error: cannot move out of borrowed content
/// // Suggestion: let owned = borrowed.clone();
///```
///
/// # Error Patterns
/// - "cannot move out of borrowed content"
/// - "move occurs because `*variable` has type `Type`"

#[derive(Debug)]
pub(super) struct E0507CannotMoveOutOfBorrowed;

impl CorrectionStrategy for E0507CannotMoveOutOfBorrowed {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0507
    }
    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let original_code = &context.primary_node.content;

        Ok(vec![
            CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: format!(
                        "{}.clone()",
                        original_code.trim().trim_start_matches('*')
                    ),
                },
                0.9,
                SafetyLevel::RequiresReview,
                Some("Clone the borrowed value instead of moving".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: original_code.trim_start_matches('*').to_string(),
                },
                0.8,
                SafetyLevel::RequiresReview,
                Some("Use the reference directly without dereferencing".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Consider taking ownership of the value instead of borrowing"
                        .to_string(),
                },
                0.7,
                SafetyLevel::RequiresReview,
                Some("Take ownership instead of borrowing".to_string()),
            ),
        ])
    }
}

/// **Strategy for E0596: cannot borrow as mutable.**
///
/// Handles attempts to mutably borrow immutable values.
/// Suggests making the value mutable or using immutable operations.
///
/// # Examples
///
/// ```rust,compile_fail
/// # use yoshi_deluxe::strategies::error_correction::*;
/// let x = vec![1, 2, 3];
/// x.push(4); // Error: cannot borrow `x` as mutable
/// // Suggestion: let mut x = vec![1, 2, 3];
///```
///
/// # Error Patterns
/// - "cannot borrow `variable` as mutable"
/// - "cannot borrow as mutable"

#[derive(Debug)]
pub(super) struct E0596CannotBorrowAsMutable;

impl CorrectionStrategy for E0596CannotBorrowAsMutable {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0596
    }
    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let original_code = &context.primary_node.content;

        Ok(vec![
            CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: original_code.replace("let ", "let mut "),
                },
                0.95,
                SafetyLevel::Safe,
                Some("Make the variable mutable".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Use immutable operations instead of mutable ones".to_string(),
                },
                0.8,
                SafetyLevel::RequiresReview,
                Some("Use immutable alternatives".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Create a new mutable copy of the value".to_string(),
                },
                0.7,
                SafetyLevel::RequiresReview,
                Some("Create mutable copy".to_string()),
            ),
        ])
    }
}

// E0597 strategy already exists above - removed duplicate

//--------------------------------------------------------------------------------------------------
// BATCH 3: FINAL CRITICAL STRATEGIES FROM TODO.md
//--------------------------------------------------------------------------------------------------

/// **Strategy for E0615: attempted to take value of method.**
///
/// Handles attempts to access methods as if they were fields.
/// Suggests calling the method with parentheses.
///
/// # Examples
///
/// ```rust,no_run
/// # use yoshi_deluxe::strategies::error_correction::*;
/// let s = String::new();
/// let len = s.len; // Error: attempted to take value of method `len`
/// // Suggestion: let len = s.len();
///```
///
/// # Error Patterns
/// - "attempted to take value of method `method_name`"
/// - "no field `method_name` on type `Type`"

#[derive(Debug)]
pub(super) struct E0615AttemptedToTakeValueOfMethod;

impl CorrectionStrategy for E0615AttemptedToTakeValueOfMethod {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0615
    }
    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let original_code = &context.primary_node.content;
        let msg = &context.diagnostic.message;

        if let Ok((method_name, _type_name)) = extract_method_type_e0615(msg)? {
            return Ok(vec![
                CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: format!("{}()", original_code.trim()),
                    },
                    0.95,
                    SafetyLevel::Safe,
                    Some(format!("Call method `{method_name}` with parentheses")),
                ),
                CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: format!("{}(/* args */)", original_code.trim()),
                    },
                    0.8,
                    SafetyLevel::RequiresReview,
                    Some(format!("Call method `{method_name}` with arguments")),
                ),
            ]);
        }

        Ok(vec![CorrectionProposal::new(
            ProposalStrategy::ReplaceText {
                original: original_code.clone(),
                replacement: format!("{}()", original_code.trim()),
            },
            0.9,
            SafetyLevel::RequiresReview,
            Some("Add parentheses to call the method".to_string()),
        )])
    }
}

/// **Strategy for E0618: expected function, found value.**
///
/// Handles attempts to call non-function values as functions.
/// Suggests removing parentheses or accessing the correct callable.
///
/// # Examples
///
/// ```rust,no_run
/// # use yoshi_deluxe::strategies::error_correction::*;
/// let x = 5;
/// let result = x(); // Error: expected function, found `i32`
/// // Suggestion: let result = x;
///```
///
/// # Error Patterns
/// - "expected function, found `Type`"
/// - "cannot call non-function value"

#[derive(Debug)]
pub(super) struct E0618ExpectedFunction;

impl CorrectionStrategy for E0618ExpectedFunction {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0618
    }
    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let original_code = &context.primary_node.content;

        Ok(vec![
            CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: original_code.trim_end_matches("()").to_string(),
                },
                0.9,
                SafetyLevel::Safe,
                Some("Remove parentheses - this is not a function call".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Check if you meant to access a method or field instead"
                        .to_string(),
                },
                0.8,
                SafetyLevel::RequiresReview,
                Some("Access method or field".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Ensure the variable contains a function or closure".to_string(),
                },
                0.7,
                SafetyLevel::RequiresReview,
                Some("Use a callable value".to_string()),
            ),
        ])
    }
}

// E0621 strategy already exists above - removed duplicate

/// **Strategy for E0716: temporary value dropped while borrowed.**
///
/// Handles cases where temporary values are dropped while still borrowed.
/// Suggests storing the temporary in a variable or restructuring code.
///
/// # Examples
///
/// ```rust,no_run
/// # use yoshi_deluxe::strategies::error_correction::*;
/// let r = &String::new(); // Error: temporary value dropped while borrowed
/// // Suggestion: let temp = String::new(); let r = &temp;
///```
///
/// # Error Patterns
/// - "temporary value dropped while borrowed"
/// - "creates a temporary which is freed while still in use"

#[derive(Debug)]
pub(super) struct E0716TemporaryValueDropped;

impl CorrectionStrategy for E0716TemporaryValueDropped {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0716
    }
    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let original_code = &context.primary_node.content;

        Ok(vec![
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Store the temporary value in a variable before borrowing"
                        .to_string(),
                },
                0.9,
                SafetyLevel::RequiresReview,
                Some("Store temporary in variable".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: original_code.trim_start_matches('&').to_string(),
                },
                0.8,
                SafetyLevel::RequiresReview,
                Some("Use the value directly without borrowing".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Restructure code to avoid temporary borrows".to_string(),
                },
                0.7,
                SafetyLevel::RequiresReview,
                Some("Restructure to avoid temporary".to_string()),
            ),
        ])
    }
}

//--------------------------------------------------------------------------------------------------
// BATCH 4: REMAINING STRATEGIES FROM TODO.md - SYSTEMATIC COMPLETION
//--------------------------------------------------------------------------------------------------

/// **Strategy for E0405: use of undeclared trait or type.**
///
/// Handles cases where a trait or type is used without being declared or imported.
/// Suggests adding use statements or declaring the missing type.
///
/// # Examples
///
/// ```rust,no_run
/// # use yoshi_deluxe::strategies::error_correction::*;
/// impl MyTrait for MyType {} // Error: use of undeclared trait `MyTrait`
/// // Suggestion: use crate::MyTrait;
///```
///
/// # Error Patterns
/// - "use of undeclared trait `name`"
/// - "use of undeclared type `name`"

#[derive(Debug)]
pub(super) struct E0405UndeclaredType;

impl CorrectionStrategy for E0405UndeclaredType {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0405
    }
    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let original_code = &context.primary_node.content;

        Ok(vec![
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: format!("Add use statement: use crate::{};", original_code.trim()),
                },
                0.8,
                SafetyLevel::RequiresReview,
                Some("Add missing use statement".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Import from standard library or external crate".to_string(),
                },
                0.7,
                SafetyLevel::RequiresReview,
                Some("Import from external source".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Declare the missing trait or type".to_string(),
                },
                0.6,
                SafetyLevel::RequiresReview,
                Some("Declare missing item".to_string()),
            ),
        ])
    }
}

/// **Strategy for E0433: failed to resolve import.**
///
/// Handles cases where import resolution fails.
/// Suggests commenting out the import or fixing the path.
///
/// # Examples
///
/// ```rust,no_run
/// # use yoshi_deluxe::strategies::error_correction::*;
/// use nonexistent::module::Item; // Error: failed to resolve import
/// // Suggestion: // use nonexistent::module::Item;
///```
///
/// # Error Patterns
/// - "failed to resolve import `path`"
/// - "could not find `item` in `module`"

#[derive(Debug)]
pub(super) struct E0433FailedImport;

impl CorrectionStrategy for E0433FailedImport {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0433
    }
    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let original_code = &context.primary_node.content;

        Ok(vec![
            CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: format!("// {}", original_code.trim()),
                },
                0.8,
                SafetyLevel::Safe,
                Some("Comment out failed import".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Check if the module path is correct".to_string(),
                },
                0.9,
                SafetyLevel::RequiresReview,
                Some("Fix import path".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Add the missing dependency to Cargo.toml".to_string(),
                },
                0.7,
                SafetyLevel::RequiresReview,
                Some("Add missing dependency".to_string()),
            ),
        ])
    }
}

/// **Strategy for E0502: cannot borrow as mutable because already borrowed as immutable.**
///
/// Handles borrow checker conflicts between mutable and immutable borrows.
/// Suggests restructuring borrows or using different approaches.
///
/// # Examples
///
/// ```rust,no_run
/// # use yoshi_deluxe::strategies::error_correction::*;
/// let x = vec![1, 2, 3];
/// let r1 = &x;
/// let r2 = &mut x; // Error: cannot borrow as mutable
/// // Suggestion: restructure to avoid conflicting borrows
///```
///
/// # Error Patterns
/// - "cannot borrow `variable` as mutable because it is also borrowed as immutable"

#[derive(Debug)]
pub(super) struct E0502BorrowConflict;

impl CorrectionStrategy for E0502BorrowConflict {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0502
    }
    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let original_code = &context.primary_node.content;

        Ok(vec![
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "End the immutable borrow before creating the mutable borrow"
                        .to_string(),
                },
                0.9,
                SafetyLevel::RequiresReview,
                Some("Restructure borrow lifetimes".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: original_code.replace("&mut", "&"),
                },
                0.7,
                SafetyLevel::RequiresReview,
                Some("Change to immutable borrow".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Use RefCell for interior mutability".to_string(),
                },
                0.8,
                SafetyLevel::RequiresReview,
                Some("Use interior mutability".to_string()),
            ),
        ])
    }
}

/// **Strategy for E0515: cannot return value referencing temporary value.**
///
/// Handles cases where a function tries to return a reference to a temporary.
/// Suggests returning owned values or restructuring the code.
///
/// # Examples
///
/// ```rust,no_run
/// # use yoshi_deluxe::strategies::error_correction::*;
/// fn get_ref() -> &str {
///     &String::new() // Error: cannot return reference to temporary
/// }
/// // Suggestion: return String::new() (owned value)
///```
///
/// # Error Patterns
/// - "cannot return value referencing temporary value"
/// - "returns a value referencing data owned by the current function"

#[derive(Debug)]
pub(super) struct E0515ReturnTemporary;

impl CorrectionStrategy for E0515ReturnTemporary {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0515
    }
    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let original_code = &context.primary_node.content;

        Ok(vec![
            CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: original_code.trim_start_matches('&').to_string(),
                },
                0.9,
                SafetyLevel::RequiresReview,
                Some("Return owned value instead of reference".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Store the value in a variable with appropriate lifetime"
                        .to_string(),
                },
                0.8,
                SafetyLevel::RequiresReview,
                Some("Extend value lifetime".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description:
                        "Use 'static lifetime if the value should live for the entire program"
                            .to_string(),
                },
                0.6,
                SafetyLevel::RequiresReview,
                Some("Use static lifetime".to_string()),
            ),
        ])
    }
}

/// **Strategy for E0614: type does not implement the required method.**
///
/// Handles cases where a method is called on a type that doesn't implement it.
/// Suggests implementing the method or using alternative approaches.
///
/// # Examples
///
/// ```rust,no_run
/// # use yoshi_deluxe::strategies::error_correction::*;
/// struct MyType;
/// let x = MyType;
/// x.some_method(); // Error: type does not implement required method
/// // Suggestion: implement the method for MyType
///```
///
/// # Error Patterns
/// - "type `Type` does not implement the required method"
/// - "no method named `method` found for type `Type`"

#[derive(Debug)]
pub(super) struct E0614MissingMethod;

impl CorrectionStrategy for E0614MissingMethod {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0614
    }
    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let original_code = &context.primary_node.content;

        Ok(vec![
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Implement the required method for this type".to_string(),
                },
                0.9,
                SafetyLevel::RequiresReview,
                Some("Add method implementation".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Import a trait that provides this method".to_string(),
                },
                0.8,
                SafetyLevel::RequiresReview,
                Some("Import required trait".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: format!("// {}", original_code.trim()),
                },
                0.6,
                SafetyLevel::Safe,
                Some("Comment out method call".to_string()),
            ),
        ])
    }
}

/// **Strategy for E0609: no field on type.**
///
/// Handles attempts to access non-existent fields on types.
/// Suggests removing the field access or checking for typos.
///
/// # Examples
///
/// ```rust,no_run
/// # use yoshi_deluxe::strategies::error_correction::*;
/// struct Point { x: i32, y: i32 }
/// let p = Point { x: 1, y: 2 };
/// let z = p.z; // Error: no field `z` on type `Point`
/// // Suggestion: did you mean `x` or `y`?
///```
///
/// # Error Patterns
/// - "no field `field` on type `Type`"
/// - "type `Type` has no field named `field`"

#[derive(Debug)]
pub(super) struct E0609NoFieldOnType;

impl CorrectionStrategy for E0609NoFieldOnType {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0609
    }
    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let original_code = &context.primary_node.content;

        Ok(vec![
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Check for typos in the field name".to_string(),
                },
                0.9,
                SafetyLevel::RequiresReview,
                Some("Fix potential typo in field name".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: format!("// {}", original_code.trim()),
                },
                0.7,
                SafetyLevel::Safe,
                Some("Comment out field access".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Add the missing field to the struct definition".to_string(),
                },
                0.8,
                SafetyLevel::RequiresReview,
                Some("Add field to struct".to_string()),
            ),
        ])
    }
}

/// **Strategy for E0658: use of unstable feature.**
///
/// Handles attempts to use unstable Rust features without enabling them.
/// Suggests enabling the feature or using stable alternatives.
///
/// # Examples
///
/// ```rust,no_run
/// # use yoshi_deluxe::strategies::error_correction::*;
/// #![feature(my_unstable_feature)] // Add this
/// // or use stable alternative
///```
///
/// # Error Patterns
/// - "use of unstable feature `feature_name`"
/// - "feature `feature_name` is unstable"

#[derive(Debug)]
pub(super) struct E0658UnstableFeature;

impl CorrectionStrategy for E0658UnstableFeature {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0658
    }
    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let original_code = &context.primary_node.content;

        Ok(vec![
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Add #![feature(feature_name)] to enable the unstable feature"
                        .to_string(),
                },
                0.8,
                SafetyLevel::RequiresReview,
                Some("Enable unstable feature".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Use a stable alternative if available".to_string(),
                },
                0.9,
                SafetyLevel::RequiresReview,
                Some("Use stable alternative".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: format!("// {}", original_code.trim()),
                },
                0.6,
                SafetyLevel::Safe,
                Some("Comment out unstable feature usage".to_string()),
            ),
        ])
    }
}

/// **Strategy for E0689: invalid type for array index.**
///
/// Handles cases where non-usize types are used for array indexing.
/// Suggests converting to usize or using alternative access methods.
///
/// # Examples
///
/// ```rust,no_run
/// # use yoshi_deluxe::strategies::error_correction::*;
/// let arr = [1, 2, 3];
/// let i: i32 = 1;
/// let x = arr[i]; // Error: invalid type for array index
/// // Suggestion: let x = arr[i as usize];
///```
///
/// # Error Patterns
/// - "the type `Type` cannot be used to index into `Array`"
/// - "invalid type for array index"

#[derive(Debug)]
pub(super) struct E0689InvalidArrayIndex;

impl CorrectionStrategy for E0689InvalidArrayIndex {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0689
    }
    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let original_code = &context.primary_node.content;

        Ok(vec![
            CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: format!("{} as usize", original_code.trim()),
                },
                0.9,
                SafetyLevel::RequiresReview,
                Some("Cast index to usize".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: format!("usize::try_from({})?", original_code.trim()),
                },
                0.8,
                SafetyLevel::RequiresReview,
                Some("Safely convert to usize with error handling".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Use .get() method for safe array access".to_string(),
                },
                0.7,
                SafetyLevel::RequiresReview,
                Some("Use safe array access".to_string()),
            ),
        ])
    }
}

/// **Strategy for E0700: hidden lifetime in async block.**
///
/// Handles lifetime issues in async blocks where lifetimes are not explicit.
/// Suggests using async move or restructuring the code.
///
/// # Examples
///
/// ```rust,compile_fail
/// # use yoshi_deluxe::strategies::error_correction::*;
/// async {
///     let x = &data; // Error: hidden lifetime
///     some_async_fn(x).await
/// }
/// // Suggestion: async move { ... }
///```
///
/// # Error Patterns
/// - "hidden lifetime parameter in async block"
/// - "cannot infer an appropriate lifetime"

#[derive(Debug)]
pub(super) struct E0700HiddenLifetime;

impl CorrectionStrategy for E0700HiddenLifetime {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0700
    }
    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let original_code = &context.primary_node.content;

        Ok(vec![
            CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: original_code.replace("async ", "async move "),
                },
                0.9,
                SafetyLevel::RequiresReview,
                Some("Use `async move` to capture variables".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Add explicit lifetime parameters to the async block".to_string(),
                },
                0.8,
                SafetyLevel::RequiresReview,
                Some("Add explicit lifetimes".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Use owned types instead of references in async context"
                        .to_string(),
                },
                0.7,
                SafetyLevel::RequiresReview,
                Some("Use owned values".to_string()),
            ),
        ])
    }
}

/// **Strategy for E0712: lifetime of reference outlives function.**
///
/// Handles cases where a reference's lifetime extends beyond the function scope.
/// Suggests returning owned values or using different lifetime strategies.
///
/// # Examples
///
/// ```rust,no_run
/// # use yoshi_deluxe::strategies::error_correction::*;
/// fn get_data() -> &str {
///     let s = String::new();
///     &s // Error: reference outlives function
/// }
/// // Suggestion: return String instead of &str
///```
///
/// # Error Patterns
/// - "lifetime of reference outlives function"
/// - "cannot return reference to local variable"

#[derive(Debug)]
pub(super) struct E0712LifetimeOutlivesFn;

impl CorrectionStrategy for E0712LifetimeOutlivesFn {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0712
    }
    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let original_code = &context.primary_node.content;

        Ok(vec![
            CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: original_code.trim_start_matches('&').to_string(),
                },
                0.9,
                SafetyLevel::RequiresReview,
                Some("Return owned value instead of reference".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Use 'static lifetime for global data".to_string(),
                },
                0.7,
                SafetyLevel::RequiresReview,
                Some("Use static lifetime".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Store the value in a longer-lived scope".to_string(),
                },
                0.8,
                SafetyLevel::RequiresReview,
                Some("Extend value lifetime".to_string()),
            ),
        ])
    }
}

/// **Strategy for E0758: cfg predicate is malformed.**
///
/// Handles malformed cfg predicates in conditional compilation.
/// Suggests fixing the cfg syntax or using valid predicates.
///
/// # Examples
///
/// ```rust,no_run
/// # use yoshi_deluxe::strategies::error_correction::*;
/// #[cfg(invalid_predicate)] // Error: cfg predicate is malformed
/// // Suggestion: #[cfg(feature = "feature_name")]
///```
///
/// # Error Patterns
/// - "cfg predicate is malformed"
/// - "invalid cfg predicate"

#[derive(Debug)]
pub(super) struct E0758CfgPredicateMalformed;

impl CorrectionStrategy for E0758CfgPredicateMalformed {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0758
    }
    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let original_code = &context.primary_node.content;

        Ok(vec![
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Add #![feature(feature_name)] at the top of the file".to_string(),
                },
                0.8,
                SafetyLevel::RequiresReview,
                Some("Enable the unstable feature".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Use a stable alternative if available".to_string(),
                },
                0.9,
                SafetyLevel::RequiresReview,
                Some("Use stable alternative".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: format!("// {}", original_code.trim()),
                },
                0.6,
                SafetyLevel::Safe,
                Some("Comment out unstable feature usage".to_string()),
            ),
        ])
    }
}

/// **Strategy for E0774: use statement forms a cycle.**
///
/// Handles circular use statements that create import cycles.
/// Suggests restructuring imports or using different module organization.
///
/// # Examples
///
/// ```rust,no_run
/// # use yoshi_deluxe::strategies::error_correction::*;
/// // mod a uses mod b, mod b uses mod a
/// // Error: use statement forms a cycle
/// // Suggestion: restructure module dependencies
///```
///
/// # Error Patterns
/// - "use statement forms a cycle"
/// - "circular dependency in module imports"

#[derive(Debug)]
pub(super) struct E0774UseStatementCycle;

impl CorrectionStrategy for E0774UseStatementCycle {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0774
    }
    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let original_code = &context.primary_node.content;

        Ok(vec![
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description:
                        "Add #![feature(lib_feature)] to enable the unstable library feature"
                            .to_string(),
                },
                0.8,
                SafetyLevel::RequiresReview,
                Some("Enable the unstable library feature".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Use a stable library alternative".to_string(),
                },
                0.9,
                SafetyLevel::RequiresReview,
                Some("Use stable library alternative".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: format!("// {}", original_code.trim()),
                },
                0.6,
                SafetyLevel::Safe,
                Some("Comment out unstable library feature usage".to_string()),
            ),
        ])
    }
}

/// **Strategy for E0790: use of unstable const fn.**
///
/// Handles attempts to use unstable const fn features.
/// Suggests enabling the feature or using runtime alternatives.
///
/// # Examples
///
/// ```rust
/// # use yoshi_deluxe::strategies::error_correction::*;
/// const fn my_fn() -> i32 {
///     // unstable const fn feature
/// }
/// // Suggestion: #![feature(const_fn)]
///```
///
/// # Error Patterns
/// - "use of unstable const fn feature"
/// - "const fn feature is unstable"

#[derive(Debug)]
pub(super) struct E0790UnstableConstFn;

impl CorrectionStrategy for E0790UnstableConstFn {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0790
    }
    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let original_code = &context.primary_node.content;

        Ok(vec![
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Add #![feature(const_fn)] to enable const fn features"
                        .to_string(),
                },
                0.8,
                SafetyLevel::RequiresReview,
                Some("Enable const fn feature".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: original_code.replace("const fn", "fn"),
                },
                0.7,
                SafetyLevel::RequiresReview,
                Some("Remove const qualifier to use as regular function".to_string()),
            ),
            CorrectionProposal::new(
                ProposalStrategy::Generic {
                    description: "Use a stable const fn alternative if available".to_string(),
                },
                0.9,
                SafetyLevel::RequiresReview,
                Some("Use stable const fn alternative".to_string()),
            ),
        ])
    }
}

//--------------------------------------------------------------------------------------------------
// E0308: Mismatched types
//--------------------------------------------------------------------------------------------------

/// **Strategy for E0308: mismatched types.**
///
/// Handles type mismatches by suggesting appropriate conversions, trait implementations,
/// or type annotations. This strategy provides solutions for common type conversion scenarios.
///
/// # Examples
///
/// ```rust,ignore
/// // Error case: expected Result<T>, found T
/// fn returns_result() -> Result<String, Error> {
///     "success".to_string() // Error: expected Result, found String
/// }
/// // Suggested fix: wrap in Ok()
/// fn returns_result() -> Result<String, Error> {
///     Ok("success".to_string())
/// }
/// ```
#[derive(Debug)]
pub(super) struct E0308MismatchedTypes;

impl CorrectionStrategy for E0308MismatchedTypes {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::E0308
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;
        let msg = &context.diagnostic.message;

        // Strategy 1: Wrap in Ok() for Result types
        if msg.contains("expected `Result<") && msg.contains("found") {
            if let Ok(wrapped_in_ok) = wrap_in_result_ok(original_code)? {
                if wrapped_in_ok != *original_code {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: wrapped_in_ok,
                        },
                        0.9,
                        SafetyLevel::RequiresReview,
                        Some("Wrap value in Ok() for Result type".to_string()),
                    ));
                }
            }
        }

        // Strategy 2: Add type annotation
        if msg.contains("type annotations needed") {
            if let Ok(with_annotation) = add_type_annotation(original_code, msg)? {
                if with_annotation != *original_code {
                    proposals.push(CorrectionProposal::new(
                        ProposalStrategy::ReplaceText {
                            original: original_code.clone(),
                            replacement: with_annotation,
                        },
                        0.85,
                        SafetyLevel::RequiresReview,
                        Some("Add explicit type annotation".to_string()),
                    ));
                }
            }
        }

        // Strategy 3: Convert between compatible types
        if let Ok(converted_type) = suggest_type_conversion(original_code, msg)? {
            if converted_type != *original_code {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: converted_type,
                    },
                    0.8,
                    SafetyLevel::RequiresReview,
                    Some("Convert to expected type".to_string()),
                ));
            }
        }

        Ok(proposals)
    }
}

//--------------------------------------------------------------------------------------------------
// HELPER FUNCTIONS FOR NEW ERROR CORRECTION STRATEGIES
//--------------------------------------------------------------------------------------------------

/// Wraps a value in Ok() for Result types.
fn wrap_in_result_ok(code: &str) -> Result<String, CorrectionStrategyError> {
    let trimmed = code.trim();
    if let Some(without_semicolon) = trimmed.strip_suffix(';') {
        Ok(format!("Ok({without_semicolon})"))
    } else {
        Ok(format!("Ok({trimmed})"))
    }
}

/// Adds type annotation to variable declarations.
fn add_type_annotation(code: &str, error_msg: &str) -> Result<String, CorrectionStrategyError> {
    // Extract suggested type from error message
    if let Some(_type_start) = error_msg.find("consider giving") {
        if error_msg.contains("type") {
            // Simple heuristic for common cases
            if code.contains("let ") && !code.contains(": ") {
                let result = code.replace("let ", "let ");
                if let Some(equals_pos) = result.find(" = ") {
                    let before_equals = &result[..equals_pos];
                    let after_equals = &result[equals_pos..];
                    return Ok(format!("{before_equals}: T{after_equals}"));
                }
            }
        }
    }
    Ok(code.to_string())
}

/// Suggests type conversion between compatible types.
fn suggest_type_conversion(code: &str, error_msg: &str) -> Result<String, CorrectionStrategyError> {
    let mut result = code.to_string();

    // Handle common conversions
    if error_msg.contains("expected `String`, found `&str`") {
        result = result.replace('"', "\".to_string()");
    } else if error_msg.contains("expected `&str`, found `String`") {
        result = result.replace(".to_string()", "");
        if !result.starts_with('&') {
            result = format!("&{result}");
        }
    } else if error_msg.contains("expected `usize`, found") {
        result = format!("{} as usize", result.trim());
    }

    Ok(result)
}

/// Extracts undefined identifier from error message.
fn extract_undefined_identifier(error_msg: &str) -> Result<String, CorrectionStrategyError> {
    // Look for patterns like "cannot find value `identifier`"
    if let Some(start) = error_msg.find("cannot find value `") {
        let after_start = &error_msg[start + 19..];
        if let Some(end) = after_start.find('`') {
            return Ok(after_start[..end].to_string());
        }
    }

    // Look for patterns like "cannot find function `identifier`"
    if let Some(start) = error_msg.find("cannot find function `") {
        let after_start = &error_msg[start + 22..];
        if let Some(end) = after_start.find('`') {
            return Ok(after_start[..end].to_string());
        }
    }

    Ok("unknown".to_string())
}

/// Suggests similar identifiers for typo correction.
fn suggest_similar_identifiers(
    undefined_name: &str,
    _context: &ASTContext,
) -> Result<Vec<String>, CorrectionStrategyError> {
    let mut suggestions = Vec::new();

    // Common typos and corrections
    let common_corrections = [
        ("lenght", "length"),
        ("widht", "width"),
        ("heigth", "height"),
        ("colum", "column"),
        ("indx", "index"),
        ("cnt", "count"),
        ("str", "string"),
        ("num", "number"),
    ];

    for (typo, correction) in &common_corrections {
        if undefined_name.contains(typo) {
            suggestions.push(undefined_name.replace(typo, correction));
        }
    }

    // Add some common variable names if no specific suggestions
    if suggestions.is_empty() {
        suggestions.extend_from_slice(&[
            "value".to_string(),
            "result".to_string(),
            "data".to_string(),
            "item".to_string(),
        ]);
    }

    Ok(suggestions)
}

//--------------------------------------------------------------------------------------------------
// ExpectUsed: clippy::expect_used lint
//--------------------------------------------------------------------------------------------------

/// **Strategy for ExpectUsed: clippy::expect_used lint.**
///
/// Handles cases where `.expect()` is used on `Option` or `Result` types.
/// This strategy provides safer alternatives to `.expect()` calls by converting
/// them to proper error handling patterns.
///
/// # Examples
///
/// ```rust,ignore
/// // Before: value.expect("error message")
/// // After: value.unwrap_or_else(|| panic!("error message"))
/// // Or: if let Some(v) = value { v } else { panic!("error message") }
/// ```
#[derive(Debug)]
pub(super) struct ExpectUsed;

impl CorrectionStrategy for ExpectUsed {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::ExpectUsed
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;

        // Extract the expect call pattern
        if let Ok((variable_part, expect_message)) = extract_expect_call(original_code)? {
            // Strategy 1: Convert to if let pattern (safest)
            let if_let_replacement = format!(
                "if let Some(value) = {variable_part} {{ value }} else {{ panic!({expect_message}) }}"
            );
            proposals.push(CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: if_let_replacement,
                },
                0.95,
                SafetyLevel::Safe,
                Some("Convert .expect() to if let pattern for safer error handling".to_string()),
            ));

            // Strategy 2: Convert to unwrap_or_else (maintains panic but more explicit)
            let unwrap_or_else_replacement =
                format!("{variable_part}.unwrap_or_else(|| panic!({expect_message}))");
            proposals.push(CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: unwrap_or_else_replacement,
                },
                0.9,
                SafetyLevel::RequiresReview,
                Some(
                    "Convert .expect() to .unwrap_or_else() for explicit panic handling"
                        .to_string(),
                ),
            ));

            // Strategy 3: Convert to match pattern (most explicit)
            let match_replacement = format!(
                "match {variable_part} {{\n    Some(value) => value,\n    None => panic!({expect_message}),\n}}"
            );
            proposals.push(CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: match_replacement,
                },
                0.85,
                SafetyLevel::Safe,
                Some("Convert .expect() to explicit match pattern".to_string()),
            ));

            // Strategy 4: Convert to unwrap_or with default value (if applicable)
            if let Ok(default_value) = suggest_default_value_for_type(context)? {
                let unwrap_or_replacement = format!("{variable_part}.unwrap_or({default_value})");
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: unwrap_or_replacement,
                    },
                    0.8,
                    SafetyLevel::RequiresReview,
                    Some("Convert .expect() to .unwrap_or() with default value".to_string()),
                ));
            }
        }

        Ok(proposals)
    }
}

/// Extracts the variable part and expect message from an expect call.
fn extract_expect_call(code: &str) -> Result<(String, String), CorrectionStrategyError> {
    // Look for pattern: something.expect("message")
    if let Some(expect_pos) = code.find(".expect(") {
        let variable_part = code[..expect_pos].trim().to_string();

        // Find the message part
        let after_expect = &code[expect_pos + 8..]; // Skip ".expect("
        if let Some(closing_paren) = after_expect.rfind(')') {
            let message_part = after_expect[..closing_paren].trim().to_string();
            return Ok((variable_part, message_part));
        }
    }

    // Fallback for malformed expect calls
    Ok((
        code.to_string(),
        "\"Failed to extract expect message\"".to_string(),
    ))
}

/// Suggests a default value for the type based on context.
fn suggest_default_value_for_type(context: &ASTContext) -> Result<String, CorrectionStrategyError> {
    // Try to infer type from context or use common defaults
    let code = &context.primary_node.content;

    // Common type patterns and their defaults
    if code.contains("String") || code.contains("&str") {
        Ok("String::new()".to_string())
    } else if code.contains("Vec") {
        Ok("Vec::new()".to_string())
    } else if code.contains("i32") || code.contains("usize") || code.contains("u32") {
        Ok("0".to_string())
    } else if code.contains("bool") {
        Ok("false".to_string())
    } else if code.contains("Option") {
        Ok("None".to_string())
    } else {
        // Generic default - use Default trait
        Ok("Default::default()".to_string())
    }
}

//--------------------------------------------------------------------------------------------------
// Clippy: panic! should not be present in production code
//--------------------------------------------------------------------------------------------------

/// **Strategy for clippy::panic: panic! should not be present in production code.**
///
/// This strategy converts panic! calls to proper error handling using Result types.
/// Confidence: 95% - Very reliable transformation with clear error semantics.
#[derive(Debug)]
pub(super) struct ClippyPanicUsed;

impl CorrectionStrategy for ClippyPanicUsed {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::ClippyPanic
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;

        // Strategy 1: Convert panic! to return Err()
        if let Ok(error_return) = convert_panic_to_error_return(original_code)? {
            proposals.push(CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: error_return,
                },
                0.95,
                SafetyLevel::Safe,
                Some("Convert panic! to proper error return".to_string()),
            ));
        }

        // Strategy 2: Convert to unreachable!() if truly unreachable
        if is_unreachable_context(original_code) {
            if let Ok(unreachable_code) = convert_panic_to_unreachable(original_code)? {
                proposals.push(CorrectionProposal::new(
                    ProposalStrategy::ReplaceText {
                        original: original_code.clone(),
                        replacement: unreachable_code,
                    },
                    0.85,
                    SafetyLevel::RequiresReview,
                    Some("Convert to unreachable!() if this code path is impossible".to_string()),
                ));
            }
        }

        Ok(proposals)
    }
}

//--------------------------------------------------------------------------------------------------
// Clippy: indexing may panic
//--------------------------------------------------------------------------------------------------

/// **Strategy for clippy::indexing_slicing: indexing may panic.**
///
/// This strategy converts array/slice indexing to safe .get() calls.
/// Confidence: 90% - Reliable transformation but may need context adjustment.
#[derive(Debug)]
pub(super) struct ClippyIndexingSlicing;

impl CorrectionStrategy for ClippyIndexingSlicing {
    fn error_code(&self) -> ErrorCode {
        ErrorCode::ClippyIndexingSlicing
    }

    fn generate_proposals(&self, context: &ASTContext) -> Hatch<Vec<CorrectionProposal>> {
        let mut proposals = Vec::new();
        let original_code = &context.primary_node.content;

        // Strategy 1: Convert to .get() with unwrap_or_default()
        if let Ok(safe_get) = convert_indexing_to_safe_get(original_code)? {
            proposals.push(CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: safe_get,
                },
                0.90,
                SafetyLevel::Safe,
                Some("Convert indexing to safe .get() with default".to_string()),
            ));
        }

        // Strategy 2: Convert to .get() with expect()
        if let Ok(safe_get_expect) = convert_indexing_to_get_expect(original_code)? {
            proposals.push(CorrectionProposal::new(
                ProposalStrategy::ReplaceText {
                    original: original_code.clone(),
                    replacement: safe_get_expect,
                },
                0.85,
                SafetyLevel::RequiresReview,
                Some("Convert indexing to .get().expect() with bounds check".to_string()),
            ));
        }

        Ok(proposals)
    }
}

//--------------------------------------------------------------------------------------------------
// Helper Functions for New Strategies
//--------------------------------------------------------------------------------------------------

/// Converts panic! calls to proper error returns
fn convert_panic_to_error_return(code: &str) -> Result<String, CorrectionStrategyError> {
    // Look for panic! patterns
    if let Some(panic_start) = code.find("panic!(") {
        let before_panic = &code[..panic_start];
        let after_panic_start = &code[panic_start..];

        // Extract the panic message
        if let Some(closing_paren) = after_panic_start.find(')') {
            let panic_content = &after_panic_start[7..closing_paren]; // Skip "panic!("
            let after_panic = &after_panic_start[closing_paren + 1..];

            // Convert to return Err()
            let error_return = format!(
                "{}return Err(CorrectionStrategyError::AstAnalysisFailed {{ context: {}.to_string() }}){}",
                before_panic,
                panic_content,
                after_panic
            );

            return Ok(error_return);
        }
    }

    // Fallback - just replace panic! with return Err()
    Ok(code.replace("panic!", "return Err(CorrectionStrategyError::AstAnalysisFailed {{ context: \"panic occurred\".to_string() }})"))
}

/// Checks if the panic is in an unreachable context
fn is_unreachable_context(code: &str) -> bool {
    // Simple heuristics for unreachable contexts
    code.contains("else {") && code.contains("// Safe because")
        || code.contains("// This should never happen")
        || code.contains("// Unreachable")
        || code.contains("// SAFETY:")
}

/// Converts panic! to unreachable!()
fn convert_panic_to_unreachable(code: &str) -> Result<String, CorrectionStrategyError> {
    Ok(code.replace("panic!", "unreachable!"))
}

/// Converts array indexing to safe .get() calls
fn convert_indexing_to_safe_get(code: &str) -> Result<String, CorrectionStrategyError> {
    // Look for patterns like array[index] or slice[index]
    let re =
        Regex::new(r"(\w+)\[(\w+)\]").map_err(|e| CorrectionStrategyError::RegexCompilation {
            pattern: format!("Regex error: {}", e),
        })?;

    let result = re.replace_all(code, |caps: &regex::Captures| {
        let array = &caps[1];
        let index = &caps[2];
        format!("{}.get({}).unwrap_or(&Default::default())", array, index)
    });

    Ok(result.to_string())
}

/// Converts array indexing to .get().expect() calls
fn convert_indexing_to_get_expect(code: &str) -> Result<String, CorrectionStrategyError> {
    // Look for patterns like array[index] or slice[index]
    let re =
        Regex::new(r"(\w+)\[(\w+)\]").map_err(|e| CorrectionStrategyError::RegexCompilation {
            pattern: format!("Regex error: {}", e),
        })?;

    let result = re.replace_all(code, |caps: &regex::Captures| {
        let array = &caps[1];
        let index = &caps[2];
        format!(
            "{}.get({}).expect(\"Index should be within bounds\")",
            array, index
        )
    });

    Ok(result.to_string())
}
