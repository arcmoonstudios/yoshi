//! # WebAssembly Bridge for Yoshi MCP
//!
//! This module provides WebAssembly bindings for the Yoshi MCP extension
//! to enable high-performance inference in the browser/VS Code environment.

use crate::{error::YoshiMcpError, inference::InferenceEngine, Hatch, YoshiMcpConfig};
use js_sys::Promise;
use std::sync::Arc;
use tokio::sync::RwLock;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use web_sys::console;
use yoshi_std::{Yoshi, YoshiKind};

// Set up panic hook for better error reporting in WASM
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

/// WebAssembly wrapper for the Yoshi MCP inference engine
#[wasm_bindgen]
pub struct WasmInferenceEngine {
    engine: Arc<RwLock<InferenceEngine>>,
    config: YoshiMcpConfig,
}

#[wasm_bindgen]
impl WasmInferenceEngine {
    /// Create a new WASM inference engine
    #[wasm_bindgen(constructor)]
    pub fn new(config_json: &str) -> Result<WasmInferenceEngine, JsValue> {
        let config: YoshiMcpConfig = serde_json::from_str(config_json)
            .map_err(|e| JsValue::from_str(&format!("Config parse error: {}", e)))?;

        // Note: In WASM, we can't use async in constructor, so we create uninitialized
        let engine = Arc::new(RwLock::new(
            // This is a placeholder - actual initialization happens in init()
            InferenceEngine::new_placeholder(config.clone()),
        ));

        Ok(WasmInferenceEngine { engine, config })
    }

    /// Initialize the inference engine (async)
    #[wasm_bindgen]
    pub fn init(&self) -> Promise {
        let engine = self.engine.clone();
        let config = self.config.clone();

        future_to_promise(async move {
            let mut engine_guard = engine.write().await;
            *engine_guard = InferenceEngine::new(config)
                .await
                .map_err(|e| JsValue::from_str(&e.to_string()))?;

            Ok(JsValue::from_str("Initialized"))
        })
    }

    /// Load a model from the given path
    #[wasm_bindgen]
    pub fn load_model(&self, model_path: &str) -> Promise {
        let engine = self.engine.clone();
        let model_path = model_path.to_string();

        future_to_promise(async move {
            let mut engine_guard = engine.write().await;
            engine_guard
                .load_model(&model_path)
                .await
                .map_err(|e| JsValue::from_str(&e.to_string()))?;

            Ok(JsValue::from_str("Model loaded"))
        })
    }

    /// Generate text from a prompt
    #[wasm_bindgen]
    pub fn generate_text(&self, prompt: &str, max_tokens: usize) -> Promise {
        let engine = self.engine.clone();
        let prompt = prompt.to_string();

        future_to_promise(async move {
            let engine_guard = engine.read().await;
            let result = engine_guard
                .generate_text(&prompt, max_tokens)
                .await
                .map_err(|e| JsValue::from_str(&e.to_string()))?;

            Ok(JsValue::from_str(&result))
        })
    }

    /// Check if a model is loaded
    #[wasm_bindgen]
    pub fn is_model_loaded(&self) -> Promise {
        let engine = self.engine.clone();

        future_to_promise(async move {
            let engine_guard = engine.read().await;
            Ok(JsValue::from_bool(engine_guard.is_model_loaded()))
        })
    }

    /// Get performance metrics
    #[wasm_bindgen]
    pub fn get_metrics(&self) -> Promise {
        let engine = self.engine.clone();

        future_to_promise(async move {
            let engine_guard = engine.read().await;
            let metrics = engine_guard.get_metrics();
            let metrics_json = serde_json::to_string(&metrics)
                .map_err(|e| JsValue::from_str(&format!("Metrics serialization error: {}", e)))?;

            Ok(JsValue::from_str(&metrics_json))
        })
    }

    /// Clear the inference cache
    #[wasm_bindgen]
    pub fn clear_cache(&self) -> Promise {
        let engine = self.engine.clone();

        future_to_promise(async move {
            let engine_guard = engine.read().await;
            engine_guard.clear_cache().await;
            Ok(JsValue::from_str("Cache cleared"))
        })
    }

    /// Get current configuration
    #[wasm_bindgen]
    pub fn get_config(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.config)
            .map_err(|e| JsValue::from_str(&format!("Config serialization error: {}", e)))
    }

    /// Update configuration
    #[wasm_bindgen]
    pub fn update_config(&mut self, config_json: &str) -> Result<(), JsValue> {
        self.config = serde_json::from_str(config_json)
            .map_err(|e| JsValue::from_str(&format!("Config parse error: {}", e)))?;
        Ok(())
    }
}

/// WebAssembly utilities for the extension
#[wasm_bindgen]
pub struct WasmUtils;

#[wasm_bindgen]
impl WasmUtils {
    /// Log a message to the browser console
    #[wasm_bindgen]
    pub fn log(message: &str) {
        console::log_1(&JsValue::from_str(message));
    }

    /// Log an error to the browser console
    #[wasm_bindgen]
    pub fn error(message: &str) {
        console::error_1(&JsValue::from_str(message));
    }

    /// Validate a configuration object
    #[wasm_bindgen]
    pub fn validate_config(config_json: &str) -> Result<String, JsValue> {
        let config: YoshiMcpConfig = serde_json::from_str(config_json)
            .map_err(|e| JsValue::from_str(&format!("Config parse error: {}", e)))?;

        // Validate configuration
        if config.max_tokens == 0 {
            return Err(JsValue::from_str("max_tokens must be greater than 0"));
        }

        if config.temperature < 0.0 || config.temperature > 2.0 {
            return Err(JsValue::from_str("temperature must be between 0.0 and 2.0"));
        }

        if config.cache_size == 0 {
            return Err(JsValue::from_str("cache_size must be greater than 0"));
        }

        Ok("Configuration is valid".to_string())
    }

    /// Get system information
    #[wasm_bindgen]
    pub fn get_system_info() -> String {
        let info = serde_json::json!({
            "platform": "wasm32",
            "target_arch": env!("CARGO_CFG_TARGET_ARCH"),
            "target_os": env!("CARGO_CFG_TARGET_OS"),
            "rust_version": env!("CARGO_PKG_RUST_VERSION"),
            "package_version": env!("CARGO_PKG_VERSION")
        });

        info.to_string()
    }

    /// Calculate memory usage estimate
    #[wasm_bindgen]
    pub fn estimate_memory_usage(prompt_length: usize, max_tokens: usize) -> usize {
        // Rough estimation for memory usage
        let base_memory = 50 * 1024 * 1024; // 50MB base
        let prompt_memory = prompt_length * 4; // 4 bytes per character
        let output_memory = max_tokens * 4; // 4 bytes per token

        base_memory + prompt_memory + output_memory
    }
}

/// Performance monitor for WASM environment
#[wasm_bindgen]
pub struct WasmPerformanceMonitor {
    start_time: f64,
    metrics: std::collections::HashMap<String, f64>,
}

#[wasm_bindgen]
impl WasmPerformanceMonitor {
    /// Create a new performance monitor
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmPerformanceMonitor {
        WasmPerformanceMonitor {
            start_time: js_sys::Date::now(),
            metrics: std::collections::HashMap::new(),
        }
    }

    /// Start timing an operation
    #[wasm_bindgen]
    pub fn start_timing(&mut self, operation: &str) {
        self.metrics
            .insert(format!("{}_start", operation), js_sys::Date::now());
    }

    /// End timing an operation
    #[wasm_bindgen]
    pub fn end_timing(&mut self, operation: &str) -> f64 {
        let start_key = format!("{}_start", operation);
        if let Some(start_time) = self.metrics.get(&start_key) {
            let duration = js_sys::Date::now() - start_time;
            self.metrics
                .insert(format!("{}_duration", operation), duration);
            duration
        } else {
            0.0
        }
    }

    /// Get all metrics as JSON
    #[wasm_bindgen]
    pub fn get_metrics(&self) -> String {
        serde_json::to_string(&self.metrics).unwrap_or_else(|_| "{}".to_string())
    }

    /// Reset all metrics
    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.metrics.clear();
        self.start_time = js_sys::Date::now();
    }
}

// Implementation for InferenceEngine placeholder (WASM-specific)
impl InferenceEngine {
    /// Create a placeholder inference engine for WASM initialization
    pub fn new_placeholder(config: YoshiMcpConfig) -> Self {
        // This is a minimal placeholder that will be replaced during init()
        Self {
            device: candle_core::Device::Cpu,
            model: None,
            tokenizer: None,
            model_config: None,
            kv_cache: None,
            config,
            cache: Arc::new(RwLock::new(lru::LruCache::new(
                std::num::NonZeroUsize::new(100).unwrap(),
            ))),
            metrics: Arc::new(dashmap::DashMap::new()),
            model_loaded: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_wasm_utils_validation() {
        let config = r#"{"max_tokens": 200, "temperature": 0.7, "cache_size": 100}"#;
        let result = WasmUtils::validate_config(config);
        assert!(result.is_ok());
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
}
