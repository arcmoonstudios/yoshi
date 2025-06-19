//! # Security Module for Yoshi MCP
//!
//! This module provides security features including sandboxed execution,
//! resource limits, and input validation with Yoshi error handling.

use crate::{error::YoshiMcpError, Hatch};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use yoshi_std::{Yoshi, YoshiKind};

/// Security policy configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SecurityPolicy {
    /// Maximum memory usage in MB
    pub max_memory_mb: usize,
    /// Maximum inference time in seconds
    pub max_inference_time_seconds: u64,
    /// Maximum prompt length in characters
    pub max_prompt_length: usize,
    /// Maximum output length in characters
    pub max_output_length: usize,
    /// Whether to enable sandboxed execution
    pub enable_sandbox: bool,
    /// Allowed file extensions for model files
    pub allowed_model_extensions: Vec<String>,
    /// Maximum model file size in MB
    pub max_model_size_mb: u64,
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self {
            max_memory_mb: 8192, // 8GB
            max_inference_time_seconds: 30,
            max_prompt_length: 10000,
            max_output_length: 50000,
            enable_sandbox: true,
            allowed_model_extensions: vec!["gguf".to_string()],
            max_model_size_mb: 32768, // 32GB
        }
    }
}

/// Resource monitor for tracking usage
#[derive(Debug)]
pub struct ResourceMonitor {
    /// Security policy
    policy: SecurityPolicy,
    /// Current memory usage tracking
    memory_usage: Arc<RwLock<usize>>,
    /// Active inference sessions
    active_sessions: Arc<RwLock<Vec<InferenceSession>>>,
}

/// Inference session tracking
#[derive(Debug, Clone)]
pub struct InferenceSession {
    /// Session ID
    pub id: String,
    /// Start time
    pub start_time: Instant,
    /// Prompt length
    pub prompt_length: usize,
    /// Memory allocated for this session
    pub memory_allocated: usize,
}

impl ResourceMonitor {
    /// Create a new resource monitor
    pub fn new(policy: SecurityPolicy) -> Self {
        Self {
            policy,
            memory_usage: Arc::new(RwLock::new(0)),
            active_sessions: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Start monitoring an inference session
    pub async fn start_session(&self, prompt: &str) -> Hatch<String> {
        // Validate prompt length
        if prompt.len() > self.policy.max_prompt_length {
            return Err(YoshiMcpError::security_violation(
                "max_prompt_length",
                &format!("prompt length: {}", prompt.len()),
            ));
        }

        // Validate prompt content
        self.validate_prompt_content(prompt)?;

        // Check resource availability
        self.check_resource_availability().await?;

        // Create session
        let session_id = format!("session_{}", uuid::Uuid::new_v4());
        let session = InferenceSession {
            id: session_id.clone(),
            start_time: Instant::now(),
            prompt_length: prompt.len(),
            memory_allocated: self.estimate_memory_usage(prompt.len()),
        };

        // Add to active sessions
        {
            let mut sessions = self.active_sessions.write().await;
            sessions.push(session.clone());
        }

        // Update memory usage
        {
            let mut memory = self.memory_usage.write().await;
            *memory += session.memory_allocated;
        }

        Ok(session_id)
    }

    /// End monitoring an inference session
    pub async fn end_session(&self, session_id: &str, output: &str) -> Hatch<()> {
        // Validate output length
        if output.len() > self.policy.max_output_length {
            return Err(YoshiMcpError::security_violation(
                "max_output_length",
                &format!("output length: {}", output.len()),
            ));
        }

        // Find and remove session
        let session = {
            let mut sessions = self.active_sessions.write().await;
            let position = sessions.iter().position(|s| s.id == session_id);

            if let Some(pos) = position {
                sessions.remove(pos)
            } else {
                return Err(YoshiMcpError::config_invalid(
                    "session_id",
                    "existing session ID",
                    session_id,
                ));
            }
        };

        // Check inference time
        let elapsed = session.start_time.elapsed();
        if elapsed.as_secs() > self.policy.max_inference_time_seconds {
            return Err(YoshiMcpError::security_violation(
                "max_inference_time",
                &format!("elapsed time: {}s", elapsed.as_secs()),
            ));
        }

        // Update memory usage
        {
            let mut memory = self.memory_usage.write().await;
            *memory = memory.saturating_sub(session.memory_allocated);
        }

        Ok(())
    }

    /// Validate prompt content for security issues
    fn validate_prompt_content(&self, prompt: &str) -> Hatch<()> {
        // Check for potentially dangerous patterns
        let dangerous_patterns = [
            "system(",
            "exec(",
            "eval(",
            "import os",
            "subprocess",
            "__import__",
            "file://",
            "javascript:",
            "<script",
        ];

        for pattern in &dangerous_patterns {
            if prompt.to_lowercase().contains(pattern) {
                return Err(YoshiMcpError::security_violation(
                    "dangerous_content",
                    &format!("pattern: {}", pattern),
                ));
            }
        }

        // Check for excessive repetition (potential DoS)
        if self.has_excessive_repetition(prompt) {
            return Err(YoshiMcpError::security_violation(
                "excessive_repetition",
                "prompt contains excessive repetitive content",
            ));
        }

        Ok(())
    }

    /// Check for excessive repetition in prompt
    fn has_excessive_repetition(&self, prompt: &str) -> bool {
        let words: Vec<&str> = prompt.split_whitespace().collect();
        if words.len() < 10 {
            return false;
        }

        // Check for repeated sequences
        let mut repetition_count = 0;
        let window_size = 5;

        for i in 0..words.len().saturating_sub(window_size * 2) {
            let window1 = &words[i..i + window_size];
            let window2 = &words[i + window_size..i + window_size * 2];

            if window1 == window2 {
                repetition_count += 1;
                if repetition_count > 3 {
                    return true;
                }
            }
        }

        false
    }

    /// Check resource availability
    async fn check_resource_availability(&self) -> Hatch<()> {
        let current_memory = *self.memory_usage.read().await;
        let memory_limit = self.policy.max_memory_mb * 1024 * 1024; // Convert to bytes

        if current_memory > memory_limit {
            return Err(YoshiMcpError::security_violation(
                "memory_limit",
                &format!(
                    "current: {}MB, limit: {}MB",
                    current_memory / (1024 * 1024),
                    self.policy.max_memory_mb
                ),
            ));
        }

        // Check active session count
        let active_count = self.active_sessions.read().await.len();
        if active_count > 10 {
            // Arbitrary limit
            return Err(YoshiMcpError::security_violation(
                "session_limit",
                &format!("active sessions: {}", active_count),
            ));
        }

        Ok(())
    }

    /// Estimate memory usage for a prompt
    fn estimate_memory_usage(&self, prompt_length: usize) -> usize {
        // Rough estimation: 4 bytes per character + overhead
        (prompt_length * 4) + (1024 * 1024) // 1MB overhead
    }

    /// Get current resource usage
    pub async fn get_resource_usage(&self) -> ResourceUsage {
        let memory_usage = *self.memory_usage.read().await;
        let active_sessions = self.active_sessions.read().await.len();

        ResourceUsage {
            memory_usage_mb: memory_usage / (1024 * 1024),
            memory_limit_mb: self.policy.max_memory_mb,
            active_sessions,
            session_limit: 10,
        }
    }

    /// Clean up expired sessions
    pub async fn cleanup_expired_sessions(&self) -> Hatch<()> {
        let now = Instant::now();
        let timeout = Duration::from_secs(self.policy.max_inference_time_seconds);

        let mut sessions = self.active_sessions.write().await;
        let mut memory = self.memory_usage.write().await;

        let mut expired_memory = 0;
        sessions.retain(|session| {
            if now.duration_since(session.start_time) > timeout {
                expired_memory += session.memory_allocated;
                false
            } else {
                true
            }
        });

        *memory = memory.saturating_sub(expired_memory);

        Ok(())
    }
}

/// Current resource usage information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResourceUsage {
    /// Current memory usage in MB
    pub memory_usage_mb: usize,
    /// Memory limit in MB
    pub memory_limit_mb: usize,
    /// Number of active sessions
    pub active_sessions: usize,
    /// Session limit
    pub session_limit: usize,
}

/// Input sanitizer for cleaning user inputs
pub struct InputSanitizer;

impl InputSanitizer {
    /// Sanitize user input for safe processing
    pub fn sanitize_prompt(input: &str) -> String {
        input
            .chars()
            .filter(|c| c.is_ascii() && !c.is_control() || c.is_whitespace())
            .take(10000) // Limit length
            .collect()
    }

    /// Sanitize file path to prevent directory traversal
    pub fn sanitize_file_path(path: &str) -> Hatch<String> {
        let path = path.replace("../", "").replace("..\\", "");

        if path.contains("..") || path.starts_with('/') || path.contains(':') {
            return Err(YoshiMcpError::security_violation("path_traversal", path));
        }

        Ok(path)
    }

    /// Validate model file extension
    pub fn validate_model_file(filename: &str, policy: &SecurityPolicy) -> Hatch<()> {
        let extension = std::path::Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        if !policy
            .allowed_model_extensions
            .contains(&extension.to_lowercase())
        {
            return Err(YoshiMcpError::security_violation(
                "invalid_file_extension",
                &format!("extension: {}", extension),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_resource_monitor() {
        let policy = SecurityPolicy::default();
        let monitor = ResourceMonitor::new(policy);

        let session_id = monitor.start_session("test prompt").await.unwrap();
        assert!(!session_id.is_empty());

        monitor
            .end_session(&session_id, "test output")
            .await
            .unwrap();
    }

    #[test]
    fn test_input_sanitizer() {
        let sanitized = InputSanitizer::sanitize_prompt("Hello\x00World\nTest");
        assert_eq!(sanitized, "HelloWorld\nTest");
    }

    #[test]
    fn test_path_sanitization() {
        assert!(InputSanitizer::sanitize_file_path("../etc/passwd").is_err());
        assert!(InputSanitizer::sanitize_file_path("model.gguf").is_ok());
    }
}
