//! # Yoshi MCP Error Handling
//!
//! This module provides comprehensive error handling for the Yoshi MCP extension
//! using the Yoshi framework exclusively. All errors are structured using YoshiKind
//! and provide rich context for debugging and auto-correction.

use yoshi_derive::YoshiError;
use yoshi_std::{Yoshi, YoshiKind};

/// Primary error type for Yoshi MCP extension using Yoshi framework
#[derive(Debug, YoshiError)]
pub enum YoshiMcpError {
    /// Model loading and management errors
    #[yoshi(
        message = "Model operation failed",
        help = "Check model file path and format compatibility"
    )]
    ModelError {
        /// The specific model operation that failed
        operation: String,
        /// Path to the model file
        model_path: String,
        /// Underlying error details
        details: String,
    },

    /// Inference engine errors
    #[yoshi(
        message = "Inference operation failed",
        help = "Verify model is loaded and input is valid"
    )]
    InferenceError {
        /// The inference operation that failed
        operation: String,
        /// Input prompt that caused the error
        prompt: String,
        /// Error details from the inference engine
        details: String,
    },

    /// WebAssembly bridge errors
    #[yoshi(
        message = "WebAssembly bridge error",
        help = "Check WASM module compilation and bindings"
    )]
    WasmError {
        /// The WASM operation that failed
        operation: String,
        /// Error details from WASM runtime
        details: String,
    },

    /// MCP server errors
    #[yoshi(
        message = "MCP server operation failed",
        help = "Check server configuration and network connectivity"
    )]
    McpServerError {
        /// The server operation that failed
        operation: String,
        /// Server endpoint or configuration details
        endpoint: String,
        /// Error details from MCP server
        details: String,
    },

    /// Configuration and validation errors
    #[yoshi(
        message = "Configuration validation failed",
        help = "Review configuration parameters and file paths"
    )]
    ConfigError {
        /// The configuration parameter that failed validation
        parameter: String,
        /// Expected value or format
        expected: String,
        /// Actual value provided
        actual: String,
    },

    /// Security and sandboxing errors
    #[yoshi(
        message = "Security constraint violation",
        help = "Review security policies and resource limits"
    )]
    SecurityError {
        /// The security constraint that was violated
        constraint: String,
        /// Resource or operation that triggered the violation
        resource: String,
        /// Security policy details
        policy: String,
    },

    /// File system and I/O errors
    #[yoshi(
        message = "File system operation failed",
        help = "Check file permissions and disk space"
    )]
    IoError {
        /// The I/O operation that failed
        operation: String,
        /// File path involved in the operation
        path: String,
        /// System error details
        details: String,
    },

    /// Network and download errors
    #[yoshi(
        message = "Network operation failed",
        help = "Check internet connectivity and server availability"
    )]
    NetworkError {
        /// The network operation that failed
        operation: String,
        /// URL or endpoint involved
        url: String,
        /// HTTP status code or network error details
        details: String,
    },

    /// Component initialization errors
    #[yoshi(
        message = "Component initialization failed",
        help = "Check dependencies and system requirements"
    )]
    InitializationError {
        /// The component that failed to initialize
        component: String,
        /// Initialization step that failed
        step: String,
        /// Error details from initialization
        details: String,
    },

    /// Resource management errors
    #[yoshi(
        message = "Resource management error",
        help = "Check system resources and memory limits"
    )]
    ResourceError {
        /// The resource type (memory, GPU, etc.)
        resource_type: String,
        /// Current resource usage
        current_usage: String,
        /// Resource limit that was exceeded
        limit: String,
    },
}

/// Extension trait for converting external errors to Yoshi MCP errors
pub trait IntoYoshiMcpError<T> {
    /// Convert the result into a Yoshi MCP error with context
    fn into_yoshi_mcp_error(self, operation: &str, context: &str) -> crate::Hatch<T>;
}

impl<T, E> IntoYoshiMcpError<T> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn into_yoshi_mcp_error(self, operation: &str, context: &str) -> crate::Hatch<T> {
        self.map_err(|e| {
            Yoshi::new(YoshiKind::External {
                message: format!("Operation '{}' failed in context '{}'", operation, context),
                source: Some(Box::new(YoshiMcpError::InitializationError {
                    component: context.to_string(),
                    step: operation.to_string(),
                    details: e.to_string(),
                })),
                category: Some("yoshi_mcp".into()),
            })
        })
    }
}

/// Helper functions for creating common Yoshi MCP errors
impl YoshiMcpError {
    /// Create a model loading error
    pub fn model_load_failed(model_path: &str, details: &str) -> Yoshi {
        Yoshi::new(YoshiKind::External {
            message: "Model loading failed".into(),
            source: Some(Box::new(Self::ModelError {
                operation: "load".to_string(),
                model_path: model_path.to_string(),
                details: details.to_string(),
            })),
            category: Some("model".into()),
        })
    }

    /// Create an inference error
    pub fn inference_failed(prompt: &str, details: &str) -> Yoshi {
        Yoshi::new(YoshiKind::External {
            message: "Text generation failed".into(),
            source: Some(Box::new(Self::InferenceError {
                operation: "generate_text".to_string(),
                prompt: prompt.to_string(),
                details: details.to_string(),
            })),
            category: Some("inference".into()),
        })
    }

    /// Create a configuration error
    pub fn config_invalid(parameter: &str, expected: &str, actual: &str) -> Yoshi {
        Yoshi::new(YoshiKind::Validation {
            message: "Configuration validation failed".into(),
            field: parameter.to_string(),
            expected: expected.to_string(),
            actual: actual.to_string(),
            suggestion: Some(format!("Set {} to {}", parameter, expected)),
        })
    }

    /// Create a security error
    pub fn security_violation(constraint: &str, resource: &str) -> Yoshi {
        Yoshi::new(YoshiKind::Security {
            message: "Security constraint violated".into(),
            violation_type: constraint.to_string(),
            resource: resource.to_string(),
            severity: "high".to_string(),
            mitigation: Some("Review security policies and resource limits".to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_error_creation() {
        let error = YoshiMcpError::model_load_failed("test.gguf", "File not found");
        assert!(error.to_string().contains("Model loading failed"));
    }

    #[test]
    fn test_inference_error_creation() {
        let error = YoshiMcpError::inference_failed("test prompt", "GPU memory exhausted");
        assert!(error.to_string().contains("Text generation failed"));
    }

    #[test]
    fn test_config_error_creation() {
        let error = YoshiMcpError::config_invalid("temperature", "0.0-2.0", "3.5");
        assert!(error
            .to_string()
            .contains("Configuration validation failed"));
    }
}
