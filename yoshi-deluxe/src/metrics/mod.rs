/* yoshi-deluxe/src/metrics.rs */
//! **Brief:** System metrics collection and monitoring for yoshi-deluxe.
//!
//! This module provides comprehensive metrics collection, performance monitoring,
//! and system health tracking capabilities. It integrates with the yoshi error
//! framework to provide structured error tracking and analysis.

use crate::errors::{Result, YoshiDeluxeExt};
use std::{
    collections::{HashMap, VecDeque},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{Duration, SystemTime},
};
use tokio::sync::RwLock;
use yoshi_std::{HatchExt, LayText};

//--------------------------------------------------------------------------------------------------
// System Metrics Collection
//--------------------------------------------------------------------------------------------------

/// Comprehensive system metrics collector
pub struct SystemMetricsCollector {
    /// System start time
    start_time: SystemTime,
    /// Analysis metrics
    analysis_metrics: Arc<RwLock<AnalysisMetrics>>,
    /// Performance metrics
    performance_metrics: Arc<RwLock<PerformanceMetrics>>,
    /// Error tracking
    error_metrics: Arc<RwLock<ErrorMetrics>>,
    /// Resource utilization
    resource_metrics: Arc<RwLock<ResourceMetrics>>,
}

/// Analysis operation metrics
#[derive(Debug, Default)]
struct AnalysisMetrics {
    /// Total analyses started
    total_analyses: u64,
    /// Analyses completed successfully
    successful_analyses: u64,
    /// Failed analyses
    failed_analyses: u64,
    /// Total diagnostics found
    total_diagnostics: u64,
    /// Total corrections generated
    total_corrections: u64,
    /// Corrections successfully applied
    applied_corrections: u64,
    /// Analysis times (recent measurements)
    analysis_times: VecDeque<Duration>,
    /// Corrections per analysis
    corrections_per_analysis: VecDeque<usize>,
}

/// Performance tracking metrics
#[derive(Debug, Default)]
struct PerformanceMetrics {
    /// Average processing time per component
    component_times: HashMap<String, VecDeque<Duration>>,
    /// Throughput measurements
    throughput_data: VecDeque<ThroughputMeasurement>,
    /// Cache performance
    cache_performance: HashMap<String, CachePerformanceData>,
    /// Concurrent operation counts
    concurrent_operations: HashMap<String, u64>,
}

/// Error tracking and categorization
#[derive(Debug, Default)]
struct ErrorMetrics {
    /// Errors by category
    errors_by_category: HashMap<String, u64>,
    /// Errors by severity
    errors_by_severity: HashMap<String, u64>,
    /// Recent error patterns
    recent_errors: VecDeque<ErrorEvent>,
    /// Error recovery success rate
    recovery_success_rate: f64,
    /// Error frequency over time
    error_frequency: VecDeque<(SystemTime, u64)>,
}

/// Resource utilization tracking
#[derive(Debug, Default)]
struct ResourceMetrics {
    /// Memory usage over time
    memory_usage: VecDeque<MemoryMeasurement>,
    /// Cache sizes
    cache_sizes: HashMap<String, usize>,
    /// Concurrent operation limits
    concurrency_limits: HashMap<String, usize>,
    /// Resource exhaustion events
    resource_exhaustion_events: u64,
}

/// Throughput measurement
#[derive(Debug, Clone)]
struct ThroughputMeasurement {
    /// Timestamp of measurement
    timestamp: SystemTime,
    /// Operations per second
    ops_per_second: f64,
    /// Operation type
    operation_type: String,
}

/// Cache performance data
#[derive(Debug, Clone)]
struct CachePerformanceData {
    /// Hit count
    hits: u64,
    /// Miss count
    misses: u64,
    /// Eviction count
    evictions: u64,
    /// Average lookup time
    avg_lookup_time: Duration,
}

impl CachePerformanceData {
    /// Calculate hit ratio
    fn hit_ratio(&self) -> f64 {
        let total = self.hits + self.misses;
        if total > 0 {
            self.hits as f64 / total as f64
        } else {
            0.0
        }
    }
}

/// Error event for tracking
#[derive(Debug, Clone)]
struct ErrorEvent {
    /// When the error occurred
    timestamp: SystemTime,
    /// Error category
    category: String,
    /// Error severity
    severity: String,
    /// Error message
    message: String,
    /// Recovery attempted
    recovery_attempted: bool,
    /// Recovery successful
    recovery_successful: bool,
}

/// Memory usage measurement
#[derive(Debug, Clone)]
struct MemoryMeasurement {
    /// Timestamp
    timestamp: SystemTime,
    /// Total memory usage in bytes
    total_bytes: usize,
    /// Cache memory usage
    cache_bytes: usize,
    /// Working set size
    working_set_bytes: usize,
}

//--------------------------------------------------------------------------------------------------
// Metrics Collector Implementation
//--------------------------------------------------------------------------------------------------

impl SystemMetricsCollector {
    /// Create new metrics collector
    #[must_use]
    pub fn new() -> Self {
        Self {
            start_time: SystemTime::now(),
            analysis_metrics: Arc::new(RwLock::new(AnalysisMetrics::default())),
            performance_metrics: Arc::new(RwLock::new(PerformanceMetrics::default())),
            error_metrics: Arc::new(RwLock::new(ErrorMetrics::default())),
            resource_metrics: Arc::new(RwLock::new(ResourceMetrics::default())),
        }
    }

    /// Record analysis start
    pub async fn record_analysis_start(&self) {
        let mut metrics = self.analysis_metrics.write().await;
        metrics.total_analyses += 1;
    }

    /// Record analysis completion
    pub async fn record_analysis_complete(&self, duration: Duration) {
        let mut metrics = self.analysis_metrics.write().await;
        metrics.successful_analyses += 1;

        // Keep recent measurements (last 1000)
        metrics.analysis_times.push_back(duration);
        if metrics.analysis_times.len() > 1000 {
            metrics.analysis_times.pop_front();
        }
    }

    /// Record analysis failure
    pub async fn record_analysis_failure(&self) {
        let mut metrics = self.analysis_metrics.write().await;
        metrics.failed_analyses += 1;
    }

    /// Record diagnostics found
    pub async fn record_diagnostics_found(&self, count: usize) {
        let mut metrics = self.analysis_metrics.write().await;
        metrics.total_diagnostics += count as u64;
    }

    /// Record corrections generated
    pub async fn record_corrections_generated(&self, count: usize) {
        let mut metrics = self.analysis_metrics.write().await;
        metrics.total_corrections += count as u64;

        metrics.corrections_per_analysis.push_back(count);
        if metrics.corrections_per_analysis.len() > 1000 {
            metrics.corrections_per_analysis.pop_front();
        }
    }

    /// Record correction applied
    pub async fn record_correction_applied(&self) {
        let mut metrics = self.analysis_metrics.write().await;
        metrics.applied_corrections += 1;
    }

    /// Record processing error
    pub async fn record_processing_error(&self) {
        self.record_error("processing", "error", "Processing failed", false, false)
            .await;
    }

    /// Record application error
    pub async fn record_application_error(&self) {
        self.record_error("application", "error", "Application failed", false, false)
            .await;
    }

    /// Record application complete
    pub async fn record_application_complete(&self, _duration: Duration) {
        // Could track application timing if needed
    }

    /// Record error with details
    pub async fn record_error(
        &self,
        category: &str,
        severity: &str,
        message: &str,
        recovery_attempted: bool,
        recovery_successful: bool,
    ) {
        let mut metrics = self.error_metrics.write().await;

        // Update category counts
        *metrics
            .errors_by_category
            .entry(category.to_string())
            .or_insert(0) += 1;
        *metrics
            .errors_by_severity
            .entry(severity.to_string())
            .or_insert(0) += 1;

        // Add to recent errors
        metrics.recent_errors.push_back(ErrorEvent {
            timestamp: SystemTime::now(),
            category: category.to_string(),
            severity: severity.to_string(),
            message: message.to_string(),
            recovery_attempted,
            recovery_successful,
        });

        // Keep only recent errors (last 1000)
        if metrics.recent_errors.len() > 1000 {
            metrics.recent_errors.pop_front();
        }

        // Update recovery success rate
        if recovery_attempted {
            let successful_recoveries = metrics
                .recent_errors
                .iter()
                .filter(|e| e.recovery_attempted && e.recovery_successful)
                .count();
            let total_recovery_attempts = metrics
                .recent_errors
                .iter()
                .filter(|e| e.recovery_attempted)
                .count();

            if total_recovery_attempts > 0 {
                metrics.recovery_success_rate =
                    successful_recoveries as f64 / total_recovery_attempts as f64;
            }
        }
    }

    /// Record component performance
    pub async fn record_component_performance(&self, component: &str, duration: Duration) {
        let mut metrics = self.performance_metrics.write().await;

        let times = metrics
            .component_times
            .entry(component.to_string())
            .or_insert_with(VecDeque::new);

        times.push_back(duration);
        if times.len() > 500 {
            times.pop_front();
        }
    }

    /// Record cache performance
    pub async fn record_cache_performance(
        &self,
        cache_name: &str,
        hit: bool,
        lookup_time: Duration,
    ) {
        let mut metrics = self.performance_metrics.write().await;

        let cache_perf = metrics
            .cache_performance
            .entry(cache_name.to_string())
            .or_insert_with(|| CachePerformanceData {
                hits: 0,
                misses: 0,
                evictions: 0,
                avg_lookup_time: Duration::ZERO,
            });

        if hit {
            cache_perf.hits += 1;
        } else {
            cache_perf.misses += 1;
        }

        // Update average lookup time
        let total_operations = cache_perf.hits + cache_perf.misses;
        if total_operations > 0 {
            cache_perf.avg_lookup_time = Duration::from_nanos(
                (cache_perf.avg_lookup_time.as_nanos() as u64 * (total_operations - 1)
                    + lookup_time.as_nanos() as u64)
                    / total_operations,
            );
        }
    }

    /// Record throughput measurement
    pub async fn record_throughput(&self, operation_type: &str, ops_per_second: f64) {
        let mut metrics = self.performance_metrics.write().await;

        metrics.throughput_data.push_back(ThroughputMeasurement {
            timestamp: SystemTime::now(),
            ops_per_second,
            operation_type: operation_type.to_string(),
        });

        // Keep only recent measurements (last 1000)
        if metrics.throughput_data.len() > 1000 {
            metrics.throughput_data.pop_front();
        }
    }

    /// Record memory usage
    pub async fn record_memory_usage(
        &self,
        total_bytes: usize,
        cache_bytes: usize,
        working_set_bytes: usize,
    ) {
        let mut metrics = self.resource_metrics.write().await;

        metrics.memory_usage.push_back(MemoryMeasurement {
            timestamp: SystemTime::now(),
            total_bytes,
            cache_bytes,
            working_set_bytes,
        });

        // Keep only recent measurements (last 500)
        if metrics.memory_usage.len() > 500 {
            metrics.memory_usage.pop_front();
        }
    }

    /// Record cache size
    pub async fn record_cache_size(&self, cache_name: &str, size: usize) {
        let mut metrics = self.resource_metrics.write().await;
        metrics.cache_sizes.insert(cache_name.to_string(), size);
    }

    /// Record resource exhaustion
    pub async fn record_resource_exhaustion(&self) {
        let mut metrics = self.resource_metrics.write().await;
        metrics.resource_exhaustion_events += 1;
    }

    /// Get system uptime
    pub async fn get_uptime(&self) -> Duration {
        self.start_time.elapsed().unwrap_or_default()
    }

    /// Get total analyses performed
    pub async fn get_total_analyses(&self) -> u64 {
        let metrics = self.analysis_metrics.read().await;
        metrics.total_analyses
    }

    /// Get total corrections generated
    pub async fn get_total_corrections(&self) -> u64 {
        let metrics = self.analysis_metrics.read().await;
        metrics.total_corrections
    }

    /// Get success rate
    pub async fn get_success_rate(&self) -> f64 {
        let metrics = self.analysis_metrics.read().await;
        if metrics.total_analyses > 0 {
            metrics.successful_analyses as f64 / metrics.total_analyses as f64
        } else {
            0.0
        }
    }

    /// Get average analysis time
    pub async fn get_average_analysis_time(&self) -> Duration {
        let metrics = self.analysis_metrics.read().await;
        if metrics.analysis_times.is_empty() {
            Duration::ZERO
        } else {
            let total: Duration = metrics.analysis_times.iter().sum();
            total / metrics.analysis_times.len() as u32
        }
    }

    /// Get comprehensive metrics snapshot
    pub async fn get_metrics_snapshot(&self) -> MetricsSnapshot {
        let analysis = self.analysis_metrics.read().await;
        let performance = self.performance_metrics.read().await;
        let errors = self.error_metrics.read().await;
        let resources = self.resource_metrics.read().await;

        MetricsSnapshot {
            timestamp: SystemTime::now(),
            uptime: self.start_time.elapsed().unwrap_or_default(),
            analysis_summary: AnalysisSummary {
                total_analyses: analysis.total_analyses,
                successful_analyses: analysis.successful_analyses,
                failed_analyses: analysis.failed_analyses,
                success_rate: if analysis.total_analyses > 0 {
                    analysis.successful_analyses as f64 / analysis.total_analyses as f64
                } else {
                    0.0
                },
                total_diagnostics: analysis.total_diagnostics,
                total_corrections: analysis.total_corrections,
                applied_corrections: analysis.applied_corrections,
                average_analysis_time: if analysis.analysis_times.is_empty() {
                    Duration::ZERO
                } else {
                    analysis.analysis_times.iter().sum::<Duration>()
                        / analysis.analysis_times.len() as u32
                },
                average_corrections_per_analysis: if analysis.corrections_per_analysis.is_empty() {
                    0.0
                } else {
                    analysis.corrections_per_analysis.iter().sum::<usize>() as f64
                        / analysis.corrections_per_analysis.len() as f64
                },
            },
            performance_summary: PerformanceSummary {
                component_performance: performance
                    .component_times
                    .iter()
                    .map(|(name, times)| {
                        let avg_time = if times.is_empty() {
                            Duration::ZERO
                        } else {
                            times.iter().sum::<Duration>() / times.len() as u32
                        };
                        (name.clone(), avg_time)
                    })
                    .collect(),
                cache_performance: performance
                    .cache_performance
                    .iter()
                    .map(|(name, data)| {
                        (
                            name.clone(),
                            CacheMetrics {
                                hit_ratio: data.hit_ratio(),
                                total_operations: data.hits + data.misses,
                                average_lookup_time: data.avg_lookup_time,
                            },
                        )
                    })
                    .collect(),
                recent_throughput: performance
                    .throughput_data
                    .iter()
                    .rev()
                    .take(10)
                    .cloned()
                    .collect(),
            },
            error_summary: ErrorSummary {
                total_errors: errors.errors_by_category.values().sum(),
                errors_by_category: errors.errors_by_category.clone(),
                errors_by_severity: errors.errors_by_severity.clone(),
                recovery_success_rate: errors.recovery_success_rate,
                recent_error_count: errors.recent_errors.len(),
            },
            resource_summary: ResourceSummary {
                current_memory_usage: resources.memory_usage.back().cloned(),
                cache_sizes: resources.cache_sizes.clone(),
                resource_exhaustion_events: resources.resource_exhaustion_events,
                total_cache_memory: resources.cache_sizes.values().sum(),
            },
        }
    }

    /// Generate performance report
    pub async fn generate_performance_report(&self) -> PerformanceReport {
        let snapshot = self.get_metrics_snapshot().await;

        PerformanceReport {
            report_timestamp: SystemTime::now(),
            system_uptime: snapshot.uptime,
            overall_health_score: self.calculate_health_score(&snapshot).await,
            analysis_performance: AnalysisPerformanceReport {
                success_rate: snapshot.analysis_summary.success_rate,
                average_time: snapshot.analysis_summary.average_analysis_time,
                throughput: if snapshot.analysis_summary.average_analysis_time > Duration::ZERO {
                    1.0 / snapshot
                        .analysis_summary
                        .average_analysis_time
                        .as_secs_f64()
                } else {
                    0.0
                },
                efficiency_score: self
                    .calculate_efficiency_score(&snapshot.analysis_summary)
                    .await,
            };
            
        let error_analysis = ErrorAnalysisReport {
            error_rate: snapshot.error_summary.total_errors as f64
                / snapshot.analysis_summary.total_analyses.max(1) as f64,
            recovery_rate: snapshot.error_summary.recovery_success_rate,
            most_common_errors: {
                let mut errors: Vec<_> =
                    snapshot.error_summary.errors_by_category.iter().collect();
                errors.sort_by(|a, b| b.1.cmp(a.1));
                errors
                    .into_iter()
                    .take(5)
                    .map(|(k, v)| (k.clone(), *v))
                    .collect()
            },
            },
        };

        let recommendations = self.generate_recommendations(&snapshot).await;
        
        SystemHealthReport {
            timestamp: snapshot.timestamp,
            overall_health: self.calculate_health_score(&snapshot).await,
            performance: performance_report,
            component_performance: snapshot.performance_summary.component_performance,
            cache_efficiency: snapshot
                .performance_summary
                .cache_performance
                .iter()
                .map(|(name, metrics)| (name.clone(), metrics.hit_ratio))
                .collect(),
            error_analysis: error_analysis,
            recommendations,
        }
    }

    /// Calculate overall system health score (0.0 - 1.0)
    async fn calculate_health_score(&self, snapshot: &MetricsSnapshot) -> f64 {
        let mut score = 1.0;

        // Factor in success rate
        score *= snapshot.analysis_summary.success_rate;

        // Factor in error rate
        let error_rate = snapshot.error_summary.total_errors as f64
            / snapshot.analysis_summary.total_analyses.max(1) as f64;
        score *= (1.0 - error_rate.min(1.0));

        // Factor in cache performance
        let avg_cache_hit_ratio = if snapshot.performance_summary.cache_performance.is_empty() {
            1.0
        } else {
            snapshot
                .performance_summary
                .cache_performance
                .values()
                .map(|m| m.hit_ratio)
                .sum::<f64>()
                / snapshot.performance_summary.cache_performance.len() as f64
        };
        score *= avg_cache_hit_ratio;

        // Factor in recovery rate
        score *= snapshot.error_summary.recovery_success_rate;

        score.max(0.0).min(1.0)
    }

    /// Calculate efficiency score for analysis operations
    async fn calculate_efficiency_score(&self, analysis: &AnalysisSummary) -> f64 {
        let mut score = 1.0;

        // Factor in corrections per analysis (more corrections = more efficiency)
        if analysis.average_corrections_per_analysis > 0.0 {
            score *= (analysis.average_corrections_per_analysis / 5.0).min(1.0);
            // Normalize to max 5 corrections
        }

        // Factor in applied correction rate
        if analysis.total_corrections > 0 {
            let application_rate =
                analysis.applied_corrections as f64 / analysis.total_corrections as f64;
            score *= application_rate;
        }

        // Factor in analysis speed (faster = better, up to a point)
        if analysis.average_analysis_time > Duration::ZERO {
            let time_score = (Duration::from_secs(10).as_secs_f64()
                / analysis.average_analysis_time.as_secs_f64())
            .min(1.0);
            score *= time_score;
        }

        score.max(0.0).min(1.0)
    }

    /// Generate performance recommendations
    async fn generate_recommendations(&self, snapshot: &MetricsSnapshot) -> Vec<String> {
        let mut recommendations = Vec::new();

        // Analysis performance recommendations
        if snapshot.analysis_summary.success_rate < 0.8 {
            recommendations.push("Consider investigating frequent analysis failures".to_string());
        }

        if snapshot.analysis_summary.average_analysis_time > Duration::from_secs(30) {
            recommendations.push(
                "Analysis times are high - consider optimizing or increasing parallelism"
                    .to_string(),
            );
        }

        // Cache performance recommendations
        for (cache_name, metrics) in &snapshot.performance_summary.cache_performance {
            if metrics.hit_ratio < 0.7 {
                recommendations.push(format!("Cache '{}' has low hit ratio ({:.1}%) - consider tuning cache size or eviction policy", cache_name, metrics.hit_ratio * 100.0));
            }
        }

        // Error rate recommendations
        if snapshot.error_summary.total_errors > 0 {
            let error_rate = snapshot.error_summary.total_errors as f64
                / snapshot.analysis_summary.total_analyses.max(1) as f64;
            if error_rate > 0.1 {
                recommendations.push(
                    "High error rate detected - review error patterns and improve error handling"
                        .to_string(),
                );
            }
        }

        // Recovery rate recommendations
        if snapshot.error_summary.recovery_success_rate < 0.8 {
            recommendations
                .push("Low error recovery rate - improve error recovery mechanisms".to_string());
        }

        // Resource utilization recommendations
        if let Some(memory) = &snapshot.resource_summary.current_memory_usage {
            if memory.total_bytes > 1024 * 1024 * 1024 {
                // > 1GB
                recommendations.push("High memory usage detected - consider implementing memory optimization strategies".to_string());
            }
        }

        if recommendations.is_empty() {
            recommendations
                .push("System is performing well - no specific recommendations".to_string());
        }

        recommendations
    }
}

impl Default for SystemMetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

//--------------------------------------------------------------------------------------------------
// Metrics Data Structures
//--------------------------------------------------------------------------------------------------

/// Comprehensive metrics snapshot
#[derive(Debug, Clone)]
pub struct MetricsSnapshot {
    /// When snapshot was taken
    pub timestamp: SystemTime,
    /// System uptime
    pub uptime: Duration,
    /// Analysis metrics summary
    pub analysis_summary: AnalysisSummary,
    /// Performance metrics summary
    pub performance_summary: PerformanceSummary,
    /// Error metrics summary
    pub error_summary: ErrorSummary,
    /// Resource utilization summary
    pub resource_summary: ResourceSummary,
}

/// Analysis metrics summary
#[derive(Debug, Clone)]
pub struct AnalysisSummary {
    /// Total analyses performed
    pub total_analyses: u64,
    /// Successful analyses
    pub successful_analyses: u64,
    /// Failed analyses
    pub failed_analyses: u64,
    /// Success rate (0.0 - 1.0)
    pub success_rate: f64,
    /// Total diagnostics found
    pub total_diagnostics: u64,
    /// Total corrections generated
    pub total_corrections: u64,
    /// Applied corrections
    pub applied_corrections: u64,
    /// Average analysis time
    pub average_analysis_time: Duration,
    /// Average corrections per analysis
    pub average_corrections_per_analysis: f64,
}

/// Performance metrics summary
#[derive(Debug, Clone)]
pub struct PerformanceSummary {
    /// Performance by component
    pub component_performance: HashMap<String, Duration>,
    /// Cache performance metrics
    pub cache_performance: HashMap<String, CacheMetrics>,
    /// Recent throughput measurements
    pub recent_throughput: Vec<ThroughputMeasurement>,
}

/// Cache performance metrics
#[derive(Debug, Clone)]
pub struct CacheMetrics {
    /// Hit ratio (0.0 - 1.0)
    pub hit_ratio: f64,
    /// Total operations
    pub total_operations: u64,
    /// Average lookup time
    pub average_lookup_time: Duration,
}

/// Error metrics summary
#[derive(Debug, Clone)]
pub struct ErrorSummary {
    /// Total errors
    pub total_errors: u64,
    /// Errors by category
    pub errors_by_category: HashMap<String, u64>,
    /// Errors by severity
    pub errors_by_severity: HashMap<String, u64>,
    /// Recovery success rate
    pub recovery_success_rate: f64,
    /// Number of recent errors tracked
    pub recent_error_count: usize,
}

/// Resource utilization summary
#[derive(Debug, Clone)]
pub struct ResourceSummary {
    /// Current memory usage
    pub current_memory_usage: Option<MemoryMeasurement>,
    /// Cache sizes
    pub cache_sizes: HashMap<String, usize>,
    /// Resource exhaustion events
    pub resource_exhaustion_events: u64,
    /// Total cache memory usage
    pub total_cache_memory: usize,
}

/// Performance report
#[derive(Debug, Clone)]
pub struct PerformanceReport {
    /// Report timestamp
    pub report_timestamp: SystemTime,
    /// System uptime
    pub system_uptime: Duration,
    /// Overall health score (0.0 - 1.0)
    pub overall_health_score: f64,
    /// Analysis performance details
    pub analysis_performance: AnalysisPerformanceReport,
    /// Component performance
    pub component_performance: HashMap<String, Duration>,
    /// Cache efficiency ratios
    pub cache_efficiency: HashMap<String, f64>,
    /// Error analysis
    pub error_analysis: ErrorAnalysisReport,
    /// Performance recommendations
    pub recommendations: Vec<String>,
}

/// Analysis performance report
#[derive(Debug, Clone)]
pub struct AnalysisPerformanceReport {
    /// Success rate
    pub success_rate: f64,
    /// Average processing time
    pub average_time: Duration,
    /// Throughput (analyses per second)
    pub throughput: f64,
    /// Efficiency score (0.0 - 1.0)
    pub efficiency_score: f64,
}

/// Error analysis report
#[derive(Debug, Clone)]
pub struct ErrorAnalysisReport {
    /// Error rate (errors per analysis)
    pub error_rate: f64,
    /// Recovery success rate
    pub recovery_rate: f64,
    /// Most common error categories
    pub most_common_errors: Vec<(String, u64)>,
}

//--------------------------------------------------------------------------------------------------
// Legacy System Metrics Types (for compatibility)
//--------------------------------------------------------------------------------------------------

/// System-wide performance metrics
#[derive(Debug, Clone)]
pub struct SystemMetrics {
    /// Diagnostic processing metrics
    pub diagnostic_metrics: DiagnosticMetricsSnapshot,
    /// AST analysis metrics
    pub ast_metrics: ASTMetricsSnapshot,
    /// Code generation metrics
    pub generation_metrics: GenerationMetricsSnapshot,
}

/// Diagnostic processing metrics snapshot
#[derive(Debug, Clone)]
pub struct DiagnosticMetricsSnapshot {
    /// Cache hit ratio (0.0-1.0)
    pub cache_hit_ratio: f64,
    /// Total diagnostics processed
    pub total_processed: u64,
    /// Parse errors encountered
    pub parse_errors: u64,
}

/// AST analysis metrics snapshot
#[derive(Debug, Clone)]
pub struct ASTMetricsSnapshot {
    /// Cache hit ratio (0.0-1.0)
    pub cache_hit_ratio: f64,
    /// Files processed
    pub files_processed: u64,
    /// AST nodes analyzed
    pub nodes_analyzed: u64,
}

/// Code generation metrics snapshot
#[derive(Debug, Clone)]
pub struct GenerationMetricsSnapshot {
    /// Total corrections generated
    pub corrections_generated: u64,
    /// Successful validations
    pub successful_validations: u64,
    /// Template cache hits
    pub template_cache_hits: u64,
}

/// Enhanced system metrics snapshot for compatibility
pub type SystemMetricsSnapshot = MetricsSnapshot;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_metrics_collector_creation() {
        let collector = SystemMetricsCollector::new();
        let uptime = collector.get_uptime().await;
        assert!(uptime < Duration::from_secs(1)); // Should be very recent
    }

    #[tokio::test]
    async fn test_analysis_metrics() {
        let collector = SystemMetricsCollector::new();

        // Record some analysis operations
        collector.record_analysis_start().await;
        collector.record_diagnostics_found(5).await;
        collector.record_corrections_generated(3).await;
        collector
            .record_analysis_complete(Duration::from_millis(500))
            .await;

        let total_analyses = collector.get_total_analyses().await;
        let total_corrections = collector.get_total_corrections().await;
        let success_rate = collector.get_success_rate().await;

        assert_eq!(total_analyses, 1);
        assert_eq!(total_corrections, 3);
        assert_eq!(success_rate, 1.0);
    }

    #[tokio::test]
    async fn test_error_tracking() {
        let collector = SystemMetricsCollector::new();

        collector
            .record_error("network", "error", "Connection failed", true, false)
            .await;
        collector
            .record_error("parsing", "warning", "Invalid syntax", false, false)
            .await;

        let snapshot = collector.get_metrics_snapshot().await;
        assert_eq!(snapshot.error_summary.total_errors, 2);
        assert_eq!(
            snapshot.error_summary.errors_by_category.get("network"),
            Some(&1)
        );
        assert_eq!(
            snapshot.error_summary.errors_by_severity.get("error"),
            Some(&1)
        );
    }

    #[tokio::test]
    async fn test_cache_performance() {
        let collector = SystemMetricsCollector::new();

        // Record cache hits and misses
        collector
            .record_cache_performance("test_cache", true, Duration::from_millis(1))
            .await;
        collector
            .record_cache_performance("test_cache", true, Duration::from_millis(2))
            .await;
        collector
            .record_cache_performance("test_cache", false, Duration::from_millis(5))
            .await;

        let snapshot = collector.get_metrics_snapshot().await;
        let cache_metrics = snapshot
            .performance_summary
            .cache_performance
            .get("test_cache")
            .unwrap();

        assert_eq!(cache_metrics.total_operations, 3);
        assert!((cache_metrics.hit_ratio - 0.6667).abs() < 0.001); // 2/3 hit ratio
    }

    #[tokio::test]
    async fn test_memory_tracking() {
        let collector = SystemMetricsCollector::new();

        collector
            .record_memory_usage(1024 * 1024, 512 * 1024, 768 * 1024)
            .await;
        collector.record_cache_size("test_cache", 256 * 1024).await;

        let snapshot = collector.get_metrics_snapshot().await;
        assert!(snapshot.resource_summary.current_memory_usage.is_some());
        assert_eq!(
            snapshot.resource_summary.cache_sizes.get("test_cache"),
            Some(&(256 * 1024))
        );
    }

    #[tokio::test]
    async fn test_throughput_measurement() {
        let collector = SystemMetricsCollector::new();

        collector.record_throughput("analysis", 10.5).await;
        collector.record_throughput("correction", 25.0).await;

        let snapshot = collector.get_metrics_snapshot().await;
        assert_eq!(snapshot.performance_summary.recent_throughput.len(), 2);
    }

    #[tokio::test]
    async fn test_performance_report_generation() {
        let collector = SystemMetricsCollector::new();

        // Add some sample data
        collector.record_analysis_start().await;
        collector
            .record_analysis_complete(Duration::from_millis(100))
            .await;
        collector.record_corrections_generated(2).await;
        collector.record_correction_applied().await;

        let report = collector.generate_performance_report().await;

        assert!(report.overall_health_score > 0.0);
        assert!(report.analysis_performance.success_rate > 0.0);
        assert!(!report.recommendations.is_empty());
    }

    #[tokio::test]
    async fn test_health_score_calculation() {
        let collector = SystemMetricsCollector::new();

        // Create a good performance scenario
        collector.record_analysis_start().await;
        collector
            .record_analysis_complete(Duration::from_millis(50))
            .await;
        collector
            .record_cache_performance("test", true, Duration::from_millis(1))
            .await;

        let report = collector.generate_performance_report().await;
        assert!(report.overall_health_score > 0.8); // Should be healthy

        // Add some errors to see health score decrease
        collector
            .record_error("test", "error", "Test error", true, false)
            .await;
        collector.record_analysis_failure().await;

        let report2 = collector.generate_performance_report().await;
        assert!(report2.overall_health_score < report.overall_health_score);
    }

    #[test]
    fn test_cache_performance_data() {
        let mut cache_data = CachePerformanceData {
            hits: 8,
            misses: 2,
            evictions: 0,
            avg_lookup_time: Duration::from_millis(5),
        };

        assert_eq!(cache_data.hit_ratio(), 0.8);

        cache_data.misses += 3;
        assert_eq!(cache_data.hit_ratio(), 8.0 / 13.0);
    }

    #[tokio::test]
    async fn test_recommendation_generation() {
        let collector = SystemMetricsCollector::new();

        // Create scenario with performance issues
        collector.record_analysis_start().await;
        collector.record_analysis_failure().await; // Low success rate
        collector
            .record_cache_performance("slow_cache", false, Duration::from_millis(100))
            .await; // Low hit rate
        collector
            .record_error("frequent", "error", "Common error", true, false)
            .await; // Errors

        let report = collector.generate_performance_report().await;

        // Should generate multiple recommendations
        assert!(report.recommendations.len() > 1);
        assert!(report
            .recommendations
            .iter()
            .any(|r| r.contains("analysis failures")));
        assert!(report
            .recommendations
            .iter()
            .any(|r| r.contains("cache") && r.contains("hit ratio")));
    }
}
