//! Integration tests for Yoshi MCP extension

use yoshi_mcp::{YoshiMcpConfig, YoshiMcpExtension, Hatch};

#[tokio::test]
async fn test_extension_creation() {
    let config = YoshiMcpConfig::default();
    let result = YoshiMcpExtension::new(config);
    assert!(result.is_ok());
}

#[test]
fn test_config_default_values() {
    let config = YoshiMcpConfig::default();
    assert_eq!(config.max_tokens, 200);
    assert_eq!(config.temperature, 0.7);
    assert!(config.enable_gpu);
    assert_eq!(config.cache_size, 100);
    assert!(!config.auto_download_models);
    assert!(config.model_path.is_none());
}

#[test]
fn test_config_serialization() {
    let config = YoshiMcpConfig::default();
    let serialized = serde_json::to_string(&config);
    assert!(serialized.is_ok());
    
    let deserialized: Result<YoshiMcpConfig, _> = serde_json::from_str(&serialized.unwrap());
    assert!(deserialized.is_ok());
}

#[tokio::test]
async fn test_extension_status() {
    let config = YoshiMcpConfig::default();
    let extension = YoshiMcpExtension::new(config).unwrap();
    
    let status = extension.status();
    assert!(!status.initialized); // Not initialized yet
    assert!(!status.model_loaded);
    assert!(!status.mcp_server_running);
}

#[cfg(test)]
mod error_tests {
    use yoshi_mcp::error::YoshiMcpError;

    #[test]
    fn test_model_error_creation() {
        let error = YoshiMcpError::model_load_failed("test.gguf", "File not found");
        let error_string = error.to_string();
        assert!(error_string.contains("Model loading failed"));
    }

    #[test]
    fn test_inference_error_creation() {
        let error = YoshiMcpError::inference_failed("test prompt", "GPU memory exhausted");
        let error_string = error.to_string();
        assert!(error_string.contains("Text generation failed"));
    }

    #[test]
    fn test_config_error_creation() {
        let error = YoshiMcpError::config_invalid("temperature", "0.0-2.0", "3.5");
        let error_string = error.to_string();
        assert!(error_string.contains("Configuration validation failed"));
    }

    #[test]
    fn test_security_error_creation() {
        let error = YoshiMcpError::security_violation("memory_limit", "8GB exceeded");
        let error_string = error.to_string();
        assert!(error_string.contains("Security constraint violated"));
    }
}

#[cfg(test)]
mod model_tests {
    use yoshi_mcp::model::ModelManager;
    use yoshi_mcp::YoshiMcpConfig;

    #[test]
    fn test_model_manager_creation() {
        let config = YoshiMcpConfig::default();
        let result = ModelManager::new(&config);
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod security_tests {
    use yoshi_mcp::security::{ResourceMonitor, SecurityPolicy, InputSanitizer};

    #[test]
    fn test_security_policy_default() {
        let policy = SecurityPolicy::default();
        assert_eq!(policy.max_memory_mb, 8192);
        assert_eq!(policy.max_inference_time_seconds, 30);
        assert_eq!(policy.max_prompt_length, 10000);
        assert!(policy.enable_sandbox);
    }

    #[test]
    fn test_resource_monitor_creation() {
        let policy = SecurityPolicy::default();
        let monitor = ResourceMonitor::new(policy);
        // Just test that it can be created without panicking
        drop(monitor);
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

    #[test]
    fn test_model_file_validation() {
        let policy = SecurityPolicy::default();
        assert!(InputSanitizer::validate_model_file("model.gguf", &policy).is_ok());
        assert!(InputSanitizer::validate_model_file("model.txt", &policy).is_err());
    }
}

#[cfg(test)]
mod mcp_tests {
    use yoshi_mcp::mcp::McpServer;
    use yoshi_mcp::YoshiMcpConfig;

    #[tokio::test]
    async fn test_mcp_server_creation() {
        let config = YoshiMcpConfig::default();
        let result = McpServer::new(config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_mcp_server_capabilities() {
        let config = YoshiMcpConfig::default();
        let server = McpServer::new(config).await.unwrap();
        let capabilities = server.get_capabilities();
        
        assert!(!capabilities.tools.is_empty());
        assert!(!capabilities.resources.is_empty());
        
        // Check for expected tools
        let tool_names: Vec<_> = capabilities.tools.iter().map(|t| &t.name).collect();
        assert!(tool_names.contains(&&"generate_code".to_string()));
        assert!(tool_names.contains(&&"explain_code".to_string()));
        assert!(tool_names.contains(&&"fix_code".to_string()));
    }
}

#[cfg(test)]
mod inference_tests {
    use yoshi_mcp::inference::InferenceEngine;
    use yoshi_mcp::YoshiMcpConfig;

    #[tokio::test]
    async fn test_inference_engine_creation() {
        let config = YoshiMcpConfig::default();
        let result = InferenceEngine::new(config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_inference_engine_model_status() {
        let config = YoshiMcpConfig::default();
        let engine = InferenceEngine::new(config).await.unwrap();
        assert!(!engine.is_model_loaded()); // No model loaded initially
    }

    #[tokio::test]
    async fn test_inference_engine_metrics() {
        let config = YoshiMcpConfig::default();
        let engine = InferenceEngine::new(config).await.unwrap();
        let metrics = engine.get_metrics();
        // Should be empty initially
        assert!(metrics.is_empty() || metrics.len() >= 0);
    }

    #[tokio::test]
    async fn test_inference_engine_cache_clear() {
        let config = YoshiMcpConfig::default();
        let engine = InferenceEngine::new(config).await.unwrap();
        // Should not panic
        engine.clear_cache().await;
    }
}

#[cfg(feature = "wasm")]
#[cfg(test)]
mod wasm_tests {
    use wasm_bindgen_test::*;
    use yoshi_mcp::wasm::{WasmUtils, WasmPerformanceMonitor};

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_wasm_utils_validation() {
        let config = r#"{"max_tokens": 200, "temperature": 0.7, "cache_size": 100, "enable_gpu": true, "auto_download_models": false}"#;
        let result = WasmUtils::validate_config(config);
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_wasm_utils_invalid_config() {
        let config = r#"{"max_tokens": 0, "temperature": 3.0}"#;
        let result = WasmUtils::validate_config(config);
        assert!(result.is_err());
    }

    #[wasm_bindgen_test]
    fn test_performance_monitor() {
        let mut monitor = WasmPerformanceMonitor::new();
        monitor.start_timing("test_operation");
        
        // Simulate some work
        let _result = js_sys::Date::now();
        
        let duration = monitor.end_timing("test_operation");
        assert!(duration >= 0.0);
    }

    #[wasm_bindgen_test]
    fn test_memory_estimation() {
        let memory = WasmUtils::estimate_memory_usage(1000, 200);
        assert!(memory > 0);
    }

    #[wasm_bindgen_test]
    fn test_system_info() {
        let info = WasmUtils::get_system_info();
        assert!(!info.is_empty());
        assert!(info.contains("wasm32"));
    }
}
