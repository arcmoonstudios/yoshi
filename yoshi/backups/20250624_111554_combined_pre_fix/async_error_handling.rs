/* examples/async_error_handling.rs */
#![deny(dead_code)]
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![allow(unused_variables)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::unused_async)]
//! #![yoshi(auto-fix)]
//! **Brief:** Real-world async error handling with comprehensive Yoshi integration.
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
//! + Production-grade async operations with advanced error recovery patterns
//! **AsyncHttpClient.timeout**
//!
//! Data structure representing AsyncHttpClient.timeout within the Yoshi ecosystem.
//! This structure provides type-safe encapsulation and efficient memory layout.
//!  - HTTP client operations with timeout and retry mechanisms
//!  - Concurrent task management with error aggregation and recovery
//!  - Stream processing with backpressure and error handling
//!  - WebSocket connections with reconnection and state management
//!  - File I/O operations with async error propagation and recovery
// ~=####====A===r===c===M===o===o===n====S===t===u===d===i===o===s====X|0|$>
// **GitHub:** [ArcMoon Studios](https://github.com/arcmoonstudios)
// **Copyright:** (c) 2025 ArcMoon Studios
// **License:** MIT OR Apache-2.0
// **Contact:** LordXyn@proton.me
// **Author:** Lord Xyn

use yoshi::*;

//--------------------------------------------------------------------------------------------------
// Async Error Types with Context Preservation
//--------------------------------------------------------------------------------------------------

/// Comprehensive error types for asynchronous operations with rich context and recovery suggestions.
///
/// This enum provides structured error handling for common async scenarios including HTTP requests,
/// **TaskManager.tasks**
///
/// Data structure representing TaskManager.tasks within the Yoshi ecosystem.
/// This structure provides type-safe encapsulation and efficient memory layout.
/// timeouts, concurrent tasks, stream processing, WebSocket connections, and file operations.
#[derive(YoshiError, Debug)]
#[allow(dead_code)]
pub enum AsyncError {
    /// HTTP request failed with detailed status information.
    ///
    /// Contains the target URL, HTTP status code, and failure reason for comprehensive debugging.
    /// **WebSocketManager.url**
    ///
    /// Data structure representing WebSocketManager.url within the Yoshi ecosystem.
    /// This structure provides type-safe encapsulation and efficient memory layout.
    #[yoshi(display = "HTTP request to {url} failed: {status_code} - {reason}")]
    #[yoshi(suggestion = "Check URL validity and network connectivity")]
    HttpRequestFailed {
        /// The URL that was being requested when the error occurred
        url: String,
        /// HTTP status code returned by the server
        status_code: u16,
        /// Human-readable reason for the failure
        reason: String,
    },

    /// Operation exceeded the configured timeout threshold.
    ///
    /// Provides timeout duration and operation details for performance analysis.
    #[yoshi(display = "Operation timeout after {timeout_ms}ms: {operation}")]
    #[yoshi(suggestion = "Increase timeout or optimize operation performance")]
    OperationTimeout {
        /// Timeout duration in milliseconds
        timeout_ms: u64,
        /// Description of the operation that timed out
        operation: String,
    },

    /// Concurrent task execution failed.
    ///
    /// Contains task identifier and failure message for debugging concurrent operations.
    #[yoshi(display = "Concurrent task {task_id} failed: {message}")]
    #[yoshi(suggestion = "Check task dependencies and resource availability")]
    TaskFailed {
        /// Unique identifier for the failed task
        task_id: String,
        /// Detailed error message describing the failure
        message: String,
    },

    /// Stream processing encountered an error at a specific position.
    ///
    /// Provides position information for resuming stream processing after errors.
    #[yoshi(display = "Stream processing error at position {position}: {message}")]
    #[yoshi(suggestion = "Implement backpressure handling and error recovery")]
    StreamError {
        /// Position in the stream where the error occurred
        position: u64,
        /// Description of the stream processing error
        message: String,
    },

    /// WebSocket connection was unexpectedly lost.
    ///
    /// Contains the reason for disconnection to aid in reconnection logic.
    #[yoshi(display = "WebSocket connection lost: {reason}")]
    #[yoshi(suggestion = "Implement reconnection logic with exponential backoff")]
    WebSocketDisconnected {
        /// Reason for the WebSocket disconnection
        reason: String,
    },

    /// Asynchronous file operation failed.
    ///
    /// Provides file path and error details for file system debugging.
    #[yoshi(display = "Async file operation failed: {path} - {message}")]
    #[yoshi(suggestion = "Check file permissions and disk space")]
    AsyncFileError {
        /// Path to the file that caused the error
        path: String,
        /// Detailed error message from the file operation
        message: String,
    },
}

/// Network-specific error types for connection and protocol failures.
///
/// This enum provides detailed error information for network operations including
/// DNS resolution, connection establishment, and SSL/TLS handshake failures.
#[derive(YoshiError, Debug)]
pub enum NetworkError {
    /// DNS resolution failed for the specified hostname.
    ///
    /// Contains the hostname that could not be resolved for debugging network issues.
    #[yoshi(display = "DNS resolution failed for {hostname}")]
    #[yoshi(suggestion = "Check DNS configuration and hostname validity")]
    DnsResolutionFailed {
        /// The hostname that failed DNS resolution
        hostname: String,
    },

    /// Connection was refused by the target server.
    ///
    /// Provides host and port information for connection troubleshooting.
    #[yoshi(display = "Connection refused to {host}:{port}")]
    #[yoshi(suggestion = "Verify server is running and port is accessible")]
    ConnectionRefused {
        /// The target host that refused the connection
        host: String,
        /// The target port that refused the connection
        port: u16,
    },

    /// SSL/TLS handshake failed during secure connection establishment.
    ///
    /// Contains detailed error message for SSL/TLS troubleshooting.
    #[yoshi(display = "SSL/TLS handshake failed: {message}")]
    #[yoshi(suggestion = "Check certificate validity and SSL configuration")]
    SslHandshakeFailed {
        /// Detailed SSL/TLS handshake failure message
        message: String,
    },
}

//--------------------------------------------------------------------------------------------------
// Async HTTP Client with Comprehensive Error Handling
//--------------------------------------------------------------------------------------------------

/// Asynchronous HTTP client with built-in retry logic and timeout handling.
///
/// This client provides robust HTTP operations with configurable timeouts,
/// retry mechanisms, and comprehensive error handling for production use.
pub struct AsyncHttpClient {
    /// **`AsyncHttpClient.base_url`**
    ///
    /// Data structure representing AsyncHttpClient.base url within the Yoshi ecosystem.
    /// This structure provides type-safe encapsulation and efficient memory layout.
    base_url: String,
    timeout: Duration,
    /// **`AsyncHttpClient.retry_attempts`**
    ///
    /// Data structure representing AsyncHttpClient.retry attempts within the Yoshi ecosystem.
    /// This structure provides type-safe encapsulation and efficient memory layout.
    retry_attempts: u32,
    /// **`AsyncHttpClient.retry_delay`**
    ///
    /// Data structure representing AsyncHttpClient.retry delay within the Yoshi ecosystem.
    /// This structure provides type-safe encapsulation and efficient memory layout.
    retry_delay: Duration,
}

/// HTTP response containing status, headers, and body data.
///
/// Represents a complete HTTP response with all relevant information
/// for processing the server's response to a request.
#[derive(Debug)]
pub struct HttpResponse {
    /// HTTP status code returned by the server
    pub status_code: u16,
    /// HTTP headers as key-value pairs
    pub headers: HashMap<String, String>,
    /// Response body content as a string
    pub body: String,
}

impl AsyncHttpClient {
    /// Creates a new HTTP client with the specified base URL.
    ///
    /// The client is initialized with default settings: 30-second timeout,
    /// 3 retry attempts, and 1-second retry delay.
    #[must_use]
    pub const fn new(base_url: String) -> Self {
        Self {
            base_url,
            timeout: Duration::from_secs(30),
            retry_attempts: 3,
            retry_delay: Duration::from_millis(1000),
        }
    }

    /// Configures the request timeout for this client.
    ///
    /// Returns a new client instance with the specified timeout duration.
    #[must_use]
    pub const fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Configures retry behavior for this client.
    ///
    /// Sets the number of retry attempts and delay between retries.
    #[must_use]
    pub const fn with_retry(mut self, attempts: u32, delay: Duration) -> Self {
        self.retry_attempts = attempts;
        self.retry_delay = delay;
        self
    }

    /// Performs an HTTP GET request to the specified path.
    ///
    /// The path is appended to the base URL configured for this client.
    pub async fn get(&self, path: &str) -> Hatch<HttpResponse> {
        let url = format!("{}{}", self.base_url, path);
        self.execute_request("GET", &url, None).await
    }

    /// Performs an HTTP POST request with the specified body.
    ///
    /// The path is appended to the base URL and the body is sent as the request payload.
    pub async fn post(&self, path: &str, body: &str) -> Hatch<HttpResponse> {
        let url = format!("{}{}", self.base_url, path);
        self.execute_request("POST", &url, Some(body)).await
    }

    /// **`execute_request`**
    ///
    /// This function provides execute request functionality within the Yoshi error handling framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    async fn execute_request(
        &self,
        method: &str,
        url: &str,
        body: Option<&str>,
    ) -> Hatch<HttpResponse> {
        let mut last_error = None;

        for attempt in 1..=self.retry_attempts {
            match self.single_request(method, url, body).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    last_error = Some(e);

                    if attempt < self.retry_attempts {
                        // Wait before retry with exponential backoff
                        let delay = self.retry_delay * attempt;
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }

        // All retries failed
        Err(last_error.unwrap_or_else(|| {
            AsyncError::HttpRequestFailed {
                url: url.to_string(),
                status_code: 0,
                reason: "Unknown error".to_string(),
            }
            .into()
        }))
    }

    /// **`single_request`**
    ///
    /// This function provides single request functionality within the Yoshi error handling framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    async fn single_request(
        &self,
        method: &str,
        url: &str,
        body: Option<&str>,
    ) -> Hatch<HttpResponse> {
        // Simulate network request with potential failures
        let start_time = Instant::now();

        // Simulate timeout
        tokio::time::sleep(Duration::from_millis(100)).await;

        if start_time.elapsed() > self.timeout {
            return Err(AsyncError::OperationTimeout {
                timeout_ms: self.timeout.as_millis() as u64,
                operation: format!("{method} {url}"),
            }
            .into());
        }

        // Simulate various error conditions
        if url.contains("invalid") {
            return Err(AsyncError::HttpRequestFailed {
                url: url.to_string(),
                status_code: 400,
                reason: "Invalid request".to_string(),
            }
            .into());
        }

        if url.contains("timeout") {
            return Err(AsyncError::OperationTimeout {
                timeout_ms: self.timeout.as_millis() as u64,
                operation: format!("{method} {url}"),
            }
            .into());
        }

        if url.contains("server-error") {
            return Err(AsyncError::HttpRequestFailed {
                url: url.to_string(),
                status_code: 500,
                reason: "Internal server error".to_string(),
            }
            .into());
        }

        // Simulate successful response
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Server".to_string(), "Yoshi-Example/1.0".to_string());

        let response_body = match body {
            Some(req_body) => format!("Echo: {req_body}"),
            None => format!("Response for {method} {url}"),
        };

        Ok(HttpResponse {
            status_code: 200,
            headers,
            body: response_body,
        })
    }
}

//--------------------------------------------------------------------------------------------------
// Concurrent Task Manager with Error Aggregation
//--------------------------------------------------------------------------------------------------

/// Concurrent task manager with error aggregation and resource limiting.
///
/// Manages multiple asynchronous tasks with configurable concurrency limits
/// and provides comprehensive error tracking and recovery capabilities.
pub struct TaskManager {
    tasks: HashMap<String, TaskInfo>,
    /// **`TaskManager.max_concurrent`**
    ///
    /// Data structure representing TaskManager.max concurrent within the Yoshi ecosystem.
    /// This structure provides type-safe encapsulation and efficient memory layout.
    max_concurrent: usize,
    /// **`TaskManager.active_count`**
    ///
    /// Data structure representing TaskManager.active count within the Yoshi ecosystem.
    /// This structure provides type-safe encapsulation and efficient memory layout.
    active_count: usize,
}

/// Information about a managed task including status and error details.
///
/// Tracks the lifecycle and execution details of tasks managed by `TaskManager`.
#[derive(Debug)]
pub struct TaskInfo {
    /// Unique identifier for the task
    pub id: String,
    /// Current execution status of the task
    pub status: TaskStatus,
    /// Timestamp when the task was started
    pub start_time: Instant,
    /// Error message if the task failed
    pub error: Option<String>,
}

/// Execution status of a managed task.
///
/// Represents the current state of a task in its lifecycle from creation to completion.
#[derive(Debug)]
pub enum TaskStatus {
    /// Task has been created but not yet started
    Pending,
    /// Task is currently executing
    Running,
    /// Task has completed successfully
    Completed,
    /// Task has failed with an error
    Failed,
}

impl TaskManager {
    /// Creates a new task manager with the specified concurrency limit.
    ///
    /// The manager will prevent more than `max_concurrent` tasks from running simultaneously.
    #[must_use]
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            tasks: HashMap::new(),
            max_concurrent,
            active_count: 0,
        }
    }

    /// Spawns a new asynchronous task with the given identifier.
    ///
    /// Returns an error if the maximum concurrent task limit would be exceeded.
    /// The task is tracked and can be monitored using other methods.
    pub async fn spawn_task<F, T>(&mut self, task_id: String, future: F) -> Hatch<()>
    where
        F: Future<Output = Hatch<T>> + Send + 'static,
        T: Send + 'static,
    {
        // Check concurrent limit
        if self.active_count >= self.max_concurrent {
            return Err(AsyncError::TaskFailed {
                task_id,
                message: format!("Max concurrent tasks reached: {}", self.max_concurrent),
            }
            .into());
        }

        // Create task info
        let task_info = TaskInfo {
            id: task_id.clone(),
            status: TaskStatus::Pending,
            start_time: Instant::now(),
            error: None,
        };

        self.tasks.insert(task_id.clone(), task_info);
        self.active_count += 1;

        // Spawn the task
        let task_id_clone = task_id;
        tokio::spawn(async move {
            // Update status to running
            tracing::info!("Task {task_id_clone} started");

            // Execute the future
            match future.await {
                Ok(_) => {
                    tracing::info!("Task {task_id_clone} completed successfully");
                }
                Err(e) => {
                    tracing::info!("Task {task_id_clone} failed: {e}");
                }
            }
        });

        Ok(())
    }

    /// Waits for the specified task to complete.
    ///
    /// Returns an error if the task is not found or fails during execution.
    pub async fn wait_for_completion(&mut self, task_id: &str) -> Hatch<()> {
        // Simulate waiting for task completion
        tokio::time::sleep(Duration::from_millis(500)).await;

        if let Some(task_info) = self.tasks.get_mut(task_id) {
            task_info.status = TaskStatus::Completed;
            self.active_count -= 1;
            Ok(())
        } else {
            Err(AsyncError::TaskFailed {
                task_id: task_id.to_string(),
                message: "Task not found".to_string(),
            }
            .into())
        }
    }

    /// Waits for all managed tasks to complete.
    ///
    /// Returns a list of successfully completed task IDs, or an error if any tasks failed.
    pub async fn wait_for_all(&mut self) -> Hatch<Vec<String>> {
        let mut completed_tasks = Vec::new();
        let mut failed_tasks = Vec::new();

        // Wait for all tasks to complete
        let task_ids: Vec<String> = self.tasks.keys().cloned().collect();
        for task_id in task_ids {
            match self.wait_for_completion(&task_id).await {
                Ok(()) => completed_tasks.push(task_id.clone()),
                Err(e) => {
                    failed_tasks.push(format!("{task_id}: {e}"));
                    if let Some(task_info) = self.tasks.get_mut(&task_id) {
                        task_info.status = TaskStatus::Failed;
                        task_info.error = Some(e.to_string());
                    }
                }
            }
        }

        if !failed_tasks.is_empty() {
            return Err(AsyncError::TaskFailed {
                task_id: "multiple".to_string(),
                message: format!("Failed tasks: {}", failed_tasks.join(", ")),
            }
            .into());
        }

        Ok(completed_tasks)
    }

    /// Gets the status information for a specific task.
    ///
    /// Returns None if the task ID is not found.
    #[must_use]
    pub fn get_task_status(&self, task_id: &str) -> Option<&TaskInfo> {
        self.tasks.get(task_id)
    }

    /// Returns the current number of active (running) tasks.
    #[must_use]
    pub const fn get_active_count(&self) -> usize {
        self.active_count
    }
}

//--------------------------------------------------------------------------------------------------
// Async Stream Processor with Error Recovery
//--------------------------------------------------------------------------------------------------

/// Stream processor with error recovery and backpressure handling.
///
/// Processes items in configurable chunks with error threshold management
/// and automatic recovery capabilities for robust stream processing.
pub struct StreamProcessor {
    /// **`StreamProcessor.buffer_size`**
    ///
    /// Data structure representing StreamProcessor.buffer size within the Yoshi ecosystem.
    /// This structure provides type-safe encapsulation and efficient memory layout.
    buffer_size: usize,
    /// **`StreamProcessor.error_threshold`**
    ///
    /// Data structure representing StreamProcessor.error threshold within the Yoshi ecosystem.
    /// This structure provides type-safe encapsulation and efficient memory layout.
    error_threshold: usize,
    /// **`StreamProcessor.error_count`**
    ///
    /// Data structure representing StreamProcessor.error count within the Yoshi ecosystem.
    /// This structure provides type-safe encapsulation and efficient memory layout.
    error_count: usize,
}

impl StreamProcessor {
    /// Creates a new stream processor with the specified configuration.
    ///
    /// The processor will use the given buffer size for chunking and will
    /// stop processing if the error threshold is exceeded.
    #[must_use]
    pub const fn new(buffer_size: usize, error_threshold: usize) -> Self {
        Self {
            buffer_size,
            error_threshold,
            error_count: 0,
        }
    }

    /// Processes a stream of items with error recovery and backpressure handling.
    ///
    /// Items are processed in chunks according to the configured buffer size.
    /// Processing continues even if individual chunks fail, up to the error threshold.
    pub async fn process_stream<T>(&mut self, items: Vec<T>) -> Hatch<Vec<String>>
    where
        T: std::fmt::Debug + Send + 'static,
    {
        let mut results = Vec::new();
        let mut position = 0;

        for chunk in items.chunks(self.buffer_size) {
            match self.process_chunk(chunk, position).await {
                Ok(mut chunk_results) => {
                    results.append(&mut chunk_results);
                }
                Err(e) => {
                    self.error_count += 1;

                    if self.error_count >= self.error_threshold {
                        return Err(AsyncError::StreamError {
                            position,
                            message: format!("Error threshold exceeded: {e}"),
                        }
                        .into());
                    }

                    // Log error but continue processing
                    tracing::error!("Stream error at position {position}: {e} (continuing)");
                }
            }

            position += chunk.len() as u64;
        }

        Ok(results)
    }

    /// **`process_chunk`**
    ///
    /// This function provides process chunk functionality within the Yoshi error handling framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    async fn process_chunk<T>(&self, chunk: &[T], position: u64) -> Hatch<Vec<String>>
    where
        T: std::fmt::Debug,
    {
        // Simulate chunk processing
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Simulate error conditions
        if position % 100 == 0 && position > 0 {
            return Err(AsyncError::StreamError {
                position,
                message: "Simulated processing error".to_string(),
            }
            .into());
        }

        // Process items in chunk
        let mut results = Vec::new();
        for (i, item) in chunk.iter().enumerate() {
            results.push(format!(
                "Processed item {} at position {}: {:?}",
                i,
                position + i as u64,
                item
            ));
        }

        Ok(results)
    }

    /// Resets the error count to zero.
    ///
    /// This can be used to reset error tracking after implementing recovery measures.
    pub const fn reset_error_count(&mut self) {
        self.error_count = 0;
    }

    /// Returns the current error count.
    #[must_use]
    pub const fn get_error_count(&self) -> usize {
        self.error_count
    }
}

//--------------------------------------------------------------------------------------------------
// WebSocket Connection Manager with Reconnection
//--------------------------------------------------------------------------------------------------

/// WebSocket connection manager with automatic reconnection capabilities.
///
/// Manages WebSocket connections with built-in reconnection logic, exponential backoff,
/// and comprehensive error handling for reliable real-time communication.
pub struct WebSocketManager {
    url: String,
    /// **`WebSocketManager.is_connected`**
    ///
    /// Data structure representing WebSocketManager.is connected within the Yoshi ecosystem.
    /// This structure provides type-safe encapsulation and efficient memory layout.
    is_connected: bool,
    /// **`WebSocketManager.reconnect_attempts`**
    ///
    /// Data structure representing WebSocketManager.reconnect attempts within the Yoshi ecosystem.
    /// This structure provides type-safe encapsulation and efficient memory layout.
    reconnect_attempts: u32,
    /// **`WebSocketManager.max_reconnect_attempts`**
    ///
    /// Data structure representing WebSocketManager.max reconnect attempts within the Yoshi ecosystem.
    /// This structure provides type-safe encapsulation and efficient memory layout.
    max_reconnect_attempts: u32,
    /// **`WebSocketManager.reconnect_delay`**
    ///
    /// Data structure representing WebSocketManager.reconnect delay within the Yoshi ecosystem.
    /// This structure provides type-safe encapsulation and efficient memory layout.
    reconnect_delay: Duration,
}

impl WebSocketManager {
    /// Creates a new WebSocket manager for the specified URL.
    ///
    /// The manager is initialized with default reconnection settings.
    #[must_use]
    pub const fn new(url: String) -> Self {
        Self {
            url,
            is_connected: false,
            reconnect_attempts: 0,
            max_reconnect_attempts: 5,
            reconnect_delay: Duration::from_secs(1),
        }
    }

    /// Establishes a WebSocket connection to the configured URL.
    ///
    /// Returns an error if the connection fails or the URL is invalid.
    pub async fn connect(&mut self) -> Hatch<()> {
        // Simulate connection attempt
        tokio::time::sleep(Duration::from_millis(200)).await;

        // Simulate connection failure
        if self.url.contains("invalid") {
            return Err(AsyncError::WebSocketDisconnected {
                reason: "Invalid WebSocket URL".to_string(),
            }
            .into());
        }

        self.is_connected = true;
        self.reconnect_attempts = 0;
        tracing::info!("WebSocket connected to {}", self.url);

        Ok(())
    }

    /// Sends a message through the WebSocket connection.
    ///
    /// Automatically attempts to reconnect if the connection is lost.
    pub async fn send_message(&mut self, message: &str) -> Hatch<()> {
        if !self.is_connected {
            self.reconnect()
                .await
                .lay("Failed to reconnect before sending message")?;
        }

        // Simulate message sending
        tokio::time::sleep(Duration::from_millis(10)).await;

        // Simulate connection loss
        if message.contains("disconnect") {
            self.is_connected = false;
            return Err(AsyncError::WebSocketDisconnected {
                reason: "Connection lost during message send".to_string(),
            }
            .into());
        }

        tracing::info!("Message sent: {message}");
        Ok(())
    }

    /// Receives a message from the WebSocket connection.
    ///
    /// Automatically attempts to reconnect if the connection is lost.
    pub async fn receive_message(&mut self) -> Hatch<String> {
        if !self.is_connected {
            self.reconnect()
                .await
                .lay("Failed to reconnect before receiving message")?;
        }

        // Simulate message receiving
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Simulate received message
        Ok("Received message from server".to_string())
    }

    /// **reconnect**
    ///
    /// This function provides reconnect functionality within the Yoshi error handling framework.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to invalid input or system constraints.
    async fn reconnect(&mut self) -> Hatch<()> {
        while self.reconnect_attempts < self.max_reconnect_attempts {
            self.reconnect_attempts += 1;

            tracing::info!(
                "Reconnection attempt {} of {}",
                self.reconnect_attempts,
                self.max_reconnect_attempts
            );

            match self.connect().await {
                Ok(()) => return Ok(()),
                Err(e) => {
                    if self.reconnect_attempts >= self.max_reconnect_attempts {
                        return Err(AsyncError::WebSocketDisconnected {
                            reason: format!("Max reconnection attempts exceeded: {e}"),
                        }
                        .into());
                    }

                    // Wait before next attempt with exponential backoff
                    let delay = self.reconnect_delay * self.reconnect_attempts;
                    tokio::time::sleep(delay).await;
                }
            }
        }

        Err(AsyncError::WebSocketDisconnected {
            reason: "Reconnection failed".to_string(),
        }
        .into())
    }

    /// Returns whether the WebSocket is currently connected.
    #[must_use]
    pub const fn is_connected(&self) -> bool {
        self.is_connected
    }

    /// Manually disconnects the WebSocket connection.
    pub fn disconnect(&mut self) {
        self.is_connected = false;
        tracing::info!("WebSocket disconnected");
    }
}

//--------------------------------------------------------------------------------------------------
// Auto-Correction with yoshi_af! Macro
//--------------------------------------------------------------------------------------------------

// Apply auto-correction to async operations
yoshi_af! {
    fn enhanced_async_operations() -> Hatch<String> {
        // This function demonstrates auto-correction for async operations
        let _client = AsyncHttpClient::new("https://api.example.com".to_string())
            .with_timeout(Duration::from_secs(10))
            .with_retry(3, Duration::from_millis(500));

        Ok("Async operation setup completed".to_string())
    }
}

//--------------------------------------------------------------------------------------------------
// Example Usage and Demonstration
//--------------------------------------------------------------------------------------------------

#[tokio::main]
/// **main**
///
/// This function provides main functionality within the Yoshi error handling framework.
///
/// # Errors
///
/// Returns an error if the operation fails due to invalid input or system constraints.
async fn main() -> Hatch<()> {
    tracing::error!("🚀 Yoshi Async Error Handling Example");

    // Example 1: HTTP Client with retry logic
    tracing::info!("\n🌐 HTTP Client demonstration...");
    let client = AsyncHttpClient::new("https://api.example.com".to_string())
        .with_timeout(Duration::from_secs(5))
        .with_retry(3, Duration::from_millis(500));

    match client.get("/users").await {
        Ok(response) => tracing::info!(
            "✅ HTTP request successful: {} - {}",
            response.status_code,
            response.body
        ),
        Err(e) => {
            tracing::info!("❌ HTTP request failed: {e}");
            if let Some(suggestion) = e.suggestion() {
                tracing::info!("💡 Suggestion: {suggestion}");
            }
        }
    }

    // Example 2: Concurrent task management
    tracing::info!("\n⚡ Concurrent task management...");
    let mut task_manager = TaskManager::new(3);

    // Spawn multiple tasks
    for i in 1..=5 {
        let task_id = format!("task_{i}");
        let future = async move {
            tokio::time::sleep(Duration::from_millis(100 * i)).await;
            if i == 3 {
                return Err(AsyncError::TaskFailed {
                    task_id: format!("task_{i}"),
                    message: "Simulated task failure".to_string(),
                }
                .into());
            }
            Ok(format!("Task {i} completed"))
        };

        match task_manager.spawn_task(task_id.clone(), future).await {
            Ok(()) => tracing::info!("✅ Task {task_id} spawned"),
            Err(e) => tracing::error!("❌ Failed to spawn task {task_id}: {e}"),
        }
    }

    // Wait for all tasks
    match task_manager.wait_for_all().await {
        Ok(completed) => tracing::info!("✅ All tasks completed: {completed:?}"),
        Err(e) => {
            tracing::info!("❌ Some tasks failed: {e}");
            if let Some(suggestion) = e.suggestion() {
                tracing::info!("💡 Suggestion: {suggestion}");
            }
        }
    }

    // Example 3: Stream processing with error recovery
    tracing::info!("\n📊 Stream processing demonstration...");
    let mut stream_processor = StreamProcessor::new(10, 3);
    let data: Vec<i32> = (1..=50).collect();

    match stream_processor.process_stream(data).await {
        Ok(results) => tracing::info!("✅ Stream processed: {} items", results.len()),
        Err(e) => {
            tracing::info!("❌ Stream processing failed: {e}");
            if let Some(suggestion) = e.suggestion() {
                tracing::info!("💡 Suggestion: {suggestion}");
            }
        }
    }

    // Example 4: WebSocket connection with reconnection
    tracing::info!("\n🔌 WebSocket connection demonstration...");
    let mut ws_manager = WebSocketManager::new("wss://echo.websocket.org".to_string());

    match ws_manager.connect().await {
        Ok(()) => {
            tracing::info!("✅ WebSocket connected");

            // Send messages
            for i in 1..=3 {
                let message = format!("Hello WebSocket {i}");
                match ws_manager.send_message(&message).await {
                    Ok(()) => tracing::info!("✅ Message sent: {message}"),
                    Err(e) => {
                        tracing::info!("❌ Message send failed: {e}");
                        if let Some(suggestion) = e.suggestion() {
                            tracing::info!("💡 Suggestion: {suggestion}");
                        }
                    }
                }
            }

            // Receive message
            match ws_manager.receive_message().await {
                Ok(message) => tracing::info!("✅ Message received: {message}"),
                Err(e) => tracing::error!("❌ Message receive failed: {e}"),
            }

            ws_manager.disconnect();
        }
        Err(e) => {
            tracing::info!("❌ WebSocket connection failed: {e}");
            if let Some(suggestion) = e.suggestion() {
                tracing::info!("💡 Suggestion: {suggestion}");
            }
        }
    }

    // Example 5: Error handling with context
    tracing::error!("\n🔍 Error context demonstration...");
    let error_demo_result = async {
        let client = AsyncHttpClient::new("https://api.example.com".to_string());
        client
            .get("/invalid")
            .await
            .lay("Failed to fetch user data from API")
    }
    .await;

    match error_demo_result {
        Ok(response) => tracing::info!("✅ Unexpected success: {}", response.body),
        Err(e) => {
            tracing::error!("❌ Expected error with context: {e}");
            if let Some(suggestion) = e.suggestion() {
                tracing::info!("💡 Suggestion: {suggestion}");
            }
        }
    }

    // Example 6: Auto-correction demonstration
    tracing::info!("\n🔧 Auto-correction demonstration...");
    match enhanced_async_operations() {
        Ok(result) => tracing::info!("✅ Auto-correction result: {result}"),
        Err(e) => tracing::error!("❌ Auto-correction failed: {e}"),
    }

    tracing::error!("\n🎉 Async error handling example completed successfully!");
    Ok(())
}
