/* yoshi-benches/src/lib.rs */
#![deny(unsafe_code)]
#![deny(clippy::todo)]
#![deny(clippy::expect_used)]
#![deny(clippy::unwrap_used)]
// ===== COMPREHENSIVE BENCHMARK ALLOWANCES =====
// Benchmarks are performance-focused and may use patterns that would be inappropriate in production code
// These allowances ensure the benchmark suite can focus on measuring performance without style constraints

// Core functionality allowances
#![allow(unused_variables)] // Benchmarks may have demo variables
#![allow(dead_code)] // Benchmarks may have demo code
#![allow(missing_docs)] // Benchmarks don't need comprehensive docs
#![allow(clippy::pedantic)] // Benchmarks focus on functionality over style

// Output and debugging allowances
#![allow(clippy::dbg_macro)] // Benchmarks may use debug output
#![allow(clippy::print_stdout)] // Benchmarks may print results
#![allow(clippy::print_stderr)]
// Benchmarks may print to stderr

// Safety and error handling allowances (for demonstration purposes)
#![allow(clippy::indexing_slicing)] // Benchmarks may use direct indexing
#![allow(clippy::panic)] // Benchmarks may panic for demonstration
#![allow(clippy::unimplemented)] // Benchmarks may have placeholder code
#![allow(clippy::unreachable)] // Benchmarks may have unreachable demo code

// Documentation allowances
#![allow(clippy::missing_docs_in_private_items)] // Benchmarks don't need private docs
#![allow(clippy::missing_errors_doc)] // Benchmarks don't need error docs
#![allow(clippy::missing_panics_doc)] // Benchmarks don't need panic docs
#![allow(clippy::missing_safety_doc)] // Benchmarks don't need safety docs

// Performance and complexity allowances
#![allow(clippy::too_many_lines)] // Benchmark functions may be long
#![allow(clippy::cognitive_complexity)] // Benchmarks may be complex
#![allow(clippy::cognitive_complexity)] // Benchmarks may have complex control flow
#![allow(clippy::cast_precision_loss)] // Benchmarks may need precision trade-offs for performance
#![allow(clippy::cast_possible_truncation)] // Benchmarks may truncate for performance
#![allow(clippy::cast_lossless)] // Benchmarks may use explicit casts for clarity
#![allow(clippy::cast_possible_wrap)] // Benchmarks may wrap for performance
#![allow(clippy::cast_sign_loss)] // Benchmarks may lose sign for performance

// Style and formatting allowances
#![allow(clippy::module_name_repetitions)] // Benchmark modules may repeat names
#![allow(clippy::similar_names)] // Benchmarks may have similar variable names
#![allow(clippy::redundant_else)] // Benchmarks may have redundant else for clarity
#![allow(clippy::uninlined_format_args)] // Benchmarks may use older format patterns
#![allow(clippy::empty_line_after_doc_comments)] // Benchmarks may have formatting variations

// Collection and iteration allowances
#![allow(clippy::needless_pass_by_value)] // Benchmarks may pass by value for performance
#![allow(clippy::unnecessary_wraps)] // Benchmarks may wrap for consistency
#![allow(clippy::explicit_iter_loop)] // Benchmarks may use explicit iteration
#![allow(clippy::manual_let_else)] // Benchmarks may use older patterns
#![allow(clippy::single_char_pattern)] // Benchmarks may use single char patterns

// Comparison and logic allowances
#![allow(clippy::trivially_copy_pass_by_ref)] // Benchmarks may pass small types by ref
#![allow(clippy::unused_self)] // Benchmark methods may not use self
#![allow(clippy::redundant_pattern_matching)] // Benchmarks may use explicit patterns
#![allow(clippy::unnecessary_map_or)] // Benchmarks may use explicit map_or
#![allow(clippy::manual_clamp)] // Benchmarks may use manual clamping
#![allow(clippy::derivable_impls)] // Benchmarks may have explicit impls
#![allow(clippy::used_underscore_binding)] // Benchmarks may use underscore bindings

// Literal and numeric allowances
#![allow(clippy::unreadable_literal)] // Benchmarks may have large numeric literals
#![allow(clippy::ref_option_ref)] // Benchmarks may use ref option ref patterns
#![allow(clippy::redundant_closure_for_method_calls)] // Benchmarks may use explicit closures

// Documentation formatting allowances
#![allow(rustdoc::missing_crate_level_docs)] // Benchmarks may not need crate docs
#![allow(rustdoc::broken_intra_doc_links)] // Benchmarks may have broken links during development
//! **Brief:** Comprehensive benchmarking and analysis suite for Yoshi error handling framework.
//!
//! This crate provides comprehensive benchmarking capabilities, framework comparisons,
//! and analysis tools for evaluating error handling frameworks in the Rust ecosystem.
//!
//! ## Key Features
//!
//! - **Multi-Framework Comparison**: Comprehensive analysis of Yoshi vs competitors
//! - **Performance Benchmarking**: Execution time and memory usage analysis
//! - **Developer Experience Metrics**: Ergonomics and usability evaluation
//! - **Production Readiness Assessment**: Real-world scenario validation
//! - **Advanced Reporting**: Text, HTML, and interactive report generation
//!
//! ## Usage
//!
//! ```rust,no_run
//! use yoshi_benches::EcosystemComparisonEngine;
//! let engine = EcosystemComparisonEngine::new();
//! let report = engine.execute_comprehensive_ecosystem_comparison();
//! println!("{}", report.generate_comprehensive_report());
//! ```
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Comprehensive Error Framework Analysis Suite]
//!  - [Multi-dimensional Comparison Engine: Feature, performance, ergonomics analysis]
//!  - [Advanced Benchmarking Framework: Statistical validation with Criterion integration]
//!  - [Developer Experience Assessment: Code complexity and maintainability metrics]
//!  - [Production Readiness Validation: Real-world scenario testing and analysis]
//!  - [Strategic Decision Support: Framework selection guidance with empirical evidence]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **License File:** /LICENSE
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

pub mod comprehensive_comparison;

// Re-export key types for easy access
pub use comprehensive_comparison::*;

// Convenience re-exports for common usage patterns
pub use comprehensive_comparison::{
    BusinessContext, EcosystemCapabilities, EcosystemComparisonEngine, EcosystemComparisonReport,
    EcosystemFrameworkTester, EcosystemTestScenario, PerformanceTarget, TestComplexity,
    YoshiTester,
};

/// Current version of the yoshi-benches crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate description
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// Quick start function for running a standard comparison
///
/// This function provides a convenient way to run a comprehensive comparison
/// with default settings, suitable for most evaluation scenarios.
///
/// # Returns
///
/// A comprehensive comparison report with analysis across all frameworks
///
/// # Examples
///
/// ```rust,no_run
/// use yoshi_benches::quick_comparison;
///
/// let report = quick_comparison();
/// println!("Framework comparison complete!");
/// println!("{}", report.generate_comprehensive_report());
/// ```
#[must_use]
pub fn quick_comparison() -> EcosystemComparisonReport {
    let engine = EcosystemComparisonEngine::new();
    engine.execute_comprehensive_ecosystem_comparison()
}

/// Validate framework comparison results for data integrity
///
/// This function performs data-driven validation of comparison results,
/// checking that the dynamic scoring system produces realistic and consistent results
/// across all frameworks without predetermined bias.
///
/// # Returns
///
/// `true` if the comparison results are consistent and realistic, `false` otherwise
#[must_use]
pub fn validate_comparison_integrity() -> bool {
    let report = quick_comparison();

    // Ensure Yoshi was tested
    if !report.results.contains_key("Yoshi") {
        return false;
    }

    // Ensure comparison frameworks were tested only when feature is enabled
    #[cfg(feature = "comparison")]
    {
        let required_frameworks = ["thiserror", "anyhow", "eyre", "snafu"];
        for framework in &required_frameworks {
            if !report.results.contains_key(*framework) {
                return false;
            }
        }
    }

    // Validate that results are within reasonable ranges (0-100)
    for results in report.results.values() {
        for result in results {
            if result.context_richness > 100
                || result.ergonomics_score > 100
                || result.derive_capabilities > 100
                || result.debugging_experience > 100
                || result.ecosystem_integration > 100
                || result.recoverability_score > 100
            {
                return false;
            }
        }
    }

    // Validate that derive-based frameworks score higher in derive capabilities
    // Only compare when comparison frameworks are available
    #[cfg(feature = "comparison")]
    {
        let derive_frameworks = ["Yoshi", "thiserror", "snafu"];
        let non_derive_frameworks = ["anyhow", "eyre"];

        let derive_frameworks_count = u32::try_from(derive_frameworks.len()).unwrap_or(1);
        let avg_derive_with_support = derive_frameworks
            .iter()
            .filter_map(|name| report.results.get(*name))
            .flat_map(|results| results.iter())
            .map(|r| f64::from(r.derive_capabilities))
            .sum::<f64>()
            / f64::from(derive_frameworks_count * 4); // 4 scenarios per framework

        let non_derive_frameworks_count = u32::try_from(non_derive_frameworks.len()).unwrap_or(1);
        let avg_derive_without_support = non_derive_frameworks
            .iter()
            .filter_map(|name| report.results.get(*name))
            .flat_map(|results| results.iter())
            .map(|r| f64::from(r.derive_capabilities))
            .sum::<f64>()
            / f64::from(non_derive_frameworks_count * 4); // 4 scenarios per framework

        // Frameworks with derive support should score higher in derive capabilities
        avg_derive_with_support > avg_derive_without_support
    }

    #[cfg(not(feature = "comparison"))]
    {
        // In no_std mode without comparison, just validate that results exist and are reasonable
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_comparison() {
        let report = quick_comparison();

        // Verify Yoshi is always tested
        assert!(report.results.contains_key("Yoshi"));

        // Verify comparison frameworks are tested only when feature is enabled
        #[cfg(feature = "comparison")]
        {
            assert!(report.results.contains_key("thiserror"));
            assert!(report.results.contains_key("anyhow"));
            assert!(report.results.contains_key("eyre"));
            assert!(report.results.contains_key("snafu"));
        }

        // Verify scenarios were executed
        assert!(!report.scenarios.is_empty());

        // Verify all results are within valid ranges (0-100)
        for results in report.results.values() {
            for result in results {
                assert!(result.context_richness <= 100);
                assert!(result.derive_capabilities <= 100);
                assert!(result.ecosystem_integration <= 100);
                assert!(result.ergonomics_score <= 100);
                assert!(result.debugging_experience <= 100);
                assert!(result.recoverability_score <= 100);
            }
        }
    }

    #[test]
    fn test_comparison_integrity_validation() {
        let integrity_valid = validate_comparison_integrity();
        assert!(
            integrity_valid,
            "Comparison results should be consistent and realistic!"
        );
    }

    #[test]
    fn test_framework_capabilities_consistency() {
        let engine = EcosystemComparisonEngine::new();
        let report = engine.execute_comprehensive_ecosystem_comparison();

        // Validate that Yoshi has capabilities reported
        assert!(
            report.ecosystem_capabilities.contains_key("Yoshi"),
            "Framework Yoshi should have capabilities reported"
        );

        // Validate comparison frameworks only when feature is enabled
        #[cfg(feature = "comparison")]
        {
            let required_frameworks = ["thiserror", "anyhow", "eyre", "snafu"];
            for framework in &required_frameworks {
                assert!(
                    report.ecosystem_capabilities.contains_key(*framework),
                    "Framework {framework} should have capabilities reported"
                );
            }
        }

        // Validate derive-based frameworks report derive support correctly
        #[cfg(feature = "comparison")]
        let derive_frameworks = ["Yoshi", "thiserror", "snafu"];
        #[cfg(not(feature = "comparison"))]
        let derive_frameworks = ["Yoshi"];

        for framework in &derive_frameworks {
            if let Some(caps) = report.ecosystem_capabilities.get(*framework) {
                assert!(
                    caps.derive_macro_support,
                    "Framework {framework} should support derive macros"
                );
            }
        }

        // Validate non-derive frameworks report correctly (only when comparison feature enabled)
        #[cfg(feature = "comparison")]
        {
            let non_derive_frameworks = ["anyhow", "eyre"];
            for framework in &non_derive_frameworks {
                if let Some(caps) = report.ecosystem_capabilities.get(*framework) {
                    assert!(
                        !caps.derive_macro_support,
                        "Framework {framework} should not support derive macros"
                    );
                }
            }
        }

        // Validate all quality scores are in valid ranges (0-100)
        for caps in report.ecosystem_capabilities.values() {
            assert!(caps.memory_efficiency <= 100);
            assert!(caps.type_safety <= 100);
            assert!(caps.debugging_experience <= 100);
            assert!(caps.recovery_capabilities <= 100);
        }
    }
}
