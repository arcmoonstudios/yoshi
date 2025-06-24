/* examples/advanced_error_handling.rs */
#![deny(dead_code)]
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![allow(unused_variables)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
//! **Brief:** Advanced error handling patterns with the Yoshi framework.
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + Advanced error handling concepts and patterns
//!  - Complex error chaining and context propagation
//!  - Error recovery and retry mechanisms
//!  - Custom error types with rich metadata
//!  - Error aggregation and batch processing
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

use yoshi::*;
//--------------------------------------------------------------------------------------------------
// Advanced Error Types
//--------------------------------------------------------------------------------------------------

/// Advanced error types with rich context and recovery information.
///
/// This enum demonstrates sophisticated error handling patterns with detailed
/// metadata, recovery suggestions, and error classification.
#[derive(Debug)]
#[allow(dead_code)]
pub enum AdvancedError {
    /// Service is temporarily unavailable with retry information.
    ///
    /// Contains retry timing and service health information for intelligent recovery.
    ServiceUnavailable {
        /// Name of the unavailable service
        service: String,
        /// Reason for unavailability
        reason: String,
        /// Suggested retry delay in milliseconds
        retry_after_ms: u64,
        /// Current service health score (0-100)
        health_score: u8,
    },

    /// Resource limit exceeded with usage statistics.
    ///
    /// Provides detailed resource usage information for capacity planning.
    ResourceExhausted {
        /// Type of resource that was exhausted
        resource: String,
        /// Current usage level
        current: u64,
        /// Maximum allowed limit
        limit: u64,
        /// Impact description of the exhaustion
        impact: String,
        /// Suggested scaling factor
        scale_factor: f64,
    },

    /// Validation failed with detailed field information.
    ///
    /// Contains comprehensive validation failure details for form processing.
    ValidationFailed {
        /// Number of fields that failed validation
        field_count: usize,
        /// Summary of validation failures
        summary: String,
        /// Detailed field-level errors
        field_errors: String,
        /// Validation rule set used
        rule_set: String,
    },

    /// Business logic constraint violated.
    ///
    /// Represents domain-specific business rule violations with context.
    BusinessRuleViolation {
        /// Name of the violated business rule
        rule: String,
        /// Context where the violation occurred
        context: String,
        /// Severity level of the violation
        severity: String,
        /// Suggested corrective action
        corrective_action: String,
    },
}

/// Recovery strategy for handling different types of errors.
///
/// Defines various approaches to error recovery based on error type and context.
#[derive(Debug, Clone)]
pub enum RecoveryStrategy {
    /// Retry the operation with exponential backoff
    Retry {
        /// Maximum number of retry attempts
        max_attempts: u32,
        /// Base delay between retries
        base_delay_ms: u64,
        /// Backoff multiplier for each retry
        backoff_multiplier: f64,
    },
    /// Fall back to an alternative approach
    Fallback {
        /// Description of the fallback strategy
        strategy: String,
        /// Expected success rate of fallback
        success_rate: f64,
    },
    /// Fail fast without recovery
    FailFast {
        /// Reason for failing fast
        reason: String,
    },
    /// Circuit breaker pattern
    CircuitBreaker {
        /// Failure threshold to open circuit
        failure_threshold: u32,
        /// Timeout before attempting recovery
        timeout_ms: u64,
    },
}

//--------------------------------------------------------------------------------------------------
// Advanced Error Handling Components
//--------------------------------------------------------------------------------------------------

/// Advanced error handler with recovery strategies and metrics.
///
/// Provides sophisticated error handling capabilities including retry logic,
/// circuit breaker patterns, and error analytics.
pub struct ErrorHandler {
    /// Error occurrence statistics
    error_stats: HashMap<String, u32>,
    /// Recovery strategies by error type
    recovery_strategies: HashMap<String, RecoveryStrategy>,
    /// Circuit breaker states
    circuit_states: HashMap<String, CircuitState>,
}

/// Circuit breaker state for service protection.
///
/// Tracks the state of circuit breakers to prevent cascading failures.
#[derive(Debug, Clone)]
pub struct CircuitState {
    /// Current state of the circuit
    state: CircuitStatus,
    /// Number of consecutive failures
    failure_count: u32,
    /// Timestamp of last failure
    last_failure: Option<Instant>,
    /// Timestamp when circuit was opened
    opened_at: Option<Instant>,
}

/// Status of a circuit breaker.
///
/// Represents the current operational state of a circuit breaker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CircuitStatus {
    /// Circuit is closed, allowing requests
    Closed,
    /// Circuit is open, blocking requests
    Open,
    /// Circuit is half-open, testing recovery
    HalfOpen,
}

impl Default for ErrorHandler {
/// **default**
///
/// This function provides default functionality within the Yoshi error handling framework.
///
/// # Errors
///
/// Returns an error if the operation fails due to invalid input or system constraints.
    fn default() -> Self {
        Self::new()
    }
}

impl ErrorHandler {
    /// Creates a new error handler with default configuration.
    ///
    /// Initializes the handler with empty statistics and default recovery strategies.
    #[must_use]
    pub fn new() -> Self {
        Self {
            error_stats: HashMap::new(),
            recovery_strategies: HashMap::new(),
            circuit_states: HashMap::new(),
        }
    }

    /// Registers a recovery strategy for a specific error type.
    ///
    /// Associates error types with appropriate recovery mechanisms.
    ///
    /// # Arguments
    ///
    /// * `error_type` - The type of error to handle
    /// * `strategy` - The recovery strategy to use
    pub fn register_strategy(&mut self, error_type: &str, strategy: RecoveryStrategy) {
        self.recovery_strategies
            .insert(error_type.to_string(), strategy);
    }

    /// Handles an error using the appropriate recovery strategy.
    ///
    /// Processes errors according to registered strategies and updates metrics.
    ///
    /// # Arguments
    ///
    /// * `error` - The error to handle
    /// * `context` - Additional context information
    ///
    /// # Returns
    ///
    /// A `Hatch<RecoveryAction>` indicating the recommended recovery action.
    pub fn handle_error(&mut self, error: &Yoshi, context: &str) -> Hatch<RecoveryAction> {
        // Update error statistics
        let error_type = "advanced_error"; // Simplified for example
        *self.error_stats.entry(error_type.to_string()).or_insert(0) += 1;

        // Get recovery strategy
        let strategy = self.recovery_strategies.get(error_type).cloned().unwrap_or(
            RecoveryStrategy::FailFast {
                reason: "No recovery strategy configured".to_string(),
            },
        );

        // Execute recovery strategy
        match strategy {
            RecoveryStrategy::Retry {
                max_attempts,
                base_delay_ms,
                backoff_multiplier,
            } => Ok(RecoveryAction::Retry {
                delay_ms: base_delay_ms,
                max_attempts,
                backoff_multiplier,
            }),
            RecoveryStrategy::Fallback {
                strategy,
                success_rate,
            } => Ok(RecoveryAction::Fallback {
                strategy,
                expected_success_rate: success_rate,
            }),
            RecoveryStrategy::FailFast { reason } => Ok(RecoveryAction::FailFast { reason }),
            RecoveryStrategy::CircuitBreaker {
                failure_threshold,
                timeout_ms,
            } => self.handle_circuit_breaker(error_type, failure_threshold, timeout_ms),
        }
    }

    /// Handles circuit breaker logic for service protection.
    ///
    /// Implements circuit breaker pattern to prevent cascading failures.
    fn handle_circuit_breaker(
        &mut self,
        service: &str,
        failure_threshold: u32,
        timeout_ms: u64,
    ) -> Hatch<RecoveryAction> {
        let now = Instant::now();
        let circuit = self
            .circuit_states
            .entry(service.to_string())
            .or_insert_with(|| CircuitState {
                state: CircuitStatus::Closed,
                failure_count: 0,
                last_failure: None,
                opened_at: None,
            });

        match circuit.state {
            CircuitStatus::Closed => {
                circuit.failure_count += 1;
                circuit.last_failure = Some(now);

                if circuit.failure_count >= failure_threshold {
                    circuit.state = CircuitStatus::Open;
                    circuit.opened_at = Some(now);
                    Ok(RecoveryAction::CircuitOpen {
                        service: service.to_string(),
                        retry_after_ms: timeout_ms,
                    })
                } else {
                    Ok(RecoveryAction::Retry {
                        delay_ms: 1000,
                        max_attempts: 3,
                        backoff_multiplier: 2.0,
                    })
                }
            }
            CircuitStatus::Open => {
                if let Some(opened_at) = circuit.opened_at {
                    if now.duration_since(opened_at).as_millis() > u128::from(timeout_ms) {
                        circuit.state = CircuitStatus::HalfOpen;
                        Ok(RecoveryAction::TestRecovery {
                            service: service.to_string(),
                        })
                    } else {
                        Ok(RecoveryAction::CircuitOpen {
                            service: service.to_string(),
                            retry_after_ms: timeout_ms,
                        })
                    }
                } else {
                    Ok(RecoveryAction::FailFast {
                        reason: "Circuit breaker in invalid state".to_string(),
                    })
                }
            }
            CircuitStatus::HalfOpen => Ok(RecoveryAction::TestRecovery {
                service: service.to_string(),
            }),
        }
    }

    /// Gets error statistics for monitoring and analysis.
    ///
    /// Returns current error occurrence counts by type.
    #[must_use]
    pub const fn get_error_stats(&self) -> &HashMap<String, u32> {
        &self.error_stats
    }
}

/// Recommended recovery action based on error analysis.
///
/// Represents the action that should be taken to recover from an error.
#[derive(Debug)]
pub enum RecoveryAction {
    /// Retry the operation with specified parameters
    Retry {
        /// Delay before retry in milliseconds
        delay_ms: u64,
        /// Maximum number of attempts
        max_attempts: u32,
        /// Backoff multiplier for delays
        backoff_multiplier: f64,
    },
    /// Use fallback strategy
    Fallback {
        /// Description of fallback strategy
        strategy: String,
        /// Expected success rate
        expected_success_rate: f64,
    },
    /// Fail immediately without recovery
    FailFast {
        /// Reason for failing fast
        reason: String,
    },
    /// Circuit breaker is open
    CircuitOpen {
        /// Service name
        service: String,
        /// Time to wait before retry
        retry_after_ms: u64,
    },
    /// Test if service has recovered
    TestRecovery {
        /// Service name to test
        service: String,
    },
}

//--------------------------------------------------------------------------------------------------
// Advanced Error Processing Functions
//--------------------------------------------------------------------------------------------------

/// Processes a batch of operations with sophisticated error handling.
///
/// Demonstrates advanced error aggregation, partial success handling,
/// and intelligent retry mechanisms for batch operations.
///
/// # Arguments
///
/// * `operations` - List of operations to process
/// * `error_handler` - Error handler for recovery strategies
///
/// # Returns
///
/// A `Hatch<BatchResult>` containing results and error information.
pub fn process_batch_operations(
    operations: &[&str],
    error_handler: &mut ErrorHandler,
) -> Hatch<BatchResult> {
    let mut successful = Vec::new();
    let mut failed = Vec::new();
    let mut retry_queue = Vec::new();

    for (index, operation) in operations.iter().enumerate() {
        match simulate_operation(operation) {
            Ok(result) => {
                successful.push(OperationResult {
                    index,
                    operation: (*operation).to_string(),
                    result,
                    attempts: 1,
                });
            }
            Err(error) => {
                match error_handler.handle_error(&error, &format!("batch_operation_{index}"))? {
                    RecoveryAction::Retry {
                        delay_ms,
                        max_attempts,
                        ..
                    } => {
                        retry_queue.push(RetryOperation {
                            index,
                            operation: (*operation).to_string(),
                            error: error.clone(),
                            delay_ms,
                            max_attempts,
                            current_attempt: 1,
                        });
                    }
                    RecoveryAction::Fallback { strategy, .. } => {
                        // Attempt fallback processing
                        match simulate_fallback_operation(operation, &strategy) {
                            Ok(result) => {
                                successful.push(OperationResult {
                                    index,
                                    operation: format!("{operation} (fallback: {strategy})"),
                                    result,
                                    attempts: 1,
                                });
                            }
                            Err(fallback_error) => {
                                failed.push(FailedOperation {
                                    index,
                                    operation: (*operation).to_string(),
                                    error: fallback_error,
                                    recovery_attempted: true,
                                });
                            }
                        }
                    }
                    _ => {
                        failed.push(FailedOperation {
                            index,
                            operation: (*operation).to_string(),
                            error,
                            recovery_attempted: false,
                        });
                    }
                }
            }
        }
    }

    // Process retry queue
    for retry_op in retry_queue {
        match simulate_operation_with_retry(&retry_op) {
            Ok(result) => {
                successful.push(OperationResult {
                    index: retry_op.index,
                    operation: retry_op.operation,
                    result,
                    attempts: retry_op.current_attempt + 1,
                });
            }
            Err(error) => {
                failed.push(FailedOperation {
                    index: retry_op.index,
                    operation: retry_op.operation,
                    error,
                    recovery_attempted: true,
                });
            }
        }
    }

    let success_count = successful.len();
    let total_count = operations.len();

    Ok(BatchResult {
        successful,
        failed,
        total_operations: total_count,
        success_rate: success_count as f64 / total_count as f64,
    })
}

/// Simulates an operation that may fail.
fn simulate_operation(operation: &str) -> Hatch<String> {
    if operation.contains("fail") {
        Err(yopost!(message: "Service unavailable: operation_service - Simulated failure".into()))
    } else if operation.contains("limit") {
        Err(yopost!(message: "Resource exhausted: cpu (95/100) - Performance degradation".into()))
    } else {
        Ok(format!("Success: {operation}"))
    }
}

/// Simulates a fallback operation.
fn simulate_fallback_operation(operation: &str, strategy: &str) -> Hatch<String> {
    Ok(format!("Fallback success: {operation} using {strategy}"))
}

/// Simulates operation retry with backoff.
fn simulate_operation_with_retry(retry_op: &RetryOperation) -> Hatch<String> {
    // Simulate improved success rate on retry
    if retry_op.operation.contains("fail") && retry_op.current_attempt > 1 {
        Ok(format!("Retry success: {}", retry_op.operation))
    } else {
        Err(retry_op.error.clone())
    }
}

/// Result of a batch processing operation.
///
/// Contains comprehensive information about batch processing results.
#[derive(Debug)]
pub struct BatchResult {
    /// Successfully processed operations
    pub successful: Vec<OperationResult>,
    /// Failed operations with error details
    pub failed: Vec<FailedOperation>,
    /// Total number of operations processed
    pub total_operations: usize,
    /// Success rate as a percentage
    pub success_rate: f64,
}

/// Result of a successful operation.
///
/// Contains details about a successfully completed operation.
#[derive(Debug)]
pub struct OperationResult {
    /// Index of the operation in the batch
    pub index: usize,
    /// Description of the operation
    pub operation: String,
    /// Result of the operation
    pub result: String,
    /// Number of attempts required
    pub attempts: u32,
}

/// Information about a failed operation.
///
/// Contains details about an operation that could not be completed.
#[derive(Debug)]
pub struct FailedOperation {
    /// Index of the operation in the batch
    pub index: usize,
    /// Description of the operation
    pub operation: String,
    /// Error that caused the failure
    pub error: Yoshi,
    /// Whether recovery was attempted
    pub recovery_attempted: bool,
}

/// Operation queued for retry.
///
/// Contains retry parameters and state for failed operations.
#[derive(Debug)]
pub struct RetryOperation {
    /// Index of the operation in the batch
    pub index: usize,
    /// Description of the operation
    pub operation: String,
    /// Original error that caused the retry
    pub error: Yoshi,
    /// Delay before retry in milliseconds
    pub delay_ms: u64,
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    /// Current attempt number
    pub current_attempt: u32,
}

/// Demonstrates advanced error handling patterns.
///
/// Shows sophisticated error handling techniques including recovery strategies,
/// circuit breakers, and batch processing with partial failures.
fn demonstrate_advanced_error_handling() -> Hatch<()> {
    tracing::error!("=== Advanced Error Handling Demonstration ===");
    tracing::error!("=== Advanced Error Handling Demonstration ===");

    let mut error_handler = ErrorHandler::new();

    // Register recovery strategies
    error_handler.register_strategy(
        "ServiceUnavailable",
        RecoveryStrategy::Retry {
            max_attempts: 3,
            base_delay_ms: 1000,
            backoff_multiplier: 2.0,
        },
    );

    error_handler.register_strategy(
        "ResourceExhausted",
        RecoveryStrategy::Fallback {
            strategy: "reduced_quality_processing".to_string(),
            success_rate: 0.8,
        },
    );

    // Process batch operations
    let operations = vec![
        "process_data_1",
        "fail_operation_2",
        "process_data_3",
        "limit_operation_4",
        "process_data_5",
    ];

    match process_batch_operations(&operations, &mut error_handler) {
        Ok(result) => {
            tracing::info!("Batch processing completed:");
            tracing::info!("  Total operations: {}", result.total_operations);
            tracing::error!("  Failed: {}", result.failed.len());
            tracing::info!("  Failed: {}", result.failed.len());
            tracing::info!("  Success rate: {:.2}%", result.success_rate * 100.0);

            for success in &result.successful {
                tracing::info!("  ✓ [{}] {} (attempts: {})", success.index, success.operation, success.attempts
                );
            }

            for failure in &result.failed {
                tracing::error!(
                    "  ✗ [{}] {} - {}",
                    failure.index,
                    failure.operation,
                    failure.error
                );
                tracing::info!("  ✗ [{}] {} - {}", failure.index, failure.operation, failure.error
                );
            }
        }
        Err(e) => {
            tracing::error!("Batch processing failed: {e}");
            tracing::info!("Batch processing failed: {e}");
        }
    }

    tracing::error!("\nError Statistics:");
    tracing::info!("\nError Statistics:");
    for (error_type, count) in error_handler.get_error_stats() {
        tracing::error!("  {error_type}: {count} occurrences");
        tracing::info!("  {error_type}: {count} occurrences");
    }

    Ok(())
}

/// Main function demonstrating advanced error handling concepts.
///
/// Runs comprehensive examples of sophisticated error handling patterns
/// and recovery strategies using the Yoshi framework.
pub fn main() -> Hatch<()> {
    // Initialize logging
    env_logger::init();

    // Run the demonstration
    demonstrate_advanced_error_handling()?;

    tracing::error!("\n=== Advanced Error Handling Complete ===");
    Ok(())
}
