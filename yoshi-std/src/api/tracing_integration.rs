/* yoshi-std/src/api/tracing_integration.rs */
//! **Tracing Integration** - Integration with the tracing ecosystem
//!
//! This module provides integration utilities for the tracing crate,
//! enabling structured logging and observability for Yoshi errors.

#[cfg(feature = "tracing")]
use crate::Yoshi;

/// Log a Yoshi error using tracing with appropriate level based on severity
///
/// This function automatically selects the appropriate tracing level
/// based on the error's severity score, providing consistent logging
/// behavior across the application.
///
/// # Examples
///
/// ```rust
/// use yoshi_std::{Yoshi, YoshiKind};
/// use yoshi_std::api::tracing_integration::trace_error;
///
/// let error = Yoshi::new(YoshiKind::Internal {
///     message: "Database connection failed".into(),
///     source: None,
///     component: Some("user_service".into()),
/// });
///
/// trace_error(&error);
/// ```
#[cfg(feature = "tracing")]
pub fn trace_error(error: &Yoshi) {
    let severity = error.kind().severity();

    match severity {
        0..=20 => tracing::debug!("Low severity error: {}", error),
        21..=40 => tracing::info!("Medium severity error: {}", error),
        41..=60 => tracing::warn!("High severity error: {}", error),
        61..=80 => tracing::error!("Critical error: {}", error),
        _ => tracing::error!("Fatal error: {}", error),
    }
}

/// Log a Yoshi error with structured fields
///
/// This function logs a Yoshi error with structured tracing fields,
/// making it easier to query and analyze logs in observability systems.
///
/// # Examples
///
/// ```rust
/// use yoshi_std::{Yoshi, YoshiKind};
/// use yoshi_std::api::tracing_integration::trace_error_structured;
///
/// let error = Yoshi::new(YoshiKind::Network {
///     message: "Connection timeout".into(),
///     source: None,
///     error_code: Some(408),
/// });
///
/// trace_error_structured(&error, "user_request");
/// ```
#[cfg(feature = "tracing")]
pub fn trace_error_structured(error: &Yoshi, context: &str) {
    let severity = error.kind().severity();
    let error_id = error.instance_id();
    let error_kind = format!("{:?}", error.kind());

    match severity {
        0..=20 => tracing::debug!(
            error_id = error_id,
            error_kind = error_kind,
            context = context,
            severity = severity,
            "Low severity error: {}",
            error
        ),
        21..=40 => tracing::info!(
            error_id = error_id,
            error_kind = error_kind,
            context = context,
            severity = severity,
            "Medium severity error: {}",
            error
        ),
        41..=60 => tracing::warn!(
            error_id = error_id,
            error_kind = error_kind,
            context = context,
            severity = severity,
            "High severity error: {}",
            error
        ),
        61..=80 => tracing::error!(
            error_id = error_id,
            error_kind = error_kind,
            context = context,
            severity = severity,
            "Critical error: {}",
            error
        ),
        _ => tracing::error!(
            error_id = error_id,
            error_kind = error_kind,
            context = context,
            severity = severity,
            "Fatal error: {}",
            error
        ),
    }
}

/// Create a tracing span for error context
///
/// This function creates a tracing span that can be used to provide
/// context for operations that might produce Yoshi errors.
///
/// # Examples
///
/// ```rust
/// use yoshi_std::api::tracing_integration::error_span;
/// use tracing::Instrument;
///
/// async fn risky_operation() -> Result<String, yoshi_std::Yoshi> {
///     // Operation implementation
///     Ok("success".to_string())
/// }
///
/// # async fn example() {
/// let span = error_span("database_operation", "user_service");
/// let result = risky_operation().instrument(span).await;
/// # }
/// ```
#[cfg(feature = "tracing")]
pub fn error_span(operation: &str, component: &str) -> tracing::Span {
    tracing::info_span!(
        "yoshi_operation",
        operation = operation,
        component = component,
        error_id = tracing::field::Empty,
        severity = tracing::field::Empty,
    )
}

/// Record error information in the current tracing span
///
/// This function records error information as fields in the current
/// tracing span, providing context for error analysis.
///
/// # Examples
///
/// ```rust
/// use yoshi_std::{Yoshi, YoshiKind};
/// use yoshi_std::api::tracing_integration::{error_span, record_error_in_span};
/// use tracing::Instrument;
///
/// # async fn example() {
/// let span = error_span("user_lookup", "auth_service");
/// let _guard = span.enter();
///
/// let error = Yoshi::new(YoshiKind::NotFound {
///     resource_type: "user".into(),
///     identifier: "user123".into(),
///     search_locations: None,
/// });
///
/// record_error_in_span(&error);
/// # }
/// ```
#[cfg(feature = "tracing")]
pub fn record_error_in_span(error: &Yoshi) {
    let current_span = tracing::Span::current();
    current_span.record("error_id", error.instance_id());
    current_span.record("severity", error.kind().severity());
}

/// Tracing subscriber utilities for Yoshi errors
///
/// This module provides utilities for working with tracing
/// in the context of Yoshi errors.
#[cfg(feature = "tracing")]
pub mod subscriber {
    /// Get the current tracing subscriber information
    ///
    /// This function provides information about the current tracing
    /// configuration for debugging purposes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yoshi_std::api::tracing_integration::subscriber::get_subscriber_info;
    ///
    /// let info = get_subscriber_info();
    /// println!("Tracing subscriber info: {}", info);
    /// ```
    #[must_use]
    pub fn get_subscriber_info() -> String {
        "Tracing subscriber utilities for Yoshi errors".to_string()
    }
}

/// Error metrics collection for observability
///
/// This module provides utilities for collecting metrics about Yoshi
/// errors for observability and monitoring purposes.
#[cfg(feature = "tracing")]
pub mod metrics {
    use super::Yoshi;

    /// Record error metrics for observability
    ///
    /// This function records metrics about Yoshi errors that can be
    /// consumed by observability systems.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yoshi_std::{Yoshi, YoshiKind};
    /// use yoshi_std::api::tracing_integration::metrics::record_error_metrics;
    ///
    /// let error = Yoshi::new(YoshiKind::Internal {
    ///     message: "Service unavailable".into(),
    ///     source: None,
    ///     component: Some("payment_service".into()),
    /// });
    ///
    /// record_error_metrics(&error, "payment_processing");
    /// ```
    pub fn record_error_metrics(error: &Yoshi, operation: &str) {
        let severity = error.kind().severity();
        let error_kind = format!("{:?}", error.kind());

        tracing::info!(
            target: "yoshi_metrics",
            operation = operation,
            error_kind = error_kind,
            severity = severity,
            error_id = error.instance_id(),
            "Error metrics recorded"
        );
    }
}
