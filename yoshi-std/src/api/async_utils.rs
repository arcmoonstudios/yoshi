/* yoshi-std/src/api/async_utils.rs */
//! **Async Utilities** - Async error handling utilities for Yoshi
//!
//! This module provides async-specific utilities for error handling when the
//! 'async' feature is enabled. It includes timeout wrappers, async error
//! conversion utilities, and integration with tokio.

#[cfg(feature = "tokio")]
use crate::{Hatch, Yoshi, YoshiKind};
#[cfg(feature = "tokio")]
use tokio::time::{timeout, Duration};

/// Timeout wrapper that converts timeout errors to Yoshi errors
///
/// This function wraps an async operation with a timeout and converts
/// timeout errors into structured Yoshi errors with appropriate context.
///
/// # Examples
///
/// ```rust,no_run
/// use yoshi_std::api::async_utils::with_timeout;
/// use tokio::time::Duration;
///
/// # async fn example() -> Result<(), yoshi_std::Yoshi> {
/// let result = with_timeout(
///     Duration::from_secs(5),
///     async { tokio::time::sleep(Duration::from_secs(10)).await },
///     "long_operation"
/// ).await?;
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns a `Yoshi` error if the operation times out or if the underlying future fails.
#[cfg(feature = "tokio")]
pub async fn with_timeout<F, T>(duration: Duration, future: F, operation_name: &str) -> Hatch<T>
where
    F: std::future::Future<Output = T>,
{
    timeout(duration, future).await.map_err(|_| {
        Yoshi::new(YoshiKind::Timeout {
            operation: operation_name.into(),
            duration,
            expected_max: Some(duration),
        })
        .with_metadata("timeout_type", "tokio_timeout")
        .lay(format!(
            "Operation '{operation_name}' timed out after {duration:?}"
        ))
    })
}

/// Async error conversion utilities
///
/// This trait provides async-specific error conversion methods for
/// integrating with async operations and futures.
#[cfg(feature = "tokio")]
pub trait AsyncYoshiExt<T> {
    /// Convert a future result to a Yoshi error with async context
    ///
    /// # Errors
    ///
    /// Returns a Yoshi error if the future fails or if conversion fails.
    fn into_yoshi_async(self) -> impl std::future::Future<Output = Hatch<T>> + Send;
}

#[cfg(feature = "tokio")]
impl<T, E> AsyncYoshiExt<T> for std::future::Ready<Result<T, E>>
where
    E: std::error::Error + Send + Sync + 'static,
    T: Send,
{
    async fn into_yoshi_async(self) -> Hatch<T> {
        self.await.map_err(|e| {
            Yoshi::new(YoshiKind::Internal {
                message: e.to_string().into(),
                source: Some(Box::new(Yoshi::new(YoshiKind::Internal {
                    message: "Async operation failed".into(),
                    source: None,
                    component: Some("async_utils".into()),
                }))),
                component: Some("async_conversion".into()),
            })
        })
    }
}

/// Async retry utilities with exponential backoff
///
/// This function provides retry logic for async operations with
/// exponential backoff and Yoshi error integration.
#[cfg(feature = "tokio")]
pub async fn retry_with_backoff<F, Fut, T, E>(
    mut operation: F,
    max_retries: u32,
    initial_delay: Duration,
    operation_name: &str,
) -> Hatch<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::error::Error + Send + Sync + 'static,
{
    let mut delay = initial_delay;
    let mut last_error = None;

    for attempt in 0..=max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                last_error = Some(e);
                if attempt < max_retries {
                    tokio::time::sleep(delay).await;
                    delay *= 2; // Exponential backoff
                }
            }
        }
    }

    // All retries failed
    if let Some(e) = last_error {
        Err(Yoshi::new(YoshiKind::Internal {
            message: format!("Operation '{operation_name}' failed after {max_retries} retries")
                .into(),
            source: Some(Box::new(Yoshi::new(YoshiKind::Internal {
                message: e.to_string().into(),
                source: None,
                component: Some("retry_source".into()),
            }))),
            component: Some("async_retry".into()),
        })
        .with_metadata("max_retries", max_retries.to_string())
        .with_metadata("initial_delay_ms", initial_delay.as_millis().to_string())
        .lay(format!(
            "Async retry operation failed for '{operation_name}'"
        )))
    } else {
        Err(Yoshi::new(YoshiKind::Internal {
            message: format!("Operation '{operation_name}' failed with unknown error").into(),
            source: None,
            component: Some("async_retry".into()),
        }))
    }
}

/// Async stream error handling utilities
///
/// This module provides utilities for handling errors in async streams
/// and converting them to Yoshi errors with proper context.
#[cfg(feature = "tokio")]
pub mod stream {
    use super::{Yoshi, YoshiKind};

    /// Convert stream errors to Yoshi errors with stream context
    ///
    /// This function provides a way to handle stream errors and convert
    /// them to structured Yoshi errors with appropriate stream context.
    ///
    /// # Errors
    ///
    /// Returns a Yoshi error if stream processing fails.
    pub fn stream_error_to_yoshi<E>(error: E, stream_name: &str) -> Yoshi
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Yoshi::new(YoshiKind::Internal {
            message: format!("Stream '{stream_name}' encountered an error").into(),
            source: Some(Box::new(Yoshi::new(YoshiKind::Internal {
                message: error.to_string().into(),
                source: None,
                component: Some("stream_source".into()),
            }))),
            component: Some("async_stream".into()),
        })
        .with_metadata("stream_name", stream_name)
        .lay(format!("Async stream error in '{stream_name}'"))
    }
}

/// Async task spawning utilities with error handling
///
/// This module provides utilities for spawning async tasks with
/// proper error handling and Yoshi integration.
#[cfg(feature = "tokio")]
pub mod task {
    use super::{Hatch, Yoshi};
    use tokio::task::JoinHandle;

    /// Spawn an async task with Yoshi error handling
    ///
    /// This function spawns an async task and provides proper error
    /// handling with Yoshi error conversion.
    ///
    /// # Errors
    ///
    /// Returns a Yoshi error if task spawning fails.
    pub fn spawn_with_yoshi<F, T>(future: F, task_name: &str) -> Result<JoinHandle<Hatch<T>>, Yoshi>
    where
        F: std::future::Future<Output = Hatch<T>> + Send + 'static,
        T: Send + 'static,
    {
        let task_name = task_name.to_string();
        let handle = tokio::task::spawn(async move {
            future.await.map_err(|e| {
                e.with_metadata("task_name", &task_name)
                    .lay(format!("Task '{task_name}' failed"))
            })
        });

        Ok(handle)
    }
}
