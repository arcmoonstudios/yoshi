//! # Yoshi MCP - VS Code Extension with Local LLM Inference
//!
//! This crate provides a complete VS Code extension with local LLM inference capabilities
//! using the Candle framework, WebAssembly bridge, and MCP server architecture.
//!
//! ## Architecture Overview
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                          Yoshi-MCP                          │
//! │  ┌─────────────────┐    ┌──────────────────────────────┐    │
//! │  │  TypeScript     │    │       MCP Server             │    │
//! │  │     Shell       │◄──►│    (Context Protocol)        │    │
//! │  │                 │    │                              │    │
//! │  │  ┌─────────────┐│    │  ┌─────────────────────────┐ │    │
//! │  │  │ WebAssembly ││    │  │    Candle Inference     │ │    │
//! │  │  │   Bridge    ││    │  │      Engine (Rust)      │ │    │
//! │  │  │             ││    │  │                         │ │    │
//! │  │  │  ┌─────────┐││    │  │   ┌─────────────────────┤ │    │
//! │  │  │  │ GGUF    │││    │  │   │ QWEN/Llama Models   │ │    │
//! │  │  │  │ Models  │││    │  │   │   (Quantized)       │ │    │
//! │  │  │  └─────────┘││    │  │   └─────────────────────┤ │    │
//! │  │  └─────────────┘│    │  └─────────────────────────┘ │    │
//! │  └─────────────────┘    └──────────────────────────────┘    │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Core Components
//!
//! - **Inference Engine**: Candle-based local LLM inference with GGUF model support
//! - **WebAssembly Bridge**: High-performance WASM bindings for browser integration
//! - **MCP Server**: Model Context Protocol server for VS Code integration
//! - **Error Handling**: Complete Yoshi framework integration for robust error management
//! - **Performance Optimization**: Caching, streaming, and resource management
//!
//! ## Features
//!
//! - Local LLM inference with no external API calls
//! - Support for QWEN, Llama, and other GGUF models
//! - GPU acceleration (CUDA, Metal, MKL)
//! - Streaming text generation with real-time updates
//! - Intelligent caching for improved performance
//! - Secure model verification and sandboxed execution
//! - Complete VS Code integration with commands and keybindings

// Yoshi framework - exclusive error handling
use yoshi_derive::YoshiError;
use yoshi_std::{Hatch, Yoshi, YoshiKind};

pub mod error;
pub mod inference;
pub mod mcp;
pub mod model;
pub mod security;
pub mod wasm;

// Re-export core types for easy access
pub use error::*;
pub use inference::*;
pub use mcp::*;
pub use model::*;
pub use security::*;

/// Main result type for the Yoshi MCP extension using Yoshi framework
pub type Hatch<T> = Result<T, Yoshi>;

/// Configuration for the Yoshi MCP extension
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct YoshiMcpConfig {
    /// Path to the local GGUF model file
    pub model_path: Option<String>,
    /// Maximum tokens to generate
    pub max_tokens: usize,
    /// Sampling temperature
    pub temperature: f32,
    /// Enable GPU acceleration
    pub enable_gpu: bool,
    /// Cache size for inference results
    pub cache_size: usize,
    /// Auto-download recommended models
    pub auto_download_models: bool,
}

impl Default for YoshiMcpConfig {
    fn default() -> Self {
        Self {
            model_path: None,
            max_tokens: 200,
            temperature: 0.7,
            enable_gpu: true,
            cache_size: 100,
            auto_download_models: false,
        }
    }
}

/// Main extension context and state management
#[derive(Debug)]
pub struct YoshiMcpExtension {
    config: YoshiMcpConfig,
    inference_engine: Option<InferenceEngine>,
    model_manager: ModelManager,
    mcp_server: Option<McpServer>,
}

impl YoshiMcpExtension {
    /// Create a new Yoshi MCP extension instance
    pub fn new(config: YoshiMcpConfig) -> Hatch<Self> {
        let model_manager = ModelManager::new(&config)?;

        Ok(Self {
            config,
            inference_engine: None,
            model_manager,
            mcp_server: None,
        })
    }

    /// Initialize the extension with all components
    pub async fn initialize(&mut self) -> Hatch<()> {
        // Initialize inference engine
        let mut engine = InferenceEngine::new(self.config.clone()).await?;

        // Load model if specified
        if let Some(model_path) = &self.config.model_path {
            engine.load_model(model_path).await?;
        }

        self.inference_engine = Some(engine);

        // Initialize MCP server
        let mcp_server = McpServer::new(self.config.clone()).await?;
        self.mcp_server = Some(mcp_server);

        Ok(())
    }

    /// Generate text using the local LLM
    pub async fn generate_text(&self, prompt: &str) -> Hatch<String> {
        let engine = self.inference_engine.as_ref().ok_or_else(|| {
            Yoshi::new(YoshiKind::Internal {
                message: "Inference engine not initialized".into(),
                source: None,
                component: Some("inference_engine".into()),
            })
        })?;

        engine.generate_text(prompt, self.config.max_tokens).await
    }

    /// Get extension status
    pub fn status(&self) -> YoshiMcpStatus {
        YoshiMcpStatus {
            initialized: self.inference_engine.is_some(),
            model_loaded: self
                .inference_engine
                .as_ref()
                .map(|e| e.is_model_loaded())
                .unwrap_or(false),
            mcp_server_running: self
                .mcp_server
                .as_ref()
                .map(|s| s.is_running())
                .unwrap_or(false),
            config: self.config.clone(),
        }
    }
}

/// Extension status information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct YoshiMcpStatus {
    pub initialized: bool,
    pub model_loaded: bool,
    pub mcp_server_running: bool,
    pub config: YoshiMcpConfig,
}
