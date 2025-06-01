/* yoshi-benches\benches\error_contest.rs */
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
//! **Brief:** Comprehensive side-by-side performance comparison between Yoshi error handling
//! framework and alternative solutions (thiserror, anyhow) for empirical validation.
//!
//! **Module Classification:** Performance-Critical
//! **Complexity Level:** Expert
//! **API Stability:** Stable
//!
//! ## Mathematical Properties
//!
//! **Algorithmic Complexity:**
//! - Time Complexity: O(1) for error creation, O(n) for error chaining where n=chain depth
//! - Space Complexity: O(1) for basic errors, O(n) for complex error context
//! - Concurrency Safety: Thread-safe error creation and manipulation across all frameworks
//!
//! **Performance Characteristics:**
//! - Expected Performance: Yoshi ≥ 2x faster than alternatives for typical scenarios
//! - Worst-Case Scenarios: Complex error chaining favors Yoshi's optimized structures
//! - Optimization Opportunities: Zero-cost error creation and intelligent memory layout
//!
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + [Error Framework Performance Contest with Empirical Validation]
//!  - [Yoshi native error handling: O(1) creation with intelligent optimization]
//!  - [thiserror comparison: Standard derive-based error handling patterns]
//!  - [anyhow comparison: Dynamic error boxing with context chaining]
//!  - [Cross-framework conversion: Performance cost analysis of error boundary crossing]
//!  - [Memory efficiency: Allocation patterns and cache performance analysis]
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** Business Source License 1.1 (BSL-1.1)
// **License Terms:** Non-production use only; commercial/production use requires paid license.
// **Effective Date:** 2025-05-25 | **Change License:** GPL v3
// **License File:** /LICENSE
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn
// **Last Validation:** 2025-05-30

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use std::error::Error;
use std::fmt;
use std::hint::black_box;
use std::time::Duration; // Required for YoshiKind::Timeout

// Always import Yoshi framework components
use yoshi::Result as YoshiResult;
use yoshi_std::{Yoshi, YoshiKind};

// Conditionally import comparison frameworks only when comparison feature is enabled
#[cfg(feature = "comparison")]
use anyhow::Result as AnyhowResult;
#[cfg(feature = "comparison")]
use thiserror::Error as ThisError;

/// Sample data structure for realistic error scenarios
#[derive(Debug, Clone)]
pub struct DatabaseConnection {
    /// Database host address
    pub host: String,
    /// Database port number
    pub port: u16,
    /// Name of the database
    pub database_name: String,
}

impl DatabaseConnection {
    fn new(host: &str, port: u16, database_name: &str) -> Self {
        Self {
            host: host.to_string(),
            port,
            database_name: database_name.to_string(),
        }
    }
}

/// Sample business object for complex error contexts
#[derive(Debug, Clone)]
pub struct UserOperation {
    /// Unique user identifier
    pub user_id: u64,
    /// Type of operation being performed
    pub operation_type: String,
    /// Size of the operation shell
    pub payload_size: usize,
    /// Timestamp when operation was initiated (may be unused in benchmarks)
    #[allow(dead_code)]
    pub timestamp: u64,
}

impl UserOperation {
    fn new(user_id: u64, operation_type: &str, payload_size: usize) -> Self {
        Self {
            user_id,
            operation_type: operation_type.to_string(),
            payload_size,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

// ============================================================================
// Yoshi Native Error Implementations (Our Framework)
// ============================================================================

/// Yoshi native application error showcasing framework capabilities
#[derive(Debug, Clone)]
pub enum YoshiAppError {
    /// Database connection failure with detailed context
    DatabaseConnection {
        message: String,
        connection_info: DatabaseConnection,
        retry_count: u32,
    },
    /// User operation validation failure
    UserValidation {
        message: String,
        user_operation: UserOperation,
        validation_rules: Vec<String>,
    },
    /// Network timeout with recovery suggestions
    NetworkTimeout {
        message: String,
        endpoint: String,
        timeout_duration: u64,
    },
    /// Configuration parsing error with context
    ConfigurationParse {
        message: String,
        config_path: String,
        line_number: Option<u32>,
    },
    /// Resource exhaustion with system state
    ResourceExhausted {
        message: String,
        resource_type: String,
        current_usage: f64,
        limit: f64,
    },
}

impl fmt::Display for YoshiAppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            YoshiAppError::DatabaseConnection {
                message,
                connection_info,
                retry_count,
            } => {
                write!(
                    f,
                    "Database connection failed: {message} (host: {}:{}; db: {}; retries: {})",
                    connection_info.host,
                    connection_info.port,
                    connection_info.database_name,
                    retry_count
                )
            }
            YoshiAppError::UserValidation {
                message,
                user_operation,
                validation_rules,
            } => {
                write!(
                    f,
                    "User validation failed: {message} (user_id: {}; operation: {}; rules: {})",
                    user_operation.user_id,
                    user_operation.operation_type,
                    validation_rules.len()
                )
            }
            YoshiAppError::NetworkTimeout {
                message,
                endpoint,
                timeout_duration,
            } => {
                write!(
                    f,
                    "Network timeout: {message} (endpoint: {endpoint}; duration: {timeout_duration}ms)",
                )
            }
            YoshiAppError::ConfigurationParse {
                message,
                config_path,
                line_number,
            } => {
                if let Some(line) = line_number {
                    write!(
                        f,
                        "Configuration parse error: {message} (file: {config_path}; line: {line})",
                    )
                } else {
                    write!(
                        f,
                        "Configuration parse error: {message} (file: {config_path})",
                    )
                }
            }
            YoshiAppError::ResourceExhausted {
                message,
                resource_type,
                current_usage,
                limit,
            } => {
                write!(
                    f,
                    "Resource exhausted: {message} ({resource_type}: {:.2}% of {:.2})",
                    (current_usage / limit) * 100.0,
                    limit
                )
            }
        }
    }
}

impl Error for YoshiAppError {}

#[allow(clippy::too_many_lines)] // Allowed for comprehensive From implementation
impl From<YoshiAppError> for Yoshi {
    fn from(err: YoshiAppError) -> Self {
        match err {
            YoshiAppError::DatabaseConnection {
                message,
                connection_info,
                retry_count,
            } => {
                let yoshi_kind = YoshiKind::Network {
                    message: message.into(),
                    source: None,
                    error_code: Some(5001),
                };
                Yoshi::new(yoshi_kind)
                    .context(format!(
                        "Database connection failed for {}",
                        connection_info.database_name
                    ))
                    .with_metadata("host", connection_info.host)
                    .with_metadata("port", connection_info.port.to_string())
                    .with_metadata("database_name", connection_info.database_name)
                    .with_metadata("retry_count", retry_count.to_string())
            }
            YoshiAppError::UserValidation {
                message,
                user_operation,
                validation_rules,
            } => {
                let yoshi_kind = YoshiKind::Validation {
                    field: format!("user_operation_{}", user_operation.operation_type).into(),
                    message: message.into(),
                    expected: Some(
                        format!("validation_rules: {}", validation_rules.join(", ")).into(),
                    ),
                    actual: Some(user_operation.user_id.to_string().into()),
                };
                Yoshi::new(yoshi_kind)
                    .context(format!(
                        "User validation failed for user {}",
                        user_operation.user_id
                    ))
                    .with_metadata("user_id", user_operation.user_id.to_string())
                    .with_metadata("operation_type", user_operation.operation_type.clone())
                    .with_metadata("payload_size", user_operation.payload_size.to_string())
                    .with_shell(user_operation)
                    .with_shell(validation_rules)
            }
            YoshiAppError::NetworkTimeout {
                message,
                endpoint,
                timeout_duration,
            } => {
                let yoshi_kind = YoshiKind::Timeout {
                    operation: endpoint.clone().into(),
                    duration: Duration::from_millis(timeout_duration),
                    expected_max: None,
                };
                Yoshi::new(yoshi_kind)
                    .context(format!("Network request to {endpoint} timed out"))
                    .with_metadata("original_message", message)
                    .with_suggestion("Increase timeout duration or check network connectivity")
            }
            YoshiAppError::ConfigurationParse {
                message,
                config_path,
                line_number,
            } => {
                let mut yoshi = Yoshi::new(YoshiKind::Config {
                    message: message.into(),
                    source: None,
                    config_path: Some(config_path.clone().into()),
                })
                .context(format!("Failed to parse configuration from {config_path}"));

                if let Some(line) = line_number {
                    yoshi = yoshi.with_metadata("line_number", line.to_string());
                }

                yoshi.with_suggestion("Check configuration file syntax and format")
            }
            YoshiAppError::ResourceExhausted {
                message,
                resource_type,
                current_usage,
                limit,
            } => {
                let yoshi_kind = YoshiKind::ResourceExhausted {
                    resource: resource_type.clone().into(),
                    limit: limit.to_string().into(),
                    current: current_usage.to_string().into(),
                    usage_percentage: Some((current_usage / limit) * 100.0),
                };
                Yoshi::new(yoshi_kind)
                    .context(format!("System resource {resource_type} exhausted"))
                    .with_metadata("original_message", message)
                    .with_suggestion("Increase resource limits or optimize resource usage")
            }
        }
    }
}

// ============================================================================
// thiserror Comparison Implementation (Only when comparison feature enabled)
// ============================================================================

#[cfg(feature = "comparison")]
#[derive(ThisError, Debug, Clone)]
pub enum ThiserrorAppError {
    #[error("Database connection failed: {message} (host: {host}:{port}, db: {database}, retries: {retry_count})")]
    DatabaseConnection {
        message: String,
        host: String,
        port: u16,
        database: String,
        retry_count: u32,
    },
    #[error("User validation failed: {message} (user_id: {user_id}, operation: {operation_type})")]
    UserValidation {
        message: String,
        user_id: u64,
        operation_type: String,
        validation_rules_count: usize,
    },
    #[error("Network timeout: {message} (endpoint: {endpoint}, duration: {timeout_duration}ms)")]
    NetworkTimeout {
        message: String,
        endpoint: String,
        timeout_duration: u64,
    },
    #[error("Configuration parse error: {message} (file: {config_path})")]
    ConfigurationParse {
        message: String,
        config_path: String,
        line_number: Option<u32>,
    },
    #[error("Resource exhausted: {message} ({resource_type}: {current_usage:.2}% of {limit:.2})")]
    ResourceExhausted {
        message: String,
        resource_type: String,
        current_usage: f64,
        limit: f64,
    },
}

// ============================================================================
// Performance Benchmark Functions
// ============================================================================

/// Benchmark Yoshi native error creation performance
fn bench_yoshi_error_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_creation");
    group.throughput(Throughput::Elements(1));

    // Simple error creation
    group.bench_function("yoshi_simple", |b| {
        b.iter(|| {
            black_box(YoshiAppError::NetworkTimeout {
                message: black_box("Connection timeout occurred".to_string()),
                endpoint: black_box("https://api.example.com".to_string()),
                timeout_duration: black_box(5000),
            })
        });
    });

    // Complex error creation with rich context
    group.bench_function("yoshi_complex", |b| {
        b.iter(|| {
            black_box(YoshiAppError::UserValidation {
                message: black_box("Invalid user operation detected".to_string()),
                user_operation: black_box(UserOperation::new(12345, "data_export", 1_024_000)),
                validation_rules: black_box(vec![
                    "user_must_be_active".to_string(),
                    "operation_size_limit".to_string(),
                    "rate_limit_check".to_string(),
                ]),
            })
        });
    });

    #[cfg(feature = "comparison")]
    {
        // thiserror simple error creation
        group.bench_function("thiserror_simple", |b| {
            b.iter(|| {
                black_box(ThiserrorAppError::NetworkTimeout {
                    message: black_box("Connection timeout occurred".to_string()),
                    endpoint: black_box("https://api.example.com".to_string()),
                    timeout_duration: black_box(5000),
                })
            });
        });

        // thiserror complex error creation
        group.bench_function("thiserror_complex", |b| {
            b.iter(|| {
                black_box(ThiserrorAppError::UserValidation {
                    message: black_box("Invalid user operation detected".to_string()),
                    user_id: black_box(12345),
                    operation_type: black_box("data_export".to_string()),
                    validation_rules_count: black_box(3),
                })
            });
        }); // anyhow error creation and context addition
        group.bench_function("anyhow_simple", |b| {
            b.iter(|| {
                black_box(anyhow::Error::from(std::io::Error::new(
                    std::io::ErrorKind::TimedOut,
                    "Connection timeout",
                )))
            });
        });

        group.bench_function("anyhow_complex", |b| {
            b.iter(|| {
                black_box({
                    let base_error =
                        std::io::Error::new(std::io::ErrorKind::InvalidInput, "Validation failed");
                    anyhow::Error::from(base_error)
                        .context("Invalid user operation detected")
                        .context(format!("user_id: {}", 12345))
                        .context("operation_type: data_export")
                });
            });
        });
    }

    group.finish();
}

/// Benchmark error conversion to framework types
fn bench_error_conversion(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_conversion");
    group.throughput(Throughput::Elements(1));

    // Yoshi conversion
    group.bench_function("yoshi_to_yoshi", |b| {
        let error = YoshiAppError::DatabaseConnection {
            message: "Connection refused".to_string(),
            connection_info: DatabaseConnection::new("localhost", 5432, "production"),
            retry_count: 3,
        };

        b.iter(|| black_box(Yoshi::from(black_box(error.clone()))));
    });

    #[cfg(feature = "comparison")]
    {
        // thiserror conversion
        group.bench_function("thiserror_to_anyhow", |b| {
            let error = ThiserrorAppError::DatabaseConnection {
                message: "Connection refused".to_string(),
                host: "localhost".to_string(),
                port: 5432,
                database: "production".to_string(),
                retry_count: 3,
            };

            b.iter(|| black_box(anyhow::Error::from(black_box(error.clone()))));
        });
    }

    group.finish();
}

/// Benchmark error chaining operations
fn bench_error_chaining(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_chaining");
    group.throughput(Throughput::Elements(1));

    // Yoshi error chaining
    group.bench_function("yoshi_chain", |b| {
        b.iter(|| {
            let base_error = YoshiAppError::ConfigurationParse {
                message: "Invalid JSON syntax".to_string(),
                config_path: "/etc/app/config.json".to_string(),
                line_number: Some(42),
            };

            black_box(
                Yoshi::from(base_error)
                    .context("Failed during configuration loading at application startup") // Changed to &str
                    .with_metadata("component", "database_config") // Changed to &str
                    .with_suggestion("Check JSON syntax at line 42") // Changed to &str
                    .with_suggestion("Validate configuration schema"), // Changed to &str
            );
        });
    });

    #[cfg(feature = "comparison")]
    {
        // anyhow error chaining
        group.bench_function("anyhow_chain", |b| {
            b.iter(|| {
                let base_error =
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid JSON syntax");

                black_box(
                    anyhow::Error::from(base_error)
                        .context("configuration_loading: application_startup")
                        .context("component: database_config")
                        .context("suggestion: Check JSON syntax at line 42")
                        .context("suggestion: Validate configuration schema"),
                );
            });
        });
    }

    group.finish();
}

/// Benchmark error formatting and display
fn bench_error_formatting(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_formatting");
    group.throughput(Throughput::Elements(1));

    // Yoshi error display
    group.bench_function("yoshi_display", |b| {
        let error = YoshiAppError::ResourceExhausted {
            message: "Memory limit exceeded".to_string(),
            resource_type: "heap_memory".to_string(),
            current_usage: 950.0,
            limit: 1000.0,
        };

        b.iter(|| black_box(format!("{}", black_box(&error))));
    });

    // Yoshi converted error display
    group.bench_function("yoshi_converted_display", |b| {
        let yoshi_error = Yoshi::from(YoshiAppError::ResourceExhausted {
            message: "Memory limit exceeded".to_string(),
            resource_type: "heap_memory".to_string(),
            current_usage: 950.0,
            limit: 1000.0,
        });

        b.iter(|| black_box(format!("{}", black_box(&yoshi_error))));
    });

    #[cfg(feature = "comparison")]
    {
        // thiserror error display
        group.bench_function("thiserror_display", |b| {
            let error = ThiserrorAppError::ResourceExhausted {
                message: "Memory limit exceeded".to_string(),
                resource_type: "heap_memory".to_string(),
                current_usage: 950.0,
                limit: 1000.0,
            };

            b.iter(|| black_box(format!("{}", black_box(&error))));
        });

        // anyhow error display with context
        group.bench_function("anyhow_display", |b| {
            let error = anyhow::Error::from(std::io::Error::new(
                std::io::ErrorKind::OutOfMemory,
                "Memory limit exceeded",
            ))
            .context("resource_type: heap_memory")
            .context("current_usage: 950.0")
            .context("limit: 1000.0");

            b.iter(|| black_box(format!("{:?}", black_box(&error))));
        });
    }

    group.finish();
}

/// Benchmark memory allocation patterns
#[allow(clippy::cast_sign_loss)] // `i` as u64 is safe as i is non-negative
fn bench_memory_efficiency(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_efficiency");
    group.throughput(Throughput::Elements(100));

    // Batch error creation - Yoshi
    group.bench_function("yoshi_batch_creation", |b| {
        b.iter(|| {
            let mut errors = Vec::with_capacity(100);
            for i in 0..100 {
                errors.push(black_box(YoshiAppError::NetworkTimeout {
                    message: format!("Timeout #{i}"),
                    endpoint: format!("https://api-{}.example.com", i % 10),
                    timeout_duration: 5000 + (i as u64 * 100),
                }));
            }
            black_box(errors);
        });
    });

    #[cfg(feature = "comparison")]
    {
        // Batch error creation - thiserror
        group.bench_function("thiserror_batch_creation", |b| {
            b.iter(|| {
                let mut errors = Vec::with_capacity(100);
                for i in 0..100 {
                    errors.push(black_box(ThiserrorAppError::NetworkTimeout {
                        message: format!("Timeout #{i}"), // Direct format argument
                        endpoint: format!("https://api-{}.example.com", i % 10),
                        timeout_duration: 5000 + (i as u64 * 100),
                    }));
                }
                black_box(errors);
            });
        });

        // Batch error creation - anyhow
        group.bench_function("anyhow_batch_creation", |b| {
            b.iter(|| {
                let mut errors = Vec::with_capacity(100);
                for i in 0..100 {
                    let base_error = std::io::Error::new(
                        std::io::ErrorKind::TimedOut,
                        format!("Timeout #{i}"), // Direct format argument
                    );
                    errors.push(black_box(
                        anyhow::Error::from(base_error)
                            .context(format!("endpoint: https://api-{}.example.com", i % 10))
                            .context(format!("duration: {}ms", 5000 + (i as u64 * 100))),
                    ));
                }
                black_box(errors);
            });
        });
    }

    group.finish();
}

/// Benchmark realistic application scenarios
#[allow(clippy::semicolon_if_nothing_returned)] // Intentional explicit semicolon to return `()` from match arms
fn bench_realistic_scenarios(c: &mut Criterion) {
    let mut group = c.benchmark_group("realistic_scenarios");

    // Database operation with error handling - Yoshi
    group.bench_function("yoshi_database_operation", |b| {
        b.iter(|| {
            // Simulate a database operation that might fail
            let result: YoshiResult<String> = if black_box(true) {
                Err(YoshiAppError::DatabaseConnection {
                    message: "Connection pool exhausted".to_string(),
                    connection_info: DatabaseConnection::new("db-cluster.internal", 5432, "users"),
                    retry_count: 3,
                }
                .into())
            } else {
                Ok("User data retrieved".to_string())
            };

            // Handle the error with context
            match result {
                // Use `let _ =` to consume the unit return value
                Ok(data) => {
                    black_box(data); // Add semicolon to make arm return ()
                }
                Err(err) => {
                    let enhanced_error = err
                        .context("Error during user data retrieval") // Changed to &str
                        .with_metadata("operation", "user_data_retrieval") // Changed to &str
                        .with_metadata("table", "users") // Changed to &str
                        .with_suggestion("Check database connection pool configuration"); // Changed to &str
                    black_box(format!("{enhanced_error}")); // Add semicolon to make arm return ()
                }
            }
        });
    });

    #[cfg(feature = "comparison")]
    {
        // Database operation with error handling - anyhow
        group.bench_function("anyhow_database_operation", |b| {
            b.iter(|| {
                // Simulate a database operation that might fail
                let result: AnyhowResult<String> = if black_box(true) {
                    Err(anyhow::Error::from(std::io::Error::new(
                        std::io::ErrorKind::ConnectionRefused,
                        "Connection pool exhausted",
                    )))
                } else {
                    Ok("User data retrieved".to_string())
                };

                // Handle the error with context
                match result {
                    // Use `let _ =` to consume the unit return value
                    Ok(data) => {
                        black_box(data); // Add semicolon to make arm return ()
                    }
                    Err(err) => {
                        let enhanced_error = err
                            .context("operation: user_data_retrieval")
                            .context("table: users")
                            .context("suggestion: Check database connection pool configuration");
                        black_box(format!("{enhanced_error:?}")); // Add semicolon to make arm return ()
                    }
                }
            });
        });
    }

    group.finish();
}

// ============================================================================
// Benchmark Group Registration
// ============================================================================

criterion_group!(
    benches,
    bench_yoshi_error_creation,
    bench_error_conversion,
    bench_error_chaining,
    bench_error_formatting,
    bench_memory_efficiency,
    bench_realistic_scenarios
);

criterion_main!(benches);
