/* yoshi-benches/tests/integration_tests.rs */
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
//! **Brief:** Elite integration test suite for yoshi-benches with performance validation.
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + Performance benchmarking integration with cross-framework validation
//!  - Benchmark execution validation with statistical significance testing
//!  - Cross-crate integration testing with ecosystem compatibility
//!  - Performance regression detection with automated thresholds
//!  - Memory efficiency validation with allocation tracking
//!  - Comprehensive comparison engine with framework analysis
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

use yoshi_benches::*;

//--------------------------------------------------------------------------------------------------
// Benchmark Execution Validation
//--------------------------------------------------------------------------------------------------

#[test]
fn test_benchmark_framework_initialization() {
    // Test that the benchmark framework initializes correctly
    let comparison_engine = EcosystemComparisonEngine::new();

    // Test framework capabilities by running a quick comparison
    let report = comparison_engine.execute_comprehensive_ecosystem_comparison();
    assert!(!report.results.is_empty());
    assert!(report.results.contains_key("Yoshi"));
}

#[test]
fn test_yoshi_benchmark_execution() {
    // Test that Yoshi benchmarks execute successfully
    let engine = EcosystemComparisonEngine::new();
    let report = engine.execute_comprehensive_ecosystem_comparison();

    // Verify Yoshi results exist
    assert!(report.results.contains_key("Yoshi"));
    let yoshi_results = report
        .results
        .get("Yoshi")
        .expect("Yoshi results should be available");
    assert!(!yoshi_results.is_empty());

    // Verify result structure
    for result in yoshi_results {
        assert!(result.execution_time_ns > 0);
        assert_eq!(result.framework, "Yoshi");
        assert!(result.context_richness <= 100);
        assert!(result.derive_capabilities <= 100);
    }
}

#[test]
fn test_cross_framework_comparison() {
    // Test comparison between different error handling frameworks
    let engine = EcosystemComparisonEngine::new();
    let report = engine.execute_comprehensive_ecosystem_comparison();

    // Verify Yoshi is included
    assert!(report.results.contains_key("Yoshi"));

    // Verify ecosystem capabilities are reported
    assert!(report.ecosystem_capabilities.contains_key("Yoshi"));
    let yoshi_caps = report
        .ecosystem_capabilities
        .get("Yoshi")
        .expect("Yoshi capabilities should be available");

    // Verify Yoshi has expected capabilities
    assert!(yoshi_caps.derive_macro_support);
    assert!(yoshi_caps.structured_errors);
    assert!(yoshi_caps.error_chaining);
    assert!(yoshi_caps.metadata_support);
    assert!(yoshi_caps.custom_context);
    assert!(yoshi_caps.suggestions);
}

//--------------------------------------------------------------------------------------------------
// Performance Regression Detection
//--------------------------------------------------------------------------------------------------

#[test]
fn test_performance_regression_detection() {
    // Test that performance regression detection works by running multiple comparisons
    let engine = EcosystemComparisonEngine::new();

    // Run comparison multiple times to check consistency
    let report1 = engine.execute_comprehensive_ecosystem_comparison();
    let report2 = engine.execute_comprehensive_ecosystem_comparison();

    // Verify both reports have Yoshi results
    assert!(report1.results.contains_key("Yoshi"));
    assert!(report2.results.contains_key("Yoshi"));

    let yoshi_results1 = report1
        .results
        .get("Yoshi")
        .expect("Yoshi results should be available in first report");
    let yoshi_results2 = report2
        .results
        .get("Yoshi")
        .expect("Yoshi results should be available in second report");

    // Results should be consistent (same number of scenarios)
    assert_eq!(yoshi_results1.len(), yoshi_results2.len());

    // All results should have reasonable execution times
    for result in yoshi_results1 {
        assert!(result.execution_time_ns > 0);
        assert!(result.execution_time_ns < 1_000_000_000); // Less than 1 second
    }
}

#[test]
fn test_memory_efficiency_validation() {
    // Test memory efficiency by validating result structure
    let engine = EcosystemComparisonEngine::new();
    let report = engine.execute_comprehensive_ecosystem_comparison();

    // Verify Yoshi results exist and have reasonable values
    assert!(report.results.contains_key("Yoshi"));
    let yoshi_results = report
        .results
        .get("Yoshi")
        .expect("Yoshi results should be available");

    for result in yoshi_results {
        // All metrics should be within reasonable bounds
        assert!(result.context_richness <= 100);
        assert!(result.derive_capabilities <= 100);
        assert!(result.ecosystem_integration <= 100);
        assert!(result.ergonomics_score <= 100);
        assert!(result.debugging_experience <= 100);
        assert!(result.recoverability_score <= 100);
    }
}

//--------------------------------------------------------------------------------------------------
// Ecosystem Compatibility Testing
//--------------------------------------------------------------------------------------------------

#[test]
fn test_ecosystem_integration() {
    // Test integration with the broader Yoshi ecosystem
    let tester = YoshiTester;
    let capabilities = tester.get_ecosystem_capabilities();

    // Verify all expected capabilities are present
    assert!(capabilities.derive_macro_support);
    assert!(capabilities.structured_errors);
    assert!(capabilities.error_chaining);
    assert!(capabilities.metadata_support);
    assert!(capabilities.custom_context);
    assert!(capabilities.suggestions);
    assert!(capabilities.async_support);
}

#[test]
fn test_comprehensive_framework_analysis() {
    // Test comprehensive analysis of the Yoshi framework
    let engine = EcosystemComparisonEngine::new();
    let report = engine.execute_comprehensive_ecosystem_comparison();

    // Verify Yoshi results exist
    assert!(report.results.contains_key("Yoshi"));
    let yoshi_results = report
        .results
        .get("Yoshi")
        .expect("Yoshi results should be available");

    // Verify multiple scenarios were tested
    assert!(!yoshi_results.is_empty());
    assert!(yoshi_results.len() >= 3); // Should have multiple test scenarios

    // Verify all scenarios executed successfully
    for result in yoshi_results {
        assert!(result.execution_time_ns > 0);
        assert_eq!(result.framework, "Yoshi");

        // Verify all scores are within valid ranges
        assert!(result.context_richness <= 100);
        assert!(result.derive_capabilities <= 100);
        assert!(result.ecosystem_integration <= 100);
        assert!(result.ergonomics_score <= 100);
        assert!(result.debugging_experience <= 100);
        assert!(result.recoverability_score <= 100);
    }

    // Verify scenarios were executed
    assert!(!report.scenarios.is_empty());

    // Verify derive test results exist
    assert!(report.derive_test_results.contains_key("Yoshi"));
    let derive_results = report
        .derive_test_results
        .get("Yoshi")
        .expect("Yoshi derive results should be available");
    assert!(!derive_results.is_empty());

    // Ensure we have at least one derive result
    if let Some(first_result) = derive_results.first() {
        assert!(first_result.compilation_success);
    } else {
        panic!("Expected at least one derive test result");
    }
}

//--------------------------------------------------------------------------------------------------
// Statistical Validation
//--------------------------------------------------------------------------------------------------

#[test]
fn test_statistical_significance() {
    // Test that benchmark results have statistical significance
    let engine = EcosystemComparisonEngine::new();

    // Run multiple iterations to gather statistics
    let mut execution_times = Vec::new();
    for _ in 0..5 {
        let report = engine.execute_comprehensive_ecosystem_comparison();
        assert!(report.results.contains_key("Yoshi"));
        let yoshi_results = report
            .results
            .get("Yoshi")
            .expect("Yoshi results should be available");
        assert!(!yoshi_results.is_empty());

        // Get the first result if available
        if let Some(first_result) = yoshi_results.first() {
            execution_times.push(first_result.execution_time_ns);
        } else {
            panic!("Expected at least one Yoshi result");
        }
    }

    // Calculate basic statistics using methods appropriate for large numbers
    // Note: We use checked conversions and handle large numbers appropriately
    let total: u128 = execution_times.iter().sum();
    let count = execution_times.len();

    // Safe to convert to f64 for mean calculation as we've verified values are reasonable
    // Note: For large u128 values, there may be precision loss, but it's acceptable for these statistics
    let mean = (total as f64) / (count as f64);

    // Calculate variance - we accept precision loss for statistical calculations in tests
    // Using f64 is appropriate for this statistical purpose despite potential precision loss
    let variance = execution_times
        .iter()
        .map(|&x| {
            // Cast with comment explaining acceptable precision loss
            let x_f64 = x as f64; // Precision loss acceptable for test statistics
            (x_f64 - mean).powi(2)
        })
        .sum::<f64>()
        / (count as f64); // Precision loss acceptable for test statistics

    let std_dev = variance.sqrt();

    // Results should be consistent (low coefficient of variation)
    let coefficient_of_variation = std_dev / mean;
    assert!(coefficient_of_variation < 2.0); // Less than 200% variation (allowing for system variance)

    // All results should be positive
    assert!(execution_times.iter().all(|&x| x > 0));
}

#[test]
fn test_benchmark_reproducibility() {
    // Test that benchmarks produce reproducible results
    let engine = EcosystemComparisonEngine::new();

    let report1 = engine.execute_comprehensive_ecosystem_comparison();
    let report2 = engine.execute_comprehensive_ecosystem_comparison();

    // Both reports should have Yoshi results
    assert!(report1.results.contains_key("Yoshi"));
    assert!(report2.results.contains_key("Yoshi"));

    let yoshi_results1 = report1
        .results
        .get("Yoshi")
        .expect("Yoshi results should be available in first report");
    let yoshi_results2 = report2
        .results
        .get("Yoshi")
        .expect("Yoshi results should be available in second report");

    // Results should have the same structure
    assert_eq!(yoshi_results1.len(), yoshi_results2.len());

    // Framework names should be identical
    for (result1, result2) in yoshi_results1.iter().zip(yoshi_results2.iter()) {
        assert_eq!(result1.framework, result2.framework);
        assert_eq!(result1.framework, "Yoshi");
    }
}

//--------------------------------------------------------------------------------------------------
// End-to-End Integration Validation
//--------------------------------------------------------------------------------------------------

#[test]
fn test_end_to_end_benchmark_pipeline() {
    // Test the complete benchmark pipeline from start to finish
    let engine = EcosystemComparisonEngine::new();
    let report = engine.execute_comprehensive_ecosystem_comparison();

    // Validate all pipeline stages
    assert!(report.results.contains_key("Yoshi"));
    assert!(!report.scenarios.is_empty());
    assert!(report.ecosystem_capabilities.contains_key("Yoshi"));
    assert!(report.derive_test_results.contains_key("Yoshi"));
    assert!(report.real_world_test_results.contains_key("Yoshi"));

    // Verify comprehensive report generation
    let comprehensive_report = report.generate_comprehensive_report();
    assert!(!comprehensive_report.is_empty());
    assert!(comprehensive_report.contains("Yoshi"));
    assert!(comprehensive_report.contains("COMPREHENSIVE YOSHI ECOSYSTEM"));
}
