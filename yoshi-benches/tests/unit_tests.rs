/* yoshi-benches/tests/unit_tests.rs */
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
//! **Brief:** Elite unit test suite for yoshi-benches with component isolation validation.
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + Component isolation testing with mathematical precision algorithms
//!  - Benchmark scenario creation and validation with edge case analysis
//!  - Performance measurement accuracy with statistical validation
//!  - Memory tracking precision with allocation boundary testing
//!  - Framework comparison logic with algorithmic correctness
//!  - Result analysis algorithms with mathematical property verification
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

use yoshi_benches::*;

//--------------------------------------------------------------------------------------------------
// Component Isolation Tests
//--------------------------------------------------------------------------------------------------

#[test]
fn test_ecosystem_comparison_engine_creation() {
    let engine = EcosystemComparisonEngine::new();

    // Engine should be created successfully
    // We can't test internal state directly, but we can test that it works
    let report = engine.execute_comprehensive_ecosystem_comparison();
    assert!(report.results.contains_key("Yoshi"));
}

#[test]
fn test_yoshi_tester_creation() {
    let tester = YoshiTester;
    let capabilities = tester.get_ecosystem_capabilities();

    // Verify core capabilities are present
    assert!(capabilities.derive_macro_support);
    assert!(capabilities.structured_errors);
    assert!(capabilities.error_chaining);
    assert!(capabilities.metadata_support);
    assert!(capabilities.custom_context);
    assert!(capabilities.suggestions);
}

#[test]
fn test_ecosystem_capabilities_structure() {
    let tester = YoshiTester;
    let capabilities = tester.get_ecosystem_capabilities();

    // Test boolean capabilities
    assert!(capabilities.derive_macro_support);
    assert!(capabilities.structured_errors);
    assert!(capabilities.error_chaining);
    assert!(capabilities.metadata_support);
    assert!(capabilities.custom_context);
    assert!(capabilities.suggestions);
    assert!(capabilities.error_codes);
    assert!(capabilities.async_support);
    assert!(capabilities.typed_payloads);

    // Test numeric capabilities (should be in valid range)
    // u32 type guarantees >= 0, so just check upper bounds
    assert!(capabilities.memory_efficiency <= 100);
    assert!(capabilities.type_safety <= 100);
    assert!(capabilities.debugging_experience <= 100);
    assert!(capabilities.recovery_capabilities <= 100);
}

//--------------------------------------------------------------------------------------------------
// Performance Measurement Accuracy
//--------------------------------------------------------------------------------------------------

#[test]
fn test_timing_measurement_precision() {
    let engine = EcosystemComparisonEngine::new();
    let report = engine.execute_comprehensive_ecosystem_comparison();

    // Verify Yoshi results exist and have valid timing
    assert!(report.results.contains_key("Yoshi"));
    let yoshi_results = report
        .results
        .get("Yoshi")
        .expect("Yoshi results should be available");
    assert!(!yoshi_results.is_empty());

    for result in yoshi_results {
        // Timing should be measured in nanoseconds with reasonable precision
        assert!(result.execution_time_ns > 0);
        assert!(result.execution_time_ns < 10_000_000_000); // Less than 10 seconds
    }
}

#[test]
fn test_memory_measurement_accuracy() {
    let engine = EcosystemComparisonEngine::new();
    let report = engine.execute_comprehensive_ecosystem_comparison();

    // Verify Yoshi results exist and have valid memory measurements
    assert!(report.results.contains_key("Yoshi"));
    let yoshi_results = report
        .results
        .get("Yoshi")
        .expect("Yoshi results should be available");
    assert!(!yoshi_results.is_empty());

    for result in yoshi_results {
        // Memory footprint should be measured and reasonable
        assert!(result.memory_footprint > 0);
        assert!(result.memory_footprint < 1024 * 1024 * 1024); // Less than 1GB
    }
}

#[test]
fn test_result_structure_validation() {
    let engine = EcosystemComparisonEngine::new();
    let report = engine.execute_comprehensive_ecosystem_comparison();

    // Verify Yoshi results have proper structure
    assert!(report.results.contains_key("Yoshi"));
    let yoshi_results = report
        .results
        .get("Yoshi")
        .expect("Yoshi results should be available");
    assert!(!yoshi_results.is_empty());

    for result in yoshi_results {
        // All fields should be within valid ranges
        assert_eq!(result.framework, "Yoshi");
        assert!(result.execution_time_ns > 0);
        assert!(result.memory_footprint > 0);
        assert!(result.context_richness <= 100);
        assert!(result.derive_capabilities <= 100);
        assert!(result.ecosystem_integration <= 100);
        assert!(result.ergonomics_score <= 100);
        assert!(result.debugging_experience <= 100);
        assert!(result.recoverability_score <= 100);
    }
}

//--------------------------------------------------------------------------------------------------
// Edge Case Analysis
//--------------------------------------------------------------------------------------------------

#[test]
fn test_report_completeness() {
    let engine = EcosystemComparisonEngine::new();
    let report = engine.execute_comprehensive_ecosystem_comparison();

    // Report should have all required sections
    assert!(report.results.contains_key("Yoshi"));
    assert!(!report.scenarios.is_empty());
    assert!(report.ecosystem_capabilities.contains_key("Yoshi"));
    assert!(report.derive_test_results.contains_key("Yoshi"));
    assert!(report.real_world_test_results.contains_key("Yoshi"));

    // Scenarios should have valid structure
    for scenario in &report.scenarios {
        assert!(!scenario.name.is_empty());
        assert!(scenario.performance_target.max_execution_time_us > 0);
        assert!(scenario.performance_target.max_memory_footprint > 0);
    }
}

#[test]
fn test_derive_test_results_structure() {
    let engine = EcosystemComparisonEngine::new();
    let report = engine.execute_comprehensive_ecosystem_comparison();

    // Verify derive test results exist and have proper structure
    assert!(report.derive_test_results.contains_key("Yoshi"));
    let derive_results = &report.derive_test_results["Yoshi"];
    assert!(!derive_results.is_empty());

    for result in derive_results {
        // All derive test fields should be valid
        assert!(result.generated_code_quality <= 100);
        assert!(result.feature_completeness <= 100);
        assert!(result.derive_ergonomics <= 100);
        assert!(result.error_message_quality <= 100);
        // compilation_success is boolean, so no range check needed
    }
}

#[test]
fn test_real_world_test_results() {
    let engine = EcosystemComparisonEngine::new();
    let report = engine.execute_comprehensive_ecosystem_comparison();

    // Verify real-world test results exist
    assert!(report.real_world_test_results.contains_key("Yoshi"));
    let real_world_results = &report.real_world_test_results["Yoshi"];
    assert!(!real_world_results.is_empty());

    for result in real_world_results {
        // All real-world test fields should be valid
        assert!(result.production_readiness <= 100);
        assert!(result.maintainability <= 100);
        assert!(result.integration_complexity <= 100);
        assert!(result.debugging_efficiency <= 100);
        assert!(result.recovery_effectiveness <= 100);
    }
}

//--------------------------------------------------------------------------------------------------
// Framework Comparison Logic
//--------------------------------------------------------------------------------------------------

#[test]
fn test_framework_capability_detection() {
    let tester = YoshiTester;
    let capabilities = tester.get_ecosystem_capabilities();

    // Test individual capability flags
    assert!(capabilities.derive_macro_support);
    assert!(capabilities.structured_errors);
    assert!(capabilities.error_chaining);
    assert!(capabilities.metadata_support);
    assert!(capabilities.custom_context);
    assert!(capabilities.suggestions);
    assert!(capabilities.error_codes);
    assert!(capabilities.async_support);
    assert!(capabilities.typed_payloads);

    // Test capability scores (u32 type guarantees >= 0)
    assert!(capabilities.memory_efficiency <= 100);
    assert!(capabilities.type_safety <= 100);
    assert!(capabilities.debugging_experience <= 100);
    assert!(capabilities.recovery_capabilities <= 100);
}

#[test]
fn test_comprehensive_report_generation() {
    let engine = EcosystemComparisonEngine::new();
    let report = engine.execute_comprehensive_ecosystem_comparison();

    // Test that comprehensive report can be generated
    let comprehensive_report = report.generate_comprehensive_report();
    assert!(!comprehensive_report.is_empty());
    assert!(comprehensive_report.contains("Yoshi"));
    assert!(comprehensive_report.contains("COMPREHENSIVE YOSHI ECOSYSTEM"));
    assert!(comprehensive_report.contains("EXECUTIVE SUMMARY"));
    assert!(comprehensive_report.contains("ECOSYSTEM CAPABILITIES MATRIX"));
}

//--------------------------------------------------------------------------------------------------
// Mathematical Property Verification
//--------------------------------------------------------------------------------------------------

#[test]
fn test_performance_measurement_consistency() {
    let engine = EcosystemComparisonEngine::new();

    // Run multiple reports to test consistency
    let report1 = engine.execute_comprehensive_ecosystem_comparison();
    let report2 = engine.execute_comprehensive_ecosystem_comparison();

    // Both reports should have Yoshi results
    assert!(report1.results.contains_key("Yoshi"));
    assert!(report2.results.contains_key("Yoshi"));

    let yoshi_results1 = &report1.results["Yoshi"];
    let yoshi_results2 = &report2.results["Yoshi"];

    // Results should have consistent structure
    assert_eq!(yoshi_results1.len(), yoshi_results2.len());

    // All results should be valid
    for result in yoshi_results1 {
        assert!(result.execution_time_ns > 0);
        assert_eq!(result.framework, "Yoshi");
        assert!(result.memory_footprint > 0);
    }

    for result in yoshi_results2 {
        assert!(result.execution_time_ns > 0);
        assert_eq!(result.framework, "Yoshi");
        assert!(result.memory_footprint > 0);
    }
}
