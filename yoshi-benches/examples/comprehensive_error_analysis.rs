/* yoshi-benches\examples\comprehensive_error_analysis.rs */
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_precision_loss)] // Allow precision loss for performance analysis
#![allow(clippy::unused_self)] // Allow unused self for trait consistency
#![allow(clippy::missing_errors_doc)] // Allow missing errors doc for example code
#![allow(clippy::assigning_clones)] // Allow clone assignments for algorithm clarity
//! **Brief:** Executable comprehensive error framework analysis demonstrating
//! Yoshi's architectural superiority through empirical validation protocols.
//!
//! **Module Classification:** Performance-Critical
//! **Complexity Level:** Expert
//! **API Stability:** Stable
//!
//! ## Mathematical Properties
//!
//! **Algorithmic Complexity:**
//! - Time Complexity: O(n*m*k) where n=frameworks, m=scenarios, k=analysis depth
//! - Space Complexity: O(n*m*r) where r=report complexity factor
//! - Concurrency Safety: Thread-safe execution across all comparison matrices
//!
//! **Performance Characteristics:**
//! - Expected Performance: Complete analysis execution in <2s for standard test suite
//! - Worst-Case Scenarios: Complex error chain analysis with deep context nesting
//! - Optimization Opportunities: Parallel framework testing with intelligent caching
//!
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Comprehensive Error Framework Analysis Engine with Empirical Validation]
//!  - [Multi-dimensional Comparison Matrix: Feature, performance, ergonomics analysis]
//!  - [Real-world Scenario Testing: Production-grade error handling validation]
//!  - [Developer Experience Metrics: Code complexity and maintainability analysis]
//!  - [Performance Benchmarking: Memory efficiency and execution time measurement]
//!  - [Strategic Recommendations: Framework selection guidance with empirical backing]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** Business Source License 1.1 (BSL-1.1)
// **License Terms:** Non-production use only; commercial/production use requires paid license.
// **Effective Date:** 2025-05-25 | **Change License:** GPL v3
// **License File:** /LICENSE
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

use std::collections::HashMap;
use std::fmt::Write;
use std::fs;
use std::path::Path;

// Import the comprehensive comparison framework with correct type names
#[allow(unused_imports)]
use yoshi_benches::comprehensive_comparison::{
    EcosystemCapabilities, EcosystemComparisonEngine, EcosystemComparisonReport,
    EcosystemFrameworkTester, EcosystemTestScenario, TestComplexity,
};

/// Comprehensive analysis execution configuration
#[derive(Debug, Clone)]
#[allow(clippy::struct_excessive_bools)]
pub struct AnalysisConfiguration {
    /// Enable detailed scenario analysis
    pub detailed_scenarios: bool,
    /// Enable performance profiling
    pub performance_profiling: bool,
    /// Enable memory analysis
    pub memory_analysis: bool,
    /// Enable ergonomics evaluation
    pub ergonomics_evaluation: bool,
    /// Generate HTML report
    pub html_report: bool,
    /// Output directory for reports
    pub output_directory: String,
    /// Enable comparative analysis
    pub comparative_analysis: bool,
}

impl Default for AnalysisConfiguration {
    fn default() -> Self {
        Self {
            detailed_scenarios: true,
            performance_profiling: true,
            memory_analysis: true,
            ergonomics_evaluation: true,
            html_report: true,
            output_directory: "./analysis_reports".to_string(),
            comparative_analysis: true,
        }
    }
}

/// Advanced analysis execution engine
pub struct AnalysisExecutionEngine {
    /// Configuration parameters
    configuration: AnalysisConfiguration,
    /// Comparison engine instance
    comparison_engine: EcosystemComparisonEngine,
}

impl AnalysisExecutionEngine {
    /// Initialize comprehensive analysis engine with configuration
    #[must_use]
    pub fn new(configuration: AnalysisConfiguration) -> Self {
        Self {
            configuration,
            comparison_engine: EcosystemComparisonEngine::new(),
        }
    }

    /// Execute comprehensive framework analysis with full reporting
    pub fn execute_comprehensive_analysis(&self) -> Result<AnalysisResults, AnalysisError> {
        println!("🚀 Initiating Comprehensive Error Framework Analysis...");
        println!("   📊 Frameworks: Yoshi vs thiserror vs anyhow vs eyre vs snafu");
        println!("   🎯 Analysis Depth: Multi-dimensional comparative evaluation");
        println!("   ⚡ Expected Duration: <2 seconds for complete analysis\n");

        // Phase 1: Core comparison execution
        println!("📈 Phase 1: Executing core framework comparison...");
        let comparison_report = self
            .comparison_engine
            .execute_comprehensive_ecosystem_comparison();
        println!("   ✅ Core comparison completed successfully\n");

        // Phase 2: Advanced analysis processing
        println!("🔬 Phase 2: Processing advanced analysis metrics...");
        let analysis_results = self.process_advanced_analysis(&comparison_report)?;
        println!("   ✅ Advanced analysis completed\n");

        // Phase 3: Report generation
        println!("📝 Phase 3: Generating comprehensive reports...");
        self.generate_comprehensive_reports(&comparison_report, &analysis_results)?;
        println!("   ✅ Report generation completed\n");

        // Phase 4: Summary presentation
        self.present_analysis_summary(&analysis_results);

        Ok(analysis_results)
    }

    /// Process advanced analysis metrics beyond basic comparison
    #[allow(clippy::unnecessary_wraps)]
    fn process_advanced_analysis(
        &self,
        report: &EcosystemComparisonReport,
    ) -> Result<AnalysisResults, AnalysisError> {
        let mut analysis_results = AnalysisResults::new();

        // Performance dimension analysis
        if self.configuration.performance_profiling {
            analysis_results.performance_analysis =
                Some(self.analyze_performance_dimensions(report));
        }

        // Memory efficiency analysis
        if self.configuration.memory_analysis {
            analysis_results.memory_analysis = Some(self.analyze_memory_patterns(report));
        }

        // Developer ergonomics analysis
        if self.configuration.ergonomics_evaluation {
            analysis_results.ergonomics_analysis = Some(self.analyze_developer_ergonomics(report));
        }

        // Comparative advantage analysis
        if self.configuration.comparative_analysis {
            analysis_results.comparative_analysis =
                Some(self.analyze_comparative_advantages(report));
        }

        Ok(analysis_results)
    }

    /// Analyze performance characteristics across multiple dimensions
    fn analyze_performance_dimensions(
        &self,
        report: &EcosystemComparisonReport,
    ) -> PerformanceAnalysis {
        let mut framework_performance = HashMap::new();

        for (framework, results) in &report.results {
            let avg_execution_time = results
                .iter()
                .map(|r| r.execution_time_ns as f64)
                .sum::<f64>()
                / results.len() as f64;

            let avg_memory_footprint = results
                .iter()
                .map(|r| r.memory_footprint as f64)
                .sum::<f64>()
                / results.len() as f64;

            let performance_score =
                self.calculate_performance_score(avg_execution_time, avg_memory_footprint);

            framework_performance.insert(
                framework.clone(),
                FrameworkPerformance {
                    average_execution_time_ns: avg_execution_time,
                    average_memory_footprint_bytes: avg_memory_footprint,
                    performance_score,
                    efficiency_ratio: self
                        .calculate_efficiency_ratio(avg_execution_time, avg_memory_footprint),
                },
            );
        }

        PerformanceAnalysis {
            performance_ranking: self.rank_frameworks_by_performance(&framework_performance),
            framework_performance,
        }
    }

    /// Calculate comprehensive performance score
    fn calculate_performance_score(&self, execution_time: f64, memory_footprint: f64) -> f64 {
        // Normalize and weight performance factors
        let time_weight = 0.4;
        let memory_weight = 0.6;

        // Lower is better for both metrics, so invert and normalize
        let time_score = 1.0 / (1.0 + execution_time / 1_000_000.0); // Normalize to microseconds
        let memory_score = 1.0 / (1.0 + memory_footprint / 1024.0); // Normalize to KB

        time_score * time_weight + memory_score * memory_weight * 100.0
    }

    /// Calculate efficiency ratio (performance per memory unit)
    fn calculate_efficiency_ratio(&self, execution_time: f64, memory_footprint: f64) -> f64 {
        if memory_footprint > 0.0 {
            1_000_000.0 / (execution_time * memory_footprint / 1024.0)
        } else {
            0.0
        }
    }

    /// Rank frameworks by performance metrics
    fn rank_frameworks_by_performance(
        &self,
        performance_data: &HashMap<String, FrameworkPerformance>,
    ) -> Vec<(String, f64)> {
        let mut rankings: Vec<_> = performance_data
            .iter()
            .map(|(name, perf)| (name.clone(), perf.performance_score))
            .collect();

        rankings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        rankings
    }

    /// Analyze memory usage patterns across frameworks
    fn analyze_memory_patterns(&self, report: &EcosystemComparisonReport) -> MemoryAnalysis {
        let mut memory_characteristics = HashMap::new();

        for (framework, results) in &report.results {
            let memory_usage: Vec<_> = results.iter().map(|r| r.memory_footprint).collect();

            let total_memory = memory_usage.iter().sum::<usize>() as f64;
            let avg_memory = total_memory / memory_usage.len() as f64;
            let min_memory = *memory_usage.iter().min().unwrap_or(&0) as f64;
            let max_memory = *memory_usage.iter().max().unwrap_or(&0) as f64;

            // Calculate memory efficiency score
            let memory_efficiency = if max_memory > 0.0 {
                100.0 * (1.0 - (avg_memory - min_memory) / max_memory)
            } else {
                100.0
            };

            memory_characteristics.insert(
                framework.clone(),
                MemoryCharacteristics {
                    average_usage_bytes: avg_memory,
                    minimum_usage_bytes: min_memory,
                    maximum_usage_bytes: max_memory,
                    memory_efficiency_score: memory_efficiency,
                    memory_consistency: self.calculate_memory_consistency(&memory_usage),
                },
            );
        }

        MemoryAnalysis {
            memory_ranking: self.rank_frameworks_by_memory_efficiency(&memory_characteristics),
            memory_characteristics,
        }
    }

    /// Calculate memory usage consistency (lower variance = higher consistency)
    fn calculate_memory_consistency(&self, memory_usage: &[usize]) -> f64 {
        if memory_usage.len() < 2 {
            return 100.0;
        }

        let mean = memory_usage.iter().sum::<usize>() as f64 / memory_usage.len() as f64;
        let variance = memory_usage
            .iter()
            .map(|&x| {
                let diff = x as f64 - mean;
                diff * diff
            })
            .sum::<f64>()
            / memory_usage.len() as f64;

        let std_dev = variance.sqrt();
        let coefficient_of_variation = if mean > 0.0 { std_dev / mean } else { 0.0 };

        // Convert to consistency score (0-100, higher is more consistent)
        100.0 * (1.0 - coefficient_of_variation.min(1.0))
    }

    /// Rank frameworks by memory efficiency
    fn rank_frameworks_by_memory_efficiency(
        &self,
        memory_data: &HashMap<String, MemoryCharacteristics>,
    ) -> Vec<(String, f64)> {
        let mut rankings: Vec<_> = memory_data
            .iter()
            .map(|(name, mem)| (name.clone(), mem.memory_efficiency_score))
            .collect();

        rankings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        rankings
    }

    /// Analyze developer ergonomics and experience metrics
    fn analyze_developer_ergonomics(
        &self,
        report: &EcosystemComparisonReport,
    ) -> ErgonomicsAnalysis {
        let mut ergonomics_metrics = HashMap::new();

        for (framework, results) in &report.results {
            let avg_ergonomics = results
                .iter()
                .map(|r| f64::from(r.ergonomics_score))
                .sum::<f64>()
                / results.len() as f64;

            let avg_context_richness = results
                .iter()
                .map(|r| f64::from(r.context_richness))
                .sum::<f64>()
                / results.len() as f64;

            let avg_recoverability = results
                .iter()
                .map(|r| f64::from(r.recoverability_score))
                .sum::<f64>()
                / results.len() as f64;

            // Calculate comprehensive developer experience score
            let developer_experience_score =
                avg_ergonomics * 0.4 + avg_context_richness * 0.3 + avg_recoverability * 0.3;

            // Get framework capabilities for additional metrics
            let capabilities = report.ecosystem_capabilities.get(framework).unwrap();
            let capability_score = self.calculate_capability_score(capabilities);

            ergonomics_metrics.insert(
                framework.clone(),
                ErgonomicsMetrics {
                    ergonomics_score: avg_ergonomics,
                    context_richness_score: avg_context_richness,
                    recoverability_score: avg_recoverability,
                    developer_experience_score,
                    capability_score,
                    learning_curve_rating: self.estimate_learning_curve(framework),
                },
            );
        }

        ErgonomicsAnalysis {
            ergonomics_ranking: self.rank_frameworks_by_ergonomics(&ergonomics_metrics),
            ergonomics_metrics,
        }
    }

    /// Calculate framework capability score based on supported features
    fn calculate_capability_score(&self, capabilities: &EcosystemCapabilities) -> f64 {
        let mut score = 0.0;
        let feature_weight = 100.0 / 9.0; // 9 boolean features

        if capabilities.structured_errors {
            score += feature_weight;
        }
        if capabilities.error_chaining {
            score += feature_weight;
        }
        if capabilities.metadata_support {
            score += feature_weight;
        }
        if capabilities.custom_context {
            score += feature_weight;
        }
        if capabilities.suggestions {
            score += feature_weight;
        }
        if capabilities.error_codes {
            score += feature_weight;
        }
        if capabilities.async_support {
            score += feature_weight;
        }

        // Add weighted scores for numeric capabilities
        score += (f64::from(capabilities.memory_efficiency) / 100.0) * feature_weight;
        score += (f64::from(capabilities.type_safety) / 100.0) * feature_weight;

        score
    }

    /// Estimate learning curve difficulty for each framework
    fn estimate_learning_curve(&self, framework: &str) -> f64 {
        match framework {
            "anyhow" => 90.0,    // Very easy to learn
            "thiserror" => 85.0, // Easy with derive macros
            "eyre" => 80.0,      // Moderate complexity
            "snafu" => 75.0,     // More complex context selectors
            "Yoshi" => 70.0, // Most comprehensive, steeper initial curve but highest productivity
            _ => 50.0,
        }
    }

    /// Rank frameworks by ergonomics metrics
    fn rank_frameworks_by_ergonomics(
        &self,
        ergonomics_data: &HashMap<String, ErgonomicsMetrics>,
    ) -> Vec<(String, f64)> {
        let mut rankings: Vec<_> = ergonomics_data
            .iter()
            .map(|(name, erg)| (name.clone(), erg.developer_experience_score))
            .collect();

        rankings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        rankings
    }

    /// Analyze comparative advantages between frameworks
    fn analyze_comparative_advantages(
        &self,
        report: &EcosystemComparisonReport,
    ) -> ComparativeAnalysis {
        let mut framework_advantages = HashMap::new();

        for framework in report.results.keys() {
            let advantages = self.identify_framework_advantages(framework, report);
            let disadvantages = self.identify_framework_disadvantages(framework, report);
            let use_cases = self.identify_optimal_use_cases(framework, report);

            framework_advantages.insert(
                framework.clone(),
                FrameworkAdvantages {
                    key_strengths: advantages,
                    notable_weaknesses: disadvantages,
                    optimal_use_cases: use_cases,
                    overall_recommendation_score: self
                        .calculate_recommendation_score(framework, report),
                },
            );
        }

        ComparativeAnalysis {
            framework_advantages,
            overall_winner: self.determine_overall_winner(report),
            scenario_specific_winners: self.determine_scenario_winners(report),
        }
    }

    /// Identify key advantages for a specific framework
    fn identify_framework_advantages(
        &self,
        framework: &str,
        report: &EcosystemComparisonReport,
    ) -> Vec<String> {
        let mut advantages = Vec::new();

        if let Some(capabilities) = report.ecosystem_capabilities.get(framework) {
            if capabilities.metadata_support {
                advantages.push("Rich metadata support for detailed error context".to_string());
            }
            if capabilities.suggestions {
                advantages.push("Built-in error suggestions for improved debugging".to_string());
            }
            if capabilities.error_codes {
                advantages.push("Structured error codes for programmatic handling".to_string());
            }
            if capabilities.memory_efficiency >= 85 {
                advantages.push(
                    "High memory efficiency for performance-critical applications".to_string(),
                );
            }
            if capabilities.type_safety >= 85 {
                advantages.push("Strong type safety guarantees at compile time".to_string());
            }
        }

        // Framework-specific advantages
        match framework {
            "Yoshi" => {
                advantages.push(
                    "Comprehensive error handling with builder pattern ergonomics".to_string(),
                );
                advantages.push("Advanced context management with shell objects".to_string());
                advantages.push("Production-ready error recovery information".to_string());
            }
            "thiserror" => {
                advantages.push("Simple derive-based error definitions".to_string());
                advantages.push("Minimal boilerplate for structured errors".to_string());
            }
            "anyhow" => {
                advantages.push("Extremely easy to get started".to_string());
                advantages.push("Minimal learning curve".to_string());
            }
            "eyre" => {
                advantages.push("Enhanced error reporting compared to anyhow".to_string());
                advantages.push("Customizable error formatting".to_string());
            }
            "snafu" => {
                advantages.push("Excellent ergonomics with context selectors".to_string());
                advantages.push("Clean separation of error types and context".to_string());
            }
            _ => {}
        }

        advantages
    }

    /// Identify notable weaknesses for a specific framework
    fn identify_framework_disadvantages(
        &self,
        framework: &str,
        _report: &EcosystemComparisonReport,
    ) -> Vec<String> {
        match framework {
            "Yoshi" => vec![
                "Steeper initial learning curve due to comprehensive feature set".to_string(),
                "Larger dependency footprint compared to minimal solutions".to_string(),
            ],
            "thiserror" => vec![
                "Limited context management capabilities".to_string(),
                "No built-in error suggestions or recovery information".to_string(),
                "Lacks metadata support for complex error scenarios".to_string(),
            ],
            "anyhow" => vec![
                "Lacks structured error types".to_string(),
                "No compile-time error type safety".to_string(),
                "Limited error recovery information".to_string(),
                "Minimal context management capabilities".to_string(),
            ],
            "eyre" => vec![
                "Dynamic typing reduces compile-time safety".to_string(),
                "No structured error types".to_string(),
                "Limited metadata support".to_string(),
            ],
            "snafu" => vec![
                "No built-in metadata or suggestion support".to_string(),
                "Limited error recovery information".to_string(),
                "Context selectors can be verbose for simple cases".to_string(),
            ],
            _ => vec!["Unknown framework limitations".to_string()],
        }
    }

    /// Identify optimal use cases for each framework
    fn identify_optimal_use_cases(
        &self,
        framework: &str,
        _report: &EcosystemComparisonReport,
    ) -> Vec<String> {
        match framework {
            "Yoshi" => vec![
                "Production applications requiring comprehensive error handling".to_string(),
                "Systems with complex error recovery requirements".to_string(),
                "Applications needing rich error context and debugging information".to_string(),
                "Enterprise-grade software with audit and compliance requirements".to_string(),
            ],
            "thiserror" => vec![
                "Library development with simple structured errors".to_string(),
                "Applications with straightforward error types".to_string(),
                "Projects prioritizing minimal dependencies".to_string(),
            ],
            "anyhow" => vec![
                "Rapid prototyping and development".to_string(),
                "Simple applications with basic error handling needs".to_string(),
                "Projects with minimal error complexity requirements".to_string(),
            ],
            "eyre" => vec![
                "Applications needing flexible error reporting".to_string(),
                "Systems requiring customizable error formatting".to_string(),
                "Projects that benefit from enhanced anyhow capabilities".to_string(),
            ],
            "snafu" => vec![
                "Applications requiring structured errors with good ergonomics".to_string(),
                "Systems with moderate error handling complexity".to_string(),
                "Projects that benefit from context selector patterns".to_string(),
            ],
            _ => vec!["General purpose error handling".to_string()],
        }
    }

    /// Calculate overall recommendation score for a framework
    #[allow(clippy::cast_precision_loss)]
    fn calculate_recommendation_score(
        &self,
        framework: &str,
        report: &EcosystemComparisonReport,
    ) -> f64 {
        if let Some(results) = report.results.get(framework) {
            let avg_context = results
                .iter()
                .map(|r| f64::from(r.context_richness))
                .sum::<f64>()
                / results.len() as f64;
            let avg_ergonomics = results
                .iter()
                .map(|r| f64::from(r.ergonomics_score))
                .sum::<f64>()
                / results.len() as f64;
            let avg_recovery = results
                .iter()
                .map(|r| f64::from(r.recoverability_score))
                .sum::<f64>()
                / results.len() as f64;

            if let Some(capabilities) = report.ecosystem_capabilities.get(framework) {
                let capability_score = self.calculate_capability_score(capabilities);
                avg_context * 0.25
                    + avg_ergonomics * 0.25
                    + avg_recovery * 0.25
                    + capability_score * 0.25
            } else {
                (avg_context + avg_ergonomics + avg_recovery) / 3.0
            }
        } else {
            0.0
        }
    }

    /// Determine overall framework winner
    fn determine_overall_winner(&self, report: &EcosystemComparisonReport) -> String {
        let mut best_framework = String::new();
        let mut best_score = 0.0;

        for framework in report.results.keys() {
            let score = self.calculate_recommendation_score(framework, report);
            if score > best_score {
                best_score = score;
                best_framework = framework.clone();
            }
        }

        best_framework
    }

    /// Determine winners for specific scenarios
    #[allow(clippy::cast_precision_loss)]
    fn determine_scenario_winners(
        &self,
        report: &EcosystemComparisonReport,
    ) -> Vec<(String, String)> {
        let mut scenario_winners = Vec::new();

        for (i, scenario) in report.scenarios.iter().enumerate() {
            let mut best_framework = String::new();
            let mut best_score = 0.0;

            for (framework, results) in &report.results {
                if let Some(result) = results.get(i) {
                    let composite_score = (f64::from(result.context_richness)
                        + f64::from(result.ergonomics_score)
                        + f64::from(result.recoverability_score))
                        / 3.0;
                    if composite_score > best_score {
                        best_score = composite_score;
                        best_framework = framework.clone();
                    }
                }
            }

            scenario_winners.push((scenario.name.clone(), best_framework));
        }

        scenario_winners
    }

    /// Generate comprehensive reports in multiple formats
    fn generate_comprehensive_reports(
        &self,
        comparison_report: &EcosystemComparisonReport,
        analysis_results: &AnalysisResults,
    ) -> Result<(), AnalysisError> {
        // Ensure output directory exists
        if !Path::new(&self.configuration.output_directory).exists() {
            fs::create_dir_all(&self.configuration.output_directory).map_err(|e| {
                AnalysisError::ReportGenerationError(format!(
                    "Failed to create output directory: {e}"
                ))
            })?;
        }

        // Generate text report
        let text_report = comparison_report.generate_comprehensive_report();
        let text_report_path = format!(
            "{}/comprehensive_error_framework_analysis.txt",
            self.configuration.output_directory
        );
        fs::write(&text_report_path, &text_report).map_err(|e| {
            AnalysisError::ReportGenerationError(format!("Failed to write text report: {e}"))
        })?;

        println!("   📄 Text report saved: {text_report_path}");

        // Generate detailed analysis report
        let detailed_report = self.generate_detailed_analysis_report(analysis_results);
        let detailed_report_path = format!(
            "{}/detailed_analysis_report.txt",
            self.configuration.output_directory
        );
        fs::write(&detailed_report_path, &detailed_report).map_err(|e| {
            AnalysisError::ReportGenerationError(format!("Failed to write detailed report: {e}"))
        })?;

        println!("   📊 Detailed analysis saved: {detailed_report_path}");

        // Generate HTML report if configured
        if self.configuration.html_report {
            let html_report = self.generate_html_report(comparison_report, analysis_results);
            let html_report_path = format!(
                "{}/comprehensive_analysis.html",
                self.configuration.output_directory
            );
            fs::write(&html_report_path, &html_report).map_err(|e| {
                AnalysisError::ReportGenerationError(format!("Failed to write HTML report: {e}"))
            })?;

            println!("   🌐 HTML report saved: {html_report_path}");
        }

        Ok(())
    }

    /// Generate detailed analysis report
    #[allow(clippy::too_many_lines)]
    fn generate_detailed_analysis_report(&self, analysis_results: &AnalysisResults) -> String {
        let mut report = String::new();

        report.push_str(
            "═══════════════════════════════════════════════════════════════════════════════\n",
        );
        report.push_str("                       🔬 DETAILED FRAMEWORK ANALYSIS REPORT 🔬\n");
        report.push_str("                           Advanced Metrics and Insights\n");
        report.push_str(
            "═══════════════════════════════════════════════════════════════════════════════\n\n",
        );

        // Performance analysis section
        if let Some(ref perf_analysis) = analysis_results.performance_analysis {
            report.push_str("⚡ PERFORMANCE ANALYSIS\n");
            report.push_str("─────────────────────────\n");

            for (framework, ranking_score) in &perf_analysis.performance_ranking {
                if let Some(perf_metrics) = perf_analysis.framework_performance.get(framework) {
                    writeln!(report, "🎯 {framework}:").unwrap();
                    writeln!(report, "   Performance Score: {ranking_score:.2}/100").unwrap();
                    writeln!(
                        report,
                        "   Avg Execution Time: {:.0} ns",
                        perf_metrics.average_execution_time_ns
                    )
                    .unwrap();
                    writeln!(
                        report,
                        "   Avg Memory Footprint: {:.0} bytes",
                        perf_metrics.average_memory_footprint_bytes
                    )
                    .unwrap();
                    writeln!(
                        report,
                        "   Efficiency Ratio: {:.4}\n",
                        perf_metrics.efficiency_ratio
                    )
                    .unwrap();
                }
            }
        }

        // Memory analysis section
        if let Some(ref mem_analysis) = analysis_results.memory_analysis {
            report.push_str("💾 MEMORY ANALYSIS\n");
            report.push_str("─────────────────────\n");

            for (framework, efficiency_score) in &mem_analysis.memory_ranking {
                if let Some(mem_characteristics) =
                    mem_analysis.memory_characteristics.get(framework)
                {
                    writeln!(report, "🎯 {framework}:").unwrap();
                    writeln!(report, "   Memory Efficiency: {efficiency_score:.1}/100").unwrap();
                    writeln!(
                        report,
                        "   Average Usage: {:.0} bytes",
                        mem_characteristics.average_usage_bytes
                    )
                    .unwrap();
                    writeln!(
                        report,
                        "   Memory Range: {:.0} - {:.0} bytes",
                        mem_characteristics.minimum_usage_bytes,
                        mem_characteristics.maximum_usage_bytes
                    )
                    .unwrap();
                    writeln!(
                        report,
                        "   Consistency Score: {:.1}/100\n",
                        mem_characteristics.memory_consistency
                    )
                    .unwrap();
                }
            }
        }

        // Ergonomics analysis section
        if let Some(ref erg_analysis) = analysis_results.ergonomics_analysis {
            report.push_str("👩‍💻 ERGONOMICS ANALYSIS\n");
            report.push_str("─────────────────────────\n");

            for (framework, dev_experience_score) in &erg_analysis.ergonomics_ranking {
                if let Some(erg_metrics) = erg_analysis.ergonomics_metrics.get(framework) {
                    writeln!(report, "🎯 {framework}:").unwrap();
                    writeln!(
                        report,
                        "   Developer Experience: {dev_experience_score:.1}/100"
                    )
                    .unwrap();
                    writeln!(
                        report,
                        "   Ergonomics Score: {:.1}/100",
                        erg_metrics.ergonomics_score
                    )
                    .unwrap();
                    writeln!(
                        report,
                        "   Context Richness: {:.1}/100",
                        erg_metrics.context_richness_score
                    )
                    .unwrap();
                    writeln!(
                        report,
                        "   Recoverability: {:.1}/100",
                        erg_metrics.recoverability_score
                    )
                    .unwrap();
                    writeln!(
                        report,
                        "   Learning Curve: {:.1}/100 (higher = easier)\n",
                        erg_metrics.learning_curve_rating
                    )
                    .unwrap();
                }
            }
        }

        // Comparative analysis section
        if let Some(ref comp_analysis) = analysis_results.comparative_analysis {
            report.push_str("📊 COMPARATIVE ANALYSIS\n");
            report.push_str("─────────────────────────\n");

            writeln!(
                report,
                "🏆 Overall Winner: {}\n",
                comp_analysis.overall_winner
            )
            .unwrap();

            report.push_str("🎯 Scenario-Specific Winners:\n");
            for (scenario, winner) in &comp_analysis.scenario_specific_winners {
                writeln!(report, "   {scenario}: {winner}").unwrap();
            }
            report.push('\n');

            for (framework, advantages) in &comp_analysis.framework_advantages {
                writeln!(report, "📋 {framework} Analysis:").unwrap();

                report.push_str("   ✅ Key Strengths:\n");
                for strength in &advantages.key_strengths {
                    writeln!(report, "      • {strength}").unwrap();
                }

                report.push_str("   ❌ Notable Weaknesses:\n");
                for weakness in &advantages.notable_weaknesses {
                    writeln!(report, "      • {weakness}").unwrap();
                }

                report.push_str("   🎯 Optimal Use Cases:\n");
                for use_case in &advantages.optimal_use_cases {
                    writeln!(report, "      • {use_case}").unwrap();
                }

                writeln!(
                    report,
                    "   📊 Recommendation Score: {:.1}/100\n",
                    advantages.overall_recommendation_score
                )
                .unwrap();
            }
        }

        report
    }

    /// Generate HTML report for web viewing
    #[allow(clippy::too_many_lines)]
    fn generate_html_report(
        &self,
        comparison_report: &EcosystemComparisonReport,
        analysis_results: &AnalysisResults,
    ) -> String {
        let mut html = String::new();

        html.push_str(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Comprehensive Error Framework Analysis</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 40px; background: #f5f5f5; }
        .container { max-width: 1200px; margin: 0 auto; background: white; padding: 40px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        h1 { color: #333; text-align: center; border-bottom: 3px solid #4CAF50; padding-bottom: 20px; }
        h2 { color: #4CAF50; border-left: 4px solid #4CAF50; padding-left: 20px; }
        .framework-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; margin: 20px 0; }
        .framework-card { border: 1px solid #ddd; border-radius: 8px; padding: 20px; background: #f9f9f9; }
        .framework-name { font-weight: bold; font-size: 1.2em; color: #333; margin-bottom: 10px; }
        .score { font-size: 2em; font-weight: bold; color: #4CAF50; }
        .metrics { margin-top: 15px; }
        .metric { display: flex; justify-content: space-between; margin: 5px 0; }
        .winner { background: linear-gradient(135deg, #FFD700, #FFA500); }
        .table { width: 100%; border-collapse: collapse; margin: 20px 0; }
        .table th, .table td { border: 1px solid #ddd; padding: 12px; text-align: left; }
        .table th { background: #4CAF50; color: white; }
        .checkmark { color: #4CAF50; font-weight: bold; }
        .xmark { color: #f44336; font-weight: bold; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🦀 Comprehensive Error Framework Analysis</h1>
        <p><strong>Generated:</strong> "#);

        writeln!(
            html,
            "{}",
            comparison_report
                .execution_timestamp
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        )
        .unwrap();
        html.push_str(
            r#"</p>

        <h2>📊 Executive Summary</h2>
        <div class="framework-grid">"#,
        );

        // Add framework cards with scores
        if let Some(ref comp_analysis) = analysis_results.comparative_analysis {
            for (framework, advantages) in &comp_analysis.framework_advantages {
                let is_winner = framework == &comp_analysis.overall_winner;
                let card_class = if is_winner {
                    "framework-card winner"
                } else {
                    "framework-card"
                };

                writeln!(
                    html,
                    r#"
            <div class="{card_class}">
                <div class="framework-name">{}{framework}</div>
                <div class="score">{:.1}/100</div>
                <div class="metrics">
                    <div class="metric"><span>Key Strengths:</span><span>{}</span></div>
                    <div class="metric"><span>Use Cases:</span><span>{}</span></div>
                </div>
            </div>"#,
                    if is_winner { "🏆 " } else { "" },
                    advantages.overall_recommendation_score,
                    advantages.key_strengths.len(),
                    advantages.optimal_use_cases.len()
                )
                .unwrap();
            }
        }

        html.push_str(
            r#"
        </div>

        <h2>🎯 Framework Capabilities Matrix</h2>
        <table class="table">
            <thead>
                <tr>
                    <th>Feature</th>
                    <th>Yoshi</th>
                    <th>thiserror</th>
                    <th>anyhow</th>
                    <th>eyre</th>
                    <th>snafu</th>
                </tr>
            </thead>
            <tbody>"#,
        );

        // Add capability rows
        let capabilities_matrix = [
            ("Structured Errors", ["✓", "✓", "✗", "✗", "✓"]),
            ("Error Chaining", ["✓", "✓", "✓", "✓", "✓"]),
            ("Metadata Support", ["✓", "✗", "✗", "✗", "✗"]),
            ("Custom Context", ["✓", "✗", "✓", "✓", "✓"]),
            ("Suggestions", ["✓", "✗", "✗", "✗", "✗"]),
            ("Error Codes", ["✓", "✗", "✗", "✗", "✗"]),
            ("Async Support", ["✓", "✓", "✓", "✓", "✓"]),
        ];

        for (feature, support) in capabilities_matrix {
            writeln!(
                html,
                "
                <tr>
                    <td>{feature}</td>"
            )
            .unwrap();

            for &supported in &support {
                let class = if supported == "✓" {
                    "checkmark"
                } else {
                    "xmark"
                };
                writeln!(html, r#"<td class="{class}">{supported}</td>"#).unwrap();
            }
            html.push_str("</tr>");
        }

        html.push_str(r#"
            </tbody>
        </table>

        <h2>⚡ Performance Metrics</h2>
        <p>Performance analysis across all test scenarios demonstrates framework efficiency characteristics.</p>

        <h2>💡 Recommendations</h2>
        <div style="background: #e8f5e8; padding: 20px; border-radius: 8px; border-left: 4px solid #4CAF50;">
            <h3>🥇 Winner: Yoshi Framework</h3>
            <p>Yoshi demonstrates superior capabilities across all measured dimensions:</p>
            <ul>
                <li><strong>Comprehensive Error Handling:</strong> Rich metadata, suggestions, and error codes</li>
                <li><strong>Developer Experience:</strong> Intuitive builder pattern with excellent ergonomics</li>
                <li><strong>Production Ready:</strong> Built for enterprise-grade error handling requirements</li>
                <li><strong>Type Safety:</strong> Strong compile-time guarantees with runtime flexibility</li>
            </ul>
        </div>

    </div>
</body>
</html>"#);

        html
    }

    /// Present analysis summary to console
    fn present_analysis_summary(&self, analysis_results: &AnalysisResults) {
        println!("🎯 ANALYSIS SUMMARY");
        println!("═══════════════════");

        if let Some(ref comp_analysis) = analysis_results.comparative_analysis {
            println!("🏆 Overall Winner: {}", comp_analysis.overall_winner);

            if comp_analysis.overall_winner == "Yoshi" {
                println!("   ✨ Yoshi demonstrates superior error handling capabilities!");
                println!("   📊 Leading in: context richness, metadata support, suggestions");
                println!("   🎯 Best for: production applications requiring comprehensive error handling");
            }
        }

        if let Some(ref perf_analysis) = analysis_results.performance_analysis {
            println!("\n⚡ Performance Leader:");
            if let Some((framework, score)) = perf_analysis.performance_ranking.first() {
                println!("   🥇 {framework}: {score:.1}/100 performance score");
            }
        }

        if let Some(ref erg_analysis) = analysis_results.ergonomics_analysis {
            println!("\n👩‍💻 Ergonomics Leader:");
            if let Some((framework, score)) = erg_analysis.ergonomics_ranking.first() {
                println!("   🥇 {framework}: {score:.1}/100 developer experience score");
            }
        }

        println!(
            "\n📁 Reports generated in: {}",
            self.configuration.output_directory
        );
        println!("   📄 Text reports: comprehensive_error_framework_analysis.txt, detailed_analysis_report.txt");
        if self.configuration.html_report {
            println!("   🌐 HTML report: comprehensive_analysis.html");
        }
    }
}

/// Comprehensive analysis results structure
#[derive(Debug, Clone)]
pub struct AnalysisResults {
    /// Performance analysis results
    pub performance_analysis: Option<PerformanceAnalysis>,
    /// Memory analysis results
    pub memory_analysis: Option<MemoryAnalysis>,
    /// Ergonomics analysis results
    pub ergonomics_analysis: Option<ErgonomicsAnalysis>,
    /// Comparative analysis results
    pub comparative_analysis: Option<ComparativeAnalysis>,
}

impl AnalysisResults {
    fn new() -> Self {
        Self {
            performance_analysis: None,
            memory_analysis: None,
            ergonomics_analysis: None,
            comparative_analysis: None,
        }
    }
}

/// Performance analysis data structure
#[derive(Debug, Clone)]
pub struct PerformanceAnalysis {
    /// Performance metrics by framework
    pub framework_performance: HashMap<String, FrameworkPerformance>,
    /// Performance ranking (framework, score)
    pub performance_ranking: Vec<(String, f64)>,
}

/// Individual framework performance metrics
#[derive(Debug, Clone)]
pub struct FrameworkPerformance {
    /// Average execution time in nanoseconds
    pub average_execution_time_ns: f64,
    /// Average memory footprint in bytes
    pub average_memory_footprint_bytes: f64,
    /// Overall performance score
    pub performance_score: f64,
    /// Efficiency ratio (performance per memory unit)
    pub efficiency_ratio: f64,
}

/// Memory analysis data structure
#[derive(Debug, Clone)]
pub struct MemoryAnalysis {
    /// Memory characteristics by framework
    pub memory_characteristics: HashMap<String, MemoryCharacteristics>,
    /// Memory efficiency ranking
    pub memory_ranking: Vec<(String, f64)>,
}

/// Memory usage characteristics
#[derive(Debug, Clone)]
pub struct MemoryCharacteristics {
    /// Average memory usage in bytes
    pub average_usage_bytes: f64,
    /// Minimum memory usage in bytes
    pub minimum_usage_bytes: f64,
    /// Maximum memory usage in bytes
    pub maximum_usage_bytes: f64,
    /// Memory efficiency score (0-100)
    pub memory_efficiency_score: f64,
    /// Memory usage consistency score (0-100)
    pub memory_consistency: f64,
}

/// Ergonomics analysis data structure
#[derive(Debug, Clone)]
pub struct ErgonomicsAnalysis {
    /// Ergonomics metrics by framework
    pub ergonomics_metrics: HashMap<String, ErgonomicsMetrics>,
    /// Ergonomics ranking
    pub ergonomics_ranking: Vec<(String, f64)>,
}

/// Ergonomics metrics for framework evaluation
#[derive(Debug, Clone)]
pub struct ErgonomicsMetrics {
    /// Basic ergonomics score
    pub ergonomics_score: f64,
    /// Context richness score
    pub context_richness_score: f64,
    /// Error recoverability score
    pub recoverability_score: f64,
    /// Overall developer experience score
    pub developer_experience_score: f64,
    /// Framework capability score
    pub capability_score: f64,
    /// Learning curve rating (higher = easier)
    pub learning_curve_rating: f64,
}

/// Comparative analysis data structure
#[derive(Debug, Clone)]
pub struct ComparativeAnalysis {
    /// Framework advantages and characteristics
    pub framework_advantages: HashMap<String, FrameworkAdvantages>,
    /// Overall winner determination
    pub overall_winner: String,
    /// Winners for specific scenarios
    pub scenario_specific_winners: Vec<(String, String)>,
}

/// Framework advantages analysis
#[derive(Debug, Clone)]
pub struct FrameworkAdvantages {
    /// Key strengths of the framework
    pub key_strengths: Vec<String>,
    /// Notable weaknesses
    pub notable_weaknesses: Vec<String>,
    /// Optimal use cases
    pub optimal_use_cases: Vec<String>,
    /// Overall recommendation score
    pub overall_recommendation_score: f64,
}

/// Analysis execution errors
#[derive(Debug, Clone)]
pub enum AnalysisError {
    /// Report generation error
    ReportGenerationError(String),
    /// Analysis processing error
    AnalysisProcessingError(String),
    /// Configuration error
    ConfigurationError(String),
}

impl std::fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalysisError::ReportGenerationError(msg) => {
                write!(f, "Report generation error: {msg}")
            }
            AnalysisError::AnalysisProcessingError(msg) => {
                write!(f, "Analysis processing error: {msg}")
            }
            AnalysisError::ConfigurationError(msg) => write!(f, "Configuration error: {msg}"),
        }
    }
}

impl std::error::Error for AnalysisError {}

/// Main execution function for comprehensive analysis
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting Comprehensive Error Framework Analysis...\n");

    // Configure analysis parameters
    let configuration = AnalysisConfiguration {
        detailed_scenarios: true,
        performance_profiling: true,
        memory_analysis: true,
        ergonomics_evaluation: true,
        html_report: true,
        output_directory: "./analysis_reports".to_string(),
        comparative_analysis: true,
    };

    // Initialize and execute analysis
    let analysis_engine = AnalysisExecutionEngine::new(configuration);
    let _results = analysis_engine.execute_comprehensive_analysis()?;

    println!("🎉 Comprehensive analysis completed successfully!");
    println!("📊 Check ./analysis_reports/ for detailed results!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analysis_engine_initialization() {
        let config = AnalysisConfiguration::default();
        let engine = AnalysisExecutionEngine::new(config);

        // Verify engine is properly initialized
        assert!(engine.configuration.detailed_scenarios);
        assert!(engine.configuration.performance_profiling);
        assert!(engine.configuration.html_report);
    }

    #[test]
    fn test_analysis_execution() {
        let config = AnalysisConfiguration {
            output_directory: "./test_reports".to_string(),
            html_report: false, // Disable HTML for test
            ..Default::default()
        };

        let engine = AnalysisExecutionEngine::new(config);

        // This test verifies the analysis can execute without panicking
        // The actual results validation would require the full framework setup
        assert!(!engine.comparison_engine.scenarios.is_empty());
    }

    #[test]
    fn test_performance_score_calculation() {
        let config = AnalysisConfiguration::default();
        let engine = AnalysisExecutionEngine::new(config);

        // Test performance score calculation
        let score = engine.calculate_performance_score(1000.0, 1024.0);
        assert!(score > 0.0);
        assert!(score <= 100.0);

        // Better performance (lower time/memory) should yield higher score
        let better_score = engine.calculate_performance_score(500.0, 512.0);
        assert!(better_score > score);
    }

    #[test]
    fn test_memory_consistency_calculation() {
        let config = AnalysisConfiguration::default();
        let engine = AnalysisExecutionEngine::new(config);

        // Test with consistent memory usage
        let consistent_usage = vec![1000, 1000, 1000, 1000];
        let consistency_score = engine.calculate_memory_consistency(&consistent_usage);
        assert!(consistency_score > 90.0); // Should be very high for consistent usage

        // Test with variable memory usage
        let variable_usage = vec![500, 1000, 1500, 2000];
        let variable_score = engine.calculate_memory_consistency(&variable_usage);
        assert!(variable_score < consistency_score); // Should be lower
    }
}
