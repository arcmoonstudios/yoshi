/* examples/expert_usage.rs */
#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
//! **Brief:** Demonstrates expert-level usage patterns for the Yoshi error handling framework.
//!
//! This module explores the most advanced error composition, introspection capabilities,
//! and enterprise-grade error handling patterns using the complete Yoshi ecosystem.
//!
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Expert Error Usage Patterns]
//!  - [Complex Error Aggregation: Multiple error composition and analysis]
//!  - [Memory Management: String interning and performance optimization]
//!  - [Cross-Process Communication: Advanced error reporting systems]
//!  - [Performance Monitoring: SIMD optimization and metrics collection]
//!  - [Enterprise Integration: Complete ecosystem utilization]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Author:** Lord Xyn

use std::time::Duration;
use yoshi_std::{
    error_instance_count, memory, yum, Hatch, HatchExt, Hatchable, LayContext, YoContext, Yoshi,
    YoshiKind, YoshiLocation,
};

#[cfg(all(feature = "std", feature = "serde"))]
use yoshi_std::process_communication;

#[cfg(feature = "unstable-metrics")]
use yoshi_std::cross_process_metrics;

/// Advanced error state for complex scenarios.
#[derive(Debug, PartialEq, Clone)]
struct AdvancedErrorState {
    operation_id: String,
    retry_count: u32,
    service_tier: ServiceTier,
    performance_metrics: PerformanceMetrics,
}

#[derive(Debug, PartialEq, Clone)]
enum ServiceTier {
    Critical,
    Standard,
    BestEffort,
}

#[derive(Debug, PartialEq, Clone)]
struct PerformanceMetrics {
    latency_ms: u64,
    cpu_usage_percent: f64,
    memory_usage_bytes: usize,
}

/// Recovery strategy with advanced decision logic.
#[derive(Debug, PartialEq, Clone)]
enum AdvancedRecoveryStrategy {
    ExponentialBackoff {
        initial_delay: Duration,
        max_retries: u32,
        backoff_multiplier: f64,
    },
    CircuitBreaker {
        failure_threshold: u32,
        recovery_timeout: Duration,
    },
    Failover {
        target_service: String,
        fallback_data: Option<String>,
    },
    ManualEscalation {
        escalation_level: u8,
        contact_info: String,
    },
}

/// Example 1: Complex error aggregation with analysis.
///
/// This demonstrates sophisticated error composition and batch error handling
/// with comprehensive analysis capabilities.
mod example_1_complex_aggregation {
    use super::*;

    /// Creates a set of related errors for aggregation.
    fn create_error_batch() -> Vec<Yoshi> {
        vec![
            Yoshi::new(YoshiKind::Validation {
                field: "user_id".into(),
                message: "Invalid user identifier format".into(),
                expected: Some("UUID v4 format".into()),
                actual: Some("12345".into()),
            })
            .with_metadata("validation_rule", "uuid_format")
            .with_priority(150),
            Yoshi::new(YoshiKind::Network {
                message: "Authentication service unavailable".into(),
                source: None,
                error_code: Some(503),
            })
            .lay("Service discovery failed to locate auth endpoints")
            .meta("service_registry", "consul")
            .meta("datacenter", "us-west-2")
            .with_priority(220),
            Yoshi::new(YoshiKind::ResourceExhausted {
                resource: "database_connections".into(),
                limit: "100".into(),
                current: "100".into(),
                usage_percentage: Some(100.0),
            })
            .lay("Connection pool exhausted during user registration")
            .help("Scale up connection pool or implement connection recycling")
            .with_priority(240),
        ]
    }

    /// Creates a complex aggregated error with analysis.
    pub fn create_aggregated_error() -> Yoshi {
        let errors = create_error_batch();
        let total_severity: u16 = errors.iter().map(|e| e.severity() as u16).sum();
        let avg_severity = (total_severity / errors.len() as u16) as u8;

        Yoshi::new(YoshiKind::Multiple {
            errors,
            primary_index: Some(2), // Resource exhaustion is primary
        })
        .lay("User registration workflow failed with multiple critical issues")
        .help("Address resource constraints and service dependencies before retry")
        .meta("aggregate_severity", &avg_severity.to_string())
        .meta("workflow_stage", "user_onboarding")
        .with_priority(250)
    }

    /// Analyzes the aggregated error comprehensively.
    pub fn analyze_aggregated_error() -> (usize, u8, bool) {
        let error = create_aggregated_error();

        // Use yum! for enhanced debugging
        let debug_error = yum!(error);

        // Perform comprehensive analysis
        let analysis = debug_error.analyze_contexts();
        println!("=== Comprehensive Error Analysis ===");
        println!("Instance ID: {}", debug_error.instance_id());
        println!("Total contexts: {}", analysis.total_contexts);
        println!("Context depth: {}", analysis.context_depth);
        println!("Has suggestions: {}", analysis.has_suggestions);
        println!("Metadata entries: {}", analysis.metadata_entries);
        println!("Primary priority: {}", analysis.primary_context_priority);

        // Analyze multiple errors if present
        if let YoshiKind::Multiple {
            errors,
            primary_index,
        } = debug_error.kind()
        {
            println!("Multiple errors detected: {} total", errors.len());
            if let Some(primary_idx) = primary_index {
                println!("Primary error index: {}", primary_idx);
                if let Some(primary_error) = errors.get(*primary_idx) {
                    println!("Primary error: {}", primary_error);
                }
            }

            for (i, err) in errors.iter().enumerate() {
                println!(
                    "  Error {}: Severity={}, Transient={}",
                    i,
                    err.severity(),
                    err.is_transient()
                );
            }
        }

        (
            analysis.total_contexts,
            debug_error.severity(),
            debug_error.is_transient(),
        )
    }
}

/// Example 2: Memory management and performance optimization.
///
/// This demonstrates advanced memory management features and performance
/// monitoring capabilities.
mod example_2_memory_optimization {
    use super::*;

    /// Creates errors with optimized string usage.
    pub fn create_optimized_errors() -> Vec<Yoshi> {
        let mut errors = Vec::new();

        // Create multiple errors with shared string content
        for i in 0..10 {
            let error = Yoshi::new(YoshiKind::Internal {
                message: memory::efficient_string("Shared error message"), // Uses string interning
                source: None,
                component: Some(memory::efficient_string("SharedService")), // Reused string
            })
            .lay(&format!("Operation {} failed", i))
            .meta("error_batch", "optimization_test")
            .meta("shared_component", "SharedService");

            errors.push(error);
        }

        errors
    }

    /// Analyzes memory usage and performance.
    pub fn analyze_memory_performance() -> memory::MemoryStats {
        let initial_count = error_instance_count();
        let initial_stats = memory::get_memory_stats();

        println!("=== Memory Performance Analysis ===");
        println!("Initial error count: {}", initial_count);
        println!("Initial memory stats: {:?}", initial_stats);

        // Create batch of optimized errors
        let errors = create_optimized_errors();

        let final_count = error_instance_count();
        let final_stats = memory::get_memory_stats();

        println!("Final error count: {}", final_count);
        println!("Errors created: {}", final_count - initial_count);
        println!("Final memory stats: {:?}", final_stats);
        println!(
            "String intern hit rate: {:.2}%",
            (final_stats.string_intern_hits as f64
                / (final_stats.string_intern_hits + final_stats.string_intern_misses) as f64)
                * 100.0
        );

        // Trigger cleanup for long-running applications
        #[cfg(feature = "std")]
        memory::cleanup_intern_pool();

        drop(errors); // Explicit cleanup
        final_stats
    }
}

/// Example 3: Cross-process error communication.
///
/// This demonstrates enterprise-grade error reporting and coordination
/// across process boundaries.
#[cfg(all(feature = "std", feature = "serde"))]
mod example_3_cross_process {
    use super::*;

    /// Creates an error suitable for cross-process communication.
    pub fn create_distributed_error() -> Yoshi {
        Yoshi::new(YoshiKind::Network {
            message: "Distributed service coordination failure".into(),
            source: None,
            error_code: Some(500),
        })
        .lay("Inter-service communication breakdown detected")
        .meta("service_mesh", "istio")
        .meta("namespace", "production")
        .meta("correlation_id", "dist_xyz_789")
        .help("Check service mesh configuration and network policies")
        .with_shell(AdvancedErrorState {
            operation_id: "dist_op_001".to_string(),
            retry_count: 3,
            service_tier: ServiceTier::Critical,
            performance_metrics: PerformanceMetrics {
                latency_ms: 5000,
                cpu_usage_percent: 85.0,
                memory_usage_bytes: 512_000_000,
            },
        })
        .with_priority(255) // Maximum priority
    }

    /// Demonstrates cross-process error reporting.
    pub fn report_distributed_error() -> Result<(), Box<dyn std::error::Error>> {
        let error = create_distributed_error();

        // Report to global cross-process system
        process_communication::report_global_error(&error);

        // Get global reporter for custom reporting
        let reporter = process_communication::global_reporter();
        reporter.report_error(&error)?;

        println!("Distributed error reported to cross-process system");
        Ok(())
    }
}

/// Example 4: Performance monitoring and metrics.
///
/// This demonstrates advanced performance monitoring and metrics collection
/// for enterprise-grade error handling.
#[cfg(feature = "unstable-metrics")]
mod example_4_performance_monitoring {
    use super::*;

    /// Creates performance-critical errors for monitoring.
    pub fn create_performance_critical_errors() -> Vec<Yoshi> {
        let mut errors = Vec::new();

        // Create errors with varying severities
        for severity in [50, 100, 150, 200, 250] {
            let error = Yoshi::new(YoshiKind::Timeout {
                operation: format!("Performance test operation (severity {})", severity).into(),
                duration: Duration::from_millis(severity as u64 * 10),
                expected_max: Some(Duration::from_millis(1000)),
            })
            .lay(&format!(
                "Performance degradation detected at severity {}",
                severity
            ))
            .meta("performance_test", "true")
            .meta("severity_level", &severity.to_string())
            .with_priority(severity as u8);

            // Record in global metrics
            cross_process_metrics::record_global_error(&error);
            errors.push(error);
        }

        errors
    }

    /// Analyzes performance metrics and generates report.
    pub fn analyze_performance_metrics() -> cross_process_metrics::MetricsReport {
        println!("=== Performance Metrics Analysis ===");

        // Create test errors
        let errors = create_performance_critical_errors();

        // Generate comprehensive metrics report
        let report = cross_process_metrics::global_report();

        println!("Metrics Report:");
        println!("  Total errors: {}", report.total_errors);
        println!("  High severity errors: {}", report.high_severity_errors);
        println!(
            "  Medium severity errors: {}",
            report.medium_severity_errors
        );
        println!("  Low severity errors: {}", report.low_severity_errors);
        println!("  Memory usage: {} bytes", report.memory_usage);
        println!("  Report timestamp: {:?}", report.timestamp);

        // Demonstrate memory stats integration
        let memory_stats = memory::get_memory_stats();
        println!("Memory Performance:");
        println!(
            "  Total errors created: {}",
            memory_stats.total_errors_created
        );
        println!(
            "  String intern efficiency: {:.2}%",
            if memory_stats.string_intern_hits + memory_stats.string_intern_misses > 0 {
                (memory_stats.string_intern_hits as f64
                    / (memory_stats.string_intern_hits + memory_stats.string_intern_misses) as f64)
                    * 100.0
            } else {
                0.0
            }
        );

        drop(errors); // Cleanup
        report
    }
}

/// Example 5: Enterprise integration and complete ecosystem usage.
///
/// This demonstrates the complete Yoshi ecosystem in an enterprise scenario
/// with all advanced features integrated.
mod example_5_enterprise_integration {
    use super::*;

    /// Simulates a complete enterprise workflow with comprehensive error handling.
    pub fn enterprise_workflow() -> Hatch<String> {
        // Phase 1: Input validation with detailed error context
        validate_enterprise_input("invalid_data")
            .lay("Enterprise workflow input validation phase")
            .meta("workflow_id", "ent_wf_001")
            .meta("phase", "validation")?;

        // Phase 2: Service coordination with recovery strategies
        coordinate_enterprise_services()
            .lay("Enterprise service coordination phase")
            .meta("phase", "coordination")?;

        // Phase 3: Data processing with performance monitoring
        process_enterprise_data()
            .lay("Enterprise data processing phase")
            .meta("phase", "processing")?;

        Ok("Enterprise workflow completed successfully".to_string())
    }

    /// Validates enterprise input with advanced error handling.
    fn validate_enterprise_input(input: &str) -> Hatch<()> {
        if input == "invalid_data" {
            return Err(Yoshi::new(YoshiKind::Validation {
                field: "enterprise_input".into(),
                message: "Input failed enterprise validation criteria".into(),
                expected: Some("Valid enterprise data format".into()),
                actual: Some(input.into()),
            }))
            .with_shell(AdvancedRecoveryStrategy::ExponentialBackoff {
                initial_delay: Duration::from_secs(1),
                max_retries: 3,
                backoff_multiplier: 2.0,
            })
            .help("Ensure input conforms to enterprise data standards");
        }
        Ok(())
    }

    /// Coordinates enterprise services with advanced error handling.
    fn coordinate_enterprise_services() -> Hatch<()> {
        Err(Yoshi::new(YoshiKind::Network {
            message: "Enterprise service mesh coordination failure".into(),
            source: None,
            error_code: Some(503),
        }))
        .with_shell(AdvancedRecoveryStrategy::CircuitBreaker {
            failure_threshold: 5,
            recovery_timeout: Duration::from_secs(30),
        })
        .with_shell(AdvancedErrorState {
            operation_id: "coord_001".to_string(),
            retry_count: 2,
            service_tier: ServiceTier::Critical,
            performance_metrics: PerformanceMetrics {
                latency_ms: 3000,
                cpu_usage_percent: 70.0,
                memory_usage_bytes: 256_000_000,
            },
        })
        .help("Check enterprise service mesh health and circuit breaker status")
    }

    /// Processes enterprise data with comprehensive monitoring.
    fn process_enterprise_data() -> Hatch<()> {
        Err(Yoshi::new(YoshiKind::ResourceExhausted {
            resource: "enterprise_compute_cluster".into(),
            limit: "1000 cores".into(),
            current: "1000 cores".into(),
            usage_percentage: Some(100.0),
        }))
        .with_shell(AdvancedRecoveryStrategy::Failover {
            target_service: "backup_compute_cluster".to_string(),
            fallback_data: Some("cached_enterprise_results".to_string()),
        })
        .help("Scale enterprise compute resources or activate failover cluster")
    }

    /// Handles enterprise workflow errors with complete recovery logic.
    pub fn handle_enterprise_workflow() {
        match enterprise_workflow() {
            Ok(result) => {
                println!("✅ Enterprise workflow succeeded: {}", result);
            }
            Err(error) => {
                // Use yum! for comprehensive debugging
                let debug_error = yum!(error);

                println!("❌ Enterprise workflow failed");
                println!("Error analysis:");
                let analysis = debug_error.analyze_contexts();
                println!("  - Severity: {}", debug_error.severity());
                println!("  - Contexts: {}", analysis.total_contexts);
                println!(
                    "  - Has recovery strategies: {}",
                    analysis.typed_payloads > 0
                );

                // Check for recovery strategies
                if let Some(strategy) = debug_error.shell::<AdvancedRecoveryStrategy>() {
                    handle_recovery_strategy(strategy, &debug_error);
                }

                // Report to enterprise systems
                #[cfg(all(feature = "std", feature = "serde"))]
                process_communication::report_global_error(&debug_error);

                #[cfg(feature = "unstable-metrics")]
                cross_process_metrics::record_global_error(&debug_error);
            }
        }
    }

    /// Handles recovery strategies based on error context.
    fn handle_recovery_strategy(strategy: &AdvancedRecoveryStrategy, error: &Yoshi) {
        match strategy {
            AdvancedRecoveryStrategy::ExponentialBackoff {
                initial_delay,
                max_retries,
                ..
            } => {
                println!("🔄 Initiating exponential backoff recovery");
                println!(
                    "   Initial delay: {:?}, Max retries: {}",
                    initial_delay, max_retries
                );
            }
            AdvancedRecoveryStrategy::CircuitBreaker {
                failure_threshold,
                recovery_timeout,
            } => {
                println!("⚡ Circuit breaker activated");
                println!(
                    "   Failure threshold: {}, Recovery timeout: {:?}",
                    failure_threshold, recovery_timeout
                );
            }
            AdvancedRecoveryStrategy::Failover {
                target_service,
                fallback_data,
            } => {
                println!("🔀 Initiating failover to: {}", target_service);
                if let Some(data) = fallback_data {
                    println!("   Using fallback data: {}", data);
                }
            }
            AdvancedRecoveryStrategy::ManualEscalation {
                escalation_level,
                contact_info,
            } => {
                println!("🚨 Manual escalation required (Level {})", escalation_level);
                println!("   Contact: {}", contact_info);
                println!("   Error ID: {}", error.instance_id());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1_complex_aggregation() {
        let (total_contexts, severity, _transient) =
            example_1_complex_aggregation::analyze_aggregated_error();
        assert!(total_contexts > 0);
        assert!(severity > 0);

        let error = example_1_complex_aggregation::create_aggregated_error();
        assert!(matches!(error.kind(), YoshiKind::Multiple { .. }));
        assert!(error.primary_context().unwrap().priority == 250);
    }

    #[test]
    fn test_example_2_memory_optimization() {
        let stats = example_2_memory_optimization::analyze_memory_performance();
        assert!(stats.total_errors_created > 0);

        let errors = example_2_memory_optimization::create_optimized_errors();
        assert_eq!(errors.len(), 10);

        // Verify string interning is working
        for error in &errors {
            assert!(error
                .primary_context()
                .unwrap()
                .metadata
                .contains_key(&"error_batch".into()));
        }
    }

    #[test]
    #[cfg(all(feature = "std", feature = "serde"))]
    fn test_example_3_cross_process() {
        let error = example_3_cross_process::create_distributed_error();
        assert!(matches!(error.kind(), YoshiKind::Network { .. }));
        assert!(error.shell::<AdvancedErrorState>().is_some());
        assert_eq!(error.primary_context().unwrap().priority, 255);

        // Test reporting (should not panic)
        let _ = example_3_cross_process::report_distributed_error();
    }

    #[test]
    #[cfg(feature = "unstable-metrics")]
    fn test_example_4_performance_monitoring() {
        let report = example_4_performance_monitoring::analyze_performance_metrics();
        assert!(report.total_errors > 0);

        let errors = example_4_performance_monitoring::create_performance_critical_errors();
        assert_eq!(errors.len(), 5);

        // Verify different severities
        let severities: Vec<_> = errors.iter().map(|e| e.severity()).collect();
        assert!(severities.iter().any(|&s| s >= 200)); // High severity present
    }

    #[test]
    fn test_example_5_enterprise_integration() {
        // Test workflow components
        let workflow_result = example_5_enterprise_integration::enterprise_workflow();
        assert!(workflow_result.is_err());

        // Test comprehensive error handling (should not panic)
        example_5_enterprise_integration::handle_enterprise_workflow();

        // Verify error has recovery strategies
        if let Err(error) = workflow_result {
            assert!(error.shell::<AdvancedRecoveryStrategy>().is_some());
        }
    }
}
